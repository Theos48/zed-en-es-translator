# Implementation Plan: Translation Core Contract

**Branch**: `001-translation-core-contract` | **Date**: 2026-07-01 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/001-translation-core-contract/spec.md`

## Summary

Build the first offline technical foundation for EN->ES translation: a Rust
translation core, deterministic mock provider, Rust CLI boundary, explicit
request/result/error contracts, Markdown/text preservation, safe workspace-only
file reads, limits, redacted diagnostics, and negative security tests.

This feature intentionally excludes real providers, network calls, automatic
buffer edits, Zed wrapper work, MCP server implementation, and full-file source
code translation.

## Technical Context

**Language/Version**: Rust stable for `translator-core` and `translator-cli`.
Rust is provided through the project `Makefile` using the pinned Docker image
`rust:1.96.1-bookworm`, avoiding a global Fedora Rust installation by default.

**Primary Dependencies**: Start with Rust standard library where practical.
Candidate project dependencies for implementation: `serde`, `serde_json`,
`thiserror` or equivalent, `tempfile` for tests. Any dependency must be pinned
by lockfile before implementation.

**Storage**: N/A. The feature is stateless and reads only request input or
allowed workspace files.

**Testing**: `make test` runs `cargo test` for core and CLI inside the pinned
Rust container. Integration tests exercise CLI stdin/stdout JSON and file-read
safety. No network or secrets required after dependencies are available.

**Target Platform**: Linux development host first; design remains portable for
future Zed extension and MCP integration.

**Project Type**: Library plus CLI.

**Performance Goals**: Validate and process inputs up to 20 KiB; reject
oversized input before provider processing; provider timeout 15 s.

**Constraints**: Offline-only for this feature; no real provider; no network;
no buffer edits; workspace-only file reads; no source/translation/secrets in
logs; preserve ambiguous content.

**Scale/Scope**: One language pair (`en` -> `es`), `.md`, `.markdown`, and
`.txt` for `translate_file`, direct text via `translate_text`, deterministic
mock provider.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Safety-first translation**: PASS. The plan excludes buffer edits and
  source-code full-file translation; ambiguous content is preserved.
- **Offline-first provider boundary**: PASS. The only provider in scope is
  offline/deterministic. Remote providers are denied by contract.
- **Test-first development**: PASS. Implementation tasks must start with
  failing contract, unit, integration, and negative security tests.
- **Explicit contracts and limits**: PASS. Contracts are defined in
  `contracts/`, with 20 KiB input, 4 KiB segment, 256 segment, 40 KiB output,
  and 15 s timeout limits.
- **Minimal host footprint**: PASS. Rust runs through the project-local Docker
  workflow in `Makefile`; `rustc` and `cargo` are not installed globally for
  this project by default.

## Project Structure

### Documentation (this feature)

```text
specs/001-translation-core-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── cli-wire.md
│   ├── translate-request.schema.json
│   └── translate-result.schema.json
└── checklists/
    ├── requirements.md
    └── security-testing.md
```

### Source Code (repository root)

```text
crates/
├── translator-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── contract.rs
│       ├── errors.rs
│       ├── limits.rs
│       ├── markdown.rs
│       ├── privacy.rs
│       ├── provider.rs
│       ├── redaction.rs
│       └── workspace.rs
└── translator-cli/
    ├── Cargo.toml
    ├── tests/
    │   └── cli_contract.rs
    └── src/
        └── main.rs

tests/
├── fixtures/
│   ├── markdown/
│   ├── text/
│   └── security/

Cargo.toml
Cargo.lock
```

**Structure Decision**: Implement only `crates/translator-core`,
`crates/translator-cli`, and shared fixtures/tests in this feature. Defer
`mcp-server/` and `zed-extension/` until later Spec Kit features.

## Complexity Tracking

No constitution violations accepted for this feature.
