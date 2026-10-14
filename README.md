# Scribe

A language for code written by machines and reviewed by people, built so that
the impact of a proposed change can be assessed before it is merged, quickly,
in a codebase of any size.

1. **Rapid impact assessment.** Programme- and architecture-level constructs
   (components, manifests, interfaces, systems) and a strict syntax with
   checked contracts make every path a change can travel declared, so its
   impact is computed from the declarations rather than by reading the code.
2. **Human readability is paramount.** Code is written by AI and read by
   people: the syntax balances terseness against unfamiliar notation, and
   loses to impact assessment only when no spelling serves both.
3. **Correctness by construction.**

Version 1 compiles to WebAssembly. The specification is
[base-spec.md](base-spec.md) (draft 0.4).

| Path | What it is |
| --- | --- |
| `base-spec.md` | The specification |
| `scribe-grammar-v0.2.lark` | The grammar as the specification cites it (Appendix A) |
| `grammar/`, `tools/` | The working copy of the grammar and its tools. These change as the language does |
| `research/` | The record of the pilot (experiments 3 and 7, spec Appendix B.1), kept as it ran. Do not edit |
| `docs/` | Earlier documents: the syntax philosophy and worked examples, and the change replay (Appendix B.2, B.4) |

## Reference grammar

`grammar/scribe.lark` is the Earley grammar of spec Appendix A, version 0.3,
built from `grammar/scribe.lark.tmpl`. It states the language precisely and is
the oracle a production parser will be tested against; it is not a production
parser. `scribe-grammar-v0.2.lark` is the pilot's version, as the
specification cites it.

    pip install -r requirements.txt
    python3 tools/check_grammar.py

`check_grammar.py` rebuilds nothing and changes nothing: it fails if the built
grammar is stale, if any file in `tests/grammar/accept` does not parse with
exactly one reading, if any file in `tests/grammar/reject` is accepted, or if
the spec's code blocks or the pilot corpora give results other than those in
`tests/grammar/expected.json`. After a deliberate change, `--update` rewrites
that file; review its diff.

To work on the grammar:

    python3 tools/build_grammar.py grammar/scribe.lark.tmpl grammar/scribe.lark
    python3 tools/run3.py grammar/scribe.lark FILE_OR_DIR...

`tools/pre.py` turns layout into tokens (`⏎` end of line, `⇥` indent, `⇤`
dedent, `⟪…⟫` prose) and `tools/run3.py` parses with every reading kept, then
applies rules D1 and D2. `tools/to_bison.py` converts the grammar for Bison's
conflict and counterexample reports.

These tools started as copies of the pilot's. Since then: the grammar build is
deterministic (the pilot ordered keywords of equal length by Python's set
order), and the pre-lexer reports a tab at the start of a line as a tab.

### Version 0.3 against spec 0.4

| Check | Result |
| --- | --- |
| Conformance tests, one per construct (`tests/grammar/accept`) | 34 of 34 parse with one reading |
| Forbidden forms (`tests/grammar/reject`) | 12 of 12 rejected |
| Spec code blocks | 23 of 26; the other 3 are fragments, not whole programs |
| Pilot doc examples, repaired | 41 of 42; example 05 separates fields with `and`, which spec 3.3 forbids |
| Pilot doc examples, as written | 31 of 42; example 22's `run ReportExporter with reports and partner`, one of the pilot's silent misreadings, is now an error |
| Bison | 183 shift/reduce and 20 reduce/reduce conflicts (v0.2: 172 and 20); the one proven ambiguity is still quantifier scope, which rule D2 resolves |

[grammar/DECISIONS.md](grammar/DECISIONS.md) lists the open questions the
production parser needs settled.

## The pilot

`research/` is `scribe-pilot-wave1.zip`; its README lists the commands to
repeat it, run from that folder. Its grammar and tools match
`results/checksums-final.txt`, and re-running experiment 3 reproduces the
recorded output byte for byte (33 of 42 doc examples as written, 42 of 42
repaired).

11 of experiment 7's 48 source excerpts are not in this repository, because
their licences (GPL, LGPL, Redis's, and one project with no licence) do not
allow it. [research/corpus/exp7/NOTICE.md](research/corpus/exp7/NOTICE.md)
lists every excerpt and translation with its licence, and how to restore the
missing excerpts. The 15 translations of those excerpts were removed as well.
