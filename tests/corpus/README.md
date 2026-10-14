# Real code in Scribe

Eight functions from well-used libraries, translated into Scribe (draft 0.5),
to test the syntax on code nobody wrote to show it off. Every file must parse
with exactly one reading (`tools/check_grammar.py`), and both lexers must agree
on it (`tools/compare_lexers.py`). The parser will be tested against them too.

Each translation is a derivative of the original and keeps its licence. The
licence texts are in `research/corpus/exp7/source/licenses/`.

| File | Translated from | Licence |
| --- | --- | --- |
| `bisect_right.scribe` | cpython `Lib/bisect.py`, `bisect_right` | PSF License Agreement |
| `parse_int.scribe` | Go `src/internal/strconv/atoi.go`, `ParseInt` | BSD 3-Clause |
| `prefix_and_suffix.scribe` | Go `src/os/tempfile.go`, `prefixAndSuffix` | BSD 3-Clause |
| `read_uvarint.scribe` | Go `src/encoding/binary/varint.go`, `ReadUvarint` | BSD 3-Clause |
| `read_continued_line.scribe` | Go `src/net/textproto/reader.go`, `readContinuedLineSlice` | BSD 3-Clause |
| `once.scribe` | Go `src/sync/once.go`, `Once.Do` | BSD 3-Clause |
| `poll_closed.scribe` | tokio `tokio/src/sync/oneshot.rs`, `Sender::poll_closed` | MIT |
| `match_chunk.scribe` | Go `src/path/match.go`, `matchChunk` | BSD 3-Clause |

The exact commits are in `research/corpus/exp7/NOTICE.md`.

## What the translations showed

Worked well:

- Contracts state what the originals only say in comments: `bisect_right`'s
  docstring ("all e in a[:i] have e <= x") became two `ensure` lines.
- Failures that carry data replace Go's "value and error" pairs: `parse_int`
  fails with `Range(clamped: ...)` instead of returning a value next to
  `ErrRange`.
- Records replace multiple return values (`prefix_and_suffix`).
- Typestate makes `Once`'s "at most once" a compile error.
- Rust's `unsafe` blocks map one for one onto `trust` blocks.

Struggled, and what changed (draft 0.5, decision 36):

1. **Function values.** Three of eight take a function as a parameter.
   Function types (`key: function(Int) -> Int`) now cover `bisect_right`.
   The other two, `read_continued_line`'s validator and `Once.Do`'s action,
   can fail or have effects, which a function type does not yet carry
   (spec 4.3, open decision 15), so they still use interfaces.
2. **Shared mutable state.** `Once` across tasks and tokio's atomic state need
   spec 7.4, which is Open. Both are translated in a single-task form.
3. **Bit operations** were long as named functions. With `&`, `|`, `^`, `<<`
   and `>>`, Go's `x |= uint64(b&0x7f) << s` is
   `x = x | (to_u64(b & 0x7f) << shift)`.
4. **A one-line condition group as an `if` condition** needed a `then` line;
   now it goes straight to its block.
5. **Text by character** is `text[i]`; the rest of a text is still
   `drop(text, n)`, since there is no slice syntax.
6. **Integer division and `true` / `false`** are now specified.
