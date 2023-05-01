#!/usr/bin/env sh

DIR="$1"

if [ -z "$DIR" ]; then
    echo "No directory given";
    exit 1
fi

find "$DIR" -type d -depth 1 -print0 |
while IFS= read -r -d '' EXPERIMENT; do
    DIRNAME="$(basename "$EXPERIMENT")"
    VALUE=$(cat "$EXPERIMENT/new/estimates.json" | jq .slope.point_estimate)
    echo "${DIRNAME%K},${VALUE}"
done
