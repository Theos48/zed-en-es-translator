# Implementation Plan: Real Provider Configuration

**Branch**: `005-real-provider-config` | **Date**: 2026-07-07 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/005-real-provider-config/spec.md`

**Spec Kit Flow**: This plan was prepared after running the local Spec Kit
scripts for this feature:

```bash
.specify/scripts/bash/create-new-feature.sh --json --short-name real-provider-config "<F004 provider prompt>"
.specify/scripts/bash/check-prerequisites.sh --json --paths-only
.specify/scripts/bash/setup-plan.sh --json
```

Strict command replay was also run after review feedback to verify the active
feature and plan boundary without advancing to tasks or implementation:

```bash
specify workflow info speckit
.specify/scripts/bash/create-new-feature.sh --json --allow-existing-branch --number 5 --short-name real-provider-config "<F004 provider prompt>"
.specify/scripts/bash/check-prerequisites.sh --json --paths-only
.specify/scripts/bash/setup-plan.sh --json
.specify/extensions/agent-context/scripts/bash/update-agent-context.sh specs/005-real-provider-config/plan.md
.specify/scripts/bash/check-prerequisites.sh --json
git switch -c 005-real-provider-config
```

The installed `speckit` workflow is the full cycle
`specify -> plan -> tasks -> implement`; it was not invoked end-to-end because
this iteration is intentionally stopped at plan.

Clarification found no critical open question worth blocking planning. The
spec was tightened before planning so remote confirmation can be additive and
default-deny while existing request/result/error shapes remain backward
compatible.

## Summary

Promote F004 into the fifth formal Spec Kit feature by adding a real,
explicitly configured English-to-Spanish provider path while keeping mock/offline
translation as the default. The first real provider path targets a local
self-hosted LibreTranslate-compatible service. Provider configuration is
process/server configuration, not source text and not per-request provider
metadata; per-request remote confirmation is additive and defaults to denial.

The feature keeps the existing read-only product boundary: translation output
is returned to the caller, source files and editor buffers are not modified, and
providers receive only permitted translatable segments plus language and tone.

## Technical Context

**Language/Version**: Rust 2021 through the project Docker workflow pinned in
`Makefile` to `rust:1.96.1-bookworm`; Zed extension code remains Rust/WASM via
`zed_extension_api = "0.7.0"`.

**Primary Dependencies**: Existing `translator-core`, `translator-cli`,
`translator-mcp`, and `zed-extension/`. Add a minimal blocking HTTP client path
for the provider adapter in `translator-core`, planned as `ureq` 3.x with JSON
support plus `serde`/`serde_json` where needed. Dependency changes must update
`Cargo.lock` through the existing Docker workflow and remain project-scoped.

**Storage**: No product data storage. Provider configuration is read from
controlled process configuration and Zed context-server settings. Real `.env`
files, provider secrets, request logs, translated text, and provider responses
must not be versioned.

**Testing**: Future implementation tasks must start with failing tests/checks.
Expected validation surfaces are `make test-core`, `make test-mcp`,
`make test-zed-extension`, a new focused provider-config validation target,
`make test`, `make fmt`, and `make clippy`. Provider tests use local stubs or a
reviewer-supplied loopback service with synthetic text only.

**Target Platform**: Fedora/Linux development workstation. Repository Rust
checks stay inside the project Docker workflow. A local real provider service is
a user/reviewer prerequisite for manual smoke validation and must not be
installed globally by this feature.

**Project Type**: Rust workspace plus local Zed development extension and Spec
Kit documentation.

**Performance Goals**:

- Existing mock/offline tests remain unchanged and keep current latency.
- Provider calls obey the existing 15 s provider timeout budget.
- Direct-text and allowed-file translations stay within the existing 20 KiB
  input, 4 KiB segment, 256 segment, and 40 KiB output limits.
- Failure paths return normalized errors without waiting for provider timeout
  when configuration or remote confirmation is invalid.

**Constraints**:

- Mock/offline remains the default provider when no real provider is configured.
- The first real provider is local/self-hosted and free/no-pay from the product
  user's perspective.
- Provider configuration must not be carried in translated source text, file
  paths, workspace roots, MCP prompts, or logs.
- Remote provider targets are denied unless explicitly allowlisted in provider
  configuration and confirmed for the specific request.
- Secret detection runs before any non-local provider contact.
- The provider adapter must not use shell execution, arbitrary commands,
  inherited proxy configuration, or unreviewed request headers.
- No automatic editor edits, replacements, buffer writes, or source-file writes.
- Diagnostics and visible errors must remain redacted.
- Host footprint stays minimal; project-specific Rust tooling runs through the
  existing Makefile and Docker workflow.

**Scale/Scope**: One real provider family, one provider configuration boundary,
the two existing input paths (`translate_text` and `translate_file`), additive
remote confirmation, and focused provider failure/redaction tests. Publication,
custom Zed UI, provider lifecycle automation, paid managed provider setup, and
automatic replacement remain out of scope.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Safety-first translation**: PASS. The feature only changes the provider
  implementation path and keeps output read-only. Source buffers and files are
  never modified automatically.
- **Offline-first provider boundary**: PASS. Mock/offline remains default. The
  real path is explicit, local-first, and remote default-deny with per-request
  confirmation.
- **Test-first development**: PASS. Future tasks must begin with provider
  config, provider request, timeout, remote denial, secret blocking, redaction,
  CLI, MCP, and Zed wrapper tests before implementation changes.
- **Explicit contracts and limits**: PASS. Contracts below define provider
  configuration, provider invocation, additive remote confirmation, error
  mapping, and inherited size/timeout limits.
- **Minimal host footprint**: PASS. No host runtime, SDK, service, package, or
  real `.env` file is installed or versioned by this plan.

## Project Structure

### Documentation (this feature)

```text
specs/005-real-provider-config/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── checklists/
│   └── requirements.md
└── contracts/
    ├── provider-configuration.md
    └── libretranslate-provider.md
```

`tasks.md` is generated by the `speckit-tasks` phase and is the operative task
list for implementation.

### Source Code (repository root)

Likely implementation touch points after task generation:

```text
crates/
├── translator-core/
│   ├── src/
│   │   ├── provider.rs
│   │   ├── privacy.rs
│   │   └── ...
│   └── tests/
├── translator-cli/
│   ├── src/main.rs
│   └── tests/
└── translator-mcp/
    ├── src/
    └── tests/

zed-extension/
├── src/
│   ├── settings.rs
│   ├── launch.rs
│   └── lib.rs
└── tests/

tests/integration/
Makefile
Cargo.toml
Cargo.lock
```

**Structure Decision**: Keep translation behavior in `translator-core`, expose
it through the existing CLI and MCP boundaries, and let `zed-extension/` only
validate/pass controlled provider configuration to `translator-mcp`. Do not
create a second provider implementation inside the wrapper.

## Phase 0: Research

Research output is captured in [research.md](./research.md).

Decisions covered:

- Use a LibreTranslate-compatible local/self-hosted provider as the first real
  provider path.
- Keep provider configuration outside translated content and add only
  backward-compatible request confirmation data.
- Implement the provider adapter as Rust code behind the existing `Provider`
  boundary.
- Use a minimal blocking HTTP client with explicit timeout and no inherited
  proxy behavior.
- Treat remote provider support as default-deny and synthetic-only for
  validation unless the user explicitly confirms the request.

## Phase 1: Design And Contracts

Design outputs:

- [data-model.md](./data-model.md)
- [contracts/provider-configuration.md](./contracts/provider-configuration.md)
- [contracts/libretranslate-provider.md](./contracts/libretranslate-provider.md)
- [quickstart.md](./quickstart.md)
- [checklists/provider-privacy.md](./checklists/provider-privacy.md)

The contracts are additive to the completed translation and MCP contracts. Old
mock/local requests remain valid; remote confirmation is false when absent.

## Downstream Gate Status

- **Checklist**: Applied for provider/privacy requirements in
  [checklists/provider-privacy.md](./checklists/provider-privacy.md).
- **Tasks**: Generated in [tasks.md](./tasks.md) after running
  `.specify/scripts/bash/setup-tasks.sh --json`.
- **Analyze**: Executed after task generation. No critical or high-severity
  inconsistency was found; the non-blocking corrections were folded back into
  this plan and [tasks.md](./tasks.md).
- **Implement**: Executed against [tasks.md](./tasks.md) with focused provider
  tests, full workspace tests, formatting, and clippy validation.
- **Converge**: Final gate for this implementation pass. Converge is
  append-only to [tasks.md](./tasks.md); its outcome is reported in-session
  after implementation validation.

## Post-Design Constitution Check

- **Safety-first translation**: PASS. Contracts explicitly forbid source-file
  and editor-buffer mutation and constrain provider payloads to translated
  segments only.
- **Offline-first provider boundary**: PASS. Default provider is unchanged;
  non-local targets require explicit configuration and per-request confirmation.
- **Test-first development**: PASS. Quickstart and contracts identify the
  failing tests/checks future tasks must write first.
- **Explicit contracts and limits**: PASS. Provider configuration, provider
  payload, response validation, error mapping, and inherited limits are
  documented before implementation.
- **Minimal host footprint**: PASS. Provider runtime is a documented local
  prerequisite or stubbed test dependency, not a global install performed by the
  project.

## Complexity Tracking

No constitution violations are introduced.
