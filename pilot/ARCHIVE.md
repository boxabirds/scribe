# How this folder differs from scribe-pilot-wave1.zip

Checked on import: every file in `results/checksums-final.txt` matches, and
`tools/run3.py` on `corpus/doc` and `corpus/doc-repaired` reproduces
`results/exp3-v0.2-doc.txt` and `results/exp3-v0.2-doc-repaired.txt` exactly.

**Left out.**

- `results/untrusted/`: the three files written by the translation agent that
  edited the grammar during the run. The pilot report says they were not used.
- The 48 excerpts in `corpus/exp7/source/`, which are verbatim code from other
  projects under their own licences (Linux is GPL-2.0; Redis is under its own
  source-available licences). `selection.json` is kept. To restore them, from
  this folder:

      tools/fetch_sources.sh /tmp/exp7-src
      python3 tools/select7.py /tmp/exp7-src corpus/exp7/source

  This regenerates all 48 files and `selection.json` byte for byte (checked on
  import).

**Added.** `tools/fetch_sources.sh`, which fetches each repository at the
commit in `results/source-commits.txt`, checking out only the files the draw
uses, and this file.

**Known issue.** `tools/build_grammar.py` orders keywords of equal length by
Python's set order, so rebuilding `grammar/scribe.lark` gives different bytes
on different runs with the same meaning. The copy in the repository's top-level
`tools/` fixes this.
