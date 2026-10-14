//! Impact analysis: what a change could break, and for whom (specification,
//! Section 12).
//!
//! This first slice works from the syntax alone. It compares two versions of
//! a module function by function, clause by clause, and applies the rules of
//! Section 12.3 that can be decided without a prover. Where a rule needs the
//! prover (is a changed precondition stronger or weaker?), the verdict is
//! "needs review" and says so; where a body changed under an unchanged
//! contract, the verdict is "unchanged, unchecked", because nothing checks the
//! body against its contract yet. Clauses are compared as parsed trees, so a
//! change of layout or notes is no change.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use scribe_syntax::ast::{
    Clause, File, Function, Item, ModuleSection, Param, Stmt, block_sexpr, clause_sexpr, clause_text,
    expr_sexpr, expr_text, signature_text, stmt_sexpr, type_sexpr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verdict {
    Breaking,
    AuthorityChange,
    NeedsReview,
    UnchangedUnchecked,
    Compatible,
    Identical,
}

impl Verdict {
    fn heading(self) -> &'static str {
        match self {
            Verdict::Breaking => "Breaking",
            Verdict::AuthorityChange => "Authority change",
            Verdict::NeedsReview => "Needs review",
            Verdict::UnchangedUnchecked => "Unchanged, unchecked",
            Verdict::Compatible => "Compatible",
            Verdict::Identical => "Identical",
        }
    }
}

/// One function's verdict, the most severe of its findings, with every reason.
#[derive(Clone, Debug, PartialEq)]
pub struct Finding {
    pub function: String,
    pub verdict: Verdict,
    pub reasons: Vec<String>,
    /// Modules that require this module and use this function
    pub dependents: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub module: Option<String>,
    pub findings: Vec<Finding>,
    /// Capabilities the module uses now and did not before, each with the
    /// functions that use it
    pub new_authority: Vec<(String, Vec<String>)>,
}

/// What a version of a module offers: its public functions with their
/// contracts, and the body each has.
struct Version {
    module: Option<String>,
    functions: BTreeMap<String, Function>,
    bodies: BTreeMap<String, Vec<Stmt>>,
}

fn version(file: &File) -> Version {
    let mut module = None;
    let mut exported = BTreeMap::new();
    let mut top_level = BTreeMap::new();
    for item in &file.items {
        match item {
            Item::Module(m) => {
                module = Some(m.name.clone());
                for s in &m.sections {
                    if let ModuleSection::Exports(fs) = s {
                        for f in fs {
                            exported.insert(f.name.clone(), f.clone());
                        }
                    }
                }
            }
            Item::Function(f) => {
                top_level.insert(f.name.clone(), f.clone());
            }
            Item::Interface(i) => {
                for f in &i.functions {
                    exported.insert(format!("{}.{}", i.name, f.name), f.clone());
                }
            }
            _ => {}
        }
    }
    let bodies = top_level.iter().map(|(n, f)| (n.clone(), f.body.clone())).collect();
    // with a module declaration, only its exports are public; without one,
    // every top-level function is
    let functions = if module.is_some() { exported } else { top_level.into_iter().chain(exported).collect() };
    Version { module, functions, bodies }
}

/// For each function of `module`, the dependents (modules that require it)
/// that use it. A `requires` entry with no `uses` line relies on everything.
fn dependents_of(module: &str, dependents: &[File]) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in dependents {
        for item in &file.items {
            let Item::Module(m) = item else { continue };
            for s in &m.sections {
                let ModuleSection::Requires(needs) = s else { continue };
                for need in needs.iter().filter(|n| n.module == module) {
                    let used = if need.uses.is_empty() { vec!["*".to_string()] } else { need.uses.clone() };
                    for f in used {
                        out.entry(f).or_default().push(m.name.clone());
                    }
                }
            }
        }
    }
    out
}

/// A function's contract, clause by clause. Clauses are compared by their
/// parsed trees (the keys) and shown as Scribe text (the values).
struct Contract {
    failures: BTreeSet<String>,
    preconditions: BTreeMap<String, String>,
    guarantees: BTreeMap<String, String>,
    changes_only: Option<BTreeMap<String, String>>,
    uses: BTreeSet<String>,
}

fn contract(f: &Function) -> Contract {
    let mut c = Contract {
        failures: BTreeSet::new(),
        preconditions: BTreeMap::new(),
        guarantees: BTreeMap::new(),
        changes_only: None,
        uses: BTreeSet::new(),
    };
    for clause in &f.contract {
        match clause {
            Clause::CanFail(n) => c.failures.extend(n.iter().cloned()),
            Clause::Expect(_) => {
                c.preconditions.insert(clause_sexpr(clause), clause_text(clause));
            }
            Clause::Ensure(_) | Clause::Example(_) | Clause::Property(_) => {
                c.guarantees.insert(clause_sexpr(clause), clause_text(clause));
            }
            Clause::Changes(places) => {
                c.changes_only
                    .get_or_insert_with(BTreeMap::new)
                    .extend(places.iter().map(|p| (expr_sexpr(p), expr_text(p))));
            }
            Clause::Uses(n) => c.uses.extend(n.iter().cloned()),
        }
    }
    c
}

/// The entries of `a` whose keys `b` lacks, as text.
fn missing<'m>(a: &'m BTreeMap<String, String>, b: &BTreeMap<String, String>) -> Vec<&'m String> {
    a.iter().filter(|(k, _)| !b.contains_key(*k)).map(|(_, v)| v).collect()
}

fn signature(params: &[Param], result: &Option<scribe_syntax::ast::TypeExpr>) -> (Vec<String>, String) {
    (params.iter().map(|p| type_sexpr(&p.ty)).collect(), result.as_ref().map_or("-".into(), type_sexpr))
}

fn trust_blocks(stmts: &[Stmt], out: &mut Vec<String>) {
    for s in stmts {
        match s {
            Stmt::Trust(b) => out.push(block_sexpr(b)),
            Stmt::If { branches, otherwise } => {
                for (_, b) in branches {
                    trust_blocks(b, out);
                }
                if let Some(b) = otherwise {
                    trust_blocks(b, out);
                }
            }
            Stmt::For { body, .. }
            | Stmt::While { body, .. }
            | Stmt::Hold { body, .. }
            | Stmt::Together(body) => trust_blocks(body, out),
            Stmt::Match { cases, .. } => {
                for c in cases {
                    trust_blocks(&c.body, out);
                }
            }
            Stmt::Handle { body, handlers } => {
                trust_blocks(body, out);
                for h in handlers {
                    trust_blocks(&h.body, out);
                }
            }
            _ => {}
        }
    }
}

fn list(set: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    set.into_iter().map(|s| s.as_ref().to_string()).collect::<Vec<_>>().join(", ")
}

fn module_uses(v: &Version) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, f) in &v.functions {
        for u in contract(f).uses {
            out.entry(u).or_default().push(name.clone());
        }
    }
    out
}

/// Compare two versions of a module. `dependents` are other modules' sources,
/// read for their `requires` entries.
pub fn compare(old: &File, new: &File, dependents: &[File]) -> Report {
    let old_v = version(old);
    let new_v = version(new);
    let module = new_v.module.clone().or(old_v.module.clone());
    let users = module.as_deref().map(|m| dependents_of(m, dependents)).unwrap_or_default();
    let users_of = |f: &str| -> Vec<String> {
        let mut u: Vec<String> = users.get(f).into_iter().chain(users.get("*")).flatten().cloned().collect();
        u.sort();
        u.dedup();
        u
    };
    let old_uses = module_uses(&old_v);
    let new_uses = module_uses(&new_v);

    let mut findings = Vec::new();
    let names: BTreeSet<&String> = old_v.functions.keys().chain(new_v.functions.keys()).collect();
    for name in names {
        let dependents = users_of(name);
        let finding = match (old_v.functions.get(name), new_v.functions.get(name)) {
            (Some(_), None) => Finding {
                function: name.clone(),
                verdict: Verdict::Breaking,
                reasons: vec!["removed; every caller the change leaves untouched breaks".into()],
                dependents,
            },
            (None, Some(_)) => Finding {
                function: name.clone(),
                verdict: Verdict::Compatible,
                reasons: vec!["new function".into()],
                dependents,
            },
            (Some(o), Some(n)) => compare_function(
                name,
                o,
                n,
                old_v.bodies.get(name),
                new_v.bodies.get(name),
                &old_uses,
                dependents,
            ),
            (None, None) => unreachable!(),
        };
        findings.push(finding);
    }

    let new_authority = new_uses
        .iter()
        .filter(|(cap, _)| !old_uses.contains_key(*cap))
        .map(|(cap, fs)| (cap.clone(), fs.clone()))
        .collect();
    Report { module, findings, new_authority }
}

fn compare_function(
    name: &str,
    old: &Function,
    new: &Function,
    old_body: Option<&Vec<Stmt>>,
    new_body: Option<&Vec<Stmt>>,
    old_module_uses: &BTreeMap<String, Vec<String>>,
    dependents: Vec<String>,
) -> Finding {
    let mut found: Vec<(Verdict, String)> = Vec::new();

    // the signature
    let (old_types, old_result) = signature(&old.params, &old.result);
    let (new_types, new_result) = signature(&new.params, &new.result);
    if old_types != new_types || old_result != new_result {
        found.push((
            Verdict::Breaking,
            format!(
                "signature changed: {} became {}",
                signature_text(name, &old.params, &old.result),
                signature_text(name, &new.params, &new.result)
            ),
        ));
    } else {
        for (o, n) in old.params.iter().zip(&new.params) {
            if o.name != n.name {
                found.push((
                    Verdict::NeedsReview,
                    format!(
                        "parameter `{}` renamed `{}`: callers that pass it by name break",
                        o.name, n.name
                    ),
                ));
            }
        }
    }

    let oc = contract(old);
    let nc = contract(new);

    // failures
    for f in nc.failures.difference(&oc.failures) {
        found.push((
            Verdict::NeedsReview,
            format!("new way to fail, {f}: breaking for callers that handle every failure by name"),
        ));
    }
    let gone: Vec<_> = oc.failures.difference(&nc.failures).collect();
    if !gone.is_empty() {
        found.push((Verdict::Compatible, format!("can no longer fail with {}", list(gone))));
    }

    // preconditions: more conditions can only make it stronger
    let added = missing(&nc.preconditions, &oc.preconditions);
    let removed = missing(&oc.preconditions, &nc.preconditions);
    match (added.is_empty(), removed.is_empty()) {
        (false, true) => found.push((
            Verdict::Breaking,
            format!(
                "stronger precondition: {}. Witness: any caller that does not establish it \
                 (not breaking if it follows from the other preconditions; the prover will decide)",
                list(&added)
            ),
        )),
        (false, false) => found.push((
            Verdict::NeedsReview,
            format!(
                "precondition changed from {} to {}: whether it is stronger needs the prover",
                list(&removed),
                list(&added)
            ),
        )),
        (true, false) => {
            found.push((Verdict::Compatible, format!("weaker precondition: dropped {}", list(&removed))))
        }
        (true, true) => {}
    }

    // guarantees: fewer can only make them weaker
    let mut weaker: Vec<String> = missing(&oc.guarantees, &nc.guarantees).into_iter().cloned().collect();
    match (&oc.changes_only, &nc.changes_only) {
        (Some(o), Some(n)) => {
            let wider = missing(n, o);
            if !wider.is_empty() {
                weaker.push(format!("it may now also change {}", list(wider)));
            }
        }
        (Some(o), None) => weaker.push(format!("changes only {} was dropped", list(o.values()))),
        _ => {}
    }
    if !weaker.is_empty() {
        let verdict = if dependents.is_empty() { Verdict::NeedsReview } else { Verdict::Breaking };
        let who = match dependents.len() {
            0 => "needs review for unknown callers".to_string(),
            1 => format!("breaking for {}, which relies on it", dependents[0]),
            _ => format!("breaking for {}, which rely on it", list(&dependents)),
        };
        found.push((verdict, format!("guarantee weakened or removed: {}; {who}", list(&weaker))));
    }
    let stronger = missing(&nc.guarantees, &oc.guarantees);
    if !stronger.is_empty() && weaker.is_empty() {
        found.push((Verdict::Compatible, format!("stronger guarantee: {}", list(stronger))));
    }

    // effects: a capability the module already used is not new authority
    for u in nc.uses.difference(&oc.uses) {
        if old_module_uses.contains_key(u) {
            found.push((
                Verdict::NeedsReview,
                format!("now uses {u}, which the module already used elsewhere"),
            ));
        }
    }

    // trust blocks
    let mut old_trust = Vec::new();
    let mut new_trust = Vec::new();
    trust_blocks(old_body.map_or(&old.body, |b| b), &mut old_trust);
    trust_blocks(new_body.map_or(&new.body, |b| b), &mut new_trust);
    if old_trust != new_trust {
        found.push((Verdict::NeedsReview, "a trust block is new or changed".into()));
    }

    // the body under an unchanged contract
    let body_text = |b: Option<&Vec<Stmt>>, f: &Function| -> String {
        b.map_or(&f.body, |b| b).iter().map(stmt_sexpr).collect::<Vec<_>>().join(" ")
    };
    let body_changed = body_text(old_body, old) != body_text(new_body, new);
    if found.is_empty() {
        if body_changed {
            found.push((
                Verdict::UnchangedUnchecked,
                "the body changed and the contract did not; nothing checks the new body against the contract yet"
                    .into(),
            ));
        } else {
            found.push((Verdict::Identical, String::new()));
        }
    }

    let verdict = found.iter().map(|(v, _)| *v).min().unwrap();
    Finding {
        function: name.to_string(),
        verdict,
        reasons: found.into_iter().map(|(_, r)| r).filter(|r| !r.is_empty()).collect(),
        dependents,
    }
}

impl Report {
    /// The report, as Section 12.5 lays it out: most severe first, every
    /// verdict with its reasons.
    pub fn render(&self, old_label: &str, new_label: &str) -> String {
        let mut out = String::new();
        let title = self.module.as_deref().map_or(String::new(), |m| format!("{m}: "));
        let _ = writeln!(out, "scribe impact {title}{old_label} → {new_label}");
        let order = [
            Verdict::Breaking,
            Verdict::AuthorityChange,
            Verdict::NeedsReview,
            Verdict::UnchangedUnchecked,
            Verdict::Compatible,
        ];
        for v in order {
            if v == Verdict::AuthorityChange {
                let _ = writeln!(out, "\n{} ({})", v.heading(), self.new_authority.len());
                for (cap, fs) in &self.new_authority {
                    let _ = writeln!(out, "    now uses {cap}, in {}", list(fs));
                }
                continue;
            }
            let fs: Vec<_> = self.findings.iter().filter(|f| f.verdict == v).collect();
            let _ = writeln!(out, "\n{} ({})", v.heading(), fs.len());
            for f in fs {
                let _ = writeln!(out, "    {}", f.function);
                for r in &f.reasons {
                    let _ = writeln!(out, "        {r}");
                }
                if !f.dependents.is_empty() && v != Verdict::Compatible {
                    let _ = writeln!(out, "        used by {}", list(&f.dependents));
                }
            }
        }
        let identical: Vec<_> = self
            .findings
            .iter()
            .filter(|f| f.verdict == Verdict::Identical)
            .map(|f| f.function.as_str())
            .collect();
        if !identical.is_empty() {
            let _ = writeln!(out, "\nIdentical ({})\n    {}", identical.len(), identical.join(", "));
        }
        let _ = writeln!(out, "\nSuggested version change: {}", self.suggested_version());
        out
    }

    pub fn suggested_version(&self) -> &'static str {
        if self.findings.iter().any(|f| f.verdict == Verdict::Breaking) {
            "major (a breaking change)"
        } else if self.findings.iter().any(|f| f.verdict == Verdict::Compatible) {
            "minor (compatible additions)"
        } else {
            "patch"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scribe_syntax::{LexOptions, lex, parse};

    fn file(src: &str) -> File {
        parse(&lex(src, LexOptions::default()).unwrap()).unwrap()
    }

    fn verdict_of(report: &Report, f: &str) -> Verdict {
        report.findings.iter().find(|x| x.function == f).unwrap().verdict
    }

    const OLD: &str = include_str!("../../../tests/impact/payments_old.scribe");
    const NEW: &str = include_str!("../../../tests/impact/payments_new.scribe");
    const CHECKOUT: &str = include_str!("../../../tests/impact/checkout.scribe");

    #[test]
    fn the_example_change() {
        let report = compare(&file(OLD), &file(NEW), &[file(CHECKOUT)]);
        assert_eq!(verdict_of(&report, "charge"), Verdict::NeedsReview);
        assert_eq!(verdict_of(&report, "refund"), Verdict::Breaking);
        assert_eq!(verdict_of(&report, "total_charged_to"), Verdict::Breaking);
        assert_eq!(verdict_of(&report, "receipt_text"), Verdict::UnchangedUnchecked);
        assert_eq!(verdict_of(&report, "void"), Verdict::Breaking);
        assert_eq!(verdict_of(&report, "capture"), Verdict::Compatible);
        assert_eq!(verdict_of(&report, "balance"), Verdict::Identical);
        assert_eq!(report.new_authority, vec![("audit_log".to_string(), vec!["refund".to_string()])]);
        assert_eq!(report.suggested_version(), "major (a breaking change)");
    }

    #[test]
    fn the_report_matches_the_recorded_one() {
        let report = compare(&file(OLD), &file(NEW), &[file(CHECKOUT)]);
        let expected = include_str!("../../../tests/impact/expected-report.txt");
        assert_eq!(report.render("payments_old.scribe", "payments_new.scribe"), expected);
    }

    #[test]
    fn layout_and_notes_are_not_changes() {
        let a = file("function f(x: Int) -> Int\n    return x + 1\n");
        let b = file("note the same function\nfunction f(x: Int) -> Int\n    return x + 1\n");
        assert_eq!(verdict_of(&compare(&a, &b, &[]), "f"), Verdict::Identical);
    }

    #[test]
    fn a_weaker_precondition_is_compatible() {
        let a = file("function f(x: Int) -> Int\n    expect x > 0\n    return x\n");
        let b = file("function f(x: Int) -> Int\n    return x\n");
        assert_eq!(verdict_of(&compare(&a, &b, &[]), "f"), Verdict::Compatible);
    }
}
