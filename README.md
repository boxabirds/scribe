# Scribe

A language written by agents and read by humans, built so that the impact of a
proposed change can be assessed before it is merged, quickly and reliably, in a
codebase of any size.

1. **Quick, reliable impact assessment.** Everything that links one piece of
   code to another (ownership, effects, capabilities, failures, states,
   contracts) is stated in the source, so standard static analysis finds every
   linkage.
2. **As easy to read as possible.** Code that reads like code, with English
   only where it carries meaning: contracts and condition groups.
3. **Almost provably correct.** Agents write the code, so the language can
   demand rigour no person would type: Rust-style ownership, typestate and a
   contract on every function.

```
function withdraw(amount: Money, account: editable Account) -> Receipt
    can fail with InsufficientFunds
    expect amount > 0
    ensure on success, account.balance == old(account.balance) - amount
    changes only account.balance

    if amount > account.balance
        fail with InsufficientFunds
    account.balance = account.balance - amount
    return Receipt(amount: amount)
```

Version 1 compiles to WebAssembly. The specification is
[base-spec.md](base-spec.md) (draft 0.5).

| Path | What it is |
| --- | --- |
| `base-spec.md` | The specification |
| `scribe-grammar-v0.2.lark` | The grammar as the specification cites it (Appendix A) |
| `grammar/`, `tools/` | The working copy of the grammar and its tools. These change as the language does |
| `crates/` | The compiler, in Rust |
| `tests/` | Grammar conformance tests, and checker tests for phase 3 |
| `research/` | The record of the pilot (experiments 3 and 7, spec Appendix B.1), kept as it ran. Do not edit |
| `docs/` | The [porting guide](docs/porting-guide.md), and earlier documents: the syntax philosophy and worked examples, and the change replay (Appendix B.2, B.4) |

## The compiler

The compiler is written in Rust (`crates/`). It is at the start of roadmap
phase 2: the lexer is done, the parser is next.

    cargo test --workspace
    cargo run -q -p scribe-cli -- lex FILE...
    python3 tools/compare_lexers.py

`compare_lexers.py` lexes every test file and every code block in the
specification with both the compiler and the
reference grammar, and fails on any difference in any token. The keyword list
is generated once, by the grammar build, into `grammar/keywords.txt`, and the
compiler embeds that file, so the two cannot disagree about which words are
reserved. CI runs all of this on every push.

## Reference grammar

`grammar/scribe.lark` is the Earley grammar of spec Appendix A, version 0.4,
built from `grammar/scribe.lark.tmpl`. It states the language precisely and is
the oracle a production parser will be tested against; it is not a production
parser. `scribe-grammar-v0.2.lark` is the pilot's version, as the
specification cites it.

    pip install -r requirements.txt
    python3 tools/check_grammar.py
    python3 tools/check_lalr.py          # needs Bison 3.8

`check_grammar.py` rebuilds nothing and changes nothing: it fails if the built
grammar is stale, if any file in `tests/grammar/accept` does not parse with
exactly one reading, if any file in `tests/grammar/reject` is accepted, or if
the spec's code blocks give results other than those in
`tests/grammar/expected.json`. After a deliberate change, `--update` rewrites
that file; review its diff.

To work on the grammar:

    python3 tools/build_grammar.py grammar/scribe.lark.tmpl grammar/scribe.lark
    python3 tools/run3.py grammar/scribe.lark FILE_OR_DIR...

`tools/pre.py` turns layout into tokens (`⏎` end of line, `⇥` indent, `⇤`
dedent) and `tools/run3.py` parses with every reading kept. `tools/to_bison.py` converts the grammar for Bison's
conflict and counterexample reports.

These tools started as copies of the pilot's. Since then: the grammar build is
deterministic (the pilot ordered keywords of equal length by Python's set
order), and the pre-lexer reports a tab at the start of a line as a tab.

### The grammar against the current spec

Every check runs with no disambiguation rules: every line must have exactly
one reading on its own (spec Section 1).

| Check | Result |
| --- | --- |
| Conformance tests, one per construct (`tests/grammar/accept`) | 30 of 30 parse with one reading |
| Forbidden forms (`tests/grammar/reject`) | 20 of 20 rejected |
| Spec code blocks | 24 of 26; the other 2 are contract fragments, not whole programs |
| Bison | **No LALR(1) conflicts** (v0.2: 172 shift/reduce and 20 reduce/reduce), so no text has two readings; `tools/check_lalr.py` keeps it that way |

[grammar/DECISIONS.md](grammar/DECISIONS.md) lists the open questions the
production parser needs settled.

The pilot corpora in `research/` are written in the pilot's English-sentence
syntax, which draft 0.5 replaced, so they are kept as a record and no longer
parsed.

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
