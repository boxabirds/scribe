# Decisions the parser needs

Found while bringing the grammar up to specification draft 0.4. The
hand-written parser (roadmap phase 2) needs each of these settled, because
each changes what text is accepted. Each has a recommendation; none is decided
until the specification says so.

| # | Question | Spec | Today | Recommendation |
| --- | --- | --- | --- | --- |
| 1 | How are the update forms (`increment count`, `add price to total`, `subtract`, `decrement`) recognised? As keywords they would be reserved everywhere, and `add` is a common operation name | 2.1 | Parsed as ordinary calls | Resolve after parsing, as `is` already is (5.7): a call to `add` with no operation of that name in scope is the update form. No new keywords |
| 2 | Several unlabelled parameters: `to coerce first, second` | 4.1, Proposed | Rejected | Adopt. Signatures were the largest single cause of parse failures in the pilot's translations (34, together with label words the spec does not have) |
| 3 | `some` with a call: `some item 0 of items` | 3.4 | Needs parentheses: `some (item 0 of items)` | Allow a call after `some` |
| 4 | What follows `cost at most` | 10.1 | Any expression; the spec gives no example | Start with a quantity (`cost at most 50 milliseconds`); design orders of growth (Section 12.3 compares them) separately |
| 5 | `corrects` has two shapes: under an `ensure` (10.6), and as its own clause with `because` on the next line (12.3) | 10.6, 12.3 | Both accepted | Keep both meanings, but put `because` on the same line in 12.3; that is the only place a clause continues onto a new line |
| 6 | `assume`, the escape hatch, has no syntax | 10.5 | Not accepted | `assume condition because "..." owner team`, mirroring `trust` |
| 7 | Keyword count: the grammar now has 99 hard and 79 soft keywords (was 96 and 77). `interface` must be hard, because it starts an item; `assumes` and `may` must be hard, because a clause starting with a soft word can also be read as a call | 2.5, Open 6 | As stated | Accept for now; the comprehension test in open decision 6 should include these |
| 8 | Possessive paths: `account's balance` | Open 5 | Not accepted | Not for 1.0: it binds tighter than any call, which surprises readers |
| 9 | Failures carrying data: `fail with OutOfRange with value clamped` | 6, Proposed | Not accepted | Adopt the syntax now; the type rules come in phase 5 |
| 10 | Naming fields when constructing: `a Point with x as 1, y as 2` | 5.3, Proposed | Not accepted | Adopt |
| 11 | Setting one element: `set item i of heap to x` | 4.2, Proposed | Rejected | Adopt |
| 12 | A statement that does nothing, for an empty `when` branch | 4.2, Proposed | Not accepted | Adopt, spelled `do nothing` |
