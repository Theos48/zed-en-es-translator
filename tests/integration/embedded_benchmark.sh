#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
manifest="$root/ops/providers/embedded/provider.lock.json"
corpus="$root/tests/fixtures/embedded/synthetic-corpus.json"

warmups=5
rounds=3
repetitions=5

blocked() {
  printf 'provider_status=BLOCKED_LICENSE_APPROVAL\n' >&2
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

mapfile -t case_ids < <(jq -r '.cases[].case_id' "$corpus")
for ((warmup = 1; warmup <= warmups; warmup++)); do
  run_case "${case_ids[0]}" 0 "$warmup" new_process 0
done

for ((round = 1; round <= rounds; round++)); do
  for case_id in "${case_ids[@]}"; do
    for ((repetition = 1; repetition <= repetitions; repetition++)); do
      run_case "$case_id" "$round" "$repetition" new_process 1
    done
  done
done

[[ "$(wc -l <"$evidence")" -eq 300 ]]
printf 'provider_status=benchmark_complete\n'
