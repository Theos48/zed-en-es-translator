# Manual validation: Embedded Local Provider

**Feature**: `008-embedded-local-provider`
**Date**: 2026-07-15
**Branch**: `008-embedded-local-provider`
**Outcome**: `BLOCKED_LICENSE_APPROVAL`

## Decision

The embedded provider is not promoted, enabled, bundled or represented as a
supported path. The exact model/resource set has no completed artifact-level
license conclusion and no human project-maintainer approval for local
acquisition/activation. Publication remains separately blocked for F009.

The committed manifest therefore keeps `review_status=blocked`, has no local
approval, exposes no consent digest and rejects preparation before creating an
XDG provider root. This is the required fail-closed outcome; Mock and the
supported LibreTranslate path remain unchanged.

## Evidence completed

| Gate | Redacted result |
|---|---|
| Spec Kit specify / clarify / plan / checklists | Passed; requirements and design artifacts are present |
| Controlled Rust/native wire tests | Passed at the recorded US1 checkpoint |
| Native source identity | Exact source commit and 15 direct dependency gitlinks verified |
| Offline native build | Passed with project container networking disabled |
| Reproducibility | Two clean builds produced SHA-256 `9743b4a8efbe9471145c08fcc75a42fdc3d85e6035e797023b3a623d91e886fe` and size `12000008` |
| CPU baseline | Passed for `x86-64` plus SSE4.1; no `-march=native` |
| ELF closure | Passed exact allowlist; no dynamic socket/HTTP/TLS symbols detected |
| Blocked preparation | Passed: invalid/unavailable consent creates no provider state |

The final regression, formatting, lint, dependency-policy and evidence-contract
commands all passed as recorded below.

## Gates not executed

The following are blocked by the missing human license/local-activation
approval and are deliberately not simulated with controlled fixtures:

- real model acquisition and installed dual-identity verification;
- real 20-case benchmark, RSS/thread/latency and zero-network measurements;
- real non-Mock CLI translation and interactive direct-Zed acceptance;
- real invalid update, post-promotion recovery, offline rollback and exact
  cleanup against approved model artifacts;
- F009 bundling, redistribution and publication review.

No approval, observed resource value, real translation result or manual Zed
interaction is inferred from source metadata or controlled tests.

Two controlled implementation gaps also remain explicit in `tasks.md`: crash
injection at every staging boundary is not complete at shell level, and the
one-shot runner architecture does not yet provide an unambiguous
`warm_provider` measurement class. A reviewer must decide whether that class
means warm operating-system page cache for repeated one-shot processes or
requires the separately gated persistent-process redesign. Neither is silently
assumed by the current benchmark harness.

## Final command record

| Command | Result |
|---|---|
| `make test-embedded-provider` | PASS: core/CLI/LSP/Zed fixture, native offline build/supply-chain, manager lifecycle and evidence contract |
| `make test-real-provider-config` | PASS |
| `make test-direct-zed-translation` | PASS |
| `make test-zed-extension` | PASS |
| `make test-mcp` | PASS |
| `make test` | PASS: complete Rust workspace and isolated Zed extension |
| `make fmt` | PASS |
| `make clippy` | PASS with warnings denied |
| `make deny` | PASS: advisories, bans, licenses and sources; informational duplicate-version warnings remain |
| `make worktree-audit` | PASS: one registered checkout on persistent storage |

## Redaction and mutation audit

- `git diff --check` passes.
- The pinned source checkout is clean after the offline native build.
- Blocked preparation, lifecycle and evidence tests remove their isolated
  provider roots; the normal user-scoped embedded root remains absent.
- The evidence contract accepts exactly 20 public cases, rejects a blocked
  benchmark before state mutation and finds no prohibited evidence fields.
- Repository secret-pattern review found only the pre-existing intentional
  security fixtures and detectors; no new credential material was found.
- Committed evidence contains only safe artifact identities, fixed case IDs,
  normalized outcomes and aggregate build facts. It contains no translation
  output, raw child output, credentials, environment values or host identity.
