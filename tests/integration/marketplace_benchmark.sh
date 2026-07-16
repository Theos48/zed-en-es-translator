#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/package.lock.json"
corpus="$root/tests/fixtures/marketplace/synthetic-corpus.json"
package_id=$(jq -r '.package_id' "$lock")
package_root=${MARKETPLACE_PACKAGE_ROOT:-"$root/target/marketplace-real/$package_id"}
results="$root/target/marketplace-benchmark"
image=${RUST_DEV_IMAGE:-zed-en-es-translator-rust:1.96.1}
source_sha=$(sha256sum "$corpus" | cut -d' ' -f1)

fail() {
    printf 'marketplace benchmark: %s\n' "$1" >&2
    exit 1
}

test -x "$package_root/bin/translator-embedded-runtime" || fail 'real runner is missing'
test -s "$package_root/models/model.enes.intgemm.alphas.bin" || fail 'real model is missing'
test "$(jq '.cases | length' "$corpus")" -eq 20 || fail 'public corpus must contain 20 cases'
rm -rf "$results"
mkdir -p "$results"

overall_max_rss_kib=0
overall_max_threads=0
passed=0

while IFS= read -r case_id; do
    request="$results/$case_id.request.json"
    response="$results/$case_id.response.json"
    stderr_file="$results/$case_id.stderr"
    segments=$(jq -c --arg case_id "$case_id" '
      .cases[] | select(.case_id == $case_id)
      | if has("segments") then .segments
        else .unit as $unit | .repeat as $repeat
          | [[range(0; $repeat)] | map($unit) | join(" ")]
        end
    ' "$corpus")
    jq -n --argjson segments "$segments" '{
      wire_version: 1,
      source_language: "en",
      target_language: "es",
      tone: "technical_neutral",
      preserve: ["markdown_structure", "code", "links"],
      segments: $segments
    }' >"$request"

    container="en-es-benchmark-${$}-${case_id}"
    start_ns=$(date +%s%N)
    docker run --rm --name "$container" --network none --read-only \
        --user "$(id -u):$(id -g)" \
        -i -v "$package_root:/package:ro" -w /package "$image" \
        env -i LC_ALL=C.UTF-8 OMP_NUM_THREADS=4 OPENBLAS_NUM_THREADS=4 \
        ./bin/translator-embedded-runtime \
        --model models/model.enes.intgemm.alphas.bin \
        --vocabulary models/vocab.enes.spm \
        --lexical-shortlist models/lex.50.50.enes.s2t.bin \
        <"$request" >"$response" 2>"$stderr_file" &
    docker_pid=$!
    case_max_rss_kib=0
    case_max_threads=0
    while kill -0 "$docker_pid" 2>/dev/null; do
        while IFS= read -r pid; do
            test -r "/proc/$pid/cmdline" || continue
            executable=$(readlink "/proc/$pid/exe" 2>/dev/null || true)
            test "${executable##*/}" = translator-embedded-runtime || continue
            command_line=$(tr '\0' ' ' <"/proc/$pid/cmdline")
            case "$command_line" in
                *translator-embedded-runtime*--model*model.enes.intgemm.alphas.bin*)
                    rss_kib=$(awk '/^VmRSS:/ {print $2}' "/proc/$pid/status" 2>/dev/null || true)
                    threads=$(awk '/^Threads:/ {print $2}' "/proc/$pid/status" 2>/dev/null || true)
                    rss_kib=${rss_kib:-0}
                    threads=${threads:-0}
                    (( rss_kib > case_max_rss_kib )) && case_max_rss_kib=$rss_kib
                    (( threads > case_max_threads )) && case_max_threads=$threads
                    ;;
            esac
        done < <(pgrep -u "$(id -u)" -f translator-embedded-runtime || true)
        sleep 0.01
    done
    if ! wait "$docker_pid"; then
        fail "$case_id failed inside the network-disabled container"
    fi
    end_ns=$(date +%s%N)
    elapsed_ms=$(( (end_ns - start_ns) / 1000000 ))

    test ! -s "$stderr_file" || fail "$case_id emitted raw stderr"
    jq -e --slurpfile request "$request" '
      . as $response
      | .wire_version == 1
      and (.translations | length) == ($request[0].segments | length)
      and all(range(0; ($response.translations | length));
        ($response.translations[.] | type == "string" and length > 0)
        and $response.translations[.] != $request[0].segments[.])
    ' "$response" >/dev/null || fail "$case_id returned empty, unchanged or malformed output"
    test "$elapsed_ms" -lt 15000 || fail "$case_id exceeded the 15-second deadline"
    test "$case_max_rss_kib" -gt 0 || fail "$case_id RSS could not be observed"
    test "$case_max_rss_kib" -lt 1048576 || fail "$case_id exceeded 1 GiB RSS"
    test "$case_max_threads" -gt 0 || fail "$case_id thread count could not be observed"
    test "$case_max_threads" -le 4 || fail "$case_id exceeded four inference threads"

    (( case_max_rss_kib > overall_max_rss_kib )) && overall_max_rss_kib=$case_max_rss_kib
    (( case_max_threads > overall_max_threads )) && overall_max_threads=$case_max_threads
    passed=$((passed + 1))
    printf 'case=%s status=pass elapsed_ms=%s peak_rss_kib=%s max_threads=%s\n' \
        "$case_id" "$elapsed_ms" "$case_max_rss_kib" "$case_max_threads"
    rm -f "$request" "$response" "$stderr_file"
done < <(jq -r '.cases[].case_id' "$corpus")

test "$(sha256sum "$corpus" | cut -d' ' -f1)" = "$source_sha" \
    || fail 'public source fixture changed during translation'
test "$passed" -eq 20 || fail 'not every public case passed'

printf 'marketplace_cases=20/20\n'
printf 'marketplace_peak_rss_kib=%s\n' "$overall_max_rss_kib"
printf 'marketplace_max_threads=%s\n' "$overall_max_threads"
printf 'marketplace_network=disabled\n'
