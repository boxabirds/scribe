# Experiment 7 translation brief (pilot wave 1)

You are translating real, well-trusted functions into **Scribe**, a new programming
language that exists only as a design document. The purpose is to find out where the
design cannot express real code, or can only express it clumsily. Finding gaps is
the goal. A translation that hides a gap is worse than one that reports it.

## What to read

- The language: `/home/claude/scribe/corpus/exp7/scribe-syntax-doc.md`. Read all of it
  before translating. Its vocabulary table, notation section and 50 examples are the
  whole language. Treat the doc's examples as the authority on style.
- Your source functions: `/home/claude/scribe/corpus/exp7/source/<ID>.*`, with metadata
  in `/home/claude/scribe/corpus/exp7/source/selection.json`.
- Do **not** read anything else under `/home/claude/scribe` (no grammar files, no
  results, no other translator's output). This keeps translations independent.

## How to translate

1. Translate each function into Scribe in canonical form, preserving its behaviour
   as closely as the language allows. Keep the original's names (converted to
   lower-case with underscores) so the two can be compared line by line.
2. Write the operation header the doc requires: `given`, `produces`, `fails with`,
   `uses`, and capabilities where the original does I/O, allocates, reads a clock,
   takes a lock, and so on. Add `expect` / `ensure` lines only where the original
   states or checks the condition (a documented precondition, an assertion, an
   explicit check). Do not invent specifications the original does not have.
3. Use only constructs that appear in the doc. If you need something the doc does not
   have, do **not** silently invent syntax. Either:
   - use the closest existing construct and mark the spot with a comment line
     `note WORKAROUND: <what is clumsy and why>`, or
   - if nothing reasonable exists, write your best guess at what Scribe would need,
     and mark it `note GAP: <construct needed> - <why the doc has no way to say this>`.
4. Helper types or operations the function depends on (structs, other functions it
   calls) can be declared briefly (`type`, `choice`, or a `to` header with no body) so
   the translation is self-contained. Keep these minimal.
5. If the doc is ambiguous about how to write something, pick one reading, write it,
   and add `note UNCLEAR: <what the doc leaves open>`.

## What to produce

For each assigned ID, write two files:

- `<OUT_DIR>/<ID>.scribe` — the translation.
- `<OUT_DIR>/<ID>.json` — a record:

```json
{
  "id": "F07",
  "behaviour_preserved": "exact | approximate | not possible",
  "gaps": [
    {"category": "<one of the list below, or a new one>",
     "construct": "<the specific thing the original does>",
     "description": "<why Scribe cannot express it>",
     "kind": "gap | workaround | unclear"}
  ],
  "doc_rules_contradicted": ["<any rule in the doc the translation had to break, quoted>"],
  "invented_syntax": ["<any phrase you used that is not in the doc>"],
  "notes": "<anything else a language designer should know, max 3 sentences>"
}
```

Gap categories (use these where they fit, add new ones if needed): pointers and
shared mutable references; missing values; generics and type parameters; interfaces
and dynamic dispatch; reflection; macros; closures and stored callbacks; bit
operations; characters, bytes and strings; error types and exceptions; early exit
from loops; loop forms; integer overflow and wrapping; unsafe and raw memory; default,
optional and keyword arguments; variadic arguments; objects, methods and self; global
or module state; pattern matching; concurrency, threads and locks; async and futures;
I/O primitives; time and clocks; collections and iteration; numeric types and
conversions; resource lifetime; contracts and specifications.

`kind` is:
- `gap` - the language has no way to express it; you had to invent something,
- `workaround` - expressible with existing constructs, but clumsily (say how),
- `unclear` - the doc's rules don't settle how to write it.

When all your IDs are done, reply with one line per ID: the ID, the number of gaps
and workarounds, and the single most important finding. Nothing else.
