# Scribe: Syntax Philosophy and Worked Examples

Oct 14, 2026 · @Julian Harris

## Purpose and stance

Scribe's source form exists to be read, reviewed and diffed by humans, because most code will be written by machines. This document sets out the syntax rules, the past failures they are designed to avoid, the real problems they target, and then a gallery of worked examples from one-liners up to a multi-component system.

All syntax here is candidate syntax. The examples are the test: any rule that makes a realistic example worse should change, and the "Open questions" section lists the rules most likely to.

The stance in four sentences:

1. Code is written as sentences with a small, fixed, fully specified grammar.
2. Every program has exactly one canonical spelling. Other displays are views of it, never alternative ways to write it.
3. Whatever a reviewer must judge (effects, failure, authority, contracts, assumptions) is visible in the first lines of an operation, before its body.
4. Words replace symbols wherever the word is no less clear. Symbols stay only for arithmetic and grouping.

## Failure modes to avoid

Natural-language and multi-syntax languages have a poor track record, and the reasons are specific. Each row below is a known way this idea has gone wrong, with the Scribe rule written to prevent it. Descriptions of past languages are from general knowledge and should be checked before being quoted in a public spec.

| Failure mode | Where it happened | What went wrong | Scribe rule |
| --- | --- | --- | --- |
| English-like but ungrammatical | AppleScript | Users could read it but could not predict valid phrasing, so it earned a "read-only language" reputation | Small fixed grammar published as EBNF. No synonyms. The compiler and editor offer the one valid phrasing when you miss |
| Optional filler words | HyperTalk, AppleScript ("the", "of", "in") | Two valid spellings of one statement, so style drifts and tools disagree | Every word is required or forbidden. No optional filler |
| Dialects stored in the source | AppleScript's localized dialects, Reason versus OCaml syntax, Perl's "more than one way" | Teams cannot share, search or review each other's code; communities split | One canonical stored form. Views are display only and must round-trip losslessly |
| Per-application vocabularies | AppleScript dictionaries | The same word means different things depending on the target app | Keywords are fixed by the language. Domain words come from a declared, imported vocabulary and can never redefine a keyword |
| Verbosity for its own sake | COBOL, long SQL clauses | Boilerplate buries the information a reviewer needs | Density budget per line, enforced by lint. A compact view exists for experts |
| Ambiguous parsing of prose | Free-form English chains ("the name of the first item of the list of ...") | Meaning depends on parser quirks | One phrase form per construct. Field chains limited to two steps by lint |
| Hard to write, so abandoned | AppleScript | Authors gave up guessing phrasing | Grammar-driven completion in editors. Machine authors use the same grammar as a constrained decoder |
| Layout traps | Python mixed tabs, YAML, Makefile | Invisible characters change meaning | Four spaces only. Tabs are a syntax error. The formatter is the single source of layout |
| Symbol soup | Perl, regexes, Rust generic and lifetime noise | Density hides meaning | Symbols only for arithmetic and grouping. Patterns get names, not sigils |
| Implicit conversion | JavaScript coercion, PHP loose equality | Comparisons that surprise | No implicit conversions. `is` compares like with like only |
| Fragmented tooling | Any language with several dialects | Formatters, linters and docs each need a copy per dialect | Tools operate on the canonical form only. Views are generated, never parsed as a separate language |
| Localization splits the community | Localized-keyword languages | Search results, Q&A and docs fragment | Localized keywords exist only as a display view. Docs, errors and search always use canonical words |

## Real pain points this solves

A language earns adoption by removing pain people already feel. Each row is a daily problem in current languages, what usually happens today, and the Scribe feature that addresses it. The last column points to the example section that shows it.

| Pain today | What happens now | Scribe answer | Shown in |
| --- | --- | --- | --- |
| "What breaks if I change this?" cannot be answered before the change | Grep, tests, hope, then production | Manifests plus a `scribe impact` dry run with proofs, witnesses and a "needs review" bucket | Big examples 1 and 2 |
| Side effects and failures hide inside function bodies | Reviewers read every body to find network calls or thrown exceptions | `uses` and `fails with` sit in the header of every operation that has them, and no uses line means pure | Small examples |
| Reviewing large AI-written patches | Humans skim thousands of changed lines | Spec diff first: authority, assumptions and contract changes are shown before code | Big example 2 |
| `=` versus `==`, assignment in conditions | Classic bug class in C-family languages | `set x to` for mutation, `is` for comparison. No assignment inside expressions | Small examples |
| Missing values | Crashes far from the cause | `maybe T` with `some` and `nothing`. No null | Data and types |
| Silent integer overflow and off-by-one loops | Wrong results with no error | Checked arithmetic, and ranges spelled `until` (excludes the end) or `through` (includes it) | Small examples |
| Implicit conversions and loose equality | `"1" == 1` surprises | No implicit conversion | Data and types |
| Ambient authority and supply-chain attacks | Any dependency can read files or open sockets | Capabilities passed explicitly, attenuated, and checkable by query | Capabilities |
| Semver that lies | Patch releases break callers | Versions computed from manifest diffs | Big example 1 |
| Config and environment assumptions live in a wiki | Outages when an upstream behaves differently | `assumes ... assumed` with an owner and a runtime monitor that names who is to blame | Big example 2 |
| Duplicate side effects on retry | Double charges, duplicate emails | `idempotent by` is declared and checked | Big example 1 |
| Schema drift between services and stored data | Deserialization errors after a deploy | Schemas with declared compatibility modes and checked migrations | Big example 1 |
| Data races and orphaned tasks | Heisenbugs | Structured concurrency only. A task cannot outlive its scope | Big example 3 |
| Docs and specs drift from code | Stale diagrams and READMEs | The manifest is the doc, and it is checked on every build | Big example 1 |
| Injection (SQL, shell, HTML) | Untrusted text reaches a sink | Information-flow labels: untrusted text cannot reach a query without validation | Capabilities |
| Unit and time-zone mix-ups | Spacecraft and billing bugs | Units in types. `instant` and `calendar date` are different types | Data and types |

## Core rules and the view model

The rules below are normative for this draft. The vocabulary table that follows is what every example in this document uses, so it doubles as a cheat sheet.

1. **One canonical form.** The canonical text is what is committed, hashed, diffed and quoted in errors and docs.
2. **Statements lead with a keyword.** The first word of a line tells you what kind of line it is: `let`, `set`, `if`, `expect`, `ensure`, `uses`.
3. **Every word is required or forbidden.** No optional filler, no synonyms.
4. **School-maths symbols for arithmetic and comparison, words for logic.** Stored forms are `+ - * /`, `< > <= >= != =`, the word `mod`, and parentheses. The renderer draws `<=`, `>=` and `!=` as ≤, ≥ and ≠. Words stay for `and`, `or`, `not`, `in` and `implies`. The parser accepts the aliases in the notation table below.
5.
6. **No assignment inside expressions.** A binding or mutation is always a whole line.
7. **Field access is `X of Y`.** Chains are limited to two steps by lint. Deeper access needs a named `let`.
8. **Layout is four spaces.** Blocks are opened by a header line and an indent. There are no braces, colons or semicolons.
9. **Names are single words with underscores.** Spaces inside names are not allowed. This keeps the grammar unambiguous and is the first thing to test with readers. The review view draws the underscores at about 50 percent contrast, with a floor set by the theme for accessibility, so a name reads as separate words while staying one searchable, copyable token.
10. **Mixing `and` with `or` requires parentheses.** There are at most six precedence levels in the language.

### Views

A view is a generated rendering of the canonical text. It never changes meaning, hash or diff, and it must round-trip: converting canonical to a view and back gives byte-identical canonical text. A view that cannot meet that test does not ship.

| View | Audience | What changes | Example of the same comparison |
| --- | --- | --- | --- |
| Review (default) | Reviewers reading AI-written code | Nothing. It is the canonical text, with contracts and manifest items highlighted, a pure tag beside any operation with no uses line, and proof hints folded | `if balance ≤ 0 or closed` |
| Compact | Experts who find words tedious | Logic words shown as symbols (∧ ∨ ¬ ∈ ∀ ∃ ⇒) | `if balance ≤ 0 ∨ closed` |
| Localized | Readers in other languages | Keywords shown in another language. Names and strings are untouched | translated `if`, `is at most` |

### Vocabulary used in every example

| Purpose | Canonical form |
| --- | --- |
| Define an operation | `to withdraw amount from account` |
| Parameter types and conventions | `given amount is a positive Money` and `given account is an editable Account` (conventions: read by default, `editable`, `consumed`) |
| Result and failure | `produces a Receipt` and `fails with InsufficientFunds` |
| Precondition | `expect amount is more than 0` |
| Postcondition | `ensure balance of account is balance of account at entry - amount` |
| Effects | `uses ledger` (lists the capabilities the operation exercises). With no `uses` line the operation is pure |
| Immutable binding | `let total be price * quantity` |
| Mutable variable | `var count starts at 0`, then `set count to count + 1` |
| Return a value | `give back total` |
| Fail | `fail with InsufficientFunds` |
| Propagate a failure | `try withdraw 50 from account` |
| Handle a failure | `handle` block, then `on failure CardDeclined` |
| Conditions | `if`, `otherwise if`, `otherwise` |
| Loops | `for each item in items`, `for each i from 0 until n`, `for each i from 1 through n`, `repeat while condition` |
| Early exit | `stop` (leave the loop) and `skip` (next iteration) |
| Match | `choose shape` with `when Circle with radius` lines |
| Optional values | `maybe Position`, `some 3`, `nothing` |
| Lists | `[1, 2, 3]`, `count of items`, `item i of items` |
| Comparison | `is`, `is not`, `is less than`, `is at most`, `is more than`, `is at least` |
| Logic | `and`, `or`, `not`, `implies` |
| Quantifiers in specs | `for every i from 0 until n, ...` and `there is an item in items where ...` |
| Comments | a line starting with `note` |

Note on `at entry`: inside an `ensure`, `X at entry` means the value of X when the operation began. It replaces the `old(x)` of Eiffel and SPARK with a phrase that reads aloud.

### Notation, aliases and renderers

Three things are kept apart: what is stored, what the parser accepts, and what a reader sees.

1. **Stored form.** The formatter rewrites every program to it on save, so a diff never shows a change of spelling alone. It is plain ASCII, so search and terminals work.
2. **Accepted forms.** The parser also takes the aliases in the notation table below: maths symbols, English phrases and C-style symbols. Each alias maps to exactly one core operator. The table is part of the language version, so it is closed and the same everywhere.
3. **Displays.** Each view and each reader's settings choose how an operator is drawn.

Identity comes from the parsed tree, not from the text. Spelling never changes a component's hash, a spec diff or an impact report.

| Core meaning | Stored | Also accepted | Review view | Compact view |
| --- | --- | --- | --- | --- |
| less than | `<` | `is less than` | `<` | `<` |
| at most | `<=` | `≤`, `is at most` | `≤` | `≤` |
| at least | `>=` | `≥`, `is at least` | `≥` | `≥` |
| more than | `>` | `is more than` | `>` | `>` |
| not equal | `!=` | `≠`, `is not` | `≠` | `≠` |
| equal | `=` | `==`, `is` (see below) | `=` | `=` |
| and | `and` | `∧`, `&&` | `and` | `∧` |
| or | `or` | `∨`, `\|\|` | `or` | `∨` |
| not | `not` | `¬`, `!` | `not` | `¬` |
| member of | `in` | `∈` | `in` | `∈` |
| for all | `for every` | `∀` | `for every` | `∀` |
| there is | `there is` | `∃` | `there is` | `∃` |
| implies | `implies` | `⇒` | `implies` | `⇒` |

The school-maths symbols (`< > ≤ ≥ ≠ =` and arithmetic) appear in every view. Symbols that need university maths or logic (`∧ ∨ ¬ ∈ ∀ ∃ ⇒`) appear only in the compact view. Symbols that exist only in programming (`! && ||`) are accepted as input and never displayed.

Chained comparisons keep their maths meaning, and they replace some of the longest lines in the examples:

```
invariant 0 <= low <= high <= count of sorted_items
```

In the review view the same line is drawn `invariant 0 ≤ low ≤ high ≤ count of sorted_items`.

Rules for aliases:

- Mixed spellings in one file are accepted on input and normalised on save.
- A project may switch on a lint that rejects aliases instead of rewriting them.
- Scribe has no assignment symbol, so `=` always means equality. A line such as `count = 1` is a comparison, not a statement, and gets the message: "this compares two values. To change a variable, write `set count to 1`".
- `is` is also used to test a variant, as in `status is Paid`. Rewriting `is` to `=` for plain values needs types, so the formatter does it after name resolution. This is an open point.
- Models trained on C-family code will tend to write `&&`, `==` and `!`. Accepting them means machine authors are never rejected for spelling, and the formatter fixes the text.

## Small examples

These show everyday code in canonical form. Look first at the header lines of each operation: everything a reviewer needs to judge it is there before the body starts. An operation with no uses line is pure and touches nothing outside its arguments. Membership in specs is written `is in`.

### 1. A pure function with a refined input

```
to greet name
    given name is Text with length 1 through 100
    produces Text

    give back "Hello, {name}"
```

### 2. A loop with its reasoning attached

```
to largest items
    given items is a List of Int
    expect count of items is at least 1
    produces an Int
    ensure result is in items
    ensure for every x in items, x is at most result

    var best starts at item 0 of items
    for each i from 1 until count of items
        invariant best is in items
        invariant for every j from 0 through i - 1, item j of items is at most best
        if item i of items is more than best
            set best to item i of items
    give back best
```

A `for each` loop over a range always terminates, so it needs no `decreases` line. A `repeat while` loop does.

### 3. Ranges that cannot be misread

```
note 1 through 10 includes 10. 1 until 10 stops at 9.

to sum_up_to limit
    given limit is a Nat
    produces a Nat
    ensure result is limit * (limit + 1) / 2

    var total starts at 0
    for each n from 1 through limit
        invariant total is (n - 1) * n / 2
        set total to total + n
    give back total
```

### 4. Optional results

```
to find target in items
    given target is an Int
    given items is a List of Int
    produces a maybe Position
    ensure if result is some p, item p of items is target
    ensure if result is nothing, target is not in items

    for each i from 0 until count of items
        invariant for every j from 0 until i, item j of items is not target
        if item i of items is target
            give back some i
    give back nothing

to describe_search target in items
    given target is an Int
    given items is a List of Int
    produces Text

    choose find target in items
        when some p
            give back "found at position {p}"
        when nothing
            give back "not found"
```

A caller cannot use the result of `find` without a `choose`. Leaving out the `nothing` case is a compile error.

### 5. Choices with exhaustive matching and units

```
choice Shape
    Circle with radius as a positive Length
    Rectangle with width as a positive Length and height as a positive Length

to area_of shape
    given shape is a Shape
    produces an Area

    choose shape
        when Circle with radius
            give back pi * radius * radius
        when Rectangle with width and height
            give back width * height
```

Adding a third kind of shape makes every `choose shape` in the program fail to compile until it is handled. Multiplying two lengths gives an area, and adding a length to an area is a type error.

### 6. Failure is visible, propagated or handled

```
to parse_port text
    given text is Text
    produces a Port
    fails with NotANumber, OutOfRange

    let number be try parse_int text
    if number is less than 1 or number is more than 65535
        fail with OutOfRange
    give back number

to port_or_default text
    given text is Text
    produces a Port

    handle
        give back try parse_port text
    on failure NotANumber or OutOfRange
        give back 8080
```

`try` means "pass any failure up". Inside a `handle` block it passes the failure to the `on failure` lines. An `on failure` line may not be empty: it must give back a value, fail, or call something. `port_or_default` has no `fails with` line because it handles every failure of its callee, and the compiler checks that.

### 7. Binding versus mutation

```
let rate be 0.2
var attempts starts at 0

set attempts to attempts + 1
set rate to 0.3
```

The last line is an error: `rate` was bound with `let`. The message is shown in the diagnostics section.

### Same bug, two languages

| Bug | Typical C-family code | In Scribe |
| --- | --- | --- |
| Assignment in a condition | `if (x = 0)` | Not expressible. `set` is always a whole line, and equality is `if x is 0` |
| Off by one | `for (i = 0; i <= n; i++)` | `until` and `through` say which end is included, and the loop invariant is checked against the bound |
| Missing value used as present | `user.name` when `user` has no value | `maybe User` must go through `choose` before any field is used |
| Swallowed error | `catch (e) {}` | An `on failure` line must name the failure and must do something. A silent catch-all is rejected |
| Mixed operators | \`a |  |

## Contracts, invariants and specifications

Contracts are written as sentences between the signature and the body. Reviewers read these lines; machines check them. Two small conventions: `of` binds tighter than any operator, so `count of items - 1` means the count minus one, and parentheses are required wherever that is not enough, as in `item (i + 1) of items`.

### 8. A full contract with success and failure cases

```
to withdraw amount from account
    given amount is a positive Money
    given account is an editable Account
    produces a Receipt
    fails with InsufficientFunds
    expect account is open because "closed accounts cannot be debited"
    ensure if it succeeds, balance of account is balance of account at entry - amount
    ensure if it fails, balance of account is balance of account at entry
    changes only balance of account

    if amount is more than balance of account
        fail with InsufficientFunds
    set balance of account to balance of account - amount
    give back receipt_for amount
```

`because "..."` attaches a human sentence to any clause. It becomes the diagnostic text, the API documentation and the label in impact reports. `changes only` is the frame: anything not listed is guaranteed untouched. Changing an `editable` argument is not an effect, because ownership already says who can see it.

### 9. A type with an invariant and a model

```
type Queue of T
    field capacity as a Count
    field items as a Ring of T
    model contents as sequence_of items
    invariant count of contents is at most capacity because "never holds more than capacity"

to push value onto queue
    given value is a T
    given queue is an editable Queue of T
    fails with QueueFull
    ensure if it succeeds, contents of queue is contents of queue at entry followed by value
    ensure if it fails, contents of queue is contents of queue at entry

    if count of contents of queue is capacity of queue
        fail with QueueFull
    append value to items of queue
```

The `model` line gives the abstract state that specs talk about, so contracts never mention the ring buffer. The invariant holds whenever control leaves an operation of the type. `followed by` joins sequences.

### 10. A `repeat while` loop with a variant

```
to search target in sorted_items
    given target is an Int
    given sorted_items is a List of Int
    expect sorted_items is sorted
    produces a maybe Position
    ensure if result is some p, item p of sorted_items is target
    ensure if result is nothing, target is not in sorted_items

    var low starts at 0
    var high starts at count of sorted_items
    repeat while low is less than high
        invariant 0 is at most low and low is at most high and high is at most count of sorted_items
        invariant for every j from 0 until low, item j of sorted_items is less than target
        invariant for every j from high until count of sorted_items, item j of sorted_items is more than target
        decreases high - low
        let middle be low + (high - low) / 2
        if item middle of sorted_items is target
            give back some middle
        otherwise if item middle of sorted_items is less than target
            set low to middle + 1
        otherwise
            set high to middle
    give back nothing
```

`decreases` names a quantity that gets strictly smaller on every pass and cannot go below zero. That is the termination proof, and a reader can check it by eye.

### 11. Named predicates keep specs readable

```
define items is sorted
    given items is a List of Int
    holds when for every i from 0 until count of items - 1, item i of items is at most item (i + 1) of items

define items is a permutation of others
    given items is a List of Int
    given others is a List of Int
    holds when for every x, occurrences of x in items is occurrences of x in others
```

The standard library ships a vocabulary of these, already proven, so contracts read as `expect sorted_items is sorted` rather than as nested quantifiers.

### 12. Refinement types

```
type Percent is an Int with value 0 through 100
type Email is Text matching email_format
type Position is a Nat less than count of items
```

A refinement carries its condition everywhere the type goes, so the condition is written once instead of in every `expect`.

### 13. Proof hints are separate, foldable and last

```
to reverse items
    given items is a List of Int
    produces a List of Int
    ensure count of result is count of items
    ensure for every i from 0 until count of items, item i of result is item (count of items - 1 - i) of items
    tier proven

    ... implementation ...

    proof
        use lemma reverse_keeps_length
        assert count of result is count of items
```

In the review view the `proof` block is folded by default. Reviewers judge the `ensure` lines, which say what is promised, and the tool judges the proof.

### 14. Tiers and escape hatches are explicit

```
to roll_die generator
    given generator is a RandomSource
    produces an Int with value 1 through 6
    tier checked
    uses generator

    trust "hardware generator returns uniform bits" owner security_team
    give back 1 + bits_from generator mod 6
```

Every `trust` and `assume` names a reason and an owner, is counted per package, and shows up in every review and impact report that depends on it.

### 15. Examples and properties run as tests

```
example largest [3, 9, 2] is 9
example largest [5] is 5
example find 7 in [1, 7, 7] is some 1
property for every items that is a non-empty List of Int, largest items is in items
```

Examples are executable documentation. Properties are generated from the contract and run as tests when a clause is only at the `tested` tier.

## Data, types, errors and resources

The same rules apply to data definitions: one form for each idea, and words that say what is being declared.

### 16. Records and choices

```
type Order
    field id as an OrderId
    field customer as a CustomerId
    field lines as a List of OrderLine
    field placed_at as an Instant
    field status as an OrderStatus
    invariant lines is not empty or status is Draft because "only drafts may be empty"

type OrderLine
    field product as a ProductId
    field quantity as a Positive Int
    field unit_price as Money

choice OrderStatus
    Draft
    Paid with paid_at as an Instant
    Shipped with tracking as Text
    Cancelled with reason as Text
```

`X is Paid` tests the variant. Reading the payload (`paid_at`) requires a `choose`, so a draft's payment time can never be read by mistake.

### 17. Units and time are types

```
let timeout be 30 seconds
let distance be 5 kilometres
let speed be distance / 2 hours
let cutoff be started_at + 2 hours
let due be date 2026-10-14
let closes be start_of due in zone Europe_London
let wrong be due + started_at
```

The last line is rejected: a calendar date and an instant are different types, and converting one into the other needs a time zone, which is written out (`start_of due in zone Europe_London`). `speed` is inferred to be a speed. Adding a distance to a time is a type error.

### 18. Collections in words

```
to total_paid orders
    given orders is a List of Order
    produces Money
    ensure result is sum of total_of order for each order in orders where status of order is Paid

    var total starts at zero_money
    for each order in orders
        if status of order is Paid
            set total to total + total_of order
    give back total
```

The spec uses the same comprehension form that code may use: `X for each item in items where condition`. There is one comprehension form and one loop form, and no `map`, `filter` or `reduce` spelled differently.

### 19. A resource that must be released

```
to first_line_of path using files
    given path is a Path
    given files is a FileSystem limited to "/reports"
    produces Text
    fails with FileError
    uses files

    hold input as open path for reading using files
        give back try read_line from input
```

`hold` releases the resource on every exit from the block: normal, early return or failure. A file is a linear type. If it were bound with `let` and never released, the compiler would reject the operation, naming the path on which the file leaks.

### 20. A protocol the compiler enforces

```
type Connection in state Closed or Open

to open_connection address using network
    given address is an Address
    given network is a Network limited to host "api.example.com"
    produces a Connection in state Open
    fails with NetworkError
    uses network

to send data over connection
    given data is Bytes
    given connection is an editable Connection in state Open
    fails with NetworkError
    uses connection

to close connection
    given connection is a consumed Connection in state Open
    produces a Connection in state Closed
    uses connection
```

Calling `send` on a connection in state `Closed`, or never closing an opened connection, is a compile error. There is no runtime check to forget.

### 21. Allocation is visible

```
to split_lines text using memory
    given text is Text
    given memory is an Allocator
    produces a List of Text
    fails with OutOfMemory
    uses memory

let lines be try split_lines text using memory
```

An operation that allocates must say so in `uses`, and the allocator is a value passed in. There is no hidden allocation and no garbage collector.

## Capabilities, effects and protocols

Authority is a value, passed explicitly, and can only get narrower as it is handed down. This is the foundation of the impact checks in the big examples.

### 22. Authority enters at one place and is narrowed

```
to main root
    given root is the Authority
    uses root

    let reports be root attenuated to files under "/reports" read only
    let partner be root attenuated to network host "export.partner.example" port 443
    run ReportExporter with reports and partner
```

Nothing else in the program can open a file or a socket, because there is no other source of authority. `attenuated to` can only narrow, so a component can never hold more than it was given.

### 23. Asking what a component can reach

```
$ scribe query reach ReportExporter network
ReportExporter can reach: host "export.partner.example" port 443 only
    path: main > root attenuated > partner > ReportExporter

$ scribe query reach ReportExporter files
ReportExporter can reach: "/reports" read only
```

These answers are static and complete for the checked tier. "Can this dependency ever reach the network?" stops being a research project.

### 24. Untrusted data cannot reach a sink

```
to find_user name using database
    given name is Text labelled untrusted
    given database is a Database
    produces a maybe User
    fails with InvalidName, DatabaseError
    uses database

    let query be "select * from users where name = {name}"
    ...
```

The `let query` line is rejected: untrusted text cannot be joined into a database query. The corrected body:

```
    let valid_name be try check name as UserName
    give back try fetch User from database where name is valid_name
```

`check ... as UserName` is the only way to turn untrusted text into a `UserName`, and `fetch ... where` sends the value as a parameter, never as query text.

### 25. Declassification is named, justified and visible

```
to masked_card card
    given card is a CardNumber labelled secret
    produces Text labelled public
    declassifies secret to public because "only the last four digits are shown"

    give back "**** {last 4 digits of card}"
```

`declassifies` is part of the header, so it appears in the manifest and in every impact report. A reviewer sees the few places where secrets are allowed to become public, and a change that adds one is flagged as high risk.

### 26. Pure means pure

```
to price_with_tax price
    given price is Money
    produces Money

    let rate be try fetch_tax_rate
    give back price * (1 + rate)
```

Rejected: `fetch_tax_rate` uses `network`, and this operation has no `uses` line, so it is pure. The error names the call and the effect it brings in. The fix is to take the rate as a parameter or to declare `uses tax_service`.

### 27. A protocol declared once, checked on both sides

```
protocol Upload
    client sends Begin with size as Bytes at most 1 GiB
    repeat
        client sends Chunk with data as Bytes
        server sends Ack with offset as Bytes
    until client sends Commit
    server sends one of Done with checksum as a Hash, or Failed with reason as Text

to serve_upload connection
    given connection is an editable Upload as server
    fails with ProtocolError
    ensure if it succeeds, checksum sent is hash of every Chunk data received
    uses connection
```

The client and server code are both checked against the one protocol. Sending `Commit` before `Begin`, or forgetting to answer a `Chunk`, is a compile error on whichever side does it. A new version of the protocol is accepted only if it is a refinement of the old one.

## Big example 1: a complete component

This is a checkout service, the kind of component that causes real incidents when changed: it charges money, talks to three other components, and keeps data. A component file has the manifest first, then the implementation. A reviewer can stop after the manifest and still judge the component.

### 28. The manifest

```
component Checkout version 3.0.0

    about
        Turns a cart into a paid order. Charges a customer at most once per request.

    needs
        Payments from 3.1 as payments
            uses charge, refund, total_charged_to
        Inventory from 2.0 as inventory
            uses reserve, release
        Orders_store from 1.2 as store
            uses save

    may
        nothing of its own

    assumes
        payments answers within 2 seconds for 99 percent of calls    assumed owner payments_team monitor sampling 1 percent
        charging with the same key twice charges once               assumed owner payments_team monitor checking

    provides
        to place_order cart for customer with request_id
            given cart is a Cart
            given customer is a CustomerId
            given request_id is a RequestId
            produces an Order
            fails with EmptyCart, OutOfStock, PaymentDeclined, StoreUnavailable, RefundFailed
            expect cart is not empty because "an order needs at least one line"
            ensure if it succeeds, status of result is Paid
            ensure if it succeeds, total of result is total of cart
            ensure if it fails with anything but RefundFailed, total_charged_to customer is total_charged_to customer at entry
            idempotent by request_id
            uses payments, inventory, store

    failure
        domain checkout
        retry charge at most 3 times with backoff 1 second because "charge is idempotent by key"

    tier proven for contracts, checked for everything else
```

Read it top to bottom. It holds no authority of its own, so everything it can do happens through the three components it needs, and only through the operations it lists. It states two facts about the outside world that nobody can prove, with an owner for each. It promises exactly-once charging, and the retry rule says why retrying is safe.

### 29. The implementation below it

```
to place_order cart for customer with request_id
    if cart is empty
        fail with EmptyCart
    let reservation be try reserve lines of cart in inventory
    handle
        let receipt be try charge customer for total of cart in payments with key request_id
        let order be an Order from cart and receipt with status Paid
        handle
            try save order in store
        on failure StoreUnavailable
            try refund receipt in payments
            fail with StoreUnavailable
        give back order
    on failure PaymentDeclined or StoreUnavailable as problem
        release reservation in inventory
        fail with problem
```

The signature is not repeated here. The compiler checks this body against the manifest above. If the save step failed without the refund, the contract line "if it fails, the customer's total charged is unchanged" could not be proved, and the build would say exactly that (see the diagnostics section).

### 30. A versioned schema and a checked migration

```
schema Order version 4 compatibility backward_transitive
    id           as an OrderId
    customer     as a CustomerId
    lines        as a List of OrderLine
    status       as an OrderStatus
    note         as a maybe Text     added in version 4
    discount     reserved            removed in version 3

migration Order from version 3 to version 4 turning old into new
    ensure note of new is nothing
    ensure for every field other than note, that field of new is that field of old
```

`backward_transitive` means new code can read data written by every earlier version, which is what stored data needs. Reusing the name of a removed field is rejected. The migration has a contract like any operation, so "the migration loses nothing" is checked, not asserted.

### 31. A change as the reviewer sees it

The manifest and body above are the new version. Version 2.4.0 had no refund step, so `RefundFailed` becomes a new way for `place_order` to fail.

```
$ scribe review
Checkout 2.4.0 > 3.0.0   (computed: major)

AUTHORITY    no change
ASSUMPTION   + charging with the same key twice charges once    owner payments_team   (new)
CONTRACT     + fails with RefundFailed    new failure mode that callers must handle
CONTRACT     ensure if it fails ...       weakened: now excludes RefundFailed
CODE         34 lines changed in 1 operation   (folded)

The version is computed from this diff. A person may raise it but not lower it.
```

The author did not choose "minor" or "patch", so no one gets to be optimistic about it. Adding a failure mode is a breaking change for any caller with an exhaustive `on failure`, and Big example 2 shows the callers that find out.

## Big example 2: a system, a change and its blast radius

This example puts seven components into one system and follows a change through it. It is the answer to the question that started this design: what will this change break, before I make it?

### 32. The system file

A system file does four things: lists the components, says who holds authority, says who needs whom, and states rules about the whole.

```
system Shop version 12

    components
        Storefront 5.1
        Checkout 2.4
        Payments 3.1
        Inventory 2.0
        Orders_store 1.2
        Notifications 1.7
        Reporting 4.0

    authority
        root gives Payments       network host "payments.example.com" port 443
        root gives Inventory      database "stock" read and write
        root gives Orders_store   database "orders" read and write
        root gives Notifications  network host "mail.example.com" port 587
        root gives Reporting      database "orders" read only
        nothing else holds authority

    wiring
        Storefront needs Checkout
        Checkout needs Payments, Inventory, Orders_store
        Notifications needs Orders_store (read operations only)
        Reporting needs Orders_store (read operations only)

    failure domains
        checkout contains Checkout, Payments, Inventory, Orders_store
        comms contains Notifications
        analytics contains Reporting
        a failure in comms must not stop checkout
        a failure in analytics must not stop checkout

    rules
        only Payments can reach host "payments.example.com"
        no value labelled card_data leaves Payments
        Reporting cannot write to database "orders"
```

The whole file fits on one screen. A new engineer, an auditor, or an AI agent about to make a change can learn the shape of the system from it.

### 33. What the checker proves about the whole system

```
$ scribe check Shop
authority      PROVEN   only Payments can reach host "payments.example.com"
authority      PROVEN   Reporting cannot write to database "orders"
information    PROVEN   no value labelled card_data reaches a component outside Payments
failure        PROVEN   no dependency path from the checkout domain into comms or analytics
assumptions    4 assumed facts, owned by payments_team and stock_team, all with monitors
trusted        tls_library 1.1.0 (foreign code)
```

These are the claims that normally live in architecture diagrams and wikis. Here they are checked on every build.

### 34. A rule that catches a quiet mistake

Someone adds an order confirmation email to the checkout path, in what looks like a small patch:

```
component Checkout version 2.4.1
    needs
        Notifications from 1.7 as notifications
            uses send_confirmation
```

```
$ scribe check Shop
ERROR  failure domain: the checkout domain now depends on the comms domain
       path: Checkout > Notifications
       rule: "a failure in comms must not stop checkout" (Shop, failure domains)
       fix:  declare the need as best effort, which absorbs its failures and cannot fail the caller:
                 Notifications from 1.7 as notifications, best effort
             or send the email from a component inside comms that reads Orders_store
```

The rule was written once by someone who knew about the last outage. It now protects the system from every future author, human or machine.

### 35. A change and its impact report

Now the real change from Big example 1: Checkout goes from 2.4.0 to 3.0.0 because `place_order` can fail with `RefundFailed`. Before merging, anyone can run:

```
$ scribe impact --against main
Checkout 2.4.0 > 3.0.0   manifest 7f3a91 > c91e04

SPEC DIFF (highest risk first)
  AUTHORITY    no change
  ASSUMPTION   + charging with the same key twice charges once    owner payments_team
  CONTRACT     + fails with RefundFailed

DEPENDENTS
  Storefront 5.1
    BREAKING            checkout_button calls place_order
                        witness: place_order can now fail with RefundFailed, which
                        checkout_button neither handles nor declares
                        suggestion: add `on failure RefundFailed`, or list it under `fails with`
    PROVEN-COMPATIBLE   order_history (does not use place_order)
  Notifications 1.7     not affected (does not use Checkout)
  Reporting 4.0         not affected (does not use Checkout)

SYSTEM PROPERTIES
  only Payments can reach host "payments.example.com"   PROVEN (unchanged)
  failure domains                                        PROVEN (unchanged)
  no card data leaves Payments                           PROVEN (unchanged)

RELIES ON
  assumed: 2 facts (payments_team)
  trusted: tls_library 1.1.0

VERDICT: BLOCKED until Storefront handles RefundFailed
```

The report names the one component that breaks, the exact call that breaks it, and a concrete way it would break. It also says what is not affected, which is the half of impact assessment that people usually cannot get.

### 36. The fix, and the order to deploy in

```
component Storefront version 5.2.0
    needs
        Checkout from 3.0 as checkout
            uses place_order
        Support from 1.0 as support
            uses raise_ticket

to checkout_button cart for customer with request_id
    given cart is a Cart
    given customer is a CustomerId
    given request_id is a RequestId
    produces a CustomerMessage
    uses checkout, support

    handle
        let order be try place_order cart for customer with request_id in checkout
        give back message "Thank you. Your order {id of order} is confirmed."
    on failure EmptyCart
        give back message "Your basket is empty."
    on failure OutOfStock
        give back message "An item in your basket is no longer available."
    on failure PaymentDeclined
        give back message "Your payment was declined."
    on failure StoreUnavailable
        give back message "We could not save your order. You have not been charged."
    on failure RefundFailed
        try raise_ticket "refund failed for request {request_id}" in support
        give back message "Your payment needs manual review. We will email you today."
```

```
$ scribe deploy plan production with Checkout 3.0.0 and Storefront 5.2.0
Production today: Storefront 5.1.0, Checkout 2.4.0

step 1  deploy Storefront 5.2.0   compatible with Checkout 2.4.0 and with 3.0.0
step 2  deploy Checkout 3.0.0     compatible with Storefront 5.2.0, NOT with Storefront 5.1.0

Never run Storefront 5.1.0 beside Checkout 3.0.0. A rollback of step 1 after step 2 is refused.
```

The deploy order falls out of the compatibility facts. Storefront 5.2.0 can safely arrive early because it only adds a handler for a failure that the old Checkout never produces. The check is the same machinery as the impact report, applied to what is running in production instead of what is on a branch.

### Why this is different from today

| Question | Today | In Scribe |
| --- | --- | --- |
| What breaks if I add a failure mode? | Search for callers, read their error handling, hope | Computed from the callers' own `on failure` lines |
| Can any dependency reach the payment host? | Audit by reading code and configs | A static query with a path as evidence |
| Is this a patch, minor or major release? | The author's judgement | Computed from the diff of the manifest |
| In what order do we deploy? | A runbook and a meeting | Derived from compatibility between versions |
| Did someone quietly couple two failure domains? | Found in the next outage | Rejected at build time with the rule that caught it |

## Big example 3: concurrency, pipelines and long-running work

Concurrency is where readable languages usually give up. The rule here is that concurrent structure is written as a visible shape in the code, and that a task can never outlive the block that started it.

### 37. Doing several things at once

```
to load_dashboard user using services
    given user is a UserId
    given services is the DashboardServices
    produces a Dashboard
    fails with DashboardUnavailable
    deadline 800 milliseconds
    uses services

    together
        let profile be fetch profile of user from services
        let orders be fetch recent orders of user from services
        let alerts be fetch alerts of user from services, or else none
    give back a Dashboard from profile and orders and alerts
```

Everything inside `together` starts at once, and the block does not finish until every line has finished. If a required line fails, the others are cancelled and the failure goes up. `or else none` marks the one line whose failure is acceptable. The `deadline` is a budget for the whole operation. It is monitored at run time, not proved, and the tool says so, because the latency of `services` is an assumed fact.

### 38. The race that cannot be written

```
var total starts at 0
together
    for each a in first_half
        set total to total + a
    for each b in second_half
        set total to total + b
```

Rejected: both branches change `total`. The error names both lines. The corrected form gives each branch its own result:

```
together
    let first be sum of first_half
    let second be sum of second_half
give back first + second
```

Because values are never shared mutably between tasks, there is nothing for a lock to protect, and no way to forget one.

### 39. A pipeline with a bounded queue

```
to import_orders source into store using memory
    given source is a File of Rows
    given store is an Orders_store
    given memory is an Allocator
    produces an ImportSummary
    fails with ImportFailed
    ensure every row read is either saved or listed in rejects of result
    uses source, store, memory

    pipeline
        stage read_rows from source
        queue of at most 100 rows
        stage parse_row, 4 at a time
        queue of at most 100 orders
        stage save_order into store, 2 at a time
    on failure in any stage
        stop all stages
        fail with ImportFailed
```

Every queue has a size, so there is no unbounded memory growth: a full queue makes the stage before it wait, which is backpressure. A row that cannot be parsed goes to `rejects` rather than disappearing, and the `ensure` line says so.

### 40. Work that survives a restart

```
workflow nightly_billing for date
    given date is a CalendarDate
    produces a BillingSummary
    runs at most once per date
    survives restarts
    uses billing

    let accounts be list_billable_accounts for date in billing
    for each account in accounts
        remember progress by account
        try bill account for date in billing
    give back summary of accounts
```

`survives restarts` and `remember progress by account` mean that if the machine dies half way, the run resumes after the last finished account. This is only safe if every effect in the loop can be repeated harmlessly, and the compiler checks that:

```
ERROR  nightly_billing: a restart could repeat "bill" for the same account
       bill is not declared `idempotent by` (account, date)
       fix:  declare bill idempotent by account and date in Billing,
             or remove `survives restarts`
```

This is the same duplicate-charge bug class that Big example 1 prevents with `idempotent by request_id`, caught here at the point where it would actually bite.

### 41. Time and ordering are written down

```
to expire_sessions sessions using clock
    given sessions is an editable List of Session
    given clock is a Clock
    ensure for every s in sessions, expires_at of s is after now using clock
    uses clock

    remove every s from sessions where expires_at of s is not after now using clock
```

A clock is a capability, so any operation that depends on the time says so. Tests pass a fake clock, and the question "does this code depend on the wall clock?" has a one-line answer: look at `uses`.

## One program in three views

The same operation, `largest` from example 2, shown in each view. Only the display differs. The canonical text, its hash and the diff are the same in all three.

### Review view (canonical)

```
to largest items
    given items is a List of Int
    expect count of items is at least 1
    produces an Int
    ensure result is in items
    ensure for every x in items, x is at most result

    var best starts at item 0 of items
    for each i from 1 until count of items
        invariant best is in items
        invariant for every j from 0 through i - 1, item j of items is at most best
        if item i of items is more than best
            set best to item i of items
    give back best
```

### Compact view

```
to largest items
    given items is a List of Int
    expect count of items >= 1
    produces an Int
    ensure result in items
    ensure for every x in items, x <= result

    var best starts at item 0 of items
    for each i from 1 until count of items
        invariant best in items
        invariant for every j from 0 through i - 1, item j of items <= best
        if item i of items > best
            set best to item i of items
    give back best
```

Only comparison and logic words change into symbols. Statements, headers and contracts keep their words, so the structure stays readable.

### Localized view (French keywords, names untouched)

```
pour largest items
    donné items est une Liste de Int
    attendre nombre de items est au moins 1
    produit un Int
    garantir résultat est dans items
    garantir pour chaque x dans items, x est au plus résultat

    variable best commence à élément 0 de items
    pour chaque i de 1 jusqu'à nombre de items
        invariant best est dans items
        invariant pour chaque j de 0 à travers i - 1, élément j de items est au plus best
        si élément i de items est plus que best
            fixer best à élément i de items
    rendre best
```

The mapping is token for token, so a localized view can never change the structure of the program. Identifiers, strings and notes are left alone. Technical terms such as `Int` and `invariant` may stay in English by the choice of the language pack.

### Rules that keep views safe

| Rule | Why |
| --- | --- |
| A view is generated from the canonical text and never parsed on its own | Tools, docs and the compiler see one language |
| Canonical to view and back must give identical bytes, tested in CI for every view | A view that loses information cannot ship |
| Errors, docs, search and code review comments always quote canonical text, with the view's rendering beside it | Questions and answers stay searchable across languages |
| Views may be chosen per reader, never per file or per team | Two reviewers of one change see the same program in their own preferred form |
| A view cannot add or remove words, only substitute them one for one | Prevents drift into a second grammar |

### Why machines write the canonical form

The grammar is small, fixed and free of optional words, so it can be given directly to a model as a constrained decoder. An AI author cannot produce an unparseable or non-canonical program, and the formatter has nothing to fix. That removes the AppleScript problem, where nobody could be sure of the phrasing, for the writer who actually writes most of the code.

## Diagnostics

A fixed grammar only works if the tools say what the valid phrasing is whenever you miss it. Every message names the line, says what is wrong in plain words, quotes the rule or the earlier line that caused it, and shows the fix. The tone follows Elm and Gleam: no codes to look up first, and no blame.

### 42. A phrase the grammar does not have

```
ERROR  line 3: I do not know the phrase `add 1 to count`
       The form for changing a variable is:
           set count to count + 1
```

This is the AppleScript problem handled directly. A guess that fails gets the one correct spelling, not a vague syntax error.

### 43. Changing something fixed

```
ERROR  line 4: `rate` cannot be changed
       `rate` was bound with `let` on line 1, which makes it fixed.
       fix:   declare it with `var rate starts at 0.2`, or use a new name
```

### 44. A missing case

```
ERROR  area_of, line 8: not every Shape is handled
       missing: Triangle
       Triangle was added to Shape in Shapes 2.1
       fix:   add `when Triangle with base and height` to this `choose`
```

### 45. Operators that must not be mixed

```
ERROR  line 9: `and` and `or` are mixed without parentheses
           if a is 1 or b is 2 and c is 3
       write one of:
           if (a is 1 or b is 2) and c is 3
           if a is 1 or (b is 2 and c is 3)
```

### 46. A contract that cannot be proved, with the path that breaks it

```
UNPROVEN  place_order, line 14
    ensure if it fails with anything but RefundFailed, total_charged_to customer is total_charged_to customer at entry

    This does not hold on one path:
        1. charge succeeds                          customer is charged 40.00
        2. save fails with StoreUnavailable
        3. the operation fails with StoreUnavailable, and no refund was made

    counterexample:  total_charged_to customer was 0.00 at entry and 40.00 at exit
    fix:  refund receipt on this path, or weaken the ensure line.
          Weakening changes the manifest and will be flagged in review.
```

This is the message that turns the contract in Big example 1 from decoration into a safety net. It shows the failing sequence in the order it happens, not a solver's raw output.

### 47. A contract that proves nothing

```
WARNING  weak contract: find, line 6
    ensure result is some p or result is nothing
    This is true of every possible result, so it does not constrain behaviour.
    mutation check: replacing the body with `give back nothing` still satisfies every ensure line.
```

The checker looks for contracts that are true but empty, which is the main way a verified program can still be wrong.

### 48. A leaked resource

```
ERROR  first_line_of, line 9: `input` is a File that must be released and is not
       it leaks on this path: line 10 fails with FileError
       fix:   use `hold input as open path for reading using files` so it is released on every path
```

### 49. A unit mismatch

```
ERROR  line 12: cannot add a Length to an Area
           let x be width + area
       width is a Length (line 3). area is an Area (line 4).
       Did you mean `width * width`, which is an Area?
```

### 50. The prover ran out of budget

```
UNKNOWN  search, line 18
    invariant for every j from 0 until low, item j of sorted_items is less than target
    The prover used its fixed budget (20000 steps) before finishing.
    This happens identically on every machine and every run.
    options:  add a hint in the proof block, split the invariant into two,
              or lower this clause to tier tested
    The build continues with this clause at tier tested. The assurance report lists it.
```

Budgets are counted in steps, not seconds, and solver versions are pinned, so a proof that passes for one person passes for everyone. A proof that cannot be finished never silently becomes a trusted assumption.

## Contracts and impact: revisions from the change replay

The change replay (14 October 2026) tested impact analysis against 24 real Go changes that were reverted because they broke something, and 12 that were not. The report named what broke in 18 of 24. It caught all 17 where the contract already covered the behaviour that broke, and 1 of 7 where it did not. It also flagged 10 of 12 harmless changes. The revisions below follow from those results. Each cites the replay case that motivated it.

### Terms

| Term | Meaning | Enforced? |
| --- | --- | --- |
| **Contract** | What one operation requires and promises: its `given`, `produces`, `fails with`, `uses`, `expect`, `ensure` and `example` lines | Yes. Types, failures and capabilities by the compiler; `expect` and `ensure` by proof or run-time check, by tier |
| **Manifest** | The contract of a whole component: its operations' contracts, what it needs from other components, its authority, and its assumptions | Yes, as above, plus the wiring checks |
| **Informal clause** | A quoted sentence inside a contract | No. Shown in reviews and impact reports, never trusted |

The impact tool compares contracts and manifests between two versions. It does not read the code. So its answers are only as good as the contracts are complete, which is what most of the revisions address.

### R1. An unchanged contract no longer means "no impact"

*Case C10:* `io.Copy` changed behaviour in race-detector builds without any change to its contract, and the old rule ("implementation changed, contract unchanged: no impact") guaranteed silence.

New rule. When the code of an operation changes but its contract does not, the tool runs the operation's `example` lines and the property tests generated from its `ensure` lines against both versions:

| Result | Verdict |
| --- | --- |
| Both versions agree on every example and property | UNCHANGED, checked |
| They disagree | BREAKING, with the differing example as the witness |
| The operation has no examples and no checkable `ensure` lines | UNCHANGED, unchecked. Listed in the report for review |

The fast early cut-off is kept for operations whose contracts can be checked this way, which is most of them.

### R2. Interfaces carry their own contracts, including what implementations must not do

*Case C10:* the rule that broke ("a Writer must not keep the buffer after returning") belonged to every Writer, not to `io.Copy`, so no contract on `io.Copy` could state it.

An interface states a contract that every implementation must meet, and that callers may rely on:

```
interface Reader
    to read into buffer from reader
        given buffer is an editable Bytes
        given reader is an editable Self
        produces a Count
        fails with EndOfData, ReadFailed
        ensure result <= count of buffer
        ensure "a reader that returns 0 without failing will eventually return more data or fail"
```

Each implementation is checked against the interface contract. A change to the interface contract is an impact on every implementation and every caller. In Scribe, "must not keep the buffer" needs no clause at all: `buffer` is an `editable` parameter, and the memory model already forbids keeping it after return.

### R3. Clauses scoped to a platform or build mode

*Cases C05 (Windows), C15 (MIPS), C31 and C36 (Linux kernels and sandboxes), C10 (race-detector builds):* five of the breakages depended on a specific platform or build, and contracts had no way to say so.

An `on` block scopes clauses to a platform; an `in build` block scopes them to a build mode:

```
to current_user
    produces a User
    fails with UserLookupFailed
    on windows
        assumes "the current account maps to a security ID" assumed owner os_team
    on linux
        assumes "getpwuid_r succeeds for the effective user id" assumed owner os_team
    in build race
        ensure "buffers passed to Write are overwritten after the call returns"
```

The impact tool compares contracts one platform and build at a time, and names the platform in every verdict ("NEEDS-REVIEW on windows: new assumption"). A change that adds a platform-specific path without a matching `on` block is reported as unchecked for that platform.

### R4. Blocking and cost can be declared

*Cases C03 (sendfile could hang a process on exit) and C13 (a copy change caused a slowdown):* neither behaviour had anywhere to go in a contract.

```
to copy_file from source into target
    uses files
    may block "in the kernel for the whole transfer; signals wait until it finishes"
    cost at most linear in size of source
```

`may block` is part of the operation's effects, so adding it is an AUTHORITY-CHANGE, as a new capability is. `cost` clauses are checked at the `tested` tier by benchmarks recorded with the build; a change that breaks one is NEEDS-REVIEW with the measured figures.

### R5. Examples are part of the contract, and pin exact behaviour

*Cases C11, C16 and C22:* the breakages that were caught were caught because exact behaviour was written down: which hosts are accepted, which error type comes back, the exact redirect body. That was the most effective kind of clause in the replay.

`example` lines (see example 15) now count as contract clauses. A change that alters an example's result is BREAKING, with the example as the witness:

```
to redirect_body for location
    given location is Text
    produces Text
    example redirect_body for "/a" is "<a href=\"/a\">See Other</a>.\n\n"
```

### R6. A change can mark a clause as a correction

*Case C19:* a genuine bug fix was reported as BREAKING because other code "relied on" the buggy behaviour as the old contract stated it.

```
to write_profile into output
    fails with WriteFailed
    ensure "every write failure is reported, including the failure to finish compression"
        corrects "only the first write failure was reported" because "the old behaviour lost errors"
```

A `corrects` line turns BREAKING into NEEDS-REVIEW for that clause, with the stated reason shown to every affected dependent. It is removed from the contract on the next release.

### R7. Rule fixes that remove false alarms

| Mistake in the old rules | Case | New rule |
| --- | --- | --- |
| Removing an item counted as breaking even when every caller was changed in the same change | C29, C34 | A removal is breaking only for callers that the change leaves untouched |
| Taking a lock counted as new authority in a component that already took locks | C09 | Effects and capabilities are compared per component as well as per operation |
| Informal clauses describing how code works ("uses string concatenation") changed with the implementation and caused review flags | C24, C30 | Informal clauses must describe behaviour a caller can observe. A lint rejects clauses that name implementation details |

### R8. The toolchain helps write complete contracts and keep them complete

*The 17-of-17 versus 1-of-7 result.* Completeness decided almost every case, so the toolchain treats it as a measured property:

- `scribe draft` proposes a first contract for an operation from its code: failures, capabilities, effects, and examples recorded from tests. A person reviews and edits it.
- **Contract coverage.** For each operation, the tool reports which observable outputs no clause or example constrains (results, failure kinds, effects). The weak-contract warning in the diagnostics section (example 47) becomes a per-component coverage figure, shown in reviews.
- The build checks every contract against its code, so a contract cannot drift. Drift is a compile error, or an explicit downgrade to a lower tier.

### Updated impact verdicts

| Verdict | Meaning |
| --- | --- |
| COMPATIBLE | The new contract refines the old one |
| BREAKING | A caller can observe a difference the old contract ruled out; a witness is given |
| NEEDS-REVIEW | Cannot be decided, or a clause was marked `corrects` |
| AUTHORITY-CHANGE | A new capability or effect, including `may block` |
| UNCHANGED, checked | Contract unchanged and examples and properties agree across versions (R1) |
| UNCHANGED, unchecked | Contract unchanged, and nothing could be checked. Listed for review |

Every verdict names the platform and build it applies to (R3).

## Open questions, first experiments and changes to earlier drafts

The examples make a strong case on paper. Readers, not authors, decide whether it holds. The questions below are the rules most likely to change, and the experiments are ordered by how cheaply they can settle them.

### Open questions

| Question | Why it matters | Current default |
| --- | --- | --- |
| Underscore names, hyphenated names, or names with spaces | Spaces read best aloud but make the grammar ambiguous | Underscores, drawn at about 50 percent contrast in the review view |
| Calls with label words, as in `withdraw 50 from account` | Reads like a sentence, but arguments that are expressions can swallow label words | Allowed. Label words are reserved per operation |
| Arithmetic symbols in canonical form | Words are clearer for one operator and worse for three | Symbols for `+ - * /`. Words for everything else |
| `X of Y` for field access | Reads aloud but reverses the usual left-to-right order of dotted access | `of`, with chains limited to two |
| Indentation or braces | Indentation reads best, but is fragile when pasted or generated | Indentation, four spaces |
| Eight or more header lines before a body | Contracts first is the point, but too many lines may bury the signature | Lint on density, to be tuned by readers |
| Sentence-like contract forms such as `ensure if it succeeds, ...` | Reads well, but needs a precise grammar to stay unambiguous | A closed set of phrase forms |
| One-for-one localized keywords | Some languages need different word order | Token-for-token substitution only |
| `because` text on every public clause | Gives good diagnostics, but may rot or turn into filler | Required on public contracts, linted for emptiness |

### First experiments

Each of these can be run in a few weeks and can overturn a rule.

1. **Comprehension test.** Have about 20 programmers read the same ten programs in the review view, the compact view, and Python or Rust. Ask what each does and where the bug is. Seed bugs. Measure accuracy and time.
2. **Spec-diff-first review.** Show reviewers AI-written patches with contract weakenings hidden in them, with the code diff first for half and the spec diff first for the other half. Measure how many weakenings each group catches.
3. **Grammar stress test.** Write the grammar in EBNF, feed it to a parser generator, and run all 50 examples here through it. Count the places where the parser needs special rules to stay unambiguous.
4. **Constrained generation.** Ask models for 100 programs from specs, with and without the grammar as a decoding constraint. Measure parse success and how often the formatter has to change the output.
5. **Writing test.** Have humans write ten small functions in Scribe with grammar-driven completion and in Python. Measure time and errors. This tests the claim that writing is no longer the bottleneck.
6. **Round-trip property test.** Generate random programs, render each view, and convert back. Any difference removes that view from the design.
7. **Translation test.** Port 20 real functions from Python, Go and Rust. Count tokens and header lines, and list constructs the vocabulary cannot express.
8. **Incident replay.** Take five public postmortems. For each, decide which Scribe rule, if any, would have stopped it (an unhandled failure, a duplicate retry, a coupled failure domain, a silent authority change).

The first two decide whether the word-based style helps reviewers or only feels friendlier. If a symbol-heavy view beats it on accuracy, the default view should change.

### Experiment designs: 3 to 7

Each design states its question, method, measures and the pass criteria to fix before any results are seen. The thresholds are proposals to argue about now, not after the data arrives. Where an experiment fails, the design changes, not the threshold.

| Order | Experiment | Needs first | Rough effort |
| --- | --- | --- | --- |
| 1 | 7: Translation test | Nothing but the vocabulary in this document | Several people over 4 to 8 weeks |
| 2 | 3: Grammar stress test | Nothing; runs in parallel with 7 and uses its output | 1 person, 2 to 3 weeks |
| 3 | 6: Round-trip test | The grammar from 3 and a formatter | 1 person, 1 to 2 weeks |
| 4 | 4a: Constrained generation (form) | The grammar from 3 | 1 person, 2 weeks |
| 5 | 5: Writing test | A grammar-aware editor plugin | 2 people, 4 weeks including recruitment |
| 6 | 4b: Constrained generation (correctness) | A reference interpreter | 1 person, 2 weeks once the interpreter runs |

#### Experiment 3: Grammar stress test

**Question.** Can the whole surface language be parsed without ambiguity, without type information, and without special cases that a reader would also have to know?

**Method.**

1. Write the full grammar as EBNF, including the alias table from the notation section and every multi-word keyword (`give back`, `fails with`, `on failure`, `at entry`, `for every`, `there is`).
2. Feed it to two kinds of tool: a deterministic generator that reports conflicts (an LALR(1) or LL(k) generator such as Bison or Menhir), and a general parser that reports every parse of an ambiguous input (a GLR or Earley parser). A PEG tool alone is not enough, because PEG silently picks one reading and hides the ambiguity.
3. Parse every example in this document, every translation from experiment 7, and a stream of programs generated from the grammar.
4. For each conflict, record whether it was fixed by changing the grammar, by a documented special rule, or not at all.

**Known hotspots to test first.** Writing the examples already exposed these:

| Construct | The ambiguity | Example |
| --- | --- | --- |
| `is` | Means type (`given name is Text`), equality, variant test (`status is Paid`), membership (`is in`), negation (`is not`), and user predicates (`items is sorted`) | 2, 11, 16 |
| `in` | Membership, loop source (`for each x in items`), and call target (`charge ... in payments`) | 2, 29 |
| `of` | Field access (`count of items`) and type arguments (`List of Int`), plus its precedence against `-` | 2, 10 |
| `and` | Logic, and a separator between fields (`with width as ... and height as ...`) | 5 |
| Label words in calls | `withdraw amount from account`: an argument that is an expression may contain `from` | 8 |
| Commas | Lists, failure lists (`fails with A, B`), and the clause break in `for every x in items, x <= result` | 2, 6 |
| `where` | Comprehension filter, refinement, and database query | 12, 18, 24 |
| Aliases | `is not` versus `is not in`; `=` accepted as equality in a language where people expect assignment | Notation table |

**Measures.** Number of grammar conflicts; number of special rules and how each is explained; whether any rule needs the symbol table (it must not); share of the corpus that parses; parse speed on a large file.

**Pass criteria.**

- Zero unresolved conflicts and zero ambiguous parses in the corpus.
- No rule depends on name resolution or types.
- At most a handful of documented special rules, each of which can be stated in one sentence a reader would understand.
- 100 percent of the corpus parses.

**If it fails.** Each failing hotspot gets a redesign, usually by giving one of the meanings a different word. The hotspot list above is where the vocabulary is most likely to change.

#### Experiment 4: Constrained generation

**Question.** Can models write valid, canonical Scribe from a specification, and does constraining them to the grammar cost correctness?

**Method.** Run in two stages.

- **4a, form only (needs the grammar).** Take about 200 tasks drawn from experiment 7's corpus, each stated as a Scribe header with contracts. Ask several models, frontier and open-weight, to write the body under three conditions: (A) this document in the prompt; (B) the document plus ten worked examples; (C) condition B with grammar-constrained decoding, where the model can only produce tokens the grammar allows.
- **4b, correctness (needs a reference interpreter).** Run each body against its contract-derived property tests. Run the same tasks in Python and Rust as baselines, and in Dafny for the contract side.

**Measures.** Parse success rate; how much the formatter has to change the output; how often aliases such as `&&` appear before formatting; property-test pass rate; tokens and time per task; and, for condition C, any drop in correctness compared with B (the cost of constraining).

**Pass criteria.**

- Condition C parses 100 percent of the time; A and B above 90 percent.
- After formatting, every accepted program is canonical.
- In 4b, Scribe correctness is within 5 percentage points of the Python baseline, despite models having seen almost no Scribe in training.

**Risks to validity.** Models have no Scribe training data, so early results understate long-run performance. Report results per model, and repeat after fine-tuning on the experiment 7 corpus to show the trend.

#### Experiment 5: Writing test

**Question.** Can people write Scribe without guessing at phrasing, which is the AppleScript failure mode, and how much does grammar-aware completion help?

**Participants.** About 24 programmers in three bands: under 2 years, 2 to 10 years, and over 10 years of experience. Include non-native English speakers deliberately, since the language is word-heavy.

**Method.** A within-subjects design with the order of conditions counterbalanced. Each person completes 8 to 10 small tasks, each a function with a contract, in three conditions:

1. Scribe in a plain editor with this document open.
2. Scribe with grammar-driven completion that offers only valid next phrases.
3. Python, as the familiar baseline.

Add a fourth condition that matches how Scribe is meant to be used: fixing and extending code a model has written, rather than writing from a blank file.

Log every keystroke, every failed parse, and every lookup in the document. A "phrasing guess" is any attempt the parser rejects where the intended meaning is clear, such as `add 1 to count` before that display form existed.

**Measures.** Time per task; phrasing guesses per task and how they fall across the session (the learning curve); correctness of the contracts people write; a short workload questionnaire (NASA-TLX); and a closing interview.

**Pass criteria.**

- With completion, time per task is within 1.5 times Python by the last third of the session.
- Phrasing guesses fall close to zero by the last third of the session in the completion condition.
- In the editing condition, time is no worse than Python.

**Risks to validity.** Everyone knows Python and nobody knows Scribe, so absolute times favour Python. The learning curve matters more than the first task. If budget allows, add an unfamiliar-language control so that novelty and design can be told apart.

#### Experiment 6: Round-trip test

**Question.** Do the views and aliases ever change, lose or invent meaning?

**Method.** Property-based testing on a large scale.

1. Generate random syntax trees from the grammar, including deep nesting, long chains, every alias, comments, and strings that contain keywords and maths symbols.
2. For every view (review, compact and each localized pack), render the canonical text into the view and convert it back. The bytes and the parsed tree must both match.
3. For every alias, write the program using the alias, parse and format it, and check the result equals the canonical form and the content hash is unchanged.
4. Check that a symbol inside a string or comment, such as `≤` in a message, is never rewritten.
5. Check that copying text from a rendered view (dimmed underscores, drawn `≤`) pastes back as canonical text.
6. For each localized pack, check that the keyword mapping is one-to-one and that no translated keyword can collide with a common identifier, such as French `pour` used as a variable name.

Any failure is shrunk automatically to the smallest program that shows it.

**Pass criteria.** Zero failures across at least a million generated programs per view, plus the full corpus. A view or alias that fails even once is removed until it is fixed. This is the only experiment with no tolerance.

#### Experiment 7: Translation test

**Question.** Can Scribe express real, well-trusted code across every common way of expressing ideas, at what size cost, and where does it break?

**Sampling frame.** Sample by coverage, not by a fixed count. The frame is a grid of construct categories against source paradigms:

| Construct categories | Source paradigms and languages |
| --- | --- |
| Control flow and loops; recursion; data structures; error handling; resource handling; I/O; text processing and parsing; numerics; time and dates; state machines and protocols; polymorphism and generics; callbacks and events; concurrency and async; persistence and schemas; configuration; security checks | Systems (C, Rust, Zig, Go); object-oriented (Java, C#, Python classes); functional (Haskell, OCaml, Erlang/Elixir); scripting (Python, JavaScript, shell); verified (SPARK, Dafny, F\*/HACL\*, seL4) |

**Sources.** Code with a long record of trust: SQLite, Redis, PostgreSQL, the Linux kernel, the Python, Go and Rust standard libraries, Erlang/OTP, tokio, ripgrep, and classic algorithm collections. Verified code (seL4, CompCert, HACL\*, SPARK case studies) tests whether contracts translate.

**Deliberately adversarial cases.** At least a fifth of the sample is chosen because it is expected to fail: closures and stored callbacks, event loops and accept loops, macros, `unsafe` and pointer arithmetic, heavy generics and traits, dynamic dispatch, reflection, deeply mutable graphs, and code that leans on null.

**Method.**

1. Someone other than the language designers picks the functions, using the grid and a random draw within each cell.
2. Each function is translated by hand, or drafted by a model and then reviewed. The record says which.
3. About one function in five is translated independently by two people, to check that "gap" and "workaround" are judged consistently.
4. Every function gets a record: source and licence; category; original tokens and lines; Scribe tokens and lines, with header lines counted separately because contracts add information the original lacked; gaps (cannot be expressed); workarounds (expressible but clumsy, with a note); and any rule in this document it contradicts.

**Stopping rule.** At least 8 functions in every occupied cell of the grid, then keep adding until 20 functions in a row reveal no new kind of gap. A few hundred functions is the likely result, but saturation decides, not the count.

**Pass criteria.**

- At least 90 percent of functions translate without a gap; adversarial cases are reported separately.
- Median body size (header lines excluded) is no more than 1.3 times the Python original and no more than the Rust original.
- Every gap category is either covered by an existing construct, given a new one, or declared a deliberate non-goal with a stated alternative.

**Outputs.** The translated corpus, which also feeds experiments 3 and 4 and seeds model training; a gap catalogue ranked by frequency; a size report by category; and a list of rules to change.

**Bias controls.** The thresholds above are fixed before translation starts. Failures are published alongside successes. The designer of a construct does not judge whether that construct passed.

### Changes from the earlier drafts

Writing the examples forced some choices. These supersede the earlier spec template and the component section where they differ.

| Earlier draft | Now | Reason |
| --- | --- | --- |
| `requires` meant both a dependency and a precondition | `needs` for dependencies, `expect` for preconditions | One word, one meaning |
| `old(x)` | `x at entry` | Reads aloud |
| `ensures on ok(x)` and `ensures on error` | `ensure if it succeeds, ...` and `ensure if it fails, ...` | Plain words, same meaning |
| `add 1 to count` and `set count to count + 1` both appeared | Stored as `set`. The formatter may display it as `add 1 to count` | Readability is paramount, so the formatter picks the clearest form |
| `effects io, alloc` | `uses ledger, memory` naming the capabilities used | Effects and capabilities are one lattice |
| Capability ceiling written under `capabilities` | Written under `may` | Short and speakable |
| Symbols for all arithmetic and comparison in the canonical form | Symbols for arithmetic only. Comparison and logic are words | Matches the reading-first goal without making formulas worse |
| "Syntax philosophy" described only in prose | The rules, vocabulary table and 50 examples in this document | The examples are the specification until the grammar exists |
| Every operation ended with a uses nothing line when pure | No uses line means pure. The review view shows a pure tag | Forgetting an effect is already a compile error, so the line only restated the default |

### Suggested next step

Turn the vocabulary table into an EBNF grammar and run experiment 3 first. It costs the least, and a grammar that will not stay unambiguous would change many of the examples above.
