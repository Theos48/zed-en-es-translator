#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
manifest="$root/zed-extension/extension.toml"
lib="$root/zed-extension/src/lib.rs"

test "$(rg -c '^\[language_servers\.en-es-translator\]$' "$manifest")" -eq 1
if rg -n '^\[context_servers\.|binary_path|provider[._]|api_key|base_url|TRANSLATOR_PROVIDER|embedded_local|make zed-|docker|cargo' \
    "$manifest" "$lib"; then
    printf 'marketplace no-setup contract: manual setup surface remains\n' >&2
    exit 1
fi

rg -q 'LanguageServerInstallationStatus::CheckingForUpdate' "$lib"
rg -q 'LanguageServerInstallationStatus::Downloading' "$lib"
rg -q 'join\("packages"\)' "$root/zed-extension/src/acquisition.rs"

printf 'marketplace no-setup contract: ok\n'
