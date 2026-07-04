# Contract: Server Launch Profile

## Purpose

The launch profile is the boundary between Zed and the existing
`translator-mcp` server. It is the only process startup behavior added by this
feature.

## Command Shape

The extension returns a `zed::Command` equivalent to:

```text
command = <configured translator-mcp binary path>
args = []
env = []
```

Working location is the process context provided by Zed for the returned
command. This wrapper does not expose a configurable cwd and must not encode
workspace roots, requested file paths, or source text in argv or environment to
simulate working-location control.

Allowed future-safe environment entries for this feature:

```text
RUST_LOG=warn
```

`RUST_LOG` is optional and must not include source text, paths, secrets, headers,
or provider values.

## Validation Rules

Before returning the launch profile, the wrapper must validate:

- the context server id is `translator-en-es`;
- a local binary path is configured;
- the configured artifact name is `translator-mcp` or otherwise clearly maps to
  the prepared server artifact;
- a configured artifact path containing spaces remains a single command value
  and is not shell-split;
- the wrapper is not asked to add extra arguments;
- the wrapper is not asked to add arbitrary environment variables;
- no provider, network, API key, or remote confirmation settings are present.

The wrapper must not:

- invoke `/bin/sh`, `bash`, PowerShell, or another shell;
- run `cargo run` as the Zed startup command;
- launch HTTP or remote MCP transports;
- pass text to translate through argv or environment;
- pass workspace roots or requested file paths through argv or environment;
- inherit the full process environment.

## Success Result

A successful launch profile starts one `translator-mcp` process over stdio.
After startup, the existing MCP server contract from
`specs/002-mcp-server/contracts/mcp-tools.md` remains authoritative.

Observable success:

- Zed can list `translate_text` and `translate_file`.
- Valid tool calls return offline deterministic translations.
- Failed tool calls keep using `isError: true` MCP results from the MCP server.
- Protocol errors remain protocol-level MCP errors.

## Failure Result

Failures before process launch return redacted Zed extension errors.

Failure categories:

- `BINARY_PATH_NOT_CONFIGURED`
- `BINARY_NOT_FOUND`
- `BINARY_NOT_EXECUTABLE`
- `BINARY_STALE_OR_INCOMPATIBLE`
- `UNSUPPORTED_CONTEXT_SERVER`
- `UNSAFE_LAUNCH_CONFIGURATION`
- `INTERNAL_EXTENSION_ERROR`

Failure messages must:

- identify the corrective action;
- avoid full absolute paths;
- avoid environment dumps;
- avoid source text, translated text, segments, secrets, headers, and tokens;
- become visible during manual validation within the Zed context-server
  initialization window (observed ~60 seconds in the current Zed WASM
  extension runtime; see `spec.md` SC-004 and Status Notes). A sub-15-second
  target is a future goal gated on a Zed extension API capability this
  feature does not currently have.

## Repeatability

Running the preparation workflow repeatedly without source changes must leave:

- one extension directory;
- one manifest;
- one launch profile contract;
- one prepared server artifact path;
- no duplicate generated extension state;
- no provider configuration or secret files.

Repeated registration, repeated startup, and restart after failed startup must:

- revalidate the configured context server id, binary path, and environment
  allowlist;
- return the same redacted failure class while the underlying problem remains;
- allow a later corrected configuration or artifact to start without requiring
  unrelated cleanup;
- avoid creating duplicate manifests, settings, generated files, or server
  registrations.
