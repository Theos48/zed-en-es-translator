#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/package.lock.json"
package_id=$(jq -r '.package_id' "$lock")
prepared="$root/target/marketplace-real/$package_id"
smoke_root="$root/target/marketplace-real-smoke"
fixture='The extension translates this public sentence without a mock provider.'
request=$(jq -cn --arg fixture "$fixture" '{wire_version:1,source_language:"en",target_language:"es",tone:"technical_neutral",preserve:["markdown_structure","code","links"],segments:[$fixture]}')

test -d "$prepared"
rm -rf "$smoke_root"
mkdir -p "$smoke_root"

for iteration in 1 2 3; do
    package_root="$smoke_root/run-$iteration/$package_id"
    mkdir -p "$(dirname "$package_root")"
    cp -a "$prepared" "$package_root"
    response=$(cd "$package_root" && printf '%s' "$request" | timeout --signal=KILL 15 \
        env -i LC_ALL=C.UTF-8 OMP_NUM_THREADS=4 OPENBLAS_NUM_THREADS=4 \
        ./bin/translator-embedded-runtime \
        --model models/model.enes.intgemm.alphas.bin \
        --vocabulary models/vocab.enes.spm \
        --lexical-shortlist models/lex.50.50.enes.s2t.bin)
    jq -e --arg fixture "$fixture" '
      .wire_version == 1
      and (.translations | length) == 1
      and (.translations[0] | length) > 0
      and .translations[0] != $fixture
      and (.translations[0] | ascii_downcase | contains("mock") | not)
    ' <<<"$response" >/dev/null
    test -f "$package_root/installed.json"
    printf 'marketplace_smoke_run=%s status=pass provider=embedded_local source_mutation=none\n' "$iteration"
done

printf 'marketplace_real_smoke=3/3\n'
