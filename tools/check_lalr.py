"""Check that the reference grammar has no LALR(1) conflicts.

usage: python3 tools/check_lalr.py

A grammar with no LALR(1) conflicts is unambiguous: no text has two readings
(specification, Section 1). This converts grammar/scribe.lark to Bison and
fails if Bison reports any conflict, printing Bison's counterexamples so the
cause is visible. Needs Bison 3.8 or later.
"""
import os
import re
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GRAMMAR = os.path.join(ROOT, "grammar", "scribe.lark")


def main():
    with tempfile.TemporaryDirectory() as tmp:
        y = os.path.join(tmp, "scribe.y")
        subprocess.run([sys.executable, os.path.join(ROOT, "tools", "to_bison.py"), GRAMMAR, y],
                       check=True, capture_output=True)
        result = subprocess.run(["bison", "-o", os.devnull, y], capture_output=True, text=True)
        if result.returncode != 0:
            sys.exit(f"bison failed:\n{result.stderr}")
        conflicts = re.findall(r"(\d+) (shift/reduce|reduce/reduce) conflicts?", result.stderr)
        if not conflicts:
            print("no LALR(1) conflicts: the grammar is unambiguous")
            return 0
        print("LALR(1) conflicts: " + ", ".join(f"{n} {kind}" for n, kind in conflicts))
        detail = subprocess.run(["bison", "-Wcounterexamples", "-o", os.devnull, y],
                                capture_output=True, text=True, timeout=1800)
        for line in detail.stderr.splitlines():
            if "conflict on token" in line or "xample:" in line:
                print("   ", line.replace(y + ": ", "").replace("nt_", "").replace("T_", ""))
        return 1


if __name__ == "__main__":
    sys.exit(main())
