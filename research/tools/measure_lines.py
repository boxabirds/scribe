"""Line-level parse coverage.

A file-level pass/fail hides how much of a translation the grammar accepts. This
tool parses a file, and at each error replaces the failing line (and any lines
indented under it) with an elision mark, then parses again, until the file parses.
It reports which code lines the grammar accepts and which it does not.

Comment lines (`note ...`) and blank lines are not counted.

usage: python3 measure_lines.py MEASURE_GRAMMAR DIR [--json OUT]
"""
import json
import os
import sys

from lark.exceptions import UnexpectedInput

sys.path.insert(0, os.path.dirname(__file__))
from pre import preprocess, LayoutError  # noqa: E402
from run3 import load  # noqa: E402


def is_code(line):
    s = line.strip()
    return bool(s) and not (s == "note" or s.startswith("note "))


def indent(line):
    return len(line) - len(line.lstrip(" "))


def block_of(lines, i):
    """Line i and every following line indented deeper (comments included)."""
    j = i + 1
    while j < len(lines) and (not lines[j].strip() or indent(lines[j]) > indent(lines[i])):
        j += 1
    return range(i, j)


def prev_code(lines, i):
    k = i - 1
    while k >= 0 and not is_code(lines[k]):
        k -= 1
    return k


def measure(parser, src):
    lines = src.split("\n")
    code_lines = [i for i, l in enumerate(lines) if is_code(l)]
    failed = []          # (line number, text) of each line that caused an error
    elided = set()
    for _ in range(len(lines) + 5):
        try:
            parser.parse(preprocess("\n".join(lines)))
            break
        except LayoutError as e:
            return {"error": str(e)}
        except UnexpectedInput as e:
            line = (getattr(e, "line", None) or len(lines)) - 1
            tok = getattr(e, "token", None)
            ttype = getattr(tok, "type", "")
            if ttype in ("_DEDENT", "_INDENT", "$END") or line >= len(lines) or not is_code(lines[line]):
                line = prev_code(lines, min(line, len(lines)))
            if line < 0 or lines[line].strip() == "...":
                line = prev_code(lines, line if line >= 0 else len(lines))
                if line < 0:
                    return {"error": "could not recover"}
            failed.append((line + 1, lines[line].strip()))
            blk = block_of(lines, line)
            for k in blk:
                if is_code(lines[k]) and lines[k].strip() != "...":
                    elided.add(k)
            pad = " " * indent(lines[line])
            for k in blk:
                lines[k] = ""
            lines[line] = pad + "..."
    total = len(code_lines)
    ok = total - len([k for k in elided if k in set(code_lines)])
    return {"code_lines": total, "parsed_lines": ok, "failing_lines": failed}


def main():
    grammar, d = sys.argv[1], sys.argv[2]
    out = sys.argv[sys.argv.index("--json") + 1] if "--json" in sys.argv else None
    parser = load(grammar)
    results = {}
    tot = ok = 0
    for f in sorted(os.listdir(d)):
        if not f.endswith(".scribe"):
            continue
        r = measure(parser, open(os.path.join(d, f)).read())
        results[f] = r
        if "error" in r:
            print(f"{f}: {r['error']}")
            continue
        tot += r["code_lines"]
        ok += r["parsed_lines"]
        print(f"{f}: {r['parsed_lines']}/{r['code_lines']} lines parse, {len(r['failing_lines'])} failure points")
    print(f"\nTOTAL {ok}/{tot} code lines parse ({100*ok/max(tot,1):.0f}%)")
    if out:
        json.dump(results, open(out, "w"), indent=1)


if __name__ == "__main__":
    main()
