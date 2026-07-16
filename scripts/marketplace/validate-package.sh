#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
package_id=en-es-translator-0.1.0-linux-x86_64
archive="$root/target/marketplace-package/$package_id.tar.gz"
validation="$root/target/marketplace-package-validation"
lock="$root/ops/marketplace/package.lock.json"

test -f "$archive"
mapfile -t members < <(tar -tzf "$archive" | sed 's#^\./##' | sed '/^$/d' | sort)
expected=(
    LICENSES/
    LICENSES/MPL-2.0.txt
    LICENSES/SOURCE.md
    LICENSES/THIRD_PARTY_NOTICES.md
    bin/
    bin/translator-embedded-runtime
    bin/translator-lsp
)
test "${members[*]}" = "${expected[*]}"
if tar -tvzf "$archive" | awk '$1 !~ /^[-d]/ {exit 1}'; then
    :
else
    printf 'marketplace package contains a non-file/non-directory entry\n' >&2
    exit 1
fi

rm -rf "$validation"
mkdir -p "$validation"
tar -xzf "$archive" -C "$validation"
mapfile -t executables < <(find "$validation" -type f -perm /111 -printf '%P\n' | sort)
test "${executables[*]}" = "bin/translator-embedded-runtime bin/translator-lsp"

if test -f "$lock"; then
    while IFS=$'\t' read -r path expected_size expected_sha executable; do
        file="$validation/$path"
        test -f "$file"
        test ! -L "$file"
        test "$(stat -c '%s' "$file")" = "$expected_size"
        test "$(sha256sum "$file" | cut -d' ' -f1)" = "$expected_sha"
        if test "$executable" = true; then
            test -x "$file"
        else
            test ! -x "$file"
        fi
    done < <(jq -r '.server_archive.files[] | [.path, .installed_size, .installed_sha256, .executable] | @tsv' "$lock")
fi

printf 'marketplace_status=release_archive_verified\n'
