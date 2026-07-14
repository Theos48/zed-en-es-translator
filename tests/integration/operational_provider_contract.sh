#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
COMPOSE="$ROOT/ops/providers/libretranslate/compose.yaml"
LOCK="$ROOT/ops/providers/libretranslate/provider.lock"
SCRIPT="$ROOT/scripts/providers/libretranslate.sh"
LOOPBACK_PROXY="$ROOT/ops/providers/libretranslate/loopback_proxy.py"

fail() {
  printf 'local provider static contract failed: %s\n' "$1" >&2
  exit 1
}

[[ -f "$COMPOSE" ]] || fail 'compose.yaml is missing'
[[ -f "$LOCK" ]] || fail 'provider.lock is missing'
[[ -f "$SCRIPT" ]] || fail 'lifecycle script is missing'
[[ -f "$LOOPBACK_PROXY" ]] || fail 'fixed loopback relay is missing'

jq -e '
  .schema_version == 1 and
  .image.reference == "libretranslate/libretranslate:v1.9.6@sha256:1de2d7056bb8ad607a412f4563d9abe324ff632b43b5be9428bcc8e213aebb32" and
  .package_index.revision == "ff90de60728f7c1338ff6b75974e4c89b2442d22" and
  (.models | length == 2) and
  ([.models[].sha256] | index("d698d0ef87ad70d5d184b7fa6965905bf4368f09a2bb9ffb165a79bac96af0c4") != null) and
  ([.models[].sha256] | index("c54df2b62fceaf54a3ce5d97db6bf56efd7940063329f6778f4212d2acb370d4") != null) and
  .license.redistribution == "forbidden-until-resolved"
' "$LOCK" >/dev/null || fail 'provider.lock does not contain the reviewed immutable inputs'

# shellcheck disable=SC2016 # Compose interpolation must remain literal.
for needle in \
  'libretranslate/libretranslate:v1.9.6@sha256:1de2d7056bb8ad607a412f4563d9abe324ff632b43b5be9428bcc8e213aebb32' \
  'pull_policy: never' \
  '127.0.0.1:5000:5000' \
  'LT_LOAD_ONLY: en,es' \
  'LT_DISABLE_WEB_UI: "true"' \
  'LT_DISABLE_FILES_TRANSLATION: "true"' \
  'LT_UPDATE_MODELS: "false"' \
  'test: ["CMD", "/app/venv/bin/python"' \
  'internal: ${PROVIDER_NETWORK_INTERNAL:-true}' \
  'cpus: "4.0"' \
  'memory: 4G' \
  'candidate:' \
  'current:' \
  'previous:'
do
  grep -Fq -- "$needle" "$COMPOSE" || fail "compose contract is missing: $needle"
done

grep -Fq 'COMPOSE_PROJECT_NAME=zed-en-es-translator-providers' "$SCRIPT" \
  || fail 'fixed Compose project identity is missing'
grep -Fq 'READINESS_TIMEOUT_SECONDS=120' "$SCRIPT" \
  || fail 'fixed readiness budget is missing'

compose_config="$(docker compose --file "$COMPOSE" config --format json)"
jq -e '
  ((.services.libretranslate.ports // []) | length) == 0 and
  ((.services.libretranslate.networks | keys) == ["runtime"]) and
  (.services.loopback.ports == [{"mode":"ingress","target":5000,"published":"5000","protocol":"tcp","host_ip":"127.0.0.1"}]) and
  ((.services.loopback.networks | keys | sort) == ["edge", "runtime"]) and
  .networks.runtime.internal == true and
  ((.networks.edge.internal // false) == false)
' <<<"$compose_config" >/dev/null \
  || fail 'compose topology does not isolate the provider behind the loopback relay'

for proxy_needle in \
  'UPSTREAM_HOST = "libretranslate"' \
  'UPSTREAM_PORT = 5000' \
  'MAX_REQUEST_BYTES = 128 * 1024' \
  'MAX_RESPONSE_BYTES = 40 * 1024' \
  'def log_message'
do
  grep -Fq -- "$proxy_needle" "$LOOPBACK_PROXY" \
    || fail "loopback relay contract is missing: $proxy_needle"
done

printf 'local provider static contract ok\n'
