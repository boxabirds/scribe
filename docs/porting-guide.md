# Porting to Scribe

A guide for translating code from other languages into Scribe. It grows as
porting finds more to say; for now it covers functions, conditions, missing
values, failures, bit operations and names.

Most of Scribe reads like the languages you know: calls `f(a, b)`, fields
`a.b`, `let x = 1`, `==`. What is different is what Scribe makes you state:
every parameter's convention, every way a function can fail, every capability
it uses, and its contract.

## Functions

| Source | Scribe |
| --- | --- |
| Rust `fn gcd(a: u64, b: u64) -> u64` | `function gcd(a: Nat, b: Nat) -> Nat` |
| Rust `fn push(v: &mut Vec<i32>, x: i32)` | `function push(v: editable List[Int], x: Int)` |
| Rust `fn close(c: Conn)` (takes ownership) | `function close(c: consumed Connection)` |
| Python `def f(x: int) -> int | None` | `function f(x: Int) -> Int or nothing` |
| Swift `move(amount, from: a, to: b)` | `move(amount, from: a, to: b)` |
| Any `x = y` where `x` exists | `x = y` |
| Any `const x = 1` / `let x = 1` | `let x = 1` |
| Any `let mut x = 0` / `var x = 0` | `var x = 0` |
| Any `a == b`, `a != b` | `a == b`, `a != b` |
| Any `a % b` | `a % b` |
| Any `List<Int>`, `Vec<i32>` | `List[Int]` |

A read parameter (no convention) is Rust's `&T`, `editable` is `&mut T`, and
`consumed` is a move (specification, Section 7.1).

## Conditions

Every line of Scribe has exactly one reading, so a condition never relies on
precedence between `and`, `or` and `not`. A chain of only `and`, or only `or`,
fits on one line. Anything that mixes them becomes a condition group:
`all of`, `any of` or `not any of`, with one condition per line beneath it.

| Source | Scribe |
| --- | --- |
| C `paid && shipped` | `paid and shipped` |
| C `a && (b \|\| c)` | `all of` / `a` / `any of b, c` on the lines below, as shown next |
| C `!(a \|\| b)` | `not any of a, b` |
| C `!a && b` | `all of` / `not a` / `b`: `not` never sits inside a chain |
| C `x > 0 ? y : z` | `if` statements; a condition is never a value chooser |
| Python `all(x > 0 for x in xs)` | `for all x in xs, x > 0` |
| Python `any(x.late for x in xs)` | `there is x in xs where x.late` |

```
return all of
    order.status in [Paid, Shipped]
    not any of order.is_disputed, customer.is_suspended
    any of
        order.days_since_payment ≤ 30
        all of customer.is_premium, order.days_since_payment ≤ 90
```

Brackets group arithmetic only, never logic. A comma at the end of a line in a
group is optional; the review view draws it.

## Missing values and failures

| Source | Scribe |
| --- | --- |
| Rust `Option<User>`, Python `User \| None` | `User or nothing` |
| Rust `if let Some(u) = found` | `if found is not nothing`, then use `found` |
| Rust `Result<T, E>` | `-> T` with `can fail with E1, E2` in the contract |
| Rust `f(x)?`, Swift `try f(x)` | `try f(x)` |
| Rust `f(x).unwrap_or(d)` | `let y = try f(x) else d` |
| Java `try { … } catch (E e) { … }` | `handle` block, then `on failure E as e` block |
| Rust `return Err(E::Bad)` | `fail with Bad` |

## Bit operations

Scribe has no bit operators. Bit operations are named functions on the
fixed-width unsigned types, so they need no precedence:

| Source | Scribe |
| --- | --- |
| C `flags & mask` | `bitwise_and(flags, mask)` |
| C `flags \| bit` | `bitwise_or(flags, bit)` |
| C `a ^ b` | `bitwise_xor(a, b)` |
| C `~x` | `bitwise_not(x)` |
| C `x << 3` | `shift_left(x, 3)` |
| C `x >> 3` | `shift_right(x, 3)` |
| C `(x >> n) & 1` | `bit(x, n)` |
| Literals `0xFF`, `0b1010` | `0xFF`, `0b1010`; `_` may separate digits: `0xFFFF_FFFF` |

## Reserved words

Scribe has 58 keywords, which can never be names, and three words that are
keywords only in their own position (`of` after `all` or `any`, and `requires`
and `exports` in a module). Scribe is written by machines and read by people,
so a word reserved to make a line unambiguous costs readers nothing
(specification, Section 2.5).

When a name in the source collides with a keyword, rename it. Two rules make
this easier than the list suggests:

- **Compound names are fine.** A name is one token, so `result_count`,
  `file_handle` and `type_name` are all allowed even though `result`,
  `handle` and `type` are not.
- **Capitalised names are fine.** Keywords are lower case, and types start
  with a capital (Section 2.4), so `Result`, `Type` and `Interface` can name
  types.

### Keywords that are common names elsewhere

| Keyword | What it does in Scribe | Common as a name for | Use instead |
| --- | --- | --- | --- |
| `result` | The value a function returns, in contracts | Return values, outcomes | `outcome`, `answer`, `found` |
| `type` | Declares a type | Kinds of thing, discriminators | `kind`, `category`, `variety` |
| `handle` | Handles failures | File and resource handles | `file_handle`, `descriptor`, `token` |
| `fail`, `failure` | `fail with`, `on failure` | Error flags and error values | `failed`, `error`, `problem` |
| `success` | `ensure on success` | Status flags | `succeeded`, `ok` |
| `hold` | Holds a resource for a block | Holding state | `held`, `holding`, `keep` |
| `changes` | `changes only` in a contract | Diffs, change lists | `edits`, `deltas`, `change_list` |
| `uses` | The capabilities a function uses | Usage counts | `usage_count`, `usages` |
| `expect` | A precondition | Test assertions (JavaScript) | `expected`, `require` |
| `model` | The abstract state of a type | Data models, ML models | `data_model`, `entity`, `predictor` |
| `states` | The states a type moves through | State lists | `state_list`, `phases` |
| `module` | A module: what it requires and exports | Module names, Python modules | `unit`, `package_name`, `part` |
| `interface` | A contract every implementation meets | APIs, network interfaces | `api`, `port`, `nic` |
| `property` | A rule run as generated tests | Object properties, attributes | `attribute`, `prop`, `trait_name` |
| `example` | An exact example in a contract | Samples, fixtures | `sample`, `specimen`, `fixture` |
| `match`, `case` | Matches every variant of an enum | Regex matches, test cases | `found`, `hit`, `scenario` |
| `old` | `old(x)`: a value when the function began | Previous values | `previous`, `former` |
| `editable`, `consumed` | Parameter conventions | Flags | `is_editable`, `used_up` |
| `trust` | A block the checker does not verify | Trust levels, scores | `trust_level`, `confidence` |
| `can` | `can fail with` | Permission flags | `allowed`, `able` |
| `until`, `through` | Range ends | End times | `end_time`, `due`, `limit` |
| `with` | `fail with`, `can fail with` | Options, companions | `options`, `companion` |
| `as` | Binds a name: `on failure E as e`, `Payments as payments` | Aliases | `alias`, `label` |
| `nothing` | The absent value: `User or nothing` | Rarely a name | `empty`, `none_found` |

### Other keywords

These are rarely names in other languages, so they seldom need renaming.

| Keyword | What it does in Scribe |
| --- | --- |
| `function` | Declares a function |
| `enum` | Declares an enum: variants that may carry data, as in Rust |
| `let`, `var` | Fixed binding and variable |
| `return` | Ends a function with its result |
| `if`, `else`, `then` | Branches; `then` follows a condition group |
| `for`, `in`, `while`, `do` | Loops; `in` also tests membership; `do` follows a condition group and in `do nothing` |
| `break`, `continue` | Leave a loop, or go to its next pass |
| `try` | Passes a failure up |
| `on` | `on failure`, `ensure on success` |
| `and`, `or`, `not` | Logic |
| `is` | Tests a variant or a missing value: `status is Paid`, `found is nothing` |
| `all`, `any` | Condition groups `all of`, `any of`, `not any of`; the quantifier `for all` |
| `there`, `where` | `there is x in xs where …`; `where` also filters a comprehension |
| `ensure` | A guarantee |
| `only` | `changes only` |
| `invariant`, `decreases` | Loop and type reasoning |
| `assert` | A condition proven at that point |
| `together` | Runs lines at once |
