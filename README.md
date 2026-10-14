# Scribe

A systems language for code written by machines and reviewed by people. The
specification is [base-spec.md](base-spec.md) (draft 0.3).

| Path | What it is |
| --- | --- |
| `base-spec.md` | The specification |
| `scribe-grammar-v0.2.lark` | The grammar as the specification cites it (Appendix A) |
| `grammar/`, `tools/` | The working copy of the grammar and its tools. These change as the language does |
| `research/` | The record of the pilot (experiments 3 and 7, spec Appendix B.1), kept as it ran. Do not edit |
| `docs/` | Earlier documents: the syntax philosophy and worked examples, and the change replay (Appendix B.2, B.4) |

## Reference grammar

`grammar/scribe.lark` is the Earley grammar of spec Appendix A, built from
`grammar/scribe.lark.tmpl`. It states the language precisely and is the oracle
a production parser will be tested against; it is not a production parser.

    pip install -r requirements.txt
    python3 tools/build_grammar.py grammar/scribe.lark.tmpl grammar/scribe.lark
    python3 tools/run3.py grammar/scribe.lark FILE_OR_DIR...

`tools/pre.py` turns layout into tokens (`⏎` end of line, `⇥` indent, `⇤`
dedent, `⟪…⟫` prose) and `tools/run3.py` parses with every reading kept, then
applies rules D1 and D2. To parse the code in the specification:

    python3 tools/spec_blocks.py base-spec.md /tmp/spec
    python3 tools/run3.py grammar/scribe.lark /tmp/spec

These started as copies of the pilot's tools. One change since: the pilot's
`build_grammar.py` ordered keywords of equal length by Python's set order, so
the built grammar's bytes changed from run to run (the language it accepts did
not). Ties are now broken by spelling, so `grammar/scribe.lark` differs from
`scribe-grammar-v0.2.lark` in the order of that one list and nowhere else.

### The grammar against spec 0.3

21 of the 26 Scribe code blocks in the specification parse with exactly one
reading; one of them (5.4) needs rule D1. The other 5 use what draft 0.3 added
after the v0.2 grammar:

| Section | Construct |
| --- | --- |
| 10.2 | `example` as a contract clause (v0.2 allows it only at top level) |
| 10.3 | `on <platform>` and `in build` scoping |
| 10.6, 12.3 | `corrects` |
| 11.2 | `interface` |

## The pilot

`research/` is `scribe-pilot-wave1.zip`; its README lists the commands to
repeat it, run from that folder. Its grammar and tools match
`results/checksums-final.txt`, and re-running experiment 3 reproduces the
recorded output byte for byte (33 of 42 doc examples as written, 42 of 42
repaired).
