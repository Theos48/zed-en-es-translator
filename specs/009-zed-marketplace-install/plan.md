# Implementation Plan: Plug-and-Play Zed Marketplace Installation

**Branch**: `009-zed-marketplace-install` | **Date**: 2026-07-16 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from
`/specs/009-zed-marketplace-install/spec.md`

## Summary

Publish the existing direct English-to-Spanish workflow as a normal Zed
extension whose supported journey is install, open a document and translate.
The extension uses Zed's documented language-server installation flow to
download one fixed Linux `x86_64` package into the extension `work` directory,
verify every executable and model resource, and launch `translator-lsp` with
the embedded runtime selected internally. Translation stays offline after that
preparation, and Zed removes the owned package when the extension is
uninstalled.

The implementation deliberately does not carry forward the user-visible F012
lifecycle manager. It reuses only the tested bounded native process boundary,
the portable translation runner and exact model identities from the prototype.
There is no user command, Docker lifecycle, arbitrary path, runtime setting,
account, key, service or separate XDG store.

## Technical Context

**Language/Version**: Rust 2021 on Rust 1.96.1 for the core, LSP, package
acquisition and Zed WASM extension; C++17 for the already prototyped bounded
translation runner

**Primary Dependencies**: Existing `translator-core` and `translator-lsp`;
`zed_extension_api = 0.7.0`; pure-Rust SHA-256 and Zstandard decoding in the
extension; Mozilla Translations/Bergamot source pinned at
`f31423c7c2c6ed8ae57d71a3d19a9db6f156060e`; exact Firefox Translations
`en -> es` version-3.0 base-memory resources

**Storage**: Immutable versioned packages, staging, an exclusive preparation
lock and a tiny active/previous state record inside Zed's extension-owned
`work/en-es-translator` directory; no workspace, host-global or independent
product data root

**Testing**: Rust unit/contract/integration tests and native build checks
through the project Makefile and pinned Docker image; deterministic acquisition
doubles; real package build, real model acquisition, offline corpus benchmark,
marketplace-shaped clean-profile test and final interactive Zed acceptance

**Target Platform**: Zed on Linux `x86_64`, CPU-only local inference; other
operating systems and architectures fail before acquisition

**Project Type**: Multi-crate Rust editor integration with one Zed WASM
extension, one Rust language server and one private native child process

**Performance Goals**: First preparation within five minutes on a 10 Mbps
connection in at least 95% of controlled runs; 20/20 offline cases under the
existing 15-second request deadline; active package below 128 MiB, peak RSS
below 1 GiB and at most four inference threads

**Constraints**: No terminal, checkout, build, container, service, account,
key, path or runtime setting in the user journey; 20 KiB request, 4 KiB
segment, 256 segments and 40 KiB output; no source mutation or content logs;
fixed HTTPS sources only; verify before activation; last-known-good fallback;
extension-work storage only; Zed Extension Gallery publication rules

**Scale/Scope**: One extension ID, one platform package, one language pair, one
direct LSP UX, three model resources, one current and at most one previous
package, 20 public translation fixtures and one marketplace submission

## Spec Kit Execution Record

Commands executed for this cycle:

```bash
git fetch origin main
git switch -c 009-zed-marketplace-install origin/main
.specify/scripts/bash/create-new-feature.sh --json --number 9 \
  --short-name zed-marketplace-install '<confirmed feature description>'
.specify/scripts/bash/check-prerequisites.sh --json --paths-only
.specify/scripts/bash/setup-plan.sh --json
```

`speckit-clarify` asked zero questions: product scope and acceptance behavior
were already testable, while the runtime and package mechanism belonged to
Phase 0 research. The optional agent-context hook is executed after the Phase
1 artifacts exist.

## Constitution Check

### Before Phase 0: PASS

- **Safety-first translation**: The existing read-only hover preview remains;
  neither acquisition nor translation edits a buffer, file or clipboard.
- **Single offline product boundary**: The marketplace path selects only the
  verified adjacent embedded runtime. Acquisition carries no document data,
  and feature 010 removes every configurable translation path before release.
- **Test-first development**: Every behavior change begins with extension,
  package, process or release-contract tests. Real clean-install and offline
  evidence supplement controlled doubles.
- **Explicit contracts and limits**: Existing request/result/error, segment,
  output and timeout contracts remain authoritative. Phase 1 adds versioned
  package, acquisition and publication contracts.
- **Minimal host footprint**: Development remains in the pinned project
  container. Runtime files live only in Zed's extension work directory; no
  host package, runtime, service or configuration is installed.

### After Phase 1: PASS

The package model makes readiness atomic and keeps incomplete content in
staging. The extension validates platform, fixed identities, sizes and hashes
before activation; the LSP launches only an adjacent verified runner without a
shell or inherited network configuration. Zed owns both installation and
uninstall cleanup. The release contract includes accepted extension licensing,
native notices, MPL source links and exact model attribution. No constitutional
exception is required.

## Phase 0 Research Decisions

Complete in [research.md](./research.md):

1. follow the official Zed auto-downloaded language-server pattern;
2. replace the manual F012 lifecycle with one extension-owned package state;
3. retain the compact Bergamot/Firefox resource set, but make preparation
   automatic and publication obligations explicit;
4. download the Rust LSP/native runner from a fixed project release and the
   three model resources from fixed Mozilla attachment URLs;
5. verify compressed and installed identities before atomic activation;
6. keep inference in a bounded native child process and all acquisition in the
   extension, never in translation runtime;
7. use a previous verified package only when a current update fails;
8. submit the public repository as a submodule PR to
   `zed-industries/extensions` after the real clean-install gate passes.

No `NEEDS CLARIFICATION` marker remains.

## Project Structure

### Documentation (this feature)

```text
specs/009-zed-marketplace-install/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── acquisition.md
│   ├── package-lock.schema.json
│   ├── publication.md
│   ├── translate-result.schema.json
│   └── translation-package.md
├── checklists/
│   └── requirements.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── translator-core/
│   └── src/
│       ├── embedded_process.rs       # bounded child-process boundary
│       ├── embedded_protocol.rs      # versioned private runner wire
│       └── embedded_provider.rs      # verified adjacent runtime boundary
└── translator-lsp/                   # existing direct, read-only Zed UX

native/
└── translator-embedded-runtime/      # compact C++17 Bergamot runner

zed-extension/
├── extension.toml                    # Gallery manifest; direct LSP only
├── LICENSE                           # accepted extension-code license
└── src/
    ├── acquisition.rs                # platform/download/verify/activate
    ├── package.rs                    # fixed lock and local state model
    └── lib.rs                        # automatic LSP command

ops/marketplace/
├── package.lock.json                 # exact published identities and sources
├── licenses/                         # shipped notices and source instructions
└── README.md                         # maintainer release record

scripts/marketplace/
├── build-package.sh                  # deterministic release asset
└── validate-package.sh               # offline artifact gate

tests/
├── fixtures/marketplace/             # controlled packages and public corpus
└── integration/                      # no-setup, offline, failure, release gates
```

**Structure Decision**: Keep acquisition and update state in the Zed extension
because only it knows the sanctioned `work` directory and installation status.
Keep translation and process limits in the existing Rust core/LSP. Reuse the
native runner source from the prior prototype, but do not import its separate
lifecycle or manual preparation journey.

## TDD and Verification Order

1. failing package-lock/platform/state and no-manual-settings tests;
2. failing download, hash, decompression, interruption and concurrency tests;
3. failing adjacent-runtime/process/timeout/redaction tests;
4. native reproducible-build, CPU, ELF, notice and package-budget checks;
5. direct LSP read-only regression and 20-case real offline benchmark;
6. pre-submission dev-extension smoke against the exact public package;
7. format, lint, dependency/license audit and release contract;
8. submit the extension-registry PR after every project-controlled release gate
   passes;
9. after registry merge, run 3/3 marketplace clean-profile
   install/retry/disable/uninstall acceptances without a dev extension.

No controlled fixture may substitute for the real package in the final
clean-install or offline acceptance evidence. A pre-submission dev-extension
smoke is useful evidence but cannot close the post-merge Gallery gate.
