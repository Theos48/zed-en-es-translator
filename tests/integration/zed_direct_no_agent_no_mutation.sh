#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$(mktemp)"
trap 'rm -f "$FIXTURE"' EXIT
printf 'Read the docs.\n' >"$FIXTURE"
BEFORE="$(sha256sum "$FIXTURE" | cut -d' ' -f1)"

DRY_RUN="$(make -C "$ROOT" -n zed-direct-prepare)"
if ! grep -q 'translator-lsp' <<<"$DRY_RUN" || ! grep -q 'prepare-direct.sh' <<<"$DRY_RUN"; then
  printf 'direct target does not prepare translator-lsp\n' >&2
  exit 1
fi
if grep -Eiq 'translator-mcp|context_server|agent panel|agent profile' <<<"$DRY_RUN"; then
  printf 'direct target invokes Agent or MCP compatibility infrastructure\n' >&2
  exit 1
fi

TEST_NAMES="$(sed -n 's/^DIRECT_ZED_TESTS[[:space:]]*:=[[:space:]]*//p' "$ROOT/Makefile")"
if [[ " $TEST_NAMES " != *" no_agent_no_mutation "* ]]; then
  printf 'direct test target omits the no-Agent/no-mutation contract\n' >&2
  exit 1
fi

ZED_DIRECT_PREPARE_BUILT=1 "$ROOT/scripts/zed-extension/prepare-direct.sh" >/dev/null
AFTER="$(sha256sum "$FIXTURE" | cut -d' ' -f1)"
if [[ "$BEFORE" != "$AFTER" ]]; then
  printf 'direct preparation mutated source fixture\n' >&2
  exit 1
fi

if rg -q 'WorkspaceEdit|TextEdit' "$ROOT/crates/translator-lsp/src"; then
  printf 'direct LSP implementation contains edit-bearing protocol types\n' >&2
  exit 1
fi

printf 'direct workflow has no Agent/MCP dependency or source mutation\n'
