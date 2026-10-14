"""Experiment 3 runner: parse a corpus with a general (Earley) parser that keeps
every reading, and report failures and ambiguities per file.

usage: python3 run3.py GRAMMAR FILE_OR_DIR... [--json OUT]
"""
import json
import os
import sys
from collections import Counter

from lark import Lark, Tree
from lark.exceptions import UnexpectedInput

sys.path.insert(0, os.path.dirname(__file__))
from pre import preprocess, interpolations, LayoutError  # noqa: E402


def load(grammar_path, start="start", keep_all_tokens=False):
    return Lark(open(grammar_path).read(), parser="earley", lexer="basic",
                ambiguity="explicit", propagate_positions=True, start=start,
                keep_all_tokens=keep_all_tokens)


# ---------------------------------------------------------------- disambiguation rules
# Each rule is one sentence a reader can be told. Each resolution is counted.

def _where_end(t):
    ends = [n.meta.end_pos for n in t.iter_subtrees()
            if n.data == "where_cond" and not n.meta.empty]
    return max(ends) if ends else -1


def _quant_end(t):
    ends = [n.meta.end_pos for n in t.iter_subtrees()
            if n.data in ("quantified", "exists") and not n.meta.empty]
    return max(ends) if ends else -1


RULES = [
    ("D1", "`where` takes the rest of its line", _where_end),
    # D2 was found by Bison's counterexample search (results/bison), not by the corpus
    ("D2", "a quantifier (`for every x, ...`) takes the rest of its line", _quant_end),
]


def resolve(tree, counter):
    """Apply the disambiguation rules bottom-up. Each rule scores readings and
    keeps the highest-scoring ones; an ambiguity is resolved when one is left."""
    for node in list(tree.iter_subtrees()):  # bottom-up order
        if node.data != "_ambig":
            continue
        readings = [c for c in node.children if isinstance(c, Tree)]
        for rid, _, score in RULES:
            scores = [score(r) for r in readings]
            best = max(scores)
            keep = [r for r, sc in zip(readings, scores) if sc == best]
            if len(keep) < len(readings):
                counter[rid] += 1
                readings = keep
            if len(readings) == 1:
                break
        if len(readings) == 1:
            only = readings[0]
            node.data, node.children = only.data, only.children
            node._meta = only._meta
        else:
            node.children = readings
    return tree


def ambiguities(tree):
    """Yield (line, end_line, [rule names of each reading]) for each _ambig node."""
    for node in tree.iter_subtrees():
        if node.data == "_ambig":
            readings = []
            for ch in node.children:
                readings.append(ch.data if isinstance(ch, Tree) else str(ch))
            line = getattr(node.meta, "line", None)
            if line is None:
                for ch in node.children:
                    if isinstance(ch, Tree) and getattr(ch.meta, "line", None):
                        line = ch.meta.line
                        break
            yield line, readings


def parse_file(parser, expr_parser, path):
    src = open(path).read()
    rec = {"file": path, "status": None, "ambiguities": [], "error": None,
           "interpolations": []}
    try:
        text = preprocess(src)
    except LayoutError as e:
        rec["status"] = "layout_error"
        rec["error"] = str(e)
        return rec
    try:
        tree = parser.parse(text)
    except UnexpectedInput as e:
        line = getattr(e, "line", None)
        src_line = src.split("\n")[line - 1].strip() if line and line <= len(src.split("\n")) else ""
        tok = getattr(e, "token", None)
        rec["status"] = "fail"
        rec["error"] = {"line": line, "column": getattr(e, "column", None),
                        "token": str(tok) if tok is not None else None,
                        "source": src_line}
        return rec
    rec["resolved"] = Counter()
    resolve(tree, rec["resolved"])
    rec["resolved"] = dict(rec["resolved"])
    amb = list(ambiguities(tree))
    lines = src.split("\n")
    for line, readings in amb:
        rec["ambiguities"].append({"line": line,
                                   "source": lines[line - 1].strip() if line else "",
                                   "readings": readings})
    rec["status"] = "ambiguous" if amb else "ok"
    # string interpolations are expressions too
    for lineno, inner in interpolations(src):
        r = {"line": lineno, "text": inner}
        try:
            t = resolve(expr_parser.parse(inner), Counter())
            n = len(list(ambiguities(t)))
            r["status"] = "ambiguous" if n else "ok"
        except UnexpectedInput as e:
            r["status"] = "fail"
            r["token"] = str(getattr(e, "token", ""))
        rec["interpolations"].append(r)
    return rec


def main():
    args = sys.argv[1:]
    out = None
    if "--json" in args:
        k = args.index("--json")
        out = args[k + 1]
        args = args[:k] + args[k + 2:]
    grammar, targets = args[0], args[1:]
    parser = load(grammar)
    expr_parser = load(grammar, start="expr") if "expr" in open(grammar).read() else None
    files = []
    for t in targets:
        if os.path.isdir(t):
            files += sorted(os.path.join(t, f) for f in os.listdir(t) if f.endswith(".scribe"))
        else:
            files.append(t)
    results = [parse_file(parser, expr_parser, f) for f in files]
    counts = Counter(r["status"] for r in results)
    for r in results:
        name = os.path.basename(r["file"])
        if r["status"] == "ok":
            extra = f"   (resolved by rule: {r['resolved']})" if r.get("resolved") else ""
            print(f"ok         {name}{extra}")
        elif r["status"] == "ambiguous":
            print(f"AMBIGUOUS  {name}")
            seen = set()
            for a in r["ambiguities"]:
                key = (a["line"], tuple(a["readings"]))
                if key in seen:
                    continue
                seen.add(key)
                print(f"    line {a['line']}: {a['source']}")
                print(f"        readings: {a['readings']}")
        else:
            print(f"FAIL       {name}: {r['error']}")
        for i in r["interpolations"]:
            if i["status"] != "ok":
                print(f"    interpolation line {i['line']} {{{i['text']}}}: {i['status']}")
    print()
    print(dict(counts), "of", len(results))
    if out:
        json.dump(results, open(out, "w"), indent=1)


if __name__ == "__main__":
    main()

