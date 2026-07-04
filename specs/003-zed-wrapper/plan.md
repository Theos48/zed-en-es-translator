# Implementation Plan: Zed Wrapper

**Branch**: `main` (feature id `003-zed-wrapper`) | **Date**: 2026-07-03 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/003-zed-wrapper/spec.md`

**Note**: This plan is produced by the `speckit-plan` workflow for the third
formal feature.

## Summary

Package the existing offline MCP translation server as a local Zed development
extension. The feature adds a `zed-extension/` Rust/WASM wrapper with
`extension.toml`, registers one MCP context server, and returns a controlled
`zed::Command` that launches the already implemented `translator-mcp` stdio
server. The launch profile uses a prepared local server artifact, structured
arguments, and an explicit environment allowlist.

This feature intentionally excludes real providers, network transports,
marketplace or registry publication, advanced editor UX, selection replacement,
and automatic buffer edits.

## Technical Context

**Language/Version**: Rust 2021 through the project Docker workflow pinned in
`Makefile` to `rust:1.96.1-bookworm`; Zed extension code targets Rust/WASM via
`zed_extension_api = "0.7.0"` for local development.

**Primary Dependencies**: Existing `translator-mcp` binary crate; new
`zed-extension/` Rust extension crate with `zed_extension_api = "0.7.0"`.
Reuse existing `translator-core`, `translator-cli`, and `translator-mcp`
dependencies without adding Node, npm, HTTP, or provider dependencies. Cargo
dependency changes must update the project lockfile through the Docker workflow
and remain project-scoped.

**Storage**: N/A for product data. Zed may store local extension settings, but
the project must not create persistent secrets, `.env` files, databases, or
provider credentials.

**Testing**: `make test`, `make fmt`, and `make clippy` for the full Rust
workspace inside Docker. Add focused extension validation targets for manifest,
launch profile, WASM build, environment allowlist, and redacted diagnostics.
Manual Zed smoke validation is required when the host already satisfies Zed's
own dev-extension prerequisites.

**Target Platform**: Fedora/Linux development host with Zed local dev
extension. The extension itself is Rust/WASM; the launched MCP server is the
Linux native `translator-mcp` stdio binary built by the project workflow.

**Project Type**: Rust workspace plus a Zed extension wrapper subproject.

**Performance Goals**: Zed receives the server launch command immediately after
configuration validation. Missing or unusable artifact failures are visible
within 15 seconds. Translation behavior keeps the existing 20 KiB input, 4 KiB
segment, 256 segment, 40 KiB output, and 15 s provider timeout limits from the
core/MCP features.

**Constraints**: Offline-only; stdio MCP only; one local server process; no
real provider; no network call or download; no shell execution; no automatic
buffer/source-file mutation; no inherited full Zed/shell environment; no source
text, translated text, secrets, environment dumps, tokens, headers, or
sensitive unredacted paths in diagnostics.

**Scale/Scope**: One Zed local development extension, one context server entry,
one prepared `translator-mcp` artifact, one launch profile, and the two existing
MCP tools (`translate_text`, `translate_file`).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Safety-first translation**: PASS. The plan only launches the existing
  read-oriented MCP server and keeps buffer/source-file mutation out of scope.
- **Offline-first provider boundary**: PASS. The wrapper exposes no provider
  selection and launches the existing offline deterministic server path.
- **Test-first development**: PASS. Tasks must start with failing manifest,
  launch-profile, environment allowlist, redaction, missing-artifact, and manual
  smoke validation checks before implementation.
- **Explicit contracts and limits**: PASS. The plan references the existing
  `TranslateRequest`, success/error output, `ErrorCode`, provider segment
  contract, CLI contract, MCP tool contracts, and inherited size/timeout limits;
  this feature adds only the Zed launch contract.
- **Minimal host footprint**: PASS with a documented Zed prerequisite caveat.
  Project builds and automated checks stay inside the project Docker workflow.
  Zed currently documents that Rust via rustup is required for local development
  extensions with custom Rust; this feature does not install it. Manual Zed
  validation may be blocked unless that host prerequisite already exists or is
  approved under the host policy.

## Project Structure

### Documentation (this feature)

```text
specs/003-zed-wrapper/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── zed-extension.md
│   └── launch-profile.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
crates/
├── translator-core/
├── translator-cli/
└── translator-mcp/
    ├── src/
    └── tests/

zed-extension/
├── extension.toml
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── settings.rs
│   ├── launch.rs
│   └── diagnostics.rs
└── tests/
    ├── extension_manifest.rs
    ├── launch_profile.rs
    └── diagnostics_redaction.rs

docker/
└── rust-toolchain.Dockerfile

Makefile
```

**Structure Decision**: Add `zed-extension/` as the Zed extension directory,
separate from the Rust workspace crates and from `specs/`. Keep
`translator-mcp` as the server binary; the wrapper must not duplicate MCP tool
logic or translation behavior.

## Complexity Tracking

No constitution violations accepted for this feature.

## Phase 0 Research Output

See [research.md](./research.md).

## Phase 1 Design Output

- [data-model.md](./data-model.md)
- [contracts/zed-extension.md](./contracts/zed-extension.md)
- [contracts/launch-profile.md](./contracts/launch-profile.md)
- [quickstart.md](./quickstart.md)

## Post-Design Constitution Check

- **Safety-first translation**: PASS. Contracts require no editor buffer edits,
  no source-file mutation, and reuse of the existing MCP tool behavior.
- **Offline-first provider boundary**: PASS. Contracts expose no provider
  settings, remote confirmation fields, network transports, downloads, or HTTP
  endpoints.
- **Test-first development**: PASS. Quickstart and future tasks require failing
  checks for manifest validity, launch profile, environment minimization,
  diagnostics redaction, missing artifact handling, offline behavior, and
  no-mutation behavior before implementation.
- **Explicit contracts and limits**: PASS. Zed extension and launch-profile
  contracts are versioned under `contracts/`; translation and MCP contracts are
  inherited from completed features 001 and 002.
- **Minimal host footprint**: PASS with caveat. The project plan does not
  install host runtimes. Automated checks run through Docker; manual Zed dev
  extension validation depends on Zed's own host prerequisites and must follow
  the system policy if missing.
