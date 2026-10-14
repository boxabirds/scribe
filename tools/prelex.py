"""Pre-lexer for the Earley reference grammar.

Turns Scribe source into the token stream the grammar expects: every line ends
with ⏎, a deeper indent opens ⇥ and a shallower one closes ⇤. Blocks are four
spaces; a tab is an error. Blank lines and `note` comments are dropped.

This is a reconstruction: the pilot's own pre-lexer (tools/ in
scribe-pilot-wave1.zip) is not in this repository. The lines under an `about`
header are prose, joined into one ⟪ ⟫ token on the header's line.
"""

import sys

NL, INDENT, DEDENT = "⏎", "⇥", "⇤"


class LayoutError(Exception):
    pass


def join_prose(lines):
    """Fold the indented lines under each `about` header into ⟪prose⟫."""
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        out.append(line)
        i += 1
        if line.strip() != "about":
            continue
        indent = len(line) - len(line.lstrip(" "))
        prose = []
        while i < len(lines) and (
            not lines[i].strip() or len(lines[i]) - len(lines[i].lstrip(" ")) > indent
        ):
            prose.append(lines[i].strip())
            i += 1
        out[-1] = f"{line} ⟪{' '.join(p for p in prose if p)}⟫"
    return out


def prelex(source):
    out = []
    depth = 0
    for number, line in enumerate(join_prose(source.splitlines()), 1):
        if "\t" in line[: len(line) - len(line.lstrip())]:
            raise LayoutError(f"line {number}: tabs are a syntax error")
        text = line.strip()
        if not text or text == "note" or text.startswith("note "):
            continue
        spaces = len(line) - len(line.lstrip(" "))
        if spaces % 4:
            raise LayoutError(f"line {number}: indent is not a multiple of four spaces")
        level = spaces // 4
        if level > depth + 1:
            raise LayoutError(f"line {number}: indented more than one level")
        if level > depth:
            out.append(INDENT)
        out.extend(DEDENT * (depth - level))
        depth = level
        out.append(f"{text} {NL}\n")
    out.extend(DEDENT * depth)
    return " ".join(out)


if __name__ == "__main__":
    sys.stdout.write(prelex(sys.stdin.read()))
