# Feature Specification: Direct Zed Translation

**Feature Branch**: `006-direct-zed-translation`

**Created**: 2026-07-13

**Status**: Complete; automated and manual Zed validation passed

**Input**: User description: "Promote F010 from docs/feature-map.md: provide a native Zed extension action for English-to-Spanish translation without requiring Agent Panel, Agent profiles, manual prompts, or an intermediary model. Accept reliable editor selection or allowed content from the open workspace document, reuse the existing translation and provider configuration behavior, show a readable preview in Zed, keep buffers unchanged, allow explicit copy when supported, preserve formatting and all current limits, keep remote providers default-deny with per-request confirmation, and document any verified Zed platform limitation instead of rebuilding the product around Agent Panel."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Translate Selected Text Directly (Priority: P1)

A developer selects English prose in Zed and invokes a translation action owned
by the extension, receiving a Spanish result without opening or configuring
Agent Panel and without changing the source buffer.

**Why this priority**: Selection translation is the shortest high-value workflow
and establishes that the extension itself, rather than an intermediary agent,
is the product surface.

**Independent Test**: Install the extension locally, select safe English prose,
invoke its translation action, and verify that a readable Spanish result appears
while the selected text and document remain byte-for-byte unchanged.

**Acceptance Scenarios**:

1. **Given** a user has selected permitted English prose in an open document, **When** the user invokes the extension translation action, **Then** the extension shows the Spanish translation without requiring Agent Panel, an Agent profile, a prompt, or an intermediary model.
2. **Given** a successful direct translation, **When** the user returns to the source document, **Then** the original selection and the rest of the buffer are unchanged.
3. **Given** a direct translation result is visible, **When** the user explicitly chooses to copy it and copying is available, **Then** the translated text is copied without modifying the source document.

---

### User Story 2 - Translate Safe Open-Document Content (Priority: P2)

A developer with no active selection can invoke the same direct extension flow
for an allowed open workspace document and receive a preview that preserves
Markdown structure and protected regions.

**Why this priority**: Open-document translation covers the established file
workflow while retaining the same safety controls and native interaction model.

**Independent Test**: Open an allowed workspace Markdown document containing
prose and protected code, invoke direct translation with no selection, and
verify that only permitted prose is translated, formatting is preserved, and
the source remains unchanged.

**Acceptance Scenarios**:

1. **Given** no text is selected and an allowed workspace document is open, **When** the user invokes the extension translation action, **Then** the extension validates and translates the permitted document content and shows a readable preview.
2. **Given** the open document contains Markdown and protected code regions, **When** translation succeeds, **Then** visible prose is translated while structure and protected content are preserved.
3. **Given** the open document is unsafe, unsupported, outside the authorized workspace, non-UTF-8, binary, or over a current limit, **When** translation is invoked, **Then** the request is rejected before provider contact and the buffer remains unchanged.

---

### User Story 3 - Control Provider Privacy Directly (Priority: P3)

A privacy-conscious user can identify whether the configured provider is local
or remote and must explicitly confirm each remote translation before any
selected or document content leaves the machine.

**Why this priority**: Moving the workflow out of Agent Panel must not bypass
the existing provider consent, secret detection, or diagnostic redaction
boundaries.

**Independent Test**: Exercise the direct action with mock, local, and remote
provider configurations; verify offline default behavior, per-request remote
denial and confirmation, secret blocking, and redacted failures.

**Acceptance Scenarios**:

1. **Given** no real provider is configured, **When** the user invokes direct translation, **Then** the existing deterministic offline behavior remains the default.
2. **Given** a remote provider is configured but the current request is not confirmed, **When** direct translation is invoked, **Then** no content is sent and the user receives an actionable denial.
3. **Given** a remote provider is configured, **When** the user explicitly confirms the current request and the permitted content contains no detected secret, **Then** only permitted translatable segments and required language and tone metadata may be sent.
4. **Given** a remote request is confirmed but a secret is detected in content that would be sent, **When** translation is invoked, **Then** the request is blocked before provider contact.

### Edge Cases

- The direct action is invoked with an empty or whitespace-only selection.
- The selection crosses prose, Markdown syntax, and protected code boundaries.
- The selection or open document changes while a translation is in progress.
- No document is open or the active item does not expose permitted text.
- The open document has not been saved and therefore has no authorized workspace path.
- Selected text, document content, a segment, or provider output reaches an existing size or count limit.
- The configured provider is unavailable, times out, rejects the request, or returns malformed or oversized output.
- A remote provider is configured but confirmation cannot be presented or recorded for the current request.
- A diagnostic or user-visible error could contain source text, translated text, secrets, tokens, headers, workspace roots, or sensitive paths.
- The host editor does not expose a required direct-action or preview capability to extensions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The extension MUST expose a user-invoked translation action in Zed that does not require Agent Panel, Agent profiles, manual prompts, or an intermediary model.
- **FR-002**: The direct action MUST use selected text when Zed exposes a non-empty reliable selection.
- **FR-003**: When no reliable selection exists, the direct action MUST accept an allowed open workspace document only after applying the existing path, file-type, encoding, binary-content, size, and workspace authorization checks.
- **FR-004**: The direct action MUST present translated output in a readable preview owned by the extension workflow.
- **FR-005**: The direct workflow MUST NOT modify, replace, insert, delete, or automatically apply content to an editor buffer or source file.
- **FR-006**: Users MAY explicitly copy a successful translation when the host editor exposes a safe copy capability; copying MUST NOT change the source document.
- **FR-007**: The direct workflow MUST preserve the existing English-to-Spanish language pair, tone behavior, Markdown reconstruction, protected-region handling, input and output limits, segment limits, and provider timeout.
- **FR-008**: The direct workflow MUST keep deterministic mock/offline translation as the default unless a real provider is explicitly configured.
- **FR-009**: Before invocation, the direct workflow MUST identify provider locality in the translation action title as `offline`, `local`, or `remote - confirmation required`, without exposing a provider name, URL, or configuration value.
- **FR-010**: The direct workflow MUST deny every remote request unless the provider is explicitly configured and the user confirms that specific translation request before content is sent.
- **FR-011**: The direct workflow MUST block a confirmed remote request before provider contact when an obvious secret is detected in content that would be sent.
- **FR-012**: Provider invocations MUST receive only permitted translatable segments, source language, target language, and tone; they MUST NOT receive workspace roots, paths, protected code, unrelated editor context, environment data, logs, secrets, tokens, or headers.
- **FR-013**: The direct workflow MUST normalize and redact validation, provider, timeout, and platform failures without including source text, translated text, translatable segments, secrets, tokens, headers, workspace roots, or sensitive paths.
- **FR-014**: If Zed does not expose a capability required for the direct action or preview, the feature MUST document reproducible evidence of the limitation and MUST NOT silently substitute Agent Panel as the primary workflow.
- **FR-015**: The project MUST document local setup, invocation, privacy states, failure behavior, and manual validation for the direct workflow.
- **FR-016**: The project MUST define failing automated tests or explicit checks before each behavioral implementation change, including selection handling, open-document safety, non-mutation, remote denial, confirmation, secret blocking, output validation, timeout, and redaction.
- **FR-SEC-A**: The system MUST NOT modify editor buffers unless a later constitution amendment allows it.
- **FR-SEC-B**: The system MUST reject unsafe file paths, unsupported file types, non-UTF-8 input, and binary content.
- **FR-SEC-C**: The system MUST deny remote provider use unless explicitly configured and confirmed per request.
- **FR-SEC-D**: The system MUST NOT log source text, translated text, segments, secrets, headers, tokens, or sensitive paths.
- **FR-TEST-A**: The system MUST define testable acceptance criteria and negative tests before implementation.

### Key Entities

- **Direct Translation Invocation**: One explicit user action containing a safe selection or authorized open-document reference, provider state, confirmation state, timing, and normalized outcome.
- **Editor Input Snapshot**: The exact permitted input captured for one invocation so that later editor changes cannot cause unintended content to be translated or applied.
- **Translation Preview**: A read-only presentation of translated output associated with one invocation and kept separate from the source buffer.
- **Provider Privacy State**: The offline, local, or remote classification shown by the direct workflow together with whether request-specific confirmation is required and recorded.
- **Platform Capability Evidence**: Reproducible validation of a required Zed extension capability or a concrete limitation that constrains the direct workflow.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reviewer can invoke selection translation and see a readable result in Zed in at most 3 explicit user actions, without opening or configuring Agent Panel.
- **SC-002**: 100% of successful selection and open-document validation runs leave the source buffer and source file byte-for-byte unchanged.
- **SC-003**: 100% of allowed Markdown validation cases preserve document structure and protected code content in the preview.
- **SC-004**: 100% of unsafe, unsupported, unauthorized, non-UTF-8, binary, and over-limit open-document cases are rejected before provider contact.
- **SC-005**: 100% of unconfirmed remote requests are denied before any content is sent, and 100% of confirmed requests containing detected secrets are blocked before provider contact.
- **SC-006**: 100% of observed provider requests contain only permitted segments and required language and tone metadata.
- **SC-007**: 100% of automated and manual failure evidence excludes source text, translated text, full translatable segments, secrets, tokens, headers, workspace roots, and sensitive paths.
- **SC-008**: The documented direct workflow can be completed successfully in at least 3 manual Zed validation runs covering selection, allowed open-document content, and a denied privacy or safety case.
- **SC-009**: Every required but unavailable Zed extension capability has reproducible evidence and an explicit scope decision; zero unavailable capabilities are hidden behind an undocumented Agent Panel fallback.

## Assumptions

- The existing English-to-Spanish language pair and technical-neutral tone remain the only supported product mode for this feature.
- The current translation engine, provider configuration, remote confirmation, secret detection, limits, Markdown preservation, and redaction behavior remain authoritative and reusable.
- Selection is the primary direct workflow; authorized open-document content is the fallback when no reliable non-empty selection is available.
- Translation preview is read-only. Insert, replace, and apply actions are out of scope because the current constitution prohibits editor-buffer mutation.
- Unsaved documents without an authorized canonical workspace path are not eligible for the open-document fallback; a safe selection may still be eligible.
- The exact Zed action entry point and preview surface are selected during planning from capabilities verified against the current official platform contract.
- Agent Panel and the existing translation server remain compatibility or test infrastructure only, not the primary workflow or a fallback presented as feature completion.
- Extension publication and marketplace packaging remain outside this feature.
