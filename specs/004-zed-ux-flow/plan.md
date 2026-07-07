# Implementation Plan: Zed UX Flow

**Branch**: `004-zed-ux-flow` | **Date**: 2026-07-06 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/004-zed-ux-flow/spec.md`

**Spec Kit Flow**: This plan was prepared after running the real Spec Kit
scripts for this feature:

```bash
SPECIFY_PROMPT='Promote F007 as the fourth formal Spec Kit feature after the Zed wrapper merge. Define a polished in-editor reading workflow for Zed where a user can complete an English-to-Spanish translation without leaving the editor. The feature must build on the already merged local Zed development extension and existing MCP tools. It must document a low-friction Agent Panel based flow, produce readable translation results, preserve the original buffer and files without automatic edits, keep direct text and authorized workspace-file inputs bounded by the active translate_text and translate_file contracts, and manually validate the real Zed selection behavior before claiming selection support. Keep real providers, remote network translation, marketplace publication, API-key setup, arbitrary provider configuration, and automatic replacement/editing out of scope. Preserve the constitution: offline/mock by default, no source mutation, no unsafe file access, no secret leakage, and no logs or diagnostics containing source text, translated text, tokens, headers, environment dumps, or sensitive paths.'
.specify/scripts/bash/create-new-feature.sh --json --number 4 --short-name zed-ux-flow "$SPECIFY_PROMPT"
.specify/scripts/bash/setup-plan.sh --json
```

The same `speckit-specify` prompt is preserved in [spec.md](./spec.md). The plan
command consumes that specification and expands it into the design artifacts
below. This iteration stops at planning so the feature can be reviewed before
task generation or implementation.

## Summary

Promote F007 into the fourth formal Spec Kit feature by defining a polished,
safe, low-friction Zed reading workflow on top of the merged local development
extension and existing MCP tools. The feature uses Zed's Agent Panel and MCP
tool path as the primary UX surface, keeps the already implemented
`translate_text` and `translate_file` contracts as the only supported
translation inputs, and requires manual Zed validation before selection support
is claimed.

The plan deliberately does not add a real provider, remote network translation,
marketplace publication, arbitrary provider configuration, API-key setup, or
automatic buffer replacement. The product boundary remains read-only: the user
can see translated Spanish output inside Zed, but source buffers and files are
not modified by the tool.

## Technical Context

**Language/Version**: Rust 1.96.1 through the project Docker workflow
(`rust:1.96.1-bookworm`). The existing Zed wrapper is an isolated Rust/WASM
extension crate using `zed_extension_api = "0.7.0"`.

**Primary Dependencies**: Existing `translator-core`, existing
`translator-mcp`, existing `zed-extension/` local development extension, Zed
Agent Panel, and Zed MCP context-server integration. No new runtime dependency
is planned for this planning phase.

**Storage**: N/A. The feature defines an in-editor request/result workflow and
manual validation evidence. It must not add persistent project secrets,
provider config, generated local settings, or source-file mutations.

**Testing**: Future implementation tasks must start with checks/tests. Expected
validation surfaces are `make test-zed-extension`, `make test`, `make fmt`,
`make clippy`, focused integration scripts if behavior changes, and manual Zed
smoke validation for Agent Panel behavior, selection handling, redaction, and
no-mutation evidence. Manual validation must also record the Agent model route,
tool-permission posture, and synthetic/redacted evidence used to inspect tool
inputs.

**Target Platform**: Fedora/Linux development workstation with Zed installed by
the user outside this project workflow. Repository checks stay inside the
project Docker workflow. Any missing host prerequisite remains governed by the
workstation policy and is not silently installed by this feature.

**Project Type**: Rust workspace plus local Zed development extension and Spec
Kit documentation.

**Performance Goals**:

- Reviewer completes one direct-text translation inside Zed in 3 minutes or
  less after the local extension is already prepared and registered.
- Successful output is readable without inspecting raw MCP protocol messages,
  terminal logs, or repository internals.
- Failure categories are visible enough for correction without exposing source
  text, translated text, secrets, tokens, headers, environment dumps, or
  sensitive paths.

**Constraints**:

- Offline/mock provider remains the default and only in-scope provider path.
- No network translation, no remote provider setup, and no API-key workflow.
- No automatic editor edits, replacements, buffer writes, or source-file writes.
- No sensitive validation content may be sent through an unverified non-local
  Agent model route. Non-local or unknown Agent model routes are limited to
  synthetic canary text.
- Agent tool permissions must be restricted to the local translator workflow:
  built-in edit/write/delete/move/copy, terminal, URL fetch, and web-search
  tools must be denied or require confirmation before no-mutation evidence can
  be accepted.
- Direct text and workspace-file input remain bounded by the active MCP
  contracts and existing constitution limits.
- Selection support is gated on real Zed validation because Agent Panel context,
  model/tool availability, and selected-content forwarding must be observed in
  the actual UI before the feature claims support.
- Diagnostics and visible errors must remain redacted.
- Host footprint stays minimal; project-specific Rust tooling runs through the
  existing Makefile and Docker workflow.

**Scale/Scope**: One canonical Zed UX flow, two supported input paths
(`translate_text` and `translate_file`), one recorded selection-support
decision, and one manual validation protocol. No new provider, no registry
publication, no source editor automation.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Safety-first translation**: PASS. The feature is explicitly read-only and
  forbids automatic buffer, file, and selection mutation. The plan now requires
  a translation-only Agent Profile or equivalent permission setup before
  no-mutation claims are accepted.
- **Offline-first provider boundary**: PASS. The only in-scope provider path is
  the existing deterministic offline/mock translator behavior. Remote/provider
  fields remain rejected, and the separate Zed Agent model route is documented
  as a validation privacy boundary.
- **Test-first development**: PASS. The plan stops before implementation and
  requires future tasks to begin with executable checks or manual validation
  evidence before code changes are accepted.
- **Explicit contracts and limits**: PASS. The feature references the active
  MCP tool contracts for `translate_text` and `translate_file`, including
  language direction, input limits, file safety, output shape, and redaction.
- **Minimal host footprint**: PASS. No global project runtime, SDK, service,
  provider, or package installation is introduced. Manual Zed prerequisites are
  handled outside the repository workflow and under host policy.

## Project Structure

### Documentation (this feature)

```text
specs/004-zed-ux-flow/
├── plan.md
├── spec.md
├── research.md
├── data-model.md
├── quickstart.md
├── tasks.md
├── manual-validation-template.md
├── checklists/
│   ├── pre-tasks.md
│   └── requirements.md
└── contracts/
    ├── manual-validation.md
    └── ux-flow.md
```

`tasks.md` is produced in the later `speckit-tasks` phase; this branch includes
the generated task artifact for review.

### Source Code (repository root)

No source edits are planned by this plan-only step. If review approves moving
to task generation and implementation, likely touch points are:

```text
zed-extension/
├── Cargo.toml
├── extension.toml
└── src/

crates/
├── translator-core/
└── translator-mcp/

tests/
└── integration/

docs/
```

The expected implementation bias is to improve documentation, validation, and
thin integration behavior around the existing MCP tools before adding any new
code. New code is only justified if the manual Zed flow exposes a concrete UX
or diagnostic gap that cannot be closed by documentation and validation.

**Structure Decision**: Keep the existing repository architecture. The local
Zed extension remains a wrapper around `translator-mcp`; translation behavior
continues to live in `translator-core` and `translator-mcp`. This feature owns
the Zed user workflow and validation evidence, not a second translation engine.

## Phase 0: Research

Research output is captured in [research.md](./research.md).

Decisions covered:

- Use Agent Panel plus MCP tools as the baseline UX surface.
- Treat direct text and authorized workspace file input as the supported
  baseline.
- Treat the Zed Agent model route as a separate privacy boundary from the local
  translator.
- Require a translation-only Agent Profile or equivalent tool-permission setup
  for manual validation.
- Gate selection support on manual Zed validation.
- Keep provider, remote, publication, and automatic replacement out of scope.

## Phase 1: Design And Contracts

Design outputs:

- [data-model.md](./data-model.md)
- [contracts/ux-flow.md](./contracts/ux-flow.md)
- [contracts/manual-validation.md](./contracts/manual-validation.md)
- [quickstart.md](./quickstart.md)

The contracts define user-visible behavior and evidence requirements rather
than new network or storage APIs.

## Post-Design Constitution Check

- **Safety-first translation**: PASS. Contracts require no-mutation evidence
  and forbid automatic edits; tool-permission requirements reduce the chance
  that unrelated Agent tools mutate files during validation.
- **Offline-first provider boundary**: PASS. Research and contracts preserve
  offline/mock-only translator behavior, remote/provider denial, and explicit
  Agent model-route evidence.
- **Test-first development**: PASS. The manual validation contract defines
  reviewable checks that future tasks must satisfy before claiming completion.
- **Explicit contracts and limits**: PASS. UX contracts reference the existing
  MCP schemas and error/result shapes instead of inventing a parallel interface.
- **Minimal host footprint**: PASS. Quickstart separates repository validation
  from host prerequisites and does not prescribe new global installation.

## Complexity Tracking

No constitution violations are introduced.
