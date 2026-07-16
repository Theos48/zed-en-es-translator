#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/providers/embedded/source.lock.json"
recipe="$root/ops/providers/embedded/build-recipe.txt"
source_root="$root/.cache/embedded-source/mozilla-translations"
build_root="$root/target/embedded-native-release"
runner="$build_root/translator-embedded-runtime"

[[ -x "$runner" ]]
[[ "$(git -C "$source_root" rev-parse HEAD)" == "$(jq -r .source_commit "$lock")" ]]
[[ -z "$(git -C "$source_root" status --porcelain --untracked-files=no)" ]]

while IFS=$'\t' read -r path expected; do
  [[ "$(git -C "$source_root" rev-parse "HEAD:$path")" == "$expected" ]]
done < <(jq -r '.recursive_dependencies[] | [.path, .commit] | @tsv' "$lock")

[[ "$(sha256sum "$recipe" | cut -d' ' -f1)" == "$(jq -r .build_recipe_sha256 "$lock")" ]]
[[ "$(sha256sum "$root/docker/rust-toolchain.Dockerfile" | cut -d' ' -f1)" == "$(jq -r .container_recipe_sha256 "$lock")" ]]
[[ "$(sha256sum "$runner" | cut -d' ' -f1)" == "$(jq -r .binary_sha256 "$lock")" ]]
[[ "$(stat -c '%s' "$runner")" == "$(jq -r .binary_size "$lock")" ]]
[[ "$(jq -r .reproducible_build_status "$lock")" == "verified" ]]
[[ "$(jq -r .reproducible_clean_builds "$lock")" -ge 2 ]]

file "$runner" | rg -q 'ELF 64-bit LSB pie executable, x86-64'
rg -q -- '-march=x86-64' "$build_root/build.ninja"
rg -q -- '-msse4.1' "$build_root/build.ninja"
if rg -q -- '-march=native' "$build_root/build.ninja"; then
  printf 'provider_status=UNSAFE_NATIVE_CPU_FLAG\n' >&2
  exit 1
fi

mapfile -t expected_libraries < <(jq -r '.elf_dependency_allowlist[]' "$lock" | sort)
mapfile -t actual_libraries < <(
  readelf -d "$runner" |
    sed -n 's/.*Shared library: \[\(.*\)\]/\1/p' |
    sort
)
[[ "${actual_libraries[*]}" == "${expected_libraries[*]}" ]]

if nm -D "$runner" | rg -q ' (connect|socket|listen|getaddrinfo|curl_|SSL_)'; then
  printf 'provider_status=NETWORK_SYMBOL_PRESENT\n' >&2
  exit 1
fi

set +e
invalid_output=$(
  printf '%s' '{"wire_version":1,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"],"segments":["Public synthetic text."]}' |
    "$runner"
)
invalid_status=$?
set -e
[[ "$invalid_status" -eq 1 ]]
[[ "$invalid_output" == '{"wire_version":1,"error":"INVALID_REQUEST"}' ]]

printf 'provider_status=native_supply_chain_verified\n'
