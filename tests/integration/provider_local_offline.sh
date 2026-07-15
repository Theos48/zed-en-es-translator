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

operational_provider_write_fake_docker "$BIN_DIR" "$DOCKER_CALLS"
operational_provider_write_fake_http "$BIN_DIR" "$HTTP_CALLS"
export PATH="$BIN_DIR:$PATH"
export OPERATIONAL_PROVIDER_DOCKER_CALLS="$DOCKER_CALLS"
export OPERATIONAL_PROVIDER_HTTP_CALLS="$HTTP_CALLS"
export OPERATIONAL_PROVIDER_IMAGE_ID='libretranslate/libretranslate@sha256:1de2d7056bb8ad607a412f4563d9abe324ff632b43b5be9428bcc8e213aebb32'
export PROVIDER_CACHE_ROOT="$TEMP_ROOT/state"
export PROVIDER_SKIP_DISK_CHECK=1

SCRIPT="$ROOT/scripts/providers/libretranslate.sh"
"$SCRIPT" prepare >/dev/null
: >"$DOCKER_CALLS"
: >"$HTTP_CALLS"

"$SCRIPT" start >/dev/null
"$SCRIPT" verify >/dev/null
"$SCRIPT" stop >/dev/null

if grep -Eiq '(^|[[:space:]])(pull|build|https?://|latest)([[:space:]]|$)' "$DOCKER_CALLS"; then
  operational_provider_fail 'normal runtime attempted an online/acquisition operation'
fi
if grep -Ev '127\.0\.0\.1:5000/(health|translate)' "$HTTP_CALLS" | grep -q .; then
  operational_provider_fail 'normal runtime contacted a non-loopback target'
fi
grep -Fq 'PROVIDER_NETWORK_INTERNAL=true' "$DOCKER_CALLS" \
  || operational_provider_fail 'normal runtime did not enforce the internal network'

printf 'local provider no-egress contract ok\n'
