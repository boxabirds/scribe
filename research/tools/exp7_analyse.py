"""Experiment 7 analysis: sizes, gap records, and translator agreement.

usage: python3 exp7_analyse.py  (run from the scribe directory)
"""
import json
import os
import re
import statistics
from collections import Counter, defaultdict

ROOT = "corpus/exp7"
SEL = json.load(open(f"{ROOT}/source/selection.json"))
TOKEN = re.compile(r'"(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\'|\w+|[^\w\s]')

CLAUSE = re.compile(r"\s*(given|produces|fails|expect|ensure|uses|tier|changes|idempotent|"
                    r"declassifies|deadline|holds|runs|survives)\b")
DECL = re.compile(r"\s*(type|choice|field|model|define|component|system|protocol|schema|"
                  r"migration|interface|module)\b")


def strip_comments(text, lang):
    if lang in ("go", "c", "javascript", "rust", "dafny"):
        text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
        text = re.sub(r"//[^\n]*", "", text)
    elif lang == "python":
        text = re.sub(r'("""|\'\'\').*?\1', " ", text, flags=re.S)
        text = re.sub(r"#[^\n]*", "", text)
    elif lang == "ocaml":
        text = re.sub(r"\(\*.*?\*\)", " ", text, flags=re.S)
    elif lang == "erlang":
        text = re.sub(r"%[^\n]*", "", text)
    elif lang == "ada-spark":
        text = re.sub(r"--[^\n]*", "", text)
    return text


def source_body(path, lang):
    text = open(path).read()
    if lang == "ada-spark":
        text = text.split("-- body:", 1)[-1]
    text = strip_comments(text, lang)
    lines = [l for l in text.split("\n") if l.strip()]
    # drop the signature: everything up to the first line that opens the body
    for i, l in enumerate(lines):
        if lang in ("python",) and l.rstrip().endswith(":"):
            lines = lines[i + 1:]
            break
        if lang in ("go", "c", "javascript", "rust", "dafny") and "{" in l:
            lines = lines[i + 1:]
            break
        if lang in ("ocaml", "erlang") and ("=" in l or "->" in l):
            lines = lines[i + 1:]
            break
        if lang == "ada-spark" and re.match(r"\s*is\s*$|.*\bis$", l):
            lines = lines[i + 1:]
            break
    return lines


def scribe_parts(path):
    header, body, decl = [], [], []
    in_decl_indent = None
    for l in open(path).read().split("\n"):
        s = l.strip()
        if not s or s == "note" or s.startswith("note "):
            continue
        ind = len(l) - len(l.lstrip())
        if in_decl_indent is not None and ind > in_decl_indent:
            decl.append(l)
            continue
        in_decl_indent = None
        if DECL.match(l):
            decl.append(l)
            in_decl_indent = ind
        elif re.match(r"\s*(to|workflow)\b", l) or CLAUSE.match(l):
            header.append(l)
        else:
            body.append(l)
    return header, body, decl


def ntok(lines):
    return sum(len(TOKEN.findall(l)) for l in lines)


def main():
    rows = []
    for p in SEL:
        src = os.path.join(ROOT, "source", p["id"] + os.path.splitext(p["path"])[1])
        sb = source_body(src, p["language"])
        tr = os.path.join(ROOT, "scribe-A", p["id"] + ".scribe")
        rec = json.load(open(os.path.join(ROOT, "scribe-A", p["id"] + ".json")))
        h, b, d = scribe_parts(tr)
        notes = [l for l in open(tr).read().split("\n") if l.strip().startswith("note ")]
        rows.append({
            "id": p["id"], "category": p["category"], "language": p["language"],
            "adversarial": p["adversarial"],
            "src_body_lines": len(sb), "src_body_tokens": ntok(sb),
            "scribe_body_lines": len(b), "scribe_body_tokens": ntok(b),
            "scribe_header_lines": len(h), "scribe_decl_lines": len(d),
            "ratio_tokens": ntok(b) / max(ntok(sb), 1),
            "behaviour": rec.get("behaviour_preserved"),
            "gaps": [g for g in rec.get("gaps", []) if g.get("kind") == "gap"],
            "workarounds": [g for g in rec.get("gaps", []) if g.get("kind") == "workaround"],
            "unclear": [g for g in rec.get("gaps", []) if g.get("kind") == "unclear"],
            "invented": rec.get("invented_syntax", []),
            "contradicted": rec.get("doc_rules_contradicted", []),
            "gap_notes": sum("GAP" in n for n in notes),
        })
    json.dump(rows, open("results/exp7-rows.json", "w"), indent=1)

    n = len(rows)
    print(f"functions: {n}")
    print("behaviour preserved:", Counter(r["behaviour"] for r in rows))
    no_gap = [r for r in rows if not r["gaps"]]
    print(f"translated with no gap: {len(no_gap)}/{n}; non-adversarial: "
          f"{sum(1 for r in no_gap if not r['adversarial'])}/{sum(1 for r in rows if not r['adversarial'])}; "
          f"adversarial: {sum(1 for r in no_gap if r['adversarial'])}/{sum(1 for r in rows if r['adversarial'])}")
    print("total gaps / workarounds / unclear:",
          sum(len(r["gaps"]) for r in rows), sum(len(r["workarounds"]) for r in rows),
          sum(len(r["unclear"]) for r in rows))
    print("invented phrases:", sum(len(r["invented"]) for r in rows),
          "| doc rules contradicted:", sum(len(r["contradicted"]) for r in rows))

    # size
    ratios = [r["ratio_tokens"] for r in rows if r["src_body_tokens"] >= 10]
    print(f"median body token ratio (Scribe/original): {statistics.median(ratios):.2f} over {len(ratios)}")
    by_lang = defaultdict(list)
    for r in rows:
        if r["src_body_tokens"] >= 10:
            by_lang[r["language"]].append(r["ratio_tokens"])
    for lang, v in sorted(by_lang.items()):
        print(f"   {lang:11} median {statistics.median(v):.2f}  (n={len(v)})")
    print("median header lines per function:", statistics.median(r["scribe_header_lines"] for r in rows))

    # gap categories, counting each category once per function
    cat_fn = Counter()
    cat_all = Counter()
    for r in rows:
        seen = set()
        for g in r["gaps"] + r["workarounds"]:
            c = g.get("category", "?").strip().lower()
            cat_all[c] += 1
            seen.add(c)
        for c in seen:
            cat_fn[c] += 1
    print("\ngap and workaround categories (functions affected / total mentions):")
    for c, k in cat_fn.most_common():
        print(f"  {k:3} / {cat_all[c]:3}  {c}")

    # translator agreement on the double-translated functions
    dup = json.load(open(os.path.join(ROOT, "double_translated.json")))
    print("\nindependent second translations:")
    agree_beh = 0
    jac = []
    for fid in dup:
        a = json.load(open(os.path.join(ROOT, "scribe-A", fid + ".json")))
        b = json.load(open(os.path.join(ROOT, "scribe-B", fid + ".json")))
        ca = {g["category"].lower() for g in a.get("gaps", []) if g.get("kind") in ("gap", "workaround")}
        cb = {g["category"].lower() for g in b.get("gaps", []) if g.get("kind") in ("gap", "workaround")}
        j = len(ca & cb) / len(ca | cb) if (ca | cb) else 1.0
        jac.append(j)
        agree_beh += a.get("behaviour_preserved") == b.get("behaviour_preserved")
        ga = sum(1 for g in a.get("gaps", []) if g.get("kind") == "gap")
        gb = sum(1 for g in b.get("gaps", []) if g.get("kind") == "gap")
        print(f"  {fid}: gaps A={ga} B={gb}, category overlap {j:.2f}, behaviour A={a.get('behaviour_preserved')} B={b.get('behaviour_preserved')}")
    print(f"  mean category overlap {statistics.mean(jac):.2f}; behaviour verdict agreement {agree_beh}/{len(dup)}")


if __name__ == "__main__":
    main()
