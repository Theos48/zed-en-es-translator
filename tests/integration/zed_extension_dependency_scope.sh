#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if grep -q 'zed-extension' "$ROOT/Cargo.toml"; then
  printf 'zed-extension must remain isolated from the root workspace\n' >&2
  exit 1
fi

if [[ ! -f "$ROOT/zed-extension/Cargo.lock" ]]; then
  printf 'missing zed-extension/Cargo.lock\n' >&2
  exit 1
fi

if ! grep -q 'name = "zed_extension_api"' "$ROOT/zed-extension/Cargo.lock"; then
  printf 'zed_extension_api is missing from zed-extension/Cargo.lock\n' >&2
  exit 1
fi

if grep -q 'name = "zed_extension_api"' "$ROOT/Cargo.lock"; then
  printf 'zed_extension_api leaked into root Cargo.lock\n' >&2
  exit 1
fi

printf 'dependency scope ok\n'
