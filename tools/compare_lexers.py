"""Check that the compiler's lexer and the reference grammar's agree, token for
token, on every test file and every code block in the specification.

usage: python3 tools/compare_lexers.py [--verbose]

Builds the `scribe` command with cargo, lexes each file with `scribe lex
--oracle` and with the reference grammar's lexer (tools/pre.py, then Lark's
lexer for grammar/scribe.lark), and reports the first difference in each file
that differs. Exits non-zero on any difference.
"""
import os
import re
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))

from check_grammar import CORPORA, CORPUS, RENAMES, SPEC, TESTS, scribe_files  # noqa: E402
from pre import LayoutError, preprocess  # noqa: E402
from run3 import load  # noqa: E402
from spec_blocks import blocks  # noqa: E402

from lark.exceptions import LarkError  # noqa: E402

GRAMMAR = os.path.join(ROOT, "grammar", "scribe.lark")
WORD = re.compile(r"[A-Za-z][A-Za-z0-9_]*\Z")
LAYOUT = {"⏎": "newline", "⇥": "indent", "⇤": "dedent", "⋯": "elided"}
JOINED = {name for name, _ in __import__("build_grammar").JOINED}


def keyword_sets():
    hard, soft = set(), set()
    for line in open(os.path.join(ROOT, "grammar", "keywords.txt")):
        if line.startswith(("hard ", "soft ")):
            kind, word = line.split()
            (hard if kind == "hard" else soft).add(word)
    return hard, soft


def oracle_tokens(parser, text, hard, soft):
    """The reference grammar's tokens, in the form `scribe lex --oracle` prints."""
    try:
        tokens = list(parser.lex(preprocess(text)))
    except (LayoutError, LarkError):
        return ["error"]
    out = []
    for t in tokens:
        v = str(t)
        if v in LAYOUT:
            out.append(f"{LAYOUT[v]}\t")
        elif t.type == "LINE_COMMA":
            # the reference reads a line-ending comma with the line's end
            out += ["symbol\t,", "newline\t"]
        elif t.type in JOINED:
            out.append(f"hard\t{v}")
        elif v[0].isdigit():
            out.append(f"{t.type.lower()}\t{v}")
        elif v.startswith('"'):
            out.append(f"string\t{v}")
        elif WORD.match(v):
            kind = "hard" if v in hard else "soft" if v in soft else "name"
            out.append(f"{kind}\t{v}")
        else:
            out.append(f"symbol\t{v}")
    return out


def compiler_tokens(paths, flags):
    """`scribe lex --oracle` for many files at once: {path: tokens}."""
    exe = os.path.join(ROOT, "target", "debug", "scribe")
    result = subprocess.run([exe, "lex", "--oracle", *flags, *paths], capture_output=True, text=True)
    if result.returncode not in (0, 1) or not result.stdout:
        sys.exit(f"scribe lex failed:\n{result.stderr}")
    found, current = {}, None
    for line in result.stdout.split("\n"):
        if line.startswith("== "):
            current = line[3:]
            found[current] = []
        elif line and current is not None:
            kind = line.split("\t", 1)[0]
            if kind == "error":
                found[current] = ["error"]
                current = None
            else:
                # the reference marks a placeholder line with ⋯ and drops its text
                found[current].append("elided\t" if kind == "elided" else line)
    return found


def cases(tmp):
    """(label, path, flags) for every file to compare."""
    out = []
    for group in ("accept", "reject"):
        for f in scribe_files(os.path.join(TESTS, group)):
            out.append((f"tests/{group}/{os.path.basename(f)}", f, []))
    for f in scribe_files(CORPUS):
        out.append((f"tests/corpus/{os.path.basename(f)}", f, []))
    for line, text in blocks(open(SPEC).read()):
        path = os.path.join(tmp, f"spec-{line:04d}.scribe")
        open(path, "w").write(text)
        out.append((f"spec block at line {line}", path, ["--fragment"]))
    for corpus in CORPORA:
        directory = os.path.join(ROOT, "research", "corpus", corpus)
        for f in scribe_files(directory):
            text = open(f).read()
            for pattern, replacement, _ in RENAMES:
                text = re.sub(pattern, replacement, text)
            path = os.path.join(tmp, corpus.replace("/", "-") + "-" + os.path.basename(f))
            open(path, "w").write(text)
            out.append((f"{corpus}/{os.path.basename(f)}", path, ["--fragment", "--elision"]))
    return out


def main():
    verbose = "--verbose" in sys.argv[1:]
    subprocess.run(["cargo", "build", "-q", "-p", "scribe-cli"], cwd=ROOT, check=True)
    hard, soft = keyword_sets()
    parser = load(GRAMMAR)
    differences = 0
    with tempfile.TemporaryDirectory() as tmp:
        all_cases = cases(tmp)
        by_flags = {}
        for label, path, flags in all_cases:
            by_flags.setdefault(tuple(flags), []).append(path)
        compiled = {}
        for flags, paths in by_flags.items():
            compiled.update(compiler_tokens(paths, list(flags)))
        for label, path, flags in all_cases:
            want = oracle_tokens(parser, open(path).read(), hard, soft)
            got = compiled.get(path, ["missing"])
            if want == got:
                if verbose:
                    print(f"same     {label}")
                continue
            differences += 1
            k = next((i for i, (a, b) in enumerate(zip(want, got)) if a != b), min(len(want), len(got)))
            print(f"DIFFERS  {label}: token {k}: reference {want[k:k + 3]!r}, compiler {got[k:k + 3]!r}")
    print(f"{len(all_cases) - differences} of {len(all_cases)} files lex the same")
    return 1 if differences else 0


if __name__ == "__main__":
    sys.exit(main())
