#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
manifest="$root/ops/providers/embedded/provider.lock.json"
corpus="$root/tests/fixtures/embedded/synthetic-corpus.json"

warmups=5
rounds=3
repetitions=5
max_transfer_bytes=$((64 * 1024 * 1024))
max_active_installed_bytes=$((128 * 1024 * 1024))
max_lifecycle_bytes=$((384 * 1024 * 1024))
max_peak_rss_bytes=$((1024 * 1024 * 1024))
max_inference_threads=4
max_cold_readiness_ms=10000
max_warm_short_p95_ms=2000
max_warm_mixed_p95_ms=5000
max_provider_request_ms=14999

blocked() {
  printf 'provider_status=BLOCKED_LICENSE_APPROVAL\n' >&2
  exit 1
}

resource_blocked() {
  printf 'provider_status=BLOCKED_RESOURCE_BUDGET\n' >&2
  exit 1
}

[[ "$(jq -r '.review_status' "$manifest")" == approved ]] || blocked
[[ "$(jq -r '.local_approval.kind // empty' "$manifest")" == human ]] || blocked
[[ "${EMBEDDED_NETWORK_ISOLATED:-0}" == 1 ]] || {
  printf 'provider_status=NETWORK_ISOLATION_REQUIRED\n' >&2
  exit 1
}

data_home=${XDG_DATA_HOME:-"${HOME:?HOME is required}/.local/share"}
provider_root="$data_home/zed-en-es-translator/embedded"
state="$provider_root/state.json"
[[ -f "$state" ]] || {
  printf 'provider_status=STATE_INVALID\n' >&2
  exit 1
}

current=$(jq -er '.current' "$state")
set_record="$provider_root/sets/$current.json"
[[ -f "$set_record" ]] || {
  printf 'provider_status=STATE_INVALID\n' >&2
  exit 1
}

object_path() {
  local role=$1
  jq -er --arg role "$role" '
    if $role == "runner" then .runner
    else .artifacts[] | select(.role == $role)
    end | "objects/\(.object_digest)/\(.installed_name)"
  ' "$set_record"
}

runner_relative=$(object_path runner)
model_relative=$(object_path model)
vocabulary_relative=$(object_path vocabulary)
shortlist_relative=$(object_path lexical_shortlist)
runner="$provider_root/$runner_relative"
[[ -x "$runner" ]] || {
  printf 'provider_status=STATE_INVALID\n' >&2
  exit 1
}

work="$root/target/embedded-benchmark-work-$$"
mkdir -p "$work"
trap 'rm -rf "$work"' EXIT
evidence=${EMBEDDED_EVIDENCE_PATH:-"$root/target/embedded-benchmark.ndjson"}
: >"$evidence"

segments_for_case() {
  local case_id=$1
  jq -c --arg id "$case_id" '
    .cases[] | select(.case_id == $id) |
    if has("segments") then .segments
    else . as $case | [([range(0; $case.repeat) | $case.unit] | join(" "))]
    end
  ' "$corpus"
}

run_case() {
  local case_id=$1 round=$2 repetition=$3 execution_class=$4 record=$5
  local segments request response metrics timer_pid child threads thread_peak=1
  segments=$(segments_for_case "$case_id")
  request=$(jq -cn --argjson segments "$segments" '{
    wire_version: 1,
    source_language: "en",
    target_language: "es",
    tone: "technical_neutral",
    preserve: ["markdown_structure", "code", "links"],
    segments: $segments
  }')
  response="$work/response.json"
  metrics="$work/metrics.tsv"

  (
    cd "$provider_root"
    /usr/bin/time -f '%e\t%U\t%S\t%M' -o "$metrics" \
      "$runner" \
      --model "$model_relative" \
      --vocabulary "$vocabulary_relative" \
      --lexical-shortlist "$shortlist_relative" \
      >"$response" 2>/dev/null <<<"$request"
  ) &
  timer_pid=$!
  while kill -0 "$timer_pid" 2>/dev/null; do
    for child in "$timer_pid" $(cat "/proc/$timer_pid/task/$timer_pid/children" 2>/dev/null || true); do
      [[ -r "/proc/$child/status" ]] || continue
      threads=$(awk '$1 == "Threads:" {print $2}' "/proc/$child/status")
      if [[ "$threads" =~ ^[0-9]+$ ]] && (( threads > thread_peak )); then
        thread_peak=$threads
      fi
    done
  done
  wait "$timer_pid" || {
    printf 'provider_status=RUNTIME_FAILED\n' >&2
    exit 1
  }

  jq -e --argjson count "$(jq length <<<"$segments")" '
    .wire_version == 1 and
    (.translations | type == "array" and length == $count) and
    (all(.translations[]; type == "string" and length > 0))
  ' "$response" >/dev/null || {
    printf 'provider_status=PROTOCOL_INVALID\n' >&2
    exit 1
  }
  [[ "$(jq -c '.translations' "$response")" != "$segments" ]] || {
    printf 'provider_status=QUALITY_FAILED\n' >&2
    exit 1
  }

  if (( record == 1 )); then
    IFS=$'\t' read -r elapsed user_cpu system_cpu max_rss_kib <"$metrics"
    elapsed_ms=$(awk -v value="$elapsed" 'BEGIN {printf "%.0f", value * 1000}')
    process_cpu_ms=$(awk -v user="$user_cpu" -v system="$system_cpu" 'BEGIN {printf "%.0f", (user + system) * 1000}')
    peak_rss_bytes=$((max_rss_kib * 1024))
    jq -cn \
      --arg manifest_digest "$current" \
      --arg fixture_set_version "$(jq -r '.fixture_set_version' "$corpus")" \
      --arg case_id "$case_id" \
      --arg execution_class "$execution_class" \
      --argjson round "$round" \
      --argjson repetition "$repetition" \
      --argjson elapsed_ms "$elapsed_ms" \
      --argjson process_cpu_ms "$process_cpu_ms" \
      --argjson peak_rss_bytes "$peak_rss_bytes" \
      --argjson thread_peak "$thread_peak" '{
        gate_id: "embedded-real-benchmark",
        manifest_digest: $manifest_digest,
        platform_class: "fedora-linux-x86_64",
        fixture_set_version: $fixture_set_version,
        case_id: $case_id,
        surface: "runner",
        execution_class: $execution_class,
        round: $round,
        repetition: $repetition,
        elapsed_ms: $elapsed_ms,
        process_cpu_ms: $process_cpu_ms,
        peak_rss_bytes: $peak_rss_bytes,
        thread_peak: $thread_peak,
        network_attempts: 0,
        locality: "offline_local",
        normalized_outcome: "passed",
        non_mutation: true,
        reviewer_status: "pending"
      }' >>"$evidence"
  fi
}

percentile_95() {
  local values rank
  mapfile -t values
  ((${#values[@]} > 0)) || {
    printf 'provider_status=EVIDENCE_INCOMPLETE\n' >&2
    exit 1
  }
  rank=$((((${#values[@]} * 95) + 99) / 100))
  printf '%s\n' "${values[@]}" | sort -n | sed -n "${rank}p"
}

enforce_budgets() {
  local transfer_bytes active_installed_bytes lifecycle_bytes peak_rss_bytes
  local thread_peak cold_readiness_ms warm_short_p95_ms warm_mixed_p95_ms
  local request_peak_ms

  transfer_bytes=$(jq '[.artifacts[].compressed_size] | add' "$manifest")
  active_installed_bytes=$(jq '
    .runner.installed_size + ([.artifacts[].installed_size] | add)
  ' "$set_record")
  lifecycle_bytes=$(du -sb --apparent-size "$provider_root" | awk '{print $1}')
  peak_rss_bytes=$(jq -s 'map(.peak_rss_bytes) | max' "$evidence")
  thread_peak=$(jq -s 'map(.thread_peak) | max' "$evidence")
  cold_readiness_ms=$(jq -s '
    map(select(.execution_class == "new_process") | .elapsed_ms) | max
  ' "$evidence")
  warm_short_p95_ms=$(jq -r --slurpfile corpus "$corpus" '
    ($corpus[0].cases
      | map(select(.class == "short_technical") | .case_id)) as $short_ids |
    select(.execution_class == "warm_provider") |
    select(.case_id as $case_id | $short_ids | index($case_id)) |
    .elapsed_ms
  ' "$evidence" | percentile_95)
  warm_mixed_p95_ms=$(jq -r '
    select(.execution_class == "warm_provider") | .elapsed_ms
  ' "$evidence" | percentile_95)
  request_peak_ms=$(jq -s 'map(.elapsed_ms) | max' "$evidence")

  (( transfer_bytes <= max_transfer_bytes )) || resource_blocked
  (( active_installed_bytes <= max_active_installed_bytes )) || resource_blocked
  (( lifecycle_bytes <= max_lifecycle_bytes )) || resource_blocked
  (( peak_rss_bytes <= max_peak_rss_bytes )) || resource_blocked
  (( thread_peak <= max_inference_threads )) || resource_blocked
  (( cold_readiness_ms <= max_cold_readiness_ms )) || resource_blocked
  (( warm_short_p95_ms <= max_warm_short_p95_ms )) || resource_blocked
  (( warm_mixed_p95_ms <= max_warm_mixed_p95_ms )) || resource_blocked
  (( request_peak_ms <= max_provider_request_ms )) || resource_blocked
  jq -e -s 'all(.[]; .network_attempts == 0)' "$evidence" >/dev/null \
    || resource_blocked
}

mapfile -t case_ids < <(jq -r '.cases[].case_id' "$corpus")
run_case "${case_ids[0]}" 0 1 new_process 1
for ((warmup = 1; warmup <= warmups; warmup++)); do
  run_case "${case_ids[0]}" 0 "$warmup" warm_provider 0
done

for ((round = 1; round <= rounds; round++)); do
  for case_id in "${case_ids[@]}"; do
    for ((repetition = 1; repetition <= repetitions; repetition++)); do
      run_case "$case_id" "$round" "$repetition" warm_provider 1
    done
  done
done

[[ "$(wc -l <"$evidence")" -eq 301 ]]
[[ "$(jq -s 'map(select(.execution_class == "new_process")) | length' "$evidence")" -eq 1 ]]
[[ "$(jq -s 'map(select(.execution_class == "warm_provider")) | length' "$evidence")" -eq 300 ]]
enforce_budgets
printf 'provider_status=benchmark_complete\n'
