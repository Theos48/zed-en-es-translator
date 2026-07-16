#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/zed-source.lock.json"
evidence="$root/target/marketplace-zed-evidence/extension_host.rs"
extension_sources=(
    "$root/zed-extension/src/lib.rs"
    "$root/zed-extension/src/acquisition.rs"
    "$root/crates/translator-core/src/embedded_provider.rs"
)

fail() {
    printf 'marketplace removal contract: %s\n' "$1" >&2
    exit 1
}

commit=$(jq -r '.commit' "$lock")
path=$(jq -r '.path' "$lock")
expected_size=$(jq -r '.size' "$lock")
expected_sha=$(jq -r '.sha256' "$lock")
url="https://raw.githubusercontent.com/zed-industries/zed/$commit/$path"

mkdir -p "$(dirname "$evidence")"
if ! test -f "$evidence" \
    || ! test "$(stat -c '%s' "$evidence")" = "$expected_size" \
    || ! test "$(sha256sum "$evidence" | cut -d' ' -f1)" = "$expected_sha"; then
    curl --fail --silent --show-error --location --proto '=https' --tlsv1.2 \
        "$url" --output "$evidence.next"
    mv "$evidence.next" "$evidence"
fi
test "$(stat -c '%s' "$evidence")" = "$expected_size" || fail 'Zed evidence size drifted'
test "$(sha256sum "$evidence" | cut -d' ' -f1)" = "$expected_sha" \
    || fail 'Zed evidence hash drifted'

rg -q 'pub fn uninstall_extension' "$evidence" || fail 'Zed uninstall entry point is absent'
rg -q 'installed_dir.join\(extension_id.as_ref\(\)\)' "$evidence" \
    || fail 'Zed no longer targets the installed extension directory'
rg -q 'wasm_host.work_dir.join\(extension_id.as_ref\(\)\)' "$evidence" \
    || fail 'Zed no longer targets the extension work directory'
test "$(sed -n '/pub fn uninstall_extension/,/pub fn install_dev_extension/p' "$evidence" | rg -c '\.remove_dir\(')" -ge 2 \
    || fail 'Zed uninstall no longer removes both owned directories'
rg -q 'for i in 0\.\.3' "$evidence" || fail 'Zed work-directory removal retry changed'

if rg -n 'dirs::|XDG_|\.local/share|/usr/|/opt/|systemctl|sudo' "${extension_sources[@]}"; then
    fail 'product runtime creates state outside Zed extension ownership'
fi

printf 'marketplace_zed_removal_commit=%s\n' "$commit"
printf 'marketplace_removal_contract=ok\n'
