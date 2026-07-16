#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/package.lock.json"
manifest="$root/zed-extension/extension.toml"
download_root="$root/target/marketplace-public-release-check"

fail() {
    printf 'marketplace public release check: %s\n' "$1" >&2
    exit 1
}

version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$manifest")
package_version=$(jq -r '.package_version' "$lock")
url=$(jq -r '.server_archive.url' "$lock")
tag=$(sed -n 's#.*releases/download/\([^/]*\)/.*#\1#p' <<<"$url")
asset=$(basename "$url")
repository=$(sed -n 's/^repository = "\([^"]*\)"/\1/p' "$manifest")

test "$version" = "$package_version" || fail 'extension/package versions differ'
test "$tag" = "v$version" || fail 'release tag does not match extension version'
test "$asset" = "en-es-translator-$version-linux-x86_64.tar.gz" \
    || fail 'release asset name does not match the package identity'
test "$url" = "$repository/releases/download/$tag/$asset" \
    || fail 'release URL does not match the extension repository'

git ls-remote --exit-code "$repository.git" "refs/tags/$tag" >/dev/null \
    || fail 'public project tag is absent'

rm -rf "$download_root"
mkdir -p "$download_root/extracted"
curl --fail --show-error --location --proto '=https' --tlsv1.2 \
    --connect-timeout 20 --max-time 300 --output "$download_root/$asset" "$url" \
    || fail 'public release asset is absent'

mapfile -t members < <(tar -tzf "$download_root/$asset" | sed 's#^\./##' | sed '/^$/d' | sort)
expected=(
    LICENSES/
    LICENSES/MPL-2.0.txt
    LICENSES/SOURCE.md
    LICENSES/THIRD_PARTY_NOTICES.md
    bin/
    bin/translator-embedded-runtime
    bin/translator-lsp
)
test "${members[*]}" = "${expected[*]}" || fail 'public archive allowlist drifted'
tar -xzf "$download_root/$asset" -C "$download_root/extracted"

while IFS=$'\t' read -r path expected_size expected_sha executable; do
    file="$download_root/extracted/$path"
    test -f "$file" && test ! -L "$file" || fail 'public asset contains a missing/unsafe file'
    test "$(stat -c '%s' "$file")" = "$expected_size" || fail 'public file size drifted'
    test "$(sha256sum "$file" | cut -d' ' -f1)" = "$expected_sha" \
        || fail 'public file hash drifted'
    if test "$executable" = true; then
        test -x "$file" || fail 'public runtime lost its executable mode'
    else
        test ! -x "$file" || fail 'public data unexpectedly became executable'
    fi
done < <(jq -r '.server_archive.files[] | [.path, .installed_size, .installed_sha256, .executable] | @tsv' "$lock")

printf 'marketplace_public_tag=%s\n' "$tag"
printf 'marketplace_public_asset=%s\n' "$asset"
printf 'marketplace_public_release_check=ok\n'
