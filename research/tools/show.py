"""Show how lines of a Scribe file parse, as compact bracketed readings.

usage: python3 show.py GRAMMAR FILE [line ...]
       python3 show.py GRAMMAR --expr "text"
"""
import os
import sys
from collections import Counter

from lark import Tree, Token

sys.path.insert(0, os.path.dirname(__file__))
from run3 import load, resolve  # noqa: E402
from pre import preprocess  # noqa: E402

SKIP = {"implication", "expr_stmt"}


def sx(t):
    if isinstance(t, Token):
        return "" if t.type.startswith("_") else str(t)
    kids = [k for k in (sx(c) for c in t.children if c is not None) if k]
    if t.data in ("name", "label", "operand") and len(kids) == 1:
        return kids[0]
    if t.data in SKIP and len(kids) == 1:
        return kids[0]
    if t.data == "_ambig":
        return "AMBIGUOUS{ " + "  ||  ".join(kids) + " }"
    return "(" + t.data + " " + " ".join(kids) + ")"


def stmt_nodes(tree):
    for n in tree.iter_subtrees_topdown():
        if not n.meta.empty and n.data not in ("start", "block", "operation", "workflow"):
            yield n


def main():
    g = sys.argv[1]
    if sys.argv[2] == "--expr":
        p = load(g, start="expr", keep_all_tokens=True)
        t = resolve(p.parse(sys.argv[3]), Counter())
        print(sx(t))
        return
    src = open(sys.argv[2]).read()
    want = set(int(x) for x in sys.argv[3:])
    tree = resolve(load(g, keep_all_tokens=True).parse(preprocess(src)), Counter())
    shown = set()
    for n in stmt_nodes(tree):
        if n.meta.line in want and n.meta.line not in shown and n.meta.line == n.meta.end_line:
            shown.add(n.meta.line)
            print(f"{n.meta.line:3}: {src.split(chr(10))[n.meta.line-1].strip()}")
            print("     " + sx(n))


if __name__ == "__main__":
    main()
