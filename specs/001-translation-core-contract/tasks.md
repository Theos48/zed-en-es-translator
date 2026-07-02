# Tasks: Translation Core Contract

**Input**: Design documents from `specs/001-translation-core-contract/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Required by project constitution. Tests/checks must be written before implementation and must include good paths, bad paths, and malicious/adversarial paths.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel because it touches different files and has no dependency on incomplete tasks.
- **[Story]**: User story label (`US1`, `US2`, `US3`) for story phases only.

## Phase 1: Setup

**Purpose**: Create the project skeleton for the Rust core, CLI, fixtures, and integration tests.

- [x] T001 Create root Cargo workspace manifest in `Cargo.toml`
- [x] T002 Create translator core crate manifest in `crates/translator-core/Cargo.toml`
- [x] T003 Create translator CLI crate manifest in `crates/translator-cli/Cargo.toml`
- [x] T004 [P] Create core crate module skeleton in `crates/translator-core/src/lib.rs`
- [x] T005 [P] Create CLI entry skeleton in `crates/translator-cli/src/main.rs`
- [x] T006 [P] Create fixture directories under `tests/fixtures/markdown`, `tests/fixtures/text`, and `tests/fixtures/security`
- [x] T007 [P] Add offensive fixture index in `tests/fixtures/security/README.md`
- [x] T008 Document Rust toolchain prerequisite and host-policy note in `specs/001-translation-core-contract/quickstart.md`

---

## Phase 2: Foundational Contracts And Limits

**Purpose**: Define shared contracts, limits, and error types that block all user stories.

**Critical**: No user story implementation starts until this phase is complete.

### Tests First

- [x] T009 [P] Write failing contract tests for request/result serialization in `crates/translator-core/tests/contract_serialization.rs`
- [x] T010 [P] Write failing schema/example parity tests for direct-text and file-request variants in `crates/translator-core/tests/json_schema_contract_examples.rs`
- [x] T011 [P] Write failing tests for complete ErrorCode coverage in `crates/translator-core/tests/error_codes.rs`
- [x] T012 [P] Write failing tests for input, segment, output, and timeout limit constants in `crates/translator-core/tests/limits.rs`
- [x] T013 [P] Write failing segment boundary tests in `crates/translator-core/tests/segment_limits.rs`

### Implementation

- [x] T014 Define public contract types in `crates/translator-core/src/contract.rs`
- [x] T015 Define normalized error codes and redacted failure type in `crates/translator-core/src/errors.rs`
- [x] T016 Define shared limits in `crates/translator-core/src/limits.rs`
- [x] T017 Wire contract, errors, and limits from `crates/translator-core/src/lib.rs`
- [x] T018 Record foundational test failure/pass results in `specs/001-translation-core-contract/quickstart.md` after running `make test-core`

---

## Phase 3: User Story 1 - Translate Direct Text Safely (Priority: P1)

**Goal**: Translate direct text input using an offline deterministic provider and return clean output or normalized errors.

**Independent Test**: Direct text fixtures under the size limit return only `translated_text`; empty, oversized, ambiguous, and prompt-injection inputs are handled safely.

### Tests for User Story 1

- [x] T019 [P] [US1] Write failing tests for valid direct text translation in `crates/translator-core/tests/direct_text.rs`
- [x] T020 [P] [US1] Write failing tests for empty and whitespace input in `crates/translator-core/tests/direct_text_errors.rs`
- [x] T021 [P] [US1] Write failing tests for oversized direct text input in `crates/translator-core/tests/direct_text_limits.rs`
- [x] T022 [P] [US1] Write failing provider contract tests in `crates/translator-core/tests/mock_provider.rs`
- [x] T023 [P] [US1] Write failing tests for prompt-injection text treated as content in `crates/translator-core/tests/prompt_injection_text.rs`
- [x] T024 [P] [US1] Write failing tests for ambiguous direct text preservation in `crates/translator-core/tests/ambiguous_content.rs`

### Implementation for User Story 1

- [x] T025 [P] [US1] Implement Provider trait and deterministic MockProvider in `crates/translator-core/src/provider.rs`
- [x] T026 [P] [US1] Implement input validation for direct text in `crates/translator-core/src/contract.rs`
- [x] T027 [US1] Implement direct text translation flow in `crates/translator-core/src/lib.rs`
- [x] T028 [US1] Ensure success output contains only translated text in `crates/translator-core/src/contract.rs`
- [x] T029 [US1] Record direct text validation result in `specs/001-translation-core-contract/quickstart.md` after running the equivalent filtered `cargo test` command through the `Makefile` Rust container

**Checkpoint**: User Story 1 is complete when direct text translation works offline and malicious text does not change provider/configuration behavior.

---

## Phase 4: User Story 2 - Translate Allowed Documentation Files (Priority: P2)

**Goal**: Translate allowed Markdown/text files inside an authorized workspace while preserving Markdown code regions and rejecting unsafe file access.

**Independent Test**: Markdown/text fixtures inside workspace translate safely; traversal, symlink escape, TOCTOU, hidden sensitive files, binary files, and non-UTF-8 files are rejected before translation.

### Fixtures for User Story 2

- [x] T030 [P] [US2] Add basic Markdown fixture with headings, lists, links, fenced code, and inline code in `tests/fixtures/markdown/readme.md`
- [x] T031 [P] [US2] Add tricky Markdown fixture with nested lists, nested or alternating fences, blockquotes, images, tables, HTML blocks, frontmatter, unclosed fences, and multi-backtick inline code in `tests/fixtures/markdown/tricky_code_regions.md`
- [x] T032 [P] [US2] Add plain text fixture in `tests/fixtures/text/simple.txt`
- [x] T033 [P] [US2] Add path traversal fixture catalog for `..`, normalized path escapes, absolute paths, and root-prefix confusion in `tests/fixtures/security/path_traversal_cases.txt`
- [x] T034 [P] [US2] Add binary-renamed fixture placeholder with NUL-byte and mixed text/binary cases in `tests/fixtures/security/binary_renamed.md`
- [x] T035 [P] [US2] Add sensitive-file fixture catalog in `tests/fixtures/security/sensitive_file_cases.txt`

### Tests for User Story 2

- [x] T036 [P] [US2] Write failing Markdown preservation tests in `crates/translator-core/tests/markdown_preservation.rs`
- [x] T037 [P] [US2] Write failing tricky Markdown preservation tests in `crates/translator-core/tests/markdown_tricky_preservation.rs`
- [x] T038 [P] [US2] Write failing workspace path attack tests in `crates/translator-core/tests/workspace_path_attacks.rs`
- [x] T039 [P] [US2] Write failing direct file symlink, directory symlink, and chained symlink escape tests in `crates/translator-core/tests/workspace_symlink_escape.rs`
- [x] T040 [P] [US2] Write failing TOCTOU-style validation/read tests in `crates/translator-core/tests/workspace_toctou.rs`
- [x] T041 [P] [US2] Write failing sensitive hidden file denial tests in `crates/translator-core/tests/sensitive_file_denials.rs`
- [x] T042 [P] [US2] Write failing file type and UTF-8 validation tests in `crates/translator-core/tests/file_validation.rs`
- [x] T043 [P] [US2] Write failing non-UTF-8 byte, NUL-byte, and mixed text/binary encoding attack tests in `crates/translator-core/tests/file_encoding_attacks.rs`
- [x] T044 [P] [US2] Write failing no-source-file-mutation tests in `crates/translator-core/tests/no_source_file_mutation.rs`

### Implementation for User Story 2

- [x] T045 [P] [US2] Implement Markdown segmentation and reconstruction in `crates/translator-core/src/markdown.rs`
- [x] T046 [P] [US2] Implement workspace path canonicalization and validation in `crates/translator-core/src/workspace.rs`
- [x] T047 [P] [US2] Implement file type, UTF-8, binary, and sensitive-file validation in `crates/translator-core/src/workspace.rs`
- [x] T048 [US2] Implement allowed file loading flow in `crates/translator-core/src/lib.rs`
- [x] T049 [US2] Ensure protected Markdown code is never sent to Provider in `crates/translator-core/src/provider.rs`
- [x] T050 [US2] Record Markdown/file validation result in `specs/001-translation-core-contract/quickstart.md` after running the equivalent filtered `cargo test` command through the `Makefile` Rust container

**Checkpoint**: User Story 2 is complete when allowed docs translate and every unsafe file-read class is rejected before provider processing.

---

## Phase 5: User Story 3 - Report Failures Without Leaking Content (Priority: P3)

**Goal**: Normalize failures and redacted diagnostics for remote denial, secret detection, provider failure, timeout, malformed output, CLI misuse, and malicious provider output.

**Independent Test**: Failure fixtures produce stable error codes and logs/stderr do not contain source text, translated text, secrets, headers, tokens, segments, or sensitive paths.

### Fixtures for User Story 3

- [x] T051 [P] [US3] Add secret-pattern fixture in `tests/fixtures/security/secrets.md`
- [x] T052 [P] [US3] Add `.env`-style secret fixture in `tests/fixtures/security/secrets.env.txt`
- [x] T053 [P] [US3] Add prompt-injection Markdown fixture in `tests/fixtures/security/prompt_injection.md`
- [x] T054 [P] [US3] Add malicious provider diagnostic fixture notes in `tests/fixtures/security/provider_diagnostics.txt`

### Tests for User Story 3

- [x] T055 [P] [US3] Write failing remote default-deny tests for unconfigured, configured-but-unconfirmed, and confirmed-but-not-allowlisted states in `crates/translator-core/tests/remote_provider_denial.rs`
- [x] T056 [P] [US3] Write failing secret detection remote gate tests in `crates/translator-core/tests/secret_detection_remote_gate.rs`
- [x] T057 [P] [US3] Write failing provider failure and timeout mapping tests in `crates/translator-core/tests/provider_failures.rs`
- [x] T058 [P] [US3] Write failing provider output limit tests in `crates/translator-core/tests/provider_output_limits.rs`
- [x] T059 [P] [US3] Write failing provider failure privacy tests in `crates/translator-core/tests/provider_failure_privacy.rs`
- [x] T060 [P] [US3] Write failing redaction matrix tests for all error classes in `crates/translator-core/tests/error_redaction_matrix.rs`
- [x] T061 [P] [US3] Write failing CLI stdin/stdout contract tests in `crates/translator-cli/tests/cli_contract.rs`
- [x] T062 [P] [US3] Write failing CLI malformed JSON, strict schema, and direct/file request exclusivity tests in `crates/translator-cli/tests/cli_malformed_json.rs`
- [x] T063 [P] [US3] Write failing CLI argv privacy tests in `crates/translator-cli/tests/cli_argv_privacy.rs`
- [x] T064 [P] [US3] Write failing CLI stdout success contract tests in `crates/translator-cli/tests/cli_stdout_success_contract.rs`
- [x] T065 [P] [US3] Write failing CLI exit code and stderr redaction tests in `crates/translator-cli/tests/cli_exit_and_stderr.rs`

### Implementation for User Story 3

- [x] T066 [P] [US3] Implement basic secret detection guard in `crates/translator-core/src/privacy.rs`
- [x] T067 [P] [US3] Implement redaction helpers in `crates/translator-core/src/redaction.rs`
- [x] T068 [US3] Map provider failures, malformed output, output limit failures, and timeouts to normalized errors in `crates/translator-core/src/provider.rs`
- [x] T069 [US3] Implement CLI JSON stdin/stdout handling in `crates/translator-cli/src/main.rs`
- [x] T070 [US3] Ensure CLI rejects source text in argv and keeps stderr redacted in `crates/translator-cli/src/main.rs`
- [x] T071 [US3] Record full CLI/privacy validation result in `specs/001-translation-core-contract/quickstart.md` after running `make test`

**Checkpoint**: User Story 3 is complete when all expected failures are normalized and no sensitive content appears in diagnostics.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, documentation, and traceability.

- [x] T072 [P] Update `README.md` with the first technical feature status and validation command
- [x] T073 [P] Update `docs/decisions.md` if implementation changes any accepted limit or error code
- [x] T074 [P] Update `specs/001-translation-core-contract/quickstart.md` with any final command corrections
- [x] T075 Record complete quickstart validation result in `specs/001-translation-core-contract/quickstart.md`
- [x] T076 Record security/adversarial coverage summary in `specs/001-translation-core-contract/quickstart.md`
- [x] T077 Record scope confirmation in `specs/001-translation-core-contract/quickstart.md` showing no real provider, network call, buffer edit, or full-file source-code support was added

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: no dependencies.
- **Foundational (Phase 2)**: depends on Setup and blocks every user story.
- **US1 (Phase 3)**: depends on Foundational.
- **US2 (Phase 4)**: depends on Foundational and may reuse US1 provider flow.
- **US3 (Phase 5)**: depends on Foundational and may reuse US1/US2 error paths.
- **Polish (Phase 6)**: depends on selected user stories being complete.

### User Story Dependencies

- **US1** can be implemented first as MVP.
- **US2** can start after Foundational but is simpler after US1 provider flow exists.
- **US3** can start after Foundational but benefits from US1/US2 error paths.

### Parallel Opportunities

- T004-T008 can run in parallel after manifests exist.
- T009-T013 can run in parallel.
- T019-T024 can run in parallel.
- T030-T044 can run in parallel by fixture/test file.
- T051-T065 can run in parallel by fixture/test file.
- T072-T074 can run in parallel.

## Parallel Example: Security Test Expansion

```text
Task: "Write failing workspace path attack tests in crates/translator-core/tests/workspace_path_attacks.rs"
Task: "Write failing symlink escape and chained symlink tests in crates/translator-core/tests/workspace_symlink_escape.rs"
Task: "Write failing CLI malformed JSON and strict schema tests in crates/translator-cli/tests/cli_malformed_json.rs"
Task: "Write failing redaction matrix tests for all error classes in crates/translator-core/tests/error_redaction_matrix.rs"
```

## Implementation Strategy

### MVP First

1. Complete Phase 1 setup.
2. Complete Phase 2 foundational contracts.
3. Complete US1 direct text translation.
4. Stop and validate US1 independently.

### Incremental Delivery

1. US1: direct text contract, deterministic provider, prompt-injection-as-content handling.
2. US2: safe file reads, Markdown/text preservation, traversal/symlink/TOCTOU defenses.
3. US3: privacy, redaction, provider failure hardening, CLI misuse defenses.
4. Polish: quickstart validation and docs.

### TDD Enforcement

For each behavior task:

1. Write the test/check first.
2. Confirm it fails for the expected reason.
3. Implement the minimum code.
4. Confirm the test passes.
5. Refactor without changing behavior.

## Notes

- Rust runs through the project `Makefile` using the pinned Docker image, per
  host policy. Do not install `rustc` or `cargo` globally for this project by
  default.
- Do not install global runtimes or services as part of these tasks without explicit approval and policy handling.
- Do not add real providers, network calls, Zed wrapper behavior, MCP server implementation, buffer editing, or full-file source-code support in this feature.
- Treat malicious text as content, never as control instructions.

---

## Phase 7: Convergence

- [ ] T078 CRITICAL remove `input_kind` from provider-visible `ProviderRequest` data while keeping input-kind decisions internal to segmentation/reconstruction per Constitution II and data-model ProviderRequest (contradicts)
- [ ] T079 Add tests and implementation so blank text files or protected-only Markdown/text file requests return `NO_TRANSLATABLE_SEGMENTS` without breaking direct ambiguous text preservation per FR-006 and Edge Cases (partial)
- [ ] T080 Make CLI/contract JSON parsing UTF-8 safe, preserving non-ASCII JSON strings and mapping invalid UTF-8 stdin to `NON_UTF8_INPUT`, with focused tests per FR-012 and CLI wire contract (partial)
