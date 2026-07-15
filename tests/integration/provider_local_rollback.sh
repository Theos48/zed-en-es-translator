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
jq -e '
  .current_volume != null and
  .candidate_volume != null and
  .current_volume != .candidate_volume and
  (.current_image_reference | contains("@sha256:"))
' "$STATE_ROOT/state.json" >/dev/null \
  || operational_provider_fail 'initial promotion did not record immutable physical-slot roles'

if "$SCRIPT" update >"$TEMP_ROOT/unchanged.out" 2>&1; then
  operational_provider_fail 'unchanged lock unexpectedly passed the update review gate'
fi
grep -Fq 'UPDATE_REVIEW_REQUIRED' "$TEMP_ROOT/unchanged.out" \
  || operational_provider_fail 'unchanged update lacked the review-gate status'

jq '.current_lock_digest = "previous-reviewed-lock" | .lock_digest = "previous-reviewed-lock"' \
  "$STATE_ROOT/state.json" >"$TEMP_ROOT/state.changed"
mv "$TEMP_ROOT/state.changed" "$STATE_ROOT/state.json"
cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-failed-update.json"
export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' up --detach '
if "$SCRIPT" update >"$TEMP_ROOT/failed-update.out" 2>&1; then
  operational_provider_fail 'failed candidate unexpectedly promoted'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN
cmp -s "$TEMP_ROOT/before-failed-update.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'failed update changed current/previous state'

"$SCRIPT" update >/dev/null
jq -e '
  .active_slot == "current" and
  .offline_verified == true and
  .previous_identity != null and
  .previous_lock_digest == "previous-reviewed-lock" and
  .current_lock_digest != .previous_lock_digest and
  .current_volume != .previous_volume and
  .candidate_volume != .current_volume and
  .candidate_volume != .previous_volume and
  (.previous_image_reference | contains("@sha256:"))
' "$STATE_ROOT/state.json" >/dev/null \
  || operational_provider_fail 'successful update did not rotate verified state'

jq '.current_lock_digest = "second-reviewed-lock" | .lock_digest = "second-reviewed-lock"' \
  "$STATE_ROOT/state.json" >"$TEMP_ROOT/state.second-update"
mv "$TEMP_ROOT/state.second-update" "$STATE_ROOT/state.json"
cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-late-promotion-failure.json"
candidate_volume="$(jq -r .candidate_volume "$STATE_ROOT/state.json")"
: >"$DOCKER_CALLS"
export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' up --detach '
export OPERATIONAL_PROVIDER_DOCKER_FAIL_OCCURRENCE=3
if "$SCRIPT" update >"$TEMP_ROOT/late-promotion-failure.out" 2>&1; then
  operational_provider_fail 'late promotion failure unexpectedly succeeded'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_OCCURRENCE
cmp -s "$TEMP_ROOT/before-late-promotion-failure.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'late promotion failure changed current/previous metadata'
grep -Fq "volume rm zed-en-es-translator-providers-$candidate_volume" "$DOCKER_CALLS" \
  || operational_provider_fail 'update did not provision a fresh unused candidate slot'

"$SCRIPT" update >/dev/null
jq -e '
  .current_volume != .previous_volume and
  .candidate_volume != .current_volume and
  .candidate_volume != .previous_volume
' "$STATE_ROOT/state.json" >/dev/null \
  || operational_provider_fail 'repeated promotion produced overlapping slot roles'

previous_reference='libretranslate/libretranslate:v1.9.5@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
jq --arg reference "$previous_reference" '
  .previous_image_reference = $reference |
  .previous_identity = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
' "$STATE_ROOT/state.json" >"$TEMP_ROOT/state.previous-image"
mv "$TEMP_ROOT/state.previous-image" "$STATE_ROOT/state.json"
export OPERATIONAL_PROVIDER_IMAGE_ID_FROM_ARGUMENT=1

: >"$DOCKER_CALLS"
: >"$HTTP_CALLS"
"$SCRIPT" rollback >/dev/null
jq -e '.active_slot == "previous" and .offline_verified == true and .lifecycle_state == "ready"' \
  "$STATE_ROOT/state.json" >/dev/null \
  || operational_provider_fail 'rollback did not activate the verified previous slot'
grep -Fq "PROVIDER_IMAGE_REFERENCE=$previous_reference" "$DOCKER_CALLS" \
  || operational_provider_fail 'rollback did not select the recorded previous image reference'
if grep -Eiq '(^|[[:space:]])(pull|https?://|argos-net|download)([[:space:]]|$)' "$DOCKER_CALLS"; then
  operational_provider_fail 'rollback attempted acquisition or external contact'
fi
if grep -Ev '127\.0\.0\.1:5000/(health|translate)' "$HTTP_CALLS" | grep -q .; then
  operational_provider_fail 'rollback used a non-loopback HTTP target'
fi

"$SCRIPT" stop >/dev/null
: >"$DOCKER_CALLS"
"$SCRIPT" start >/dev/null
grep -Fq "PROVIDER_IMAGE_REFERENCE=$previous_reference" "$DOCKER_CALLS" \
  || operational_provider_fail 'start did not use the active previous image reference'

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-unavailable-image.json"
: >"$DOCKER_CALLS"
export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' image inspect '
if "$SCRIPT" rollback >"$TEMP_ROOT/unavailable-image.out" 2>&1; then
  operational_provider_fail 'rollback with unavailable prior image unexpectedly succeeded'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN
cmp -s "$TEMP_ROOT/before-unavailable-image.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'unavailable rollback image changed last-known-good state'
grep -Fq 'ROLLBACK_IMAGE_UNAVAILABLE' "$TEMP_ROOT/unavailable-image.out" \
  || operational_provider_fail 'unavailable rollback image lacked a safe recovery category'
if grep -Fq ' pull ' "$DOCKER_CALLS"; then
  operational_provider_fail 'rollback tried to pull an unavailable previous image'
fi

cp "$STATE_ROOT/state.json" "$TEMP_ROOT/before-failed-rollback.json"
export OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN=' up --detach '
if "$SCRIPT" rollback >"$TEMP_ROOT/failed-rollback.out" 2>&1; then
  operational_provider_fail 'failed rollback unexpectedly succeeded'
fi
unset OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN
unset OPERATIONAL_PROVIDER_IMAGE_ID_FROM_ARGUMENT
cmp -s "$TEMP_ROOT/before-failed-rollback.json" "$STATE_ROOT/state.json" \
  || operational_provider_fail 'failed rollback changed the last-known-good state'

printf 'local provider update and rollback contract ok\n'
