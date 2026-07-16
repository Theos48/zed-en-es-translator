# Tasks: Repository Convergence and Cleanup

**Input**: Design documents from `/specs/010-repository-convergence/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`,
`contracts/`, `quickstart.md`, constitution 2.0.0

**Tests**: Every removal wave starts with an explicit failing/negative boundary
and retains equivalent live safety coverage before obsolete tests are removed.

**Organization**: Tasks are grouped by user story and use disjoint ownership
for Rust/source, automation/operations and documentation/traceability.

## Phase 1: Setup and Governance

**Purpose**: Resolve the constitutional blocker and capture a reviewable baseline.

- [X] T001 Record workspace, worktree, tracked-file, line, Make-target, integration-script, dependency and generated-residue baselines in `specs/010-repository-convergence/validation.md`
- [X] T002 Amend `.specify/memory/constitution.md` to 2.0.0 and synchronize `.specify/templates/plan-template.md`, `.specify/templates/spec-template.md`, `.specify/templates/tasks-template.md`, `docs/decisions.md` and `docs/adr/0007-repository-convergence.md` per FR-011
- [X] T003 [P] Reconcile feature 009 compatibility and package-identity requirements with feature 010 under documentation-subagent ownership in `specs/009-zed-marketplace-install/spec.md`, `specs/009-zed-marketplace-install/plan.md` and `specs/009-zed-marketplace-install/tasks.md` per FR-016

---

## Phase 2: Foundational Negative Contracts

**Purpose**: Make the desired repository boundary executable before deletion.

**CRITICAL**: No removal task starts until T004 has been observed failing on the pre-cleanup tree.

- [X] T004 Add `tests/integration/repository_boundary.sh`, wire `make test-repository-boundary`, and record its expected pre-cleanup failure in `specs/010-repository-convergence/validation.md` per FR-001, FR-003 through FR-006, FR-012 and FR-TEST-A
- [X] T005 [P] Map live safety, privacy, package and removal invariants from retired tests to retained gates in `specs/010-repository-convergence/validation.md` per FR-002 and FR-SEC-A through FR-SEC-D
- [X] T006 [P] Add focused failing tests for direct adjacent embedded-provider construction and rejection/absence of runtime provider configuration in `crates/translator-lsp/tests/direct_translation.rs` per FR-005 and FR-012

---

## Phase 3: User Story 1 - Maintain One Product Architecture (Priority: P1)

**Goal**: Leave one runtime chain from Zed to the embedded local translator.

**Independent Test**: Root workspace has two members; retired entry points and
provider settings are absent; retained core/LSP tests pass.

### Tests for User Story 1

- [X] T007 [P] [US1] Preserve focused core tests for limits, Markdown reconstruction, workspace-safe files, UTF-8/binary rejection, process bounds and redaction in `crates/translator-core/tests/` per FR-002, FR-SEC-A, FR-SEC-B and FR-SEC-D
- [X] T008 [P] [US1] Preserve focused LSP tests for read-only previews, stale versions, adjacent package resolution and offline execution in `crates/translator-lsp/tests/` per FR-002, FR-005 and FR-SEC-C

### Implementation for User Story 1

- [X] T009 [P] [US1] Delete `crates/translator-mcp/` and `crates/translator-cli/` after their live invariants are mapped per FR-003, FR-004 and SC-001
- [X] T010 [P] [US1] Remove Azure, LibreTranslate, privacy/configuration-only modules and remote/configurable provider branches from `crates/translator-core/src/` and its retired tests per FR-004
- [X] T011 [US1] Simplify `crates/translator-lsp/src/` to construct only `EmbeddedProcessProvider::from_current_executable()` and remove provider URL/key/remote-confirmation/arbitrary-binary inputs per FR-005
- [X] T012 [US1] Reduce root `Cargo.toml` to `translator-core` and `translator-lsp`, prune direct retired dependencies, and regenerate `Cargo.lock` through the pinned Make/Docker flow per FR-006 and SC-002
- [X] T013 [US1] Align `zed-extension/src/`, `zed-extension/extension.toml` and extension tests with the single LSP package path and no MCP/context-server surface per FR-002 and FR-003
- [X] T014 [US1] Run focused Rust and extension gates and make `tests/integration/repository_boundary.sh` pass its source/runtime assertions per SC-001 and SC-004

---

## Phase 4: User Story 2 - Work With Focused Automation (Priority: P2)

**Goal**: Keep only commands, scripts, fixtures and CI gates consumed by the Gallery product.

**Independent Test**: `make help` advertises only retained targets and the full
project gate has no reference to a retired crate/provider/workflow.

### Tests for User Story 2

- [X] T015 [P] [US2] Extend `tests/integration/repository_boundary.sh` to reject orphaned Make targets, scripts, fixtures, workflow entries and dependency names per FR-006 and FR-013
- [X] T016 [P] [US2] Preserve marketplace acquisition, supply-chain, package, offline, privacy, resource and removal coverage in `tests/integration/marketplace_*.sh` per FR-002 and SC-006

### Implementation for User Story 2

- [X] T017 [P] [US2] Remove `ops/providers/`, `scripts/providers/`, retired provider fixtures/helpers and operational-provider integration tests per FR-004 and FR-006
- [X] T018 [P] [US2] Remove `scripts/zed-extension/` wrapper flows and retired `zed_direct_*`, `zed_extension_*` and `zed_ux_flow_*` integration scripts after migrating live coverage per FR-003 and FR-006
- [X] T019 [US2] Reduce `Makefile` to retained Rust, extension, marketplace, repository-audit, worktree and cleanup targets; rename the retained LSP release target and update all consumers per SC-002
- [X] T020 [P] [US2] Align `.github/workflows/`, `.github/dependabot.yml` and `.github/pull_request_template.md` with the two-crate workspace and retained marketplace gates per FR-006
- [X] T021 [US2] Implement previewable normal and explicit deep cleanup tiers in `Makefile` and `scripts/cleanup/` with fixed preserve/prohibit allowlists per FR-014 and FR-015
- [X] T022 [US2] Verify target/script/fixture counts, `make help` and repository-boundary automation in `specs/010-repository-convergence/validation.md` per SC-004 and SC-006

---

## Phase 5: User Story 3 - Read Current Documentation Without Losing History (Priority: P3)

**Goal**: Make Gallery status and release sequence current while Git preserves full removed history.

**Independent Test**: All retained local links resolve; current guidance has no
retired instructions; every retained ADR has an explicit coherent status.

### Implementation for User Story 3

- [X] T023 [P] [US3] Rewrite current product/status guidance in `README.md`, `docs/PLAN.md`, `docs/feature-map.md` and `docs/diagrams.md` around the Gallery/LSP/embedded-runtime objective per FR-009
- [X] T024 [P] [US3] Mark `docs/adr/0001-zed-extension-scope.md` through `docs/adr/0005-operational-provider-pair.md` superseded or partially superseded by ADR 0007, retain ADR 0006 as accepted, and reconcile `docs/decisions.md` per FR-008
- [X] T025 [P] [US3] Remove obsolete `docs/product.md`, `docs/zed-ux-flow.md` and `docs/research/` after migrating live constraints into retained governance/current docs per FR-007 and FR-013
- [X] T026 [US3] Remove `specs/001-*` through `specs/007-*` after confirming every live contract/invariant exists in `specs/009-zed-marketplace-install/`, `specs/010-repository-convergence/`, the constitution or ADR 0007 per FR-007
- [X] T027 [US3] Update `AGENTS.md` hierarchy text and validate every retained Markdown link plus the exact superseded-history keyword allowlist per FR-008, FR-013 and SC-005

---

## Phase 6: User Story 4 - Remove Generated Residue Safely (Priority: P4)

**Goal**: Remove stale build/validation outputs while preserving locked source inputs and user/agent state.

**Independent Test**: Normal preview selects only allowlisted outputs; normal
cleanup removes them; `.cache/embedded-source/`, `.agents/`, `.codex/` and
persistent data remain.

### Tests and Implementation for User Story 4

- [X] T028 [US4] Record `make clean-preview` output and prove prohibited/preserved paths are absent or retained in `specs/010-repository-convergence/validation.md` per FR-014 and SC-008
- [X] T029 [US4] Run normal cleanup to remove root/nested `target/`, `zed-extension/extension.wasm` and `.cache/zed-local-validation/` while preserving `.cache/embedded-source/` per SC-008
- [X] T030 [US4] Rebuild the exact LSP/runtime/archive, update `ops/marketplace/package.lock.json` and feature-009 package evidence, then run three non-Mock marketplace-shaped translations per FR-010, SC-007 and SC-009
- [X] T031 [US4] Validate `make clean-deep-preview` without executing deep cache deletion and record that explicit confirmation remains required per FR-014 and FR-015

---

## Phase 7: Polish, Full Gates and Convergence

**Purpose**: Prove the post-cleanup tree satisfies every retained contract.

- [X] T032 Run `make workspace-storage-check`, `make worktree-audit`, formatting, Clippy, dependency, unit, extension, repository-boundary and marketplace gates from `specs/010-repository-convergence/quickstart.md` per SC-006
- [X] T033 Verify at least 150 obsolete tracked files were removed, every remaining tracked path maps to `specs/010-repository-convergence/contracts/retained-surface.md`, and no broken backlinks or prohibited current terms remain per SC-003 through SC-005
- [X] T034 Run `speckit-converge`, append and implement any remaining work, then mark feature status and final gate evidence in `specs/010-repository-convergence/spec.md`, `specs/010-repository-convergence/tasks.md` and `specs/010-repository-convergence/validation.md`
- [X] T035 Confirm in `specs/010-repository-convergence/validation.md` that rollback is the reviewable Git diff/commit sequence and no untracked archive or backup was created per FR-017

---

## Dependencies & Execution Order

- Phase 1 blocks task generation approval and is complete before Phase 2.
- T004 must be observed failing before source, automation or documentation removals.
- After T004, Rust/source (Phase 3), automation/operations (Phase 4) and
  documentation/traceability (Phase 5) run in parallel with disjoint ownership.
- T012 is coordinator-owned because root locks are shared; T013 is
  coordinator-owned because extension/package integration is shared.
- Phase 6 starts after Phases 3-5 and repository-boundary checks pass.
- Phase 7 starts after the exact package has been rebuilt from the cleaned tree.

## Parallel Execution Example

```text
Rust/source subagent: T006-T011 and focused portions of T014
Automation/operations subagent: T004, T015-T022
Documentation/traceability subagent: T003, T023-T027
Coordinator: T001-T002, T005, T012-T014 integration, T028-T035
```

## Implementation Strategy

1. Establish governance and the failing negative boundary.
2. Converge the three disjoint source/automation/documentation streams.
3. Integrate shared Cargo/extension/package identities.
4. Remove generated residue only after source validation.
5. Run the complete gate matrix and an append-only convergence pass.

## Notes

- `[P]` tasks use disjoint files and may run concurrently.
- Every user-story task has an exact repository path and traceable requirement.
- Deleted history remains in Git; no in-tree archive is created.
- Deep cache deletion is intentionally not automatic and requires separate explicit confirmation.
