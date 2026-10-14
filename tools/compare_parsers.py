"""Check that the compiler's parser and the reference grammar agree, tree for
tree, on every test file, every corpus file and every code block in the
specification.

usage: python3 tools/compare_parsers.py [--verbose]

Builds the `scribe` command with cargo and runs `scribe parse --oracle`, which
prints each tree as an s-expression. Parses each file with the reference grammar
(grammar/scribe.lark, every reading kept) and converts its tree to the same
s-expression. A file must give the same tree from both, or be rejected by both.
Exits non-zero on any difference.
"""
import os
import subprocess
import sys
import tempfile

from lark import Token, Tree
from lark.exceptions import LarkError

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))

from check_grammar import CORPUS, SPEC, TESTS, scribe_files  # noqa: E402
from pre import LayoutError, preprocess  # noqa: E402
from run3 import load  # noqa: E402
from spec_blocks import blocks  # noqa: E402

GRAMMAR = os.path.join(ROOT, "grammar", "scribe.lark")
LAYOUT = {"_NL", "_INDENT", "_DEDENT", "LINE_COMMA"}
CLAUSES = {"fails", "expect", "ensure", "ensure_group", "ensure_if_then", "changes",
           "uses_clause", "example", "property"}
DECLARATIONS = {"name", "param", "typeexpr"}
COMPARISONS = {"≠": "!=", "≤": "<=", "≥": ">=", "∈": "in", "is not": "is-not", "not in": "not-in"}


class Ambiguous(Exception):
    pass


def kids(t):
    """A tree's children without layout tokens and empty optional slots."""
    return [c for c in t.children
            if c is not None and not (isinstance(c, Token) and c.type in LAYOUT)]


LITERALS = {"INT", "DECIMAL", "STRING", "QUANTITY", "HEX", "BINARY"}


def trees(t, *names):
    """A tree's subtrees, or those with the given rule names. Without names,
    literal tokens count too, since they stand where an expression would."""
    if names:
        return [c for c in kids(t) if isinstance(c, Tree) and c.data in names]
    return [c for c in kids(t) if isinstance(c, Tree) or c.type in LITERALS]


def data(x):
    return x.data if isinstance(x, Tree) else None


def words(t):
    return [c.value for c in kids(t) if isinstance(c, Token)]


def sx(head, *parts):
    return "(" + " ".join([head, *parts]) + ")"


def name(t):
    return words(t)[0]


def name_list(t):
    return [name(n) for n in trees(t, "name")]


def typeexpr(t):
    parts = [words(m)[0] for m in trees(t, "modifier")]
    for s in trees(t, "type_state"):
        parts.append("state=" + name(trees(s, "name")[0]))
    tn = trees(t, "type_name")[0]
    parts.append(name(trees(tn, "name")[0]))
    parts += [typeexpr(a) for a in trees(tn, "typeexpr")]
    if "nothing" in words(t):
        parts.append("?")
    return sx("type", *parts)


def range_(t):
    low, word, high = kids(t)
    return sx("range", expr(low), words(word)[0], expr(high))


def group(t):
    kind = "-".join(words(trees(t, "group_kind")[0]))
    items = []
    for c in trees(t):
        if c.data == "group_kind":
            continue
        if c.data == "inline_group":
            items += [expr(i) for i in kids(c) if not (isinstance(i, Token) and i.value == ",")]
            continue
        while data(c) in ("group_item", "group_last"):
            c = trees(c)[0]
        items.append(group(c) if data(c) == "group" else expr(c))
    return sx("group " + kind, *items)


def line_condition(t):
    if isinstance(t, Tree):
        if t.data == "group":
            return group(t)
        if t.data == "if_then":
            a, b = trees(t)
            return sx("if-then", expr(a), expr(b))
        if t.data == "line_condition":
            return line_condition(kids(t)[0])
    return expr(t)


def expr(t):
    if isinstance(t, Token):
        return t.value
    d, c = t.data, kids(t)
    if d == "_ambig":
        raise Ambiguous()
    if d in ("name", "line_condition", "atom") and len(trees(t)) == 1 and d != "name":
        return expr(trees(t)[0])
    if d == "name":
        return name(t)
    if d == "quantified":
        cs = trees(t)
        var = name(cs[1])
        dom = cs[2]
        domain = range_(dom) if data(dom) == "range" else typeexpr(dom) if data(dom) == "typeexpr" else expr(dom)
        return sx("forall", var, domain, expr(cs[3]))
    if d == "exists":
        cs = trees(t)
        return sx("exists", name(cs[0]), expr(cs[1]), expr(cs[2]))
    if d == "not_":
        return sx("not", expr(trees(t)[-1]))
    if d in ("and_chain", "or_chain"):
        ops = [x for x in trees(t) if data(x) not in ("and_op", "or_op")]
        return sx("and" if d == "and_chain" else "or", *[expr(x) for x in ops])
    if d == "compare":
        parts = []
        for x in c:
            if isinstance(x, Tree) and x.data == "comp_op":
                op = " ".join(words(x))
                parts.append(COMPARISONS.get(op, op))
            else:
                parts.append(expr(x))
        return sx("compare", *parts)
    if d in ("sum", "product"):
        left = expr(c[0])
        for i in range(1, len(c), 2):
            left = sx(c[i].value, left, expr(c[i + 1]))
        return left
    if d == "neg":
        return sx("neg", expr(c[-1]))
    if d == "try_":
        return sx("try", expr(c[-1]))
    if d == "field_access":
        return sx("field", expr(c[0]), name(c[-1]))
    if d == "call":
        args = []
        for a in c[1:]:
            if isinstance(a, Token) and a.type not in LITERALS:
                continue
            if data(a) == "named_arg":
                n, v = trees(a)
                args.append(sx("named", name(n), expr(v)))
            else:
                args.append(expr(a))
        return sx("call", expr(c[0]), *args)
    if d == "index":
        return sx("index", expr(c[0]), expr(c[2]))
    if d == "list":
        return sx("list", *[expr(x) for x in c if not (isinstance(x, Token) and x.value in "[],")])
    if d == "comprehension":
        cs = [x for x in c if not (isinstance(x, Token) and x.type != "NAME")]
        element, var, source = expr(cs[0]), name(cs[1]), expr(cs[2])
        return sx("comprehension", element, var, source, expr(cs[3]) if len(cs) > 3 else "-")
    if d == "result":
        return "result"
    if d == "nothing":
        return "nothing"
    if d == "old":
        return sx("old", expr(trees(t)[0]))
    if d == "atom":
        return expr(trees(t)[0])
    raise ValueError(f"no conversion for expression rule {d}")


def block(t):
    return sx("block", *[stmt(s) for s in trees(t)])


def condition_or_group(t):
    return group(t) if data(t) == "group" else expr(t)


def stmt(t):
    d = t.data
    cs = trees(t)
    if d == "let":
        value = expr(cs[1])
        return sx("let", name(cs[0]), value, *([sx("else", expr(cs[2]))] if len(cs) > 2 else []))
    if d == "let_group":
        return sx("let", name(cs[0]), group(cs[1]))
    if d == "var":
        return sx("var", name(cs[0]), expr(cs[1]))
    if d == "assign":
        return sx("assign", expr(cs[0]), expr(cs[1]))
    if d == "return_stmt":
        return sx("return", *[expr(x) for x in cs])
    if d == "return_group":
        return sx("return", group(cs[0]))
    if d == "fail":
        return sx("fail", expr(cs[0]))
    if d == "do_nothing":
        return "(do-nothing)"
    if d == "if_stmt":
        parts = []
        for x in cs:
            if x.data == "if_body":
                cond, body = trees(x)
                parts.append(sx("branch", condition_or_group(cond), block(body)))
            else:
                parts.append(sx("else", block(x)))
        return sx("if", *parts)
    if d == "for_stmt":
        it = cs[1]
        return sx("for", name(cs[0]), range_(it) if data(it) == "range" else expr(it), block(cs[2]))
    if d == "while_stmt":
        return sx("while", condition_or_group(cs[0]), block(cs[1]))
    if d == "invariant":
        return sx("invariant", line_condition(kids(t)[1]))
    if d == "assert_stmt":
        return sx("assert", line_condition(kids(t)[1]))
    if d == "decreases":
        return sx("decreases", expr(cs[0]))
    if d == "match":
        cases = []
        for case in cs[1:]:
            pattern, body = trees(case)
            cases.append(sx("case", *name_list(pattern), block(body)))
        return sx("match", expr(cs[0]), *cases)
    if d == "handle":
        handlers = []
        for h in cs[1:]:
            c = kids(h)
            names, binding = [], "-"
            for i, x in enumerate(c):
                if isinstance(x, Tree) and x.data == "name":
                    if isinstance(c[i - 1], Token) and c[i - 1].value == "as":
                        binding = name(x)
                    else:
                        names.append(name(x))
            handlers.append(sx("on-failure", sx("failures", *names), binding, block(trees(h, "block")[0])))
        return sx("handle", block(cs[0]), *handlers)
    if d == "hold":
        return sx("hold", name(cs[0]), expr(cs[1]), block(cs[2]))
    if d == "together":
        return sx("together", block(cs[0]))
    if d == "trust":
        return sx("trust", block(cs[0]))
    if d == "break_stmt":
        return "(break)"
    if d == "continue_stmt":
        return "(continue)"
    if d == "elided":
        return "(elided)"
    if d == "expr_stmt":
        return sx("expr", expr(cs[0]))
    if d == "try_":
        return sx("try-stmt", expr(cs[0]))
    raise ValueError(f"no conversion for statement rule {d}")


def clause(t):
    d, cs = t.data, trees(t)
    if d == "fails":
        return sx("can-fail", *name_list(cs[0]))
    if d == "expect":
        return sx("expect", line_condition(kids(t)[1]))
    if d == "ensure":
        body = cs[0]
        if data(body) == "on_success":
            inner = sx("on-success", expr(trees(body)[0]))
        elif data(body) == "on_failure_case":
            bs = trees(body)
            inner = sx("on-failure", sx("failures", *[name(n) for n in bs[:-1]]), expr(bs[-1]))
        elif data(body) == "conditional":
            a, b = trees(body)
            inner = sx("if", expr(a), expr(b))
        else:
            inner = expr(trees(body)[0])
        return sx("ensure", inner)
    if d == "ensure_group":
        return sx("ensure", group(cs[0]))
    if d == "ensure_if_then":
        a, b = trees(cs[0])
        return sx("ensure", sx("if-then", expr(a), expr(b)))
    if d == "changes":
        return sx("changes", *[expr(x) for x in cs])
    if d == "uses_clause":
        return sx("uses", *name_list(cs[0]))
    if d == "example":
        return sx("example", expr(cs[0]))
    if d == "property":
        return sx("property", expr(cs[0]))
    raise ValueError(f"no conversion for clause {d}")


def function(t):
    cs = trees(t)
    params = [sx("param", name(trees(p, "name")[0]), typeexpr(trees(p, "typeexpr")[0])) for p in trees(t, "param")]
    results = trees(t, "typeexpr")
    clauses = [clause(x) for x in cs if data(x) in CLAUSES]
    body = [stmt(x) for x in cs if data(x) not in CLAUSES | DECLARATIONS]
    return sx("function", name(cs[0]), sx("params", *params),
              typeexpr(results[0]) if results else "-", sx("contract", *clauses), sx("block", *body))


def member(t):
    d, cs = t.data, trees(t)
    if d == "field":
        return sx("field", name(cs[0]), typeexpr(cs[1]))
    if d == "model":
        return sx("model", name(cs[0]), expr(cs[1]))
    if d == "type_invariant":
        return sx("invariant", line_condition(kids(t)[1]))
    if d == "states":
        return sx("states", *name_list(t))
    raise ValueError(f"no conversion for member {d}")


def item(t):
    d, cs = t.data, trees(t)
    if d == "function":
        return function(t)
    if d == "typedecl":
        names = trees(t, "name")
        base = trees(t, "typeexpr")
        ranges = trees(t, "range")
        members = [member(x) for x in cs if data(x) in ("field", "model", "type_invariant", "states")]
        return sx("typedecl", name(names[0]), sx("params", *[name(n) for n in names[1:]]),
                  typeexpr(base[0]) if base else "-", range_(ranges[0]) if ranges else "-",
                  sx("members", *members))
    if d == "enum":
        variants = []
        for v in trees(t, "variant"):
            params = [sx("param", name(trees(p, "name")[0]), typeexpr(trees(p, "typeexpr")[0]))
                      for p in trees(v, "param")]
            variants.append(sx("variant", name(trees(v, "name")[0]), *params))
        return sx("enum", name(cs[0]), *variants)
    if d == "interface":
        return sx("interface", name(cs[0]), *[function(f) for f in trees(t, "function")])
    if d == "module":
        sections = []
        for s in cs[1:]:
            if s.data == "requires":
                needs = []
                for n in trees(s, "need"):
                    names = trees(n, "name")
                    parts = [name(names[0]), name(names[1])]
                    uses = trees(n, "name_list")
                    if uses:
                        parts.append(sx("uses", *name_list(uses[0])))
                    needs.append(sx("need", *parts))
                sections.append(sx("requires", *needs))
            else:
                sections.append(sx("exports", *[function(f) for f in trees(s, "function")]))
        return sx("module", name(cs[0]), *sections)
    return stmt(t)


def oracle_tree(parser, text):
    try:
        tree = parser.parse(preprocess(text))
    except (LayoutError, LarkError):
        return "error"
    if any(n.data == "_ambig" for n in tree.iter_subtrees()):
        return "ambiguous"
    return sx("file", *[item(x) for x in trees(tree)])


def compiler_trees(paths, flags):
    exe = os.path.join(ROOT, "target", "debug", "scribe")
    out = subprocess.run([exe, "parse", "--oracle", *flags, *paths], capture_output=True, text=True)
    found, current = {}, None
    for line in out.stdout.split("\n"):
        if line.startswith("== "):
            current = line[3:]
        elif current is not None and line:
            found[current] = "error" if line.startswith("error ") else line
            current = None
    return found


def main():
    verbose = "--verbose" in sys.argv[1:]
    subprocess.run(["cargo", "build", "-q", "-p", "scribe-cli"], cwd=ROOT, check=True)
    parser = load(GRAMMAR, keep_all_tokens=True)
    differences = 0
    with tempfile.TemporaryDirectory() as tmp:
        cases = []
        for group_name in ("accept", "reject"):
            for f in scribe_files(os.path.join(TESTS, group_name)):
                cases.append((f"tests/grammar/{group_name}/{os.path.basename(f)}", f, []))
        for f in scribe_files(CORPUS):
            cases.append((f"tests/corpus/{os.path.basename(f)}", f, []))
        for line, text in blocks(open(SPEC).read()):
            path = os.path.join(tmp, f"spec-{line:04d}.scribe")
            open(path, "w").write(text)
            cases.append((f"spec block at line {line}", path, ["--fragment"]))
        compiled = {}
        for flags in ([], ["--fragment"]):
            paths = [p for _, p, f in cases if f == flags]
            compiled.update(compiler_trees(paths, flags))
        for label, path, _ in cases:
            want = oracle_tree(parser, open(path).read())
            got = compiled.get(path, "missing")
            if want == got:
                if verbose:
                    print(f"same     {label}")
                continue
            differences += 1
            k = next((i for i, (a, b) in enumerate(zip(want, got)) if a != b), min(len(want), len(got)))
            print(f"DIFFERS  {label}\n    reference …{want[max(0, k - 60):k + 60]}\n    compiler  …{got[max(0, k - 60):k + 60]}")
        print(f"{len(cases) - differences} of {len(cases)} files parse the same")
    return 1 if differences else 0


if __name__ == "__main__":
    sys.exit(main())
