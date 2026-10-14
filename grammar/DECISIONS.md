# Decisions the parser needs

Found while bringing the grammar up to specification draft 0.4. The
hand-written parser (roadmap phase 2) needs each of these settled, because
each changes what text is accepted. Each has a recommendation; none is decided
until the specification says so.

| # | Question | Spec | Today | Recommendation |
| --- | --- | --- | --- | --- |
| 1 | ~~How are the update forms (`increment count`, `add price to total`, `subtract`, `decrement`) recognised?~~ Settled: the four words are hard keywords, because a reserved word costs readers nothing and Scribe's authors are machines | 2.1 | Update statements; the words cannot be names | Done; listed in the porting guide |
| 2 | ~~Several unlabelled parameters~~ Settled: `to gcd first, second`, called `gcd 12, 18`; such a call stands bare only as a whole statement or value, and takes parentheses elsewhere | 4.1, 5.1 | Accepted | Done |
| 3 | ~~`some` with a call~~ Settled: `some` takes the whole arithmetic expression after it, under the rule that any expression goes wherever a value goes | 3.4 | `some item 0 of items` accepted | Done |
| 4 | ~~What follows `cost at most`~~ Settled: any expression; the checker accepts an amount or a growth. How growth is written is spec open decision 13 | 10.1 | Any expression | Growth vocabulary in phase 5 |
| 5 | ~~`corrects` across two lines~~ Settled: every statement and clause is one line; both `corrects` shapes are written on one line | 2.6, 10.6, 12.3 | One line only | Done |
| 6 | `assume`, the escape hatch, has no syntax | 10.5 | Not accepted | `assume condition because "..." owner team`, mirroring `trust` |
| 7 | ~~Keyword count~~ Settled: not a concern. Scribe is written by machines, so a large keyword set costs readers nothing | 2.5 | 102 hard, 80 soft | Done |
| 8 | ~~Possessive paths~~ Settled: `account's balance` is allowed and chains; a path never mixes possessives with `of` | 5.1 | Accepted; mixed paths rejected | Done |
| 9 | ~~Failures carrying data~~ Settled: a failure is a value and `fail with` takes any expression; fields are given with record construction (5.3), so the named-field form waits on decision 10 | 6 | `fail with an OutOfRange from clamped` accepted | Type rules in phase 5 (spec open decision 12) |
| 10 | ~~Naming fields when constructing~~ Settled: `a Point with x as 1, y as 2`, after or instead of parts by position | 5.3 | Accepted | Done |
| 11 | ~~Setting one element~~ Settled: `set item i of heap to x` | 4.2 | Accepted | Done |
| 12 | ~~A statement that does nothing~~ Settled: `do nothing`; `choose` keeps no catch-all branch | 4.2 | Accepted; `do` is reserved | Done |
| 13 | ~~What names a platform in `on <platform>`~~ Settled: source never names a platform. Host differences are facts declared on capability interfaces (`varies by host`, spec 10.3, Proposed), and contracts are compared once per host binding | 10.3 | `on <platform>` rejected; `varies by host` accepted in interfaces | Re-express the change replay's platform-specific breakages with it before adopting the syntax (open decision 11) |
| 14 | ~~Positions limited to a name or literal~~ Settled: deadlines, queue sizes, concurrency, refinements, sampling, retries, record parts and capability parts take expressions. Values in contracts and authority must be constant expressions (checker). Kept on purpose: free text, declaration versions, names being introduced, and an unmarked capability resource, which must be a string | 1 | Accepted | Constant check in phase 3 |

## Readability review

Decisions from the review of readability and unambiguous presentation
(spec Section 1: every line has exactly one reading).

| # | Decision | Status |
| --- | --- | --- |
| 1 | Every line has exactly one reading, visible without knowing any precedence rule beyond school arithmetic; this holds for the stored text and every view | Approved; in spec Section 1 |
| 2 | Condition groups: `all of`, `any of` and `none of` open blocks that nest; one condition per line inside a block; `implies` across lines is `if` / `then` on two aligned lines; `all`, `any` and `none` reserved | Open |
| 3 | A word that opens a scope (`for every`, `there is`, `where`, `for each`, `if …,` in `ensure`) covers the rest of its line; any other grouping uses a block. Replaces rules D1 and D2 | Open |
| 4 | Parentheses group arithmetic only; logic is grouped by layout, and `and` and `or` never mix on one line | Open |
| 5 | A block indented under a line is part of that line's structure, as for statements; a line never continues onto the next | Open |
| 6 | Which call a label belongs to, and calls inside lists: trial a rule on the pilot translations before deciding | Open |
| 7 | `some`, `try` and `not` apply to a single term | Open |
| 11 | Commas at line ends in condition blocks are optional on input, checked if present, left out of the stored text and drawn by the review view; one-line groups need them | Approved; takes effect with 2 |
| 12 | `is` only introduces a test (variant, `nothing`, predicate, membership); equality is only `=` and `≠` | Open |
| 13 | Bit operations are named operations on fixed-width unsigned types (`bitwise_and flags with mask`, `shift_left value by 1`); hex and binary literals | Open |
