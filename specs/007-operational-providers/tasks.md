# Tasks: Operational Real Providers

**Input**: Design documents from `/specs/007-operational-providers/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`,
`contracts/`, `quickstart.md`, and the provider operations/privacy checklist

**Tests**: Mandatory. Every behavior change follows RED -> GREEN -> REFACTOR.
Automated tests use controlled doubles and must never contact LibreTranslate or
Azure, require credentials, or retain source/translation content.

**Rust quality**: Apply `rust-best-practices`: borrow payload data where
possible, avoid redundant allocation/cloning, return typed `Result` values,
never use `unwrap`/`expect` in production paths, keep errors redacted, prefer a
small statically dispatched test seam over new runtime type erasure, document
public invariants/errors, and keep Clippy at `-D warnings`.

**Compatibility scope**: MCP/Agent Panel is a compatibility bridge only. F011
adds no MCP-specific provider flow or real-service acceptance task; existing
MCP regression suites remain part of the final automatic gate.

**Organization**: Tasks are grouped by user story. Each story begins with
tests/checks that must demonstrably fail before its implementation tasks.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it changes different files and has no
  dependency on another incomplete task in the same phase.
- **[Story]**: Maps the task to `US1`, `US2`, `US3`, or `US4` from `spec.md`.
- Every task names the exact repository path(s) it changes or validates.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish failing project-interface checks, public fixtures, and
reusable controlled-test infrastructure without implementing provider
behavior.

- [X] T001 Add independently runnable failing contract cases for `provider-cache/` Docker/Git exclusions, local/recovery/gate Make targets, script delegation, `translator-cli-release`, and provider-state preservation by ordinary `clean` in `tests/integration/operational_provider_make_targets.sh`
- [X] T002 [P] Create public synthetic cases and safe expected metadata in `tests/fixtures/operational-providers/plain.txt`, `tests/fixtures/operational-providers/protected.md`, `tests/fixtures/operational-providers/synthetic-secret.txt`, and `tests/fixtures/operational-providers/expected-metadata.json`
- [X] T003 [P] Add reusable fake Docker/Compose, HTTP-contact counter, temporary-state, and redaction assertions in `tests/integration/lib/operational_provider_helpers.sh`

---

## Phase 2: Foundational Provider Configuration (Blocking Prerequisite)

**Purpose**: Make the three-mode configuration and safe locality classification
a single fail-closed foundation for every story.

**⚠️ CRITICAL**: Complete this phase before implementing any provider story.

### Tests first (RED)

- [X] T004 [P] Add failing table-driven tests for mock default, exact local profile, exact Azure profile, missing/empty/conflicting/mode-inapplicable values, safe key-reference names, URL overrides, and proxy-disable invariants in `crates/translator-core/tests/operational_provider_configuration.rs`
- [X] T005 [P] Add failing Zed configuration tests for the same four-key matrix, duplicate keys, absence of raw key values, Azure URL rejection, and safe emitted environment in `zed-extension/tests/provider_settings.rs`
- [X] T006 [P] Add failing tests for offline/local/remote labels derived from one parsed configuration without provider name, URL, model, path, tier, or key-reference leakage in `crates/translator-lsp/tests/operational_provider_locality.rs`

### Minimal implementation (GREEN, then REFACTOR)

- [X] T007 Implement `ProviderMode::AzureTranslator`, exhaustive mode-specific validation, exact LibreTranslate operational target validation, Azure URL prohibition, key-reference requirements, and redacted typed failures in `crates/translator-core/src/provider_config.rs`
- [X] T008 Implement the matching fail-closed Azure/local/mock settings matrix while omitting inapplicable environment entries in `zed-extension/src/settings.rs`
- [X] T009 Derive the safe locality descriptor exhaustively from the parsed provider mode/target without duplicating secret-bearing state in `crates/translator-lsp/src/state.rs`
- [X] T010 Run the focused RED-to-GREEN configuration suites through `Makefile`, then verify existing provider configuration, diagnostics-redaction, and mock-default tests still pass without weakening assertions

**Checkpoint**: Configuration is exhaustive and default-deny; no network
adapter or real service is needed to validate this phase.

---

## Phase 3: User Story 1 - Translate With a Real Local Provider (Priority: P1) 🎯 MVP

**Goal**: Prepare and run the pinned LibreTranslate profile inside the project,
translate through CLI/direct Zed after egress is disabled, preserve protected
content, and leave mock as the default.

**Independent Test**: Use controlled Docker/HTTP doubles to prove preparation,
offline restart, local CLI translation, safe Zed locality, Markdown
preservation, and non-mutation; after automatic gates pass, perform the two
explicit real local acceptance rows using only public synthetic fixtures.

### Tests first (RED)

- [X] T011 [P] [US1] Add a failing static contract test for pinned image tag+digest, loopback-only port, internal runtime network, `pull_policy: never`, English/Spanish-only loading, disabled UI/file translation/auto-update, 4 CPU/4 GiB caps, and fixed Compose project identity in `tests/integration/operational_provider_contract.sh`
- [X] T012 [P] [US1] Add failing controlled lifecycle tests from an isolated clean checkout for prepare, exact 120-second readiness timeout, candidate verification, promotion, repeated start/status/verify/stop, port conflict, interrupted preparation, absence of global host changes, safe output, and current-slot preservation in `tests/integration/provider_local_lifecycle.sh`
- [X] T013 [P] [US1] Add failing no-egress tests proving prepared start/verify/translation/stop cannot pull, download, resolve external targets, or use a non-loopback endpoint in `tests/integration/provider_local_offline.sh`
- [X] T014 [P] [US1] Add failing CLI tests for exact local selection, real-adapter output through a controlled loopback server, mock default, Markdown/protected-region preservation, limits, and byte-for-byte source non-mutation in `crates/translator-cli/tests/cli_operational_providers.rs`

### Minimal implementation (GREEN, then REFACTOR)

- [X] T015 [US1] Add schema version, exact image identity, reviewed manifest/platform data, Argos index revision, project-observed model hashes, and unresolved-license/no-redistribution metadata in `ops/providers/libretranslate/provider.lock`
- [X] T016 [US1] Add the project-scoped candidate/current/previous Compose topology with a provider-only internal network, hardened loopback relay, persistent slots, health configuration, resource caps, immutable image, and no normal-runtime pull in `ops/providers/libretranslate/compose.yaml`
- [X] T017 [US1] Implement lock parsing, disk/resource preflight, exact artifact verification, candidate-only preparation, an enforced 120-second readiness deadline with synthetic probes, offline re-verification, and atomic safe promotion in `scripts/providers/libretranslate.sh`
- [X] T018 [US1] Implement idempotent start/status/verify/stop against only the fixed Compose project and active verified slot, with `Result`-style shell exits and redacted bounded output in `scripts/providers/libretranslate.sh`
- [X] T019 [US1] Make the relevant T001 contract cases GREEN by excluding `provider-cache/` in `.dockerignore`, preserving its `.gitignore` exclusion, and exposing `provider-local-prepare`, `provider-local-start`, `provider-local-status`, `provider-local-verify`, `provider-local-stop`, and `translator-cli-release` through `Makefile` without duplicating lifecycle logic or changing ordinary `clean`
- [X] T020 [US1] Replace planning placeholders with the implemented local prerequisites, resource/network boundary, commands, offline proof, non-mutation check, and no-redistribution warning in `specs/007-operational-providers/quickstart.md`
- [X] T021 [US1] Finalize but do not execute the clean-checkout `LOCAL-CLI-01`/`LOCAL-ZED-01` procedure, 120-second readiness and 15-second invocation checks, safe case IDs, and redacted fields in `specs/007-operational-providers/manual-validation.md`; real execution is deferred to T056 after T054 passes

**Checkpoint**: US1 is independently testable as the offline-first MVP with
controlled doubles. Real execution remains deferred to T056 and never
authorizes global installation, unrestricted Docker prune, or retention of
translated content.

---

## Phase 4: User Story 2 - Use a Real Remote Provider With Explicit Consent (Priority: P2)

**Goal**: Add the fixed Azure Translator v3 F0 path while proving exact target,
minimal payload, safe credential reference, fresh consent, secret blocking,
bounded response validation, and zero automatic retries.

**Independent Test**: A controlled transport records attempted contacts and
proves denial/dismissal/stale/mismatch/reuse/secret cases make zero calls while
one freshly confirmed safe request sends only ordered segments and fixed
language metadata; real Azure validation remains a separate approved manual
gate.

### Tests first (RED)

- [X] T022 [P] [US2] Add failing adapter tests for the exact Azure HTTPS host/path/query, fixed `en`/`es`, internal technical-neutral tone/format validation, rejection of unsupported future modes before contact, absence of invented external metadata, ordered borrowed segments, only the key header, minimal JSON elements, response cardinality/order, non-empty Spanish results, and 40 KiB aggregate bound in `crates/translator-core/tests/azure_translator_provider.rs`
- [X] T023 [P] [US2] Add failing tests for DNS/TLS/HTTP 408/auth/quota/rate-limit/other HTTP failures, timeout, redirect, malformed/non-text/empty/mismatched/oversized responses, and zero retry in `crates/translator-core/tests/azure_translator_failures.rs`
- [X] T024 [P] [US2] Add failing tests proving request/response types, errors, and diagnostics never expose keys, headers, endpoint details, raw bodies, source segments, translations, or environment contents in `crates/translator-core/tests/operational_provider_redaction.rs`
- [X] T025 [P] [US2] Add failing CLI tests for missing/denied confirmation, fresh confirmation, second-request re-confirmation, stale/mismatched/reused consent, pre-contact secret blocking, safe key references, and normalized stderr/exit behavior in `crates/translator-cli/tests/cli_operational_providers.rs`
- [X] T026 [P] [US2] Add failing direct-LSP tests proving one parsed configuration creates both selection and label, every request gets a new confirmation, denial/dismissal/mismatch makes zero contacts, and hover/source remain read-only in `crates/translator-lsp/tests/operational_provider_locality.rs`
- [X] T027 [P] [US2] Add failing extension tests for Azure mode, omitted provider URL, emission of the key-reference name only, proof that the extension never reads/copies/emits the parent secret value, controlled environment allowlist, safe launch diagnostics, and remote locality in `zed-extension/tests/direct_lsp.rs`

### Minimal implementation (GREEN, then REFACTOR)

- [X] T028 [US2] Implement `AzureTranslatorProvider` with internal technical-neutral tone/format validation, no invented external metadata, a small statically dispatched controlled-test transport seam, production `ureq` agent with global timeout/proxy disabled/redirect limit zero, borrowed request slices, bounded body reads, private non-`Debug` wire types, and strict response validation in `crates/translator-core/src/azure_translator.rs`
- [X] T029 [US2] Add Azure selection and preserve the gate order configuration -> fresh confirmation -> secret detection -> contact -> response validation, with no panic/retry and only existing `ErrorCode` values in `crates/translator-core/src/provider.rs`
- [X] T030 [US2] Register and document the Azure module/public provider boundary without exporting credential-bearing wire types in `crates/translator-core/src/lib.rs`
- [X] T031 [US2] Parse provider configuration once at LSP startup and derive both `ProviderSelection` and `ProviderDescriptor` from that value so execution/locality cannot drift in `crates/translator-lsp/src/lib.rs` and `crates/translator-lsp/src/state.rs`
- [X] T032 [US2] Emit only the three safe Azure selection entries including the key-reference name, reject nested/binary conflicts, and prove `settings.rs` neither reads nor copies the actual key value from the parent Zed environment into settings, arguments, or the generated launch profile in `zed-extension/src/settings.rs`
- [X] T033 [US2] Wire Azure through the existing CLI request-specific confirmation and redacted result boundary without adding arguments, raw provider diagnostics, or persistent content in `crates/translator-cli/src/main.rs`
- [X] T034 [US2] Run the focused Azure/core/CLI/LSP/extension RED-to-GREEN suites through `Makefile` and confirm a second remote request cannot reuse the first request's consent
- [X] T035 [US2] Finalize but do not execute the `REMOTE-CLI-01`/`REMOTE-ZED-01` procedure, parent-environment key-reference prerequisite, fresh-consent cases, 15-second budget check, safe case IDs, and redacted fields in `specs/007-operational-providers/manual-validation.md`; real Azure execution is deferred to T056 after T054 passes

**Checkpoint**: US2 is independently testable with controlled doubles. Real
Azure execution remains deferred to T056 and requires explicit account/privacy
review, a user-supplied external key reference, and per-request consent.

---

## Phase 5: User Story 3 - Operate and Recover the Local Provider (Priority: P3)

**Goal**: Complete update, rollback, persistence, idempotency, and narrowly
confirmed removal without changing the host or sacrificing the last known-good
slot.

**Independent Test**: Fake Docker/storage boundaries simulate a corrupt or
failed candidate, prove current is unchanged, perform rollback with all
external contact disabled, and prove cleanup touches only the fixed project
after the exact confirmation token.

### Tests first (RED)

- [X] T036 [P] [US3] Add failing tests for changed-lock review gate, fresh candidate, failed verification preserving current/previous, successful promotion rotation, offline rollback, rollback failure preservation, and recovered translation in `tests/integration/provider_local_rollback.sh`
- [X] T037 [P] [US3] Add failing tests for repeated update/rollback behavior, exact destructive confirmation, project-only resource removal, persistent ordinary stop/clean, and rejection of global prune/sudo/package/service commands in `tests/integration/provider_local_update_cleanup.sh`

### Minimal implementation (GREEN, then REFACTOR)

- [X] T038 [US3] Persist only safe validated candidate/current/previous identities, active slot, offline-verification status, and normalized lifecycle state under ignored `provider-cache/libretranslate/` via `scripts/providers/libretranslate.sh`
- [X] T039 [US3] Implement reviewed-lock change detection, candidate-only online update, integrity/readiness/offline gates, current-to-previous rotation, and failed-candidate quarantine without in-place mutation in `scripts/providers/libretranslate.sh`
- [X] T040 [US3] Implement download-free rollback to a previously verified slot with offline readiness+translation checks and preservation of the last known-good reference on failure in `scripts/providers/libretranslate.sh`
- [X] T041 [US3] Implement exact-token destructive cleanup scoped to the fixed Compose project, its three slots, two allowlisted networks, provider/relay containers, and ignored metadata only in `scripts/providers/libretranslate.sh`
- [X] T042 [US3] Make the recovery/clean T001 contract cases GREEN by adding `provider-local-update`, `provider-local-rollback`, and `provider-local-clean CONFIRM=remove-provider-data` targets while proving `clean` does not touch provider state in `Makefile`
- [X] T043 [US3] Run the controlled lifecycle suites and finalize, but do not execute, the failed-update/offline-rollback/idempotent-stop/explicit-clean manual procedure and safe fields in `specs/007-operational-providers/manual-validation.md`; real execution is deferred to T056 after T054 passes

**Checkpoint**: US3 can recover without Internet or host changes and no
ordinary command can delete prepared provider data.

---

## Phase 6: User Story 4 - Diagnose Real Provider Failures Safely (Priority: P4)

**Goal**: Distinguish normalized configuration, consent, secret, timeout,
availability, invalid-response, and lifecycle failures without exposing any
prohibited data or mutating sources.

**Independent Test**: Controlled failures across local/Azure, CLI/LSP/Zed, and
lifecycle commands produce only stable codes/safe actions; automated evidence
checks reject fixtures containing content, keys, raw URLs, paths, or bodies.

### Tests first (RED)

- [X] T044 [P] [US4] Add a failing cross-provider matrix for stable error codes, bounded messages, no raw cause/body, and no `Debug` leakage in `crates/translator-core/tests/operational_provider_redaction.rs`
- [X] T045 [P] [US4] Add failing CLI process tests for normalized exit/stderr, missing/rejected key, timeout, invalid response, no source/translation/key/path leakage, and unchanged fixture hashes in `crates/translator-cli/tests/cli_operational_providers.rs`
- [X] T046 [P] [US4] Add failing LSP tests for safe diagnostics, unavailable/timeout/invalid-response outcomes, stale preview invalidation, and byte-for-byte document non-mutation in `crates/translator-lsp/tests/operational_provider_locality.rs`
- [X] T047 [P] [US4] Add a failing evidence/privacy contract test that checks required case IDs/fields and rejects prohibited content patterns in `tests/integration/operational_provider_evidence_contract.sh`

### Minimal implementation (GREEN, then REFACTOR)

- [X] T048 [US4] Complete Azure error classification and generic redacted messages for timeout versus all other provider failures without automatic retry in `crates/translator-core/src/azure_translator.rs`
- [X] T049 [US4] Normalize lifecycle exit codes/status vocabulary, bound subprocess output, redact paths/URLs/content, and preserve actionable recovery categories in `scripts/providers/libretranslate.sh`
- [X] T050 [US4] Make only the minimal failing-test-driven boundary change: existing CLI failure output already passed unchanged, while failed retranslation invalidates stale LSP preview state in `crates/translator-lsp/src/protocol.rs`
- [X] T051 [US4] Run the complete controlled negative matrix and update only synthetic expected metadata in `tests/fixtures/operational-providers/expected-metadata.json`; do not copy automated outcomes into `specs/007-operational-providers/manual-validation.md`

**Checkpoint**: US4 exposes enough normalized information to recover while
all prohibited provider/content/credential/path detail remains absent.

---

## Phase 7: Polish & Cross-Cutting Completion Gates

**Purpose**: Assemble the reproducible automatic gate, run Rust/supply-chain
quality checks, complete redacted evidence review, and synchronize feature
status without widening scope.

- [X] T052 Make the final gate T001 contract case GREEN by adding `test-operational-providers` with the exact Rust and shell test inventory, include it in the supported validation interface, and keep automatic execution offline/credential-free in `Makefile`
- [X] T053 [P] Document implemented lifecycle commands, security/privacy boundaries, Azure account/F0 caveats, resource budgets, license/publication gate, worktree storage guard, complete removal, and MCP/Agent compatibility-only scope in `README.md` and `specs/007-operational-providers/quickstart.md`
- [X] T054 Run `make workspace-storage-check`, `make test-operational-providers`, `make test-real-provider-config`, `make test-direct-zed-translation`, `make test-zed-extension`, `make test`, `make fmt`, and `make clippy`, fixing only failures attributable to paths listed in `specs/007-operational-providers/tasks.md`
- [X] T055 Run `make deny` and review `ops/providers/libretranslate/provider.lock`, `Cargo.lock`, `zed-extension/Cargo.lock`, `compose.yaml`, and model-license metadata for mutable references, unreviewed sources, unresolved redistribution, or dependency drift
- [ ] T056 Only after T054 and T055 pass, obtain explicit approval and required external prerequisites, execute the real clean-checkout local and Azure CLI/direct-Zed matrix plus offline/rollback/negative cases, record 120-second readiness and 15-second invocation compliance, then review all rows for non-mutation/redaction and leave status incomplete if anything is missing or prohibited in `specs/007-operational-providers/manual-validation.md`; progress: `LOCAL-CLI-01` and `LOCAL-ZED-01` pass, while Azure and the remaining manual cases stay open
- [X] T057 Update implemented requirement traceability, checklist results, gate status, and remaining external publication/license blockers without claiming F011 complete prematurely in `specs/007-operational-providers/spec.md`, `specs/007-operational-providers/checklists/provider-operations-privacy.md`, and `specs/007-operational-providers/plan.md`

---

## Dependencies & Execution Order

### Phase dependencies

- **Phase 1 (Setup)**: No dependency.
- **Phase 2 (Foundation)**: Depends on Phase 1 and blocks every user story.
- **US1 (Phase 3)**: Starts after Phase 2; it is the MVP.
- **US2 (Phase 4)**: Starts after Phase 2 and is independent of US1's local
  runtime; it may proceed in parallel after shared configuration is green.
- **US3 (Phase 5)**: Depends on US1 because it extends the local lifecycle and
  its candidate/current/previous state.
- **US4 (Phase 6)**: Depends on US1 and US2 adapters; lifecycle-failure coverage
  also depends on US3.
- **Phase 7 (Polish)**: Depends on every selected story. T056 additionally
  depends on T054 and T055 plus explicit approval/external prerequisites.
  Missing real-service evidence keeps F011 open rather than being bypassed.

### Within each phase/story

- T004-T006 must fail for the intended missing behavior before T007-T009.
- T001 provides independently runnable RED cases that must fail for the
  intended reason before T019, T042, and T052 make their respective interface
  subsets GREEN.
- T011-T014 must fail before T015-T019; T021 requires their GREEN result and
  prepares documentation only and performs no real service execution.
- T022-T027 must fail before T028-T033; T035 requires GREEN automatic tests,
  but prepares documentation only and performs no real Azure execution.
- T036-T037 must fail before T038-T042.
- T044-T047 must fail before T048-T050.
- T056 is the only task authorized to execute real providers and cannot start
  until the full automatic and supply-chain gates T054-T055 pass.
- Production Rust uses typed `Result`/`?`; `unwrap`/`expect` remain test-only.
- Refactor only after the relevant RED tests pass and never weaken exact-target,
  consent, secret, size, timeout, response, non-mutation, or redaction checks.

### User story dependency graph

```text
Setup -> Foundation -> US1 (local MVP) -> US3 (lifecycle recovery) --+
                    \-> US2 (remote consent) -------------------------+-> US4 -> Completion gates
```

---

## Parallel Opportunities

- T002 and T003 can run in parallel after T001 starts because fixtures and
  shell helpers are independent files.
- T004, T005, and T006 are independent RED suites.
- T011-T014 are independent US1 RED suites.
- T022-T027 are independent US2 RED suites across core, CLI, LSP, and Zed.
- T036 and T037 are independent US3 RED scripts.
- T044-T047 are independent US4 RED suites.
- After Foundation, US1 and US2 can be implemented concurrently if changes to
  shared `provider_config.rs` are integrated first.

## Parallel Example: User Story 1

```text
Task T011: Write the static Compose/lock contract test.
Task T012: Write the controlled lifecycle state test.
Task T013: Write the no-egress runtime test.
Task T014: Write the controlled local CLI test.
```

## Parallel Example: User Story 2

```text
Task T022: Write Azure happy-path/payload tests.
Task T023: Write Azure transport/response failure tests.
Task T024: Write core privacy/redaction tests.
Task T025: Write CLI consent tests.
Task T026: Write LSP consent/locality tests.
Task T027: Write Zed launch/settings tests.
```

## Parallel Example: User Story 3

```text
Task T036: Write update/rollback state-transition tests.
Task T037: Write update/cleanup isolation tests.
```

## Parallel Example: User Story 4

```text
Task T044: Write core normalized-error/redaction tests.
Task T045: Write CLI process privacy tests.
Task T046: Write LSP diagnostics/non-mutation tests.
Task T047: Write evidence privacy contract tests.
```

---

## Implementation Strategy

### MVP first: User Story 1

1. Complete Setup and Foundation.
2. Complete US1 strictly RED -> GREEN -> REFACTOR.
3. Run its controlled local/offline/non-mutation checks.
4. Finalize the unexecuted local acceptance procedure and stop to review the
   controlled MVP before adding Azure or lifecycle update complexity.
5. Defer real provider preparation and evidence to T056 after all automatic
   and supply-chain gates pass.

### Incremental delivery

1. **Foundation**: exact provider configuration and safe locality.
2. **US1**: real project-scoped offline translation.
3. **US2**: exact Azure path with per-request consent.
4. **US3**: update/rollback/removal safety.
5. **US4**: complete failure diagnosis and redaction.
6. **Completion**: full automated gates plus real-service evidence review.

### Security stop conditions

- Stop before any global install, `sudo`, service/system configuration, Docker
  global prune, unrestricted endpoint, raw credential storage, or real `.env`.
- Stop before real LibreTranslate/Azure execution unless T054-T055 pass, the
  user explicitly authorizes the external operation, and prerequisites are
  supplied safely; only T056 performs that execution.
- Stop and keep F011 incomplete if model redistribution rights, F0 eligibility,
  endpoint/privacy terms, automatic gates, or required evidence no longer meet
  the reviewed contracts.

---

## Notes

- Automated tests never use real providers, credentials, or external network.
- Use descriptive tests with one behavior/assertion focus and safe assertion
  diagnostics; snapshots must be small and explicitly redacted if introduced.
- Keep request/response wire structs private and non-`Debug`; error messages
  report normalized categories, not raw causes or bodies.
- Prefer `&str`/slices/iterators at provider boundaries and avoid clones or
  intermediate collections unless ownership is required by the HTTP API.
- Do not add typestate/generics merely for novelty; use them only if they make
  an invalid provider state unrepresentable without obscuring runtime recovery.
- No task authorizes buffer/file/clipboard mutation, arbitrary providers,
  additional languages, publication, model redistribution, or Agent Panel.
- Existing MCP tests are compatibility regressions only; no task adds an
  MCP/Agent product flow or real-service acceptance row.

## Phase 8: Convergence

- [X] T058 Add a controlled readiness-probe contract and implement the exact LibreTranslate object payload plus bounded response-shape/non-empty translation validation before declaring health, per FR-004, FR-005, and US1/AC1 (partial)
- [X] T059 Add late-promotion and repeated-cycle tests, then implement fresh candidate provisioning and metadata-atomic physical-slot role rotation so failed update/promotion preserves both current and previous last-known-good resources, per FR-005, FR-021, and plan: local lifecycle (partial)
- [X] T060 Add image-selection tests and make start/verify/rollback use the immutable image reference recorded for the selected logical slot, rejecting an unavailable prior image without pull or state loss, per FR-021 and US3/AC3 (partial)
- [X] T061 Add missing/failing Docker cleanup tests and make stop/removal report normalized failure unless fixed project resources were handled successfully, preserving operational metadata on incomplete cleanup, per FR-006 and US3/AC2-US3/AC4 (partial)
- [X] T062 Reject cache-root/disk-check overrides outside namespaced temporary test roots and validate bounded allowlisted state fields before status output or destructive cleanup, per FR-006 and FR-017 (partial)
