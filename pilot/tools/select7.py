"""Experiment 7, pilot wave 1: draw functions from trusted source files.

The sampling frame is a grid of construct categories. Each category names three
source files from different languages or paradigms; one function is drawn at
random from each file with a fixed seed, so anyone can repeat the draw.

Known deviation from the experiment design: the file pools below were chosen by
the language designer, not by an independent person. Only the draw within each
file is random.

usage: python3 select7.py SRC_ROOT OUT_DIR
"""
import json
import os
import random
import re
import sys

SEED = 20261014
MIN_LINES, MAX_LINES = 4, 120

# category -> list of (repo dir, path, adversarial?)
FRAME = {
    "control flow and loops": [
        ("cpython", "Lib/bisect.py", False),
        ("go", "src/sort/search.go", False),
        ("spark-by-example", "*_p.adb", False),
    ],
    "recursion": [
        ("ocaml", "stdlib/list.ml", False),
        ("go", "src/path/match.go", False),
        ("otp", "lib/stdlib/src/lists.erl", False),
    ],
    "data structures": [
        ("redis", "src/adlist.c", True),          # pointer graph, nulls
        ("cpython", "Lib/heapq.py", False),
        ("dafny", "Source/DafnyStandardLibraries/src/Std/Collections/Seq.dfy", False),
    ],
    "error handling": [
        ("go", "src/internal/strconv/atoi.go", False),
        ("ripgrep", "crates/globset/src/glob.rs", False),
        ("cpython", "Lib/json/decoder.py", False),
    ],
    "resource handling": [
        ("go", "src/os/tempfile.go", False),
        ("cpython", "Lib/tempfile.py", False),
        ("redis", "src/rio.c", False),
    ],
    "input and output": [
        ("go", "src/bufio/scan.go", False),
        ("cpython", "Lib/shutil.py", False),
        ("node", "lib/events.js", True),           # stream-style emitter
    ],
    "text processing and parsing": [
        ("go", "src/encoding/csv/reader.go", False),
        ("cpython", "Lib/shlex.py", False),
        ("redis", "src/sds.c", False),
    ],
    "numerics": [
        ("cpython", "Lib/statistics.py", False),
        ("sqlite", "src/util.c", False),
        ("go", "src/math/bits/bits.go", False),
    ],
    "time and dates": [
        ("go", "src/time/format.go", False),
        ("cpython", "Lib/_pydatetime.py", False),
        ("otp", "lib/stdlib/src/calendar.erl", False),
    ],
    "state machines and protocols": [
        ("go", "src/net/textproto/reader.go", False),
        ("cpython", "Lib/http/client.py", False),
        ("redis", "src/networking.c", False),
    ],
    "polymorphism and generics": [
        ("ocaml", "stdlib/map.ml", True),          # functor
        ("ripgrep", "crates/matcher/src/lib.rs", True),   # traits, generics
        ("go", "src/encoding/json/encode.go", True),      # reflection
    ],
    "callbacks and events": [
        ("node", "lib/events.js", True),
        ("cpython", "Lib/sched.py", True),
        ("redis", "src/ae.c", True),
    ],
    "concurrency and async": [
        ("go", "src/sync/once.go", True),
        ("cpython", "Lib/concurrent/futures/_base.py", True),
        ("tokio", "tokio/src/sync/oneshot.rs", True),
    ],
    "persistence and schemas": [
        ("cpython", "Lib/dbm/dumb.py", False),
        ("redis", "src/rdb.c", False),
        ("go", "src/encoding/binary/varint.go", False),
    ],
    "configuration": [
        ("cpython", "Lib/configparser.py", False),
        ("go", "src/flag/flag.go", False),
        ("redis", "src/config.c", False),
    ],
    "security checks": [
        ("go", "src/crypto/internal/fips140/subtle/constant_time.go", False),
        ("cpython", "Lib/hmac.py", False),
        ("linux", "include/linux/overflow.h", True),  # macros
    ],
}

LANG = {".py": "python", ".go": "go", ".c": "c", ".h": "c", ".ml": "ocaml",
        ".erl": "erlang", ".js": "javascript", ".rs": "rust", ".dfy": "dafny",
        ".adb": "ada-spark"}


def brace_block(lines, i):
    depth, started = 0, False
    for j in range(i, len(lines)):
        depth += lines[j].count("{") - lines[j].count("}")
        if "{" in lines[j]:
            started = True
        if started and depth <= 0:
            return j
    return None


def functions(path, lang):
    lines = open(path, errors="replace").read().split("\n")
    found = []
    for i, line in enumerate(lines):
        name, end = None, None
        if lang == "python":
            m = re.match(r"(\s*)def (\w+)\(", line)
            if m:
                ind = len(m.group(1))
                name = m.group(2)
                end = i
                for j in range(i + 1, len(lines)):
                    if lines[j].strip() and (len(lines[j]) - len(lines[j].lstrip())) <= ind:
                        break
                    if lines[j].strip():
                        end = j
        elif lang in ("go",):
            m = re.match(r"func (\([^)]*\) )?(\w+)", line)
            if m:
                name, end = m.group(2), brace_block(lines, i)
        elif lang in ("rust", "dafny", "javascript"):
            m = (re.match(r"\s*(pub(\([^)]*\))? )?(async )?(unsafe )?fn (\w+)", line) if lang == "rust"
                 else re.match(r"\s*(function|method|lemma|predicate|ghost function|opaque function) (\w+)", line) if lang == "dafny"
                 else re.match(r"(?:async )?function (\w+)\(|(\w+(?:\.prototype)?\.\w+) = function", line))
            if m:
                name = [g for g in m.groups() if g and g.strip() not in ("pub", "async", "unsafe", "function", "method", "lemma", "predicate", "ghost function", "opaque function") and not g.startswith("pub")][-1]
                end = brace_block(lines, i)
        elif lang == "c":
            m = re.match(r"#define (\w+)\(", line)
            if m:
                name, end = m.group(1), i
                while end + 1 < len(lines) and lines[end].rstrip().endswith("\\"):
                    end += 1
            else:
                m = re.match(r"[A-Za-z_][\w \*]*?\b(\w+)\([^;]*$", line)
                if m and not line.startswith(("typedef", "struct", "#", "return", "if", "else")) \
                        and i + 1 < len(lines) and ("{" in line or lines[i + 1].startswith("{")
                                                    or (not line.rstrip().endswith(")") and "{" not in line)):
                    j = brace_block(lines, i)
                    if j is not None and j - i < 400:
                        name, end = m.group(1), j
        elif lang == "ocaml":
            m = re.match(r"( {0,4})(let|and) (rec )?([a-z_][\w']*)", line)
            if m:
                ind = len(m.group(1))
                name = m.group(4)
                end = i
                for j in range(i + 1, len(lines)):
                    lj = lines[j]
                    if lj.strip() and len(lj) - len(lj.lstrip()) <= ind and \
                            re.match(r"\s*(let|and|module|type|external|exception|\(\*|end|val)\b", lj):
                        break
                    if lj.strip():
                        end = j
        elif lang == "erlang":
            m = re.match(r"([a-z]\w*)\(", line)
            if m and (i == 0 or not re.match(re.escape(m.group(1)) + r"\(", lines[i - 1])):
                if any(f["name"] == m.group(1) for f in found):
                    continue
                name, end = m.group(1), None
                for j in range(i, len(lines)):
                    s = lines[j].split("%")[0].rstrip()
                    if s.endswith(".") and not s.endswith(".."):
                        end = j
                        break
        elif lang == "ada-spark":
            m = re.match(r"\s*(procedure|function) (\w+)", line)
            if m and " is" in "".join(lines[i:i + 8]) and not line.rstrip().endswith(";"):
                name = m.group(2)
                for j in range(i, len(lines)):
                    if re.match(r"\s*end " + re.escape(name) + r"\s*;", lines[j]):
                        end = j
                        break
        if name and end is not None:
            n = end - i + 1
            found.append({"name": name, "start": i + 1, "end": end + 1, "lines": n,
                          "text": "\n".join(lines[i:end + 1])})
    return found


def main():
    root, outdir = sys.argv[1], sys.argv[2]
    rng = random.Random(SEED)
    os.makedirs(outdir, exist_ok=True)
    picks = []
    for cat_i, (cat, pool) in enumerate(FRAME.items()):
        for repo, rel, adversarial in pool:
            if "*" in rel:
                import glob
                cands = sorted(glob.glob(os.path.join(root, repo, "**", rel), recursive=True))
                path = cands[rng.randrange(len(cands))]
            else:
                path = os.path.join(root, repo, rel)
            lang = LANG[os.path.splitext(path)[1]]
            fns = [f for f in functions(path, lang) if MIN_LINES <= f["lines"] <= MAX_LINES]
            # don't pick the same function twice across categories
            taken = {(p["path"], p["name"]) for p in picks}
            fns = [f for f in fns if (os.path.relpath(path, root), f["name"]) not in taken]
            f = fns[rng.randrange(len(fns))]
            pid = f"F{len(picks)+1:02d}"
            rec = {"id": pid, "category": cat, "adversarial": adversarial, "repo": repo,
                   "path": os.path.relpath(path, root), "language": lang, "name": f["name"],
                   "lines": [f["start"], f["end"]], "line_count": f["lines"],
                   "candidates_in_file": len(fns)}
            ext = os.path.splitext(path)[1]
            text = f["text"] + "\n"
            if lang == "ada-spark":
                spec = path[:-4] + ".ads"
                if os.path.exists(spec):
                    text = "-- specification (" + os.path.basename(spec) + "):\n" + open(spec).read() + "\n-- body:\n" + text
            open(os.path.join(outdir, f"{pid}{ext}"), "w").write(text)
            picks.append(rec)
            print(f"{pid} {cat:30} {repo:16} {lang:10} {f['name']:28} {f['lines']:4} lines  (of {len(fns)})")
    json.dump(picks, open(os.path.join(outdir, "selection.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
