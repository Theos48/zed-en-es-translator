# Feature Specification: Operational Real Providers

**Feature Branch**: `007-operational-providers`

**Created**: 2026-07-14

**Status**: Partial Real-Service Validation — Azure and Direct Zed Pending

**Input**: User description: "Promote F011 from docs/feature-map.md: configure and validate one real local/offline English-to-Spanish provider and one real remote/online provider from both the CLI and the direct Zed workflow. Keep MockProvider as the deterministic default; isolate the local provider inside the project with documented lifecycle, persistence, verification, updates, and rollback; require HTTPS allowlisting, secrets outside the repository, and per-request confirmation for remote use; preserve existing limits, segmentation, Markdown protection, normalized errors, non-mutation, secret blocking, and redaction; require redacted manual evidence against both real services; exclude publication, paid-only providers, global host installation, and buffer mutation."

## Clarifications

### Session 2026-07-14

- Q: What access policy should govern selection of the real remote provider? → A: A free account and API key are allowed, but a paid subscription must not be mandatory.
- Q: Does F011 extend or validate the MCP/Agent Panel path as a product surface? → A: No. MCP/Agent Panel remains a compatibility bridge only; new operational acceptance targets the CLI and direct Zed workflow, while existing MCP regressions must keep passing.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Translate With a Real Local Provider (Priority: P1)

A developer can prepare and run one supported local provider inside the project boundary, translate synthetic English content into Spanish from both the command line and the direct Zed workflow, and continue translating without Internet access after the provider artifacts are prepared.

**Why this priority**: A real offline path delivers the feature's primary value while keeping source content on the user's machine and preserving the project's privacy-first default.

**Independent Test**: Follow the documented local-provider setup from a clean project checkout, disable external network access after preparation, translate permitted synthetic text and Markdown through the command line and Zed, and verify non-mock Spanish output while the source remains unchanged.

**Acceptance Scenarios**:

1. **Given** the supported local provider artifacts have been prepared within the project boundary, **When** the user starts the provider and translates permitted English text without Internet access, **Then** the product returns a real Spanish translation without contacting a remote service.
2. **Given** the local provider is running and explicitly selected, **When** the user translates the same safe synthetic input through the command line and the direct Zed workflow, **Then** both surfaces return provider-backed Spanish output and identify the request as local before execution.
3. **Given** a permitted Markdown document contains prose and protected regions, **When** the user translates it with the local provider, **Then** only permitted segments are translated, protected regions remain unchanged, and neither the file nor the editor buffer is modified.
4. **Given** no real provider is explicitly selected, **When** a translation is requested, **Then** the deterministic mock/offline behavior remains the default.

---

### User Story 2 - Use a Real Remote Provider With Explicit Consent (Priority: P2)

A developer can explicitly configure one supported remote provider that does not require a paid account, see that content would leave the machine, and approve or reject each individual translation request before any permitted content is sent.

**Why this priority**: The remote path demonstrates practical online translation while retaining user control over disclosure and preventing silent network use.

**Independent Test**: Configure the supported remote provider using only safe references to any required credentials, attempt synthetic translations through the command line and Zed with confirmation denied and granted, and verify that only confirmed, secret-free requests contact the allowlisted service.

**Acceptance Scenarios**:

1. **Given** the supported remote provider is configured but the current request has not been confirmed, **When** translation is requested, **Then** the product identifies the request as remote and denies it before provider contact.
2. **Given** the user explicitly confirms one remote request and the content passes all safety checks, **When** the translation runs, **Then** only permitted segments and protocol-required language metadata are sent to the allowlisted HTTPS service, while tone/format invariants are validated internally and omitted externally when the reviewed protocol has no such fields.
3. **Given** one remote request was confirmed, **When** a later request is made, **Then** the product requires a new confirmation and does not reuse the earlier decision.
4. **Given** a confirmed remote request contains a detected secret, **When** translation is requested, **Then** the product blocks it before provider contact and reports a normalized redacted error.

---

### User Story 3 - Operate and Recover the Local Provider (Priority: P3)

A maintainer can start, stop, verify, update, and roll back the supported local provider using documented project commands without installing a global runtime or service, losing required provider artifacts, or disturbing translation source files.

**Why this priority**: A provider is not operationally useful if it can only be started once or if recovery requires undocumented host changes.

**Independent Test**: Execute the documented lifecycle from a clean checkout, verify idempotent start/stop behavior and persistent artifact handling, simulate an unsuccessful update, apply rollback, and confirm that the last known-good provider can translate the synthetic acceptance sample again.

**Acceptance Scenarios**:

1. **Given** the local provider is not running, **When** the maintainer follows the documented start and health verification steps, **Then** exactly one project-scoped provider instance becomes ready for translation.
2. **Given** the provider is running, **When** the maintainer invokes the documented stop procedure twice, **Then** the provider stops cleanly and repeated cleanup does not delete persistent artifacts.
3. **Given** a new provider version or model is prepared, **When** the update verification fails, **Then** the documented rollback restores the last known-good configuration and translation capability.
4. **Given** the project is removed or its isolated provider environment is explicitly cleaned, **When** the documented cleanup is followed, **Then** no global runtime, service, or unmanaged project artifact remains on the host.

---

### User Story 4 - Diagnose Real Provider Failures Safely (Priority: P4)

A reviewer can distinguish provider unavailability, timeout, invalid response, rejected remote consent, secret blocking, and configuration errors without exposing source text, translated text, provider credentials, sensitive URLs, or local paths in logs or evidence.

**Why this priority**: Live services fail differently from test doubles, and operational evidence must prove both recovery behavior and privacy guarantees.

**Independent Test**: Trigger each required local and remote failure with synthetic data, inspect all user-visible errors, logs, and manual evidence, and verify stable outcomes, complete redaction, and unchanged source content.

**Acceptance Scenarios**:

1. **Given** the selected provider is unavailable or exceeds the existing timeout, **When** translation is requested, **Then** the product returns the corresponding normalized error within the existing time budget.
2. **Given** a real provider returns an empty, malformed, mismatched, or oversized result, **When** the response is validated, **Then** the product rejects it without presenting partial success or mutating source content.
3. **Given** a reviewer completes the real-provider validation matrix, **When** evidence is recorded, **Then** it contains only synthetic case identifiers, safe locality labels, commands, timestamps, normalized outcomes, and redacted observations.

### Edge Cases

- Initial local-provider preparation requires network access but network is unavailable, interrupted, or returns incomplete artifacts.
- Local-provider artifacts exist but are incompatible with the documented provider version, corrupt, or only partially updated.
- Start is requested while the local provider is already healthy, or stop is requested when no instance is running.
- The configured local endpoint resolves outside the permitted local boundary or the remote endpoint is not the exact allowlisted HTTPS host.
- The local or remote provider becomes unavailable between the readiness check and the translation request.
- A provider exceeds the existing timeout, rate-limits a request, rejects credentials, or returns an empty, malformed, non-textual, reordered, mismatched, or oversized response.
- The user dismisses remote confirmation, the confirmation response does not match the pending request, or a later request attempts to reuse previous consent.
- Content contains a detected secret after remote confirmation but before provider contact.
- Credential configuration names a missing secret reference, exposes a raw secret in versioned configuration, or introduces an unsupported setting.
- The same synthetic sample produces wording differences between providers while remaining a valid English-to-Spanish translation.
- Evidence collection encounters provider text, a sensitive URL, a token, a local path, source content, or translated content that must not be recorded.
- A validation run succeeds through one surface and fails through the other, revealing configuration drift between the command line and direct Zed workflow.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The product MUST support exactly one documented real local/offline provider path and exactly one documented real remote/online provider path for English-to-Spanish translation in this feature.
- **FR-002**: The product MUST keep deterministic `MockProvider` behavior as the default whenever no real provider is explicitly configured.
- **FR-003**: The local provider MUST run within a project-scoped isolated environment and MUST NOT require a global host runtime, package, database, or service.
- **FR-004**: The local provider MUST translate without Internet access after its reviewed provider artifacts and language resources have been prepared.
- **FR-005**: The project MUST provide documented, repeatable operations for local-provider preparation, start, readiness verification, stop, update, rollback, persistent artifact handling, and explicit cleanup.
- **FR-006**: Local-provider start, stop, verification, and cleanup operations MUST be idempotent or fail safely with an actionable redacted explanation.
- **FR-007**: The remote provider MUST use an allowlisted HTTPS destination and MUST NOT become the default provider.
- **FR-008**: The base acceptance path for both real providers MUST NOT require a paid subscription; the remote provider MAY require a free account and API key, but every credential MUST use a secret reference whose value remains outside versioned files and recorded evidence.
- **FR-009**: The direct Zed workflow MUST identify provider locality as offline/mock, local, or remote-confirmation-required before a translation executes, without exposing provider names, URLs, executable paths, models, or credential details.
- **FR-010**: Every remote translation invocation MUST require a request-specific user confirmation; denial, dismissal, stale confirmation, mismatched confirmation, or absent confirmation MUST prevent provider contact.
- **FR-011**: Remote confirmation MUST NOT bypass secret detection, provider allowlisting, input validation, limits, response validation, or redaction.
- **FR-012**: Every real provider invocation at the internal provider boundary MUST receive only permitted translatable segments, fixed source and target languages, the existing technical-neutral tone, and formatting intent; it MUST NOT receive workspace roots, file paths, protected regions, environment dumps, logs, unrelated editor context, or unselected secrets. A provider protocol that has no reviewed tone/format field MUST validate those values internally and omit them from the external payload instead of inventing unsupported metadata.
- **FR-013**: Both real provider paths MUST preserve the existing 20 KiB input, 4 KiB segment, 256-segment, 40 KiB output, and 15-second provider timeout limits.
- **FR-014**: Both real provider paths MUST preserve existing Markdown segmentation and reconstruction, supported-file validation, normalized success/error contracts, and protected-content behavior.
- **FR-015**: The feature MUST reject unavailable providers, timeouts, rejected credentials or quota, unsafe targets, and empty, malformed, mismatched, non-textual, or oversized responses with stable normalized errors.
- **FR-016**: Neither real provider path MUST create edits, modify buffers, write source files, place translated content on the clipboard, or introduce Agent Panel as the primary workflow.
- **FR-017**: Logs, diagnostics, standard error, failure output, validation records, and manual evidence MUST NOT contain source text, translated text, permitted segments, raw response bodies, tokens, secret values, sensitive URLs, headers, environment contents, workspace roots, or sensitive local paths. The existing ephemeral command-line success result MAY contain translated text for the requesting user, but that content MUST NOT be copied into logs or recorded evidence.
- **FR-018**: Automated checks MUST cover configuration validation, payload minimization, default mock behavior, local readiness/failure, remote denial, request-specific confirmation, secret blocking, timeouts, malformed responses, limits, redaction, and non-mutation using controlled test doubles where appropriate.
- **FR-019**: Manual acceptance MUST exercise the actual supported local and remote services with synthetic content through both the command line and the direct Zed workflow; test doubles alone MUST NOT close this feature. MCP/Agent Panel is not an F011 acceptance surface and receives no new provider-specific product flow, although its existing compatibility regression suites MUST remain green.
- **FR-020**: Manual evidence MUST record provider version or service identity in a separately redacted operational field, validation surface, safe locality, synthetic case identifier, timestamp, normalized outcome, and reviewer result without recording translated content or secrets.
- **FR-021**: The local-provider rollback procedure MUST restore a last known-good version/configuration and successful synthetic translation after a failed update or incompatible artifact is detected.
- **FR-022**: The project MUST document prerequisites, expected storage and network use, privacy boundaries, lifecycle commands, failure recovery, validation steps, and complete removal for the supported provider paths.
- **FR-023**: The project MUST document provider software and language-resource licenses and MUST NOT vendor, bundle, publish, or redistribute any model whose redistribution rights are not explicitly established; unresolved licensing MUST remain a visible publication gate.
- **FR-SEC-A**: As the security traceability alias of FR-016, the product MUST NOT modify editor buffers unless a later constitution amendment allows it.
- **FR-SEC-B**: The product MUST reject unsafe file paths, unsupported file types, non-UTF-8 input, and binary content.
- **FR-SEC-C**: As the security traceability alias of FR-007, FR-010, and FR-011, the product MUST deny remote provider use unless explicitly configured and confirmed per request.
- **FR-SEC-D**: As the security traceability alias of FR-017, the product MUST NOT log source text, translated text, segments, secrets, headers, tokens, or sensitive paths.
- **FR-TEST-A**: As the test traceability alias of FR-018, the product MUST define testable acceptance criteria and negative tests before implementation.

### Key Entities

- **Operational Provider Profile**: The supported provider identity and version, safe locality classification, required artifacts, language pair, credential-reference requirement, allowlisted destination, readiness signal, and lifecycle state without storing secret values.
- **Local Provider Environment**: The project-scoped runtime boundary, prepared artifacts, persistent language resources, active version, last known-good version, health state, update state, and cleanup state for the offline provider.
- **Remote Access Configuration**: The reviewed HTTPS destination, allowlist match, optional secret-reference name, remote-use enablement, and safe validation status for the online provider.
- **Provider Invocation**: One command-line or Zed translation attempt with synthetic case identifier, provider locality, request-specific confirmation state, timing, and normalized outcome; source and translated content are excluded from recorded metadata.
- **Validation Record**: A redacted manual result that identifies provider/service version, surface, locality, synthetic case, timestamp, expected outcome, actual normalized outcome, reviewer result, and non-mutation/redaction checks.
- **Rollback Point**: The last known-good local-provider version, configuration reference, artifact identity, readiness result, and restoration outcome retained without source content or secrets.

### Scope Boundaries

In scope:

- operationalizing and validating one real local/offline provider and one real remote/online provider;
- project-scoped provider lifecycle and rollback documentation;
- command-line and direct Zed validation with synthetic English-to-Spanish content;
- safe provider-locality disclosure, remote confirmation, secret blocking, failure handling, redaction, and evidence.

Out of scope:

- extension packaging or publication;
- a paid-only provider as either required acceptance path;
- installing provider runtimes or services globally on Fedora;
- supporting arbitrary providers, arbitrary endpoints, additional language pairs, or automatic provider discovery;
- translating full source-code files, changing existing limits, weakening secret detection, or adding buffer/file/clipboard mutation;
- replacing the direct Zed workflow with Agent Panel or adding a new custom Zed UI surface;
- storing real secrets, source text, translations, or provider response bodies in the repository or evidence;
- adding a new MCP/Agent Panel provider flow or treating that compatibility bridge as an F011 acceptance surface; existing MCP regressions remain required.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reviewer completes 4 successful real-service acceptance runs: local through the command line, local through direct Zed, remote through the command line, and remote through direct Zed, all using synthetic English input and returning valid non-mock Spanish output.
- **SC-002**: After initial preparation, 100% of documented local-provider acceptance runs complete with external network access disabled and no contact outside the permitted local boundary.
- **SC-003**: A reviewer can execute the documented local-provider start, readiness, translation, stop, restart, and cleanup sequence from a clean checkout without any global host installation or undocumented command.
- **SC-004**: 100% of unconfirmed, denied, dismissed, stale, or mismatched remote-request cases stop before provider contact; every new remote request requires a fresh confirmation.
- **SC-005**: 100% of confirmed remote cases containing a detected secret stop before provider contact.
- **SC-006**: 100% of acceptance and negative cases leave tracked source files and open editor buffers byte-for-byte unchanged.
- **SC-007**: 100% of inspected logs, diagnostics, standard error, failure outputs, and recorded evidence from the acceptance matrix contain none of the prohibited source, translation, credential, sensitive URL, response-body, workspace, or path data; successful command-line translation output is observed ephemerally and is not retained as evidence.
- **SC-008**: 100% of unavailable, timeout, unsafe-target, rejected-credential/quota, malformed, mismatched, non-textual, and oversized-response cases return the documented normalized outcome within the existing provider timeout budget.
- **SC-009**: Following a failed local-provider update simulation, a reviewer restores the last known-good provider and completes the synthetic health translation using only the documented rollback procedure.
- **SC-010**: All pre-existing mock, provider-contract, direct-Zed, segmentation, file-safety, limit, privacy, and redaction validation suites continue to pass unchanged.
- **SC-011**: The final manual record contains all 4 required successful real-service runs plus the required negative scenarios, with every required field present and no prohibited evidence content.

## Implementation Traceability (2026-07-14)

| Requirement group | Implemented evidence | Status |
|---|---|---|
| FR-001-FR-004, FR-007-FR-014 | Exact provider matrix, pinned local profile, fixed Azure adapter, mock default, payload/limit/consent tests | automatic gates pass |
| FR-005-FR-006, FR-021-FR-022 | Project Make lifecycle, isolated provider plus loopback relay, candidate/current/previous state, offline verification, update, rollback and token-gated removal tests | automatic gates pass; real prepare/offline/idempotency/rollback pass; failed-update and clean rows pending |
| FR-015-FR-018, FR-SEC-A-D, FR-TEST-A | Core/CLI/LSP/Zed failure, timeout, redaction, secret, non-mutation and evidence-contract matrices | automatic gates pass |
| FR-019-FR-020; SC-001-SC-009, SC-011 | Four real service/surface rows and real negative/offline/rollback evidence | T056 partial: local CLI/offline/rollback and three pre-contact negatives pass; direct Zed, Azure and remaining rows pending |
| FR-023 | Provider image/license metadata recorded; model license unresolved and redistribution forbidden | implemented publication gate; upstream resolution pending |
| SC-010 | Full workspace, direct Zed, extension, format, Clippy and cargo-deny gates | pass |

Automatic evidence uses only controlled loopback/process doubles and public
synthetic fixtures. The separate partial manual record proves the local CLI
path but does not substitute for the missing real Zed/Azure rows; F011
therefore remains incomplete.

## Assumptions

- Initial preparation of local provider artifacts may require an explicit network download, but normal local translation is offline after preparation.
- The selected local provider supports a reproducible project-scoped execution path on the target Linux workstation. Its English-to-Spanish model may be acquired only as a user-local prerequisite under a documented license caveat; the project does not assume redistribution rights, and acceptance blocks if user-local use cannot be supported responsibly.
- The selected remote provider offers a real no-payment acceptance path; it may require a free account and API credential, but not a paid subscription, and the credential remains outside the repository.
- Synthetic acceptance content is designed to exercise prose, Markdown preservation, protected regions, and secret blocking without using private or production material.
- Existing provider selection, translation, error, segmentation, LSP confirmation, and Zed launch contracts remain authoritative unless planning identifies a documented additive change.
- MCP/Agent Panel remains a compatibility bridge only; F011 implementation and real-service acceptance focus on the CLI and direct Zed workflow.
- Provider wording may differ; acceptance validates language direction, non-empty coherent output, contract preservation, and safety rather than exact phrase equality.
- Operational artifacts may persist inside documented project-managed storage, but translated content, source content, and credentials do not.
