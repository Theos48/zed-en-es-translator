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
"$SCRIPT" prepare >/dev/null
"$SCRIPT" stop >/dev/null
[[ -f "$STATE_ROOT/state.json" ]] \
  || operational_provider_fail 'ordinary stop removed persistent state'

unsafe_cache_root="/tmp/not-operational-provider-cache-$$"
if PROVIDER_CACHE_ROOT="$unsafe_cache_root" \
  "$SCRIPT" clean remove-provider-data >"$TEMP_ROOT/unsafe-cache.out" 2>&1; then
  operational_provider_fail 'cleanup accepted an unsafe cache-root override'
fi
grep -Fq 'UNSAFE_RUNTIME_OVERRIDE' "$TEMP_ROOT/unsafe-cache.out" \
  || operational_provider_fail 'unsafe cache override lacked a normalized status'
operational_provider_assert_redacted "$(<"$TEMP_ROOT/unsafe-cache.out")" "$unsafe_cache_root"

if PROVIDER_CACHE_ROOT="$ROOT/provider-cache/libretranslate" \
  PROVIDER_SKIP_DISK_CHECK=1 "$SCRIPT" status >"$TEMP_ROOT/unsafe-disk-skip.out" 2>&1; then
  operational_provider_fail 'production cache accepted the test-only disk bypass'
fi
grep -Fq 'UNSAFE_RUNTIME_OVERRIDE' "$TEMP_ROOT/unsafe-disk-skip.out" \
  || operational_provider_fail 'unsafe disk bypass lacked a normalized status'

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-corrupt-status.json"
jq '.lifecycle_state = "/home/PRIVATE_SOURCE" | .current_identity = "PRIVATE_TRANSLATION"' \
  "$STATE_ROOT/state.json" >"$TEMP_ROOT/state.corrupt"
mv "$TEMP_ROOT/state.corrupt" "$STATE_ROOT/state.json"
if "$SCRIPT" status >"$TEMP_ROOT/corrupt-status.out" 2>&1; then
  operational_provider_fail 'status accepted corrupt unbounded state'
fi
grep -Fq 'STATE_INVALID' "$TEMP_ROOT/corrupt-status.out" \
  || operational_provider_fail 'corrupt state lacked a normalized status'
operational_provider_assert_redacted \
  "$(<"$TEMP_ROOT/corrupt-status.out")" '/home/PRIVATE_SOURCE' 'PRIVATE_TRANSLATION'
cp "$TEMP_ROOT/before-corrupt-status.json" "$STATE_ROOT/state.json"

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-missing-docker-stop.json"
if DOCKER_BIN=provider-docker-does-not-exist \
  "$SCRIPT" stop >"$TEMP_ROOT/missing-docker-stop.out" 2>&1; then
  operational_provider_fail 'stop without Docker unexpectedly succeeded'
fi
grep -Fq 'PREREQUISITE_MISSING' "$TEMP_ROOT/missing-docker-stop.out" \
  || operational_provider_fail 'stop without Docker lacked a safe prerequisite status'
cmp -s "$TEMP_ROOT/before-missing-docker-stop.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'failed stop changed provider metadata'

export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' down --remove-orphans '
if "$SCRIPT" stop >"$TEMP_ROOT/failed-stop.out" 2>&1; then
  operational_provider_fail 'failed Docker stop unexpectedly succeeded'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN
grep -Fq 'PROVIDER_STOP_FAILED' "$TEMP_ROOT/failed-stop.out" \
  || operational_provider_fail 'failed stop lacked a safe recovery status'

set +e
"$SCRIPT" clean >"$TEMP_ROOT/missing-token.out" 2>&1
missing_token_status=$?
"$SCRIPT" clean wrong-token >"$TEMP_ROOT/wrong-token.out" 2>&1
wrong_token_status=$?
set -e
[[ "$missing_token_status" -eq 1 && "$wrong_token_status" -eq 1 ]] \
  || operational_provider_fail 'rejected cleanup did not use exit status 1'
[[ "$(<"$TEMP_ROOT/missing-token.out")" == 'provider_status=CLEAN_CONFIRMATION_REQUIRED' ]] \
  || operational_provider_fail 'cleanup confirmation output is not normalized'
[[ "$(<"$TEMP_ROOT/wrong-token.out")" == 'provider_status=CLEAN_CONFIRMATION_REQUIRED' ]] \
  || operational_provider_fail 'wrong cleanup token output is not normalized'
[[ -f "$STATE_ROOT/state.json" ]] \
  || operational_provider_fail 'rejected cleanup removed persistent state'

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-missing-docker-clean.json"
if DOCKER_BIN=provider-docker-does-not-exist \
  "$SCRIPT" clean remove-provider-data >"$TEMP_ROOT/missing-docker-clean.out" 2>&1; then
  operational_provider_fail 'cleanup without Docker unexpectedly succeeded'
fi
cmp -s "$TEMP_ROOT/before-missing-docker-clean.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'cleanup without Docker removed metadata'

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-failed-removal.json"
export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' volume rm zed-en-es-translator-providers-candidate '
if "$SCRIPT" clean remove-provider-data >"$TEMP_ROOT/failed-removal.out" 2>&1; then
  operational_provider_fail 'cleanup with failed resource removal unexpectedly succeeded'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN
grep -Fq 'CLEAN_FAILED' "$TEMP_ROOT/failed-removal.out" \
  || operational_provider_fail 'failed resource cleanup lacked a safe recovery status'
cmp -s "$TEMP_ROOT/before-failed-removal.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'incomplete resource cleanup removed operational metadata'

: >"$DOCKER_CALLS"
"$SCRIPT" clean remove-provider-data >/dev/null
[[ ! -e "$STATE_ROOT" ]] \
  || operational_provider_fail 'confirmed cleanup preserved provider metadata'

for resource in \
  zed-en-es-translator-providers-candidate \
  zed-en-es-translator-providers-current \
  zed-en-es-translator-providers-previous \
  zed-en-es-translator-providers-runtime \
  zed-en-es-translator-providers-edge
do
  grep -Fq "$resource" "$DOCKER_CALLS" \
    || operational_provider_fail "cleanup did not scope resource: $resource"
done
operational_provider_assert_no_host_mutation_commands "$DOCKER_CALLS"
if grep -Eiq '(^|[[:space:]])(prune|sudo|dnf|rpm|flatpak|systemctl)([[:space:]]|$)' "$DOCKER_CALLS"; then
  operational_provider_fail 'cleanup attempted a global or host mutation'
fi

printf 'local provider cleanup isolation contract ok\n'
