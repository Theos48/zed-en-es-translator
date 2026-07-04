#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT="${1:-"$ROOT/target/release/translator-mcp"}"

if [[ ! -x "$ARTIFACT" ]]; then
  printf 'prepared translator-mcp artifact is not executable: %s\n' "$ARTIFACT" >&2
  exit 1
fi

# Launch the exact artifact the Zed wrapper's LaunchProfile would run: a direct
# command and no arguments. The test clears its own shell environment to keep
# output deterministic; Zed's real context-server transport still inherits the
# Zed process environment before applying wrapper-provided env values (D064).
coproc MCP { env -i PATH="$PATH" "$ARTIFACT"; }

cleanup() {
  kill "$MCP_PID" >/dev/null 2>&1 || true
  wait "$MCP_PID" 2>/dev/null || true
}
trap cleanup EXIT

send() {
  printf '%s\n' "$1" >&"${MCP[1]}"
}

read_response_for_id() {
  local expected_id="$1"
  local line
  while IFS= read -r -t 5 -u "${MCP[0]}" line; do
    if printf '%s' "$line" | grep -qE "\"id\":$expected_id[,}]"; then
      printf '%s' "$line"
      return 0
    fi
  done
  printf 'timed out waiting for response id=%s\n' "$expected_id" >&2
  return 1
}

send '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"zed-extension-remote-denial-check","version":"0.0.0"}}}'
read_response_for_id 1 >/dev/null

send '{"jsonrpc":"2.0","method":"notifications/initialized"}'

send '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"translate_text","arguments":{"source_text":"Read the docs.","provider":"remote"}}}'
RESPONSE="$(read_response_for_id 2)"

if ! printf '%s' "$RESPONSE" | grep -q '"isError":true'; then
  printf 'expected translate_text to reject a provider selection field, got: %s\n' "$RESPONSE" >&2
  exit 1
fi

if ! printf '%s' "$RESPONSE" | grep -q '"code":"INVALID_INPUT"'; then
  printf 'expected INVALID_INPUT for provider selection field, got: %s\n' "$RESPONSE" >&2
  exit 1
fi

send '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"translate_text","arguments":{"source_text":"Read the docs.","remote_confirmation":true}}}'
RESPONSE="$(read_response_for_id 3)"

if ! printf '%s' "$RESPONSE" | grep -q '"isError":true'; then
  printf 'expected translate_text to reject remote_confirmation, got: %s\n' "$RESPONSE" >&2
  exit 1
fi

if ! printf '%s' "$RESPONSE" | grep -q '"code":"INVALID_INPUT"'; then
  printf 'expected INVALID_INPUT for remote_confirmation field, got: %s\n' "$RESPONSE" >&2
  exit 1
fi

# Confirm the extension-launched server still exposes exactly the two existing
# translation tools; no provider-selection or remote-control tool was added.
send '{"jsonrpc":"2.0","id":4,"method":"tools/list"}'
RESPONSE="$(read_response_for_id 4)"

for tool in translate_text translate_file; do
  if ! printf '%s' "$RESPONSE" | grep -q "\"name\":\"$tool\""; then
    printf 'expected tool %s in tools/list response, got: %s\n' "$tool" "$RESPONSE" >&2
    exit 1
  fi
done

if printf '%s' "$RESPONSE" | grep -Eiq '"name":"[^"]*(provider|remote)[^"]*"'; then
  printf 'unexpected provider/remote tool exposed by the extension-launched server\n' >&2
  exit 1
fi

printf 'remote denial ok\n'
