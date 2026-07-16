#!/usr/bin/env bash
set -euo pipefail

runner=${EMBEDDED_RUNNER:?EMBEDDED_RUNNER is required}

request='{"wire_version":1,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"],"segments":["Public synthetic text."]}'
response=$(printf '%s' "$request" | "$runner")
test "$response" = '{"wire_version":1,"translations":["Texto sintetico publico."]}'

if printf '%s' '{"wire_version":1,"segments":["private marker"],"unknown":true}' \
    | "$runner" >runner.out 2>runner.err; then
    echo "runner accepted invalid wire" >&2
    exit 1
fi

if rg -q 'private marker|/home/|http|socket|listen' runner.out runner.err; then
    echo "runner exposed content or network/path detail" >&2
    exit 1
fi

rm -f runner.out runner.err
