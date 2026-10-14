# Scribe

A systems language for code written by machines and reviewed by people. The
specification is [docs/spec.md](docs/spec.md) (draft 0.3).

## Reference grammar

`grammar/scribe-grammar-v0.2.lark` is the Earley grammar from the pilot
(spec Appendix A). It is the oracle a production parser will be tested
against, not a production parser itself.

    pip install -r requirements.txt
    python3 tools/parse.py --spec docs/spec.md    # every code block in the spec
    python3 tools/parse.py FILE...

The grammar expects layout as tokens (`⏎` end of line, `⇥` indent, `⇤`
dedent, `⟪…⟫` prose) and every keyword as its own token, so it runs with
Lark's standard lexer behind `tools/prelex.py`. Both tools are
reconstructions: the pilot's own pre-lexer and runner (`tools/` in
`scribe-pilot-wave1.zip`) are not in this repository, and neither are rules
D1 and D2, which the pilot applied after parsing.

### The grammar against spec 0.3

20 of the 27 Scribe code blocks in the spec parse with exactly one tree.

| Block | Result | Why |
| --- | --- | --- |
| 5.4 comprehension | ambiguous | `where` scope; rule D1 resolves it after parsing |
| 10.2 examples | error | `example` as a contract clause; v0.2 allows it only at top level |
| 10.3 platform scoping | error | `on <platform>` and `in build` are new in 0.3 |
| 10.6 corrections | error | `corrects` is new in 0.3 (the block is also a fragment) |
| 11.2 interfaces | error | `interface` is new in 0.3 |
| 12.3 correction | error | a fragment, not a whole item |
