#!/bin/sh
# Fetch the experiment 7 source repositories at the commits the pilot used,
# checking out only the files tools/select7.py draws from.
#
# usage: tools/fetch_sources.sh SRC_ROOT        (run from the pilot folder)
# then:  python3 tools/select7.py SRC_ROOT corpus/exp7/source
set -eu
root=$1
mkdir -p "$root"
python3 - <<'PY' > "$root/.paths"
import sys
sys.path.insert(0, "tools")
from select7 import FRAME
for pool in FRAME.values():
    for repo, path, _ in pool:
        print(repo, path)
PY
while read -r name url commit; do
    dir="$root/$name"
    [ -d "$dir/.git" ] || git init -q "$dir"
    git -C "$dir" fetch -q --depth 1 --filter=blob:none "$url" "$commit"
    paths=$(awk -v r="$name" '$1 == r && $2 !~ /\*/ { print "/" $2 }' "$root/.paths")
    if [ -n "$paths" ]; then
        git -C "$dir" sparse-checkout set --no-cone $paths
    fi
    git -C "$dir" checkout -q "$commit"
    echo "$name $commit"
done < results/source-commits.txt
rm "$root/.paths"
