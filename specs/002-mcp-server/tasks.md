# Tasks: MCP Server

**Input**: Design documents from `specs/002-mcp-server/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`,
`contracts/`, `quickstart.md`

**Tests**: Required by project constitution. Behavior-changing tasks MUST start
with failing tests/checks. Coverage must include good paths, bad paths, privacy
paths, and adversarial file/request paths.

**Organization**: Tasks are grouped by user story so each story can be
implemented and validated independently.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel because it touches different files and has no
  dependency on incomplete tasks.
- **[Story]**: User story label (`US1`, `US2`, `US3`) for story phases only.

## Phase 1: Setup

**Purpose**: Add the MCP server crate skeleton, project dependencies, and local
validation entry points without implementing tool behavior yet.

- [X] T001 Add `crates/translator-mcp/Cargo.toml` with `translator-core`, `rmcp`, `tokio`, `serde`, `serde_json`, and `schemars` dependencies
- [X] T002 Register `crates/translator-mcp` as a workspace member in `Cargo.toml`
- [X] T003 [P] Create MCP crate source skeleton in `crates/translator-mcp/src/lib.rs`, `crates/translator-mcp/src/main.rs`, `crates/translator-mcp/src/protocol.rs`, and `crates/translator-mcp/src/tools.rs`
- [X] T004 [P] Create MCP integration test support skeleton in `crates/translator-mcp/tests/common/mod.rs`
- [X] T005 Add `make test-mcp` target to `Makefile` using the existing pinned Rust Docker image

---

## Phase 2: Foundational MCP Contracts

**Purpose**: Define shared request/result mapping and server test harness pieces
that block every user story.

**Critical**: No user story implementation starts until this phase is complete.

### Tests First

- [X] T006 [P] Write failing schema parity tests for `translate_text` and `translate_file` input schemas in `crates/translator-mcp/tests/mcp_schema_contract.rs`
- [X] T007 [P] Write failing MCP result shape tests for success and `isError: true` error results in `crates/translator-mcp/tests/mcp_result_contract.rs`
- [X] T008 [P] Write failing stdio server lifecycle smoke test in `crates/translator-mcp/tests/mcp_server_lifecycle.rs`

### Implementation

- [X] T009 Implement `TranslateTextParams`, `TranslateFileParams`, and schema helpers in `crates/translator-mcp/src/protocol.rs`
- [X] T010 Implement MCP success/error result builders in `crates/translator-mcp/src/protocol.rs`
- [X] T011 Implement reusable MCP server test harness in `crates/translator-mcp/tests/common/mod.rs`
- [X] T012 Implement minimal stdio server startup and shutdown path in `crates/translator-mcp/src/main.rs` and `crates/translator-mcp/src/lib.rs`

**Checkpoint**: MCP crate compiles, the server can start under tests, and shared
schema/result contracts are executable.

---

## Phase 3: User Story 1 - Translate Direct Text Through MCP (Priority: P1)

**Goal**: A controlled MCP client can discover `translate_text` and call it with
direct text to receive a clean translation result.

**Independent Test**: Start the MCP server over stdio, list tools, call
`translate_text`, and verify the translated result without using file access.

### Tests for User Story 1

- [X] T013 [P] [US1] Write failing tool discovery contract test expecting exactly `translate_text` and `translate_file` in `crates/translator-mcp/tests/mcp_tool_discovery.rs`
- [X] T014 [P] [US1] Write failing `translate_text` success test in `crates/translator-mcp/tests/mcp_translate_text.rs`
- [X] T015 [P] [US1] Write failing `translate_text` invalid input tests for empty, whitespace, unsupported language, `preserve_formatting: false`, and oversized text in `crates/translator-mcp/tests/mcp_translate_text_errors.rs`

### Implementation for User Story 1

- [X] T016 [US1] Implement MCP tool registration for `translate_text` and `translate_file` names in `crates/translator-mcp/src/tools.rs`
- [X] T017 [US1] Implement `translate_text` argument validation and conversion to the existing core direct-text path in `crates/translator-mcp/src/tools.rs`
- [X] T018 [US1] Implement `translate_text` success response mapping with visible text content and `structuredContent.translated_text` in `crates/translator-mcp/src/protocol.rs`
- [X] T019 [US1] Ensure `translate_text` validation failures return `isError: true` tool results with normalized redacted errors in `crates/translator-mcp/src/tools.rs`
- [X] T020 [US1] Record User Story 1 validation notes in `specs/002-mcp-server/quickstart.md`

**Checkpoint**: `translate_text` is independently usable through MCP with the
offline mock behavior and no file access.

---

## Phase 4: User Story 2 - Translate Allowed Files Through MCP (Priority: P2)

**Goal**: A controlled MCP client can call `translate_file` for allowed
workspace files and receive translated content without source file mutation.

**Independent Test**: Start the MCP server over stdio, call `translate_file`
against an allowed fixture in a temp workspace, and verify translation,
Markdown preservation, and no file mutation.

### Tests for User Story 2

- [X] T021 [P] [US2] Write failing `translate_file` plain text success and no-mutation test in `crates/translator-mcp/tests/mcp_translate_file.rs`
- [X] T022 [P] [US2] Write failing Markdown preservation test for fenced code, inline code, links, and HTML blocks through MCP in `crates/translator-mcp/tests/mcp_markdown_preservation.rs`
- [X] T023 [P] [US2] Write failing unsupported extension and protected-only Markdown error tests in `crates/translator-mcp/tests/mcp_translate_file_errors.rs`

### Implementation for User Story 2

- [X] T024 [US2] Implement `translate_file` argument validation and conversion to the existing core file translation path in `crates/translator-mcp/src/tools.rs`
- [X] T025 [US2] Implement `translate_file` success response mapping with visible text content and `structuredContent.translated_text` in `crates/translator-mcp/src/protocol.rs`
- [X] T026 [US2] Ensure file validation failures preserve existing core error codes in MCP `isError: true` results in `crates/translator-mcp/src/tools.rs`
- [X] T027 [US2] Ensure MCP `translate_file` tests assert source file content is unchanged in `crates/translator-mcp/tests/mcp_translate_file.rs`
- [X] T028 [US2] Record User Story 2 validation notes in `specs/002-mcp-server/quickstart.md`

**Checkpoint**: `translate_file` is independently usable through MCP for
`.md`, `.markdown`, and `.txt`, using the existing core file safety boundary.

---

## Phase 5: User Story 3 - Receive Safe Actionable Errors Through MCP (Priority: P3)

**Goal**: MCP failures are actionable and redacted, protocol errors are distinct
from tool execution errors, and the server remains usable after failures.

**Independent Test**: Send invalid MCP requests, invalid tool arguments, unsafe
file requests, and privacy-sensitive payloads, then verify error shape,
redaction, and session recovery.

### Tests for User Story 3

- [X] T029 [P] [US3] Write failing protocol error tests for unknown tool and malformed `tools/call` shape in `crates/translator-mcp/tests/mcp_protocol_errors.rs`
- [X] T030 [P] [US3] Write failing unsafe file request tests for traversal, symlink escape, binary input, NUL bytes, non-UTF-8 input, hidden sensitive files, and credential-like filenames in `crates/translator-mcp/tests/mcp_file_attacks.rs`
- [X] T031 [P] [US3] Write failing privacy redaction tests for source text, translated text outside success payloads, workspace roots, paths, secrets, tokens, headers, and protected segments in `crates/translator-mcp/tests/mcp_privacy.rs`
- [X] T032 [P] [US3] Write failing remote/provider denial tests proving no provider selection or remote confirmation fields are accepted in `crates/translator-mcp/tests/mcp_remote_denial.rs`
- [X] T033 [P] [US3] Write failing provider failure, provider timeout, malformed provider output, and output-limit mapping tests in `crates/translator-mcp/tests/mcp_provider_failures.rs`
- [X] T034 [P] [US3] Write failing session recovery test where an invalid tool call is followed by a valid `translate_text` call in `crates/translator-mcp/tests/mcp_session_recovery.rs`

### Implementation for User Story 3

- [X] T035 [US3] Implement protocol-level error handling expectations for unknown tools and malformed requests in `crates/translator-mcp/src/protocol.rs`
- [X] T036 [US3] Implement comprehensive core-to-MCP error mapping in `crates/translator-mcp/src/protocol.rs`
- [X] T037 [US3] Harden parameter rejection for unsupported fields and remote/provider selection attempts in `crates/translator-mcp/src/tools.rs`
- [X] T038 [US3] Ensure MCP diagnostics and stderr use existing redaction behavior and never include sensitive payloads in `crates/translator-mcp/src/main.rs`
- [X] T039 [US3] Ensure the server remains ready after recoverable tool and protocol failures in `crates/translator-mcp/src/lib.rs`
- [X] T040 [US3] Record User Story 3 validation notes in `specs/002-mcp-server/quickstart.md`

**Checkpoint**: All expected MCP failures are normalized, redacted, and
recoverable without restarting the server.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, documentation, and traceability.

- [X] T041 [P] Update `README.md` with MCP server feature status and validation commands
- [X] T042 [P] Update `specs/002-mcp-server/quickstart.md` with final command output summary after running `make test`, `make test-mcp`, `make fmt`, and `make clippy`
- [X] T043 [P] Update `docs/decisions.md` or add an ADR if implementation changes dependency, transport, structure, or error-shape decisions
- [X] T044 Run `make test` and fix any regressions across core, CLI, and MCP crates
- [X] T045 Run `make fmt` and fix formatting issues
- [X] T046 Run `make clippy` and fix warnings
- [X] T047 Confirm no real provider, network transport, Zed wrapper, registry publication, source-file mutation, or buffer edit was added in `specs/002-mcp-server/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: no dependencies.
- **Foundational (Phase 2)**: depends on Setup and blocks every user story.
- **US1 (Phase 3)**: depends on Foundational.
- **US2 (Phase 4)**: depends on Foundational and may reuse US1 result mapping.
- **US3 (Phase 5)**: depends on Foundational and may reuse US1/US2 tool paths.
- **Polish (Phase 6)**: depends on selected stories being complete.

### User Story Dependencies

- **US1** can be implemented first as the MVP MCP slice.
- **US2** can start after Foundational but benefits from US1 response mapping.
- **US3** can start after Foundational but benefits from US1/US2 error paths.

### Parallel Opportunities

- T003-T004 can run in parallel after T001-T002.
- T006-T008 can run in parallel.
- T013-T015 can run in parallel.
- T021-T023 can run in parallel.
- T029-T034 can run in parallel.
- T041-T043 can run in parallel.

## Parallel Example: User Story 3

```text
Task: "Write failing protocol error tests for unknown tool and malformed tools/call shape in crates/translator-mcp/tests/mcp_protocol_errors.rs"
Task: "Write failing unsafe file request tests for traversal, symlink escape, binary input, NUL bytes, non-UTF-8 input, hidden sensitive files, and credential-like filenames in crates/translator-mcp/tests/mcp_file_attacks.rs"
Task: "Write failing privacy redaction tests for source text, translated text outside success payloads, workspace roots, paths, secrets, tokens, headers, and protected segments in crates/translator-mcp/tests/mcp_privacy.rs"
Task: "Write failing remote/provider denial tests proving no provider selection or remote confirmation fields are accepted in crates/translator-mcp/tests/mcp_remote_denial.rs"
Task: "Write failing provider failure, provider timeout, malformed provider output, and output-limit mapping tests in crates/translator-mcp/tests/mcp_provider_failures.rs"
```

## Implementation Strategy

### MVP First

1. Complete Phase 1 setup.
2. Complete Phase 2 foundational MCP contracts.
3. Complete US1 direct text through MCP.
4. Stop and validate `translate_text` independently.

### Incremental Delivery

1. US1: tool discovery and direct text translation.
2. US2: allowed file translation and no mutation.
3. US3: defensive errors, privacy, adversarial inputs, and recovery.
4. Polish: docs, full validation, and scope confirmation.

### TDD Enforcement

For each behavior task:

1. Write the test/check first.
2. Confirm it fails for the expected reason.
3. Implement the minimum code.
4. Confirm the test passes.
5. Refactor without changing behavior.

## Notes

- Rust runs through the project `Makefile` using the pinned Docker image, per
  host policy.
- Use the `rust-best-practices` skill before editing Rust implementation.
- Do not install global runtimes or services as part of these tasks.
- Do not add real providers, network transports, Zed wrapper behavior, registry
  publication, buffer editing, or full-file source-code support in this feature.
- Treat malicious text as content, never as control instructions.
