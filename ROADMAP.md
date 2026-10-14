# Roadmap to Scribe 1.0

Scribe's aims, in order: rapid impact assessment, human readability,
correctness by construction ([base-spec.md](base-spec.md), Section 1).
Version 1 compiles to WebAssembly.

1.0 is a stability promise, as it was for Rust: the stored form, the grammar,
the manifest format and the impact verdicts stop changing incompatibly. It is
not a promise that the language is complete.

Each phase lists its tasks and the test that ends it. Status: **done**,
**now**, or blank for not started. This file is updated as work lands.

## Phase 1. Grammar v0.3

Bring the reference grammar up to spec 0.4, so the hand-written parser has an
exact oracle.

| Task | Status |
| --- | --- |
| Audit every normative construct in the spec against the grammar; a conformance snippet for each, and negative cases for what the spec forbids | done |
| Update `grammar/scribe.lark.tmpl` until every snippet parses with one reading and every negative case is rejected | done |
| One command that runs all grammar checks and fails on any regression | done |
| List the open decisions the parser needs settled, with a recommendation each ([grammar/DECISIONS.md](grammar/DECISIONS.md)) | done |
| Settle those decisions in the spec, then update the grammar and tests to match | done |
| Readability review: every line has exactly one reading; condition groups; no disambiguation rules | done |
| The grammar has no LALR(1) conflicts, checked in CI, so it is proven unambiguous | done |

Done when: the spec's own code blocks and the conformance snippets all parse
with one reading, and the negative cases fail. (Draft 0.5 replaced the pilot's syntax, so the pilot corpora are now a record, not tests.)

## Phase 2. Front end

| Task | Status |
| --- | --- |
| Choose the implementation language for the compiler (Rust) and set up the workspace and CI | done |
| Lexer with the layout rules of Section 2.6 and the hard and soft keywords; agrees with the reference grammar's lexer token for token on every test and spec block | done |
| Decide between a parser generated from the grammar and a hand-written one: hand-written, as Rust, Go, Swift and TypeScript chose, for error messages in the reader's terms | done |
| The parser, producing a syntax tree, tested against the Earley grammar on every build: same accept or reject, same tree (`tools/compare_parsers.py`) | done |
| Formatter that writes the stored form; formatting twice changes nothing, and every view round-trips | |
| Diagnostics as Section 13.2 describes, and `scribe check --explain` | |

Done when: the parser and the oracle agree on every test and spec block, and
the formatter is idempotent on all of them.

## Phase 2b. Impact analysis, first slice

Straight after the parser, ahead of the checker, because impact analysis is
the first aim and a useful part of it needs only syntax.

| Task | Status |
| --- | --- |
| `scribe impact OLD NEW [--dependents FILE...]`: compare signatures, contracts, effects, trust blocks and bodies function by function, and apply the rules of Section 12.3 that need no prover | done |
| Dependents from other modules' `requires` entries, named against each verdict | done |
| A worked example with its recorded report (`tests/impact`) | done |

What it cannot do yet, and says so in its verdicts: decide whether a changed
precondition or guarantee is stronger or weaker (the prover, phase 4), check a
changed body against its contract (the checker, phase 3), and find which
callers handle every failure by name.

## Phase 3. Checker

| Task | Status |
| --- | --- |
| Name resolution, including what `is` means (Section 5.7) | |
| Types: records, enums, `or nothing`, refinements, units, typestate | |
| Failures, effects and capabilities, checked completely | |
| Ownership conventions: read, editable, consumed; linear resources | |
| `example` lines evaluated by an interpreter, before there is a back end | |
| Run `tests/checker`: text the grammar accepts but the checker must reject, each with its expected diagnostic | |

Done when: the spec's examples check, and a deliberately broken copy of each
fails with the diagnostic the spec describes.

## Phase 4. Impact analysis v0

Before code generation, because impact analysis is the first aim and needs
only the checker.

| Task | Status |
| --- | --- |
| Modules and their `requires` graph (Section 11.1) | done in phase 2b |
| `scribe impact`: every verdict of Section 12, using the checker and the prover where phase 2b says "needs review" | |
| Contract coverage, so unchecked changes are reported as such | |
| Replay validation: re-express change-replay cases in Scribe and compare the tool's verdicts with the judges' | |

Done when: on the replay cases the tool matches the judges where the contracts
cover the change, and says "unchecked" where they do not.

## Phase 5. Close the language gaps

The decisions real code needs (pilot report, spec Section 14), each settled
with evidence and written into the spec.

| Task | Status |
| --- | --- |
| Shared state: arena handles, with readers and writers declared | |
| Operation types that carry effects and failures | |
| Failures as values with data | |
| Waiting without `async`, and the concurrency model on WebAssembly | |
| `Text`, the numeric types, bytes and bit operations | |
| Collections: List, Map, Set, slices, element update | |
| Translation experiment wave 2, with the safeguards the pilot lacked | |

Done when: wave 2 translates most functions without a gap.

## Phase 6. WebAssembly back end and runtime

| Task | Status |
| --- | --- |
| Code generation for the core language to WebAssembly components | |
| WASI 0.2/0.3 host bindings; capabilities map to imports | |
| Runs on Wasmtime and in a Cloudflare Worker | |
| `scribe test` runs examples and properties as compiled code | |

## Phase 7. Standard library and reference implementations

| Task | Status |
| --- | --- |
| Command-line programs that behave in a Unix pipeline: standard streams, exit codes, arguments, environment, files (`wasi:cli`, `wasi:filesystem`) | |
| Collections, text, formatting, time and randomness as capabilities, JSON. Time is library only: instants, calendar dates, time zones and time units, all read through a clock capability | |
| A timeout operation on the clock capability, which cancels the work it is given and fails with `TimedOut` | |
| Capability narrowing operations (a read-only directory, a network endpoint, a database), each with a contract that says what the result can reach, so `scribe query reach` works on them | |
| A channel type, and a pipeline operation built on it and `together` | |
| Serialisation with version tags, so stored formats and their upgrades are ordinary types and operations | |
| HTTP client on `wasi:http`; TLS from the host | |
| HTTP server written in Scribe, with its protocol declared (Section 11.4) | |
| Interop with other components through WIT; hand-written manifests for foreign code, marked trusted | |
| A retry operation in the standard library, whose contract says how many times it may call the operation it retries | |
| Every standard library release checked with `scribe impact` against the last | |

## Phase 8. Toolchain and 1.0

| Task | Status |
| --- | --- |
| Package manager and registry; versions computed from the manifest change | |
| Language server for editors, including the review view | |
| Documentation from contracts, the book, an index of every diagnostic | |
| Porting guide from other languages ([docs/porting-guide.md](docs/porting-guide.md)): reserved words and what to use instead, checked against the grammar; grows with phase 5 and wave 2 | started |
| A compact language reference for AI authors, and an evaluation of how well models write Scribe from it | |
| The stability promise, including stable impact verdicts across compiler versions | |

## After 1.0

A solver for proving contracts (until then, a clause the compiler cannot prove is checked at run time), a native back end with direct C interop,
localised views, and the system-level rule language.
