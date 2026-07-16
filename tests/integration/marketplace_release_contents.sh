#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/package.lock.json"
source_lock="$root/ops/marketplace/source.lock.json"
stage="$root/target/marketplace-package-validation"

fail() {
    printf 'marketplace release contents: %s\n' "$1" >&2
    exit 1
}

test -d "$stage" || fail 'validated release package is missing'

active_bytes=$(jq '[.server_archive.files[].installed_size] + [.model_resources[].installed_size] | add' "$lock")
active_budget=$(jq '.budgets.maximum_active_installed_bytes' "$lock")
test "$active_bytes" -lt "$active_budget" || fail 'active package exceeds 128 MiB'

mapfile -t actual_executables < <(find "$stage" -type f -perm /111 -printf '%P\n' | sort)
test "${actual_executables[*]}" = 'bin/translator-embedded-runtime bin/translator-lsp' \
    || fail 'release executable allowlist drifted'

while IFS= read -r path; do
    test -s "$stage/$path" || fail 'a required license/notice/source file is absent'
    test ! -x "$stage/$path" || fail 'a license/notice/source file is executable'
done < <(jq -r '.license_bundle.required_paths[]' "$lock")

rg -q 'Mozilla Public License Version 2.0' "$stage/LICENSES/MPL-2.0.txt" \
    || fail 'MPL-2.0 text is incomplete'
source_commit=$(jq -r '.source_commit' "$source_lock")
rg -q "$source_commit" "$stage/LICENSES/SOURCE.md" \
    || fail 'corresponding-source instructions do not identify the locked source'
rg -q 'Mozilla Translations|Bergamot|Marian' "$stage/LICENSES/THIRD_PARTY_NOTICES.md" \
    || fail 'native provenance notice is incomplete'
rg -q 'MIT License' "$root/zed-extension/LICENSE" || fail 'extension MIT license is absent'

if ldd "$stage/bin/translator-embedded-runtime" | rg -qi 'curl|ssl|crypto'; then
    fail 'translation runner unexpectedly links a network/TLS client'
fi

printf 'marketplace_active_bytes=%s\n' "$active_bytes"
printf 'marketplace_release_contents=ok\n'
