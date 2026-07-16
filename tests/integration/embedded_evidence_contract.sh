#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
corpus="$root/tests/fixtures/embedded/synthetic-corpus.json"
benchmark="$root/tests/integration/embedded_benchmark.sh"
manual="$root/specs/008-embedded-local-provider/manual-validation.md"
data_home="$root/target/embedded-evidence-contract-$$"
trap 'rm -rf "$data_home"' EXIT

fail() {
  printf 'embedded evidence contract failed: %s\n' "$1" >&2
  exit 1
}

jq -e '
  .schema_version == 1 and
  .fixture_set_version == "embedded-en-es-public-v1" and
  (.cases | length == 20) and
  ([.cases[].case_id] | unique | length == 20) and
  ([.cases[] | select(.class == "short_technical")] | length == 8) and
  ([.cases[] | select(.class == "unicode_punctuation")] | length == 4) and
  ([.cases[] | select(.class == "mixed_0_5_to_2_kib")] | length == 4) and
  ([.cases[] | select(.class == "near_4_kib")] | length == 2) and
  ([.cases[] | select(.class == "markdown_multi_segment")] | length == 2) and
  all(.cases[];
    (.case_id | test("^(short|unicode|mixed|limit|markdown)-[0-9]{2}$")) and
    (.expect.non_empty == true) and
    (.expect.changed == true)
  )
' "$corpus" >/dev/null || fail 'invalid 20-case corpus structure'

while IFS=$'\t' read -r case_id class byte_length segment_count; do
  case "$class" in
    short_technical|unicode_punctuation)
      (( byte_length > 0 && byte_length < 512 && segment_count == 1 )) \
        || fail "$case_id has invalid short/unicode size"
      ;;
    mixed_0_5_to_2_kib)
      (( byte_length >= 512 && byte_length <= 2048 && segment_count == 1 )) \
        || fail "$case_id is outside 0.5-2 KiB"
      ;;
    near_4_kib)
      (( byte_length >= 3500 && byte_length <= 4096 && segment_count == 1 )) \
        || fail "$case_id is not near the 4 KiB limit"
      ;;
    markdown_multi_segment)
      (( segment_count >= 2 )) || fail "$case_id is not multi-segment"
      ;;
    *) fail "$case_id has unknown class" ;;
  esac
done < <(jq -r '
  .cases[] |
  (if has("segments") then .segments
   else . as $case | [([range(0; $case.repeat) | $case.unit] | join(" "))]
   end) as $segments |
  [.case_id, .class, ($segments | join("") | utf8bytelength), ($segments | length)] |
  @tsv
' "$corpus")

rg -q '^warmups=5$' "$benchmark" || fail 'warmup count is not fixed'
rg -q '^rounds=3$' "$benchmark" || fail 'round count is not fixed'
rg -q '^repetitions=5$' "$benchmark" || fail 'repetition count is not fixed'
rg -q "BLOCKED_LICENSE_APPROVAL" "$manual" || fail 'manual no-go outcome missing'

set +e
output=$(XDG_DATA_HOME="$data_home" EMBEDDED_NETWORK_ISOLATED=1 "$benchmark" 2>&1)
status=$?
set -e
[[ "$status" -ne 0 ]] || fail 'blocked manifest unexpectedly benchmarked'
[[ "$output" == 'provider_status=BLOCKED_LICENSE_APPROVAL' ]] \
  || fail 'benchmark did not fail with the normalized license gate'
[[ ! -e "$data_home/zed-en-es-translator/embedded" ]] \
  || fail 'blocked benchmark mutated provider state'

if rg -n -i \
  '(hostname=|username=|machine_id=|source_text=|translation=|translated_text=|/home/[^ ]+|XDG_(DATA|STATE)_HOME=)' \
  "$manual"; then
  fail 'manual evidence contains a prohibited field or sensitive path'
fi

printf 'provider_status=evidence_contract_verified\n'
