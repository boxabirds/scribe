"""Convert the lark grammar's expanded BNF rules into a Bison grammar, so that a
deterministic LALR(1) generator can report conflicts and counterexamples.

usage: python3 to_bison.py GRAMMAR OUT.y
"""
import sys

from lark import Lark
from lark.grammar import Terminal


def main():
    lark_src = open(sys.argv[1]).read()
    parser = Lark(lark_src, parser="earley", lexer="basic", keep_all_tokens=True)
    rules = parser.rules
    terms = set()
    by_origin = {}
    for r in rules:
        by_origin.setdefault(r.origin.name, []).append(r.expansion)
        for sym in r.expansion:
            if isinstance(sym, Terminal):
                terms.add(sym.name)

    def nt(name):
        return "nt_" + name.replace("$", "S")

    def tm(name):
        return "T_" + name

    out = ["%define lr.type lalr", "%define parse.error verbose"]
    out.append("%token " + " ".join(tm(t) for t in sorted(terms)))
    out.append("%start nt_start")
    out.append("%%")
    for origin, exps in by_origin.items():
        alts = []
        for exp in exps:
            syms = [tm(s.name) if isinstance(s, Terminal) else nt(s.name) for s in exp]
            alts.append(" ".join(syms) if syms else "%empty")
        out.append(nt(origin) + "\n    : " + "\n    | ".join(alts) + "\n    ;")
    out.append("%%")
    open(sys.argv[2], "w").write("\n".join(out) + "\n")
    print(f"{len(by_origin)} nonterminals, {sum(len(v) for v in by_origin.values())} productions, {len(terms)} terminals")


if __name__ == "__main__":
    main()
