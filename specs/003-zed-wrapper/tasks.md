# Tasks: Zed Wrapper

**Input**: Design documents from `specs/003-zed-wrapper/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md),
[research.md](./research.md), [data-model.md](./data-model.md),
[contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: Required. The constitution requires tests/checks before behavior
changes, and the feature spec requires validation for startup, missing
prerequisites, repeated preparation, environment minimization, redaction,
offline default behavior, and no mutation.

**Organization**: Tasks are grouped by user story so each story can be
implemented and validated independently.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add the Zed extension project shell and project commands needed for
test-first implementation.

- [X] T001 Add `wasm32-wasip1` target installation to `docker/rust-toolchain.Dockerfile`
- [X] T002 Add `zed-extension/Cargo.toml` as an isolated Rust/WASM `cdylib` crate using `zed_extension_api = "0.7.0"`
- [X] T003 [P] Create placeholder Zed extension source modules in `zed-extension/src/lib.rs`, `zed-extension/src/settings.rs`, `zed-extension/src/launch.rs`, and `zed-extension/src/diagnostics.rs`
- [X] T004 [P] Create test file skeletons in `zed-extension/tests/extension_manifest.rs`, `zed-extension/tests/launch_profile.rs`, and `zed-extension/tests/diagnostics_redaction.rs`
- [X] T005 Add `make zed-extension-build`, `make zed-extension-prepare`, and `make test-zed-extension` targets to `Makefile`
- [X] T006 Add local artifact preparation script skeleton in `scripts/zed-extension/prepare.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared contracts and helpers that every user story needs.

**CRITICAL**: No user story work can begin until this phase is complete.

- [X] T007 [P] Write failing manifest contract tests for required metadata and one context server in `zed-extension/tests/extension_manifest.rs`
- [X] T008 [P] Write failing settings parsing tests for allowed `binary_path` and rejected provider/env/arg settings in `zed-extension/tests/launch_profile.rs`
- [X] T009 [P] Write failing diagnostic redaction tests for source text, translated text, tokens, env dumps, and full paths in `zed-extension/tests/diagnostics_redaction.rs`
- [X] T010 Implement manifest fixture/parser helpers for `extension.toml` validation in `zed-extension/tests/extension_manifest.rs`
- [X] T011 Implement `LaunchSettings`, forbidden setting names, and validation result types in `zed-extension/src/settings.rs`
- [X] T012 Implement `DiagnosticCode` and redacted diagnostic formatting in `zed-extension/src/diagnostics.rs`
- [X] T013 Implement shared launch profile types and constants for `translator-en-es`, `translator-mcp`, empty args, and allowlisted env in `zed-extension/src/launch.rs`
- [X] T014 Wire shared modules through `zed-extension/src/lib.rs` without exposing translation behavior in the wrapper

**Checkpoint**: Foundation ready. The extension crate can compile under the
project Docker workflow, and user-story tasks can proceed.

---

## Phase 3: User Story 1 - Start Translation From Zed (Priority: P1) MVP

**Goal**: A local Zed development extension can return a controlled command that
starts the existing `translator-mcp` stdio server without a separately started
server shell.

**Independent Test**: Configure a prepared local `translator-mcp` artifact path,
request the `translator-en-es` context server command, and verify the returned
launch profile uses the artifact as a direct command with controlled args/env
while preserving existing MCP behavior.

### Tests for User Story 1

- [X] T015 [P] [US1] Write failing extension command test for valid `translator-en-es` launch profile in `zed-extension/tests/launch_profile.rs`
- [X] T016 [P] [US1] Write failing unsupported context server id test in `zed-extension/tests/launch_profile.rs`
- [X] T017 [P] [US1] Write failing path-with-spaces command preservation test in `zed-extension/tests/launch_profile.rs`
- [X] T018 [P] [US1] Write failing extension manifest declaration test for `[context_servers.translator-en-es]` in `zed-extension/tests/extension_manifest.rs`
- [X] T019 [P] [US1] Write failing MCP server artifact smoke check in `tests/integration/zed_extension_prepare_artifact.sh` that proves the prepared artifact is `translator-mcp`

### Implementation for User Story 1

- [X] T020 [US1] Implement `zed::Extension` and `zed::register_extension!` in `zed-extension/src/lib.rs`
- [X] T021 [US1] Implement `context_server_command` dispatch for `translator-en-es` in `zed-extension/src/lib.rs`
- [X] T022 [US1] Implement valid launch profile construction with direct command, empty args, and allowlisted env in `zed-extension/src/launch.rs`
- [X] T023 [US1] Add valid Zed metadata and `[context_servers.translator-en-es]` to `zed-extension/extension.toml`
- [X] T024 [US1] Implement release artifact build and printed binary path in `scripts/zed-extension/prepare.sh`
- [X] T025 [US1] Update `Makefile` targets so `make zed-extension-build` checks the extension crate, `make zed-extension-prepare` prepares `target/release/translator-mcp`, and `tests/integration/zed_extension_prepare_artifact.sh` is runnable through `make test-zed-extension`
- [X] T026 [US1] Document US1 validation status in `specs/003-zed-wrapper/quickstart.md`

**Checkpoint**: User Story 1 is independently usable as the MVP wrapper startup
slice.

---

## Phase 4: User Story 2 - Reproduce The Development Package (Priority: P2)

**Goal**: A maintainer can rerun the documented preparation workflow and get a
stable local extension setup without duplicate or conflicting generated state.

**Independent Test**: Remove generated local artifacts, run the preparation
workflow twice, and verify extension metadata, launch configuration, and the
prepared server artifact remain stable.

### Tests for User Story 2

- [X] T027 [P] [US2] Write failing repeatable preparation test script in `tests/integration/zed_extension_prepare_idempotent.sh`
- [X] T028 [P] [US2] Write failing generated-state absence checks for duplicate manifests, provider settings, and secret files in `tests/integration/zed_extension_prepare_idempotent.sh`
- [X] T029 [P] [US2] Write failing Make target contract check for `zed-extension-prepare` and `test-zed-extension` in `tests/integration/zed_extension_make_targets.sh`
- [X] T030 [P] [US2] Write failing lockfile/dependency scope check for `zed-extension/Cargo.lock` and `zed_extension_api` in `tests/integration/zed_extension_dependency_scope.sh`

### Implementation for User Story 2

- [X] T031 [US2] Make `scripts/zed-extension/prepare.sh` idempotent and safe to rerun without duplicate generated state
- [X] T032 [US2] Ensure `scripts/zed-extension/prepare.sh` builds the existing `translator-mcp` artifact through project-scoped Docker workflow only
- [X] T033 [US2] Add `tests/integration/zed_extension_prepare_idempotent.sh` to `make test-zed-extension` in `Makefile`
- [X] T034 [US2] Add `tests/integration/zed_extension_make_targets.sh` and `tests/integration/zed_extension_dependency_scope.sh` to `make test-zed-extension` in `Makefile`
- [X] T035 [US2] Commit or update the project-scoped extension lockfile in `zed-extension/Cargo.lock`
- [X] T036 [US2] Document repeated preparation validation status in `specs/003-zed-wrapper/quickstart.md`

**Checkpoint**: User Stories 1 and 2 both work independently and the local setup
is reproducible.

---

## Phase 5: User Story 3 - Diagnose Startup Safely (Priority: P3)

**Goal**: Startup and configuration failures produce actionable redacted
diagnostics without exposing source text, translated text, secrets, tokens,
environment dumps, or sensitive paths.

**Independent Test**: Break binary path, artifact usability, context server id,
environment settings, and provider settings, then verify stable redacted failure
categories and no sensitive content in diagnostics.

### Tests for User Story 3

- [X] T037 [P] [US3] Write failing missing `binary_path` diagnostic test in `zed-extension/tests/diagnostics_redaction.rs`
- [X] T038 [P] [US3] Write failing missing/non-executable/stale artifact diagnostic tests in `zed-extension/tests/diagnostics_redaction.rs`
- [X] T039 [P] [US3] Write failing fake secret and environment dump redaction tests in `zed-extension/tests/diagnostics_redaction.rs`
- [X] T040 [P] [US3] Write failing provider/remote setting rejection tests in `zed-extension/tests/launch_profile.rs`
- [X] T040A [P] [US3] Write failing offline-default denial integration check in `tests/integration/zed_extension_remote_denial.sh` covering rejected remote/provider requests through the extension-launched MCP flow with no network activity
- [X] T041 [P] [US3] Write failing repeated startup after failure test in `zed-extension/tests/launch_profile.rs`
- [X] T042 [P] [US3] Write failing no-buffer-or-file-mutation validation script in `tests/integration/zed_extension_no_mutation.sh`

### Implementation for User Story 3

- [X] T043 [US3] Implement missing and unusable artifact diagnostics in `zed-extension/src/diagnostics.rs`
- [X] T044 [US3] Implement artifact status validation for missing, not executable, stale, incompatible checkout, and failed-on-start states in `zed-extension/src/launch.rs`
- [X] T045 [US3] Implement provider, remote, extra arg, and arbitrary env rejection in `zed-extension/src/settings.rs`
- [X] T046 [US3] Ensure repeated startup failures revalidate state without duplicate output in `zed-extension/src/launch.rs`
- [X] T047 [US3] Add no-mutation validation coverage to `tests/integration/zed_extension_no_mutation.sh`
- [X] T048 [US3] Add `tests/integration/zed_extension_no_mutation.sh` and `tests/integration/zed_extension_remote_denial.sh` to `make test-zed-extension` in `Makefile`
- [X] T049 [US3] Document diagnostic, offline-default denial, and no-mutation validation status in `specs/003-zed-wrapper/quickstart.md`

**Checkpoint**: All user stories are independently functional and startup
failure behavior is privacy-safe.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, documentation, scope confirmation, and remaining
manual Zed validation.

- [X] T050 [P] Update `README.md` with Zed wrapper feature status and validation commands
- [X] T051 [P] Update `docs/PLAN.md` to mark `specs/003-zed-wrapper/` implementation status after validation
- [X] T052 Run `make test-zed-extension` and record output summary in `specs/003-zed-wrapper/quickstart.md`
- [X] T053 Run `make test`, `make fmt`, and `make clippy`, then record output summary in `specs/003-zed-wrapper/quickstart.md`
- [X] T054 Record approved host-prerequisite resolution for Zed dev-extension validation in `specs/003-zed-wrapper/quickstart.md`
- [X] T055 Confirm no real provider, network transport, publication flow, source-file mutation, or buffer edit path was added in `specs/003-zed-wrapper/quickstart.md`
- [X] T056 Revalidate manual Zed smoke through `zed: install dev extension`, the extension configuration modal, `translator-en-es`, and `translate_text`, then record the result in `specs/003-zed-wrapper/quickstart.md`
- [X] T057 Rescope manual Zed failure timing for missing/unusable binary diagnostics after WASM filesystem/preflight probes caused modal timeouts, record the accepted limitation, and update `specs/003-zed-wrapper/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational completion.
- **Polish (Phase 6)**: Depends on all selected user stories.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. This is the MVP.
- **User Story 2 (P2)**: Can start after Foundational, but its validation is most useful after US1 establishes the launch path.
- **User Story 3 (P3)**: Can start after Foundational and may proceed in parallel with US1/US2 because most diagnostics code is isolated.

### Within Each User Story

- Tests/checks must be written and observed failing before implementation.
- Shared models/settings before launch services.
- Launch services before extension integration.
- Integration scripts before Makefile wiring.
- Quickstart validation notes after runnable checks exist.

## Parallel Opportunities

- T003 and T004 can run in parallel after T002.
- T007, T008, and T009 can run in parallel.
- In US1, T015, T016, T017, T018, and T019 can run in parallel.
- In US2, T027, T028, T029, and T030 can run in parallel.
- In US3, T037, T038, T039, T040, T040A, T041, and T042 can run in parallel.
- T050 and T051 can run in parallel during polish.

## Parallel Example: User Story 1

```bash
Task: "Write failing extension command test for valid `translator-en-es` launch profile in zed-extension/tests/launch_profile.rs"
Task: "Write failing unsupported context server id test in zed-extension/tests/launch_profile.rs"
Task: "Write failing path-with-spaces command preservation test in zed-extension/tests/launch_profile.rs"
Task: "Write failing extension manifest declaration test for `[context_servers.translator-en-es]` in zed-extension/tests/extension_manifest.rs"
Task: "Write failing MCP server artifact smoke check in tests/integration/zed_extension_prepare_artifact.sh that proves the prepared artifact is `translator-mcp`"
```

## Parallel Example: User Story 2

```bash
Task: "Write failing repeatable preparation test script in tests/integration/zed_extension_prepare_idempotent.sh"
Task: "Write failing generated-state absence checks for duplicate manifests, provider settings, and secret files in tests/integration/zed_extension_prepare_idempotent.sh"
Task: "Write failing Make target contract check for `zed-extension-prepare` and `test-zed-extension` in tests/integration/zed_extension_make_targets.sh"
Task: "Write failing lockfile/dependency scope check for `zed-extension/Cargo.lock` and `zed_extension_api` in tests/integration/zed_extension_dependency_scope.sh"
```

## Parallel Example: User Story 3

```bash
Task: "Write failing missing `binary_path` diagnostic test in zed-extension/tests/diagnostics_redaction.rs"
Task: "Write failing missing/non-executable/stale artifact diagnostic tests in zed-extension/tests/diagnostics_redaction.rs"
Task: "Write failing fake secret and environment dump redaction tests in zed-extension/tests/diagnostics_redaction.rs"
Task: "Write failing provider/remote setting rejection tests in zed-extension/tests/launch_profile.rs"
Task: "Write failing repeated startup after failure test in zed-extension/tests/launch_profile.rs"
Task: "Write failing no-buffer-or-file-mutation validation script in tests/integration/zed_extension_no_mutation.sh"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. Stop and validate `make zed-extension-prepare` plus the US1 checks.
5. Demo local Zed startup only if host prerequisites already exist.

### Incremental Delivery

1. Setup + Foundational: extension crate, shared settings, diagnostics, launch types.
2. US1: Zed can request a controlled launch command for `translator-mcp`.
3. US2: preparation is repeatable and project-scoped.
4. US3: startup failures and privacy-sensitive paths are redacted and recoverable.
5. Polish: docs, full automated validation, manual Zed smoke, and manual
   failure-timing evidence.

### Parallel Team Strategy

After Phase 2:

- Developer A: US1 launch command and manifest.
- Developer B: US2 preparation workflow.
- Developer C: US3 diagnostics and negative privacy checks.

## Notes

- `[P]` tasks touch different files or independent checks.
- `[US1]`, `[US2]`, and `[US3]` map to the user stories in `spec.md`.
- Every behavior-changing task has a preceding failing test/check task.
- Offline-default denial of remote/provider behavior is covered by both settings rejection and an explicit integration check for the extension-launched MCP flow.
- Manual Zed validation must use only already available or explicitly approved
  host prerequisites; record a blocker if prerequisites are absent.
- First failing test/check outputs for this feature were not preserved in the
  task artifact; future features should record the initial failing run when
  test-first traceability is required.
- Stop at checkpoints to validate each story independently.

---

## Phase 7: Convergence

- [ ] T058 Close or formally re-scope the SC-004 missing-artifact 15-second failure-visibility target: implement a Zed-WASM-compatible fast-fail mechanism, or run `/speckit-clarify`/amend `spec.md` SC-004 and `contracts/launch-profile.md` to reflect the observed ~60s Zed context-server timeout documented in `specs/003-zed-wrapper/quickstart.md` (Manual Failure Timing) per SC-004 (partial)
