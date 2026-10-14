# Scribe Language Specification

Oct 14, 2026 · @Julian Harris

**Draft 0.5.** This document is the whole specification as it stands. It replaces the syntax doc, the pilot results and the change replay as the place to read what Scribe is; those remain as evidence, summarised in Appendix B.

Every rule is normative unless it carries one of two marks:

- **Proposed**: recommended by the evidence but not yet agreed.
- **Open**: a real choice that has not been made. Section 14 lists every one, with what blocks it.

Code examples are shown in the **review view**, the default way people read Scribe (Section 2). Where the stored text differs, the difference is only the drawing of `≤`, `≥` and `≠`, which are stored as `<=`, `>=` and `!=`.

## 1. Purpose and principles

Scribe is a language written by agents and read by humans. No person is expected to write it: AI agents write it, with endless patience for complexity and rigour, and people read and review what they wrote. That division decides almost everything else.

It has three aims, in order:

1. **Quick, reliable impact assessment.** Most languages cannot say reliably what a proposed change affects, because so much is decided only at run time. Scribe removes that uncertainty: everything that links one piece of code to another (ownership, effects, capabilities, failures, states, contracts) is stated in the source, so standard static analysis finds every linkage, and the impact of a change is computed before it is merged, in time that grows with the change and what depends on it, not with the size of the codebase.
2. **As easy to read as possible.** The approach is plain English plus basic maths operators, drawing on similar languages and, above all, on what did and did not work across them over the decades. Every line has exactly one reading, and a reader who knows what the words mean sees it without knowing any rule of precedence beyond school arithmetic: text that can be read two ways is not readable, and needing a manual to read an expression is a failure of design. This holds for the stored text and for every view. When readability and impact assessment conflict, Scribe looks for a spelling that serves both; only when there is none does impact assessment win, and the cost to readability is recorded as a decision. Against every other aim, readability wins.
3. **Almost provably correct.** Because agents write the code, the language can demand validation no person would put up with writing: Rust-style ownership, states tracked in types, contracts on every function. No undefined behaviour, no missing-value errors, no data races, no leaked resources, no unhandled failures; beyond that, contracts are proven wherever the compiler can, and checked at run time where it cannot. The writer's effort is free; the reader's is not, so every check must still read plainly.

And two constraints:

- **Fast, predictable compilation**, closer to Zig than to Rust.
- **No garbage collector** and no hidden allocation. Memory is managed at compile time: the compiler checks ownership from each parameter's convention (Section 7), and nothing is reclaimed at run time except where an owner's block ends.

Version 1 compiles to WebAssembly.

Principles that follow from these and decide individual rules:

| Principle | Consequence |
| --- | --- |
| Rigour costs the writer nothing, reading costs the reader | Validation can be as demanding as correctness needs, because agents write it; every construct is judged by how it reads, not by how hard it is to write |
| What a reviewer must judge is visible before the body | Failures, capabilities, effects and contracts sit in each function's header |
| One stored form, many readings | The source has exactly one canonical text; people see it through views, and the formatter chooses the clearest phrasing |
| Words where words are clearer, symbols where symbols are | School-level maths symbols for arithmetic and comparison; words for logic and structure |
| A promise is code, not commentary | Contracts are checked against the body on every build. A contract holds only conditions the compiler can check; explanations go in `note` lines |
| Never overclaim | Every proof, verdict and report states the unchecked code (`trust` blocks) it rests on |
| Nothing implicit that matters | No ambient authority, no implicit conversion, no missing value without `or nothing`, no failure without a `can fail with` |
| If in doubt, make it a library | As in Rust, the language is kept minimal. A construct is language only if it cannot be an ordinary type or function without losing what the compiler must check; everything else (capability narrowing, time, pipelines, serialisation) is a library |
| Any expression wherever a value goes | No position in the grammar is limited to a name or a literal without a stated reason; types, not keywords, decide what fits. A value in a contract or a grant of authority must be a constant expression, because impact analysis compares it without running code |

## 2. Source text

### 2.1 One stored form, many readings

Three things are kept apart:

| Layer | What it is | Who chooses |
| --- | --- | --- |
| **Stored form** | The canonical text in the file. Plain ASCII, one spelling per construct. It is what is committed, hashed and diffed | The formatter, on every save |
| **Accepted forms** | Other spellings the parser takes: maths symbols such as `≤` and `∀`, and the C-family `&&`, `\|\|`, `!`. Each maps to exactly one construct | Fixed by the language version |
| **Views** | How a reader sees the text | Each reader |

A program's identity comes from its parsed tree, so a change of spelling never changes a hash, a contract comparison or an impact report.

### 2.2 Views

| View | For | What changes |
| --- | --- | --- |
| **Review** (default) | Reviewers | `<=`, `>=`, `!=` drawn as `≤`, `≥`, `≠`; underscores in names drawn at about half contrast; a `pure` tag on functions with no `uses` line |
| **Compact** | Experts | As review, plus logic words drawn as `∧ ∨ ¬ ∈ ∀ ∃` |
| **Localised** | Readers of other languages | Keywords replaced one for one; names, strings and notes untouched |

A view must round-trip: rendering stored text into a view and converting it back gives identical bytes. A view that fails this does not ship. Errors, documentation and search always quote the stored text, with the reader's view alongside.

### 2.3 Notation

The code reads like code in the languages most readers already know: calls `f(a, b)`, fields `a.b`, `let x = 1`, `==`. English is used where it carries meaning that symbols would hide: contracts (Section 10), condition groups (Section 5.2) and the logic words `and`, `or`, `not`. Symbols are used where school mathematics or almost every programming language has already taught them.

| Meaning | Stored | Also accepted | Review view |
| --- | --- | --- | --- |
| Arithmetic, remainder | `+ - * /`, `%` |  | same |
| Less than, more than | `<`, `>` |  | `<`, `>` |
| At most, at least | `<=`, `>=` | `≤`, `≥` | `≤`, `≥` |
| Equal, not equal | `==`, `!=` | `≠` | `==`, `≠` |
| Bind, assign | `=` |  | `=` |
| And, or, not | `and`, `or`, `not` | `∧`, `&&`, `∨`, `\|\|`, `¬`, `!` | words |
| Member of | `in`, `not in` | `∈` | `in` |
| For all, there is | `for all`, `there is` | `∀`, `∃` | words |

Rules:

- `==` is equality, as in almost every language. `=` only binds (`let x = 1`) or assigns (`total = total + price`), and only as a whole statement, never inside an expression.
- Comparisons chain as in mathematics: `0 ≤ low ≤ high ≤ count(items)`.
- `is` tests what kind of value something is: a variant (`status is Paid`) or a missing value (`found is nothing`); `is not` negates the test.
- `and` and `or` never mix on one line. A condition that combines them is written as a condition group (Section 5.2).
- The only precedence a reader needs is what school arithmetic teaches (Section 5.2).
- Symbols that exist only in programming (`!`, `&&`, `||`) are accepted on input, so machine authors are never rejected for spelling, and never displayed.

### 2.4 Names

- Names are single words joined by underscores: `total_charged_to`. The review view draws underscores at about half contrast, with a floor set by the theme for accessibility, so names read as separate words but remain one searchable token.
- No hyphens and no spaces inside names.
- Types, variants and states start with a capital letter (`Order`, `Paid`, `Open`); everything else is lower case. **Open:** whether to drop capitals entirely and rely on declarations (Section 14).

### 2.5 Keywords

Keywords are reserved everywhere and can never be names, except three that are keywords only in their own position (`of` after `all` or `any`, and `requires` and `exports` in a module). There are 61 in all (Appendix A). Each is a contract word, an instruction (flow, storage, loops), or a word that makes a line read plainly, and no two look alike.

A reserved word is judged by what it costs readers, not authors. Scribe is written by machines and read by people, so reserving a word that makes a line unambiguous is cheap. The porting guide (`docs/porting-guide.md`) lists the reserved words that are common names in other languages, with what to use instead.

### 2.6 Layout and comments

- Blocks are opened by a header line and four spaces of indentation. Tabs are a syntax error. There are no braces and no semicolons, and a header line does not end with a colon.
- A line starting with `note` is a comment.
- Every statement and every clause is one whole line, and nothing continues onto the next line. A view may wrap a long line on screen; that wrapping is never part of the text.
- A block indented under a line is part of that line's structure, for conditions as for statements: a condition group's items sit in the block under its header (Section 5.2).
- No assignment inside an expression.
- Arguments, parameters and list items are separated by commas. `and` is only logic.

## 3. Values and types

### 3.1 Built-in types

Integers (`Int`, `Nat`, and fixed widths), decimals, `Bool`, `Text`, `Bytes`, lists (`List[Int]`), maps (`Map[Text, Int]`), and units of measure. A type that takes other types names them in square brackets, as in Python and Scala; angle brackets would clash with "less than". There are no implicit conversions: a conversion is always a named function.

Bit functions are named functions on the fixed-width unsigned types, never symbols and never on `Int`: `bitwise_and(flags, mask)`, `bitwise_or`, `bitwise_xor`, `bitwise_not`, `shift_left(value, 1)`, `shift_right`, `rotate_left`, `rotate_right`, and `bit(flags, 3)`. A shift by the width or more is a contract violation, proven or checked like any other. Integers can be written in hex and binary, with `_` between digits: `0xFF`, `0xFFFF_FFFF`, `0b1010_0001`.

**Open:** the exact numeric tower, fixed-width names, whether `Text` is bytes or characters, and character and byte literals (Section 14).

### 3.2 Records

```
type OrderLine
    product: ProductId
    quantity: Int
    unit_price: Money
```

A record is built like a call with named arguments: `OrderLine(product: id, quantity: 2, unit_price: price)`.

A record may state invariants. They hold whenever control leaves a function that changes the record:

```
type Order
    lines: List[OrderLine]
    status: OrderStatus
    invariant count(lines) > 0 or status is Draft
```

A `model` line gives the abstract state that contracts talk about, so contracts never mention the representation:

```
type Queue[T]
    capacity: Int
    items: Ring[T]
    model contents = sequence_of(items)
    invariant count(contents) ≤ capacity
```

### 3.3 Enums

```
enum OrderStatus
    Draft
    Paid(paid_at: Instant)
    Shipped(tracking: Text)
    Cancelled(reason: Text)
```

An enum's variants may carry data, as in Rust and Swift, and a variant with data is built like a record: `Paid(paid_at: now)`. An enum is used through `match`, which must handle every variant:

```
function describe(status: OrderStatus) -> Text
    match status
        case Draft
            return "draft"
        case Paid(paid_at)
            return "paid"
        case Shipped(tracking)
            return tracking
        case Cancelled(reason)
            return reason
```

There is no catch-all case, so adding a variant makes every `match` that does not handle it fail to compile.

### 3.4 Missing values

There is no null. A value that may be absent has a type that says so in English: `User or nothing`. A `User` is already a valid `User or nothing`, so returning one needs no wrapper. Before the value is used, it is tested, and after the test the compiler knows which it is, as in Kotlin and TypeScript:

```
function describe(found: User or nothing) -> Text
    if found is nothing
        return "nobody"
    else
        return found.name
```

Using `found.name` before the test is a compile error.

### 3.5 Refinements

A refinement carries its condition wherever the type goes:

```
type Percent is Int in 0 through 100
type Index is Int in 0 until limit + 1
```

A condition that is not a range, such as "a well-formed email address", is a distinct type built by a checking function (8.3), or an `invariant` on a record.

### 3.6 Units

A number followed directly by a unit is a quantity: `5 kilometres`, `1 GiB`, `30 seconds`. Units are checked: adding a length to an area is an error, and a length divided by a time is a speed. Units are defined by libraries, not by the language.

The language has no concept of time: no time types, no date literals, no time comparisons and no time limits. Instants, calendar dates, time zones and timeouts are standard-library types and functions, and anything that reads the time does so through a clock capability (8.1). Values of those types are compared with the ordinary comparisons: `due < now`.

### 3.7 Ranges

`1 through 10` includes 10; `1 until 10` stops at 9: `for i in 0 until count(items)`. There is no other range form.

## 4. Functions and statements

### 4.1 Functions

A function is a header line, its contract, and its body:

```
type Account
    balance: Money
    is_open: Bool

type Receipt
    amount: Money

function withdraw(amount: Money, account: editable Account) -> Receipt
    can fail with InsufficientFunds
    expect amount > 0
    expect account.is_open
    ensure on success, account.balance == old(account.balance) - amount
    ensure on failure, account.balance == old(account.balance)
    changes only account.balance

    if amount > account.balance
        fail with InsufficientFunds
    account.balance = account.balance - amount
    return Receipt(amount: amount)
```

The header names the function, its parameters with their types, and its result type after `->`, as in Rust and Swift. A parameter's type may start with its convention, `editable` or `consumed` (Section 7.1). A function that returns nothing has no `->`.

A call is the function's name and its arguments in brackets: `withdraw(50, savings)`. An argument may be named where that reads better, as in Swift and Kotlin: `move(amount, from: source, to: target)`.

The lines between the header and the body are the function's **contract** (Section 10). A function with no `uses` line uses no capability and is pure; the review view shows a `pure` tag.

### 4.2 Statements

Every statement starts with a keyword, or is an assignment or a call.

| Statement | Form |
| --- | --- |
| Fixed binding | `let rate = 0.2` |
| Variable | `var count = 0` |
| Assignment | `count = count + 1`, `item.price = 10`, `heap[i] = x` |
| Return | `return total`, or `return` alone in a function with no result |
| Fail | `fail with OutOfRange(value: n)` |
| Pass a failure up | `try parse_port(text)`, also inside expressions: `let n = try parse_int(text)` |
| Use a fallback when a call fails | `let alerts = try fetch_alerts(user) else nothing` |
| Handle failures | `handle` block, then `on failure Name` blocks (Section 6) |
| Branch | `if` / `else if` / `else` |
| Match | `match value` with `case Variant(field)` blocks |
| Count | `for i in 0 until count(items)` |
| Iterate | `for order in orders` |
| Loop | `while condition`, with `invariant` and `decreases` lines |
| Leave or continue a loop | `break`, `continue` |
| Do nothing | `do nothing` |
| Hold a resource | `hold input = open(path, files)` (Section 7) |
| Concurrency | `together` (Section 9) |

`=` binds or assigns, and only as a whole statement. Equality is `==`.

`do nothing` is for a `case` with nothing to do. `match` has no catch-all case, so a new variant is never silently absorbed: every `match` must say what happens to it, even if that is nothing.

### 4.3 Functions as values

Scribe has no anonymous functions and no closures that outlive the call they are created in. A block passed to a statement (`together`, `handle`, `hold`) runs before the statement ends. Where other languages store a callback, Scribe uses a named type holding the data, or an interface (Section 11.2).

**Proposed:** function types, so a function can be passed as a parameter, with its effects and failures carried in the type, for example `key: function(Order) -> Money`. This is the most common gap found in real code (Appendix B).

## 5. Expressions

### 5.1 Calls, fields and items

- A call is a name followed by its arguments in brackets: `count(items)`, `gcd(12, 18)`.
- A named argument is written `name: value`: `move(amount, from: source, to: target)`. Named arguments follow the positional ones.
- A field is read with a dot: `account.balance`, `order.customer.name`.
- An item of a list or map is read with square brackets: `items[0]`, `prices[product]`.
- These chain from left to right, as in every language that has them: `orders[0].lines[2].price`.

### 5.2 Precedence and grouping

Every line has exactly one reading (Section 1), and the only precedence a reader needs is what school arithmetic teaches:

| From tightest to loosest | Example | Why a reader already knows it |
| --- | --- | --- |
| Calls, fields, items | `cart.total + tax` is `(cart.total) + tax` | Like `sin(x) + 1` |
| `-`, `try` before a term | `-a + b`, `(try parse_int(text)) + 1` | A minus sign applies to the next term |
| `*`, `/`, `%`, then `+`, `-` | `a + b * c` | School arithmetic |
| Comparisons, which chain | `0 ≤ i < n` | School arithmetic |
| A chain of `and`, or a chain of `or` | `x > 0 and y > 0` | Plain English |

Brackets group arithmetic only: `(a + b) * c`. They never group logic.

`and` and `or` never mix on one line, and `not` negates a whole condition, never one link of a chain: `not paid and shipped` is an error. One condition implying another is written across two aligned lines as `if` and `then`:

```
ensure
    if for all x in items, x > 0
    then count(items) < limit
```

Anything else is grouped by layout. A **condition group** is `all of`, `any of` or `not any of` followed by its conditions, one per line in the block beneath, or several on one line separated by commas. Groups nest; a line inside a group holds one condition, with no `and` or `or`, so the group's header is the only thing joining its lines:

```
return all of
    order.status in [Paid, Shipped]
    customer.is_active
    not any of
        order.is_disputed
        customer.is_suspended
    any of
        order.days_since_payment ≤ 30
        all of customer.is_premium, order.days_since_payment ≤ 90
```

On one line, an item holds no quantifier, since a quantifier covers the rest of its line and would take in the items after it; such an item goes on its own line. A comma at the end of a line in a group is optional. If one is written, another item of the same group must follow it, so a comma that disagrees with the indentation is an error. The formatter leaves these commas out of the stored text; the review view draws them, and draws a labelled rule beside each group. A group can follow `expect`, `ensure`, `invariant`, `let … =` and `return`.

A group can also be the condition of an `if` or a `while`. Then `then` (for `if`) or `do` (for `while`) stands on its own line between the condition's block and the statement's, so the two blocks are never confused:

```
if all of
    account.is_open
    account.balance ≥ amount
then
    account.balance = account.balance - amount
else
    do nothing

while all of
    account.balance < 0
    account.retries > 0
do
    account.retries = account.retries - 1
```

An `if` / `then` pair is not a statement's condition, because its `then` would read as the statement's; name it with `let` first.

### 5.3 Constructing records

A record or an enum variant with data is built like a call, with its fields named:

```
let origin = Point(x: 0, y: 0)
let order = Order(lines: cart.lines, status: Paid(paid_at: now))
return Dashboard(profile: profile, orders: orders, alerts: alerts)
```

### 5.4 Collections

A list is written in square brackets: `[1, 2, 3]`. One comprehension form makes a list from another, in code and in contracts, as in Python:

```
let paid = [order.total for order in orders where order.status is Paid]
let total_paid = sum(paid)
```

There is no `map`, `filter` or `reduce`; a comprehension says the same in one readable line.

### 5.5 Quantifiers and conditions

```
for all x in items, x ≤ result
for all j in 0 until low, items[j] < target
there is item in items where item.price > limit
```

A quantifier covers the rest of its line. This is the same rule a statement header follows, and the only one: there is no other way for a scope to end. Any other grouping is written with a block (Section 5.2), so a quantifier is never one operand among others on a line.

### 5.6 Strings

Text in double quotes, with `{expression}` interpolation: `"Your order {order.id} is confirmed."`.

### 5.7 What `is` means

`x is y` tests what kind of value `x` is: a variant (`status is Paid`) or a missing value (`found is nothing`). `x == y` compares values. `is` is the one place where meaning needs name resolution, which happens after parsing.

## 6. Failures

A function lists every way it can fail. The compiler checks the list is complete: a body cannot fail in an undeclared way, and a function that handles all its callees' failures has no `can fail with` line.

```
enum PortFailure
    NotANumber(text: Text)
    OutOfRange(value: Int)

type Port
    number: Int

function parse_port(text: Text) -> Port
    can fail with NotANumber, OutOfRange

    let number = try parse_int(text)
    if number < 1 or number > 65535
        fail with OutOfRange(value: number)
    return Port(number: number)

function port_or_default(text: Text) -> Port
    handle
        return try parse_port(text)
    on failure NotANumber or OutOfRange
        return Port(number: 8080)
```

Rules:

- `try` passes a failure up to the caller, or to the enclosing `handle` block, as `?` does in Rust and `try` in Swift.
- `let x = try f(a) else fallback` uses the fallback when the call fails, for a failure that is acceptable.
- Each `on failure` names the failures it handles and must do something: return a value, fail, or call something. There is no catch-all that swallows failures.
- `on failure A or B as problem` binds the failure so it can be passed on: `fail with problem`.
- Adding a way to fail is a change to the contract (Section 12).

A failure is a value. Failures are declared as variants of an `enum`, so a failure can carry fields and two failures with the same name can be told apart, and `fail with` takes any expression whose value is a failure: `fail with OutOfRange(value: clamped)`. Failures that carried no data were the most common gap in translated real code. **Open:** the type rules for failure values (Section 14).

**Open:** whether panics exist at all. The current rule is that a function cannot panic, and the compiler proves it; what a bounds check it cannot prove does is undecided.

## 7. Memory, ownership and resources

### 7.1 Values, not references

A variable holds its value, not a pointer to it. Each value has exactly one **owner**, the variable or field that holds it; when the owner goes out of scope the value is freed. There are no references to write and no lifetime annotations. Safety comes from how each parameter may be used, stated in the contract:

| Convention | Written | Rust | The function may | The caller afterwards |
| --- | --- | --- | --- | --- |
| Read (default) | `items: List[Int]` | `&T` | Look, not change | Still owns it, unchanged |
| Editable | `account: editable Account` | `&mut T` | Change it; nobody else can see it meanwhile | Still owns it, possibly changed |
| Consumed | `connection: consumed Connection` | `T` (move) | Take ownership | Cannot use it again |

A read or editable parameter cannot be kept after the function returns. This is a type rule, not a convention: a whole class of bugs (keeping a buffer someone else will reuse) cannot be written.

### 7.2 Allocation

There is no garbage collector and no hidden allocation. Memory is released when its owner's block ends.

**Proposed** (replacing explicit allocator parameters on every call):

- A function that may request memory whose size depends on its input says so in its contract: `allocates`. Fixed-size values, locals and in-place edits never need it, and neither does stack use.
- Call sites do not repeat it; the allocator comes from the caller's context.
- A block chooses the allocator for everything inside it, and frees it all at the end: `within an arena`.
- Bounded collections need no allocator.
- Running out of memory is an ordinary failure, `OutOfMemory`.

Recursive functions get a stack-depth line in reports ("stack depth unbounded") unless a `decreases` measure and a size bound limit it.

### 7.3 Resources

Files, sockets and locks are linear: they must be released exactly once. `hold` releases on every exit from its block, including failures:

```
hold input = open(path, files)
    return try read_line(input)
```

A linear value bound with `let` and not released on some path is a compile error naming that path.

A type can list the states its values move through, and a state is written as an adjective before the type. Using a value in the wrong state is a compile error, not a run-time check, and every change of state is visible in a signature, where impact analysis compares it:

```
type Connection
    socket: Socket
    states Closed, Open

function send(data: Bytes, connection: editable Open Connection)
    transmit(data, connection.socket)

function close(connection: consumed Open Connection) -> Closed Connection
    return release(connection)
```

Nothing can send to a `Closed Connection`, and after `close(connection)` the old variable cannot be used at all, because `close` consumed it. A conversation between two parties (a protocol) is written the same way: each point in the conversation is a state of the channel, and each message a function that consumes the channel in one state and returns it in the next.

### 7.4 Shared state

**Open:** how to write data that several owners share or that forms cycles, such as graphs and doubly linked lists. The candidates are explicit counted references and handles into an arena; either way the readers and writers are declared in contracts. Real code needs this (Appendix B), so the decision cannot be deferred long.

## 8. Effects, capabilities and authority

### 8.1 No ambient authority

Nothing can open a file, a socket or a clock unless it has been handed a **capability** for it. All authority starts in one place, the program's entry point, and can only be narrowed as it is passed on:

```
function main(root: Authority)
    uses root

    let reports = read_only_directory(root, "/reports")
    let partner = network_endpoint(root, "export.partner.example", 443)
    export_reports(reports, partner)
```

Narrowing a capability is an ordinary standard-library function, not syntax; its contract says what the result can reach. Because authority only flows along capabilities, and the arguments that narrow it are constants, "what can this module reach?" has a static answer, with the path as evidence:

```
$ scribe query reach export_reports network
export_reports can reach: host "export.partner.example" port 443 only
    path: main > network_endpoint > partner > export_reports
```

### 8.2 Effects

A function's effects are what it may do beyond computing its result. They are declared in its contract and checked against the body:

| Effect | Declared as |
| --- | --- |
| Use a capability (files, network, clock, database, another module) | `uses files, clock` |
| Request memory (**Proposed**) | `allocates` |

A function with none of these is pure, and calling an effectful function from it is a compile error naming the call. Clocks are capabilities, so code that depends on the time says so, and tests pass a fake clock.

### 8.3 Information flow

Information flow is controlled with distinct types, as in Rust's newtype pattern; there are no labels. Text from outside is an `UntrustedText`, and a database query takes a `UserName`, so untrusted text reaches a query only through a checking function that produces one: `let checked = try check_user_name(name)`. Secret data is a distinct type such as `CardNumber`, and it becomes public only through a named conversion function, which every review and impact report shows wherever it is called.

## 9. Concurrency

Concurrent structure is written as a visible shape, and no task outlives the block that started it.

### 9.1 Doing several things at once

```
function load_dashboard(user: UserId, services: DashboardServices) -> Dashboard
    can fail with DashboardUnavailable
    uses services

    together
        let profile = try fetch_profile(user, services)
        let orders = try fetch_orders(user, services)
        let alerts = try fetch_alerts(user, services) else nothing
    return Dashboard(profile: profile, orders: orders, alerts: alerts)
```

Everything inside `together` starts at once, and the block ends when every line has finished. If a required line fails, the others are cancelled and the failure goes up; `else` marks a line whose failure is acceptable, with the value to use instead. Cancellation unwinds through `hold`, so resources are still released. A time limit is not part of the language: it is a standard-library function on a clock capability. So is a pipeline: stages joined by bounded queues are built from `together` and a channel type in the standard library.

### 9.2 No shared mutation

Two branches of a `together` block cannot change the same variable; the compiler names both lines. Each branch produces its own result instead. With no shared mutable values there is nothing for a lock to protect, and no data race can be written.

### 9.3 Pipelines (removed)

Removed from the language. A pipeline is a standard-library function built on `together` and a channel type (9.5).

### 9.4 Work that survives a restart

Removed from the language. Resuming work after a restart, running it at most once per key and remembering its progress are services of the platform a program runs on, not part of what the program means. Where a program needs them, they come from a library or a capability, used through ordinary functions.

### 9.5 Waiting without `async`

**Proposed:** there is no `async` keyword. A function that can wait takes a capability for it, which shows in its `uses` line; callers write an ordinary call. The capability decides whether waiting blocks a thread or yields to an event loop, so the same code runs either way, and every place a task may suspend is visible from the contracts.

**Open:** a channel type for passing results between tasks, and how event-driven servers with an accept loop are written.

## 10. Contracts

A **contract** is what a function requires and promises. It is the lines between the function's header line and its body, and it is code: the compiler holds the body to it on every build. A contract that no longer matches its code is a compile error, so contracts cannot drift.

### 10.1 Clauses

| Clause | Says | Checked by |
| --- | --- | --- |
| `name: T` and `-> T` in the header | Each parameter, its type and convention, and the result | Types |
| `can fail with A, B` | Every way it can fail | Compiler, completely |
| `uses`, `allocates` | Its effects and capabilities. Waiting needs a capability too, so it shows in `uses` (9.5) | Compiler, completely |
| `expect condition` | What callers must ensure | Proof at call sites, or run-time check |
| `ensure condition` | What it guarantees. Forms: `ensure on success, ...`, `ensure on failure, ...`, `ensure on failure A, ...`, `ensure if x > 0, ...` | Proof where the checker can; otherwise a run-time check, and reports say which (10.5) |
| `changes only x, y` | Everything else is untouched | Compiler |
| `example f(x) == value` | An exact result for an exact input | Run on every build |
| `property for all x: T, condition` | A rule over every input | Generated tests |

Every clause is a condition or a declaration the compiler checks. A quoted sentence is Text, not a condition, so it cannot be a clause; an explanation goes in a `note` line. Inside an `ensure`, `old(x)` is the value of `x` when the function began, as in Eiffel and Dafny, and `result` is the value it returns.

### 10.2 Examples

Examples are part of the contract, not separate tests. They are the cheapest way to pin exact behaviour, and the change replay found exact behaviour was what caught most real breakages.

```
function redirect_body(location: Text) -> Text
    example redirect_body("/a") == "<a href=\"/a\">See Other</a>.\n\n"
    return "<a href=\"{location}\">See Other</a>.\n\n"
```

Properties generalise examples and run as generated tests: `property for all items: List[Int], count(items) == 0 or largest(items) in items`.

### 10.3 Host-dependent behaviour

Source never names an operating system or a host. Everything that can behave differently from one host to another reaches code through a capability (Section 8.1), so a difference between hosts is a fact about a capability, and it is stated where the capability is declared.

A fact that varies by host needs no syntax of its own: it is a function of the capability's interface, and a contract calls it like any other:

```
interface Files
    function replaces_existing(files: Self) -> Bool

    function rename(source: Path, target: Path, files: editable Self)
        can fail with NotFound, TargetExists
        ensure on failure TargetExists, not replaces_existing(files)
```

A function whose contract tests a host-dependent fact holds wherever the fact has the value it tests for; a contract that tests none must hold on every host. Each host binding, the implementation of a capability's interface for one host, gives every fact its value. The toolchain supplies the host bindings, and they are the only place a particular operating system is named.

Contracts are compared once for each host binding (Section 12).

### 10.4 Loops, types and predicates

- Loops carry `invariant` lines, and `while` loops a `decreases` line naming a quantity that shrinks on every pass and cannot go below zero. A `for` loop over a range or a list always terminates.
- Types carry `invariant` and `model` lines (Section 3.2).
- A predicate is an ordinary function that returns a `Bool`: `function is_sorted(items: List[Int]) -> Bool`, then `expect is_sorted(items)`.

```
function sum_to(n: Int) -> Int
    expect n >= 0
    var total = 0
    var i = 0
    while i < n
        invariant total == i * (i - 1) / 2
        decreases n - i
        total = total + i
        i = i + 1
    return total
```

### 10.5 How clauses are checked

The source does not say how hard to check. On every build the compiler proves that there is no undefined behaviour, no unhandled failure and no run-time error, and checks types, effects, capabilities, units and states. It then proves each `ensure`, `expect` and invariant where it can; a clause it cannot prove is checked at run time instead. `example` and `property` lines run as tests. Every report says, for each clause, which of these held it.

A condition in a contract or an `assert` may call only pure functions, ones with no `uses` line, as in Dafny and SPARK. Checking a condition must never change the program, and the prover can only reason about calls that cannot.

Two statements belong to the body:

```
function sorted_copy(items: List[Int], native: NativeSort) -> List[Int]
    ensure is_sorted(result)
    uses native
    let copy = sort(items, native)
    assert count(copy) == count(items)
    trust
        assert is_sorted(copy)
    return copy
```

- `assert condition` is a condition the compiler must prove at that point, and may then use. Where it cannot be proven, it is checked at run time.
- `trust` opens a block the checker does not verify, and an `assert` inside it is taken as true. Trust blocks are counted per package, and every review and report that depends on one names it with its file and line. Who reviews them is a matter for the repository's own rules, not the language.

### 10.6 Corrections (removed)

Removed from the language. A `corrects` line described an earlier version of the program, not this one. Whether a changed clause fixes a false promise is said in the change's review, where impact analysis lists the dependents that relied on it (12.3).

### 10.7 Writing complete contracts

Impact analysis is only as good as contracts are complete (Appendix B), so the toolchain treats completeness as a measured property:

- `scribe draft` proposes a first contract from the code: failures, effects, capabilities, and examples recorded from tests. A person reviews and edits it.
- **Contract coverage** reports, for each function, the observable outputs that no clause or example constrains: results, failure kinds and effects. It is shown in reviews as a figure per module.
- A function that returns a result but has no `ensure` or `example` gets a weak-contract warning.

## 11. Modules and interfaces

### 11.1 Modules

A **module** groups functions and types, and declares its static relationships: which modules it requires, which of their functions it calls, and which of its own functions it exports. These relationships are the graph impact analysis walks; nothing in them happens at run time. The declaration is written at the top of the module's source, as Java declares modules:

```
module Checkout

    requires
        Payments as payments
            uses charge, refund, total_charged_to
        Inventory as inventory
            uses reserve, release

    exports
        function place_order(cart: Cart, customer: CustomerId, request_id: RequestId) -> Order
            can fail with EmptyCart, OutOfStock, PaymentDeclined, RefundFailed
            expect count(cart.lines) > 0
            ensure on success, result.status is Paid
            uses payments, inventory

```

| Section | Says |
| --- | --- |
| `requires` | Each module it depends on, and which of its functions it uses. Impact is judged against exactly these |
| `exports` | The functions other modules may call, with their contracts |

Version numbers, of the module and of what it requires, are not part of the language: they are kept in the module's package file. The toolchain computes the next version from the change to the module declaration (Section 12), and a person may raise it but not lower it.

### 11.2 Interfaces

An interface states a contract that every implementation must meet and every caller may rely on:

```
interface Reader
    function read(buffer: editable Bytes, reader: editable Self) -> Int
        can fail with EndOfData, ReadFailed
        ensure result ≤ count(buffer)
```

Each implementation is checked against the interface's contract. A change to an interface's contract affects every implementation and every caller.

### 11.3 Stored data (schemas removed)

Removed from the language. A stored or transmitted format is an ordinary type, and reading an older version is an ordinary function with a contract:

```
function upgrade(previous: OrderV3) -> OrderV4
    ensure result.gift_message is nothing
    ensure result.lines == previous.lines
    return OrderV4(lines: previous.lines, gift_message: nothing)
```

Serialisation and version tags are a standard-library matter. Impact analysis compares these types and functions like any others.

### 11.4 Protocols (removed)

Removed from the language. A protocol is written with typestate (7.3): each point in the conversation is a state of the channel, and each message a function from one state to the next, as Rust's session-type crates do.

### 11.5 Systems (removed)

Removed from the language. A program is assembled in code: its entry point hands each module its capabilities and calls it (8.1), and each module's `requires` say what it calls. Whole-program questions, such as what can reach a host or where a sensitive type can flow, are answered from that code by `scribe query`; a project that wants one checked on every change runs the query in its own build.

## 12. Impact analysis

Impact analysis answers one question before a change is merged: **what could this change break, for whom, and how sure are we?** It works from contracts, not from the code's text, so it can answer for callers it has never seen.

### 12.1 What is compared

`scribe impact` compares two versions of a module: its declaration, every contract in it, its effects and authority, and whether its implementation still meets its contracts. It then checks the result against:

1. **every known dependent**, using the clauses that dependent's `requires` entry relies on (written or inferred from its code);
2. **any possible caller**, using the public contract alone, because most users of a library are outside its repository.

Where a contract tests a host-dependent fact (Section 10.3), the comparison is made separately for each host binding, and a verdict names the bindings it applies to. A guarantee that weakens only where file names ignore case is reported as breaking there, not hidden in an average.

### 12.2 Verdicts

Each provided function and each dependent gets exactly one verdict:

| Verdict | Meaning | Must carry |
| --- | --- | --- |
| Compatible | The new contract refines the old: preconditions no stronger, guarantees no weaker, no new failure or effect | A proof, or the solver's reason |
| Breaking | Some caller can observe a difference the old contract promised would not happen | A witness: a concrete input or scenario |
| Needs review | The tool cannot decide: a guarantee changed in a way that may or may not matter, or a `trust` block changed | The clause and the reason |
| Authority change | The module now holds or uses authority it did not before | The new capability and the path to it |
| Unchanged, checked | The contract did not change, the new implementation was checked against it, and the contract covers the code that changed | How each clause was checked, and the coverage |
| Unchanged, unchecked | The contract did not change, but the changed code does something the contract does not speak to, or was not checked | The uncovered lines or behaviours |

"Unchanged, unchecked" is the tool being honest about its blind spot. A green result is only ever as good as the contract behind it, so the report never says a change has no impact when the contract does not cover what changed.

### 12.3 Rules

| Change to a contract | Verdict |
| --- | --- |
| Weaker precondition; stronger guarantee; new function | Compatible |
| Stronger precondition | Breaking for any caller that may not meet it, with that caller's input as witness |
| Weaker or removed guarantee | Breaking for dependents that rely on it; needs review for unknown callers |
| New way to fail | Breaking for callers that handle every failure by name; needs review otherwise |
| New or changed `trust` block | Needs review |
| New capability or effect for the module | Authority change |
| Removed or renamed function | Breaking for every caller the same change leaves untouched; compatible for callers the change updates |

Two points of detail:

- **Effects are compared per module.** Moving an existing effect from one function to another inside the same module is not an authority change. It is reported against the functions concerned under the rules above.
- **A removed guarantee is reported, even when it was never true.** A change that drops a clause the old code did not meet is still a weaker guarantee: dependents that relied on it are listed, because their code may have depended on the false promise in practice. Saying that the old clause was false belongs in the change's review, not in the source.

### 12.4 Contract coverage

The tool knows which lines of an implementation each contract clause constrains: by proof, by the checker's dependency tracking, or by the tests generated from `example` and `property` lines. Code that no clause constrains is **uncovered**. A change that touches only covered code can be reported as "Unchanged, checked"; a change that touches uncovered code is reported as "Unchanged, unchecked" with the uncovered behaviour named, for example a new retry loop, a changed timeout, or a new order of side effects.

`scribe coverage` reports coverage for a module, and a project may set a floor below which merges are refused.

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

Removed from the language. Ordering a rollout so that no module runs alongside a dependency it breaks against is a deployment tool, which can be built on the impact comparison of Section 12; it is not part of what a program means.

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
| `scribe draft` | Proposes a contract for a function from its body and tests. The proposal is marked as a draft until a person accepts it, and never counts as checked on its own |
| `scribe query reach` | Answers "what can reach this capability, value or module, and by which path?" |
| `scribe review` | Shows a change in the review view: contract changes first, then the impact report, then the code |
| `scribe view` | Renders source in a chosen view (review, compact, localized) and converts it back |

The compiler is one program with no build scripts that run arbitrary code. A module compiles from its source and its dependencies' declarations alone, so modules compile in parallel and a change to an implementation never recompiles its callers.

### 13.2 Diagnostics

Every diagnostic:

- says what is wrong in one sentence of plain English, in sentence case;
- quotes the stored text and points at it;
- names the rule that was broken, with a link to this specification;
- gives a fix when there is exactly one sensible fix, and none when there are several.

```
withdraw.scribe:14: this changes account.deposits, which the contract does not allow.
    14 │     account.deposits = account.deposits + amount
       │     ^^^^^^^^^^^^^^^^
    The contract says: changes only account.balance
    Either add account.deposits to the changes only line, or remove this change.
    Rule: 10.1 changes only
```

`scribe check --explain` shows the tree the parser built for any line.

## 14. Open decisions

These are the decisions this draft does not settle. Each names what blocks it. Anything marked Proposed in the sections above is listed here.

| # | Decision | Status | Blocked by |
| --- | --- | --- | --- |
| 1 | Shared mutable state between tasks: forbid entirely, or allow through a single checked construct | Open | A worked corpus of real concurrent servers to test against (Section 9) |
| 2 | No `async` keyword: waiting takes a capability, which shows in `uses`, and the runtime schedules | Proposed | A prototype scheduler and a latency measurement |
| 3 | `allocates` as a contract clause, with the allocator passed as context | Proposed | Measuring how many contracts it adds lines to |
| 4 | ~~A rule language for system files~~ | Removed | System files are removed; `scribe query` answers reach and flow questions from the code (8.1) |
| 5 | Possessive paths: `account's balance` | Settled | Allowed, never mixed with `of` in one path (5.1) |
| 6 | Reduce the keyword set (now 58 keywords and 3 positional ones) | Open | Experiment 1, the comprehension test, extended to compare candidate words for the same idea, for example `hold` against `using` |
| 7 | Parser strategy | Settled | Written by hand, by recursive descent, as Rust, Go, Swift and TypeScript are, for error messages in the reader's terms; checked tree for tree against the reference grammar on every build |
| 8 | Code generator for WebAssembly, the version 1 target: a back end of our own, or one built on LLVM or Cranelift | Open | A compile-time benchmark on the pilot corpus |
| 9 | Solver for proving contracts | Open | Choosing a solver that is fast and gives stable results across versions |
| 10 | Localized views beyond English | Open | Round-trip proof for at least one other language |
| 11 | Which host bindings impact analysis compares by default (10.3) | Open | Re-expressing the replay's platform-specific breakages with it |
| 12 | Type rules for failures as values: how a failure `enum` relates to `can fail with`, and how impact analysis compares a failure's fields | Open | Roadmap phase 5, with the translation wave that needs them |
| 13 | ~~Cost bounds (`cost at most`)~~ | Removed | A cost bound is not an instruction the program carries out; speed is measured by tools outside the language |
| 14 | A condition group as the condition of an `if` or `while` | Settled | `then` or `do` on its own line between the two blocks (5.2) |

## Appendix A: grammar notes

The reference grammar is `grammar/scribe.lark` (version 0.4) in this project; `scribe-grammar-v0.2.lark` is the version the pilot measured. It is an Earley grammar: it states the language precisely but is not yet a production parser. `tools/check_grammar.py` tests it against a file per construct in this specification, a file per forbidden form, and this specification's code blocks. The pilot corpora in `research/corpus` use the pilot's English-sentence syntax, which this draft replaced, and are kept as a record.

**Vocabulary.** 61 reserved words: 58 keywords, which can never be names, and 3 that are keywords only in their own position (`of` in a condition group, `requires` and `exports` in a module). A comma that ends a line is read with the line's end. Nothing else is joined or rewritten by the lexer.

**No special rules.** The grammar is unambiguous by construction. Converted to Bison it has no LALR(1) conflicts, which proves that no text has two readings, and `tools/check_lalr.py` fails the build if one appears. `tools/check_grammar.py` also parses every test and every code block in this specification with no disambiguation applied. Calls are written with brackets, so which arguments belong to which call is always visible; mixing `and` with `or` is a condition group (Section 5.2); and a quantifier covers the rest of its line (Section 5.5).

**Parser class.** LALR(1), with no conflicts (version 0.2 had 172 shift/reduce and 20 reduce/reduce).

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
- Typestate is part of the language: a type lists its `states`, and a state reads as an adjective, `an Open Connection`. `editable` and `consumed` became hard keywords so a state name is never mistaken for one (7.3).
- A value that may be absent is `a User or nothing`, with no `maybe` and no `some`: a `User` is already a valid `User or nothing`, and testing `is nothing` tells the compiler which it is (3.4). `match` replaces `choose`, as in Rust, Scala and Python (3.3). A module declares what it `requires` and `exports`, as Java modules do, replacing `component`, `needs` and `provides` (11.1). Conditions in contracts may call only pure operations (10.5).
- Look-alike keywords are replaced: the contract `fails with` (next to the statement `fail with`) is `can fail with`; `ensure if it succeeds` and `ensure if it fails` are `ensure on success` and `ensure on failure`; the quantifier `for every` (next to the loop `for each`) is `for all`; and the group `none of` (next to the value `nothing`) is `not any of` (5.2, 5.5, 6, 10.1).
- Draft 0.5: the code reads like code. The English-sentence syntax of earlier drafts (label words between arguments, `of`-paths, `given … is a …` lines, articles, `let x be`) followed AppleScript, HyperTalk, Inform 7 and COBOL, and the decades of evidence on those languages is that English-like code reads well a line at a time and badly at scale. Calls are `f(a, b)` with named arguments where they help, fields are `a.b`, parameters are typed in the header with `:` (`function withdraw(amount: Money, account: editable Account) -> Receipt`), `==` is equality and `=` binds or assigns, `old(x)` replaces `x at entry`, and control flow uses the familiar words `if`/`else`, `for … in`, `while`, `break`, `continue`, `match`/`case`. English stays where it carries meaning: contracts, condition groups and the logic words. Reserved words fall from 80 to 61 (2–6).
- Scribe is not described as a systems language. Having no garbage collector is a choice about memory management, managed at compile time as in Rust, not about the kind of software Scribe is for (1, 7).

### B.4 Earlier documents

[Scribe: Syntax Philosophy and Worked Examples](https://claude.ai/code/artifact/ad546c4f-aff1-4425-93f2-386b7fc8ee29) holds the 50 worked examples and the full experiment designs. This specification supersedes it where they differ.
