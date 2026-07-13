# Implementation Plan: Direct Zed Translation

**Branch**: `006-direct-zed-translation` | **Date**: 2026-07-13 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/006-direct-zed-translation/spec.md`

**Spec Kit Flow**: The active feature was selected from F010 and initialized
through the local Spec Kit scripts:

```bash
.specify/scripts/bash/check-prerequisites.sh --json --paths-only
.specify/scripts/bash/setup-plan.sh --json
```

Clarification found no critical product ambiguity. The requirements checklist
passed 16/16. The domain checklist prerequisite was initially blocked because
`plan.md` did not yet exist and is rerun after this plan.

## Summary

Add a native, Agent-free translation workflow to the Zed extension by exposing
an LSP code action for Markdown and plain-text buffers. A new native Rust
`translator-lsp` process receives Zed document snapshots and selection ranges,
delegates all translation, provider, limits, privacy, and formatting behavior
to `translator-core`, and caches a read-only Markdown hover preview tied to the
document version and selected range.

Zed's stable extension API 0.7.0 does not expose custom actions, editor
selection, clipboard access, or custom preview panes directly to extension
WASM. The language-server route is the supported native integration surface:
Zed provides the selected range to `textDocument/codeAction`, whose safe title
identifies the provider as offline, local, or remote-confirmation-required;
executes a server-owned command; displays request-specific confirmation through
`window/showMessageRequest`, and renders the result through
`textDocument/hover`. Agent Panel is not involved.

## Technical Context

**Language/Version**: Rust 2021 through the project Docker workflow pinned in
`Makefile` to `rust:1.96.1-bookworm`; Zed wrapper Rust/WASM through
`zed_extension_api = "0.7.0"`.

**Primary Dependencies**: Existing `translator-core` and `zed-extension/`; add
workspace binary crate `translator-lsp` using `lsp-server = "0.7"`,
`lsp-types = "0.97"`, `serde`, and `serde_json`. The synchronous LSP scaffold
matches the existing synchronous core and supports in-memory protocol tests.

**Storage**: No persistent product storage. Open-document snapshots, pending
invocations, and previews live only in `translator-lsp` memory and are removed
on document change/close or server exit. Source and translated text are never
written to logs, files, settings, or extension state.

**Testing**: TDD through Makefile/Docker. Add focused core selection/snapshot
tests, in-memory LSP contract tests, native Zed wrapper tests, shell integration
contracts, and a manual Zed validation template. Final gates are
`make test-direct-zed-translation`, `make test-zed-extension`, `make test`,
`make fmt`, and `make clippy`.

**Target Platform**: Fedora/Linux workstation running a current Zed build that
supports extension API 0.7.0 and standard LSP code actions, execute-command,
show-message requests, and Markdown hover.

**Project Type**: Rust workspace plus a local Zed development extension.

**Performance Goals**:

- Listing the translation code action performs no provider call and does not
  copy unrelated workspace content.
- Translation obeys the existing 15 s provider timeout.
- Input remains at 20 KiB, segments at 4 KiB and 256 count, and output at
  40 KiB.
- Stale previews are invalidated synchronously when a newer document version
  arrives.

**Constraints**:

- The code action applies only to Zed Markdown and Plain Text buffers.
- A non-empty range means selection translation; an empty range means the
  authorized open-document fallback.
- LSP UTF-16 positions are converted to checked UTF-8 byte boundaries before
  slicing; invalid or stale positions fail closed.
- Markdown selection is translated only when the selected range is classed as
  visible translatable content by the core; a range intersecting protected or
  ambiguous content is rejected rather than guessed.
- Open-document fallback validates the URI/path against the canonical worktree
  and current file rules before translating the in-memory snapshot.
- Mock/offline remains default. Remote provider calls require configuration,
  allowlisting, `window/showMessageRequest` confirmation, and the existing core
  secret gate.
- The LSP command arguments contain URI, version, range, and input kind only;
  they never contain source or translated text.
- The code-action title includes only the safe provider locality label
  `offline`, `local`, or `remote - confirmation required`; it never includes a
  provider name, URL, executable, model, or configuration value.
- Hover previews are read-only and version-bound. No workspace edit, buffer
  edit, source write, clipboard write, custom webview, or Agent fallback is
  implemented.
- The wrapper launches only a configured executable named `translator-lsp`
  with an empty argument vector and allowlisted provider environment.
- Rust stays project-scoped in Docker; no host runtime or service is installed.

**Scale/Scope**: One code action, one execute-command identifier, one hover
preview per open document, Markdown and `.txt` inputs only, and the existing
provider modes. Custom UI, clipboard integration, apply/insert actions, code
file translation, publication, and Zed core changes are out of scope.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **Safety-first translation**: PASS. The plan creates no edits and rejects
  protected or ambiguous selection ranges. Preview state is separate from the
  source buffer.
- **Offline-first provider boundary**: PASS. `translator-core` remains the
  provider authority; mock is default and each remote invocation is confirmed
  before the core receives `remote_confirmed = true`.
- **Test-first development**: PASS. Tasks must add failing core, protocol,
  wrapper, privacy, stale-state, and non-mutation checks before behavior.
- **Explicit contracts and limits**: PASS. Contracts below fix the LSP methods,
  command payload, state transitions, errors, existing size limits, and 15 s
  timeout before implementation.
- **Minimal host footprint**: PASS. Dependencies are locked Rust workspace
  crates used only through the project container; no host install is required.

## Project Structure

### Documentation (this feature)

```text
specs/006-direct-zed-translation/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── checklists/
│   ├── requirements.md
│   └── direct-ux-privacy.md
└── contracts/
    ├── direct-zed-workflow.md
    ├── translator-lsp.md
    └── zed-lsp-launch.md
```

`tasks.md` is generated only by `speckit-tasks` after plan and checklist gates
pass.

### Source Code (repository root)

```text
crates/
├── translator-core/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── markdown.rs
│   │   └── workspace.rs
│   └── tests/
└── translator-lsp/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── main.rs
    │   ├── protocol.rs
    │   ├── selection.rs
    │   └── state.rs
    └── tests/

zed-extension/
├── extension.toml
├── src/
│   ├── lib.rs
│   ├── launch.rs
│   └── settings.rs
└── tests/

scripts/zed-extension/
├── prepare.sh
└── prepare-direct.sh

tests/integration/
Makefile
Cargo.toml
Cargo.lock
```

**Structure Decision**: Put editor protocol and ephemeral UI state in a new
`translator-lsp` binary, keep all content/provider policy in `translator-core`,
and keep the WASM wrapper limited to validated process launch. The existing MCP
path remains compatibility infrastructure and is not called by the direct
workflow.

## Phase 0: Research

Research decisions and rejected alternatives are recorded in
[research.md](./research.md). All technical unknowns are resolved:

- Zed 0.7.0 has no generic custom UI/action API, but language-server code
  actions, execute-command, hover, and show-message requests cover the required
  non-mutating direct flow.
- LSP range/version data provides a reliable snapshot boundary without Agent.
- Hover is the only stable rich read-only preview available without modifying
  Zed itself; clipboard and custom panes remain unavailable.
- The existing synchronous core and provider timeout fit a synchronous
  `lsp-server` process with explicit handling of server-to-client confirmation.

## Phase 1: Design And Contracts

Design outputs:

- [data-model.md](./data-model.md)
- [contracts/direct-zed-workflow.md](./contracts/direct-zed-workflow.md)
- [contracts/translator-lsp.md](./contracts/translator-lsp.md)
- [contracts/zed-lsp-launch.md](./contracts/zed-lsp-launch.md)
- [quickstart.md](./quickstart.md)

## Post-Design Constitution Check

- **Safety-first translation**: PASS. Contracts prohibit edit-bearing code
  actions and bind previews to immutable document versions.
- **Offline-first provider boundary**: PASS. The LSP adds a consent UI but does
  not replace the core's remote deny/secret checks.
- **Test-first development**: PASS. Quickstart and contracts define negative
  tests for UTF-16 ranges, stale versions, protected content, paths, remote
  denial, secrets, redaction, and non-mutation.
- **Explicit contracts and limits**: PASS. LSP payloads and state lifetime are
  specified without changing the translation result/error or provider segment
  contracts.
- **Minimal host footprint**: PASS. The build extends existing Docker targets
  and creates no host-level dependency.

## Downstream Gate Status

- **Checklist**: Applied after the plan prerequisite became available; 38/38
  direct UX/privacy requirement checks pass in
  [checklists/direct-ux-privacy.md](./checklists/direct-ux-privacy.md).
- **Tasks**: PASS; all 48 tasks are complete. The 46 initial tasks cover setup,
  foundations, three stories, and evidence; T047/T048 in Phase 7 remediate
  provider launch propagation and misleading direct diagnostics.
- **Analyze**: PASS after remediating one initial high-severity user-visible
  locality gap. The second pass covers 30/30 requirements and measurable
  criteria with zero critical/high findings or constitution conflicts.
- **Implement**: PASS; all 48 tasks pass focused and inherited automated gates.
  All three real-Zed scenarios pass, including remote dismissal and confirmed
  secret denial; SC-008 is complete.
- **Converge**: PASS. A manual finding triggered a second pass with one HIGH
  partial gap and one MEDIUM contradiction, producing T047/T048. The follow-up
  pass found zero remaining implementation gaps and left `tasks.md` unchanged.

## Complexity Tracking

No constitution violations are introduced.
