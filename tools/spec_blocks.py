"""Write each Scribe code block of a Markdown document to its own file, so
run3.py can parse a specification the same way it parses a corpus.

usage: python3 tools/spec_blocks.py docs/spec.md OUT_DIR

Files are named by the line the block starts on (spec-0128.scribe). Blocks that
show tool output rather than source (a shell prompt, a `scribe` report, a
diagnostic) are skipped.
"""
import os
import re
import sys

NOT_SOURCE = re.compile(r"^(\$ |scribe |\w+\.scribe:\d+:)")


def blocks(markdown):
    for m in re.finditer(r"^```[^\n]*\n(.*?)^```", markdown, re.S | re.M):
        if not NOT_SOURCE.match(m.group(1)):
            yield markdown[: m.start()].count("\n") + 2, m.group(1)


def main():
    path, out = sys.argv[1], sys.argv[2]
    os.makedirs(out, exist_ok=True)
    stem = os.path.splitext(os.path.basename(path))[0]
    n = 0
    for line, text in blocks(open(path).read()):
        open(os.path.join(out, f"{stem}-{line:04d}.scribe"), "w").write(text)
        n += 1
    print(f"{n} blocks written to {out}")


if __name__ == "__main__":
    main()
