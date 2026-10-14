"""Run every grammar check and fail on any change from the expected results.

usage: python3 tools/check_grammar.py            check
       python3 tools/check_grammar.py --update   accept the current results as expected

Checks, in order:
  1. grammar/scribe.lark is what tools/build_grammar.py builds from the template;
  2. every file in tests/grammar/accept parses with exactly one reading;
  3. every file in tests/grammar/reject is rejected;
  4. the code blocks of base-spec.md and the pilot corpora in research/corpus
     give the results recorded in tests/grammar/expected.json.

A change to the recorded results is a decision, so --update only rewrites
expected.json; review its diff before committing it.
"""
import json
import os
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))

from build_grammar import build  # noqa: E402
from run3 import load, parse_file  # noqa: E402
from spec_blocks import blocks  # noqa: E402

TEMPLATE = os.path.join(ROOT, "grammar", "scribe.lark.tmpl")
GRAMMAR = os.path.join(ROOT, "grammar", "scribe.lark")
TESTS = os.path.join(ROOT, "tests", "grammar")
EXPECTED = os.path.join(TESTS, "expected.json")
SPEC = os.path.join(ROOT, "base-spec.md")
CORPORA = ["doc", "doc-repaired", "exp7/scribe-A", "exp7/scribe-B"]


def scribe_files(directory):
    return sorted(os.path.join(directory, f) for f in os.listdir(directory) if f.endswith(".scribe"))


def statuses(parser, expr_parser, files):
    return {os.path.basename(f): parse_file(parser, expr_parser, f)["status"] for f in files}


def spec_statuses(parser, expr_parser):
    """Spec blocks are keyed by their first line, which survives edits elsewhere in the spec."""
    out = {}
    with tempfile.TemporaryDirectory() as tmp:
        for line, text in blocks(open(SPEC).read()):
            key = next(l.strip() for l in text.splitlines() if l.strip())
            path = os.path.join(tmp, f"{line:04d}.scribe")
            open(path, "w").write(text)
            out[key] = parse_file(parser, expr_parser, path)["status"]
    return out


def main():
    update = "--update" in sys.argv[1:]
    problems = []

    with tempfile.TemporaryDirectory() as tmp:
        built = os.path.join(tmp, "scribe.lark")
        build(TEMPLATE, built)
        if open(built).read() != open(GRAMMAR).read():
            problems.append("grammar/scribe.lark is stale: run tools/build_grammar.py "
                            "grammar/scribe.lark.tmpl grammar/scribe.lark")

    parser = load(GRAMMAR)
    expr_parser = load(GRAMMAR, start="expr")

    accept = statuses(parser, expr_parser, scribe_files(os.path.join(TESTS, "accept")))
    reject = statuses(parser, expr_parser, scribe_files(os.path.join(TESTS, "reject")))
    for name, status in accept.items():
        if status != "ok":
            problems.append(f"accept/{name}: {status}, expected exactly one reading")
    for name, status in reject.items():
        if status not in ("fail", "layout_error"):
            problems.append(f"reject/{name}: {status}, expected to be rejected")

    results = {"spec": spec_statuses(parser, expr_parser)}
    for corpus in CORPORA:
        results[corpus] = statuses(parser, expr_parser,
                                   scribe_files(os.path.join(ROOT, "research", "corpus", corpus)))

    if update:
        json.dump(results, open(EXPECTED, "w"), indent=1, ensure_ascii=False, sort_keys=True)
        open(EXPECTED, "a").write("\n")
        print(f"wrote {os.path.relpath(EXPECTED, ROOT)}; review its diff")
    else:
        expected = json.load(open(EXPECTED))
        for group in sorted(set(expected) | set(results)):
            want, got = expected.get(group, {}), results.get(group, {})
            for key in sorted(set(want) | set(got)):
                if want.get(key) != got.get(key):
                    problems.append(f"{group}: {key[:70]}: expected {want.get(key)}, got {got.get(key)}")

    print(f"accept {sum(s == 'ok' for s in accept.values())}/{len(accept)}, "
          f"reject {sum(s != 'ok' and s != 'ambiguous' for s in reject.values())}/{len(reject)}")
    for group, found in results.items():
        counts = {}
        for s in found.values():
            counts[s] = counts.get(s, 0) + 1
        print(f"{group}: " + ", ".join(f"{k} {v}" for k, v in sorted(counts.items())))
    for p in problems:
        print("PROBLEM", p)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
