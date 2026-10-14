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
| Settle those decisions in the spec, then update the grammar and tests to match | done, except `assume` (decision 6) |

Done when: the spec's own code blocks and the conformance snippets all parse
with one reading, the negative cases fail, and the pilot corpora still pass.

## Phase 2. Front end

| Task | Status |
| --- | --- |
| Choose the implementation language for the compiler (Rust) and set up the workspace and CI | done |
| Lexer with the layout rules of Section 2.6 and the hard and soft keywords; agrees with the reference grammar's lexer token for token on every test, spec and corpus file | done |
| Hand-written parser producing a syntax tree, tested against the Earley grammar on every build: same accept or reject, same tree | now |
| Formatter that writes the stored form, with display forms; formatting twice changes nothing, and every view round-trips | |
| Diagnostics as Section 13.2 describes, and `scribe check --explain` | |
| Diagnostics for spellings the language has dropped: `give back x` parses as a call to `give`, so name resolution should suggest `return` | |

Done when: the parser and the oracle agree on every corpus file, and the
formatter is idempotent on all of them.

## Phase 3. Checker

| Task | Status |
| --- | --- |
| Name resolution, including what `is` means (Section 5.7) | |
| Types: records, choices, `maybe`, refinements, units | |
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
| Manifests and components (Section 11.1) | |
| `scribe impact`: compare two versions and give the verdicts of Section 12 | |
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
| Collections, text, formatting, time and randomness as capabilities, JSON | |
| HTTP client on `wasi:http`; TLS from the host | |
| HTTP server written in Scribe, with its protocol declared (Section 11.4) | |
| Interop with other components through WIT; hand-written manifests for foreign code, marked trusted | |
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

The `proven` tier and its solver, a native back end with direct C interop,
localised views, and the system-level rule language.
