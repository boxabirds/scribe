# Third-party code in experiment 7

## Source excerpts

Each file in `source/` is one function copied verbatim from another project, at the
commit listed in `results/source-commits.txt`, for experiment 7. They keep
their own licences, which are not this repository's Apache-2.0 licence. The
licence texts are in `source/licenses/`; for Node.js only its own section of the
licence file is included, which covers `lib/events.js`.

### Kept (37)

| File | Project | Source | Licence |
| --- | --- | --- | --- |
| F01.py | cpython | [Lib/bisect.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/bisect.py#L21-L54) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F02.go | go | [src/sort/search.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/sort/search.go#L58-L73) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F05.go | go | [src/path/match.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/path/match.go#L121-L194) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F06.erl | otp | [lib/stdlib/src/lists.erl](https://github.com/erlang/otp/blob/02a27602e62c1551f7ffa462aa10c65d040486f9/lib/stdlib/src/lists.erl#L4321-L4329) | Apache-2.0 ([text](source/licenses/otp-LICENSE.txt)) |
| F08.py | cpython | [Lib/heapq.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/heapq.py#L221-L233) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F09.dfy | dafny | [Source/DafnyStandardLibraries/src/Std/Collections/Seq.dfy](https://github.com/dafny-lang/dafny/blob/5f717bf447b19d38cad1b69b1bf9a9f102feccfb/Source/DafnyStandardLibraries/src/Std/Collections/Seq.dfy#L1048-L1070) | MIT ([text](source/licenses/dafny-LICENSE.txt)) |
| F10.go | go | [src/internal/strconv/atoi.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/internal/strconv/atoi.go#L172-L212) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F11.rs | ripgrep | [crates/globset/src/glob.rs](https://github.com/BurntSushi/ripgrep/blob/3fce3b5bb0236da2df6d99672afb8a719642eca7/crates/globset/src/glob.rs#L390-L415) | MIT or Unlicense ([text](source/licenses/ripgrep-COPYING)) |
| F12.py | cpython | [Lib/json/decoder.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/json/decoder.py#L353-L362) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F13.go | go | [src/os/tempfile.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/os/tempfile.go#L64-L76) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F14.py | cpython | [Lib/tempfile.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/tempfile.py#L703-L776) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F16.go | go | [src/bufio/scan.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/bufio/scan.go#L257-L261) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F17.py | cpython | [Lib/shutil.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/shutil.py#L554-L558) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F18.js | node | [lib/events.js](https://github.com/nodejs/node/blob/87db28b6bac0bb3e2919a7843c02ff798a31cd1f/lib/events.js#L332-L354) | MIT ([text](source/licenses/node-LICENSE)) |
| F19.go | go | [src/encoding/csv/reader.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/encoding/csv/reader.go#L292-L295) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F20.py | cpython | [Lib/shlex.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/shlex.py#L277-L285) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F22.py | cpython | [Lib/statistics.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/statistics.py#L1452-L1505) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F23.c | sqlite | [src/util.c](https://github.com/sqlite/sqlite/blob/9919c36b55bf5d37ad8eb67270c001b1d00db0e9/src/util.c#L442-L453) | Public domain ([text](source/licenses/sqlite-LICENSE.md)) |
| F24.go | go | [src/math/bits/bits.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/math/bits/bits.go#L292-L297) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F25.go | go | [src/time/format.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/time/format.go#L1508-L1523) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F26.py | cpython | [Lib/_pydatetime.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/_pydatetime.py#L2091-L2114) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F27.erl | otp | [lib/stdlib/src/calendar.erl](https://github.com/erlang/otp/blob/02a27602e62c1551f7ffa462aa10c65d040486f9/lib/stdlib/src/calendar.erl#L998-L1015) | Apache-2.0 ([text](source/licenses/otp-LICENSE.txt)) |
| F28.go | go | [src/net/textproto/reader.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/net/textproto/reader.go#L136-L188) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F29.py | cpython | [Lib/http/client.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/http/client.py#L249-L261) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F32.rs | ripgrep | [crates/matcher/src/lib.rs](https://github.com/BurntSushi/ripgrep/blob/3fce3b5bb0236da2df6d99672afb8a719642eca7/crates/matcher/src/lib.rs#L390-L399) | MIT or Unlicense ([text](source/licenses/ripgrep-COPYING)) |
| F33.go | go | [src/encoding/json/encode.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/encoding/json/encode.go#L382-L406) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F34.js | node | [lib/events.js](https://github.com/nodejs/node/blob/87db28b6bac0bb3e2919a7843c02ff798a31cd1f/lib/events.js#L791-L806) | MIT ([text](source/licenses/node-LICENSE)) |
| F35.py | cpython | [Lib/sched.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/sched.py#L53-L60) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F37.go | go | [src/sync/once.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/sync/once.go#L52-L71) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F38.py | cpython | [Lib/concurrent/futures/_base.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/concurrent/futures/_base.py#L543-L556) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F39.rs | tokio | [tokio/src/sync/oneshot.rs](https://github.com/tokio-rs/tokio/blob/4890d027f8a566ccf081e7f019ed766f780df1af/tokio/src/sync/oneshot.rs#L820-L868) | MIT ([text](source/licenses/tokio-LICENSE)) |
| F40.py | cpython | [Lib/dbm/dumb.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/dbm/dumb.py#L260-L264) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F42.go | go | [src/encoding/binary/varint.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/encoding/binary/varint.go#L130-L151) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F43.py | cpython | [Lib/configparser.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/configparser.py#L1123-L1135) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| F44.go | go | [src/flag/flag.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/flag/flag.go#L180-L183) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F46.go | go | [src/crypto/internal/fips140/subtle/constant_time.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/crypto/internal/fips140/subtle/constant_time.go#L17-L29) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| F47.py | cpython | [Lib/hmac.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/hmac.py#L161-L164) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |

### Not in this repository (11)

These excerpts were part of the pilot but their licences do not allow them to
be redistributed here. `selection.json` still records them, and the
translations of them were removed too (see below). To restore them locally,
from `research/`:

    tools/fetch_sources.sh /tmp/exp7-src
    python3 tools/select7.py /tmp/exp7-src corpus/exp7/source

This regenerates all 48 excerpts byte for byte. `tools/exp7_analyse.py` reads the excerpts,
so restore them before running it; with them restored it reproduces
`results/exp7-analysis.txt` and `results/exp7-rows.json` exactly. `source/.gitignore` keeps these 11
out of commits.

| Id | Project | Source | Licence |
| --- | --- | --- | --- |
| F03 | spark-by-example | [numeric/adjacent_difference_p.adb](https://github.com/tofgarion/spark-by-example/blob/2e4eb5a56e85c804dc48d58ea5ebb3fd46364c69/numeric/adjacent_difference_p.adb#L5-L30) | no licence in the repository, so no right to redistribute |
| F04 | ocaml | [stdlib/list.ml](https://github.com/ocaml/ocaml/blob/63027193163b31d0398ed6d95f3fa61022fd87d5/stdlib/list.ml#L401-L409) | LGPL-2.1 with the OCaml linking exception |
| F07 | redis | [src/adlist.c](https://github.com/redis/redis/blob/3228d82f49a976dd65e0f16125a6f822ad3f0416/src/adlist.c#L209-L212) | RSALv2, SSPLv1 or AGPLv3 (stated in each file header) |
| F15 | redis | [src/rio.c](https://github.com/redis/redis/blob/3228d82f49a976dd65e0f16125a6f822ad3f0416/src/rio.c#L387-L391) | RSALv2, SSPLv1 or AGPLv3 (stated in each file header) |
| F21 | redis | [src/sds.c](https://github.com/redis/redis/blob/3228d82f49a976dd65e0f16125a6f822ad3f0416/src/sds.c#L1066-L1174) | RSALv2, SSPLv1 or AGPLv3 (stated in each file header) |
| F30 | redis | [src/networking.c](https://github.com/redis/redis/blob/3228d82f49a976dd65e0f16125a6f822ad3f0416/src/networking.c#L6108-L6116) | RSALv2, SSPLv1 or AGPLv3 (stated in each file header) |
| F31 | ocaml | [stdlib/map.ml](https://github.com/ocaml/ocaml/blob/63027193163b31d0398ed6d95f3fa61022fd87d5/stdlib/map.ml#L312-L315) | LGPL-2.1 with the OCaml linking exception |
| F36 | redis | [src/ae.c](https://github.com/redis/redis/blob/3228d82f49a976dd65e0f16125a6f822ad3f0416/src/ae.c#L47-L81) | RSALv2, SSPLv1 or AGPLv3 (stated in each file header) |
| F41 | redis | [src/rdb.c](https://github.com/redis/redis/blob/3228d82f49a976dd65e0f16125a6f822ad3f0416/src/rdb.c#L403-L454) | RSALv2, SSPLv1 or AGPLv3 (stated in each file header) |
| F45 | redis | [src/config.c](https://github.com/redis/redis/blob/3228d82f49a976dd65e0f16125a6f822ad3f0416/src/config.c#L1513-L1542) | RSALv2, SSPLv1 or AGPLv3 (stated in each file header) |
| F48 | linux | [include/linux/overflow.h](https://github.com/torvalds/linux/blob/4056b3c9d3c92ea45955641dbafeb9b960e146ef/include/linux/overflow.h#L282-L289) | GPL-2.0 WITH Linux-syscall-note |

## Translations

Each `.scribe` file in `scribe-A/` and `scribe-B/` is a translation into
Scribe of the excerpt with the same id, made for experiment 7 and kept close
to the original so the two can be compared line by line. A translation is a
derivative work of its original, so each stays under the original's licence,
not this repository's. Changes made: the function was translated from its
original language into Scribe, with names converted to Scribe's style and
comments recording where the language fell short. The `.json` file beside each
translation is the translator's record; some quote a line or two of the
original to show where a gap is, under the same licence as the translation.

### Kept (43)

| File | Project | Original | Licence |
| --- | --- | --- | --- |
| scribe-A/F01.scribe | cpython | [Lib/bisect.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/bisect.py#L21-L54) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F02.scribe | go | [src/sort/search.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/sort/search.go#L58-L73) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F05.scribe | go | [src/path/match.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/path/match.go#L121-L194) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F06.scribe | otp | [lib/stdlib/src/lists.erl](https://github.com/erlang/otp/blob/02a27602e62c1551f7ffa462aa10c65d040486f9/lib/stdlib/src/lists.erl#L4321-L4329) | Apache-2.0 ([text](source/licenses/otp-LICENSE.txt)) |
| scribe-A/F08.scribe | cpython | [Lib/heapq.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/heapq.py#L221-L233) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F09.scribe | dafny | [Source/DafnyStandardLibraries/src/Std/Collections/Seq.dfy](https://github.com/dafny-lang/dafny/blob/5f717bf447b19d38cad1b69b1bf9a9f102feccfb/Source/DafnyStandardLibraries/src/Std/Collections/Seq.dfy#L1048-L1070) | MIT ([text](source/licenses/dafny-LICENSE.txt)) |
| scribe-A/F10.scribe | go | [src/internal/strconv/atoi.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/internal/strconv/atoi.go#L172-L212) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F11.scribe | ripgrep | [crates/globset/src/glob.rs](https://github.com/BurntSushi/ripgrep/blob/3fce3b5bb0236da2df6d99672afb8a719642eca7/crates/globset/src/glob.rs#L390-L415) | MIT or Unlicense ([text](source/licenses/ripgrep-COPYING)) |
| scribe-A/F12.scribe | cpython | [Lib/json/decoder.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/json/decoder.py#L353-L362) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F13.scribe | go | [src/os/tempfile.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/os/tempfile.go#L64-L76) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F14.scribe | cpython | [Lib/tempfile.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/tempfile.py#L703-L776) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F16.scribe | go | [src/bufio/scan.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/bufio/scan.go#L257-L261) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F17.scribe | cpython | [Lib/shutil.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/shutil.py#L554-L558) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F18.scribe | node | [lib/events.js](https://github.com/nodejs/node/blob/87db28b6bac0bb3e2919a7843c02ff798a31cd1f/lib/events.js#L332-L354) | MIT ([text](source/licenses/node-LICENSE)) |
| scribe-A/F19.scribe | go | [src/encoding/csv/reader.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/encoding/csv/reader.go#L292-L295) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F20.scribe | cpython | [Lib/shlex.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/shlex.py#L277-L285) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F22.scribe | cpython | [Lib/statistics.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/statistics.py#L1452-L1505) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F23.scribe | sqlite | [src/util.c](https://github.com/sqlite/sqlite/blob/9919c36b55bf5d37ad8eb67270c001b1d00db0e9/src/util.c#L442-L453) | Public domain ([text](source/licenses/sqlite-LICENSE.md)) |
| scribe-A/F24.scribe | go | [src/math/bits/bits.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/math/bits/bits.go#L292-L297) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F25.scribe | go | [src/time/format.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/time/format.go#L1508-L1523) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F26.scribe | cpython | [Lib/_pydatetime.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/_pydatetime.py#L2091-L2114) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F27.scribe | otp | [lib/stdlib/src/calendar.erl](https://github.com/erlang/otp/blob/02a27602e62c1551f7ffa462aa10c65d040486f9/lib/stdlib/src/calendar.erl#L998-L1015) | Apache-2.0 ([text](source/licenses/otp-LICENSE.txt)) |
| scribe-A/F28.scribe | go | [src/net/textproto/reader.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/net/textproto/reader.go#L136-L188) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F29.scribe | cpython | [Lib/http/client.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/http/client.py#L249-L261) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F32.scribe | ripgrep | [crates/matcher/src/lib.rs](https://github.com/BurntSushi/ripgrep/blob/3fce3b5bb0236da2df6d99672afb8a719642eca7/crates/matcher/src/lib.rs#L390-L399) | MIT or Unlicense ([text](source/licenses/ripgrep-COPYING)) |
| scribe-A/F33.scribe | go | [src/encoding/json/encode.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/encoding/json/encode.go#L382-L406) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F34.scribe | node | [lib/events.js](https://github.com/nodejs/node/blob/87db28b6bac0bb3e2919a7843c02ff798a31cd1f/lib/events.js#L791-L806) | MIT ([text](source/licenses/node-LICENSE)) |
| scribe-A/F35.scribe | cpython | [Lib/sched.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/sched.py#L53-L60) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F37.scribe | go | [src/sync/once.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/sync/once.go#L52-L71) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F38.scribe | cpython | [Lib/concurrent/futures/_base.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/concurrent/futures/_base.py#L543-L556) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F39.scribe | tokio | [tokio/src/sync/oneshot.rs](https://github.com/tokio-rs/tokio/blob/4890d027f8a566ccf081e7f019ed766f780df1af/tokio/src/sync/oneshot.rs#L820-L868) | MIT ([text](source/licenses/tokio-LICENSE)) |
| scribe-A/F40.scribe | cpython | [Lib/dbm/dumb.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/dbm/dumb.py#L260-L264) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F42.scribe | go | [src/encoding/binary/varint.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/encoding/binary/varint.go#L130-L151) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F43.scribe | cpython | [Lib/configparser.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/configparser.py#L1123-L1135) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-A/F44.scribe | go | [src/flag/flag.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/flag/flag.go#L180-L183) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F46.scribe | go | [src/crypto/internal/fips140/subtle/constant_time.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/crypto/internal/fips140/subtle/constant_time.go#L17-L29) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-A/F47.scribe | cpython | [Lib/hmac.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/hmac.py#L161-L164) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-B/F01.scribe | cpython | [Lib/bisect.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/bisect.py#L21-L54) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-B/F25.scribe | go | [src/time/format.go](https://github.com/golang/go/blob/e60b4c37fa344513cdecfffe6efc3ca48c4a6cb2/src/time/format.go#L1508-L1523) | BSD 3-Clause ([text](source/licenses/go-LICENSE)) |
| scribe-B/F26.scribe | cpython | [Lib/_pydatetime.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/_pydatetime.py#L2091-L2114) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-B/F35.scribe | cpython | [Lib/sched.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/sched.py#L53-L60) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-B/F38.scribe | cpython | [Lib/concurrent/futures/_base.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/concurrent/futures/_base.py#L543-L556) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |
| scribe-B/F47.scribe | cpython | [Lib/hmac.py](https://github.com/python/cpython/blob/d6f2129abdf14c2d653b8fc03a40d4a61674ec54/Lib/hmac.py#L161-L164) | PSF License Agreement ([text](source/licenses/cpython-LICENSE)) |

### Removed (15)

The translations of the 11 excerpts above whose licences do not allow them
here were removed with their records: `scribe-A/` F03, F04, F07, F15, F21,
F30, F31, F36, F41, F45 and F48, and `scribe-B/` F15, F21, F31 and F41. Unlike
the excerpts, they cannot be regenerated: an AI translator wrote them.
Experiment 7's recorded results in `results/` still count them, and a few of
those files quote single lines from them where parsing failed, so re-running
the analysis on this repository will not reproduce those results.
