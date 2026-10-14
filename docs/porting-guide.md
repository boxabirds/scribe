# Porting to Scribe

A guide for translating code from other languages into Scribe. It grows as
porting finds more to say; for now it covers parameters, conditions, bit
operations and names.

## Parameters

A function's parameters become an operation's header. Parameters with no
natural label are listed first, separated by commas; parameters that read
better with a label word follow:

| Source | Scribe header | Call |
| --- | --- | --- |
| `gcd(a, b)` | `to gcd first, second` | `gcd 12, 18` |
| `withdraw(account, amount)` | `to withdraw amount from account` | `withdraw 50 from savings` |
| `move(x, y, grid)` | `to move x, y to grid` | `move 3, 4 to board` |

The label words are `from`, `to`, `in`, `into`, `onto`, `over`, `for`, `with`,
`by`, `as`, `using` and `of`. A call with several unlabelled arguments stands
bare only as a whole statement or a whole value (`let g be gcd a, b`);
anywhere else it goes in parentheses: `if (gcd a, b) = 1`.

## Conditions

Every line of Scribe has exactly one reading, so a condition never relies on
precedence between `and`, `or` and `not`. A chain of only `and`, or only `or`,
fits on one line. Anything that mixes them becomes a condition group:
`all of`, `any of` or `none of`, with one condition per line beneath it.

| Source | Scribe |
| --- | --- |
| `paid and shipped` | `paid and shipped` |
| `a and (b or c)` | `all of` / `a` / `any of b, c` on the lines below, as shown next |
| `not (a or b)` | `none of a, b` |
| `!a && b` | `all of` / `not a` / `b`: `not` never sits inside a chain |
| `x > 0 ? y : z` | `if` statements; a condition is never a value chooser |

```
return all of
    status of order is in [Paid, Shipped]
    none of order is disputed, customer is suspended
    any of
        days_since_payment of order ≤ 30
        all of customer is premium, days_since_payment of order ≤ 90
```

Brackets group arithmetic only, never logic. A comma at the end of a line in a
group is optional; the review view draws it.

## Bit operations

Scribe has no bit operators. Bit operations are named operations on the
fixed-width unsigned types, so they need no precedence:

| Source | Scribe |
| --- | --- |
| `flags & mask` | `bitwise_and flags with mask` |
| `flags \| bit` | `bitwise_or flags with bit` |
| `a ^ b` | `bitwise_xor a with b` |
| `~x` | `bitwise_not x` |
| `x << 3` | `shift_left x by 3` |
| `x >> 3` | `shift_right x by 3` |
| `(x >> n) & 1` | `bit n of x` |
| `0xFF`, `0b1010` | `0xFF`, `0b1010`; `_` may separate digits: `0xFFFF_FFFF` |

## Reserved words

Scribe reserves more words than most languages: 111 hard keywords, which can
never be names, and 66 soft keywords, which are keywords only inside their own
phrase and ordinary names everywhere else. Scribe is written by machines and
read by people, so a word reserved to make a line unambiguous costs readers
nothing (specification, Section 2.5).

When a name in the source collides with a hard keyword, rename it. Three rules
make this easier than the list suggests:

- **Compound names are fine.** A name is one token, so `add_item`,
  `file_handle` and `result_count` are all allowed even though `add`,
  `handle` and `result` are not.
- **Capitalised names are fine.** Keywords are lower case, and types start
  with a capital (Section 2.4), so `Set`, `Type`, `Result` and `Interface`
  can name types.
- **Soft keywords are fine as names.** `value`, `version`, `queue`, `network`
  and the other soft keywords can name variables. Only the hard keywords below
  are a problem.

### Hard keywords that are common names elsewhere

| Keyword | What it does in Scribe | Common as a name for | Use instead |
| --- | --- | --- | --- |
| `a`, `an`, `the` | Articles in types: `a List of Int` | Short variables, the first of a pair | `x`, `first`, `left` |
| `add` | Update: `add price to total` | Operations that append or insert | `add_item`, `insert`, `append`, `plus` |
| `subtract` | Update: `subtract fee from balance` | Arithmetic helpers | `deduct`, `minus`, `take_away` |
| `increment` | Update: `increment count` | Counter helpers | `bump`, `advance`, `count_up` |
| `decrement` | Update: `decrement count` | Counter helpers | `count_down`, `lower` |
| `result` | The value an operation produces, in contracts | Return values, outcomes | `outcome`, `answer`, `found` |
| `return` | Ends an operation with its result | Fields holding a returned value | `returned`, `outcome` |
| `type` | Declares a type | Kinds of thing, discriminators | `kind`, `category`, `variety` |
| `set` | Changes a variable or field | Collections, setters | `members`, `seen`, `put` |
| `handle` | Handles failures | File and resource handles | `file_handle`, `descriptor`, `token` |
| `fail`, `fails`, `failure` | Failures and their handling | Error flags and error values | `failed`, `error`, `problem` |
| `try` | Passes a failure up | Retry counters, attempts | `attempt`, `try_count` |
| `stop`, `skip` | Leave or continue a loop | Flags and control variables | `halted`, `should_stop`, `skipped` |
| `hold` | Holds a resource for a block | Holding state | `held`, `holding`, `keep` |
| `holds` | Gives a predicate's condition | Holdings, positions | `holdings`, `positions` |
| `changes` | Says what an operation may change | Diffs, change lists | `edits`, `deltas`, `change_list` |
| `uses` | Says which capabilities an operation uses | Usage counts | `usage_count`, `usages` |
| `expect` | A precondition | Test assertions (JavaScript) | `expected`, `require` |
| `model` | The abstract state of a type | Data models, ML models | `data_model`, `entity`, `predictor` |
| `field` | A field of a record | Form fields, columns | `field_name`, `column`, `entry` |
| `schema` | A versioned data schema | Database schemas | `record_schema`, `shape`, `layout` |
| `migration` | A schema migration | Database migrations | `schema_change`, `upgrade` |
| `system` | A system file | The operating system, a subsystem | `platform`, `host`, `subsystem` |
| `component` | A unit of review and deployment | UI components, parts | `part`, `module`, `widget` |
| `interface` | An interface's contract | APIs, network interfaces | `api`, `port`, `nic` |
| `protocol` | A declared conversation | Wire protocols, scheme names | `wire_format`, `scheme`, `proto` |
| `property` | A generated test | Object properties, attributes | `attribute`, `prop`, `trait_name` |
| `example` | An exact example in a contract | Samples, fixtures | `sample`, `specimen`, `fixture` |
| `proof` | Proof hints | Cryptographic proofs | `evidence`, `witness` |
| `trust` | An escape hatch with an owner | Trust levels, scores | `trust_level`, `confidence` |
| `tier` | A clause's assurance tier | Pricing or service tiers | `level`, `plan_tier`, `grade` |
| `deadline` | A time limit monitored at run time | Due dates | `due`, `due_at`, `cutoff` |
| `choice` | Declares a choice type | User selections | `selection`, `option`, `pick` |
| `pipeline` | Stages joined by bounded queues | Build or data pipelines | `stages`, `chain`, `flow` |
| `repeat` | `repeat while` loops | Repetition counts | `repeat_count`, `count` |
| `when` | A branch of `choose` | Times and moments | `moment`, `at_time`, `occurred_at` |
| `where` | Filters in comprehensions and quantifiers | Locations | `place`, `location`, `site` |
| `while`, `until` | Loop and range words | End times | `end_time`, `due`, `limit` |
| `before`, `after` | Time comparisons | Earlier and later values | `earlier`, `later`, `start_time` |
| `from`, `to` | Argument labels | Senders and recipients, range ends | `sender`, `recipient`, `start`, `end`, `source`, `target` |
| `in`, `into`, `onto`, `over`, `for`, `with`, `by`, `as`, `using`, `of` | Argument labels | Inputs, options, units | `input`, `options`, `unit`, a name for what it holds |
| `it` | `if it succeeds` in contracts | Iterators, the current item | `item`, `current`, `element` |
| `mod` | Remainder | Modules, modulus | `module_name`, `modulus` |
| `some`, `nothing` | Present and absent values of a `maybe` | Rarely names; `some` in some query code | `any_of`, `empty` |
| `let`, `var` | Bindings | Template variables | `variable`, `binding` |
| `may` | `may block` | The month (lower case) | `month_may`, or the capitalised variant `May` |
| `all`, `any`, `none` | Condition groups: `all of`, `any of`, `none of` | Collection helpers, flags | `everything`, `any_found`, `none_found`, `empty` |
| `only` | `changes only`, `read only` | Filters and flags | `sole`, `exclusive`, `just_one` |
| `times` | `retry … at most 3 times` | Counts and timestamps | `count`, `occurrences`, `timestamps` |
| `labelled`, `matching`, `limited` | Type limits: `Text labelled untrusted`, `Text matching email_format` | Flags and filters | `has_label`, `is_match`, `capped` |
| `added`, `corrects` | Schema history, corrections | Change records | `added_at`, `correction` |

### Other hard keywords

These are rarely names in other languages, so they seldom need renaming.

| Keyword | What it does in Scribe |
| --- | --- |
| `and`, `or`, `not`, `implies` | Logic |
| `is` | Tests: equality with words, variants, predicates, membership |
| `at`, `less`, `more`, `than`, `most`, `least` | Comparison phrases: `is at most`, `cost at most` |
| `each`, `every`, `there` | `for each`, `for every`, `there is` |
| `if`, `otherwise`, `on` | Branches, `on failure` |
| `be`, `starts` | `let x be`, `var x starts at` |
| `through` | Ranges that include their end |
| `because` | The reason attached to a clause |
| `given`, `produces` | Parameters and results in a contract |
| `ensure` | A guarantee |
| `assumes` | A fact about the outside world |
| `idempotent`, `declassifies` | Contract clauses |
| `invariant`, `decreases` | Loop and type reasoning |
| `define` | Names a predicate |
| `choose` | Matches every variant of a choice |
| `together` | Runs lines at once |
| `followed` | `followed by`, joining sequences |
| `attenuated` | Narrows a capability |
| `do` | `do nothing`, for a branch with nothing to do |
| `then` | The second line of a condition written as `if` / `then` |
