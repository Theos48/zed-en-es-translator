# Tasks: Plug-and-Play Zed Marketplace Installation

**Input**: Design documents from `/specs/009-zed-marketplace-install/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`,
`contracts/`, `quickstart.md`

**Tests**: Required. Every behavior-changing group starts with a failing
contract, acceptance or negative test and finishes with the focused passing
gate before broader validation.

**Constitution**: Preserve read-only translation, offline/default-deny provider
boundaries, explicit limits, log redaction and project-container development.

**Organization**: Tasks are grouped by user story so the zero-setup MVP can be
implemented and evaluated before recovery, offline/removal and unsupported
platform refinements.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can proceed in parallel because it changes distinct files and has no
  incomplete dependency.
- **[Story]**: Maps the task to a user story in `spec.md`.
- Every task names the concrete file or directory it changes or validates.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish release inputs and remove accidental workspace noise
without changing product behavior.

- [x] T001 Fix nested Rust build-output ignores after the `!crates/**` rule in `.gitignore` and verify the existing `crates/*/target/` residues remain untracked
- [x] T002 [P] Add the accepted MIT extension-code license at `zed-extension/LICENSE`
- [x] T003 [P] Add the public 20-case English-to-Spanish corpus and expected structural metadata in `tests/fixtures/marketplace/synthetic-corpus.json`
- [x] T004 [P] Add package/release fixture directories and controlled good/bad manifests under `tests/fixtures/marketplace/packages/`
- [x] T005 Record the exact Mozilla source/model identities and package budgets in `ops/marketplace/model.lock.json`; the final server identities remain fail-closed until T027 assembles `ops/marketplace/package.lock.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Restore only the tested local inference boundary needed by every
story and create reproducible package validation.

**⚠️ CRITICAL**: No user-story implementation starts until the portable runner,
core provider and package lock have executable failing-first contracts.

- [x] T006 [P] Add a failing schema/semantic contract for `ops/marketplace/package.lock.json` in `tests/integration/marketplace_package_lock.sh`
- [x] T007 [P] Add failing controlled native runner wire/limit/UTF-8/cardinality tests in `native/translator-embedded-runtime/tests/runner_contract.sh`
- [x] T008 [P] Add failing Rust process-boundary, timeout, redaction and adjacent-package tests in `crates/translator-core/tests/embedded_runner_boundary.rs` and `crates/translator-core/tests/embedded_provider.rs`
- [x] T009 Pin the native build tools and base-image digest in `docker/rust-toolchain.Dockerfile`, `.dockerignore` and `Makefile`
- [x] T010 Add the minimal C++17 Bergamot runner and portable build definition in `native/translator-embedded-runtime/src/main.cpp` and `native/translator-embedded-runtime/CMakeLists.txt`
- [x] T011 Implement the private runner wire and killable bounded child process in `crates/translator-core/src/embedded_protocol.rs` and `crates/translator-core/src/embedded_process.rs`
- [x] T012 Implement verified adjacent-package resolution and provider trait integration in `crates/translator-core/src/embedded_provider.rs`, `crates/translator-core/src/provider.rs` and `crates/translator-core/src/lib.rs`
- [x] T013 Add the `embedded_local` configuration matrix with no URL/key/remote allowance in `crates/translator-core/src/provider_config.rs` and focused configuration tests in `crates/translator-core/tests/embedded_provider_configuration.rs`
- [x] T014 Add exact native source/dependency locks, fetch/build scripts and offline CPU/ELF/reproducibility checks under `ops/marketplace/source.lock.json`, `scripts/marketplace/` and `tests/integration/marketplace_native_supply_chain.sh`
- [x] T015 Run the focused foundation gates through `Makefile` and record the passing commands in `specs/009-zed-marketplace-install/quickstart.md`

**Checkpoint**: A real portable local runner can be invoked only through the
bounded core provider; no acquisition or marketplace setting exists yet.

---

## Phase 3: User Story 1 - Install and Translate Without Setup (Priority: P1) 🎯 MVP

**Goal**: A clean supported Zed installation automatically prepares the fixed
package and produces a real read-only Spanish preview with no configuration.

**Independent Test**: Install the marketplace-shaped extension in a clean Zed
profile with no checkout, toolchain, provider or binary setting, open the public
fixture and obtain a non-Mock Spanish hover preview using only Zed.

### Tests for User Story 1 ⚠️

- [x] T016 [P] [US1] Add failing strict package-lock parsing, role uniqueness, budget and path-safety tests in `zed-extension/tests/package_lock.rs`
- [x] T017 [P] [US1] Add a failing 20-run automatic clean-preparation acceptance with a controlled 10 Mbps HTTPS downloader and a 19/20 under-five-minute threshold in `zed-extension/tests/acquisition_happy_path.rs`
- [x] T018 [P] [US1] Add a failing manifest/user-journey contract excluding context-server, binary-path and provider settings in `tests/integration/marketplace_no_setup.sh`
- [x] T019 [P] [US1] Add a failing LSP embedded-provider locality and read-only preview test in `crates/translator-lsp/tests/marketplace_embedded.rs`

### Implementation for User Story 1

- [x] T020 [US1] Implement strict published-lock and installed-state models in `zed-extension/src/package.rs`
- [x] T021 [US1] Implement fixed-source download, byte caps and SHA-256 verification in `zed-extension/src/acquisition.rs`
- [x] T022 [US1] Implement bounded pure-Rust Zstandard decoding and installed-object verification in `zed-extension/src/acquisition.rs`
- [x] T023 [US1] Implement the supported clean preparation and atomic package promotion path in `zed-extension/src/acquisition.rs`
- [x] T024 [US1] Replace manual LSP/context-server configuration with automatic direct-LSP launch and Zed installation statuses in `zed-extension/src/lib.rs` and `zed-extension/extension.toml`
- [x] T025 [US1] Resolve the runner/models adjacent to `translator-lsp` and select `embedded_local` without a user path in `crates/translator-lsp/src/state.rs` and `crates/translator-core/src/embedded_provider.rs`
- [x] T026 [US1] Create the deterministic LSP/runner/notices release archive in `scripts/marketplace/build-package.sh` and validate safe contents in `scripts/marketplace/validate-package.sh`
- [x] T027 [US1] Fill the released server file identities in `ops/marketplace/package.lock.json` and make the extension compile that exact lock
- [x] T028 [US1] Add `marketplace-package`, `test-marketplace-contract` and `test-marketplace-package` targets to `Makefile`
- [x] T029 [US1] Run three independent clean marketplace-shaped package preparations against the public smoke fixture and record 3/3 non-Mock translation, no-setup and no-mutation evidence in `specs/009-zed-marketplace-install/validation.md`

**Checkpoint**: User Story 1 is a complete MVP. It downloads, verifies and uses
the real package automatically; the repository's historical manual paths are
not part of the extension surface.

---

## Phase 4: User Story 2 - Recover Automatically From Preparation Problems (Priority: P2)

**Goal**: Interrupted, corrupt, concurrent or failed-update preparation is
retryable through Zed and never activates invalid content.

**Independent Test**: Inject network loss, truncation, corruption, storage
failure, interruption and two concurrent installers, then restore the condition
and complete through the normal action while a prior valid package remains
usable on update failure.

### Tests for User Story 2 ⚠️

- [x] T030 [P] [US2] Add failing truncation, oversize, hash, decode, storage and interrupted-staging cases in `zed-extension/tests/acquisition_failures.rs`
- [x] T031 [P] [US2] Add failing atomic state/current/previous transition cases in `zed-extension/tests/package_state.rs`
- [x] T032 [P] [US2] Add a failing two-process lock/stale-lock recovery contract in `tests/integration/marketplace_acquisition_concurrency.sh`
- [x] T033 [P] [US2] Add a failing invalid-update/last-known-good fallback acceptance in `zed-extension/tests/acquisition_rollback.rs`

### Implementation for User Story 2

- [x] T034 [US2] Implement exclusive preparation locking, bounded stale recovery and staging cleanup in `zed-extension/src/acquisition.rs`
- [x] T035 [US2] Implement atomic active/previous generation state and immutable package retention in `zed-extension/src/package.rs`
- [x] T036 [US2] Implement retry-safe error mapping, interruption cleanup and last-known-good selection in `zed-extension/src/acquisition.rs`
- [x] T037 [US2] Add content-free failure/retry diagnostics and focused redaction tests in `zed-extension/src/diagnostics.rs` and `zed-extension/tests/diagnostics_redaction.rs`

**Checkpoint**: User Stories 1 and 2 both work; no failed preparation requires
manual deletion and no failed update displaces a verified package.

---

## Phase 5: User Story 3 - Remain Private, Offline and Removable (Priority: P3)

**Goal**: After preparation all translation is offline, bounded and private,
and Zed disable/uninstall owns shutdown and complete data removal.

**Independent Test**: Prepare once, disable networking, run the public 20-case
set, inspect content-free logs/network evidence, then disable/uninstall through
Zed and validate the documented extension-owned boundary.

### Tests for User Story 3 ⚠️

- [x] T038 [P] [US3] Add failing zero-network-after-ready and acquisition-content privacy contracts in `tests/integration/marketplace_offline_privacy.sh`
- [x] T039 [P] [US3] Add a failing 20-case latency/RSS/thread/non-mutation benchmark harness in `tests/integration/marketplace_benchmark.sh`
- [x] T040 [P] [US3] Add failing active-package size, executable allowlist, license/notice and source-offer checks in `tests/integration/marketplace_release_contents.sh`
- [x] T041 [P] [US3] Add a disable/uninstall ownership requirements contract against current Zed source evidence in `tests/integration/marketplace_removal_contract.sh`

### Implementation for User Story 3

- [x] T042 [US3] Enforce that readiness and translation paths contain no download client and clear runner environment/network configuration in `zed-extension/src/acquisition.rs` and `crates/translator-core/src/embedded_process.rs`
- [x] T043 [US3] Add the exact native/model notices, MPL text and corresponding-source instructions under `ops/marketplace/licenses/`
- [x] T044 [US3] Add `test-marketplace-offline` and release-content gates to `Makefile`, execute the real 20-case network-disabled benchmark and record redacted metrics in `specs/009-zed-marketplace-install/validation.md`
- [x] T045 [US3] Document Zed-owned disable/uninstall behavior and zero external product state in `README.md` and `ops/marketplace/README.md`

**Checkpoint**: The prepared extension is a private offline product and its
complete runtime footprint is owned by normal Zed extension lifecycle.

---

## Phase 6: User Story 4 - Understand Unsupported Platforms (Priority: P4)

**Goal**: Unsupported hosts receive a stable in-editor explanation and perform
zero package requests or misleading setup guidance.

**Independent Test**: Present every non-Linux-`x86_64` platform variant exposed
by the extension API and observe the same content-free unsupported result with
no downloader invocation.

### Tests and Implementation for User Story 4 ⚠️

- [x] T046 [P] [US4] Add failing platform-matrix/zero-downloader tests in `zed-extension/tests/unsupported_platform.rs`
- [x] T047 [US4] Implement the pre-acquisition platform guard in `zed-extension/src/acquisition.rs`
- [x] T048 [US4] Add the stable unsupported message with no terminal/path recommendation in `zed-extension/src/diagnostics.rs`

**Checkpoint**: All four user stories are independently testable and the first
published platform boundary is explicit.

---

## Phase 7: Polish, Release and Cross-Cutting Gates

**Purpose**: Synchronize the stable product direction, validate the full
repository and move from a project package to an actual Gallery submission.

- [x] T049 [P] Update the user-first installation/usage/security/removal narrative in `README.md` and remove manual setup from the primary path
- [x] T050 [P] Record the marketplace architecture and replacement of the manual F012 lifecycle in `docs/adr/0006-zed-marketplace-package.md` and `docs/decisions.md`
- [x] T051 [P] Synchronize current direction/status in `docs/PLAN.md` and `docs/feature-map.md` without deleting future-platform backlog detail
- [x] T052 Review all 43 items in `specs/009-zed-marketplace-install/checklists/marketplace-release.md` against final artifacts and record any changed result inline
- [x] T053 Add the deterministic tagged-package CI/release workflow in `.github/workflows/marketplace-package.yml`
- [ ] T054 Add and run `marketplace-release-check` against the public tag/asset/version/lock in `Makefile` and `tests/integration/marketplace_release_check.sh`
- [x] T055 Run `make workspace-storage-check`, `make worktree-audit`, `make format`, `make fmt`, `make clippy`, `make deny` and `git diff --check`, fixing only feature-related findings
- [x] T056 Run `make test` plus every marketplace-focused target and record the complete redacted gate matrix in `specs/009-zed-marketplace-install/validation.md`
- [ ] T057 Run the pre-publication interactive Zed test with the exact release package (not a repository binary) and record redacted evidence in `specs/009-zed-marketplace-install/validation.md`
- [ ] T058 Publish the exact public project tag/release asset named by `ops/marketplace/package.lock.json` and rerun `make marketplace-release-check`
- [ ] T059 Commit and push the feature branch with Conventional Commits, open the project PR and derive every Spec Kit/test/manual/external gate in its body from `specs/009-zed-marketplace-install/validation.md`
- [ ] T060 Submit the HTTPS submodule/version change to `zed-industries/extensions`, then after upstream merge run three independent clean Gallery acceptances and append the 3/3 result to `specs/009-zed-marketplace-install/validation.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on setup and blocks every story.
- **US1 (Phase 3)**: Depends on foundation and creates the MVP package path.
- **US2 (Phase 4)**: Depends on US1 package/acquisition models.
- **US3 (Phase 5)**: Depends on the ready package and can overlap with late US2
  work in distinct test/docs files.
- **US4 (Phase 6)**: Depends only on the US1 acquisition entry point and can
  proceed alongside US2/US3.
- **Release (Phase 7)**: Documentation tasks may begin after US1; release,
  full validation and submissions depend on all desired stories.

### User Story Dependencies

- **US1 (P1)**: Foundation only; independently delivers install + real
  translation.
- **US2 (P2)**: Uses US1 package/acquisition but is independently testable with
  controlled failure sources.
- **US3 (P3)**: Uses a ready US1 package; its offline/privacy/removal acceptance
  does not require US2 failure injection.
- **US4 (P4)**: Uses only the acquisition entry point; must short-circuit before
  any US1 download behavior.

### Within Each Story

- Add the listed tests first and observe the expected focused failure.
- Implement models/contracts before orchestration.
- Make the focused story gate pass before broader regression tests.
- Do not use controlled download/runner fixtures as final real-package evidence.
- Preserve unrelated user changes and build residues throughout.

### External Dependency

T060 has two parts. Creating the upstream submission is in project control;
merging it and making the registry entry available are controlled by Zed
maintainers. If review is pending, record the exact PR/check state and keep only
the post-merge Gallery acceptance open. Do not substitute a dev extension.

## Parallel Opportunities

- T002-T004 change independent license/fixture paths.
- T006-T008 introduce independent lock, native and Rust failing contracts.
- T016-T019 cover separate extension/integration/LSP test surfaces.
- T030-T033 cover independent recovery classes before shared implementation.
- T038-T041 cover offline, resource, release and removal requirement surfaces.
- T049-T051 update distinct strategic documents after the architecture is
  implemented.

## Parallel Example: User Story 1

```text
T016: zed-extension package-lock tests
T017: zed-extension controlled acquisition acceptance
T018: repository no-setup manifest contract
T019: translator-lsp embedded/local read-only contract
```

## Parallel Example: Recovery and Offline Gates

```text
T030: corrupt/interrupted acquisition cases
T031: atomic state transitions
T032: cross-process concurrency contract
T033: last-known-good update fallback

T038: offline/network privacy contract
T039: real resource benchmark harness
T040: release content/license/budget contract
T041: disable/uninstall ownership contract
```

## Implementation Strategy

### MVP First

1. Complete Setup and Foundation.
2. Complete US1 through T029.
3. Stop and prove the exact package translates the public fixture with zero
   settings and no mutation.
4. Only then add recovery, offline/removal and unsupported-platform hardening.

### Incremental Delivery

1. Foundation: portable bounded local inference.
2. US1: automatic Gallery-shaped package and real direct translation.
3. US2: safe retry/concurrency/last-known-good behavior.
4. US3: real offline/privacy/resource/removal evidence.
5. US4: clean unsupported-platform failure.
6. Release: tagged package, project PR, central registry PR, post-merge Gallery
   acceptance.

## Notes

- `[P]` tasks operate on distinct files but still respect phase prerequisites.
- User-story labels provide spec traceability; setup/foundation/release tasks
  intentionally have no story label.
- Rust changes require the repository `rust-best-practices` skill and relevant
  ownership/error/performance/test references before editing.
- Rust/native execution uses only Make/Docker; no host toolchain installation.
- T060 may remain the sole externally blocked acceptance only after the actual
  upstream PR exists and all project-controlled work passes.
