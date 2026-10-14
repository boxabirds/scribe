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
| 4 | ~~What follows `cost at most`~~ Moot: cost bounds are removed, because they are not instructions | 10.1 | Removed | Done |
| 5 | ~~`corrects` across two lines~~ Moot: `corrects` is removed, because it describes an earlier program. Every statement and clause is one line | 2.6, 10.6 | Removed | Done |
| 6 | ~~`assume`, the escape hatch, has no syntax~~ Settled: an `assert` inside a `trust` block is taken as true, so no `assume` is needed | 10.5 | `trust` block | Done |
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
| 2 | Condition groups: `all of`, `any of` and `none of` open blocks that nest; one condition per line inside a block; `implies` across lines is `if` / `then` on two aligned lines; `all`, `any` and `none` reserved | Approved; in spec and grammar |
| 3 | A word that opens a scope (`for every`, `there is`, `where`, `for each`, `if …,` in `ensure`) covers the rest of its line; any other grouping uses a block. Replaces rules D1 and D2 | Approved; in spec and grammar |
| 4 | Parentheses group arithmetic only; logic is grouped by layout, and `and` and `or` never mix on one line | Approved; in spec and grammar |
| 5 | A block indented under a line is part of that line's structure, as for statements; a line never continues onto the next | Approved; in spec and grammar |
| 6 | Which call a label belongs to, and calls inside lists: trial a rule on the pilot translations before deciding | Settled by 15: an argument is one term, so a label belongs to the outermost call; a labelled call inside an argument is bracketed. No trial needed |
| 7 | `some`, `try` and `not` apply to a single term | Approved; in spec and grammar |
| 11 | Commas at line ends in condition blocks are optional on input, checked if present, left out of the stored text and drawn by the review view; one-line groups need them | Approved; in spec and grammar |
| 12 | `is` only introduces a test (variant, `nothing`, predicate, membership); equality is only `=` and `≠` | Approved; in spec and grammar |
| 13 | Bit operations are named operations on fixed-width unsigned types (`bitwise_and flags with mask`, `shift_left value by 1`); hex and binary literals | Approved; in spec and grammar |
| 14 | `followed by` joins single terms; arithmetic in an operand takes brackets | Approved; in spec and grammar |
| 15 | The reference grammar has zero LALR(1) conflicts, checked in CI, so it is proven unambiguous | Approved; done. Zero conflicts, checked in CI by tools/check_lalr.py |
| 16 | Generated or hand-written parser, decided once 15 holds, by comparing error messages | Approved; to do: compare error messages on the pilot's real mistakes |

### What reaching zero conflicts changed (decision 15)

| Change | Why | Effect on what is written |
| --- | --- | --- |
| `labelled`, `matching`, `limited`, `added`, `only` are hard keywords | Each follows a value and could be read as part of it | They cannot be names |
| A record built with fields is a whole value or in brackets | Its last field ran into what followed: `a Point with x as 1 + 2` | `let p be a Point with …` is unchanged; inside a larger expression it is bracketed. Turned the pilot's misreading `an Order from cart and receipt` into an error |
| A comparison ending in `where` or a comprehension is a whole condition | The `where` covers the rest of the line, so nothing can follow it | Not usable as one link of an `and` chain; use a group |
| The amount in `add`/`subtract`, and a limit in a type, is one term | `add price of item to total`: the `to` must belong to `add` | A labelled call or arithmetic there is bracketed |
| An item of a one-line group holds no scope word | `any of x where c, y`: the `where` would take in `y` | Such an item goes on its own line |
| A pipeline stage is a call; the condition before the comma in `ensure if …,` is a plain condition | A record's commas ran into the phrase's comma | None in practice |
| `for each`, `at entry`, `, or else`, `, or`, and a comma ending a line, are read as one token by the lexer | Lets a one-token parser see both words | None: the text is unchanged |

## Language, not organisation

| # | Decision | Status |
| --- | --- | --- |
| 1 | Owners and teams are not part of the language. `trust`, `assume` and `assumes` say what is unchecked and why; the repository's own ownership rules say who reviews it | Approved; done |
| 2 | `workflow` (durable execution) is removed; it is a platform service. `idempotent by` stays | Approved; done |
| 3 | Audit the specification for other operational concepts, and bring each back as a decision | Approved; list presented |
| 4 | Only what the running program does, and what is checked about it, is language. Removed: monitoring policies on assumptions, a manifest's `failure` section (retries are written in code), a system's `failure domains`, and `scribe deploy plan` | Approved; done |
| 5 | Only what describes the program's instructions is language. Removed: `cost at most`, a component's `about` prose, and `corrects` (which described an earlier version of the program) | Approved; done |
| 6 | Reasons (`because "..."`) are removed everywhere; explanations go in `note` lines | Approved; done |
| 7 | Informal quoted clauses are removed: a quoted sentence is Text, not a condition, and the checker rejects it | Approved; done |
| 8 | `assumes` is removed; `trust` is a block the checker does not verify, with no reason string | Approved; done |
| 9 | Tiers and `proof` blocks are removed: the compiler proves what it can, checks the rest at run time and reports which; `assert` and `trust` are body statements | Approved; done |
| 10 | Contracts (`expect`, `ensure`, `changes only`, `fails with`, `uses`, `example`, `property`, `idempotent by`, invariants, `decreases`, system rules) stay: they state what the instructions do and are checked against them | Approved |
| 11 | Version numbers and schema compatibility modes move to the package file; schemas and migrations stay | Approved; done |
| 12 | A system's `components`, `authority` and `wiring` are removed: they repeated what the entry point does in code. `deadline` stays, as a run-time time limit | Approved; done |
| 13 | `idempotent by` is removed: nothing can enforce it | Approved; done |
| 14 | The language has no concept of time: `deadline`, `is after` / `is before`, date literals and built-in time types are removed; time is a standard-library matter, read through a clock capability. Version literals (`3.0.0`) went with component versions | Approved; done |
| 15 | `may block` is removed: waiting needs a capability, which `uses` already shows | Approved; done |
| 16 | `varies by host` is removed: a host-dependent fact is an operation of the capability's interface | Approved; done |
| 17 | System files are removed; `scribe query` answers reach and flow questions from the code | Approved; done |
| 18 | A condition group as the condition of `if` / `repeat while`, with `then` / `do` on its own line | Approved; done |
| 19 | "Owner" (memory) is defined in spec 7.1 | Approved; done |
| 20 | Principle: if in doubt, make it a library, as Rust does | Approved |
| 21 | Capability narrowing (`attenuated`, `files`, `network`, `database`, `host`, `port`, `under`, `read`, `write`, `limited`) becomes standard-library operations | Approved; done |
| 22 | A component's `may` section (`nothing of its own`) is removed: all authority starts at the entry point | Approved; done |
| 23 | Labels and `declassifies` become distinct types and named conversions | Approved; done |
| 24 | `define` / `holds when` removed: a predicate is an operation that produces a Bool | Approved; done |
| 25 | `implies` (and `⇒`), `pipeline`, `protocol` and `in build` are removed; pipelines and protocols are library code. `, or` is no longer joined by the lexer | Approved; done |
| 26 | Record members lose the word `field` | Approved; done |
| 27 | A quantifier over a type is `for every x as T` | Approved; done |
| 28 | `schema` and `migration` removed: a stored format is a type, and an upgrade is an operation | Approved; done |
| 29 | `enum` replaces `choice` (Rust, Swift, Scala 3) | Approved; done |
| 30 | Every keyword is a contract word, an instruction or a readability word, and no two look alike. Removed: `in state` (typestate), `matching`, `positive`, `non_empty`, `less`, `least`, `more`, `most`, `than`, `followed by`, `onto`, `over`, `using`, `anything but`; `, or else` becomes `, otherwise` | Approved; done |
| 31 | Look-alikes replaced: `fails with` → `can fail with`; `ensure if it succeeds/fails` → `ensure on success/failure`; `for every` → `for all`; `none of` → `not any of` | Approved; done |
| 32 | Typestate returns: `states Closed, Open` in a type, and a state as an adjective (`an Open Connection`); `editable`, `consumed`, `maybe` become hard. Protocols are a library pattern on typestate, not a keyword | Approved; done |
| 33 | `a User or nothing` replaces `a maybe User`; no `some` (flow narrowing after `is nothing`). Conditions in contracts call only pure operations. `match` replaces `choose`. `module` / `requires` / `exports` replace `component` / `needs` / `provides` | Approved; done |
