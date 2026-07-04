# Quickstart: Zed Wrapper

## Prerequisites

- Docker available on the host.
- Zed installed by the user outside this project workflow.
- No project-specific Rust installation on Fedora is required for repository
  checks.
- Manual Zed dev-extension loading requires Zed's Rust/rustup prerequisite for
  custom Rust extensions. This repository workflow must not install it as a
  project dependency; use only an approved host-policy exception or record the
  manual validation as blocked.

## Scope Guard

This feature validates only the local Zed wrapper:

- included: `zed-extension/extension.toml`, Rust/WASM wrapper,
  `context_server_command`, local `translator-mcp` startup, redacted startup
  diagnostics, minimal environment, and repeatable preparation;
- excluded: real providers, provider settings, remote/network calls, MCP
  registry or marketplace publication, advanced editor UX, selection
  replacement, and automatic buffer edits.

## Planned Repository Validation Commands

After implementation tasks complete, run:

```bash
make zed-extension-prepare
make test-zed-extension
make test
make fmt
make clippy
```

Expected repository results:

- `translator-mcp` release artifact is prepared for local Zed startup;
- extension manifest and launch profile checks pass;
- extension diagnostics redaction checks pass;
- existing core, CLI, and MCP tests continue to pass;
- formatting and Clippy remain clean.

## Launch Working Location

The launch profile intentionally documents command intent, args, environment,
and working location. In this Zed wrapper implementation, the returned
`zed::Command` controls `command`, `args`, and `env`; the process context is
provided by Zed and is not encoded by the wrapper as a configurable cwd.

The wrapper must not simulate working-location control by passing workspace
roots, requested file paths, or source text through argv or environment.

## Manual Zed Smoke Validation

Run this only when the host already satisfies Zed's local dev-extension
prerequisites.

1. Prepare the server artifact:

   ```bash
   make zed-extension-prepare
   ```

2. Open Zed.

3. Run `zed: install dev extension` and select:

   ```text
   zed-extension/
   ```

4. Use the extension configuration modal to set `binary_path` to the prepared
   `translator-mcp` artifact path shown by the preparation command. Zed stores
   this extension configuration in the user settings file; do not keep a
   duplicate `.zed/settings.json` entry for the same context server during this
   manual validation.

5. Start or invoke the `translator-en-es` context server through Zed's MCP/Agent
   Panel flow.

6. Call `translate_text` with:

   ```json
   {
     "source_text": "Read the docs.",
     "source_language": "en",
     "target_language": "es",
     "tone": "technical_neutral",
     "preserve_formatting": true
   }
   ```

Expected manual result:

- Zed starts `translator-mcp`; no manually started server shell is needed.
- Tool discovery still exposes exactly `translate_text` and `translate_file`.
- The successful result contains the deterministic offline Spanish translation.
- The original file or editor buffer remains unchanged.

## Failure Validation Scenarios

### Missing Binary Path

Leave `binary_path` unset in the extension configuration modal and request the
context server.

Expected:

- Zed shows an actionable error such as `BINARY_PATH_NOT_CONFIGURED`;
- the message tells the user to run the preparation workflow and configure the
  artifact path;
- no absolute path, environment dump, source text, translated text, token, or
  secret is logged.

### Unusable Binary

Set `binary_path` in the extension configuration modal to a missing,
non-executable, stale, or incompatible artifact and request startup.

Expected:

- Zed shows a redacted setup error within 15 seconds;
- the error identifies the category and next corrective action;
- the full local path is not echoed in diagnostics.

Manual timing evidence is required to close this criterion: record the observed
time from requesting the context server to seeing the redacted diagnostic.

### Repeated Startup After Failure

Repeat a failed startup request before and after correcting the extension modal
configuration.

Expected:

- repeated failures return the same redacted failure category while the problem
  remains;
- corrected configuration starts without deleting duplicate generated state;
- no duplicate manifest, context server, provider setting, or secret file is
  created.

### Environment Minimization

Start Zed from a shell containing unrelated variables and fake secrets, then
request the context server.

Expected:

- the launched `translator-mcp` process receives only allowlisted environment
  values;
- fake secrets and unrelated shell variables do not appear in diagnostics,
  server logs, or test output.

### Remote Provider Denial

Attempt to provide remote/provider settings through extension configuration or
tool arguments.

Expected:

- extension configuration rejects provider settings;
- MCP tool calls reject remote/provider fields according to the existing MCP
  server contract;
- no network request occurs.

Automated coverage: `tests/integration/zed_extension_remote_denial.sh` spawns
the exact prepared `translator-mcp` artifact with the same direct-command,
minimal-environment shape the wrapper's `LaunchProfile` produces, then performs
a real MCP stdio handshake to confirm `translate_text` rejects `provider` and
`remote_confirmation` fields with `INVALID_INPUT`, and that `tools/list` still
exposes exactly `translate_text` and `translate_file` with no provider/remote
tool added. This closes the gap between unit-level contract coverage in
`crates/translator-mcp/tests/mcp_remote_denial.rs` and the actual
extension-launched artifact.

### No Mutation

Open a file in Zed with unsaved changes, invoke translation through the wrapper,
and inspect the editor buffer and source file afterwards.

Expected:

- translation output appears only as a tool result;
- the buffer content and file on disk remain unchanged.

## Validation Results

Recorded on 2026-07-03.

### Repository Checks

- `make zed-extension-prepare`: PASS. Built `translator-mcp` in release mode
  through the project Docker workflow and printed
  `target/release/translator-mcp`.
- `make test-zed-extension`: PASS. Ran extension unit/contract tests, built the
  extension for `wasm32-wasip1`, rebuilt the release server artifact, and passed
  artifact, idempotency, Make target, dependency-scope, no-mutation, and
  remote-denial scripts.
- `make test`: PASS. Existing CLI, core, and MCP tests pass in the project
  Docker workflow.
- `make fmt`: PASS. Rust formatting checks pass for both the root workspace and
  the isolated `zed-extension/` crate.
- `make clippy`: PASS. Clippy passes for both the root workspace and the
  isolated `zed-extension/` crate with warnings denied.

Re-verified 2026-07-04 after adding `tests/integration/zed_extension_remote_denial.sh`:

```text
./tests/integration/zed_extension_prepare_artifact.sh
prepared artifact ok: translator-mcp
./tests/integration/zed_extension_prepare_idempotent.sh
prepare idempotent ok
./tests/integration/zed_extension_make_targets.sh
make target contract ok
./tests/integration/zed_extension_dependency_scope.sh
dependency scope ok
./tests/integration/zed_extension_no_mutation.sh
no mutation ok
./tests/integration/zed_extension_remote_denial.sh
remote denial ok
```

`make test`, `make fmt`, and `make clippy` were re-run after this addition and
remain PASS with no warnings.

### User Story Validation

- US1 startup wrapper: PASS by automated contract coverage. The manifest
  declares one `translator-en-es` context server, and launch-profile tests verify
  direct `translator-mcp` command construction, empty args, allowlisted env, and
  path-with-spaces preservation.
- US2 reproducible preparation: PASS by automated integration coverage. Repeated
  preparation prints stable output, keeps one manifest, creates no secret/env
  files, and keeps `zed_extension_api` scoped to `zed-extension/Cargo.lock`.
- US3 safe diagnostics: PASS by automated negative coverage. Missing,
  non-executable, stale, incompatible, unsupported-context, provider/remote,
  arbitrary env/arg, repeated failure, path, token, environment dump, source
  text, and translated text cases return stable redacted diagnostics.
- US3 offline-default denial (FR-006, FR-013, SC-008): PASS by automated
  end-to-end coverage. `tests/integration/zed_extension_remote_denial.sh`,
  recorded 2026-07-04, ran against the real prepared `translator-mcp` artifact
  through `make test-zed-extension` and printed `remote denial ok`.

### Manual Zed Smoke

Host prerequisite resolved on 2026-07-03 by approved system-policy exception:
`rustup` was installed from Fedora DNF, the stable Rust 1.96.1 toolchain was
installed user-scoped with `rustup-init --no-modify-path`, and
`wasm32-wasip1` was added. `cargo build --manifest-path
zed-extension/Cargo.toml --target wasm32-wasip1 --release --locked` passes on
the host.

Manual UI smoke in Zed: PASS on 2026-07-04.

- `zed: install dev extension` accepts `zed-extension/`.
- Before configuring the modal, Zed reports `BINARY_PATH_NOT_CONFIGURED`; this
  is expected because the extension has no artifact path yet.
- The extension configuration modal stores one `translator-en-es` entry in Zed
  user settings.
- Zed starts `translator-mcp` through the `translator-en-es` context server.
- `translate_text` returns the deterministic offline Spanish translation.
- The original file or editor buffer remains unchanged.

Observed tool result:

```json
{"translated_text":"Lee la documentacion."}
```

Revalidated through Zed's extension modal after removing the duplicate workspace
`.zed/settings.json` path. The wrapper returns a direct launch command to Zed
for valid `translator-mcp` paths and lets Zed start the server.

### Manual Failure Timing

Manual failure timing: RESCOPED on 2026-07-04.

Observed with missing artifact path
`/tmp/zed-en-es-missing/translator-mcp` configured through Zed's extension
modal:

```text
2026-07-04T00:18:33-06:00 ERROR [crates/context_server/src/transport/stdio_transport.rs:58] Broken pipe (os error 32)
2026-07-04T00:19:33-06:00 ERROR [context_server::client] cancelled csp request task for "initialize" id 0 which took over 60s
2026-07-04T00:19:33-06:00 ERROR [project::context_server_store] translator-en-es context server failed to start: Context server request timeout
```

The missing-artifact case does not satisfy the original 15 second diagnostic
target in this Zed runtime. Zed blocks in context-server startup and surfaces a
60 second initialization timeout with the server indicator red.

Implementation note: Zed's WASM extension runtime could not reliably validate
the artifact with `std::fs::metadata` or by running `translator-mcp` directly as
a preflight command. A follow-up attempt with host-side `/usr/bin/test -e/-f/-x`
also caused the Zed configuration modal to time out. The current wrapper avoids
extension-side host probes in WASM to keep the valid-path startup path direct;
the missing-artifact fast-fail requirement is therefore rescheduled for a future
Zed API strategy or a different packaging path.

### Test-First Traceability

The task plan required tests/checks to be written and observed failing before
implementation. The final artifacts preserve the task order and passing
validation summaries, but they do not preserve the initial failing command
outputs. Future feature cycles should record the first expected failing run when
that traceability is required.

### Scope Confirmation

Confirmed: this feature adds no real provider, no network transport, no
publication flow, no source-file mutation path, and no editor buffer edit path.
