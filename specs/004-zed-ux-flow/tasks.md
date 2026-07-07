# Tasks: Zed UX Flow

**Input**: Design documents from `/specs/004-zed-ux-flow/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: Required by the project constitution before behavior or documentation contract changes. This feature is mostly UX documentation and validation evidence, so checks are shell/doc-contract checks plus manual Zed smoke validation.

**Organization**: Tasks are grouped by user story so each story can be implemented and reviewed independently.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add the project-scoped validation entry points that future tasks will use.

- [X] T001 Add failing Make target dry-run check for `test-zed-ux-flow` in `tests/integration/zed_ux_flow_make_targets.sh`
- [X] T002 [P] Add failing user-guide documentation contract check in `tests/integration/zed_ux_flow_docs_contract.sh`
- [X] T003 [P] Add failing manual-evidence template contract check in `tests/integration/zed_ux_flow_evidence_contract.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared docs and validation harness required before user-story work.

**Critical**: No user-story task should start until these tasks are complete.

- [X] T004 Add `ZED_UX_FLOW_TESTS` and `test-zed-ux-flow` target wiring in `Makefile`
- [X] T005 Create the reviewer-facing Zed UX guide skeleton in `docs/zed-ux-flow.md`
- [X] T006 Create the manual validation evidence template skeleton in `specs/004-zed-ux-flow/manual-validation-template.md`
- [X] T007 Make the new Zed UX validation scripts executable in `tests/integration/zed_ux_flow_make_targets.sh`, `tests/integration/zed_ux_flow_docs_contract.sh`, and `tests/integration/zed_ux_flow_evidence_contract.sh`

**Checkpoint**: The new validation target exists and can fail against missing story-specific content.

---

## Phase 3: User Story 1 - Complete A Translation Inside Zed (Priority: P1) - MVP

**Goal**: A reviewer can follow a canonical Agent Panel flow and obtain a readable direct-text Spanish result without manually starting the server during the request.

**Independent Test**: Run `make test-zed-ux-flow`, then follow the direct-text section in `docs/zed-ux-flow.md` using the prepared local extension and record the result in `specs/004-zed-ux-flow/manual-validation-template.md`.

### Tests for User Story 1

- [X] T008 [P] [US1] Extend direct-text requirements in `tests/integration/zed_ux_flow_docs_contract.sh` for artifact preparation, dev-extension registration, `binary_path`, Agent Panel invocation, expected `translate_text` input, readable result, and no manual server startup
- [X] T009 [P] [US1] Extend direct-text evidence requirements in `tests/integration/zed_ux_flow_evidence_contract.sh` for Agent model route, tool-permission posture, result status, and no-mutation notes

### Implementation for User Story 1

- [X] T010 [US1] Document the canonical Agent Panel direct-text workflow in `docs/zed-ux-flow.md`
- [X] T011 [US1] Add direct-text scenario fields and pass/fail prompts in `specs/004-zed-ux-flow/manual-validation-template.md`
- [X] T012 [US1] Update `specs/004-zed-ux-flow/quickstart.md` to point reviewers to the direct-text section in `docs/zed-ux-flow.md`

**Checkpoint**: User Story 1 is independently reviewable with the local extension and synthetic direct text.

---

## Phase 4: User Story 2 - Know What Content Is Sent (Priority: P2)

**Goal**: A reviewer can tell which content is sent for direct text, workspace files, and selection attempts, and can distinguish the local MCP translator from the Agent model route.

**Independent Test**: Run `make test-zed-ux-flow`, then complete the workspace-file and selection-decision sections in `docs/zed-ux-flow.md` using synthetic canary content and redacted evidence only.

### Tests for User Story 2

- [X] T013 [P] [US2] Create executable supported-input-path checks for direct text, workspace file, and gated selection in `tests/integration/zed_ux_flow_privacy_contract.sh`
- [X] T014 [US2] Add Agent privacy-boundary checks for model route enum and synthetic-only non-local validation in `tests/integration/zed_ux_flow_privacy_contract.sh`
- [X] T015 [US2] Add translation-only tool-permission checks for edit/write/delete/move/copy, terminal, fetch, and search posture in `tests/integration/zed_ux_flow_privacy_contract.sh`
- [X] T016 [P] [US2] Extend evidence-template checks for canary identifiers, length/hash metadata, and redacted summaries in `tests/integration/zed_ux_flow_evidence_contract.sh`

### Implementation for User Story 2

- [X] T017 [US2] Document the authorized workspace-file flow and no-mutation proof in `docs/zed-ux-flow.md`
- [X] T018 [US2] Document Agent model-route privacy boundaries and translation-only permissions in `docs/zed-ux-flow.md`
- [X] T019 [US2] Document selection-support states and decision criteria in `docs/zed-ux-flow.md`
- [X] T020 [US2] Add workspace-file, Agent privacy boundary, tool-permission, and selection-decision sections in `specs/004-zed-ux-flow/manual-validation-template.md`
- [X] T021 [US2] Update `specs/004-zed-ux-flow/quickstart.md` to point reviewers to the workspace-file and selection sections in `docs/zed-ux-flow.md`

**Checkpoint**: User Story 2 is independently reviewable with clear content boundaries and safe evidence requirements.

---

## Phase 5: User Story 3 - Recover Safely From UX Failures (Priority: P3)

**Goal**: A reviewer can trigger setup, unsafe-input, provider/remote-denial, and redaction scenarios without leaking protected content or mutating user files.

**Independent Test**: Run `make test-zed-ux-flow`, then complete the failure and redaction sections in `docs/zed-ux-flow.md` and record redacted outcomes in `specs/004-zed-ux-flow/manual-validation-template.md`.

### Tests for User Story 3

- [X] T022 [P] [US3] Create executable setup-failure documentation checks for missing `binary_path`, missing artifact, stale artifact, and corrective action in `tests/integration/zed_ux_flow_failure_contract.sh`
- [X] T023 [US3] Add unsafe-input and remote/provider-denial documentation checks in `tests/integration/zed_ux_flow_failure_contract.sh`
- [X] T024 [P] [US3] Create executable redaction evidence checks for no source text, translated text, secrets, tokens, headers, environment dumps, workspace roots, or sensitive paths in `tests/integration/zed_ux_flow_redaction_contract.sh`

### Implementation for User Story 3

- [X] T025 [US3] Document setup-failure recovery paths in `docs/zed-ux-flow.md`
- [X] T026 [US3] Document unsafe-input and remote/provider-denial flows in `docs/zed-ux-flow.md`
- [X] T027 [US3] Document redaction inspection rules and failure categories in `docs/zed-ux-flow.md`
- [X] T028 [US3] Add setup-failure, unsafe-input denial, remote/provider denial, and redaction sections in `specs/004-zed-ux-flow/manual-validation-template.md`
- [X] T029 [US3] Update `specs/004-zed-ux-flow/quickstart.md` to point reviewers to the failure and redaction sections in `docs/zed-ux-flow.md`

**Checkpoint**: User Story 3 is independently reviewable with actionable, redacted failure evidence.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency, validation, and project-level status updates.

- [X] T030 [P] Update fourth-feature status and Zed UX guide link in `README.md`
- [X] T031 [P] Update strategic roadmap references for the active UX feature in `docs/PLAN.md`
- [X] T032 [P] Update strategic backlog status for F007 in `docs/feature-map.md`
- [X] T033 Run `make test-zed-ux-flow` and record the result in `specs/004-zed-ux-flow/quickstart.md`
- [X] T034 Run `make test-zed-extension` and record the result in `specs/004-zed-ux-flow/quickstart.md`
- [X] T035 Run `make test`, `make fmt`, and `make clippy`, then record the results in `specs/004-zed-ux-flow/quickstart.md`
- [X] T036 Perform the manual Zed UX smoke validation and record synthetic/redacted outcomes in `specs/004-zed-ux-flow/quickstart.md`
- [X] T037 Confirm `specs/004-zed-ux-flow/checklists/pre-tasks.md` remains complete after implementation updates

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks user-story work.
- **User Story 1 (Phase 3)**: Depends on Foundational completion.
- **User Story 2 (Phase 4)**: Depends on Foundational completion; may run after or alongside US1, but final guide edits must reconcile `docs/zed-ux-flow.md`.
- **User Story 3 (Phase 5)**: Depends on Foundational completion; may run after or alongside US1/US2, but final guide edits must reconcile `docs/zed-ux-flow.md`.
- **Polish (Phase 6)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **US1 (P1)**: MVP. No dependency on US2 or US3 after Foundational.
- **US2 (P2)**: Independent validation of input boundaries; no behavior dependency on US1, but shares `docs/zed-ux-flow.md`.
- **US3 (P3)**: Independent validation of failure handling; no behavior dependency on US1/US2, but shares `docs/zed-ux-flow.md`.

### Within Each User Story

- Test/check tasks must be written first and fail before implementation tasks.
- Documentation contract scripts precede guide/template updates.
- Manual Zed validation happens only after the guide, template, and automated contract checks are complete.

## Parallel Opportunities

- T002 and T003 can run in parallel after T001.
- T008 and T009 can run in parallel for US1.
- T013, T016 can run in parallel for US2. T014 and T015 share `tests/integration/zed_ux_flow_privacy_contract.sh` and should be serialized with T013.
- T022 and T024 can run in parallel for US3. T023 shares `tests/integration/zed_ux_flow_failure_contract.sh` and should be serialized with T022.
- T030, T031, and T032 can run in parallel after story documentation is stable.

## Parallel Example: User Story 1

```text
Task: "Extend direct-text requirements in tests/integration/zed_ux_flow_docs_contract.sh for artifact preparation, dev-extension registration, binary_path, Agent Panel invocation, expected translate_text input, readable result, and no manual server startup"
Task: "Extend direct-text evidence requirements in tests/integration/zed_ux_flow_evidence_contract.sh for Agent model route, tool-permission posture, result status, and no-mutation notes"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 setup checks.
2. Complete Phase 2 foundational docs and Make target.
3. Complete Phase 3 direct-text UX flow.
4. Stop and validate US1 with `make test-zed-ux-flow` plus the direct-text manual Zed smoke.

### Incremental Delivery

1. Add US1 for direct-text translation.
2. Add US2 for workspace-file, selection, privacy, and evidence boundaries.
3. Add US3 for failure recovery and redaction validation.
4. Finish with project-level status and full validation commands.

### Validation Boundary

No task in this feature should add a real provider, remote translation, automatic editor replacement, marketplace publication, or global host runtime installation.

## Phase 7: Convergence

- [X] T038 Record the actual Agent model route or explicit unknown/blocked route with synthetic-only rationale per FR-013 / SC-009
- [X] T039 Record the translation-only Agent tool-permission posture for mutation, terminal, fetch, and search tools before accepting no-mutation evidence per FR-014 / SC-010 (blocked profile-level audit recorded; no-mutation evidence limited to observed translator operations)
- [X] T040 Add missing manual validation run metadata, including Zed version, operating system, git branch/revision, Agent profile, and `translator-en-es` tool availability per contracts/manual-validation.md
- [X] T041 Record the setup-failure visible category, corrective action, redaction result, and no-mutation evidence per US3/AC1
