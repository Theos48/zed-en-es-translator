#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/package.lock.json"
acquisition="$root/zed-extension/src/acquisition.rs"
process="$root/crates/translator-core/src/embedded_process.rs"

fail() {
    printf 'marketplace offline/privacy contract: %s\n' "$1" >&2
    exit 1
}

# Acquisition requests are fixed, public, content-free URLs compiled into the
# lock. Query strings/fragments could accidentally carry dynamic data.
jq -e '
  ([.server_archive.url] + [.model_resources[].url]) as $urls
  | ($urls | length) == 4
  and (all($urls[]; startswith("https://") and (contains("?") | not) and (contains("#") | not)))
' "$lock" >/dev/null || fail 'download URLs are not four fixed content-free HTTPS resources'

# The ready path verifies local bytes before the preparation lock/download
# path; the normal runner process receives a cleared, bounded environment.
rg -q 'if let Ok\(command\) = verified_command' "$acquisition" \
    || fail 'ready package is not checked before preparation'
rg -q '\.env_clear\(\)' "$process" || fail 'runner environment is not cleared'
test "$(rg -c '\.env\("(OMP|OPENBLAS)_NUM_THREADS", "4"\)' "$process")" -eq 2 \
    || fail 'runner thread caps drifted'
if rg -n 'source_text|translated_text|workspace|authorization|api[_-]?key' "$acquisition"; then
    fail 'acquisition surface contains translation, workspace or credential fields'
fi
rg -q 'lower.contains\("source_text="\)' "$root/zed-extension/src/diagnostics.rs" \
    || fail 'diagnostic redaction no longer covers source content fields'
rg -q 'lower.contains\("translated_text="\)' "$root/zed-extension/src/diagnostics.rs" \
    || fail 'diagnostic redaction no longer covers translation content fields'

printf 'marketplace offline/privacy contract: ok\n'
