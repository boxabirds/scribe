# Scribe Change Replay: Does Impact Analysis Catch Real Breakages?

Oct 14, 2026 · @Julian Harris

## The answer

**Mostly yes, with one condition and one serious problem.**

We took 24 real changes to the Go standard library that were later undone because they broke something, plus 12 similar changes that were never undone. For each, one set of agents wrote the contracts and manifests for the code as it was before the change, without seeing it, a second set predicted the impact using Scribe's rules, and a third set compared the prediction with what actually happened.

| Finding | Result |
| --- | --- |
| Real breakages where the impact report named what broke | **18 of 24** (14 flagged as breaking, 4 as needs review) |
| Flagged, but for a different reason | 4 of 24 |
| Missed completely | 2 of 24 |
| Breakages caught when the "before" contracts already covered the behaviour that broke | **17 of 17** |
| Breakages caught when it did not | **1 of 7** |
| Harmless changes the report still flagged | **10 of 12** (5 as breaking) |

**The condition:** the impact report is exactly as good as the contracts it starts from. When the contracts covered what the code did, every breakage was caught. When it didn't, almost none were. This is the single clearest result, and it moves the hard problem from "compute impact" to "write complete contracts and keep them complete".

**The problem:** the report flags almost everything. Ten of the twelve harmless changes were flagged, five of them as breaking. About half of those flags were real concerns a maintainer would accept; the rest were noise, mostly from two fixable rule mistakes. A tool that blocks most changes will be switched off, so precision has to improve before this is useful as a gate.

## What was tested

**The changes.** All Go standard-library changes reverted between January 2023 and October 2026 with a stated reason, limited to library code (not the compiler or runtime) and small changes (1 to 4 files, at most 200 lines). That gave 30. I removed four reverted for process reasons rather than breakage (release timing, a proposal not yet accepted, a revert forced by another revert, and an API design objection), and two repeat reverts of the same change, leaving 24 breakages. Their reasons range from "breaks a variety of code inside Google" to "broke windows/arm64" to "introduced a data race on darwin". The 12 controls were drawn at random (fixed seed) from same-sized library changes in the same period that were never reverted and that no later commit mentions. The 36 cases were shuffled and given neutral IDs, so no agent could tell breakers from controls.

**Three stages, three separate sets of agents.**

1. **Write contracts, blind.** Each agent got only the code as it was just before the change, plus the names of the files and declarations that would change. It wrote Scribe manifests for that code and for the code that depends on it: what each part promises, how it fails, what it uses, what it assumes about the platform, and what each dependent relies on. It never saw the change, its message, or any history.
2. **Predict.** A different agent got those manifests and the change itself (the diff, the files after, and the author's commit message, written before any revert). It wrote the manifest as it would be after the change, then applied Scribe's impact rules mechanically: breaking, needs review, authority change, compatible, or no impact. Separately, it listed what a careful human reviewer would worry about.
3. **Score.** Two independent judges per case compared each prediction with the revert reason. A prediction counted as a hit only if a flag named the behaviour or area that actually broke.

**Keeping it honest.**

- The answer key and all history were moved out of reach before stage 1, and the change itself was only copied in after stage 1 finished.
- The machine's installed Go toolchain, which includes a newer copy of the Go source and so could reveal later reverts, was moved out of reach before stage 2.
- Every file outside each stage's output folder was fingerprinted before and after the stage. All three stages came back clean: nothing added, removed or changed.
- The two judges agreed on 35 of 36 scores. The one disagreement (C05, Windows) was settled by reading the Go issue the revert cites: the failure was `user.Current` erroring with "no mapping between account names and security IDs", which is the failure the prediction named.

## Results on the 24 real breakages

| Score | Count | Meaning |
| --- | --- | --- |
| Hit, flagged as breaking | 14 | A breaking or authority-change flag named what broke |
| Hit, flagged for review | 4 | A needs-review flag named what broke |
| Partial | 4 | The change was flagged, but for an unrelated reason |
| Miss | 2 | No impact, or compatible |

### What decided the result: did the contract cover it?

The judges recorded whether the blind "before" contracts already covered the behaviour that later broke.

| The contract covered it | Caught |
| --- | --- |
| Yes | 11 of 11 |
| Partly | 6 of 6 |
| No | 1 of 7 |

This is the mechanism the whole design rests on, and it held: when a behaviour is written down, a change to it is found. It also shows where the risk lives. The blind contracts here were written by agents told to be thorough, and they were often very detailed (exact URL output formats, exact error identities, which descriptor a copy may use). A team writing thinner contracts would sit further down this table.

### By kind of breakage

| What broke | Caught |
| --- | --- |
| Behaviour that users relied on (output formats, accepted inputs, error identity) | **8 of 8** |
| Concurrency (races, atomicity) | 3 of 4 |
| Platform or environment (Windows, Linux kernels, sandboxes, architectures) | 4 of 6 |
| Memory safety or compiler interaction | 1 of 2 |
| Its own documented contract | 1 of 1 |
| Test or process (flaky tests) | 1 of 2 |
| Performance | 0 of 1 |

The first row matters most. "It broke code that relied on the old behaviour" is the classic case that today's tools cannot see, because the types don't change. Scribe caught all eight, including `net/url` rejecting raw IPv6 hosts, `errors.Join` returning a different concrete type, and a redirect body becoming one byte shorter.

### The six it did not catch

| Case | What broke | Why the report missed it | What would have caught it |
| --- | --- | --- | --- |
| C10 (miss) | Race-detector builds now poisoned buffers, exposing Writers in `net/http` that kept them | Nothing in the contract changed, so the rules gave "no impact". The change only altered behaviour in race-detector builds | A stated assumption that Writers never keep the buffer, and clauses that can differ by build mode |
| C15 (miss) | New `math.Compare` gave wrong answers on MIPS | A brand-new function: there was no old contract to break, and the new one held on every architecture except one | Assumptions scoped per architecture (NaN bit patterns) |
| C03 (partial) | Using `sendfile` for file-to-file copies could hang a process on exit | The report flagged other risks of the new path, not blocking in the kernel | An effect stating that a call can block uninterruptibly in the kernel |
| C13 (partial) | A copy change caused a performance regression | No contract covered speed | Declared cost properties, such as which fast path a copy must use |
| C17 (partial) | A test became flaky | Flagged the error-wrapping change for other reasons | None reasonable |
| C20 (partial) | `reflect` change broke compiled code at Google | Flagged a weakened clause, not the compiler interaction | None reasonable |

### The reviewer column

The prediction agents also listed what a careful human reviewer would worry about, separately from the tool's verdicts. Those concerns named what broke in **20 of 24** cases, two more than the tool. Both extra catches were the misses above, where the reviewer spotted risks that the contracts had no words for. This test had no condition where reviewers worked without the contracts, so it cannot say how much the contracts helped the reviewers themselves.

## Results on the 12 controls

The controls were never reverted, so they are the test of false alarms. They are not guaranteed harmless: a change can break someone quietly and never be reverted.

| Report | Count |
| --- | --- |
| Clean (no impact or compatible) | 2 |
| Needs review or authority change only | 5 |
| Breaking claimed | 5 |

The judges then asked whether each flag was a concern a maintainer would accept: 5 yes, 3 unclear, 2 no. So roughly half the alarms were useful, and half were noise.

**Useful alarms, even on changes that were never reverted:**

- C18 (`net`, cancelled DNS lookups): the report found that on Plan 9 a cancelled lookup now returns no result and no error, which then crashes the caller. That looks like a real bug.
- C14 (`reflect`): a new unchecked assumption that compiler-generated code never keeps a pointer, which if ever false would be a silent memory-safety bug.
- C25 (`archive/tar` and `archive/zip`): directory names gain a trailing slash. A genuine, deliberate output change that external users could notice.

**Noise, and the rule mistakes behind it:**

| Rule mistake | Cases | Fix |
| --- | --- | --- |
| Removing an unexported item counted as breaking, even though every caller was rewritten in the same change | C29, C34 | Only removals that leave a caller unchanged are breaking. Callers updated in the same change are updated, not broken |
| An intended bug fix counted as breaking because dependents "relied on" the buggy behaviour as written in the contract | C19 | Let a change mark a clause as a correction, which turns breaking into needs review with a stated reason |
| Taking a lock counted as a new authority, in code that already took locks | C09 | Compare effects per component, not per call |
| A changed informal clause flagged even when the formal clauses were unchanged | C24, C30 | Informal clauses that describe implementation, not behaviour, should not be in the manifest |

Applying these four fixes by hand (my reading, not re-run by the agents) gives 4 clean, 5 needs review and 3 breaking. The three remaining breaking claims (C18 Plan 9 lookups, C25 archive names, and C29's change to \`net/mail\` header parsing) were all judged plausible.

## What this means for Scribe's design

1. **The core idea works.** Write down what code promises as contracts, compare the contracts before and after a change, and you find real breakages, including the ones types cannot see. 8 of 8 relied-on-behaviour breakages were caught.
2. **The contracts are the product.** 17 of 17 caught when the behaviour was written down; 1 of 7 when it was not. So the most valuable features are the ones that make contracts complete and keep them true:
   - generate the first draft of a contract from the code, and check it against the code on every build, so it cannot drift;
   - flag weak contracts (operations with no guarantees, outputs no clause covers) as the pilot's "weak contract" warning already proposed;
   - make exact outputs easy to state (formats, error identity), because that is where most relied-on breakages happened.
3. **"No change to the contract means no impact" is the dangerous rule.** It produced one of the two misses. When the code changes but the contract does not, the tool should not stay silent. It should at least say "behaviour changed behind an unchanged contract" and ask for review, or run the regression check the earlier research proposed.
4. **Platform, build mode and cost need a place in the contract.** Four of the six misses and partials were about a specific operating system, kernel, architecture, race-detector build, or speed. Scribe needs assumptions and guarantees that can be scoped to a platform or build, and a way to state cost properties.
5. **Precision needs the four rule fixes above** before the report can act as a gate. Otherwise teams will learn to ignore it.
6. **Compare with a reviewer.** The reviewer-style concerns were slightly better than the tool (20 versus 18). Scribe's value is not beating a careful reviewer; it is being systematic, giving the reason in a form that can be checked, and doing it on every change. The next test should include a reviewer working without the contracts, to measure what the contracts add.

## Limits of this test

| Limit | Likely effect |
| --- | --- |
| **No real tool exists yet.** The "tool" was an AI agent applying Scribe's rules by hand after reading the diff closely | Probably more perceptive than a mechanical checker, so the hit rate is likely an upper bound |
| **The contracts were unusually thorough.** Agents were told to write contracts as a careful engineer would, and spent roughly 3 to 12 minutes reading the source for each case | Real teams' contracts may be thinner; the 17-of-17 versus 1-of-7 split shows how much that matters |
| **All roles were AI models from the same family**, some of which may have seen Go's history in training | Blinding removed the obvious routes to the answer, but recalled knowledge cannot be ruled out |
| **Small sample, one ecosystem**: 24 breakages and 12 controls, all Go standard library, all small | Percentages are rough; a different codebase could behave differently |
| **Revert reasons are short**, and one needed the linked issue to settle | Judges scored strictly; a few partials may be hits |
| **Controls are not guaranteed harmless** | Some "false alarms" may be real problems nobody reported |
| **Reverts only catch breakages found quickly** | Slow-burn breakages, the kind most worth predicting, are not in this sample |

What the result does establish: Scribe's approach can name real breakages that today's type-based tools cannot, and its accuracy depends almost entirely on how completely the contracts cover the code.

## Case by case

| Case | Kind | Change | Tool | Reviewer | Contract covered it |
| --- | --- | --- | --- | --- | --- |
| C01 | breaker | net/http: don't discard body content when closed in handler | Hit, breaking | Hit | Yes |
| C02 | breaker | io: allocate copy buffers from a pool | Hit, breaking | Hit | Partly |
| C03 | breaker | os: sendfile for file-to-file copies on Linux | Partial | Partial | No |
| C05 | breaker | os/user: speed up Current on Windows | Hit, review | Hit | Yes |
| C06 | breaker | net/url: avoid turning Path into Host | Hit, breaking | Hit | Yes |
| C07 | breaker | net/url: empty base path in JoinPath | Hit, breaking | Hit | Yes |
| C08 | breaker | crypto/internal/boring: cgo noescape and nocallback | Hit, review | Hit | No |
| C10 | breaker | io: detect Writers that keep io.Copy's buffer | Miss | Hit | No |
| C11 | breaker | net/url: disallow raw IPv6 hosts | Hit, breaking | Hit | Yes |
| C12 | breaker | net: move cancellation logic of blocking calls | Hit, breaking | Hit | Partly |
| C13 | breaker | net/http: io.Copy in ServeContent | Partial | Partial | No |
| C15 | breaker | math: add Compare and Compare32 | Miss | Hit | No |
| C16 | breaker | errors: optimise Join for one unwrappable error | Hit, breaking | Hit | Yes |
| C17 | breaker | net/http: wrap client errors | Partial | Partial | No |
| C20 | breaker | reflect: zero-sized fields of direct structs | Partial | Partial | No |
| C21 | breaker | hex: fix panic in Decode with a short dst | Hit, review | Hit | Partly |
| C22 | breaker | net/http: remove newline on redirects | Hit, breaking | Hit | Yes |
| C23 | breaker | fips140/subtle: assembly xorBytes | Hit, review | Hit | Partly |
| C28 | breaker | testing: report subtest races once | Hit, breaking | Hit | Yes |
| C31 | breaker | internal/poll: remove accept fallback | Hit, breaking | Hit | Partly |
| C32 | breaker | internal/sync: optimise CompareAndSwap and Swap | Hit, breaking | Hit | Yes |
| C33 | breaker | encoding/xml: reject a late XML declaration | Hit, breaking | Hit | Yes |
| C35 | breaker | go/types: generic methods out of method sets | Hit, breaking | Hit | Partly |
| C36 | breaker | internal/poll: drop ENOSYS check in CopyFileRange | Hit, breaking | Hit | Yes |
| C04 | control | index/suffixarray: use built-in clear | Clean | — | — |
| C09 | control | httputil: ReverseProxy race on 1xx responses | Review | — | — |
| C14 | control | reflect: IsZero does not escape | Review | — | — |
| C18 | control | net: wrap cancellation errors, consistent DNSError | Breaking claimed | — | — |
| C19 | control | internal/profile: return gzip Close error | Breaking claimed | — | — |
| C24 | control | log/slog: concatenation in Attr.String | Review | — | — |
| C25 | control | archive: directory names end in a slash | Breaking claimed | — | — |
| C26 | control | pkgbits: documentation | Clean | — | — |
| C27 | control | net/http: don't cancel dials with requests | Review | — | — |
| C29 | control | textproto, multipart: bound MIME header reads | Breaking claimed | — | — |
| C30 | control | go/types: generate util\_test.go | Review | — | — |
| C34 | control | nistec: precomputed P-256 tables in purego | Breaking claimed | — | — |

Everything is in `scribe-change-replay.zip`: the selection scripts, the answer key, every manifest, prediction and judgment, the briefs each stage was given, and the integrity fingerprints. The code snapshots are left out because of their size (4.6 GB); the scripts rebuild them from the Go repository.
