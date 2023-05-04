#!/usr/bin/env bash

DIR="$1"

if [ -z "$DIR" ]; then
    echo "No directory given";
    exit 1
fi

find "$DIR" -maxdepth 1 -mindepth 1 -type d -print0 |
while IFS= read -r -d '' EXPERIMENT; do
    DIRNAME="$(basename "$EXPERIMENT")"
    VALUE=$(cat "$EXPERIMENT/new/estimates.json" | jq .slope.point_estimate)
    echo "${DIRNAME},${VALUE}"
done
