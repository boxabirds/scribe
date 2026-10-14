# Scribe Language Specification

Oct 14, 2026 · @Julian Harris

**Draft 0.4.** This document is the whole specification as it stands. It replaces the syntax doc, the pilot results and the change replay as the place to read what Scribe is; those remain as evidence, summarised in Appendix B.

Every rule is normative unless it carries one of two marks:

- **Proposed**: recommended by the evidence but not yet agreed.
- **Open**: a real choice that has not been made. Section 14 lists every one, with what blocks it.

Code examples are shown in the **review view**, the default way people read Scribe (Section 2). Where the stored text differs, the difference is only the drawing of `≤`, `≥` and `≠`, which are stored as `<=`, `>=` and `!=`.

## 1. Purpose and principles

Scribe is a language written by agents and read by humans. No person is expected to write it: AI agents write it, with endless patience for complexity and rigour, and people read and review what they wrote. That division decides almost everything else.

It has three aims, in order:

1. **Quick, reliable impact assessment.** Most languages cannot say reliably what a proposed change affects, because so much is decided only at run time. Scribe removes that uncertainty: everything that links one piece of code to another (ownership, effects, capabilities, failures, states, contracts) is stated in the source, so standard static analysis finds every linkage, and the impact of a change is computed before it is merged, in time that grows with the change and what depends on it, not with the size of the codebase.
2. **As easy to read as possible.** The approach is plain English plus basic maths operators, drawing on similar languages and, above all, on what did and did not work across them over the decades. Every line has exactly one reading, and a reader who knows what the words mean sees it without knowing any rule of precedence beyond school arithmetic: text that can be read two ways is not readable, and needing a manual to read an expression is a failure of design. This holds for the stored text and for every view. When readability and impact assessment conflict, Scribe looks for a spelling that serves both; only when there is none does impact assessment win, and the cost to readability is recorded as a decision. Against every other aim, readability wins.
3. **Almost provably correct.** Because agents write the code, the language can demand validation no person would put up with writing: Rust-style ownership, states tracked in types, contracts on every operation. No undefined behaviour, no missing-value errors, no data races, no leaked resources, no unhandled failures; beyond that, contracts are proven wherever the compiler can, and checked at run time where it cannot. The writer's effort is free; the reader's is not, so every check must still read plainly.

And two constraints:

- **Fast, predictable compilation**, closer to Zig than to Rust.
- **No garbage collector** and no hidden allocation. Memory is managed at compile time: the compiler checks ownership from each parameter's convention (Section 7), and nothing is reclaimed at run time except where an owner's block ends.

Version 1 compiles to WebAssembly.

Principles that follow from these and decide individual rules:

| Principle | Consequence |
| --- | --- |
| Rigour costs the writer nothing, reading costs the reader | Validation can be as demanding as correctness needs, because agents write it; every construct is judged by how it reads, not by how hard it is to write |
| What a reviewer must judge is visible before the body | Failures, capabilities, effects and contracts sit in each operation's header |
| One stored form, many readings | The source has exactly one canonical text; people see it through views, and the formatter chooses the clearest phrasing |
| Words where words are clearer, symbols where symbols are | School-level maths symbols for arithmetic and comparison; words for logic and structure |
| A promise is code, not commentary | Contracts are checked against the body on every build. A contract holds only conditions the compiler can check; explanations go in `note` lines |
| Never overclaim | Every proof, verdict and report states the unchecked code (`trust` blocks) it rests on |
| Nothing implicit that matters | No ambient authority, no implicit conversion, no missing value without a `maybe`, no failure without a `can fail with` |
| If in doubt, make it a library | As in Rust, the language is kept minimal. A construct is language only if it cannot be an ordinary type or operation without losing what the compiler must check; everything else (capability narrowing, time, pipelines, serialisation) is a library |
| Any expression wherever a value goes | No position in the grammar is limited to a name or a literal without a stated reason; types, not keywords, decide what fits. A value in a contract or a grant of authority must be a constant expression, because impact analysis compares it without running code |

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
| **Review** (default) | Reviewers | `<=`, `>=`, `!=` drawn as `≤`, `≥`, `≠`; display forms applied; underscores in names drawn at about half contrast; a `pure` tag on operations with no `uses` line |
| **Compact** | Experts | As review, plus logic words drawn as `∧ ∨ ¬ ∈ ∀ ∃` |
| **Localised** | Readers of other languages | Keywords replaced one for one; names, strings and notes untouched |

A view must round-trip: rendering stored text into a view and converting it back gives identical bytes. A view that fails this does not ship. Errors, documentation and search always quote the stored text, with the reader's view alongside.

### 2.3 Notation

Symbols are used only where school mathematics has already taught them to most readers. Everything else is a word.

| Meaning | Stored | Also accepted | Review view |
| --- | --- | --- | --- |
| Arithmetic | `+ - * /`, `mod` |  | same |
| Less than, more than | `<`, `>` |  | `<`, `>` |
| At most, at least | `<=`, `>=` | `≤`, `≥` | `≤`, `≥` |
| Equal, not equal | `=`, `!=` | `==`, `≠` | `=`, `≠` |
| And, or, not | `and`, `or`, `not` | `∧`, `&&`, `∨`, `\|\|`, `¬`, `!` | words |
| Member of | `is in` | `∈` | `is in` |
| For all, there is | `for all`, `there is` | `∀`, `∃` | words |

Rules:

- Comparisons chain as in mathematics: `0 ≤ low ≤ high ≤ count of items`.
- There is no assignment symbol, so `=` is always equality.
- Equality is only `=` and `≠`. `is` only introduces a test: a variant (`status is Paid`), a missing value (`result is nothing`), a predicate (`items is sorted`), membership (`x is in items`); `is not` negates the test.
- `and` and `or` never mix on one line. A condition that combines them is written as a condition group (Section 5.2).
- The only precedence a reader needs is what school arithmetic teaches (Section 5.2).
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
- Every statement and every clause is one whole line, and nothing continues onto the next line. A view may wrap a long line on screen; that wrapping is never part of the text.
- A block indented under a line is part of that line's structure, for conditions as for statements: a condition group's items sit in the block under its header (Section 5.2).
- No assignment inside an expression.
- Lists of parts are separated by commas. `and` is only logic.

## 3. Values and types

### 3.1 Built-in types

Integers (`Int`, `Nat`, and fixed widths), decimals, `Bool`, `Text`, `Bytes`, lists, and units of measure. There are no implicit conversions: a conversion is always a named operation.

Bit operations are named operations on the fixed-width unsigned types, never symbols and never on `Int`: `bitwise_and flags with mask`, `bitwise_or`, `bitwise_xor`, `bitwise_not`, `shift_left value by 1`, `shift_right`, `rotate_left`, `rotate_right`, and `bit 3 of flags`. A shift by the width or more is a contract violation, proven or checked like any other. Integers can be written in hex and binary, with `_` between digits: `0xFF`, `0xFFFF_FFFF`, `0b1010_0001`.

**Open:** the exact numeric tower, fixed-width names, whether `Text` is bytes or characters, and character and byte literals (Section 14).

### 3.2 Records

```
type OrderLine
    product as a ProductId
    quantity as an Int
    unit_price as Money
```

A record may state invariants. They hold whenever control leaves an operation of the type:

```
type Order
    lines as a List of OrderLine
    status as an OrderStatus
    invariant lines is not empty or status is Draft
```

A `model` line gives the abstract state that contracts talk about, so contracts never mention the representation:

```
type Queue of T
    capacity as a Count
    items as a Ring of T
    model contents as sequence_of items
    invariant count of contents ≤ capacity
```

### 3.3 Enums

```
enum OrderStatus
    Draft
    Paid with paid_at as an Instant
    Shipped with tracking as Text
    Cancelled with reason as Text
```

An enum's variants may carry data, as in Rust and Swift. An enum is used through `choose`, which must handle every variant. Adding a variant makes every `choose` that does not handle it fail to compile.

### 3.4 Missing values

There is no null. A value that may be absent has type `maybe T`, with values `some x` and `nothing`. A `maybe` value must go through `choose` before the value inside it is used. `some` applies to the next term, as a minus sign does: `some (total + tax)`. Written without brackets, `some total + tax` adds a number to a `maybe`, which is a type error, not a different reading.

### 3.5 Refinements

A refinement carries its condition wherever the type goes:

```
type Percent is an Int with value 0 through 100
type Small is an Int with value 0 until (limit_of plan + 1)
```

A limit in a type is one term: a name, an `of`-path, a literal or brackets, so the word after it always belongs to the type. A condition that is not a range, such as "a well-formed email address", is a distinct type built by a checking operation (8.3), or an `invariant` on a record.

### 3.6 Units

A number followed directly by a unit is a quantity: `5 kilometres`, `1 GiB`, `30 seconds`. Units are checked: adding a length to an area is an error, and a length divided by a time is a speed. Units are defined by libraries, not by the language.

The language has no concept of time: no time types, no date literals, no time comparisons and no time limits. Instants, calendar dates, time zones and timeouts are standard-library types and operations, and anything that reads the time does so through a clock capability (8.1). Values of those types are compared with the ordinary comparisons: `due < now`.

### 3.7 Ranges

`from 1 through 10` includes 10; `from 1 until 10` stops at 9. There is no other range form.

## 4. Operations and statements

### 4.1 Operations

An operation is a header line, its contract, and its body:

```
to withdraw amount from account
    given amount is Money
    given account is an editable Account
    produces a Receipt
    can fail with InsufficientFunds
    expect amount > 0
    expect account is open
    ensure on success, balance of account = balance of account at entry - amount
    ensure on failure, balance of account = balance of account at entry
    changes only balance of account

    if amount > balance of account
        fail with InsufficientFunds
    subtract amount from balance of account
    return receipt_for amount
```

The header line names the operation, its first parameter, and further parameters introduced by label words: `to withdraw amount from account`. The label words are a fixed set of single words: `from`, `to`, `in`, `into`, `for`, `with`, `by`, `as`, `of`. A call uses the same shape: `withdraw 50 from savings`.

An operation may take several unlabelled parameters, separated by commas, before any labelled ones: `to gcd first, second`, called as `gcd 12, 18`. Use a label where one reads better: `to withdraw amount from account`.

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
| Do nothing | `do nothing` |
| Hold a resource | `hold input as open path for reading with files` (Section 7) |
| Concurrency | `together` (Section 9) |

A name, an element (`set item i of heap to x`), an `of`-path or a possessive path can be set. The amount in `add … to` and `subtract … from` is one term, so `to` and `from` always belong to the update: `add price of item to total`, `add (price for item in usd) to total`.

`do nothing` is for a `when` branch with nothing to do. `choose` has no catch-all branch, so a new variant is never silently absorbed: every `choose` must say what happens to it, even if that is nothing.

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
- A call with several unlabelled arguments, `gcd a, b`, stands bare only as a whole statement or as the whole value of `let`, `var`, `set`, `return` or `fail`. Anywhere else it goes in parentheses, `[(gcd 12, 18), 6]` or `if (gcd a, b) = 1`, because a comma also separates list items, parts and quantifier ranges.
- `account's balance` is another spelling of `balance of account`, and possessives chain: `user's manager's name` is `name of manager of user`. A path uses possessives or `of`, never both, so `name of user's manager` is an error: each direction is clear alone, but mixed, the reader would need a precedence rule to know which applies first. The formatter chooses the clearer spelling for each path.

### 5.2 Precedence and grouping

Every line has exactly one reading (Section 1), and the only precedence a reader needs is what school arithmetic teaches:

| From tightest to loosest | Example | Why a reader already knows it |
| --- | --- | --- |
| Calls, `of`, `at entry` | `total of cart + tax` is `(total of cart) + tax` | Like `sin x + 1` |
| `-`, `some`, `try` before a term | `-a + b`, `(try parse_int text) + 1` | A minus sign applies to the next term |
| `*`, `/`, `mod`, then `+`, `-` | `a + b * c` | School arithmetic |
| Comparisons, which chain | `0 ≤ i < n` | School arithmetic |
| A chain of `and`, or a chain of `or` | `x > 0 and y > 0` | Plain English |

Brackets group arithmetic only: `(a + b) * c`. They never group logic.

`and` and `or` never mix on one line, and `not` negates a whole condition, never one link of a chain: `not paid and shipped` is an error. One condition implying another is written across two aligned lines as `if` and `then`:

```
ensure
    if for all x in items, x > 0
    then count of items < limit
```

Anything else is grouped by layout. A **condition group** is `all of`, `any of` or `not any of` followed by its conditions, one per line in the block beneath, or several on one line separated by commas. Groups nest; a line inside a group holds one condition, with no `and` or `or`, so the group's header is the only thing joining its lines:

```
return all of
    status of order is in [Paid, Shipped]
    customer is active
    not any of
        order is disputed
        customer is suspended
    any of
        days_since_payment of order ≤ 30
        all of customer is premium, days_since_payment of order ≤ 90
```

On one line, an item holds no word that covers the rest of the line (a quantifier, `where`, a comprehension), since it would take in the items after it; such an item goes on its own line. A comparison that ends in `where` or a comprehension is a whole condition, never one link of a chain. A comma at the end of a line in a group is optional. If one is written, another item of the same group must follow it, so a comma that disagrees with the indentation is an error. The formatter leaves these commas out of the stored text; the review view draws them, and draws a labelled rule beside each group. A group can follow `expect`, `ensure`, `invariant`, `holds when`, `let … be` and `return`.

A group can also be the condition of an `if` or a `repeat while`. Then `then` (for `if`) or `do` (for `repeat while`) stands on its own line between the condition's block and the statement's, so the two blocks are never confused:

```
if all of
    account is open
    balance of account ≥ amount
then
    subtract amount from balance of account
otherwise
    do nothing

repeat while all of
    balance of account < 0
    retries of account > 0
do
    decrement retries of account
```

An `if` / `then` pair is not a statement's condition, because its `then` would read as the statement's; name it with `let` first.

### 5.3 Constructing records

```
let order be an Order from cart, receipt with status as Paid
let origin be a Point with x as 0, y as 0
return a Dashboard from profile, orders, alerts
```

Parts are given by position after `from`, by name after `with`, or both, separated by commas. A part by position is written like a call argument, so a labelled call or an operator in it goes in brackets: `an Order from (total of cart + tax), receipt`. A record built this way runs to the end of its line, like a comprehension: it is the whole value of its line, or in brackets, so `an Order from cart and receipt` is an error rather than a record joined to a condition.

### 5.4 Collections

One comprehension form, used in code and in contracts:

```
let paid be sum of total_of order for each order in orders where status of order is Paid
```

A comprehension follows `of`, as above, or stands alone as a list in list brackets: `[price of item for each item in items]`. Like `where`, it covers the rest of its line (Section 5.5), so a comprehension is the whole value of its line, the last operand of a comparison, or in brackets. There is no `map`, `filter` or `reduce`.

### 5.5 Quantifiers and conditions

```
for all x in items, x ≤ result
for all j from 0 until low, item j of items < target
there is an item in items where price of item > limit
```

A word that opens a scope covers the rest of its line: `for all`, `there is`, `where`, a comprehension's `for each`, and `if …,` in an `ensure`. This is the same rule a statement header follows, and the only one: there is no other way for a scope to end. Any other grouping is written with a block (Section 5.2), so a quantifier is never one operand among others on a line.

### 5.6 Strings

Text in double quotes, with `{expression}` interpolation: `"Your order {id of order} is confirmed."`.

### 5.7 What `is` means

`x is y` is parsed the same way in every case, but what it means depends on what `y` is: a variant (`status is Paid`), `nothing`, or a pattern that binds a name (`result is some p`). This is the one place where meaning needs name resolution, which happens after parsing.

## 6. Failures

An operation lists every way it can fail. The compiler checks the list is complete: a body cannot fail in an undeclared way, and an operation that handles all its callees' failures has no `can fail with` line.

```
to parse_port text
    given text is Text
    produces a Port
    can fail with NotANumber, OutOfRange

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

A failure is a value. Failures are declared as variants of an `enum`, so a failure can carry fields and two failures with the same name can be told apart, and `fail with` takes any expression whose value is a failure: `fail with an OutOfRange with value as clamped`. Failures that carried no data were the most common gap in translated real code. **Open:** the type rules for failure values (Section 14).

**Open:** whether panics exist at all. The current rule is that an operation cannot panic, and the compiler proves it; what a bounds check it cannot prove does is undecided.

## 7. Memory, ownership and resources

### 7.1 Values, not references

A variable holds its value, not a pointer to it. Each value has exactly one **owner**, the variable or field that holds it; when the owner goes out of scope the value is freed. There are no references to write and no lifetime annotations. Safety comes from how each parameter may be used, stated in the contract:

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
hold input as open path for reading with files
    return try read_line from input
```

A linear value bound with `let` and not released on some path is a compile error naming that path.

A type can list the states its values move through, and a state is written as an adjective before the type. Using a value in the wrong state is a compile error, not a run-time check, and every change of state is visible in a signature, where impact analysis compares it:

```
type Connection
    socket as a Socket
    states Closed, Open

to send data to connection
    given data is Bytes
    given connection is an editable Open Connection

to close connection
    given connection is a consumed Open Connection
    produces a Closed Connection
```

Nothing can send to a `Closed Connection`. A conversation between two parties (a protocol) is written the same way: each point in the conversation is a state of the channel, and each message an operation that consumes the channel in one state and produces it in the next.

### 7.4 Shared state

**Open:** how to write data that several owners share or that forms cycles, such as graphs and doubly linked lists. The candidates are explicit counted references and handles into an arena; either way the readers and writers are declared in contracts. Real code needs this (Appendix B), so the decision cannot be deferred long.

## 8. Effects, capabilities and authority

### 8.1 No ambient authority

Nothing can open a file, a socket or a clock unless it has been handed a **capability** for it. All authority starts in one place, the program's entry point, and can only be narrowed as it is passed on:

```
to main root
    given root is the Authority
    uses root

    let reports be read_only_directory "/reports" from root
    let partner be network_endpoint "export.partner.example", 443 from root
    run ReportExporter with [reports, partner]
```

Narrowing a capability is an ordinary standard-library operation, not syntax; its contract says what the result can reach. Because authority only flows along capabilities, and the arguments that narrow it are constants, "what can this component reach?" has a static answer, with the path as evidence:

```
$ scribe query reach ReportExporter network
ReportExporter can reach: host "export.partner.example" port 443 only
    path: main > network_endpoint > partner > ReportExporter
```

### 8.2 Effects

An operation's effects are what it may do beyond computing its result. They are declared in its contract and checked against the body:

| Effect | Declared as |
| --- | --- |
| Use a capability (files, network, clock, database,    N  another component) | `uses files, clock` |
| Request memory (**Proposed**) | `allocates` |

An operation with none of these is pure, and calling an effectful operation from it is a compile error naming the call. Clocks are capabilities, so code that depends on the time says so, and tests pass a fake clock.

### 8.3 Information flow

Information flow is controlled with distinct types, as in Rust's newtype pattern; there are no labels. Text from outside is an `UntrustedText`, and a database query takes a `UserName`, so untrusted text reaches a query only through a checking operation that produces one: `let checked be try check_user_name name`. Secret data is a distinct type such as `CardNumber`, and it becomes public only through a named conversion operation, which every review and impact report shows wherever it is called.

## 9. Concurrency

Concurrent structure is written as a visible shape, and no task outlives the block that started it.

### 9.1 Doing several things at once

```
to load_dashboard user with services
    given user is a UserId
    given services is the DashboardServices
    produces a Dashboard
    can fail with DashboardUnavailable
    uses services

    together
        let profile be fetch profile of user from services
        let orders be fetch orders of user from services
        let alerts be fetch alerts of user from services, otherwise nothing
    return a Dashboard from profile, orders, alerts
```

Everything inside `together` starts at once, and the block ends when every line has finished. If a required line fails, the others are cancelled and the failure goes up; `, otherwise` marks a line whose failure is acceptable, with the value to use instead. Cancellation unwinds through `hold`, so resources are still released. A time limit is not part of the language: it is a standard-library operation on a clock capability. So is a pipeline: stages joined by bounded queues are built from `together` and a channel type in the standard library.

### 9.2 No shared mutation

Two branches of a `together` block cannot change the same variable; the compiler names both lines. Each branch produces its own result instead. With no shared mutable values there is nothing for a lock to protect, and no data race can be written.

### 9.3 Pipelines (removed)

Removed from the language. A pipeline is a standard-library operation built on `together` and a channel type (9.5).

### 9.4 Work that survives a restart

Removed from the language. Resuming work after a restart, running it at most once per key and remembering its progress are services of the platform a program runs on, not part of what the program means. Where a program needs them, they come from a library or a capability, used through ordinary operations.

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
| `can fail with A, B` | Every way it can fail | Compiler, completely |
| `uses`, `allocates` | Its effects and capabilities. Waiting needs a capability too, so it shows in `uses` (9.5) | Compiler, completely |
| `expect condition` | What callers must ensure | Proof at call sites, or run-time check |
| `ensure condition` | What it guarantees. Forms: `ensure on success, ...`, `ensure on failure, ...`, `ensure if result is some p, ...` | Proof where the checker can; otherwise a run-time check, and reports say which (10.5) |
| `changes only x, y` | Everything else is untouched | Compiler |
| `example call is value` | An exact result for an exact input | Run on every build |

Every clause is a condition or a declaration the compiler checks. A quoted sentence is Text, not a condition, so it cannot be a clause; an explanation goes in a `note` line. Inside an `ensure`, `x at entry` is the value of `x` when the operation began.

### 10.2 Examples

Examples are part of the contract, not separate tests. They are the cheapest way to pin exact behaviour, and the change replay found exact behaviour was what caught most real breakages.

```
to redirect_body for location
    given location is Text
    produces Text
    example redirect_body for "/a" is "<a href=\"/a\">See Other</a>.\n\n"
```

Properties generalise examples and run as generated tests: `property for all items as a List of Int, items is empty or largest of items is in items`.

### 10.3 Host-dependent behaviour

Source never names an operating system or a host. Everything that can behave differently from one host to another reaches code through a capability (Section 8.1), so a difference between hosts is a fact about a capability, and it is stated where the capability is declared.

A fact that varies by host needs no syntax of its own: it is an operation of the capability's interface, and a contract calls it like any other:

```
interface Files
    to replaces_existing of files
        given files is a Self
        produces a Bool

    to rename source to target in files
        given source is a Path
        given target is a Path
        given files is an editable Self
        can fail with NotFound, TargetExists
        ensure on failure TargetExists, not replaces_existing of files
```

An operation whose contract tests a host-dependent fact holds wherever the fact has the value it tests for; a contract that tests none must hold on every host. Each host binding, the implementation of a capability's interface for one host, gives every fact its value. The toolchain supplies the host bindings, and they are the only place a particular operating system is named.

Contracts are compared once for each host binding (Section 12).

### 10.4 Loops, types and predicates

- Loops carry `invariant` lines, and `repeat while` loops a `decreases` line naming a quantity that shrinks on every pass and cannot go below zero. A `for each` over a range always terminates.
- Types carry `invariant` and `model` lines (Section 3.2).
- A predicate is an ordinary operation that produces a `Bool`: `to sorted of items`, then `expect sorted of items`.

### 10.5 How clauses are checked

The source does not say how hard to check. On every build the compiler proves that there is no undefined behaviour, no unhandled failure and no run-time error, and checks types, effects, capabilities, units and states. It then proves each `ensure`, `expect` and invariant where it can; a clause it cannot prove is checked at run time instead. `example` and `property` lines run as tests. Every report says, for each clause, which of these held it.

Two statements belong to the body:

```
to sorted_copy of items with native
    given items is a List of Int
    given native is the NativeSort
    produces a List of Int
    ensure result is sorted
    uses native
    let copy be sort items with native
    assert count of copy = count of items
    trust
        assert copy is sorted
    return copy
```

- `assert condition` is a condition the compiler must prove at that point, and may then use. Where it cannot be proven, it is checked at run time.
- `trust` opens a block the checker does not verify, and an `assert` inside it is taken as true. Trust blocks are counted per package, and every review and report that depends on one names it with its file and line. Who reviews them is a matter for the repository's own rules, not the language.

### 10.6 Corrections (removed)

Removed from the language. A `corrects` line described an earlier version of the program, not this one. Whether a changed clause fixes a false promise is said in the change's review, where impact analysis lists the dependents that relied on it (12.3).

### 10.7 Writing complete contracts

Impact analysis is only as good as contracts are complete (Appendix B), so the toolchain treats completeness as a measured property:

- `scribe draft` proposes a first contract from the code: failures, effects, capabilities, and examples recorded from tests. A person reviews and edits it.
- **Contract coverage** reports, for each operation, the observable outputs that no clause or example constrains: results, failure kinds and effects. It is shown in reviews as a figure per component.
- An operation that provides results but has no `ensure` or `example` gets a weak-contract warning.

## 11. Components, interfaces and manifests

### 11.1 Components and manifests

A **component** is the unit of review, versioning and deployment. Its **manifest** is its contract as a whole, written at the top of the component's source:

```
component Checkout

    needs
        Payments as payments
            uses charge, refund, total_charged_to
        Inventory as inventory
            uses reserve, release

    provides
        to place_order cart for customer with request_id
            given cart is a Cart
            given customer is a CustomerId
            given request_id is a RequestId
            produces an Order
            can fail with EmptyCart, OutOfStock, PaymentDeclined, RefundFailed
            expect cart is not empty
            ensure on success, status of result is Paid
            uses payments, inventory

```

| Section | Says |
| --- | --- |
| `needs` | Each component it depends on, and which of its operations it uses. Impact is judged against exactly these |
| `provides` | The operations other components may call, with their contracts |

Version numbers, of the component and of what it needs, are not part of the language: they are kept in the component's package file. The toolchain computes the next version from the change to the manifest (Section 12), and a person may raise it but not lower it.

### 11.2 Interfaces

An interface states a contract that every implementation must meet and every caller may rely on:

```
interface Reader
    to read into buffer from reader
        given buffer is an editable Bytes
        given reader is an editable Self
        produces a Count
        can fail with EndOfData, ReadFailed
        ensure result ≤ count of buffer
```

Each implementation is checked against the interface's contract. A change to an interface's contract affects every implementation and every caller.

### 11.3 Stored data (schemas removed)

Removed from the language. A stored or transmitted format is an ordinary type, and reading an older version is an ordinary operation with a contract:

```
to upgrade old
    given old is an OrderV3
    produces an OrderV4
    ensure gift_message of result is nothing
    ensure lines of result = lines of old
```

Serialisation and version tags are a standard-library matter. Impact analysis compares these types and operations like any others.

### 11.4 Protocols (removed)

Removed from the language. Checked conversations (session types) are research-grade; a protocol is written as ordinary operations and types, and may return after 1.0 as a library.

### 11.5 Systems (removed)

Removed from the language. A program is assembled in code: its entry point hands each component its capabilities and calls it (8.1), and each component's `needs` say what it calls. Whole-program questions, such as what can reach a host or where a sensitive type can flow, are answered from that code by `scribe query`; a project that wants one checked on every change runs the query in its own build.

## 12. Impact analysis

Impact analysis answers one question before a change is merged: **what could this change break, for whom, and how sure are we?** It works from contracts, not from the code's text, so it can answer for callers it has never seen.

### 12.1 What is compared

`scribe impact` compares two versions of a component: its manifest, every contract in it, its effects and authority, and whether its implementation still meets its contracts. It then checks the result against:

1. **every known dependent**, using the clauses that dependent's `needs` entry relies on (written or inferred from its code);
2. **any possible caller**, using the public contract alone, because most users of a library are outside its repository.

Where a contract tests a host-dependent fact (Section 10.3), the comparison is made separately for each host binding, and a verdict names the bindings it applies to. A guarantee that weakens only where file names ignore case is reported as breaking there, not hidden in an average.

### 12.2 Verdicts

Each provided operation and each dependent gets exactly one verdict:

| Verdict | Meaning | Must carry |
| --- | --- | --- |
| Compatible | The new contract refines the old: preconditions no stronger, guarantees no weaker, no new failure or effect | A proof, or the solver's reason |
| Breaking | Some caller can observe a difference the old contract promised would not happen | A witness: a concrete input or scenario |
| Needs review | The tool cannot decide: a guarantee changed in a way that may or may not matter, or a `trust` block changed | The clause and the reason |
| Authority change | The component now holds or uses authority it did not before | The new capability and the path to it |
| Unchanged, checked | The contract did not change, the new implementation was checked against it, and the contract covers the code that changed | How each clause was checked, and the coverage |
| Unchanged, unchecked | The contract did not change, but the changed code does something the contract does not speak to, or was not checked | The uncovered lines or behaviours |

"Unchanged, unchecked" is the tool being honest about its blind spot. A green result is only ever as good as the contract behind it, so the report never says a change has no impact when the contract does not cover what changed.

### 12.3 Rules

| Change to a contract | Verdict |
| --- | --- |
| Weaker precondition; stronger guarantee; new operation | Compatible |
| Stronger precondition | Breaking for any caller that may not meet it, with that caller's input as witness |
| Weaker or removed guarantee | Breaking for dependents that rely on it; needs review for unknown callers |
| New way to fail | Breaking for callers that handle every failure by name; needs review otherwise |
| New or changed `trust` block | Needs review |
| New capability or effect for the component | Authority change |
| Removed or renamed operation | Breaking for every caller the same change leaves untouched; compatible for callers the change updates |

Two points of detail:

- **Effects are compared per component.** Moving an existing effect from one operation to another inside the same component is not an authority change. It is reported against the operations concerned under the rules above.
- **A removed guarantee is reported, even when it was never true.** A change that drops a clause the old code did not meet is still a weaker guarantee: dependents that relied on it are listed, because their code may have depended on the false promise in practice. Saying that the old clause was false belongs in the change's review, not in the source.

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
    capture: trust block changed, lines 41–47

Authority change (0)

Unchanged, checked (6)
    refund, total_charged_to, void, list_charges, receipt   proven, coverage 100%

Unchanged, unchecked (1)
    authorise: changed lines 88–104 add a retry loop that no clause covers

Suggested version: 4.0.0 (a breaking change)
```

### 12.6 Deployment (removed)

Removed from the language. Ordering a rollout so that no component runs alongside a dependency it breaks against is a deployment tool, which can be built on the impact comparison of Section 12; it is not part of what a program means.

## 13. Toolchain and diagnostics

### 13.1 Commands

| Command | Does |
| --- | --- |
| `scribe check` | Parses, formats to the stored form, type-checks, checks ownership, effects, authority and every contract. This is the build's gate |
| `scribe build` | Check, then compile to WebAssembly (version 1), in debug or release mode |
| `scribe test` | Runs `example` and `property` lines and test blocks |
| `scribe prove` | Proves contracts with the solver and reports any it cannot, with a counterexample where one exists |
| `scribe impact` | Compares two versions (Section 12) |
| `scribe coverage` | Reports which code each contract clause constrains (12.4) |
| `scribe draft` | Proposes a contract for an operation from its body and tests. The proposal is marked as a draft until a person accepts it, and never counts as checked on its own |
| `scribe query reach` | Answers "what can reach this capability, value or component, and by which path?" |
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
| 2 | No `async` keyword: waiting takes a capability, which shows in `uses`, and the runtime schedules | Proposed | A prototype scheduler and a latency measurement |
| 3 | `allocates` as a contract clause, with the allocator passed as context | Proposed | Measuring how many contracts it adds lines to |
| 4 | ~~A rule language for system files~~ | Removed | System files are removed; `scribe query` answers reach and flow questions from the code (8.1) |
| 5 | Possessive paths: `account's balance` | Settled | Allowed, never mixed with `of` in one path (5.1) |
| 6 | Reduce the keyword set (now 78 hard, 4 soft) | Open | Experiment 1, the comprehension test, extended to compare candidate words for the same idea: for example `hold` against `with` and `lease` |
| 7 | Parser strategy: the grammar is LALR(1) with no conflicts (Appendix A); whether the compiler's parser is generated from it or written by hand | Open | Comparing the error messages each gives on the pilot's real mistakes |
| 8 | Code generator for WebAssembly, the version 1 target: a back end of our own, or one built on LLVM or Cranelift | Open | A compile-time benchmark on the pilot corpus |
| 9 | Solver for proving contracts | Open | Choosing a solver that is fast and gives stable results across versions |
| 10 | Localized views beyond English | Open | Round-trip proof for at least one other language |
| 11 | Which host bindings impact analysis compares by default (10.3) | Open | Re-expressing the replay's platform-specific breakages with it |
| 12 | Type rules for failures as values: how a failure `enum` relates to `can fail with`, and how impact analysis compares a failure's fields | Open | Roadmap phase 5, with the translation wave that needs them |
| 13 | ~~Cost bounds (`cost at most`)~~ | Removed | A cost bound is not an instruction the program carries out; speed is measured by tools outside the language |
| 14 | A condition group as the condition of an `if` or `repeat while` | Settled | `then` or `do` on its own line between the two blocks (5.2) |

## Appendix A: grammar notes

The reference grammar is `grammar/scribe.lark` (version 0.3) in this project; `scribe-grammar-v0.2.lark` is the version the pilot measured. It is an Earley grammar: it states the language precisely but is not yet a production parser. `tools/check_grammar.py` tests it against a file per construct in this specification, a file per forbidden form, this specification's code blocks and the pilot corpora.

**Vocabulary.** 82 reserved words: 78 hard keywords, which can never be names, and 4 soft keywords, which are names everywhere except in the position that gives them meaning. The articles `a`, `an` and `the` are hard. A few two-word phrases are read by the lexer as one token, so a parser looking one token ahead sees both words: `for each`, `at entry` and `, otherwise`; a comma that ends a line is read with the line's end. None of this changes the text.

**No special rules.** The grammar is unambiguous by construction. Converted to Bison it has no LALR(1) conflicts, which proves that no text has two readings, and `tools/check_lalr.py` fails the build if one appears. `tools/check_grammar.py` also parses every test, every code block in this specification and the pilot corpora with no disambiguation applied. The pilot's rules D1 and D2 are now the grammar's own structure (Section 5.5), mixing `and` with `or` is a condition group (Section 5.2), and a label word belongs to the outermost call on its line because an argument is one term: a labelled call inside an argument is in brackets.

Constructions list their parts with commas, never `and`, so `and` is always logical.

**Parser class.** LALR(1), with no conflicts (version 0.2 had 172 shift/reduce and 20 reduce/reduce). Reaching zero changed what can be written in a few places, all recorded in Appendix B.3: phrase words such as `only` became hard keywords; records built with fields, and comparisons that end in `where` or a comprehension, became whole values; and a value followed by a phrase word (`add … to`, a limit in a type) became one term.

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
| Fixing a false promise was reported as breaking it | `corrects` (12.3); later removed, because it described an old version of the program (10.6) |
| False alarms on controls | Removal breaking only for callers left untouched; effects compared per component; lint on implementation-detail informal clauses (12.3), later removed with informal clauses (10.1) |
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
- An operation may take several unlabelled parameters (4.1); a call with several unlabelled arguments stands bare only as a whole statement or value (5.1).
- Possessive paths are allowed and never mixed with `of` in one path, the same rule that keeps `and` and `or` apart (5.1).
- Records can be built with fields by name: `a Point with x as 1, y as 2` (5.3). One element can be set: `set item i of heap to x`. `do nothing` fills a branch with nothing to do (4.2).
- Every statement and every clause is one line; no clause continues onto the next (2.6).
- A line that can be read two ways is not readable. Where a reading would otherwise depend on a rule, the language makes the structure visible instead (1):
    - conditions that mix `and` and `or` are condition groups, `all of`, `any of` and `not any of`, one condition per line, nesting like statement blocks; commas at the ends of their lines are optional, checked if present, and drawn by the review view (2.6, 5.2);
    - a word that opens a scope covers the rest of its line, replacing the pilot's rules D1 and D2 (5.5);
    - brackets group arithmetic only, and a comprehension standing alone is a list in list brackets (5.2, 5.4);
    - `-`, `some` and `try` apply to the next term; `not` negates only a whole condition (5.2);
    - equality is only `=` and `≠`, and `is` only introduces a test (2.3);
    - bit operations are named operations, with hex and binary literals (3.1).
- Any expression goes wherever a value goes. A grammar rule that takes only a name or a literal needs a stated reason; otherwise types decide what fits, and mistakes the grammar no longer catches are the checker's to report (1).
- A failure is a value, and `fail with` takes any expression (6).
- The grammar has no LALR(1) conflicts, so it is proven that no text has two readings; the build fails if one appears (Appendix A). Reaching zero meant:
    - `only` is a hard keyword, because each follows a value and could otherwise be read as part of it;
    - a record built with fields is a whole value or in brackets (5.3), and so is a comparison that ends in `where` or a comprehension (5.2); this turned another of the pilot's silent misreadings, `an Order from cart and receipt with status Paid`, into an error;
    - the amount in `add` and `subtract`, and a limit in a type, is one term (3.5, 4.2);
    - an item of a one-line condition group holds no word that covers the rest of the line (5.2) (5.2).
- The language holds only what the running program does and what is checked about it. How a program is monitored, retried by configuration, grouped into failure domains or rolled out belongs to operations tooling: assumption monitors, a manifest's `failure` section, a system's `failure domains` and `scribe deploy plan` are removed (11.1, 11.5, 12.6). A retry is written in code, where it can be seen.
- The language describes programs, not organisations. Owners and teams are not part of it: a `trust` block marks what is unchecked, and the repository's own rules say who reviews it (10.5). Durable execution (`workflow`, `runs at most once per`, `survives restarts`) is a platform service, not language (9.4).
- Source never names an operating system. A difference between hosts is a fact about a capability, declared in its interface; this replaces clauses scoped `on <platform>` (10.3).
- Source text describes only the program's instructions and what is checked about them. Removed for that reason: cost bounds (`cost at most`); prose, whether a component's `about` section, a `because` reason, an informal quoted clause or an `assumes` fact (explanations go in `note` lines); a `corrects` line, which described an earlier version; assurance tiers and `proof` blocks (the compiler proves what it can and reports how each clause was held; `assert` and `trust` are body statements); version numbers and schema compatibility modes, which belong in the package file; and a system's `components`, `authority` and `wiring`, which repeated what the entry point does in code (10.1, 10.5, 10.6, 11.1, 11.3, 11.5).
- Every contract feature must be enforceable, and the language has no concept of time. Removed for that reason: `idempotent by`, which nothing can enforce; `deadline`, time comparisons (`is after`, `is before`), date literals and built-in time types, which are standard-library operations on a clock capability; `may block`, which repeated what `uses` shows; `varies by host`, since a host-dependent fact is an ordinary operation of a capability's interface; and system files, whose questions `scribe query` answers from the code (3.6, 9.1, 10.1, 10.3, 11.5).
- If in doubt, make it a library, as Rust does. Moved out of the language for that reason: capability narrowing (`attenuated`, and the `files`, `network`, `host`, `port`, `under`, `read only` descriptions) is standard-library operations; labels and `declassifies` became distinct types and named conversions; `define` predicates became operations that produce a `Bool`; `implies` went, since aligned `if` / `then` lines say it; pipelines, protocols and schemas with migrations became library code; clauses scoped `in build` went; a component's `may` section went, since all authority starts at the entry point; record members lost the word `field`; and a quantifier over a type is written `for all x as T` (1, 3.2, 5.2, 8, 9.3, 10, 11).
- A condition group may be the condition of an `if` or `repeat while`, with `then` or `do` on its own line between the two blocks (5.2).
- `enum` replaces `choice`, following Rust, Swift and Scala 3; no popular language uses `choice` (3.3).
- Every keyword is a contract word, an instruction (flow, storage, loops) or a word that makes a line read as English, and no two keywords look alike. Removed for that reason: typestate (`in state`; distinct types do it, as in Rust); `matching`, `positive` and `non_empty` (contracts and distinct types say these); the comparison words `less`, `least`, `more`, `most`, `than` (the school symbols `<`, `≤`, `>`, `≥` remain); `followed by` (a library `join`); the labels `onto`, `over` and `using`; `anything but` (name the failures instead); and `, or else`, which looked like `otherwise` and is now `, otherwise` (2.3, 3.5, 4.1, 7.3, 9.1).
- Typestate is part of the language: a type lists its `states`, and a state reads as an adjective, `an Open Connection`. `editable`, `consumed` and `maybe` became hard keywords so a state name is never mistaken for one (7.3).
- Look-alike keywords are replaced: the contract `fails with` (next to the statement `fail with`) is `can fail with`; `ensure if it succeeds` and `ensure if it fails` are `ensure on success` and `ensure on failure`; the quantifier `for every` (next to the loop `for each`) is `for all`; and the group `none of` (next to the value `nothing`) is `not any of` (5.2, 5.5, 6, 10.1).
- Scribe is not described as a systems language. Having no garbage collector is a choice about memory management, managed at compile time as in Rust, not about the kind of software Scribe is for (1, 7).

### B.4 Earlier documents

[Scribe: Syntax Philosophy and Worked Examples](https://claude.ai/code/artifact/ad546c4f-aff1-4425-93f2-386b7fc8ee29) holds the 50 worked examples and the full experiment designs. This specification supersedes it where they differ.
