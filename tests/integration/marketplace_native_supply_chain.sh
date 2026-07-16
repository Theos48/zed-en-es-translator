#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/source.lock.json"
recipe="$root/ops/marketplace/build-recipe.txt"
source_root="$root/.cache/embedded-source/mozilla-translations"
build_root="$root/target/embedded-native-release"
runner="$build_root/translator-embedded-runtime"

test -x "$runner"
test "$(git -C "$source_root" rev-parse HEAD)" = "$(jq -r .source_commit "$lock")"
test -z "$(git -C "$source_root" status --porcelain --untracked-files=no)"

while IFS=$'\t' read -r path expected; do
    if direct=$(git -C "$source_root" rev-parse "HEAD:$path" 2>/dev/null); then
        test "$direct" = "$expected"
    else
        test "$(git -C "$source_root/$path" rev-parse HEAD)" = "$expected"
    fi
done < <(jq -r '.recursive_dependencies[] | [.path, .commit] | @tsv' "$lock")

test "$(sha256sum "$recipe" | cut -d' ' -f1)" = "$(jq -r .build_recipe_sha256 "$lock")"
test "$(sha256sum "$root/docker/rust-toolchain.Dockerfile" | cut -d' ' -f1)" = "$(jq -r .container_recipe_sha256 "$lock")"
test "$(sha256sum "$runner" | cut -d' ' -f1)" = "$(jq -r .binary_sha256 "$lock")"
test "$(stat -c '%s' "$runner")" = "$(jq -r .binary_size "$lock")"
test "$(jq -r .reproducible_build_status "$lock")" = verified
test "$(jq -r .reproducible_clean_builds "$lock")" -ge 2

file "$runner" | rg -q 'ELF 64-bit LSB pie executable, x86-64'
rg -q -- '-march=x86-64' "$build_root/build.ninja"
rg -q -- '-msse4.1' "$build_root/build.ninja"
rg -q -- 'USE_ONNX_SGEMM=1' "$build_root/build.ninja"
if rg -q -- '-march=native' "$build_root/build.ninja"; then
    printf 'marketplace_status=UNSAFE_NATIVE_CPU_FLAG\n' >&2
    exit 1
fi

mapfile -t expected_libraries < <(jq -r '.elf_dependency_allowlist[]' "$lock" | sort)
mapfile -t actual_libraries < <(
    readelf -d "$runner" |
        sed -n 's/.*Shared library: \[\(.*\)\]/\1/p' |
        sort
)
test "${actual_libraries[*]}" = "${expected_libraries[*]}"

if nm -D "$runner" | rg -q ' (connect|socket|listen|getaddrinfo|curl_|SSL_)'; then
    printf 'marketplace_status=NETWORK_SYMBOL_PRESENT\n' >&2
    exit 1
fi

set +e
invalid_output=$(
    printf '%s' '{"wire_version":1,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"],"segments":["Public synthetic text."]}' |
        "$runner"
)
invalid_status=$?
set -e
test "$invalid_status" -eq 1
test "$invalid_output" = '{"wire_version":1,"error":"INVALID_REQUEST"}'

printf 'marketplace_status=native_supply_chain_verified\n'
