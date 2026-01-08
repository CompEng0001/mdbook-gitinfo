#!/usr/bin/env bash

for f in *; do
    if [[ -f "$f" ]]; then
        printf '=================== %s ===================\n' "$f"
        cat -- "$f"
        printf '\n'
    fi
done