#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
package_id=en-es-translator-0.1.0-linux-x86_64
stage="$root/target/marketplace-package-root/$package_id"
output_root="$root/target/marketplace-package"
archive="$output_root/$package_id.tar.gz"

server="$root/target/release/translator-lsp"
runner="$root/target/embedded-native-release/translator-embedded-runtime"
test -x "$server"
test -x "$runner"

rm -rf "$root/target/marketplace-package-root"
mkdir -p "$stage/bin" "$stage/LICENSES" "$output_root"
cp "$server" "$stage/bin/translator-lsp"
cp "$runner" "$stage/bin/translator-embedded-runtime"
cp "$root/ops/marketplace/licenses/THIRD_PARTY_NOTICES.md" "$stage/LICENSES/THIRD_PARTY_NOTICES.md"
cp "$root/ops/marketplace/licenses/MPL-2.0.txt" "$stage/LICENSES/MPL-2.0.txt"
cp "$root/ops/marketplace/licenses/SOURCE.md" "$stage/LICENSES/SOURCE.md"
chmod 0755 "$stage/bin/translator-lsp" "$stage/bin/translator-embedded-runtime"
chmod 0644 "$stage/LICENSES/THIRD_PARTY_NOTICES.md" "$stage/LICENSES/MPL-2.0.txt" "$stage/LICENSES/SOURCE.md"

rm -f "$archive"
tar --sort=name --mtime='@0' --owner=0 --group=0 --numeric-owner \
    --format=posix --pax-option=delete=atime,delete=ctime \
    -C "$stage" -czf "$archive" .

printf 'marketplace_package=%s\n' "$archive"
