<!--
Sync Impact Report
Version change: template -> 1.0.0
Modified principles:
- Template principle 1 -> I. Safety-First Translation
- Template principle 2 -> II. Offline-First Provider Boundary
- Template principle 3 -> III. Test-First Development
- Template principle 4 -> IV. Explicit Contracts And Limits
- Template principle 5 -> V. Minimal Host Footprint
Added sections:
- Security And Privacy Requirements
- Development Workflow And Quality Gates
Removed sections:
- None
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
original buffer. No feature may replace, rewrite, or auto-edit editor content
unless a later constitution amendment explicitly permits it. The system MUST
preserve Markdown structure and protected code regions, and it MUST preserve
ambiguous content instead of guessing. Full-file code translation is gated until
there is a reliable segmenter/parser and negative tests proving that code is
not translated accidentally.

Rationale: the main product risk is destructive translation of code or docs.
Reading assistance is useful only if it does not alter source material.

### II. Offline-First Provider Boundary

The default provider MUST be offline/mock and deterministic. Remote providers
MUST be disabled by default, configured explicitly, and confirmed by the user
for each translation request. Provider interfaces MUST receive only permitted
translatable segments, language metadata, and tone. Providers MUST NOT receive
file paths, workspace roots, protected code, environment variables, logs, or
detected secrets.

Rationale: translation content may contain proprietary code, credentials,
internal URLs, prompts, or private documentation. Cost-free internet is not the
same as privacy-safe internet.

### III. Test-First Development

Every implementation task that changes behavior MUST start with failing tests
or explicit checks. Core behavior MUST use TDD. Required coverage includes
contract tests, unit tests for segmentation and reconstruction, CLI integration
tests, and negative security tests. Tests MUST include path traversal, symlink
escape, size limits, non-UTF-8 input, binary content, remote calls without
confirmation, secret detection before remote calls, provider timeout, and log
redaction.

Rationale: the important behavior is defensive and easy to regress unless it is
executable as tests.

### IV. Explicit Contracts And Limits

All boundaries MUST be documented and versioned before implementation:
`TranslateRequest`, success/error output, `ErrorCode`, provider segment
contract, and TypeScript-to-Rust CLI wire contract. The MVP contract uses JSON
UTF-8 over stdin/stdout, one request per process, exit code 0 for success,
non-zero for failure, redacted stderr, and a timeout. Input and output limits
MUST be explicit: 20 KiB input, 4 KiB per segment, 256 segments, 40 KiB output,
and 15 s provider timeout unless amended.

Rationale: Zed, MCP, TypeScript, Rust, and future providers are separate
surfaces. Ambiguous contracts create security and integration bugs.

### V. Minimal Host Footprint

Project dependencies MUST be scoped to the project whenever possible. Global
runtimes, SDKs, package managers, services, databases, or model installs are
not allowed unless they are classified and approved under the host system
policy. The project MUST use lockfiles, reviewed dependency versions, and
supply-chain checks before publishing or enabling real providers. Real `.env`
files and secrets MUST NOT be versioned.

Rationale: the workstation policy prioritizes a clean Fedora host and
maintainable development environments.

## Security And Privacy Requirements

`translate_file` MUST read only inside the Zed-authorized workspace after
canonicalizing the workspace and requested path. It MUST reject path traversal,
unauthorized absolute paths, symlinks that escape the workspace, sensitive
hidden files, binary content, non-UTF-8 content, and unsupported extensions.

Logs MUST NOT include source text, translated text, translatable segments,
secrets, headers, tokens, or sensitive paths without redaction. Logs may include
request ids, error codes, redacted provider names, sizes, durations, and
redacted status.

Future CLI providers MUST use allowlisted binaries, no shell execution,
structured arguments, controlled cwd, minimal environment, stdin for text,
timeout, and stdout/stderr limits. Future HTTP-local providers MUST be
loopback-only. Future HTTP-remote providers MUST use HTTPS, host allowlists, no
dangerous redirects, request-level confirmation, and privacy review.

## Development Workflow And Quality Gates

Development follows the official Spec Kit flow:

1. Constitution.
2. Specify.
3. Clarify when needed.
4. Plan.
5. Tasks.
6. Analyze when useful.
7. Implement.

The first formal feature MUST be technical and offline: Rust core,
`MockProvider`, TypeScript-to-Rust CLI contract, limits, file-read security, and
negative tests. It MUST NOT include a real provider, network calls, automatic
buffer edits, or full-file code support.

Before implementation, each feature plan MUST pass the Constitution Check. Any
violation MUST be documented with a simpler alternative and rationale. Before
closing a feature, its quickstart or validation notes MUST show how to run the
relevant tests/checks.

## Governance

This constitution supersedes conflicting project planning documents. If a
document conflicts with this constitution, update the document or amend the
constitution before implementing.

Amendments require:

- a new entry in `docs/decisions.md` or an ADR;
- a clear rationale;
- a semantic version bump;
- updates to affected Spec Kit templates or feature specs.

Versioning policy:

- MAJOR for removing or weakening a core principle;
- MINOR for adding a principle, section, or material requirement;
- PATCH for wording clarifications that do not change obligations.

Compliance review is required during `/speckit-plan`, `/speckit-tasks`, and
before implementation begins. TDD, privacy default deny, workspace-only file
access, and log redaction are non-negotiable gates.

**Version**: 1.0.0 | **Ratified**: 2026-07-01 | **Last Amended**: 2026-07-01
