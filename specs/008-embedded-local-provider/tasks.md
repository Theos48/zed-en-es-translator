# Tasks: Embedded Local Provider

**Input**: Design documents from `/specs/008-embedded-local-provider/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`,
`contracts/`, `quickstart.md`, completed requirements checklists

**Tests**: Required. Every behavior-changing task follows TDD; controlled
doubles cover failures, but only real offline CLI/direct-Zed evidence may close
the provider-promotion gate.

**Organization**: Tasks are grouped by user story and executed in dependency
order. Every task names its principal file or directory and traces to the
requirements/contracts it fulfills.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish versioned project structure and test entry points without
activating or downloading a provider.

- [x] T001 Add `crates/translator-provider-manager` to the root workspace and create its library/binary manifests in `Cargo.toml` and `crates/translator-provider-manager/Cargo.toml`
- [x] T002 [P] Create the minimal native wrapper project skeleton and portable-build policy in `native/translator-embedded-runtime/CMakeLists.txt` and `native/translator-embedded-runtime/README.md`
- [x] T003 [P] Create reviewed manifest schema skeletons with publication blocked in `ops/providers/embedded/provider.lock.json` and `ops/providers/embedded/source.lock.json`
- [x] T004 [P] Create artifact-license review and source-offer placeholders that fail closed until completed in `ops/providers/embedded/licenses/README.md`
- [x] T005 Add embedded build/cache/model/staging exclusions while preserving versioned locks and fixtures in `.gitignore` and `.dockerignore`
- [x] T006 Add planned embedded lifecycle/build/test targets and help text without host-global commands in `Makefile`
- [x] T007 [P] Add shell contract coverage that asserts Make target names, no `sudo`/package/service mutation, and generic-clean preservation in `tests/integration/embedded_provider_make_targets.sh`
- [x] T008 Run `make workspace-storage-check` before any build and record the setup checkpoint in `specs/008-embedded-local-provider/tasks.md`

**Checkpoint**: Structure and interfaces exist; no network acquisition or product
behavior has changed.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement shared validated identities, storage state, process
boundary and normalized errors required by every user story.

**CRITICAL**: No user-story implementation starts until this phase passes.

- [x] T009 [P] Write failing serialization/schema/unknown-field tests for profile, runner, model, set and state entities in `crates/translator-provider-manager/tests/manifest_contract.rs`
- [x] T010 [P] Write failing state-transition/atomic-generation tests for absent, candidate, current and previous references in `crates/translator-provider-manager/tests/state_contract.rs`
- [x] T011 [P] Write failing Linux storage-policy tests for XDG scope, owner/mode, persistent filesystem, symlink, hard-link and containment failures in `crates/translator-provider-manager/tests/storage_security.rs`
- [x] T012 [P] Write failing bounded-process tests for exact executable/args/cwd/env, stdout/stderr saturation, malformed protocol, timeout kill/reap and no retry in `crates/translator-core/tests/embedded_runner_boundary.rs`
- [x] T013 [P] Write failing error-redaction tests covering child, manifest, storage and lifecycle failures in `crates/translator-core/tests/embedded_provider_redaction.rs`
- [x] T014 Implement typed manifest/profile/license approval validation with `Result` errors and no production unwraps in `crates/translator-provider-manager/src/manifest.rs`
- [x] T015 Implement immutable artifact-set and atomic installation-state types/transitions in `crates/translator-provider-manager/src/state.rs`
- [x] T016 Implement fixed XDG root derivation and Linux-safe owned-path validation in `crates/translator-provider-manager/src/storage.rs`
- [x] T017 Implement normalized lifecycle error hierarchy and content-free diagnostics in `crates/translator-provider-manager/src/error.rs`
- [x] T018 Implement the versioned runner request/response types and semantic limit validation in `crates/translator-core/src/embedded_protocol.rs`
- [x] T019 Implement a generic injectable process runner with concurrent bounded pipes, total deadline, kill/reap and no retry in `crates/translator-core/src/embedded_process.rs`
- [x] T020 Export only the required embedded types and preserve crate-level documentation in `crates/translator-core/src/lib.rs` and `crates/translator-provider-manager/src/lib.rs`
- [x] T021 Implement manager CLI parsing with explicit subcommands and normalized exits in `crates/translator-provider-manager/src/main.rs`
- [x] T022 Add dependency/license policy for new Rust crates and native exclusions in `deny.toml` and `Cargo.lock`
- [x] T023 Run the focused foundational Rust tests through the project container and mark T009-T022 complete in `specs/008-embedded-local-provider/tasks.md`
- [x] T024 Re-run `make fmt` and focused `make clippy` coverage for the new crates before user-story work in `Makefile`

**Checkpoint**: Safe typed foundation and testable process/storage boundaries are
ready; no real provider is selected yet.

---

## Phase 3: User Story 1 - Translate Locally Without a Managed Service (Priority: P1) MVP

**Goal**: Explicit `embedded_local` selection translates one already-prepared,
verified local set through CLI and direct Zed with the same existing safety
boundary, no remote confirmation and no service.

**Independent Test**: Install a controlled verified runner fixture, disable
networking, translate permitted public synthetic Markdown through CLI and LSP,
and prove ordered real-fixture output, `[offline]`, unchanged source and no
fallback.

### Tests for User Story 1

- [x] T025 [P] [US1] Write failing exhaustive four-key configuration tests for `embedded_local`, conflicts, Mock default and explicit-unready failure in `crates/translator-core/tests/embedded_provider_configuration.rs`
- [x] T026 [P] [US1] Write failing provider tests for one ordered batch, existing limits, invalid responses, timeout and no Mock/remote fallback in `crates/translator-core/tests/embedded_provider.rs`
- [x] T027 [P] [US1] Write failing CLI process tests for embedded success, absent state, stderr redaction and source non-mutation in `crates/translator-cli/tests/cli_embedded_provider.rs`
- [x] T028 [P] [US1] Write failing LSP tests for offline locality, no remote prompt, read-only preview and redacted failures in `crates/translator-lsp/tests/embedded_provider_locality.rs`
- [x] T029 [P] [US1] Write failing Zed extension tests proving only `TRANSLATOR_PROVIDER=embedded_local` is forwarded and no artifact path/download is accepted in `zed-extension/tests/embedded_provider_settings.rs`
- [x] T030 [P] [US1] Write failing native wire tests for strict JSON, ordered cardinality, content-free errors and no listening/network behavior in `native/translator-embedded-runtime/tests/runner_contract.sh`

### Implementation for User Story 1

- [x] T031 [US1] Add `EmbeddedLocal` mode and exact conflict matrix without new public path/env keys in `crates/translator-core/src/provider_config.rs`
- [x] T032 [US1] Implement `EmbeddedProcessProvider` construction from one verified immutable set and stable error mapping in `crates/translator-core/src/embedded_provider.rs`
- [x] T033 [US1] Integrate embedded selection into shared provider dispatch while preserving Mock/Libre/Azure behavior in `crates/translator-core/src/provider.rs`
- [x] T034 [US1] Implement the minimal C++17 Bergamot batch wrapper with no downloader/updater/logging/service in `native/translator-embedded-runtime/src/main.cpp`
- [x] T035 [US1] Integrate embedded selection and offline locality into CLI/LSP startup and direct-Zed labels in `crates/translator-cli/src/main.rs`, `crates/translator-lsp/src/main.rs`, and `crates/translator-lsp/src/state.rs`
- [x] T036 [US1] Preserve the four-key Zed launch/settings allowlist for embedded mode in `zed-extension/src/settings.rs` and `zed-extension/src/launch.rs`
- [x] T037 [US1] Add release-shaped embedded runner build and focused US1 test targets in `Makefile`
- [x] T038 [US1] Pass all US1 controlled-fixture tests offline and record the independent checkpoint in `specs/008-embedded-local-provider/tasks.md`

US1 checkpoint (2026-07-15): `make test-embedded-provider-us1` passes core,
CLI, LSP, Zed-settings and strict native wire-fixture coverage. The same C++17
entry point now links the pinned Bergamot source in production mode; the
real-artifact activation gate remains independently blocked by human review.

**Checkpoint**: P1 works independently with a controlled preprepared set; real
Bergamot evidence is still required before promotion.

---

## Phase 4: User Story 2 - Prepare Reviewed Artifacts With Informed Consent (Priority: P2)

**Goal**: Disclose the exact reviewed artifact set and prepare it only after
manifest-digest consent, with verified scoped artifacts and no privilege.

**Independent Test**: From an empty isolated XDG test root, reject preparation
with no/mismatched consent and observe zero changes; accept the exact digest
against controlled HTTPS fixtures and activate only after dual-hash,
compatibility, license and offline-smoke gates pass.

### Tests for User Story 2

- [x] T039 [P] [US2] Write failing disclosure tests for purpose, source, license role, sizes, scope, network, publication block and exact consent digest in `crates/translator-provider-manager/tests/disclosure.rs`
- [x] T040 [P] [US2] Write failing acquisition-policy tests for exact HTTPS, status 200, no redirects/proxy/retry, byte caps and cancellation in `crates/translator-provider-manager/tests/acquisition_policy.rs`
- [x] T041 [P] [US2] Write failing integrity tests for compressed/installed hash, size, language, platform, compatibility and approval failures in `crates/translator-provider-manager/tests/artifact_integrity.rs`
- [x] T042 [P] [US2] Write failing integration tests for rejection, interrupted stages, idempotent reuse and atomic first promotion in `tests/integration/embedded_provider_prepare.sh`

### Implementation for User Story 2

- [x] T043 [US2] Implement bounded content-free disclosure bound to the full manifest digest in `crates/translator-provider-manager/src/disclosure.rs`
- [x] T044 [US2] Implement exact-source HTTP acquisition with disabled proxy/redirect/retry and streaming byte/hash limits in `crates/translator-provider-manager/src/acquisition.rs`
- [x] T045 [US2] Implement single-file Zstandard expansion, dual identity validation and immutable object finalization in `crates/translator-provider-manager/src/artifact.rs`
- [x] T046 [US2] Implement consent validation, staging, offline smoke gate, fsync and atomic first promotion in `crates/translator-provider-manager/src/lifecycle.rs`
- [x] T047 [US2] Add thin no-privilege disclose/prepare orchestration in `scripts/providers/embedded.sh`
- [x] T048 [US2] Wire `provider-embedded-disclose` and consented `provider-embedded-prepare` targets in `Makefile`
- [x] T049 [US2] Pass the complete US2 controlled acquisition matrix and record zero-mutation rejection evidence in `specs/008-embedded-local-provider/tasks.md`

**Checkpoint**: First preparation is explicit, verified and atomic; normal
translation remains separate from downloader code.

US2 controlled checkpoint (2026-07-15):
`./tests/integration/embedded_provider_prepare.sh` passed through the project
Make/Docker path. Approved fixture manifests exercise exact consent, dual
identities, real process smoke, idempotent reuse and atomic promotion. Injected
failures at staging creation, staged objects, finalized objects, finalized set
and persisted candidate never create a current reference; a retry rejects any
stale candidate and promotes cleanly. The blocked production manifest and
mismatched consent both reject before network/root mutation.

---

## Phase 5: User Story 3 - Update, Recover, and Remove the Local Path (Priority: P3)

**Goal**: Provide safe status, verify, update, rollback and complete removal with
serialization, leases, known-good preservation and offline recovery.

**Independent Test**: Prepare current, stage a corrupt candidate, prove current
is unchanged, promote a valid controlled candidate, rollback offline, exercise
busy/concurrent failures, and remove only manifest-owned state with the exact
token.

### Tests for User Story 3

- [x] T050 [P] [US3] Write failing lock/lease/concurrent lifecycle tests including inference during update and busy removal in `crates/translator-provider-manager/tests/concurrency.rs`
- [x] T051 [P] [US3] Write failing safe status/verify tests for bounded metadata, offline hashing/smoke and redaction in `crates/translator-provider-manager/tests/status_verify.rs`
- [x] T052 [P] [US3] Write failing update/rollback tests for invalid candidate, post-promotion failure, missing previous and offline recovery in `crates/translator-provider-manager/tests/update_rollback.rs`
- [x] T053 [P] [US3] Write failing cleanup tests for exact token, unknown entries, unsafe links, active lease and generic-clean preservation in `crates/translator-provider-manager/tests/cleanup.rs`
- [x] T054 [P] [US3] Write failing end-to-end lifecycle shell tests for status, verify, update, rollback and clean in `tests/integration/embedded_provider_lifecycle.sh`

### Implementation for User Story 3

- [x] T055 [US3] Implement exclusive lifecycle locks, short state locks and shared inference leases within the provider deadline in `crates/translator-provider-manager/src/locking.rs` and `crates/translator-core/src/embedded_provider.rs`
- [x] T056 [US3] Implement bounded status and offline verify with no acquisition fallback in `crates/translator-provider-manager/src/status.rs`
- [x] T057 [US3] Implement separately staged update, post-promotion verification and current/previous preservation in `crates/translator-provider-manager/src/lifecycle.rs`
- [x] T058 [US3] Implement offline previous revalidation and atomic rollback without destructive failure recovery in `crates/translator-provider-manager/src/lifecycle.rs`
- [x] T059 [US3] Implement manifest-enumerated cleanup with exact token, exclusive lease, unknown-entry refusal and root preservation in `crates/translator-provider-manager/src/cleanup.rs`
- [x] T060 [US3] Wire status, verify, update, rollback and clean subcommands/Make targets in `crates/translator-provider-manager/src/main.rs`, `scripts/providers/embedded.sh`, and `Makefile`
- [x] T061 [US3] Pass the complete US3 controlled lifecycle matrix and record the independent checkpoint in `specs/008-embedded-local-provider/tasks.md`

**Checkpoint**: All lifecycle commands are reversible, offline where required
and independently tested with controlled artifacts.

US3 controlled checkpoint (2026-07-15):
`./tests/integration/embedded_provider_lifecycle.sh` passed through the project
Make/Docker path. The CLI routes prepare and update separately; update preflight
rejects an absent/current-equal set before acquisition. The controlled matrix
passes private physical staging, durable candidate recovery, current/previous
preservation, post-promotion restoration, offline verify/rollback, concurrent
inference, busy/unsafe cleanup and exact owned-root removal. Interruptions
before candidate persistence preserve byte-identical current state; an
interrupted durable candidate is rejected before a successful retry.

---

## Phase 6: User Story 4 - Review Product Fit and Publication Readiness (Priority: P4)

**Goal**: Produce reproducible candidate/build/license/resource/product evidence
and choose exactly `PROMOTED` or a safe `BLOCKED_<GATE>` outcome without
overstating publication rights.

**Independent Test**: Reproduce the exact native build and artifact identities,
run the fixed 20-case benchmark and CLI/direct-Zed acceptance with networking
disabled, review every required approval, and derive the documented go/no-go
result solely from recorded gates.

### Tests for User Story 4

- [x] T062 [P] [US4] Write failing source-lock tests for exact commit/submodules, offline build, portable CPU flags, ELF allowlist and reproducible runner hash in `tests/integration/embedded_native_supply_chain.sh`
- [x] T063 [P] [US4] Write failing evidence-schema/redaction tests for allowed metrics and prohibited content/host identity in `tests/integration/embedded_evidence_contract.sh`
- [x] T064 [P] [US4] Create the versioned 20-case public synthetic corpus and structural quality expectations in `tests/fixtures/embedded/synthetic-corpus.json`
- [x] T065 [P] [US4] Write the release benchmark harness with fixed warmups/repetitions/rounds and hard budgets in `tests/integration/embedded_benchmark.sh`

### Implementation and Evidence for User Story 4

- [ ] T066 [US4] Resolve and record exact official model/runtime URLs, record IDs, dual hashes, sizes, compatibility and local/publication approvals in `ops/providers/embedded/provider.lock.json`
- [x] T067 [US4] Pin every native source/dependency revision, build recipe, compiler/CPU flags and ELF closure in `ops/providers/embedded/source.lock.json`
- [ ] T068 [US4] Complete artifact-level license/SBOM/notice/source-offer review with separate local and F009 publication decisions in `ops/providers/embedded/licenses/README.md`
- [x] T069 [US4] Build the exact native helper inside the project container with network disabled after fetch and pass `tests/integration/embedded_native_supply_chain.sh`
- [ ] T070 [US4] Prepare the exact real artifact set with informed consent and record only redacted acquisition/integrity outcomes in `specs/008-embedded-local-provider/manual-validation.md`
- [ ] T071 [US4] Run the real 20-case release benchmark with external networking disabled and record all mandatory resource/latency fields in `specs/008-embedded-local-provider/manual-validation.md`
- [ ] T072 [US4] Run one real CLI acceptance and one interactive direct-Zed acceptance with `[offline]`, no mutation and no external contacts in `specs/008-embedded-local-provider/manual-validation.md`
- [ ] T073 [US4] Exercise real invalid update, post-promotion recovery, offline rollback and exact cleanup and record redacted outcomes in `specs/008-embedded-local-provider/manual-validation.md`
- [x] T074 [US4] Record exactly `PROMOTED` or `BLOCKED_<GATE>` from the complete evidence and update support/publication wording in `specs/008-embedded-local-provider/manual-validation.md`, `README.md`, and `docs/PLAN.md`

**Checkpoint**: The provider has an honest evidence-derived promotion/no-go
state; F009 remains a separate publication decision.

US4 blocked checkpoint (2026-07-15): the three official Mozilla Remote
Settings records were re-read by exact ID. All locked names, roles, `en-es`
language, `base-memory` architecture, version, attachment locations, compressed
and decompressed hashes/sizes match. The record schema exposes neither a
`license` nor an `spdx` field. The reproducible runner's native link/license
inventory, actual-binary SBOM and exact unresolved items are recorded in
`ops/providers/embedded/licenses/README.md` and
`ops/providers/embedded/licenses/native-sbom.json`. T066/T068 remain open
because accepted artifact-level conclusions, notice/source handling and both
human approval records are absent; T070-T073 therefore remain correctly blocked
and were not simulated. On 2026-07-16 the maintainer accepted Marian
`any_type.h` as part of the MIT-licensed Marian project, reducing the explicit
SBOM blocking review items from five to four without authorizing acquisition or
F009 publication.

---

## Phase 7: Polish and Cross-Cutting Validation

**Purpose**: Close regressions, documentation, security and Spec Kit evidence.

- [x] T075 [P] Update embedded lifecycle/privacy/resource/recovery guidance and explicit unsupported scopes in `specs/008-embedded-local-provider/quickstart.md` and `README.md`
- [x] T076 [P] Add a single credential-free `test-embedded-provider` target covering Rust/native/lifecycle/evidence gates in `Makefile`
- [x] T077 Run `make test-embedded-provider`, `make test-real-provider-config`, `make test-direct-zed-translation`, `make test-zed-extension`, `make test-mcp`, and `make test` and record results in `specs/008-embedded-local-provider/manual-validation.md`
- [x] T078 Run `make fmt`, `make clippy`, and `make deny` through the pinned project container and record results in `specs/008-embedded-local-provider/manual-validation.md`
- [x] T079 Audit diagnostics, benchmark/evidence outputs, repository changes and XDG cleanup for prohibited content and unrelated mutation in `specs/008-embedded-local-provider/manual-validation.md`
- [x] T080 Re-run both Spec Kit checklists, mark only evidence-backed items complete, and update gate status in `specs/008-embedded-local-provider/checklists/requirements.md` and `specs/008-embedded-local-provider/checklists/embedded-artifact-safety.md`
- [x] T081 Run the agent-context update hook and final `git diff --check`/link/task-format audits in `AGENTS.md` and `specs/008-embedded-local-provider/tasks.md`

---

## Dependencies and Execution Order

### Phase Dependencies

- **Phase 1 Setup**: Starts immediately and performs no provider acquisition.
- **Phase 2 Foundational**: Depends on Phase 1 and blocks every user story.
- **US1 / Phase 3**: Depends on Phase 2; produces the MVP with controlled
  preprepared artifacts.
- **US2 / Phase 4**: Depends on Phase 2 and integrates with US1's profile/runner
  contracts; independently proves consented first preparation.
- **US3 / Phase 5**: Depends on US2 lifecycle state and US1 inference leases.
- **US4 / Phase 6**: Depends on US1-US3; only this phase may promote or block the
  candidate from real evidence.
- **Polish / Phase 7**: Depends on all implemented stories and the honest US4
  conclusion.

### Within Each User Story

- Write the listed tests first and observe the focused failure.
- Implement typed models before services and integration.
- Run focused tests before marking a task complete.
- Do not mark real acceptance with controlled doubles.
- Do not promote when any mandatory measurement or human approval is absent.

### Parallel Opportunities

- T002-T004 and T007 operate on independent setup files.
- T009-T013 are independent failing foundational test suites.
- T025-T030 cover different US1 crates/surfaces.
- T039-T042 cover separate US2 requirement clusters.
- T050-T054 cover separate US3 lifecycle behaviors.
- T062-T065 cover independent US4 supply-chain/evidence inputs.
- T075-T076 can proceed in parallel after the product conclusion is known.

## Parallel Example: User Story 1

```text
T025 core configuration tests
T026 core provider/runner tests
T027 CLI contract tests
T028 LSP locality/non-mutation tests
T029 Zed allowlist tests
T030 native wire tests
```

These tests touch separate files and can be authored in parallel; production
configuration/dispatch changes T031-T036 remain ordered by their shared types.

## Implementation Strategy

### MVP First

1. Complete Setup and Foundational phases.
2. Complete US1 using a controlled verified runner fixture.
3. Prove shared CLI/direct-Zed offline behavior independently.
4. Continue to real artifact preparation only after the safe boundary passes.

### Incremental Delivery

1. US1: local inference boundary with controlled artifacts.
2. US2: informed, integrity-checked first preparation.
3. US3: reversible lifecycle and exact cleanup.
4. US4: real candidate evidence and honest promotion/no-go result.
5. Polish: full regression and review readiness.

## Notes

- `[P]` means different principal files and no dependency on an incomplete task.
- Every task uses the existing Make/Docker project workflow; no host Rust/C++
  runtime, service or package installation is authorized.
- Network is allowed only for explicit reviewed source/artifact fetch tasks;
  native build after fetch and normal provider operations are offline.
- Interactive Zed evidence, human license approval and any external publication
  decision cannot be fabricated; if unavailable they leave the relevant task
  open and force a documented `BLOCKED_<GATE>` result.

## Phase 8: Convergence

**Purpose**: Record work discovered by the post-implementation audit without
rewriting the original execution history.

- [x] T082 Resolve and encode the `warm_provider` benchmark execution class in
  `specs/008-embedded-local-provider/plan.md`,
  `specs/008-embedded-local-provider/data-model.md`, and
  `specs/008-embedded-local-provider/contracts/validation-evidence.md`: either
  define it as repeated one-shot processes with a warm operating-system page
  cache, or approve a persistent-process architecture change before collecting
  promotion evidence.

T082 decision (2026-07-15): `warm_provider` preserves the verified one-shot
boundary and means only repeated launches after five warmups with a warm
operating-system page cache. T065 records one pre-warmup `new_process` probe,
300 warm-provider matrix samples, and fails closed on every applicable hard
budget. Real-model measurements remain exclusively T071 evidence.

## Phase 9: Convergence

- [x] T083 Update the Spec Kit execution/gate record in
  `specs/008-embedded-local-provider/plan.md` to reflect the completed
  tasks/analyze/implement/converge passes and their exact remaining blockers
  in the `Spec Kit Gate Status` table
