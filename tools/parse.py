"""Parse Scribe source with the Earley reference grammar.

    python3 tools/parse.py FILE...        parse files, print one line each
    python3 tools/parse.py --spec FILE    parse every code block in a Markdown file

Rules D1 and D2 (where/quantifier take the rest of the line) are applied after
parsing in the pilot's tools/run3.py, which is not in this repository, so an
ambiguous parse is reported rather than resolved.
"""

import pathlib
import re
import sys

from lark import Lark
from lark.exceptions import LarkError

from prelex import LayoutError, prelex

ROOT = pathlib.Path(__file__).resolve().parent.parent
GRAMMAR = ROOT / "grammar" / "scribe-grammar-v0.2.lark"


def parser():
    return Lark(GRAMMAR.read_text(), parser="earley", ambiguity="explicit", lexer="basic")


def parse(p, source):
    """Return (status, detail): ok, ambiguous or error."""
    try:
        tree = p.parse(prelex(source))
    except (LarkError, LayoutError) as e:
        return "error", str(e).strip().splitlines()[0]
    ambiguities = sum(1 for _ in tree.find_data("_ambig"))
    if ambiguities:
        return "ambiguous", f"{ambiguities} ambiguous node(s)"
    return "ok", ""


# Code blocks in the spec that show tool output rather than Scribe source.
NOT_SOURCE = re.compile(r"^(\$ |scribe |\w+\.scribe:\d+:)")


def code_blocks(markdown):
    for match in re.finditer(r"^```[^\n]*\n(.*?)^```", markdown, re.S | re.M):
        if not NOT_SOURCE.match(match.group(1)):
            yield markdown[: match.start()].count("\n") + 1, match.group(1)


def main(args):
    p = parser()
    if args[:1] == ["--spec"]:
        path = pathlib.Path(args[1])
        cases = [(f"{path.name}:{line}", text) for line, text in code_blocks(path.read_text())]
    else:
        cases = [(name, pathlib.Path(name).read_text()) for name in args]
    counts = {}
    for name, text in cases:
        status, detail = parse(p, text)
        counts[status] = counts.get(status, 0) + 1
        print(f"{status:9} {name}  {detail}")
    print(", ".join(f"{k} {v}" for k, v in sorted(counts.items())))
    return 0 if set(counts) <= {"ok"} else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
