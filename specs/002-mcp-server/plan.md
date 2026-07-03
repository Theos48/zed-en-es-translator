# Implementation Plan: MCP Server

**Branch**: `002-mcp-server` | **Date**: 2026-07-02 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/002-mcp-server/spec.md`

**Note**: This plan is produced by the `speckit-plan` workflow for the second
formal feature.

## Summary

Expose the existing offline translation core as an MCP-compatible server with
two tools: `translate_text` and `translate_file`. The server will be a Rust
workspace binary using the official Rust MCP SDK over stdio, will call
`translator-core` directly, and will preserve the first feature's security,
privacy, file validation, limits, and mock-provider-only behavior.

This feature intentionally excludes the Zed extension wrapper, MCP registry or
publishing work, real providers, network transports, remote provider selection,
and automatic buffer edits.

## Technical Context

**Language/Version**: Rust stable through the project Docker workflow pinned in
`Makefile` to `rust:1.96.1-bookworm`.

**Primary Dependencies**: Existing `translator-core`; `rmcp = "2.1.0"` for MCP
server protocol support; `tokio` for async runtime; `serde`, `serde_json`, and
`schemars` for typed parameter parsing and JSON Schema generation.

**Storage**: N/A. The server is stateless and reads only request input or
allowed workspace files through the existing core.

**Testing**: `make test` for the full Rust workspace inside Docker. Add focused
MCP contract/integration tests under the new MCP crate and a `make test-mcp`
target during implementation.

**Target Platform**: Linux development host first; process-launched MCP server
over stdio for future Zed Agent Panel integration.

**Project Type**: Rust workspace with library crate, CLI crate, and MCP server
binary crate.

**Performance Goals**: Preserve existing 20 KiB input, 4 KiB segment, 256
segment, 40 KiB output, and 15 s provider timeout limits. Mock-provider MCP tool
calls should add negligible protocol overhead relative to core execution.

**Constraints**: Offline-only; stdio transport only; no HTTP listener; no real
provider; no network call; no buffer or source-file mutation; no duplicated file
authorization policy; no source text, translated text outside successful
payloads, secrets, protected segments, or sensitive paths in diagnostics.

**Scale/Scope**: Two tools, one local MCP server process, one language pair
(`en` -> `es`), direct text plus `.md`, `.markdown`, and `.txt` file requests.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Safety-first translation**: PASS. The plan adds only read-oriented MCP tool
  results and keeps buffer/source-file mutation out of scope.
- **Offline-first provider boundary**: PASS. The server uses the existing
  offline deterministic provider path; remote providers and network transports
  remain out of scope.
- **Test-first development**: PASS. Tasks must begin with failing MCP contract,
  integration, privacy, and adversarial tests before implementation.
- **Explicit contracts and limits**: PASS. The plan defines MCP tool contracts
  and reuses existing translation request/result/error limits from feature 001.
- **Minimal host footprint**: PASS. Rust runs through the project Docker
  workflow. No global Rust, Node, service, database, or provider install is
  required for this feature.

## Project Structure

### Documentation (this feature)

```text
specs/002-mcp-server/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── mcp-tools.md
│   ├── translate-text.input.schema.json
│   ├── translate-file.input.schema.json
│   └── tool-result.schema.json
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
crates/
├── translator-core/
│   └── src/
├── translator-cli/
│   └── src/
└── translator-mcp/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── lib.rs
    │   ├── protocol.rs
    │   └── tools.rs
    └── tests/
        ├── mcp_tool_discovery.rs
        ├── mcp_translate_text.rs
        ├── mcp_translate_file.rs
        ├── mcp_error_mapping.rs
        └── mcp_privacy.rs

tests/
└── fixtures/
    ├── markdown/
    ├── text/
    └── security/
```

**Structure Decision**: Add `crates/translator-mcp` as a first-class workspace
member. It depends on `translator-core` directly instead of shelling out to
`translator-cli`, because the core already owns translation behavior, safety,
limits, and privacy. Keep `translator-cli` as a separate user/test boundary from
feature 001.

## Complexity Tracking

No constitution violations accepted for this feature.

## Phase 0 Research Output

See [research.md](./research.md).

## Phase 1 Design Output

- [data-model.md](./data-model.md)
- [contracts/mcp-tools.md](./contracts/mcp-tools.md)
- [contracts/translate-text.input.schema.json](./contracts/translate-text.input.schema.json)
- [contracts/translate-file.input.schema.json](./contracts/translate-file.input.schema.json)
- [contracts/tool-result.schema.json](./contracts/tool-result.schema.json)
- [quickstart.md](./quickstart.md)

## Post-Design Constitution Check

- **Safety-first translation**: PASS. Contracts require clean tool results and
  no buffer/source mutation.
- **Offline-first provider boundary**: PASS. Contracts do not expose provider
  selection and plan uses stdio only.
- **Test-first development**: PASS. Quickstart and future tasks require contract,
  negative, and privacy tests before implementation.
- **Explicit contracts and limits**: PASS. MCP input/result contracts are
  versioned under `contracts/`, while translation limits are inherited from
  feature 001.
- **Minimal host footprint**: PASS. No new global host tools are required; new
  Rust dependencies are project dependencies locked by Cargo.
