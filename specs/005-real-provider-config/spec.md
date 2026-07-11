# Feature Specification: Real Provider Configuration

**Feature Branch**: `005-real-provider-config`

**Created**: 2026-07-07

**Status**: Implemented

**Input**: User description: "Promote F004 from docs/feature-map.md: configure a real English-to-Spanish translation provider without coupling the core to one backend. Keep mock/offline as the default, require explicit provider configuration, support a local self-hosted real provider first, deny remote providers by default unless configured and confirmed per request, never version real secrets, preserve current request/result/error contracts, send only permitted translatable segments and language/tone metadata to the provider, normalize provider failures, enforce existing limits and timeout, block obvious secrets before any non-local provider call, and keep Zed buffers read-only with no automatic replacement."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Translate With Explicit Local Provider (Priority: P1)

A developer who already has a supported local translation service available can explicitly select it for English-to-Spanish translation and receive a real Spanish result while the product continues to preserve source content and formatting.

**Why this priority**: This is the smallest useful step from deterministic mock output to real translation without widening the privacy boundary beyond the user's machine.

**Independent Test**: Configure the product for a supported local provider, translate safe direct text and an allowed workspace document, and verify the result is real Spanish output rather than deterministic mock text while the source file and editor buffer remain unchanged.

**Acceptance Scenarios**:

1. **Given** the default provider mode is still mock/offline, **When** the user translates text without provider configuration, **Then** the behavior remains deterministic and offline.
2. **Given** a supported local provider has been explicitly configured, **When** the user translates permitted English text, **Then** the product returns Spanish translated text using that provider.
3. **Given** a permitted workspace Markdown file contains prose and protected code regions, **When** the user translates it with the configured local provider, **Then** only permitted translatable segments are sent for translation and protected regions remain unchanged in the output.

---

### User Story 2 - Control Remote Provider Exposure (Priority: P2)

A privacy-conscious user can see when a provider would send content off the machine, and the product blocks that path unless remote use is configured and confirmed for the specific translation request.

**Why this priority**: A real provider feature can create data exposure risk unless remote use is denied by default and the confirmation boundary is testable.

**Independent Test**: Configure a non-local provider target, attempt translation with and without per-request confirmation, and verify that unconfirmed remote use is denied with a normalized actionable error and no source text in diagnostics.

**Acceptance Scenarios**:

1. **Given** a non-local provider target is configured but not confirmed for the request, **When** the user requests translation, **Then** the product denies the operation before sending content.
2. **Given** a non-local provider target is configured and confirmed for the request, **When** the input contains no obvious secrets and stays within limits, **Then** the product may send only permitted translatable segments plus language and tone metadata.
3. **Given** a non-local provider target is confirmed but the input contains an obvious secret pattern, **When** translation is requested, **Then** the product blocks the request before provider contact.

---

### User Story 3 - Understand Provider Failures Safely (Priority: P3)

A user receives clear, normalized feedback when the real provider is unavailable, too slow, rate-limited, misconfigured, or returns an unusable response, without leaking source text, translated text, secrets, or sensitive paths.

**Why this priority**: Real providers fail in ways the mock provider does not, and failures must remain diagnosable without privacy regressions.

**Independent Test**: Trigger each supported failure category with synthetic content and verify the user-visible result includes a stable error code, a safe message, and redacted diagnostics only.

**Acceptance Scenarios**:

1. **Given** the configured provider is unavailable, **When** translation is requested, **Then** the result uses a normalized provider failure code and includes no source content.
2. **Given** the provider exceeds the configured timeout, **When** translation is requested, **Then** the operation stops within the timeout budget and reports a timeout code.
3. **Given** the provider returns a malformed, empty, or segment-count-mismatched response, **When** translation completes, **Then** the product rejects the response and reports a provider failure code.

### Edge Cases

- The product must keep mock/offline as the default when no real provider is configured.
- Provider configuration is present but incomplete, unsupported, malformed, or points to an unsafe target.
- A local provider becomes unavailable during a translation request.
- A provider returns fewer, more, empty, oversized, or non-textual translated segments.
- A provider takes longer than the existing provider timeout.
- A provider indicates throttling, quota, or request rejection.
- Input exceeds existing size, segment, or output limits.
- Input contains obvious secrets before a non-local provider call.
- Provider diagnostics include source text, translated text, secrets, tokens, headers, workspace roots, or sensitive paths.
- The user attempts to translate unsupported files or paths outside the authorized workspace.
- The user expects editor content to be replaced automatically after a real provider translation.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The product MUST keep deterministic mock/offline translation as the default behavior unless a real provider is explicitly configured.
- **FR-002**: Users MUST be able to configure exactly one active real provider mode for a translation request.
- **FR-003**: The first supported real provider mode MUST be local to the user's machine and must not require a paid account.
- **FR-004**: The product MUST preserve backward compatibility with the existing user-facing translation request, success, and error shapes for direct text and allowed workspace files; any new confirmation data MUST be additive and default to denial when absent.
- **FR-005**: The product MUST send providers only permitted translatable segments, source language, target language, and tone.
- **FR-006**: The product MUST NOT send workspace roots, file paths, protected code regions, environment variables, logs, detected secrets, headers, tokens, or unrelated editor/workspace context to any provider.
- **FR-007**: The product MUST deny non-local provider use unless that provider is explicitly configured and the specific translation request includes explicit remote confirmation.
- **FR-008**: The product MUST block any non-local provider request when obvious secret patterns are detected in content that would be sent.
- **FR-009**: The product MUST enforce the existing input, segment, output, and provider timeout limits for real provider translations.
- **FR-010**: The product MUST validate provider responses before presenting output, including segment count, non-empty translated content, text encoding, and output size.
- **FR-011**: The product MUST map provider unavailability, timeout, throttling, rejection, malformed response, and unsupported language-pair failures to stable normalized error codes.
- **FR-012**: The product MUST redact user-visible and diagnostic failure output so it does not include source text, translated text, translatable segments, secrets, tokens, headers, workspace roots, or sensitive paths.
- **FR-013**: The product MUST NOT modify, replace, insert, or delete editor buffer or source-file content as part of real provider translation.
- **FR-014**: The product MUST document safe setup, privacy boundaries, failure behavior, and validation steps for the real provider feature.
- **FR-015**: The product MUST define automated tests or checks for successful local-provider translation, default mock fallback, remote denial, remote confirmation, secret blocking, response validation, timeout handling, and redaction.

### Key Entities

- **Provider Configuration**: The user's explicit selection of provider mode, target location, remote/local classification, and optional secret reference without storing real secret values in versioned files.
- **Provider Invocation**: A single translation attempt containing permitted segments, language metadata, tone, provider mode, remote confirmation state, timing, and normalized outcome.
- **Provider Response**: Provider-returned translated segments and status information that must be validated before becoming user-visible output.
- **Remote Confirmation**: A per-request user decision that allows a configured non-local provider to receive permitted content for that request only.
- **Provider Diagnostic**: Redacted operational information used to explain failures without exposing protected content or sensitive metadata.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With no real provider configured, 100% of existing mock/offline translation validation scenarios continue to pass unchanged.
- **SC-002**: With a supported local provider configured, a reviewer can complete direct-text and allowed-file translation using synthetic English input and receive non-mock Spanish output in at least 3 documented validation runs.
- **SC-003**: 100% of unconfirmed non-local provider attempts are denied before content is sent.
- **SC-004**: 100% of confirmed non-local attempts containing obvious secret patterns are blocked before provider contact.
- **SC-005**: 100% of provider timeout, unavailable, throttled, rejected, malformed-response, and oversized-output test cases return normalized error codes.
- **SC-006**: 100% of automated and manual failure evidence contains no source text, translated text, full translatable segments, secrets, tokens, headers, workspace roots, or sensitive paths.
- **SC-007**: 100% of real-provider translation validation scenarios preserve source files and editor buffers without automatic mutation.
- **SC-008**: Provider requests observed in tests include only permitted segments, language metadata, and tone.
- **SC-009**: Provider calls stop within the existing provider timeout budget for timeout scenarios.

## Assumptions

- The current English-to-Spanish language pair and technical-neutral tone remain the only supported product mode for this feature.
- The current direct-text and authorized workspace-file inputs remain the only supported input paths for this feature.
- The first real provider is local/self-hosted and free/no-pay from the product user's perspective; managed paid provider setup is out of scope.
- Non-local provider support is limited to explicit configuration and per-request confirmation; no non-local provider becomes a default.
- Real provider lifecycle management may be documented as a prerequisite, but this feature does not install global host runtimes, package managers, or services.
- Existing file safety, Markdown preservation, secret detection, limits, and redaction rules remain authoritative.
- Publishing, marketplace packaging, custom Zed UI, and automatic buffer replacement remain outside this feature.
