#!/usr/bin/env bash
# Reruns every tool on every case and writes the outputs to results/.
# Needs: target/debug/scribe (cargo build), cargo-semver-checks on PATH or in
# $SEMVER_CHECKS, and Verus in $VERUS.
set -u
here=$(cd "$(dirname "$0")" && pwd)
scribe=${SCRIBE:-$here/../../target/debug/scribe}
semver=${SEMVER_CHECKS:-cargo-semver-checks}
verus=${VERUS:-verus}
cd "$here/cases"
mkdir -p ../results
for c in */; do
    c=${c%/}
    for v in new silent; do
        [ -f $c/scribe/$v.scribe ] && "$scribe" impact $c/scribe/old.scribe $c/scribe/$v.scribe \
            --dependents $c/scribe/dependent.scribe > ../results/$c.scribe-$v.txt 2>&1
    done
    [ -d $c/rust ] && "$semver" semver-checks --manifest-path $c/rust/new/Cargo.toml \
        --baseline-root $c/rust/old --color never > ../results/$c.semver.txt 2>&1
    for f in $c/verus/*.rs; do
        [ -f "$f" ] && "$verus" "$f" > ../results/$c.verus-$(basename $f .rs).txt 2>&1
    done
done
[ -f C02/keep_buffer.rs ] && rustc --edition 2021 C02/keep_buffer.rs -o /dev/null > ../results/C02.rustc.txt 2>&1
exit 0
