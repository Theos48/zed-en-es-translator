#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=tests/integration/lib/operational_provider_helpers.sh
# shellcheck disable=SC1091 # Resolved from the runtime repository root.
source "$ROOT/tests/integration/lib/operational_provider_helpers.sh"

TEMP_ROOT="$(operational_provider_temp_dir)"
trap 'rm -rf "$TEMP_ROOT"' EXIT
BIN_DIR="$TEMP_ROOT/bin"
DOCKER_CALLS="$TEMP_ROOT/docker.calls"
HTTP_CALLS="$TEMP_ROOT/http.calls"
STATE_ROOT="$TEMP_ROOT/state"

operational_provider_write_fake_docker "$BIN_DIR" "$DOCKER_CALLS"
operational_provider_write_fake_http "$BIN_DIR" "$HTTP_CALLS"

export PATH="$BIN_DIR:$PATH"
export OPERATIONAL_PROVIDER_DOCKER_CALLS="$DOCKER_CALLS"
export OPERATIONAL_PROVIDER_HTTP_CALLS="$HTTP_CALLS"
export OPERATIONAL_PROVIDER_IMAGE_ID='libretranslate/libretranslate@sha256:1de2d7056bb8ad607a412f4563d9abe324ff632b43b5be9428bcc8e213aebb32'
export PROVIDER_CACHE_ROOT="$STATE_ROOT"
export PROVIDER_SKIP_DISK_CHECK=1

SCRIPT="$ROOT/scripts/providers/libretranslate.sh"

umask 077
"$SCRIPT" prepare >/dev/null
jq -e '.active_slot == "current" and .offline_verified == true and .lifecycle_state == "ready"' \
  "$STATE_ROOT/state.json" >/dev/null \
  || operational_provider_fail 'prepare did not promote a verified current slot'
[[ "$(stat -c '%a' "$STATE_ROOT/artifacts")" == 755 ]] \
  || operational_provider_fail 'verified artifact directory is not container-readable'
while IFS= read -r model; do
  [[ "$(stat -c '%a' "$STATE_ROOT/artifacts/$model")" == 644 ]] \
    || operational_provider_fail 'verified model artifact is not container-readable'
done < <(jq -r '.models[].filename' "$ROOT/ops/providers/libretranslate/provider.lock")
grep -Fq -- '--entrypoint /app/venv/bin/python' \
  "$DOCKER_CALLS" \
  || operational_provider_fail 'model install did not use the pinned image venv interpreter'
grep -Fq -- 'from argostranslate.package import install_from_path' \
  "$DOCKER_CALLS" \
  || operational_provider_fail 'model install did not use the offline package-file API'
grep -Fq -- '--data {"q":"Read the docs.","source":"en","target":"es","format":"text"}' \
  "$HTTP_CALLS" \
  || operational_provider_fail 'readiness probe did not use the exact LibreTranslate object payload'
if grep -Fq -- '--data [{' "$HTTP_CALLS"; then
  operational_provider_fail 'readiness probe used an unsupported array envelope'
fi

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-invalid-probe.json"
export OPERATIONAL_PROVIDER_HTTP_RESPONSE='{"unexpected":"PRIVATE_RAW_RESPONSE"}'
if "$SCRIPT" verify >"$TEMP_ROOT/invalid-probe.out" 2>&1; then
  operational_provider_fail 'invalid translation probe unexpectedly passed readiness'
fi
unset OPERATIONAL_PROVIDER_HTTP_RESPONSE
[[ "$(<"$TEMP_ROOT/invalid-probe.out")" == 'provider_status=PROVIDER_UNAVAILABLE' ]] \
  || operational_provider_fail 'invalid readiness response was not normalized'
cmp -s "$TEMP_ROOT/before-invalid-probe.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'invalid readiness response changed provider state'

"$SCRIPT" start >/dev/null
"$SCRIPT" start >/dev/null
status_output="$("$SCRIPT" status)"
[[ "$(sed -n '1p' <<<"$status_output")" == 'provider_status=READY' ]] \
  || operational_provider_fail 'status vocabulary is not normalized'
for safe_field in provider_active_slot provider_identity provider_lock_digest provider_offline_verified; do
  grep -Eq "^${safe_field}=[A-Za-z0-9:_-]+$" <<<"$status_output" \
    || operational_provider_fail "status omitted safe field: $safe_field"
done
[[ "$(wc -c <<<"$status_output")" -le 256 ]] \
  || operational_provider_fail 'status output is not bounded'
operational_provider_assert_redacted "$status_output" "$ROOT" 'http://127.0.0.1:5000'
"$SCRIPT" verify >/dev/null
"$SCRIPT" stop >/dev/null
"$SCRIPT" stop >/dev/null

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-port-conflict.json"
export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' up --detach '
if "$SCRIPT" start >"$TEMP_ROOT/port-conflict.out" 2>&1; then
  operational_provider_fail 'port-conflict start unexpectedly succeeded'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN
cmp -s "$TEMP_ROOT/before-port-conflict.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'port conflict changed verified provider state'
operational_provider_assert_redacted \
  "$(<"$TEMP_ROOT/port-conflict.out")" \
  "$ROOT" 'http://127.0.0.1:5000' 'controlled-ok'
[[ "$(<"$TEMP_ROOT/port-conflict.out")" == 'provider_status=PROVIDER_START_FAILED' ]] \
  || operational_provider_fail 'port conflict output is not a stable safe status'
[[ "$(wc -c <"$TEMP_ROOT/port-conflict.out")" -le 64 ]] \
  || operational_provider_fail 'lifecycle failure output is not bounded'

set +e
"$SCRIPT" unsupported-operation >"$TEMP_ROOT/invalid-operation.out" 2>&1
invalid_operation_status=$?
set -e
[[ "$invalid_operation_status" -eq 2 ]] \
  || operational_provider_fail 'invalid operation did not use exit status 2'
[[ "$(<"$TEMP_ROOT/invalid-operation.out")" == 'provider_status=INVALID_OPERATION' ]] \
  || operational_provider_fail 'invalid operation output is not normalized'

if grep -Eiq 'sudo|dnf|rpm|flatpak|systemctl|docker[[:space:]]+(system|volume|network)[[:space:]]+prune' \
  "$ROOT/scripts/providers/libretranslate.sh"; then
  operational_provider_fail 'lifecycle script contains host/global mutation commands'
fi
operational_provider_assert_no_host_mutation_commands "$DOCKER_CALLS"

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-failure.json"
export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' up --detach '
if "$SCRIPT" prepare >"$TEMP_ROOT/failure.out" 2>&1; then
  operational_provider_fail 'interrupted preparation unexpectedly succeeded'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN

before_active="$(jq -r .active_slot "$TEMP_ROOT/before-failure.json")"
after_active="$(jq -r .active_slot "$STATE_ROOT/state.json")"
[[ "$before_active" == "$after_active" ]] \
  || operational_provider_fail 'failed preparation changed the current slot'
operational_provider_assert_redacted \
  "$(<"$TEMP_ROOT/failure.out")" \
  "$ROOT" 'http://127.0.0.1:5000' 'controlled-ok'

printf 'local provider controlled lifecycle ok\n'
