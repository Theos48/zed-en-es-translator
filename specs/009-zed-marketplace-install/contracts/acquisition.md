# Contract: Automatic Package Acquisition

## Trigger

Zed calls `language_server_command` for `en-es-translator` when the direct
language server is needed. No command, settings edit or separate UI is a
prerequisite.

## Supported Platform

The extension calls the Zed platform API before any network or state mutation.
Only `(Linux, X86_64)` proceeds. Every other result returns a stable message:

```text
English to Spanish Translator currently supports Linux x86_64.
```

Unsupported execution performs zero package requests and does not recommend a
checkout, build, terminal command or binary configuration.

## Readiness Sequence

1. Parse the compiled package lock and reject an invalid or over-budget lock.
2. Validate the active package for the current package ID.
3. If ready, return its `translator-lsp` command without network access.
4. Acquire the exclusive preparation lock and recheck readiness.
5. Set Zed status to `CheckingForUpdate`, then `Downloading` before the first
   request.
6. Prepare only in a non-launchable staging directory.
7. Download the fixed server archive and exact model resources.
8. Enforce HTTPS source, byte count and SHA-256 before using each download.
9. Decode resources to fixed basenames; enforce installed count and SHA-256.
10. Mark only allowlisted executables executable.
11. Validate the complete package and atomically promote staging.
12. Atomically set active/previous state, clear the Zed installation status and
    return the active LSP command.

All requests are content-free GET operations to URLs present in the published
lock. Redirect discovery, proxies supplied by a workspace, retries to alternate
hosts and runtime registry lookup are forbidden.

## Returned Command

```text
command: packages/<active-package>/bin/translator-lsp
args: []
env: []
```

The command path is extension-owned and verified. The LSP resolves the runner
and model set adjacent to its own executable. No arbitrary path, URL, argument
or inherited runtime setting participates in the marketplace journey.

## Failure Contract

| Condition | Behavior |
|---|---|
| Network unavailable before first readiness | Set redacted `Failed` status; return retryable error; execute nothing |
| Wrong status/length/hash | Remove or quarantine staging; execute nothing |
| Zstandard or archive failure | Remove or quarantine staging; execute nothing |
| Storage budget unavailable | Fail before or during staging; preserve active |
| Zed/process interruption | Leave no active reference to staging; next call safely reacquires/restarts |
| Concurrent preparation | One owner prepares; others wait/recheck; no partial activation |
| Update failure with valid active package | Report update failure and launch active package |
| Active package invalid and update fails | Fail closed; execute neither package |

Diagnostics may include phase, stable error code, package version, public
source host, expected/observed byte counts and retry instruction. They must not
include workspace/document paths, source text, translations, request bodies,
headers, tokens, environment values or raw child output.

## Offline Contract

Once the active package validates, repeated readiness checks, Zed restarts,
translation, failed-update fallback and package validation issue no external
request. The LSP and native runner contain no acquisition code.

## Disable and Uninstall

Disabling/unloading prevents new calls and stops the LSP through Zed's normal
lifecycle. Uninstall relies on Zed's extension store to remove both the
installed extension and `work/en-es-translator`, including current, previous,
staging, lock and model files. No product-owned state exists elsewhere.
