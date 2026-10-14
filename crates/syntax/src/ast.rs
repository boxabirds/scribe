//! The syntax tree the parser builds.
//!
//! `to_sexpr` prints a tree in the form `tools/compare_parsers.py` also builds
//! from the reference grammar's parse, so the two parsers can be compared tree
//! for tree. Symbol spellings are normalised there (`≤` is `<=`, `∧` is `and`).

#[derive(Clone, Debug, PartialEq)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Function(Function),
    Type(TypeDecl),
    Enum(EnumDecl),
    Module(Module),
    Interface(Interface),
    Stmt(Stmt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub result: Option<TypeExpr>,
    pub contract: Vec<Clause>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeExpr {
    /// `editable` or `consumed`
    pub modifiers: Vec<String>,
    /// A function type, `function(Item) -> Int`; then `name` is empty
    pub function: Option<FunctionType>,
    /// A typestate: `Open` in `Open Connection`
    pub state: Option<String>,
    pub name: String,
    /// Type parameters: `Int` in `List[Int]`
    pub args: Vec<TypeExpr>,
    pub or_nothing: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType {
    pub params: Vec<TypeExpr>,
    pub result: Option<Box<TypeExpr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub low: Expr,
    /// `until` (end excluded) or `through` (end included)
    pub word: String,
    pub high: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl {
    pub name: String,
    pub params: Vec<String>,
    pub base: Option<TypeExpr>,
    pub range: Option<Range>,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Member {
    Field(String, TypeExpr),
    Model(String, Expr),
    Invariant(LineCondition),
    States(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub name: String,
    pub params: Vec<Param>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interface {
    pub name: String,
    pub functions: Vec<Function>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub name: String,
    pub sections: Vec<ModuleSection>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleSection {
    Requires(Vec<Need>),
    Exports(Vec<Function>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Need {
    pub module: String,
    pub alias: String,
    pub uses: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clause {
    CanFail(Vec<String>),
    Expect(LineCondition),
    Ensure(Ensure),
    Changes(Vec<Expr>),
    Uses(Vec<String>),
    Example(Expr),
    Property(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ensure {
    Plain(Expr),
    OnSuccess(Expr),
    OnFailure(Vec<String>, Expr),
    If(Expr, Expr),
    Group(Group),
    IfThen(Expr, Expr),
}

/// What may follow a clause word: a condition on the line, a group, or an
/// `if` / `then` pair on two lines.
#[derive(Clone, Debug, PartialEq)]
pub enum LineCondition {
    Condition(Expr),
    Group(Group),
    IfThen(Expr, Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    /// `all`, `any` or `not any`
    pub kind: String,
    pub items: Vec<GroupItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupItem {
    Condition(Expr),
    Group(Group),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Expr(Expr),
    Group(Group),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    Expr(Expr),
    Group(Group),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForIter {
    Range(Range),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Let { name: String, value: Value, fallback: Option<Expr> },
    Var { name: String, value: Expr },
    Assign { target: Expr, value: Expr },
    Return(Option<Value>),
    Fail(Expr),
    DoNothing,
    If { branches: Vec<(Condition, Vec<Stmt>)>, otherwise: Option<Vec<Stmt>> },
    For { name: String, iter: ForIter, body: Vec<Stmt> },
    While { condition: Condition, body: Vec<Stmt> },
    Invariant(LineCondition),
    Decreases(Expr),
    Match { subject: Expr, cases: Vec<Case> },
    Handle { body: Vec<Stmt>, handlers: Vec<Handler> },
    Hold { name: String, value: Expr, body: Vec<Stmt> },
    Together(Vec<Stmt>),
    Break,
    Continue,
    Trust(Vec<Stmt>),
    Assert(LineCondition),
    Elided,
    Expr(Expr),
    Try(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    pub variant: String,
    pub bindings: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handler {
    pub failures: Vec<String>,
    pub binding: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Domain {
    Range(Range),
    Expr(Expr),
    Type(TypeExpr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    ForAll {
        var: String,
        domain: Box<Domain>,
        condition: Box<Expr>,
    },
    Exists {
        var: String,
        domain: Box<Expr>,
        condition: Box<Expr>,
    },
    Not(Box<Expr>),
    And(Vec<Expr>),
    Or(Vec<Expr>),
    /// `a < b <= c`: the first operand, then each operator with its operand
    Compare(Box<Expr>, Vec<(String, Expr)>),
    Binary(String, Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Try(Box<Expr>),
    Field(Box<Expr>, String),
    Call(Box<Expr>, Vec<Arg>),
    Index(Box<Expr>, Box<Expr>),
    Name(String),
    /// A number, quantity, hex or binary literal, or a string, as written
    Literal(String),
    List(Vec<Expr>),
    Comprehension {
        element: Box<Expr>,
        var: String,
        source: Box<Expr>,
        filter: Option<Box<Expr>>,
    },
    Result,
    Nothing,
    Old(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub name: Option<String>,
    pub value: Expr,
}

// ---------------------------------------------------------------- printing

pub fn to_sexpr(file: &File) -> String {
    let mut out = String::from("(file");
    for item in &file.items {
        out.push(' ');
        out.push_str(&item_sexpr(item));
    }
    out.push(')');
    out
}

fn list(head: &str, parts: impl IntoIterator<Item = String>) -> String {
    let mut out = format!("({head}");
    for p in parts {
        out.push(' ');
        out.push_str(&p);
    }
    out.push(')');
    out
}

fn names(head: &str, names: &[String]) -> String {
    list(head, names.iter().cloned())
}

fn item_sexpr(item: &Item) -> String {
    match item {
        Item::Function(f) => function_sexpr(f),
        Item::Type(t) => list(
            "typedecl",
            [
                t.name.clone(),
                names("params", &t.params),
                t.base.as_ref().map_or("-".into(), type_sexpr),
                t.range.as_ref().map_or("-".into(), range_sexpr),
                list("members", t.members.iter().map(member_sexpr)),
            ],
        ),
        Item::Enum(e) => list(
            "enum",
            std::iter::once(e.name.clone()).chain(e.variants.iter().map(|v| {
                list("variant", std::iter::once(v.name.clone()).chain(v.params.iter().map(param_sexpr)))
            })),
        ),
        Item::Interface(i) => {
            list("interface", std::iter::once(i.name.clone()).chain(i.functions.iter().map(function_sexpr)))
        }
        Item::Module(m) => list(
            "module",
            std::iter::once(m.name.clone()).chain(m.sections.iter().map(|s| match s {
                ModuleSection::Requires(needs) => list(
                    "requires",
                    needs.iter().map(|n| {
                        let mut parts = vec![n.module.clone(), n.alias.clone()];
                        if !n.uses.is_empty() {
                            parts.push(names("uses", &n.uses));
                        }
                        list("need", parts)
                    }),
                ),
                ModuleSection::Exports(fs) => list("exports", fs.iter().map(function_sexpr)),
            })),
        ),
        Item::Stmt(s) => stmt_sexpr(s),
    }
}

fn function_sexpr(f: &Function) -> String {
    list(
        "function",
        [
            f.name.clone(),
            list("params", f.params.iter().map(param_sexpr)),
            f.result.as_ref().map_or("-".into(), type_sexpr),
            list("contract", f.contract.iter().map(clause_sexpr)),
            block_sexpr(&f.body),
        ],
    )
}

fn param_sexpr(p: &Param) -> String {
    list("param", [p.name.clone(), type_sexpr(&p.ty)])
}

pub fn type_sexpr(t: &TypeExpr) -> String {
    let mut parts: Vec<String> = t.modifiers.clone();
    if let Some(f) = &t.function {
        parts.push(list(
            "function",
            [
                list("params", f.params.iter().map(type_sexpr)),
                f.result.as_ref().map_or("-".into(), |r| type_sexpr(r)),
            ],
        ));
        return list("type", parts);
    }
    if let Some(s) = &t.state {
        parts.push(format!("state={s}"));
    }
    parts.push(t.name.clone());
    parts.extend(t.args.iter().map(type_sexpr));
    if t.or_nothing {
        parts.push("?".into());
    }
    list("type", parts)
}

fn range_sexpr(r: &Range) -> String {
    list("range", [expr_sexpr(&r.low), r.word.clone(), expr_sexpr(&r.high)])
}

fn member_sexpr(m: &Member) -> String {
    match m {
        Member::Field(n, t) => list("field", [n.clone(), type_sexpr(t)]),
        Member::Model(n, e) => list("model", [n.clone(), expr_sexpr(e)]),
        Member::Invariant(lc) => list("invariant", [line_condition_sexpr(lc)]),
        Member::States(s) => names("states", s),
    }
}

pub fn clause_sexpr(c: &Clause) -> String {
    match c {
        Clause::CanFail(n) => names("can-fail", n),
        Clause::Expect(lc) => list("expect", [line_condition_sexpr(lc)]),
        Clause::Ensure(e) => list(
            "ensure",
            [match e {
                Ensure::Plain(x) => expr_sexpr(x),
                Ensure::OnSuccess(x) => list("on-success", [expr_sexpr(x)]),
                Ensure::OnFailure(n, x) => list("on-failure", [names("failures", n), expr_sexpr(x)]),
                Ensure::If(a, b) => list("if", [expr_sexpr(a), expr_sexpr(b)]),
                Ensure::Group(g) => group_sexpr(g),
                Ensure::IfThen(a, b) => list("if-then", [expr_sexpr(a), expr_sexpr(b)]),
            }],
        ),
        Clause::Changes(es) => list("changes", es.iter().map(expr_sexpr)),
        Clause::Uses(n) => names("uses", n),
        Clause::Example(e) => list("example", [expr_sexpr(e)]),
        Clause::Property(e) => list("property", [expr_sexpr(e)]),
    }
}

pub fn line_condition_sexpr(lc: &LineCondition) -> String {
    match lc {
        LineCondition::Condition(e) => expr_sexpr(e),
        LineCondition::Group(g) => group_sexpr(g),
        LineCondition::IfThen(a, b) => list("if-then", [expr_sexpr(a), expr_sexpr(b)]),
    }
}

fn group_sexpr(g: &Group) -> String {
    list(
        &format!("group {}", g.kind.replace(' ', "-")),
        g.items.iter().map(|i| match i {
            GroupItem::Condition(e) => expr_sexpr(e),
            GroupItem::Group(g) => group_sexpr(g),
        }),
    )
}

fn condition_sexpr(c: &Condition) -> String {
    match c {
        Condition::Expr(e) => expr_sexpr(e),
        Condition::Group(g) => group_sexpr(g),
    }
}

pub fn block_sexpr(stmts: &[Stmt]) -> String {
    list("block", stmts.iter().map(stmt_sexpr))
}

pub fn stmt_sexpr(s: &Stmt) -> String {
    match s {
        Stmt::Let { name, value, fallback } => {
            let mut parts = vec![name.clone(), value_sexpr(value)];
            if let Some(f) = fallback {
                parts.push(list("else", [expr_sexpr(f)]));
            }
            list("let", parts)
        }
        Stmt::Var { name, value } => list("var", [name.clone(), expr_sexpr(value)]),
        Stmt::Assign { target, value } => list("assign", [expr_sexpr(target), expr_sexpr(value)]),
        Stmt::Return(v) => list("return", v.iter().map(value_sexpr)),
        Stmt::Fail(e) => list("fail", [expr_sexpr(e)]),
        Stmt::DoNothing => "(do-nothing)".into(),
        Stmt::If { branches, otherwise } => list(
            "if",
            branches
                .iter()
                .map(|(c, b)| list("branch", [condition_sexpr(c), block_sexpr(b)]))
                .chain(otherwise.iter().map(|b| list("else", [block_sexpr(b)]))),
        ),
        Stmt::For { name, iter, body } => list(
            "for",
            [
                name.clone(),
                match iter {
                    ForIter::Range(r) => range_sexpr(r),
                    ForIter::Expr(e) => expr_sexpr(e),
                },
                block_sexpr(body),
            ],
        ),
        Stmt::While { condition, body } => list("while", [condition_sexpr(condition), block_sexpr(body)]),
        Stmt::Invariant(lc) => list("invariant", [line_condition_sexpr(lc)]),
        Stmt::Decreases(e) => list("decreases", [expr_sexpr(e)]),
        Stmt::Match { subject, cases } => list(
            "match",
            std::iter::once(expr_sexpr(subject)).chain(cases.iter().map(|c| {
                list(
                    "case",
                    std::iter::once(c.variant.clone())
                        .chain(c.bindings.iter().cloned())
                        .chain(std::iter::once(block_sexpr(&c.body))),
                )
            })),
        ),
        Stmt::Handle { body, handlers } => list(
            "handle",
            std::iter::once(block_sexpr(body)).chain(handlers.iter().map(|h| {
                list(
                    "on-failure",
                    [
                        names("failures", &h.failures),
                        h.binding.clone().unwrap_or("-".into()),
                        block_sexpr(&h.body),
                    ],
                )
            })),
        ),
        Stmt::Hold { name, value, body } => {
            list("hold", [name.clone(), expr_sexpr(value), block_sexpr(body)])
        }
        Stmt::Together(b) => list("together", [block_sexpr(b)]),
        Stmt::Break => "(break)".into(),
        Stmt::Continue => "(continue)".into(),
        Stmt::Trust(b) => list("trust", [block_sexpr(b)]),
        Stmt::Assert(lc) => list("assert", [line_condition_sexpr(lc)]),
        Stmt::Elided => "(elided)".into(),
        Stmt::Expr(e) => list("expr", [expr_sexpr(e)]),
        Stmt::Try(e) => list("try-stmt", [expr_sexpr(e)]),
    }
}

fn value_sexpr(v: &Value) -> String {
    match v {
        Value::Expr(e) => expr_sexpr(e),
        Value::Group(g) => group_sexpr(g),
    }
}

pub fn expr_sexpr(e: &Expr) -> String {
    match e {
        Expr::ForAll { var, domain, condition } => list(
            "forall",
            [
                var.clone(),
                match domain.as_ref() {
                    Domain::Range(r) => range_sexpr(r),
                    Domain::Expr(e) => expr_sexpr(e),
                    Domain::Type(t) => type_sexpr(t),
                },
                expr_sexpr(condition),
            ],
        ),
        Expr::Exists { var, domain, condition } => {
            list("exists", [var.clone(), expr_sexpr(domain), expr_sexpr(condition)])
        }
        Expr::Not(e) => list("not", [expr_sexpr(e)]),
        Expr::And(es) => list("and", es.iter().map(expr_sexpr)),
        Expr::Or(es) => list("or", es.iter().map(expr_sexpr)),
        Expr::Compare(first, rest) => list(
            "compare",
            std::iter::once(expr_sexpr(first))
                .chain(rest.iter().flat_map(|(op, e)| [op.clone(), expr_sexpr(e)])),
        ),
        Expr::Binary(op, a, b) => list(op, [expr_sexpr(a), expr_sexpr(b)]),
        Expr::Neg(e) => list("neg", [expr_sexpr(e)]),
        Expr::Try(e) => list("try", [expr_sexpr(e)]),
        Expr::Field(e, n) => list("field", [expr_sexpr(e), n.clone()]),
        Expr::Call(f, args) => list(
            "call",
            std::iter::once(expr_sexpr(f)).chain(args.iter().map(|a| match &a.name {
                Some(n) => list("named", [n.clone(), expr_sexpr(&a.value)]),
                None => expr_sexpr(&a.value),
            })),
        ),
        Expr::Index(e, i) => list("index", [expr_sexpr(e), expr_sexpr(i)]),
        Expr::Name(n) => n.clone(),
        Expr::Literal(l) => l.clone(),
        Expr::List(es) => list("list", es.iter().map(expr_sexpr)),
        Expr::Comprehension { element, var, source, filter } => list(
            "comprehension",
            [
                expr_sexpr(element),
                var.clone(),
                expr_sexpr(source),
                filter.as_ref().map_or("-".into(), |f| expr_sexpr(f)),
            ],
        ),
        Expr::Result => "result".into(),
        Expr::Nothing => "nothing".into(),
        Expr::Old(e) => list("old", [expr_sexpr(e)]),
    }
}

// ---------------------------------------------------------------- source text
//
// The stored form of expressions, types and clauses, for reports and
// diagnostics. Brackets are written only where arithmetic needs them.

pub fn type_text(t: &TypeExpr) -> String {
    let mut out = String::new();
    for m in &t.modifiers {
        out.push_str(m);
        out.push(' ');
    }
    if let Some(f) = &t.function {
        out.push_str("function(");
        out.push_str(&f.params.iter().map(type_text).collect::<Vec<_>>().join(", "));
        out.push(')');
        if let Some(r) = &f.result {
            out.push_str(" -> ");
            out.push_str(&type_text(r));
        }
        return out;
    }
    if let Some(s) = &t.state {
        out.push_str(s);
        out.push(' ');
    }
    out.push_str(&t.name);
    if !t.args.is_empty() {
        out.push('[');
        out.push_str(&t.args.iter().map(type_text).collect::<Vec<_>>().join(", "));
        out.push(']');
    }
    if t.or_nothing {
        out.push_str(" or nothing");
    }
    out
}

pub fn signature_text(name: &str, params: &[Param], result: &Option<TypeExpr>) -> String {
    let ps: Vec<String> = params.iter().map(|p| format!("{}: {}", p.name, type_text(&p.ty))).collect();
    match result {
        Some(r) => format!("{name}({}) -> {}", ps.join(", "), type_text(r)),
        None => format!("{name}({})", ps.join(", ")),
    }
}

pub fn is_bit_operator(op: &str) -> bool {
    matches!(op, "&" | "|" | "^" | "<<" | ">>")
}

/// How tightly an expression binds. A bit operation binds least of all, so it
/// is always bracketed inside any other operation (Section 3.1).
fn precedence(e: &Expr) -> u8 {
    match e {
        Expr::Binary(op, _, _) if is_bit_operator(op) => 0,
        Expr::Binary(op, _, _) if op == "+" || op == "-" => 1,
        Expr::Binary(_, _, _) => 2,
        _ => 3,
    }
}

fn operand_text(e: &Expr, min: u8) -> String {
    if precedence(e) < min { format!("({})", expr_text(e)) } else { expr_text(e) }
}

pub fn expr_text(e: &Expr) -> String {
    match e {
        Expr::ForAll { var, domain, condition } => {
            let d = match domain.as_ref() {
                Domain::Range(r) => format!("in {}", range_text(r)),
                Domain::Expr(e) => format!("in {}", expr_text(e)),
                Domain::Type(t) => format!(": {}", type_text(t)),
            };
            format!("for all {var} {d}, {}", expr_text(condition))
        }
        Expr::Exists { var, domain, condition } => {
            format!("there is {var} in {} where {}", expr_text(domain), expr_text(condition))
        }
        Expr::Not(e) => format!("not {}", expr_text(e)),
        Expr::And(es) => es.iter().map(expr_text).collect::<Vec<_>>().join(" and "),
        Expr::Or(es) => es.iter().map(expr_text).collect::<Vec<_>>().join(" or "),
        Expr::Compare(first, rest) => {
            let mut out = operand_text(first, 1);
            for (op, e) in rest {
                let op = match op.as_str() {
                    "is-not" => "is not",
                    "not-in" => "not in",
                    o => o,
                };
                out.push_str(&format!(" {op} {}", operand_text(e, 1)));
            }
            out
        }
        Expr::Binary(op, a, b) if is_bit_operator(op) => {
            // a chain of one bit operator; any other operation is bracketed
            let left = match a.as_ref() {
                Expr::Binary(o, _, _) if o == op => expr_text(a),
                _ => operand_text(a, 3),
            };
            format!("{left} {op} {}", operand_text(b, 3))
        }
        Expr::Binary(op, a, b) => {
            let p = precedence(e);
            // the right operand of `-`, `/` and `%` needs brackets at equal precedence
            let right_min = if op == "+" || op == "*" { p } else { p + 1 };
            format!("{} {op} {}", operand_text(a, p), operand_text(b, right_min))
        }
        Expr::Neg(e) => format!("-{}", operand_text(e, 3)),
        Expr::Try(e) => format!("try {}", operand_text(e, 3)),
        Expr::Field(e, n) => format!("{}.{n}", operand_text(e, 3)),
        Expr::Call(f, args) => {
            let a: Vec<String> = args
                .iter()
                .map(|a| match &a.name {
                    Some(n) => format!("{n}: {}", expr_text(&a.value)),
                    None => expr_text(&a.value),
                })
                .collect();
            format!("{}({})", operand_text(f, 3), a.join(", "))
        }
        Expr::Index(e, i) => format!("{}[{}]", operand_text(e, 3), expr_text(i)),
        Expr::Name(n) | Expr::Literal(n) => n.clone(),
        Expr::List(es) => format!("[{}]", es.iter().map(expr_text).collect::<Vec<_>>().join(", ")),
        Expr::Comprehension { element, var, source, filter } => {
            let w = filter.as_ref().map_or(String::new(), |f| format!(" where {}", expr_text(f)));
            format!("[{} for {var} in {}{w}]", expr_text(element), expr_text(source))
        }
        Expr::Result => "result".into(),
        Expr::Nothing => "nothing".into(),
        Expr::Old(e) => format!("old({})", expr_text(e)),
    }
}

fn range_text(r: &Range) -> String {
    format!("{} {} {}", expr_text(&r.low), r.word, expr_text(&r.high))
}

/// A group on one line, as `all of a, b`; nested groups in brackets of words.
pub fn group_text(g: &Group) -> String {
    let items: Vec<String> = g
        .items
        .iter()
        .map(|i| match i {
            GroupItem::Condition(e) => expr_text(e),
            GroupItem::Group(g) => format!("({})", group_text(g)),
        })
        .collect();
    format!("{} of {}", g.kind, items.join(", "))
}

pub fn line_condition_text(lc: &LineCondition) -> String {
    match lc {
        LineCondition::Condition(e) => expr_text(e),
        LineCondition::Group(g) => group_text(g),
        LineCondition::IfThen(a, b) => format!("if {} then {}", expr_text(a), expr_text(b)),
    }
}

pub fn clause_text(c: &Clause) -> String {
    match c {
        Clause::CanFail(n) => format!("can fail with {}", n.join(", ")),
        Clause::Expect(lc) => format!("expect {}", line_condition_text(lc)),
        Clause::Ensure(e) => match e {
            Ensure::Plain(x) => format!("ensure {}", expr_text(x)),
            Ensure::OnSuccess(x) => format!("ensure on success, {}", expr_text(x)),
            Ensure::OnFailure(n, x) if n.is_empty() => format!("ensure on failure, {}", expr_text(x)),
            Ensure::OnFailure(n, x) => format!("ensure on failure {}, {}", n.join(" or "), expr_text(x)),
            Ensure::If(a, b) => format!("ensure if {}, {}", expr_text(a), expr_text(b)),
            Ensure::Group(g) => format!("ensure {}", group_text(g)),
            Ensure::IfThen(a, b) => format!("ensure if {} then {}", expr_text(a), expr_text(b)),
        },
        Clause::Changes(es) => {
            format!("changes only {}", es.iter().map(expr_text).collect::<Vec<_>>().join(", "))
        }
        Clause::Uses(n) => format!("uses {}", n.join(", ")),
        Clause::Example(e) => format!("example {}", expr_text(e)),
        Clause::Property(e) => format!("property {}", expr_text(e)),
    }
}

#[cfg(test)]
mod text_tests {
    use super::*;
    use crate::lexer::{LexOptions, lex};
    use crate::parser::parse;

    /// Parsing the printed text gives the same tree back.
    fn round_trip(line: &str) {
        let src = format!("let x = {line}\n");
        let tree = parse(&lex(&src, LexOptions::default()).unwrap()).unwrap();
        let Item::Stmt(Stmt::Let { value: Value::Expr(e), .. }) = &tree.items[0] else { panic!() };
        assert_eq!(expr_text(e), line);
    }

    #[test]
    fn expressions_print_as_written() {
        round_trip("(a + b) * c - d / (e - f)");
        round_trip("a - (b - c)");
        round_trip("orders[0].lines[2].price + old(account.balance)");
        round_trip("move(amount, from: source, to: target)");
        round_trip("[o.total for o in orders where o.status is Paid]");
        round_trip("0 <= low <= high");
        round_trip("found is not nothing");
        round_trip("x | (to_u64(b & 0x7f) << shift)");
        round_trip("a & b & c");
        round_trip("(x & mask) == 0");
        round_trip("true");
    }
}
