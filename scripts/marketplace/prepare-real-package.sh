#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/package.lock.json"
package_id=$(jq -r '.package_id' "$lock")
server_root="$root/target/marketplace-package-validation"
cache="$root/target/marketplace-model-cache"
package_root="$root/target/marketplace-real/$package_id"

fail() {
    printf 'marketplace real package: %s\n' "$1" >&2
    exit 1
}

command -v curl >/dev/null || fail 'curl is required for the maintainer-only public acquisition gate'
command -v jq >/dev/null || fail 'jq is required'
command -v zstd >/dev/null || fail 'zstd is required for the maintainer-only verification gate'
test -d "$server_root" || fail 'run make test-marketplace-package first'

verify_file() {
    local file=$1
    local expected_size=$2
    local expected_sha=$3
    test -f "$file" || return 1
    test ! -L "$file" || return 1
    test "$(stat -c '%s' "$file")" = "$expected_size" || return 1
    test "$(sha256sum "$file" | cut -d' ' -f1)" = "$expected_sha"
}

mkdir -p "$cache"
while IFS=$'\t' read -r url compressed_name compressed_size compressed_sha installed_name installed_size installed_sha; do
    compressed="$cache/$compressed_name"
    if ! verify_file "$compressed" "$compressed_size" "$compressed_sha"; then
        rm -f "$compressed" "$compressed.part"
        curl --fail --show-error --location --proto '=https' --tlsv1.2 \
            --connect-timeout 20 --max-time 300 --retry 2 \
            --output "$compressed.part" "$url"
        verify_file "$compressed.part" "$compressed_size" "$compressed_sha" \
            || fail 'a public model download did not match its locked identity'
        mv "$compressed.part" "$compressed"
        chmod 0644 "$compressed"
    fi
done < <(jq -r '.model_resources[] | [.url, .compressed_name, .compressed_size, .compressed_sha256, .installed_name, .installed_size, .installed_sha256] | @tsv' "$lock")

rm -rf "$package_root"
mkdir -p "$package_root/models"
cp -a "$server_root/." "$package_root/"

while IFS=$'\t' read -r compressed_name installed_name installed_size installed_sha; do
    destination="$package_root/models/$installed_name"
    zstd --quiet --decompress --stdout "$cache/$compressed_name" >"$destination.part"
    verify_file "$destination.part" "$installed_size" "$installed_sha" \
        || fail 'a decoded model did not match its locked installed identity'
    mv "$destination.part" "$destination"
    chmod 0644 "$destination"
done < <(jq -r '.model_resources[] | [.compressed_name, .installed_name, .installed_size, .installed_sha256] | @tsv' "$lock")

jq '{
  schema_version: 1,
  package_id,
  package_version,
  platform,
  source_language,
  target_language,
  wire_version: 1,
  state: "verified",
  artifacts: (
    [.server_archive.files[]
      | select(.role == "language_server" or .role == "native_runner")
      | {role, path, installed_size, installed_sha256, executable}]
    + [.model_resources[]
      | {role, path: ("models/" + .installed_name), installed_size, installed_sha256, executable: false}]
  )
}' "$lock" >"$package_root/installed.json.next"
mv "$package_root/installed.json.next" "$package_root/installed.json"
chmod 0600 "$package_root/installed.json"

printf 'marketplace_real_package=%s\n' "$package_root"
