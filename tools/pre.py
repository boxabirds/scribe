"""Scribe pre-lexer.

Turns Scribe source into a flat text that a context-free parser can read:
  - indentation becomes explicit INDENT / DEDENT marks,
  - each logical line ends in an explicit NEWLINE mark,
  - `note` lines are comments and are dropped,
  - `...` elision lines in the examples become an ELIDED mark (corpus placeholder,
    not part of the language),
  - blank lines are dropped.

Line numbers are preserved: every output line corresponds to the same input line,
so parser error positions point at the original source.

This stage needs no knowledge of names or types. That is a requirement of
experiment 3: no rule may depend on name resolution.
"""
import re

INDENT, DEDENT, NL, ELIDED = "⇥", "⇤", "⏎", "⋯"


class LayoutError(Exception):
    pass


def indent_of(line):
    stripped = line.lstrip(" ")
    if line[len(line) - len(stripped):].startswith("\t") or "\t" in line[: len(line) - len(stripped)]:
        raise LayoutError("tab in indentation")
    n = len(line) - len(stripped)
    return n


def preprocess(src):
    lines = src.split("\n")
    out = [""] * len(lines)
    stack = [0]
    i = 0
    first_indent = None
    while i < len(lines):
        raw = lines[i].rstrip()
        content = raw.strip()
        if not content or content.startswith("note ") or content == "note":
            i += 1
            continue
        col = indent_of(raw)
        if first_indent is None:
            first_indent = col
        col -= first_indent  # fragments may start indented
        if col < 0:
            raise LayoutError(f"line {i+1}: dedent below the first line")
        if col % 4:
            raise LayoutError(f"line {i+1}: indentation is not a multiple of four spaces")
        marks = ""
        if col > stack[-1]:
            if col != stack[-1] + 4:
                raise LayoutError(f"line {i+1}: indented more than one level")
            stack.append(col)
            marks = INDENT
        while col < stack[-1]:
            stack.pop()
            marks += DEDENT
        if col != stack[-1]:
            raise LayoutError(f"line {i+1}: dedent to a level that was never opened")

        if re.fullmatch(r"\.\.\.[^.]*(\.\.\.)?", content):
            out[i] = marks + ELIDED + " " + NL
        else:
            out[i] = marks + content + " " + NL
        i += 1
    tail = DEDENT * (len(stack) - 1)
    out.append(tail)
    return "\n".join(out)


STRING_RE = re.compile(r'"([^"\\]|\\.)*"')


def interpolations(src):
    """Return (line, expression text) for every {..} inside a string literal."""
    found = []
    for lineno, line in enumerate(src.split("\n"), 1):
        for m in STRING_RE.finditer(line):
            for inner in re.findall(r"\{([^{}]*)\}", m.group(0)):
                found.append((lineno, inner))
    return found


if __name__ == "__main__":
    import sys
    print(preprocess(open(sys.argv[1]).read()))
