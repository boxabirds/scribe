# Scribe Language Specification

Oct 14, 2026 · @Julian Harris

**Draft 0.4.** This document is the whole specification as it stands. It replaces the syntax doc, the pilot results and the change replay as the place to read what Scribe is; those remain as evidence, summarised in Appendix B.

Every rule is normative unless it carries one of two marks:

- **Proposed**: recommended by the evidence but not yet agreed.
- **Open**: a real choice that has not been made. Section 14 lists every one, with what blocks it.

Code examples are shown in the **review view**, the default way people read Scribe (Section 2). Where the stored text differs, the difference is only the drawing of `≤`, `≥` and `≠`, which are stored as `<=`, `>=` and `!=`.

## 1. Purpose and principles

Scribe is a language for a world where most code is written by machines and read, reviewed and owned by people. It has three aims, in order:

1. **Rapid impact assessment.** Every operation and component states what it promises, needs and may do, in a form a tool can compare between two versions. The impact of a proposed change is computed from those statements before it is merged, in time that grows with the change and the components that depend on it, not with the size of the codebase.
2. **Human readability.** Every token a reviewer sees is understood at reading speed. When readability and impact assessment conflict, Scribe looks for a spelling that serves both; only when there is none does impact assessment win, and the cost to readability is recorded as a decision. Against every other aim, readability wins.
3. **Correctness by construction.** No undefined behaviour, no missing-value errors, no data races, no leaked resources, no unhandled failures. Beyond that, contracts are proven to the depth the code deserves.

And two constraints:

- **Fast, predictable compilation**, closer to Zig than to Rust.
- **No garbage collector** and no hidden allocation. Memory is managed at compile time: the compiler checks ownership from each parameter's convention (Section 7), and nothing is reclaimed at run time except where an owner's block ends.

Version 1 compiles to WebAssembly.

Principles that follow from these and decide individual rules:

| Principle | Consequence |
| --- | --- |
| What a reviewer must judge is visible before the body | Failures, capabilities, effects and contracts sit in each operation's header |
| One stored form, many readings | The source has exactly one canonical text; people see it through views, and the formatter chooses the clearest phrasing |
| Words where words are clearer, symbols where symbols are | School-level maths symbols for arithmetic and comparison; words for logic and structure |
| A promise is code, not commentary | Contracts are checked against the body on every build. Only quoted informal clauses are unchecked, and they are labelled as such |
| Never overclaim | Every proof, verdict and report states the assumptions and unchecked parts it rests on |
| Nothing implicit that matters | No ambient authority, no implicit conversion, no missing value without a `maybe`, no failure without a `fails with` |

## 2. Source text

### 2.1 One stored form, many readings

Three things are kept apart:

| Layer | What it is | Who chooses |
| --- | --- | --- |
| **Stored form** | The canonical text in the file. Plain ASCII, one spelling per construct. It is what is committed, hashed and diffed | The formatter, on every save |
| **Accepted forms** | Other spellings the parser takes: maths symbols, English phrases, common symbols from C-family languages. Each maps to exactly one construct | Fixed by the language version |
| **Views** | How a reader sees the text | Each reader |

A program's identity comes from its parsed tree, so a change of spelling never changes a hash, a contract comparison or an impact report.

**Display forms.** For some constructs the formatter shows the clearest of a small closed set of phrasings, chosen by rule, not by the author. Every display form maps back to one stored form, and authors may type either. The update forms are:

| Stored | Shown as |
| --- | --- |
| `set count to count + 1` | `increment count` |
| `set count to count - 1` | `decrement count` |
| `set total to total + price` | `add price to total` |
| `set balance of account to balance of account - amount` | `subtract amount from balance of account` |

`increment`, `decrement`, `add` and `subtract` are hard keywords (Section 2.5), so a line that starts with one is always an update, never a call: no operation can be named `add`.

An update form is used only when the place being set is also the left operand, and the place is a plain path: names, fields, and positions given by names or literals. Then the place is read once either way, and the short form means exactly what the long one does. `set balance to fee - balance` stays as written.

### 2.2 Views

| View | For | What changes |
| --- | --- | --- |
| **Review** (default) | Reviewers | `<=`, `>=`, `!=` drawn as `≤`, `≥`, `≠`; display forms applied; underscores in names drawn at about half contrast; proof hints folded; a `pure` tag on operations with no `uses` line |
| **Compact** | Experts | As review, plus logic words drawn as `∧ ∨ ¬ ∈ ∀ ∃ ⇒` |
| **Localised** | Readers of other languages | Keywords replaced one for one; names, strings and notes untouched |

A view must round-trip: rendering stored text into a view and converting it back gives identical bytes. A view that fails this does not ship. Errors, documentation and search always quote the stored text, with the reader's view alongside.

### 2.3 Notation

Symbols are used only where school mathematics has already taught them to most readers. Everything else is a word.

| Meaning | Stored | Also accepted | Review view |
| --- | --- | --- | --- |
| Arithmetic | `+ - * /`, `mod` |  | same |
| Less than, more than | `<`, `>` | `is less than`, `is more than` | `<`, `>` |
| At most, at least | `<=`, `>=` | `≤`, `≥`, `is at most`, `is at least` | `≤`, `≥` |
| Equal, not equal | `=`, `!=` | `==`, `≠`, `is not` | `=`, `≠` |
| And, or, not | `and`, `or`, `not` | `∧`, `&&`, `∨`, `\|\|`, `¬`, `!` | words |
| Member of | `is in` | `∈` | `is in` |
| For all, there is | `for every`, `there is` | `∀`, `∃` | words |
| Implies | `implies` | `⇒` | `implies` |

Rules:

- Comparisons chain as in mathematics: `0 ≤ low ≤ high ≤ count of items`.
- There is no assignment symbol, so `=` is always equality.
- `is` is reserved for tests that are not plain equality: a variant (`status is Paid`), a missing value (`result is nothing`), a predicate (`items is sorted`), membership (`x is in items`).
- Mixing `and` with `or` needs parentheses. There are six precedence levels.
- Symbols that exist only in programming (`!`, `&&`, `||`) are accepted on input, so machine authors are never rejected for spelling, and never displayed.

### 2.4 Names

- Names are single words joined by underscores: `total_charged_to`. The review view draws underscores at about half contrast, with a floor set by the theme for accessibility, so names read as separate words but remain one searchable token.
- No hyphens and no spaces inside names.
- Types and variants start with a capital letter (`Order`, `Paid`); everything else is lower case. **Open:** whether to drop capitals entirely and rely on declarations (Section 14).

### 2.5 Keywords

Two kinds:

- **Hard keywords** are reserved everywhere: words that start statements and header lines, comparison and logic words, argument labels, and the articles `a`, `an`, `the`.
- **Soft keywords** are reserved only inside their own phrase and are ordinary names elsewhere: `network`, `version`, `queue`, `value` and similar.

A reserved word cannot be a name. In particular `a`, `result`, `type`, `stop` and `add` cannot name variables or operations.

A reserved word is judged by what it costs readers, not authors. Scribe is written by machines and read by people, so reserving a word that makes a line unambiguous is cheap. The porting guide (`docs/porting-guide.md`) lists the reserved words that are common names in other languages, with what to use instead.

### 2.6 Layout and comments

- Blocks are opened by a header line and four spaces of indentation. Tabs are a syntax error. There are no braces, colons or semicolons.
- A line starting with `note` is a comment.
- Every statement is a whole line; no assignment inside an expression.
- Lists of parts are separated by commas. `and` is only logic.

## 3. Values and types

### 3.1 Built-in types

Integers (`Int`, `Nat`, and fixed widths), decimals, `Bool`, `Text`, `Bytes`, lists, and units of measure. There are no implicit conversions: a conversion is always a named operation.

**Open:** the exact numeric tower, fixed-width names, whether `Text` is bytes or characters, character and byte literals, and bit operations (Section 14).

### 3.2 Records

```
type OrderLine
    field product as a ProductId
    field quantity as a positive Int
    field unit_price as Money
```

A record may state invariants. They hold whenever control leaves an operation of the type:

```
type Order
    field lines as a List of OrderLine
    field status as an OrderStatus
    invariant lines is not empty or status is Draft because "only drafts may be empty"
```

A `model` line gives the abstract state that contracts talk about, so contracts never mention the representation:

```
type Queue of T
    field capacity as a Count
    field items as a Ring of T
    model contents as sequence_of items
    invariant count of contents ≤ capacity
```

### 3.3 Choices

```
choice OrderStatus
    Draft
    Paid with paid_at as an Instant
    Shipped with tracking as Text
    Cancelled with reason as Text
```

A choice is used through `choose`, which must handle every variant. Adding a variant makes every `choose` that does not handle it fail to compile.

### 3.4 Missing values

There is no null. A value that may be absent has type `maybe T`, with values `some x` and `nothing`. A `maybe` value must go through `choose` before the value inside it is used.

### 3.5 Refinements

A refinement carries its condition wherever the type goes:

```
type Percent is an Int with value 0 through 100
type Email is Text matching email_format
```

### 3.6 Units and time

A number followed directly by a unit is a quantity: `30 seconds`, `5 kilometres`, `1 GiB`. Units are checked: adding a length to an area is an error, and a length divided by a time is a speed. An `Instant` and a `CalendarDate` are different types; converting between them needs a time zone, written out:

```
let london be time_zone "Europe/London"
let closes be start_of due in london
```

### 3.7 Ranges

`from 1 through 10` includes 10; `from 1 until 10` stops at 9. There is no other range form.

## 4. Operations and statements

### 4.1 Operations

An operation is a header line, its contract, and its body:

```
to withdraw amount from account
    given amount is a positive Money
    given account is an editable Account
    produces a Receipt
    fails with InsufficientFunds
    expect account is open because "closed accounts cannot be debited"
    ensure if it succeeds, balance of account = balance of account at entry - amount
    ensure if it fails, balance of account = balance of account at entry
    changes only balance of account

    if amount > balance of account
        fail with InsufficientFunds
    subtract amount from balance of account
    return receipt_for amount
```

The header line names the operation, its first parameter, and further parameters introduced by label words: `to withdraw amount from account`. The label words are a fixed set of single words: `from`, `to`, `in`, `into`, `onto`, `over`, `for`, `with`, `by`, `as`, `using`, `of`. A call uses the same shape: `withdraw 50 from savings`.

**Proposed:** several unlabelled parameters, separated by commas: `to coerce first, second`. Real code needs this often (Appendix B).

The lines between the header and the body are the operation's **contract** (Section 10). An operation with no `uses` line uses no capability and is pure; the review view shows a `pure` tag.

### 4.2 Statements

Every statement starts with a keyword, or is a call.

| Statement | Form |
| --- | --- |
| Fixed binding | `let rate be 0.2` |
| Variable | `var count starts at 0` |
| Change a variable or field | `set price of item to 10`. Updates of a value from itself are shown as `increment count`, `add price to total` or `subtract amount from balance of account` (Display forms, Section 2) |
| Return | `return total` |
| Fail | `fail with OutOfRange` |
| Pass a failure up | `try parse_port text` (also inside expressions: `let n be try parse_int text`) |
| Handle failures | `handle` block, then `on failure Name` blocks (Section 6) |
| Branch | `if` / `otherwise if` / `otherwise` |
| Match | `choose value` with `when Variant with field` blocks |
| Count | `for each i from 0 until count of items` |
| Iterate | `for each order in orders` |
| Loop | `repeat while condition`, with `invariant` and `decreases` lines |
| Leave or continue a loop | `stop`, `skip` |
| Hold a resource | `hold input as open path for reading using files` (Section 7) |
| Concurrency | `together`, `pipeline` (Section 9) |

Only a name or an `of`-path can be set. **Proposed:** setting one element of a list, `set item i of heap to x`.

**Proposed:** a statement that does nothing, for a `when` branch with nothing to do.

### 4.3 Functions as values

Scribe has no anonymous functions and no closures that outlive the call they are created in. A block passed to a call (`together`, `handle`, `hold`) runs before the call returns. Where other languages store a callback, Scribe uses a named type holding the data, a protocol, or a declared callback in a manifest.

**Proposed:** operation types, so an operation can be passed as a parameter, with its effects and failures carried in the type: `given key is an operation from Order to Money`, and `uses what key uses`. This is the most common gap found in real code (Appendix B).

## 5. Expressions

### 5.1 Calls and `of`

A call is the operation's name, its first argument, then label words with further arguments: `charge customer for total of cart in payments`.

- `of` is an argument label like the others. It may appear at most once in a call, and first among the labels. Reading a field and calling an operation look the same: `count of items`, `balance of account`, `item i of items`.
- The argument after `of` may itself be an `of`-path or a call with plain arguments: `count of contents of queue`, `sum of total_of order ...`.
- Every other label belongs to the outermost call on the line. To pass a labelled call as an argument, put it in parentheses.
- A label is one word. `with key request_id` is a call to `key`, not a two-word label.

### 5.2 Precedence

From loosest to tightest: `implies`; a chain of `and` or a chain of `or` (never mixed); `not`; comparisons (which chain); `followed by` (joining sequences); `+` and `-`; `*`, `/` and `mod`; calls, `of` and `at entry`.

### 5.3 Constructing records

```
let order be an Order from cart, receipt with status Paid
return a Dashboard from profile, orders, alerts
```

Parts are separated by commas. **Proposed:** naming fields during construction, `a Point with x as 1, y as 2`.

### 5.4 Collections

One comprehension form, used in code and in contracts:

```
let paid be sum of total_of order for each order in orders where status of order is Paid
```

A comprehension may only follow `of`, or sit in parentheses. There is no `map`, `filter` or `reduce`.

### 5.5 Quantifiers and conditions

```
for every x in items, x ≤ result
for every j from 0 until low, item j of items < target
there is an item in items where price of item > limit
```

Two rules settle scope:

- `where` takes the rest of its line.
- A quantifier (`for every x, ...`) takes the rest of its line.

### 5.6 Strings

Text in double quotes, with `{expression}` interpolation: `"Your order {id of order} is confirmed."`.

### 5.7 What `is` means

`x is y` is parsed the same way in every case, but what it means depends on what `y` is: a variant (`status is Paid`), `nothing`, a predicate defined with `define` (`items is sorted`), or a pattern that binds a name (`result is some p`). This is the one place where meaning needs name resolution, which happens after parsing.

## 6. Failures

An operation lists every way it can fail. The compiler checks the list is complete: a body cannot fail in an undeclared way, and an operation that handles all its callees' failures has no `fails with` line.

```
to parse_port text
    given text is Text
    produces a Port
    fails with NotANumber, OutOfRange

    let number be try parse_int text
    if number < 1 or number > 65535
        fail with OutOfRange
    return number

to port_or_default text
    given text is Text
    produces a Port

    handle
        return try parse_port text
    on failure NotANumber or OutOfRange
        return 8080
```

Rules:

- `try` passes a failure up to the caller, or to the enclosing `handle` block.
- Each `on failure` names the failures it handles and must do something: return a value, fail, or call something. There is no catch-all that swallows failures.
- `on failure A or B as problem` binds the failure so it can be passed on: `fail with problem`.
- Adding a way to fail is a change to the contract (Section 12).

**Proposed:** failures as values with fields, declared as a `choice`, so a failure can carry data and two failures with the same name can be told apart: `fail with OutOfRange with value clamped`. This was the most common gap in translated real code.

**Open:** whether panics exist at all. The current rule is that a `checked` operation cannot panic, and the compiler proves it; what an unprovable bounds check does at lower tiers is undecided.

## 7. Memory, ownership and resources

### 7.1 Values, not references

A variable holds its value, not a pointer to it. There are no references to write and no lifetime annotations. Safety comes from how each parameter may be used, stated in the contract:

| Convention | Written | The operation may | The caller afterwards |
| --- | --- | --- | --- |
| Read (default) | `given items is a List of Int` | Look, not change | Still owns it, unchanged |
| Editable | `given account is an editable Account` | Change it; nobody else can see it meanwhile | Still owns it, possibly changed |
| Consumed | `given connection is a consumed Connection` | Take ownership | Cannot use it again |

A read or editable parameter cannot be kept after the operation returns. This is a type rule, not a convention: a whole class of bugs (keeping a buffer someone else will reuse) cannot be written.

### 7.2 Allocation

There is no garbage collector and no hidden allocation. Memory is released when its owner's block ends.

**Proposed** (replacing explicit allocator parameters on every call):

- An operation that may request memory whose size depends on its input says so in its contract: `allocates`. Fixed-size values, locals and in-place edits never need it, and neither does stack use.
- Call sites do not repeat it; the allocator comes from the caller's context.
- A block chooses the allocator for everything inside it, and frees it all at the end: `within an arena`.
- Bounded collections need no allocator: `a List of Int up to 100`.
- Running out of memory is an ordinary failure, `OutOfMemory`.

Recursive operations get a stack-depth line in reports ("stack depth unbounded") unless a `decreases` measure and a size bound limit it.

### 7.3 Resources

Files, sockets and locks are linear: they must be released exactly once. `hold` releases on every exit from its block, including failures:

```
hold input as open path for reading using files
    return try read_line from input
```

A linear value bound with `let` and not released on some path is a compile error naming that path.

State can be tracked in the type, so misuse is a compile error rather than a run-time check:

```
type Connection in state Closed or Open

to send data over connection
    given connection is an editable Connection in state Open

to close connection
    given connection is a consumed Connection in state Open
    produces a Connection in state Closed
```

### 7.4 Shared state

**Open:** how to write data that several owners share or that forms cycles, such as graphs and doubly linked lists. The candidates are explicit counted references and handles into an arena; either way the readers and writers are declared in contracts. Real code needs this (Appendix B), so the decision cannot be deferred long.

## 8. Effects, capabilities and authority

### 8.1 No ambient authority

Nothing can open a file, a socket or a clock unless it has been handed a **capability** for it. All authority starts in one place, the program's entry point, and can only be narrowed as it is passed on:

```
to main root
    given root is the Authority
    uses root

    let reports be root attenuated to files under "/reports" read only
    let partner be root attenuated to network host "export.partner.example" port 443
    run ReportExporter with [reports, partner]
```

Because authority only flows along capabilities, "what can this component reach?" has a static answer, with the path as evidence:

```
$ scribe query reach ReportExporter network
ReportExporter can reach: host "export.partner.example" port 443 only
    path: main > root attenuated > partner > ReportExporter
```

### 8.2 Effects

An operation's effects are what it may do beyond computing its result. They are declared in its contract and checked against the body:

| Effect | Declared as |
| --- | --- |
| Use a capability (files, network, clock, database,    N  another component) | `uses files, clock` |
| Request memory (**Proposed**) | `allocates` |
| Block in the operating system | `may block "<when and what for>"` |
| Turn secret data public | `declassifies secret to public because "..."` |

An operation with none of these is pure, and calling an effectful operation from it is a compile error naming the call. Clocks are capabilities, so code that depends on the time says so, and tests pass a fake clock.

### 8.3 Information flow

Values may carry labels: `given name is Text labelled untrusted`, `given card is a CardNumber labelled secret`. Labelled data cannot reach a place that forbids it. Untrusted text cannot be joined into a database query; it must first pass a check that produces a typed value (`check name as UserName`). Secret data becomes public only through a declared `declassifies`, which appears in every review and impact report.

## 9. Concurrency

Concurrent structure is written as a visible shape, and no task outlives the block that started it.

### 9.1 Doing several things at once

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
        let orders be fetch orders of user from services
        let alerts be fetch alerts of user from services, or else none
    return a Dashboard from profile, orders, alerts
```

Everything inside `together` starts at once, and the block ends when every line has finished. If a required line fails, the others are cancelled and the failure goes up; `or else` marks a line whose failure is acceptable. Cancellation unwinds through `hold`, so resources are still released. A `deadline` is monitored at run time, not proven.

### 9.2 No shared mutation

Two branches of a `together` block cannot change the same variable; the compiler names both lines. Each branch produces its own result instead. With no shared mutable values there is nothing for a lock to protect, and no data race can be written.

### 9.3 Pipelines

```
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

Every queue is bounded, so a full queue makes the stage before it wait.

### 9.4 Work that survives a restart

```
workflow nightly_billing for date
    given date is a CalendarDate
    runs at most once per date
    survives restarts
    uses billing

    for each account in list_billable_accounts for date in billing
        remember progress by account
        try bill account for date in billing
```

A workflow resumes after the last completed step. The compiler rejects it unless every effect in a repeatable step is declared `idempotent by` the keys that identify the step.

### 9.5 Waiting without `async`

**Proposed:** there is no `async` keyword. An operation that can wait takes a capability for it, which shows in its `uses` line; callers write an ordinary call. The capability decides whether waiting blocks a thread or yields to an event loop, so the same code runs either way, and every place a task may suspend is visible from the contracts.

**Open:** a channel type for passing results between tasks, and how event-driven servers with an accept loop are written.

## 10. Contracts

A **contract** is what an operation requires and promises. It is the lines between the operation's header line and its body, and it is code: the compiler holds the body to it on every build. A contract that no longer matches its code is a compile error, so contracts cannot drift.

### 10.1 Clauses

| Clause | Says | Checked by |
| --- | --- | --- |
| `given x is a T` | A parameter, its type and convention | Types |
| `produces a T` | The result | Types |
| `fails with A, B` | Every way it can fail | Compiler, completely |
| `uses`, `allocates`, `may block` | Its effects and capabilities | Compiler, completely |
| `expect condition` | What callers must ensure | Proof at call sites, or run-time check |
| `ensure condition` | What it guarantees. Forms: `ensure if it succeeds, ...`, `ensure if it fails, ...`, `ensure if result is some p, ...` | Proof, or run-time check, by tier |
| `changes only x` | Everything else is untouched | Compiler |
| `example call is value` | An exact result for an exact input | Run on every build |
| `cost at most ...` | A cost bound | Benchmarks, at the `tested` tier |
| `idempotent by keys` | Repeating it with the same keys has no further effect | Tests; proof at higher tiers |
| `assumes "..." assumed owner team` | A fact about the outside world it relies on | Not proven. Listed in every report that depends on it; may carry a run-time monitor |
| `"a quoted sentence"` in place of a condition | An informal clause | Not checked. Labelled informal everywhere it appears |

Any clause may end with `because "..."`. The sentence becomes the diagnostic text, the documentation and the label in reports. Inside an `ensure`, `x at entry` is the value of `x` when the operation began.

Informal clauses must describe behaviour a caller can observe. A lint rejects informal clauses that describe how the code works ("uses string concatenation"), because they change with every refactoring.

### 10.2 Examples

Examples are part of the contract, not separate tests. They are the cheapest way to pin exact behaviour, and the change replay found exact behaviour was what caught most real breakages.

```
to redirect_body for location
    given location is Text
    produces Text
    example redirect_body for "/a" is "<a href=\"/a\">See Other</a>.\n\n"
```

Properties generalise examples and run as generated tests: `property for every items that is a non_empty List of Int, largest of items is in items`.

### 10.3 Host-dependent behaviour and build scoping

Source never names an operating system or a host. Everything that can behave differently from one host to another reaches code through a capability (Section 8.1), so a difference between hosts is a fact about a capability, and it is stated where the capability is declared.

**Proposed:** a capability's interface lists the facts that vary by host. Each is a predicate a contract can test:

```
interface Files
    varies by host
        names_ignore_case because "two names that differ only in case name the same file"
        rename_replaces_target because "renaming onto an existing file replaces it"

    to rename source to target in files
        given source is a Path
        given target is a Path
        given files is an editable Self
        fails with NotFound, TargetExists
        ensure if rename_replaces_target of files, "an existing target is replaced"
```

An operation whose contract tests a host-dependent fact holds wherever the fact has the value it tests for; a contract that tests none must hold on every host. Each host binding, the implementation of a capability's interface for one host, gives every fact its value and carries the assumptions it makes about that host. The toolchain supplies the host bindings, and they are the only place a particular operating system is named.

A clause can also hold in one build mode only:

```
to write data into stream
    given data is Bytes
    given stream is an editable Stream
    in build race
        ensure "buffers passed to write are overwritten after the call returns"
```

Contracts are compared once for each host binding and build mode (Section 12).

### 10.4 Loops, types and predicates

- Loops carry `invariant` lines, and `repeat while` loops a `decreases` line naming a quantity that shrinks on every pass and cannot go below zero. A `for each` over a range always terminates.
- Types carry `invariant` and `model` lines (Section 3.2).
- `define` names a predicate so contracts read as sentences: `define items is sorted` with a `holds when` line; then `expect items is sorted`.

### 10.5 Tiers

Each clause has an assurance tier, and every report states the weakest tier it relies on:

| Tier | Meaning |
| --- | --- |
| `informal` | Quoted text. Shown, never trusted |
| `tested` | Checked by examples, properties and benchmarks |
| `checked` | The default. No undefined behaviour, no unhandled failure, no run-time error is proven; types, effects, capabilities, units and states are checked |
| `proven` | Every `ensure` and invariant is proven |

Proof hints go in a `proof` block at the end of the operation, folded by default in the review view. Escape hatches, `trust "..." owner team` and `assume`, name a reason and an owner, are counted per package, and appear in every review and report that depends on them.

### 10.6 Corrections

When a change fixes a bug that callers may have relied on, the new clause can say so:

```
ensure "every write failure is reported, including the failure to finish compression"
    corrects "only the first write failure was reported" because "the old behaviour lost errors"
```

A `corrects` line makes the change a review item rather than a breaking change (Section 12). It is removed at the next release.

### 10.7 Writing complete contracts

Impact analysis is only as good as contracts are complete (Appendix B), so the toolchain treats completeness as a measured property:

- `scribe draft` proposes a first contract from the code: failures, effects, capabilities, and examples recorded from tests. A person reviews and edits it.
- **Contract coverage** reports, for each operation, the observable outputs that no clause or example constrains: results, failure kinds and effects. It is shown in reviews as a figure per component.
- An operation that provides results but has no `ensure` or `example` gets a weak-contract warning.

## 11. Components, interfaces, manifests and systems

### 11.1 Components and manifests

A **component** is the unit of review, versioning and deployment. Its **manifest** is its contract as a whole, written at the top of the component's source:

```
component Checkout version 3.0.0

    about
        Turns a cart into a paid order. Charges a customer at most once per request.

    needs
        Payments from 3.1 as payments
            uses charge, refund, total_charged_to
        Inventory from 2.0 as inventory
            uses reserve, release

    assumes
        "charging with the same key twice charges once"    assumed owner payments_team monitor checking

    provides
        to place_order cart for customer with request_id
            given cart is a Cart
            given customer is a CustomerId
            given request_id is a RequestId
            produces an Order
            fails with EmptyCart, OutOfStock, PaymentDeclined, RefundFailed
            expect cart is not empty
            ensure if it succeeds, status of result is Paid
            idempotent by request_id
            uses payments, inventory

    failure
        domain checkout
```

| Section | Says |
| --- | --- |
| `about` | What the component is for, in prose |
| `needs` | Each component it depends on, from which version, and which of its operations it uses. Impact is judged against exactly these |
| `may` | Authority it holds of its own (usually none: it acts through what it needs) |
| `assumes` | Facts about the world outside, each with an owner and a monitor |
| `provides` | The operations other components may call, with their contracts |
| `failure` | Its failure domain and retry rules |

Versions are computed from the change to the manifest (Section 12): a person may raise a version but not lower it.

### 11.2 Interfaces

An interface states a contract that every implementation must meet and every caller may rely on:

```
interface Reader
    to read into buffer from reader
        given buffer is an editable Bytes
        given reader is an editable Self
        produces a Count
        fails with EndOfData, ReadFailed
        ensure result ≤ count of buffer
        ensure "a reader that returns 0 without failing will later return data or fail"
```

Each implementation is checked against the interface's contract. A change to an interface's contract affects every implementation and every caller.

### 11.3 Schemas and migrations

Stored and transmitted data has a schema with a declared compatibility mode:

```
schema Order version 4 compatibility backward_transitive
    id           as an OrderId
    lines        as a List of OrderLine
    note         as a maybe Text     added in version 4
    discount     reserved            removed in version 3

migration Order from version 3 to version 4 turning old into new
    ensure note of new is nothing
```

A removed field's name is reserved and cannot be reused. A migration is checked against its `ensure` lines over every value of the old version.

### 11.4 Protocols

A conversation between two parties is declared once, and both sides are checked against it:

```
protocol Upload
    client sends Begin with size as Bytes at most 1 GiB
    repeat
        client sends Chunk with data as Bytes
        server sends Ack with offset as Bytes
    until client sends Commit
    server sends one of Done with checksum as a Hash, or Failed with reason as Text
```

A new version of a protocol is accepted only if it refines the old one.

### 11.5 Systems

A system file lists the components, gives each its authority, wires them together, and states rules about the whole:

```
system Shop version 12
    components
        Storefront 5.1
        Checkout 2.4
        Payments 3.1
    authority
        root gives Payments network host "payments.example.com" port 443
        nothing else holds authority
    wiring
        Storefront needs Checkout
        Checkout needs Payments
    failure domains
        checkout contains Checkout, Payments
        comms contains Notifications
        a failure in comms must not stop checkout
    rules
        only Payments can reach host "payments.example.com"
        no value labelled card_data leaves Payments
```

The checker proves the rules on every build and reports any rule a change would break, naming the dependency path.

**Proposed:** replace the fixed English rule templates with a small rule language of reach queries and flow queries, because every new template adds keywords (Appendix A).

## 12. Impact analysis

Impact analysis answers one question before a change is merged: **what could this change break, for whom, and how sure are we?** It works from contracts, not from the code's text, so it can answer for callers it has never seen.

### 12.1 What is compared

`scribe impact` compares two versions of a component: its manifest, every contract in it, its effects and authority, its schemas and protocols, and whether its implementation still meets its contracts. It then checks the result against:

1. **every known dependent**, using the clauses that dependent's `needs` entry relies on (written or inferred from its code);
2. **any possible caller**, using the public contract alone, because most users of a library are outside its repository;
3. **every system rule** in every system file that includes the component.

Where a contract tests a host-dependent fact or has clauses scoped `in build <mode>` (Section 10.3), the comparison is made separately for each host binding and build mode, and a verdict names the bindings it applies to. A guarantee that weakens only where file names ignore case is reported as breaking there, not hidden in an average.

### 12.2 Verdicts

Each provided operation, each dependent and each system rule gets exactly one verdict:

| Verdict | Meaning | Must carry |
| --- | --- | --- |
| Compatible | The new contract refines the old: preconditions no stronger, guarantees no weaker, no new failure, effect or assumption | A proof, or the solver's reason |
| Breaking | Some caller can observe a difference the old contract promised would not happen | A witness: a concrete input or scenario |
| Needs review | The tool cannot decide: an informal clause or an assumption changed, or a guarantee changed in a way that may or may not matter | The clause and the reason |
| Authority change | The component now holds or uses authority it did not before | The new capability and the path to it |
| Unchanged, checked | The contract did not change and the new implementation was checked against it at its stated tier, and the contract covers the code that changed | The tier and the coverage |
| Unchanged, unchecked | The contract did not change, but the changed code does something the contract does not speak to, or was not checked | The uncovered lines or behaviours |

"Unchanged, unchecked" is the tool being honest about its blind spot. A green result is only ever as good as the contract behind it, so the report never says a change has no impact when the contract does not cover what changed.

### 12.3 Rules

| Change to a contract | Verdict |
| --- | --- |
| Weaker precondition; stronger guarantee; new operation | Compatible |
| Stronger precondition | Breaking for any caller that may not meet it, with that caller's input as witness |
| Weaker or removed guarantee | Breaking for dependents that rely on it; needs review for unknown callers |
| New way to fail | Breaking for callers that handle every failure by name; needs review otherwise |
| New or changed `assumes` | Needs review, addressed to the assumption's owner |
| Changed informal clause | Needs review |
| New capability or effect for the component | Authority change |
| Removed or renamed operation | Breaking for every caller the same change leaves untouched; compatible for callers the change updates |
| Changed `cost` | Needs review if it grows in the order of growth; compatible if it shrinks |
| Changed schema | As its compatibility mode says; a change that does not meet the mode is breaking |
| Changed protocol | Compatible if the new protocol refines the old; breaking otherwise |

Three points of detail:

- **Effects are compared per component.** Moving an existing effect from one operation to another inside the same component is not an authority change. It is reported against the operations concerned under the rules above.
- **A correction is not a breakage of the false promise.** A change may declare that an old clause was never true:

```
    corrects ensure "names are returned in sorted order"
        because "the order was never sorted on Plan 9; see issue 4417"
```

The report shows corrected clauses separately. Dependents that relied on the clause are still listed with needs review, because their code may have depended on the false promise in practice.

- **Informal clauses must describe behaviour, not implementation.** A lint rejects informal clauses that name internal variables, algorithms or data structures, because a clause like `"uses a hash map"` changes with every refactor and turns impact reports into noise.

### 12.4 Contract coverage

The tool knows which lines of an implementation each contract clause constrains: by proof, by the checker's dependency tracking, or by the tests generated from `example` and `property` lines. Code that no clause constrains is **uncovered**. A change that touches only covered code can be reported as "Unchanged, checked"; a change that touches uncovered code is reported as "Unchanged, unchecked" with the uncovered behaviour named, for example a new retry loop, a changed timeout, or a new order of side effects.

`scribe coverage` reports coverage for a component, and a project may set a floor below which merges are refused.

### 12.5 The report

The report leads with the most severe verdict and gives every verdict its evidence. It is written in sentence case:

```
scribe impact Payments 3.1.0 → 3.2.0

Breaking (1)
    charge: new way to fail, RateLimited
        Checkout.place_order handles every failure of charge by name.
        witness: 101 charges in one second from one merchant

Needs review (1)
    assumes "the card network answers within 30 seconds"
        changed to 10 seconds; owner payments_team

Authority change (0)

Unchanged, checked (6)
    refund, total_charged_to, capture, void, list_charges, receipt   tier checked, coverage 100%

Unchanged, unchecked (1)
    authorise: changed lines 88–104 add a retry loop that no clause covers

Suggested version: 4.0.0 (a breaking change)
```

### 12.6 Deployment

`scribe deploy plan` applies the same comparison to a running system: it compares the system file and every component version in production with the proposed one, and orders the rollout so that no component is ever live alongside a version of a dependency it is breaking against. Where no safe order exists, it says so and names the cycle.

## 13. Toolchain and diagnostics

### 13.1 Commands

| Command | Does |
| --- | --- |
| `scribe check` | Parses, formats to the stored form, type-checks, checks ownership, effects, authority and every contract at its stated tier. This is the build's gate |
| `scribe build` | Check, then compile to WebAssembly (version 1). `in build` modes select debug or release |
| `scribe test` | Runs `example` and `property` lines and test blocks |
| `scribe prove` | Discharges `proven` clauses with the solver and reports any it cannot, with a counterexample where one exists |
| `scribe impact` | Compares two versions (Section 12) |
| `scribe coverage` | Reports which code each contract clause constrains (12.4) |
| `scribe draft` | Proposes a contract for an operation from its body and tests. The proposal is marked as a draft until a person accepts it, and never counts as checked on its own |
| `scribe query reach` | Answers "what can reach this capability, value or component, and by which path?" |
| `scribe deploy plan` | Orders a rollout (12.6) |
| `scribe review` | Shows a change in the review view: contract changes first, then the impact report, then the code |
| `scribe view` | Renders source in a chosen view (review, compact, localized) and converts it back |

The compiler is one program with no build scripts that run arbitrary code. A component compiles from its source and its dependencies' manifests alone, so components compile in parallel and a change to an implementation never recompiles its callers.

### 13.2 Diagnostics

Every diagnostic:

- says what is wrong in one sentence of plain English, in sentence case;
- quotes the stored text and points at it;
- names the rule that was broken, with a link to this specification;
- gives a fix when there is exactly one sensible fix, and none when there are several.

```
withdraw.scribe:14: this changes deposits of account, which the contract does not allow.
    14 │     add amount to deposits of account
       │     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    The contract says: changes only balance of account
    Either add deposits of account to the changes only line, or remove this change.
    Rule: 10.3 changes only
```

Where the parser accepted something by a special rule (D1, D2, label attachment), `scribe check --explain` shows the tree it built, so no reading is ever silent.

## 14. Open decisions

These are the decisions this draft does not settle. Each names what blocks it. Anything marked Proposed in the sections above is listed here.

| # | Decision | Status | Blocked by |
| --- | --- | --- | --- |
| 1 | Shared mutable state between tasks: forbid entirely, or allow through a single checked construct | Open | A worked corpus of real concurrent servers to test against (Section 9) |
| 2 | No `async` keyword: blocking is declared with `may block` and the runtime schedules | Proposed | A prototype scheduler and a latency measurement |
| 3 | `allocates` as a contract clause, with the allocator passed as context | Proposed | Measuring how many contracts it adds lines to |
| 4 | A small rule language for system files in place of fixed English templates | Proposed | Keyword count (Appendix A) |
| 5 | Possessive paths: accept `account's balance` as a form of `balance of account` | Proposed | A grammar run. `'s` would bind tighter than any call, so `total of cart's lines` means `total of (lines of cart)`. Bare `account balance` is not possible: it already reads as a call of `account`, and it would look almost the same as the name `account_balance` in the review view |
| 6 | Reduce the keyword set (now 102 hard, 80 soft) | Open | Experiment 1, the comprehension test, extended to compare candidate words for the same idea: for example `hold` against `with`, `using` and `lease` |
| 7 | Parser strategy: the grammar is not LALR(1) | Open | Deciding between a hand-written parser that follows the grammar, and changing the grammar until a generator accepts it |
| 8 | Code generator for WebAssembly, the version 1 target: a back end of our own, or one built on LLVM or Cranelift | Open | A compile-time benchmark on the pilot corpus |
| 9 | Solver and proof format for `proven` clauses | Open | Choosing a solver that is fast and gives stable results across versions |
| 10 | Localized views beyond English | Open | Round-trip proof for at least one other language |
| 11 | How host-dependent facts are declared (10.3), and which host bindings impact analysis compares by default | Proposed | Re-expressing the replay's platform-specific breakages with it |

## Appendix A: grammar notes

The reference grammar is `grammar/scribe.lark` (version 0.3) in this project; `scribe-grammar-v0.2.lark` is the version the pilot measured. It is an Earley grammar: it states the language precisely but is not yet a production parser. `tools/check_grammar.py` tests it against a file per construct in this specification, a file per forbidden form, this specification's code blocks and the pilot corpora.

**Vocabulary.** 182 reserved words: 102 hard keywords, which can never be names, and 80 soft keywords, which are names everywhere except in the position that gives them meaning. The articles `a`, `an` and `the` are hard. Reducing this count is open decision 6.

**Special rules.** Twelve rules resolve what the grammar alone leaves ambiguous. The four that most affect readers:

| Rule | Says |
| --- | --- |
| D1 | `where` takes the rest of its line |
| D2 | A quantifier (`for every`, `there is`) takes the rest of its line |
| Labels | A label word attaches to the outermost call on its line; `of` may appear at most once in a call and only first; labels are single words |
| Mixing | `and` and `or` in one expression need parentheses; a comprehension may appear only after `of` or in parentheses |

Constructions list their parts with commas, never `and`, so `and` is always logical.

**Parser class.** Converted to Bison, the grammar has 183 shift/reduce and 20 reduce/reduce conflicts (version 0.2: 172 and 20), so it is not LALR(1). One ambiguity is real, not an artefact of the conversion. Open decision 7 settles the parser strategy; the requirement whichever way it goes is that every accepted text has exactly one tree, and that `scribe check --explain` shows it.

## Appendix B: evidence and history

This appendix records how the specification got here. Nothing in it is normative: where it and the sections above differ, the sections above win.

### B.1 Pilot: grammar stress test and translation test

Full report: [Scribe Pilot Results: Experiments 3 and 7](https://claude.ai/code/artifact/5d47f896-3b48-4a23-94d6-3729779bd63b).

- 33 of the 42 parseable examples in the original design document parsed as written; all 42 after 15 repairs. Five were silent misreadings: the parser accepted them with a meaning the author did not intend. Rules D1, D2 and the label rules (Appendix A) were written to remove them.
- 48 functions from Python, Go and Rust were translated by two independent translators. 3 went across with no gap in the vocabulary. Bodies came out at 0.97 times the tokens of the originals; the median header was 12 lines.
- A translation agent altered the grammar and tools during the run. The changes were reverted, quarantined and checksummed, and everything was re-run.

### B.2 Change replay: does impact analysis catch real breakages?

Full report: [Scribe Change Replay](https://claude.ai/code/artifact/4e58ffe9-7efb-4155-aad4-d843d2cddf86).

24 changes to the Go standard library that were later reverted, and 12 that were not. One engineer wrote contracts of the code before each change, without seeing it; a second applied the impact rules to the change; two judges scored the result against what actually happened.

- The tool flagged the actual cause in 18 of 24 breakers (14 breaking, 4 needs review), partly in 4, and missed 2.
- Where the before contract covered the behaviour that broke, it caught 17 of 17. Where it did not, 1 of 7. The tool is exactly as good as its contracts.
- Of the 12 controls, 2 were clean, 5 needs review and 5 claimed breaking. Applying the rule fixes below gives 4, 5 and 3.
- Judges agreed on 35 of 36 cases.

What the replay changed in this specification:

| Finding | Change |
| --- | --- |
| Green results were given for code the contract did not speak to | "Unchanged, checked" and "Unchanged, unchecked" replace "no impact"; contract coverage (12.2, 12.4) |
| Platform-specific breakages were averaged away | Host-dependent facts stated on capabilities, and contracts compared once per host binding (10.3, 12.1) |
| Interfaces had no contract, so implementations drifted | Interfaces carry contracts that bind every implementation (11.2) |
| Fixing a false promise was reported as breaking it | `corrects` (12.3) |
| False alarms on controls | Removal breaking only for callers left untouched; effects compared per component; lint on implementation-detail informal clauses (12.3) |
| Contracts were costly to write | `scribe draft` (13.1) |

### B.3 Decisions from design review

- Readability outranks one-spelling-per-idea: several forms are fine when the formatter, not the author, chooses (2).
- School-maths symbols for comparison and arithmetic; words for logic; no assignment symbol, so `=` is always equality (5).
- No null: absence is `maybe` and `nothing` (3).
- Names use underscores drawn at half contrast; no capitals-only styles anywhere, including tool output.
- No anonymous functions or long-lived closures; the allocator is passed as context, as in Zig (4, 7).
- No `uses nothing` line: an operation with no `uses` line is pure.
- "Contract", not "description", for the clauses between header and body, because they are enforced, not informational.
- Update forms such as `subtract amount from balance of account` (2).
- The aims are reordered: rapid impact assessment first, human readability second, correctness by construction third. Impact assessment is the problem Scribe exists to solve; readability is how it must be solved (1).
- Version 1 emits WebAssembly only (1, 13).
- `return` replaces `give back` as the statement that ends an operation with its result (4.2).
- Reserved words are judged by what they cost readers, not authors: Scribe is written by machines, so a word reserved to make a line unambiguous is cheap. `increment`, `decrement`, `add` and `subtract` are reserved so the update forms can never be read as calls (2.1, 2.5).
- Source never names an operating system. A difference between hosts is a fact about a capability, declared in its interface; this replaces clauses scoped `on <platform>` (10.3).
- Scribe is not described as a systems language. Having no garbage collector is a choice about memory management, managed at compile time as in Rust, not about the kind of software Scribe is for (1, 7).

### B.4 Earlier documents

[Scribe: Syntax Philosophy and Worked Examples](https://claude.ai/code/artifact/ad546c4f-aff1-4425-93f2-386b7fc8ee29) holds the 50 worked examples and the full experiment designs. This specification supersedes it where they differ.
