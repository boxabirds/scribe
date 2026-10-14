# Scribe pilot: experiments 3 and 7 (wave 1)

Everything needed to repeat the pilot. Run commands from this folder (Python 3.13, `pip install lark`, Bison 3.8).

grammar/scribe.lark.tmpl          grammar v0.2 (template; build with tools/build_grammar.py)
grammar/scribe-measure.lark.tmpl  same grammar, tolerant of elided lines (for line coverage)
grammar/scribe-v0.1.lark          first draft, kept for reference
tools/pre.py                      indentation pre-lexer
tools/run3.py                     Earley parse with all readings kept; disambiguation rules D1, D2
tools/measure_lines.py            line-level parse coverage
tools/to_bison.py                 grammar -> Bison, for LALR conflicts and counterexamples
tools/select7.py                  experiment 7 sampling frame and seeded draw (seed 20261014)
tools/exp7_analyse.py             sizes, gap records, translator agreement
corpus/doc                        the 42 code blocks of the syntax doc (rev 62)
corpus/doc-repaired               the same with 15 documented repairs (results/exp3-doc-repairs.json)
corpus/exp7/source                the 48 drawn functions and selection.json
corpus/exp7/scribe-A              first translations (.scribe) and records (.json)
corpus/exp7/scribe-B              independent second translations of 10 functions
corpus/exp7/BRIEF.md              the translators' brief
results/                          all outputs; results/untrusted holds files written by another agent (not used)

Repeat:
  python3 tools/build_grammar.py grammar/scribe.lark.tmpl grammar/scribe.lark
  python3 tools/run3.py grammar/scribe.lark corpus/doc
  python3 tools/run3.py grammar/scribe.lark corpus/doc-repaired
  python3 tools/build_grammar.py grammar/scribe-measure.lark.tmpl grammar/scribe-measure.lark
  python3 tools/measure_lines.py grammar/scribe-measure.lark corpus/exp7/scribe-A
  python3 tools/exp7_analyse.py
  python3 tools/to_bison.py grammar/scribe.lark results/bison/scribe.y && bison -Wcounterexamples -o /dev/null results/bison/scribe.y
