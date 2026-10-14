"""Run every grammar check and fail on any change from the expected results.

usage: python3 tools/check_grammar.py            check
       python3 tools/check_grammar.py --update   accept the current results as expected

Checks, in order:
  1. grammar/scribe.lark and grammar/keywords.txt are what tools/build_grammar.py
     builds from the template;
  2. every file in tests/grammar/accept and tests/corpus parses with exactly
     one reading;
  3. every file in tests/grammar/reject is rejected;
  4. the code blocks of base-spec.md give the results recorded in
     tests/grammar/expected.json;
  5. docs/porting-guide.md lists every hard keyword, and nothing else, and none
     of the names it suggests instead is itself a hard keyword.

The pilot corpora in research/corpus use the pilot's syntax, which draft 0.5
replaced, so they are kept as a record and no longer parsed.

A change to the recorded results is a decision, so --update only rewrites
expected.json; review its diff before committing it.
"""
import json
import os
import re
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))

from build_grammar import build  # noqa: E402
from run3 import load, parse_file  # noqa: E402
from spec_blocks import blocks  # noqa: E402

TEMPLATE = os.path.join(ROOT, "grammar", "scribe.lark.tmpl")
GRAMMAR = os.path.join(ROOT, "grammar", "scribe.lark")
KEYWORDS = os.path.join(ROOT, "grammar", "keywords.txt")
TESTS = os.path.join(ROOT, "tests", "grammar")
# real code translated into Scribe; every file must parse with one reading
CORPUS = os.path.join(ROOT, "tests", "corpus")
EXPECTED = os.path.join(TESTS, "expected.json")
SPEC = os.path.join(ROOT, "base-spec.md")
PORTING = os.path.join(ROOT, "docs", "porting-guide.md")
# The pilot corpora (research/corpus) are written in the pilot's English-sentence
# syntax, which draft 0.5 replaced; they are a historical record, not tests.
CORPORA = []

# Spellings changed since the pilot: (pattern, replacement, specification change)
RENAMES = []


def scribe_files(directory):
    return sorted(os.path.join(directory, f) for f in os.listdir(directory) if f.endswith(".scribe"))


def statuses(parser, expr_parser, files):
    # no disambiguation rules: every line must have exactly one reading (spec Section 1)
    return {os.path.basename(f): parse_file(parser, expr_parser, f, rules=False)["status"] for f in files}


def corpus_statuses(parser, expr_parser, directory):
    """Parse a pilot corpus as it would be written today, without touching it."""
    with tempfile.TemporaryDirectory() as tmp:
        copies = []
        for f in scribe_files(directory):
            text = open(f).read()
            for pattern, replacement, _ in RENAMES:
                text = re.sub(pattern, replacement, text)
            copy = os.path.join(tmp, os.path.basename(f))
            open(copy, "w").write(text)
            copies.append(copy)
        return statuses(parser, expr_parser, copies)


def spec_statuses(parser, expr_parser):
    """Spec blocks are keyed by their first line, which survives edits elsewhere in the spec."""
    out = {}
    with tempfile.TemporaryDirectory() as tmp:
        for line, text in blocks(open(SPEC).read()):
            key = next(l.strip() for l in text.splitlines() if l.strip())
            path = os.path.join(tmp, f"{line:04d}.scribe")
            open(path, "w").write(text)
            out[key] = parse_file(parser, expr_parser, path, rules=False)["status"]
    return out


def porting_guide_words():
    """The keywords in the first column of the porting guide's tables, and the
    names suggested in its last column."""
    keywords, suggested = set(), set()
    for line in open(PORTING):
        if line.startswith("| `"):
            cells = line.split("|")
            keywords.update(re.findall(r"`([a-z_]+)`", cells[1]))
            if len(cells) > 5:
                suggested.update(re.findall(r"`([A-Za-z_]+)`", cells[-2]))
    return keywords, suggested


def main():
    update = "--update" in sys.argv[1:]
    problems = []

    with tempfile.TemporaryDirectory() as tmp:
        built = os.path.join(tmp, "scribe.lark")
        keywords = os.path.join(tmp, "keywords.txt")
        hard, _, _ = build(TEMPLATE, built, keywords)
        listed, suggested = porting_guide_words()
        for word in sorted(suggested & set(hard)):
            problems.append(f"docs/porting-guide.md: suggests `{word}`, which is a hard keyword")
        for word in sorted(set(hard) - listed):
            problems.append(f"docs/porting-guide.md: hard keyword `{word}` is not listed")
        for word in sorted(listed - set(hard)):
            problems.append(f"docs/porting-guide.md: `{word}` is listed but is not a hard keyword")
        for fresh, committed in ((built, GRAMMAR), (keywords, KEYWORDS)):
            if not os.path.exists(committed) or open(fresh).read() != open(committed).read():
                problems.append(f"{os.path.relpath(committed, ROOT)} is stale: run tools/build_grammar.py "
                                "grammar/scribe.lark.tmpl grammar/scribe.lark")

    parser = load(GRAMMAR)
    expr_parser = load(GRAMMAR, start="expr")

    accept = statuses(parser, expr_parser, scribe_files(os.path.join(TESTS, "accept")))
    accept.update({f"corpus/{k}": v for k, v in
                   statuses(parser, expr_parser, scribe_files(CORPUS)).items()})
    reject = statuses(parser, expr_parser, scribe_files(os.path.join(TESTS, "reject")))
    for name, status in accept.items():
        if status != "ok":
            problems.append(f"accept/{name}: {status}, expected exactly one reading")
    for name, status in reject.items():
        if status not in ("fail", "layout_error"):
            problems.append(f"reject/{name}: {status}, expected to be rejected")

    results = {"spec": spec_statuses(parser, expr_parser)}
    for corpus in CORPORA:
        results[corpus] = corpus_statuses(parser, expr_parser,
                                          os.path.join(ROOT, "research", "corpus", corpus))

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
