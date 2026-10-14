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

Struggled:

1. **Function values.** Three of eight (`bisect_right`'s `key`,
   `read_continued_line`'s validator, `Once.Do`'s action) take a function as a
   parameter. Interfaces stood in, at a cost in lines. Spec 4.3 (function
   types) is Proposed and should be settled before the checker.
2. **Shared mutable state.** `Once` across tasks and tokio's atomic state need
   spec 7.4, which is Open. Both were translated in a single-task form.
3. **Bit operations with fixed-width conversions are long.**
   Go's `x |= uint64(b&0x7f) << s` became
   `x = bitwise_or(x, shift_left(to_u64(bitwise_and(b, 0x7f)), shift))`.
4. **A one-line condition group as an `if` condition needs a `then` line.**
   Go's `if !neg && un >= cutoff` became
   `if all of not negative, unsigned >= cutoff`, then `then` on its own line.
   The `then` line exists to separate a multi-line group from its block; a
   one-line group does not need it.
5. **Text by character.** With no slices and no character literals (spec 3.1,
   Open), `s[1:]` became `drop(s, 1)` and `s[0] == '+'` became
   `first_char(s) == "+"`.
6. **Smaller gaps.** The spec does not say whether `/` on integers truncates
   (Python's `//`), and `true` and `false` are not yet defined anywhere.
