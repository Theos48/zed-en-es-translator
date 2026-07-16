<!--
Sync Impact Report
Version change: 1.0.0 -> 2.0.0
Modified principles:
- I. Safety-First Translation -> unchanged obligations, product boundary clarified
- II. Offline-First Provider Boundary -> II. Single Offline Product Boundary
- III. Test-First Development -> retained and aligned to Gallery/LSP gates
- IV. Explicit Contracts And Limits -> retired CLI/MCP wire; retained package/LSP/runtime contracts
- V. Minimal Host Footprint -> unchanged obligations, Zed-owned runtime clarified
Added sections:
- Repository Convergence Requirements
Removed sections:
- Future CLI/HTTP provider requirements
Templates requiring updates:
- .specify/templates/plan-template.md: updated
- .specify/templates/spec-template.md: updated
- .specify/templates/tasks-template.md: updated
Follow-up TODOs:
- None
-->

# zed-en-es-translator Constitution

## Core Principles

### I. Safety-First Translation

The product MUST help the user read translated content without modifying the
original buffer or source file. No feature may replace, rewrite or auto-edit
editor content unless a later constitution amendment explicitly permits it.
The system MUST preserve Markdown structure and protected code regions, and it
MUST preserve ambiguous content instead of guessing. Full-file code
translation remains gated until a reliable segmenter/parser and negative tests
prove that code is not translated accidentally.

Rationale: reading assistance is useful only when it cannot damage the source
material that the user is inspecting.

### II. Single Offline Product Boundary

The only supported product path MUST be the Zed Gallery extension launching its
verified, extension-owned local package through `translator-lsp`,
`translator-core` and the private embedded runtime. Translation MUST remain
local and offline after acquisition of fixed public package inputs. The shipped
path MUST NOT expose a selectable provider, endpoint, credential, arbitrary
binary, MCP/context server, Agent workflow or standalone CLI. `MockProvider`
MAY exist only as an injected deterministic test double.

Rationale: one immutable local path is simpler to install, audit and support,
and prevents configuration from silently widening the privacy boundary.

### III. Test-First Development

Every behavior change and removal wave MUST start with failing tests or an
explicit negative contract. Core behavior MUST use TDD. Required retained
coverage includes segmentation and reconstruction, read-only LSP behavior,
path traversal and symlink escape, size limits, non-UTF-8 and binary input,
process time/output limits, package acquisition and identity, offline
operation, log redaction and Zed-owned removal. Obsolete surface tests MAY be
deleted only after every live invariant has equivalent retained coverage.

Rationale: the product's important behavior is defensive and easy to regress;
repository deletion is safe only when the surviving boundary remains
executable as tests.

### IV. Explicit Contracts And Limits

All active boundaries MUST be documented and versioned before implementation:
the read-only LSP request/preview flow, core translation request/result/error
types, provider segment contract, embedded process protocol, package manifest,
acquisition identity and repository cleanup allowlists. The active limits MUST
remain explicit: 20 KiB input, 4 KiB per segment, 256 segments, 40 KiB output
and a 15 s translation timeout unless amended. Retired CLI and MCP wire
contracts MUST NOT remain as supported interfaces.

Rationale: Zed, the LSP, core, private runner and fixed package are separate
trust boundaries. Explicit limits and identities prevent ambiguous execution
and supply-chain behavior.

### V. Minimal Host Footprint

Project dependencies MUST be scoped to the project, its pinned build container
or Zed's extension work directory. Global runtimes, SDKs, package managers,
services, databases or model installs are not allowed unless classified and
approved under the host system policy. The project MUST use lockfiles, reviewed
dependency versions and supply-chain checks before publication. Real `.env`
files and secrets MUST NOT be versioned. Normal cleanup MUST preserve fixed
source inputs and MUST NOT target agent configuration, secrets or persistent
user data.

Rationale: the workstation and installed extension remain maintainable when
runtime state has a single owner and reproducible project inputs are bounded.

## Security And Privacy Requirements

The product MUST reject path traversal, unauthorized absolute paths, symlinks
that escape the authorized workspace, sensitive hidden files, unsupported file
types, binary content and non-UTF-8 input before translation. Providers receive
only permitted translatable segments, language metadata and tone; they MUST NOT
receive file paths, workspace roots, protected code, environment variables or
logs.

Logs MUST NOT include source text, translated text, translatable segments,
secrets, headers, tokens, raw child output or sensitive paths. Diagnostics may
include request ids, error codes, fixed component names, sizes, durations and
redacted status.

The embedded process MUST be resolved from the verified adjacent package,
started without a shell, given structured arguments and bounded stdin, and
subject to timeout and stdout/stderr limits. Translation-time network access
and configuration-driven executable/provider selection are prohibited.

## Repository Convergence Requirements

Every tracked product, release, validation and documentation path MUST have a
named consumer in the Gallery product or its governance/release process.
Unsupported compatibility paths MUST be removed rather than hidden behind
optional features. Git is the archive for completed implementation history;
ADRs retain concise decisions and MUST mark superseded operational guidance
with an explicit replacement.

Generated cleanup MUST use a previewable allowlist. Normal cleanup may remove
build and validation output while preserving fixed reusable source inputs. Any
deep cache cleanup MUST be separately explicit and MUST never select `.git/`,
`.agents/`, `.codex/`, secrets, real `.env` files or persistent user data.

## Development Workflow And Quality Gates

Development follows the official Spec Kit flow:

1. Constitution.
2. Specify.
3. Clarify when needed.
4. Plan.
5. Tasks.
6. Analyze.
7. Implement.
8. Converge.

Before implementation, each feature plan MUST pass the Constitution Check and
all domain checklists MUST be complete. Behavior changes require failing tests
or explicit negative checks first. Rust format, Clippy, tests and dependency
policy MUST run through the project Makefile and pinned container, never a host
toolchain. Before publication, package acquisition, identity, offline,
privacy, license, resource, removal and clean-install gates MUST pass against
the exact candidate.

## Governance

This constitution supersedes conflicting project planning documents. If a
document conflicts with it, update the document or amend the constitution
before implementing.

Amendments require:

- a new entry in `docs/decisions.md` or an ADR;
- a clear rationale;
- a semantic version bump;
- updates to affected Spec Kit templates and active feature artifacts.

Versioning policy:

- MAJOR for removing or redefining a core principle or supported boundary;
- MINOR for adding a principle, section or material requirement;
- PATCH for wording clarifications that do not change obligations.

Compliance review is required during `/speckit-plan`, `/speckit-tasks`,
`/speckit-analyze` and before implementation begins. TDD, read-only behavior,
local/offline translation, workspace-safe file access, bounded processes and
log redaction are non-negotiable gates.

**Version**: 2.0.0 | **Ratified**: 2026-07-01 | **Last Amended**: 2026-07-16
