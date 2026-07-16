#!/usr/bin/env bash
set -euo pipefail

runner=${EMBEDDED_RUNNER:?EMBEDDED_RUNNER is required}

request='{"wire_version":1,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"],"segments":["Public synthetic text."]}'
response=$(printf '%s' "$request" | "$runner")
test "$response" = '{"wire_version":1,"translations":["Texto sintetico publico."]}'

utf8_request='{"wire_version":1,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"],"segments":["Café 😀."]}'
utf8_response=$(printf '%s' "$utf8_request" | "$runner")
test "$utf8_response" = '{"wire_version":1,"translations":["Texto sintetico controlado."]}'

two_segment_request='{"wire_version":1,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"],"segments":["One.","Two."]}'
two_segment_response=$(printf '%s' "$two_segment_request" | "$runner")
test "$two_segment_response" = '{"wire_version":1,"translations":["Uno.","Dos."]}'

if printf '%s' '{"wire_version":1,"segments":["private marker"],"unknown":true}' \
    | "$runner" >runner.out 2>runner.err; then
    echo "runner accepted invalid wire" >&2
    exit 1
fi

oversized_segment=$(head -c 4097 /dev/zero | tr '\0' x)
oversized_request=$(printf '{"wire_version":1,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"],"segments":["%s"]}' "$oversized_segment")
if printf '%s' "$oversized_request" | "$runner" >/dev/null 2>&1; then
    echo "runner accepted an oversized segment" >&2
    exit 1
fi

if printf '\377' | "$runner" >/dev/null 2>&1; then
    echo "runner accepted invalid UTF-8" >&2
    exit 1
fi

if rg -q 'private marker|/home/|http|socket|listen' runner.out runner.err; then
    echo "runner exposed content or network/path detail" >&2
    exit 1
fi

rm -f runner.out runner.err
