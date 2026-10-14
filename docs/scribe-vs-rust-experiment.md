# Scribe or a strict Rust profile? A side-by-side test

Oct 14, 2026

## The question

Scribe shares most of its syntax with existing languages. Is a new language
justified, or would a strict profile of Rust (Rust, plus Verus for contracts,
plus cargo-semver-checks for API changes) give the same impact analysis today?

## The answer

**On detection alone, a new language is not justified.** Given the same
contracts, the Rust stack found or prevented as many real breakages as Scribe
(13 of 18 each, plus 2 more for Scribe that hold only on paper). It also had
fewer false alarms (0 of 4 controls against Scribe's 3) and needed the same
number of lines (182 against 181). It does this today, with real proofs. Scribe's
impact report is a comparison of contract text with no checker behind it.

**Scribe's lead is narrower, and real:**

1. **Every failure is named.** Rust's idiom of returning `io::Error` hid three
   of the breakages from every Rust tool. Scribe's `can fail with` caught all
   three.
2. **It cares which promises a dependent uses**, across releases and for callers
   it has never seen. cargo-semver-checks reads no contracts. Verus checks only
   the callers it is given. Nothing in Rust says "this release weakens
   `ensure` X, which these dependents use". That is what the impact report
   does, and it can be built on top of Verus.
3. **Whole classes of break cannot arise:** keeping a reused buffer (value
   semantics), and new implementations changing type inference (Scribe has no
   trait-driven inference).

**Rust's lead:**

- Ownership is enforced today.
- Verus proves both that a changed body still meets its contract and that each
  dependent still holds.
- `Send` and `Sync` changes are caught. Scribe cannot express them yet:
  shared state, spec 7.4, is open.

So the evidence points to building the impact analysis as a tool over a strict
Rust profile first. Readability is the argument for Scribe that this test does
not measure, and it is the question to settle next (see the decisions at the
end).

## Method

- **Cases.** 18 breaking changes and 4 controls:
  - 15 breakers from the Go change replay (`docs/Scribe Change Replay…md`);
  - 3 from Rust crates: pyo3 0.5.1's changed signature, clap 3.2's type that
    lost `Send`/`Sync`, and `time` 0.3's break when Rust 1.80 added a
    `FromIterator` implementation;
  - controls C04, C19, C25 and C26 from the replay, renamed K04, K19, K25 and
    K26 here.
- **The same model in both.** Each case reduces the code to the behaviour that
  broke (or, for a control, changed), written once in Scribe and once in Rust.
  The two sides use the same contract clause for clause, and each case has a
  dependent that relies on what broke. Text that Go compares exactly is
  modelled as a sequence of characters in Verus, and redirect bodies by their
  length.
- **Two kinds of change.**
  - **Honest:** the new version updates its contract to match its new body.
  - **Silent:** the body changes and the contract does not.
- **Tools.**
  - `scribe impact OLD NEW --dependents DEP` (this repository);
  - cargo-semver-checks 0.51.0 on the old and new crates;
  - Verus, built from source at its current main, on the old, new and silent
    versions, each with its dependent;
  - rustc for C02.
- **Lines.** Non-blank, non-comment lines of the old version plus its dependent,
  with Verus boilerplate (`use`, `verus! {`, `fn main`) left out.

### Limits

Each limit is listed with how it likely affects the result.

- **I wrote every contract knowing what broke.** This test compares tools given
  equal contracts; it says nothing about how complete contracts would be
  without hindsight. The replay showed that completeness decides everything:
  17 of 17 breakages were caught when the contract covered the behaviour, 1 of
  7 when it didn't.
- **The models are small**, about 15 lines each. Real code would make Verus
  proofs much harder, so Verus is favoured here. Scribe has no prover yet, so it
  would have the same problem.
- **Scribe has no checker.** Its "prevented" results (C02, R3) are rules in the
  spec that nothing enforces. They are counted separately.
- **The Rust cases are reconstructions.** R1 is modelled as an added
  parameter. The study only says pyo3's signature changed.
- **The cases are small and one person chose them.** Treat the counts as rough.

## Results

Key to the result columns:

- **Breaking** and **Review** are the impact report's verdicts.
- **Flag** means "unchanged, unchecked": the body changed and the contract did
  not, so nothing has checked the new body.
- **Dep. fails** means the dependent no longer verifies (Verus) or no longer
  compiles (rustc). **Callee fails** means the changed function no longer meets
  its old contract.
- **—** means nothing was reported.

### Breaking changes

| Case | What broke | Scribe, honest | Scribe, silent | cargo-semver-checks | Verus / rustc, honest | Verus, silent |
| --- | --- | --- | --- | --- | --- | --- |
| C01 | Closing a request body stopped draining it | Breaking | Flag | — | Dep. fails | Callee fails |
| C02 | Pooled copy buffers exposed Writers that kept them | Cannot be written (value semantics; unenforced) | | — | rustc rejects the Writer | |
| C03 | `sendfile` copies could hang on exit | Flag | | — | — (no clause covers blocking) | |
| C05 | `user.Current` gained a failure on Windows | Review: new failure | Flag | `enum_variant_added` | Dep. fails (match not exhaustive) | |
| C06 | URL text for a host-less `//x` path changed | Breaking | Flag | — | Dep. fails | Callee fails |
| C07 | `JoinPath` with an empty base changed output | Breaking | Flag | — | Dep. fails | Callee fails |
| C11 | Raw IPv6 hosts now rejected | Review: new failure | Flag | `enum_variant_added` | Dep. fails (match) | |
| C12 | A cancelled dial no longer closed its socket | Breaking | Flag | — | Dep. fails | Callee fails |
| C13 | A copy lost its fast path (performance) | Flag | | — | — (no clause covers speed) | |
| C16 | `errors.Join` of one error returned it unwrapped | Breaking | Flag | — | Dep. fails | Callee fails |
| C21 | `hex.Decode` with a short buffer fails instead of panicking | Review: new failure (and a weaker precondition) | Flag | `enum_variant_added` | Dep. fails | Passes: correct, the old precondition still rules it out |
| C22 | Redirect bodies lost a newline | Breaking | Flag | — | Dep. fails | Callee fails |
| C31 | `accept` fails on old kernels | Review: new failure | Flag | — (`io::Error`) | — (`io::Error`) | |
| C33 | A late XML declaration is rejected | Review: new failure | Flag | `enum_variant_added` | Dep. fails (match) | |
| C36 | `ENOSYS` from `copy_file_range` leaks to callers | Review: new failure | Flag | — (`io::Error`) | — (`io::Error`) | |
| R1 | pyo3 0.5.1: a public function's signature changed | Breaking | | `function_parameter_count_changed` | Dep. fails (does not compile) | |
| R2 | clap 3.2: a type stopped being `Send` and `Sync` | Cannot express (spec 7.4 open) | | `auto_trait_impl_removed` | | |
| R3 | `time` 0.3: a new std implementation broke type inference | Cannot arise (no trait-driven inference; unenforced) | | — (the change was in std) | | |

#### Tallies (18 breakers)

| | Scribe | Rust stack |
| --- | --- | --- |
| Flagged with the right reason | 13 | 12 |
| Prevented by a rule that is enforced today | 0 | 1 (C02) |
| Prevented by a rule on paper only | 2 (C02, R3) | 0 |
| Only a generic "body changed" flag | 2 (C03, C13) | 0 |
| Missed | 1 (R2) | 5 (C03, C13, C31, C36, R3) |

#### The differences, case by case

- **Scribe only:**
  - C31 and C36: Rust code returns `io::Error`, whose kinds no tool reads.
  - R3: Scribe has no trait-driven inference, so this break cannot arise.
- **Rust only:**
  - C02: enforced today; Scribe's rule is only on paper.
  - R2: Scribe has no `Send`/`Sync` concept.
- **Neither, beyond a generic flag:** C03 and C13. No contract in either language
  says anything about blocking or speed. This matches the replay's misses.

### Silent changes

The silent variant is the dangerous case: the code changes and the contract
doesn't.

| Tool | What it said |
| --- | --- |
| Verus | Caught 6 of 7 by proving the new body breaks the old contract. The seventh (C21) correctly passed: under the old precondition the new failure cannot happen |
| `scribe impact` | "Unchanged, unchecked" on all of them, and on the harmless K04 too. It cannot tell a breaking body change from a harmless one until a checker exists |

### Controls

| Case | Change | Scribe | cargo-semver-checks | Verus |
| --- | --- | --- | --- | --- |
| K04 | Built-in `clear` instead of a loop | Flag (body changed) | — | Verified |
| K19 | Returns the gzip `Close` error | Review: new failure | — (`io::Error`) | — |
| K25 | Archive directory names gain a slash | **Breaking** | — | Verified (no dependent uses the name) |
| K26 | Documentation only | Identical | — | — |

K19's flag is arguably correct: it is a new failure that callers could see. K04
and K25 are false alarms. Both have the same root: the impact report diffs
contract text. It does not check a body against its contract, and it does not
check whether a dependent uses the clause that changed. K25's dependent never
reads the name; Verus knows that, and the report does not.

### Lines

| | Scribe | Rust + Verus |
| --- | --- | --- |
| Total, 12 cases with both | 181 | 182 |
| Of which contract lines | 42 | 33 |

The totals are the same. Scribe spends about 4 lines per case on
`module`/`requires`/`exports`. Verus spends them on `final(…)`, `decreases`, and a
loop where a byte string had to be built. The failure cases are the clearest
difference in kind:

- Scribe states a failure in one contract line, `can fail with NotFound,
  NoMapping`.
- Rust gets the same effect from an exhaustive error enum and an exhaustive
  `match` in the caller, with no contract line at all. It works only when the
  crate avoids `io::Error`, `Box<dyn Error>` and `#[non_exhaustive]`.

Side by side, C12 reads:

```
function dial(socket: editable Socket, address: Address)
    can fail with Canceled, Refused
    ensure on failure Canceled, socket.is_closed
    changes only socket
```

```rust
pub fn dial(s: &mut Socket, cancel: bool) -> (r: Result<(), Error>)
    ensures r matches Err(Error::Canceled) ==> final(s).closed,
```

## What this means

1. **Detection does not justify a new language.** The Rust stack matched Scribe
   when both had the same contracts. With Verus it also proves silent changes,
   which the Scribe report cannot do.
2. **The impact report is the missing product, in either language.** No Rust
   tool compares contracts between releases and names the dependents that use
   the changed clause. Verus checks only the dependents it is given;
   cargo-semver-checks reads no contracts.
3. **The report must check reliance, not text.** K25 was flagged as breaking
   when no dependent uses the clause. Verus avoids this because it re-proves
   each dependent. So the report should run the prover over the dependents
   instead of diffing clauses.
4. **Named failures are Scribe's strongest point,** and they could be a lint in
   Rust: public functions return exhaustive, named error enums.
5. **Readability is untested.** If Scribe is worth building, it is because people
   review it faster and more accurately than Verus-annotated Rust, not because
   it finds more. That is testable.

## Decisions

1. **Problem:** Detection alone does not justify Scribe.
   **Solution:** Build the impact report first as a tool over a strict Rust
   profile, with Verus contracts, exhaustive named error enums at public
   boundaries, and cargo-semver-checks. The analysis rules in spec section 12
   carry over unchanged.
   **Action:** Pause the Scribe checker. Prototype `impact` reading Verus specs
   from two crate versions and re-verifying the given dependents against the new
   one.
2. **Problem:** The report diffs contract text, so it raises false alarms (K25)
   and cannot judge silent changes.
   **Solution:** Decide reliance by proof: re-prove each dependent against the
   new contract, and each new body against the old contract.
   **Action:** Rewrite spec section 12.3 so the prover decides each verdict, and
   a text diff only says what to re-prove.
3. **Problem:** Readability is the only remaining case for Scribe, and it is
   unmeasured.
   **Solution:** A review test. Show reviewers the same 20 changes, half in
   Scribe and half in Verus-annotated Rust, and measure time and accuracy at
   spotting the breakage.
   **Action:** Build the test from these cases. Decide Scribe's future on its
   result.
4. **Problem:** Rust's `io::Error` idiom hid C31 and C36 from every tool.
   **Solution:** Make it a rule of the strict profile: no opaque error types at
   public boundaries.
   **Action:** Write the profile's rules as clippy-style lints, starting with
   this one.

## Reproducing

```
cargo build
experiments/rust-comparison/run.sh
```

`run.sh` takes the tool paths from `SEMVER_CHECKS` and `VERUS`. Every case is in
`experiments/rust-comparison/cases/<id>/`:

- `scribe/` holds `old`, `new`, `silent` and `dependent`;
- `rust/old` and `rust/new` are the crates cargo-semver-checks compares;
- `verus/` holds `old`, `new` and `silent`, each with its dependent.

Every tool's output is in `experiments/rust-comparison/results/`.
