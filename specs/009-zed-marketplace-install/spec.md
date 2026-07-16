# Feature Specification: Plug-and-Play Zed Marketplace Installation

**Feature Branch**: `009-zed-marketplace-install`

**Created**: 2026-07-16

**Status**: Draft

**Input**: User description: "A user installs the translator from Zed's
Extension Gallery and receives real English-to-Spanish translations without a
terminal, repository checkout, manual binary path, Docker, service, account or
API key. First-use preparation is automatic and later translation is offline."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install and Translate Without Setup (Priority: P1)

A person using a clean supported Zed installation finds the extension in the
Extension Gallery, installs it, opens Markdown or plain text, invokes the
translation action and receives a real Spanish preview without performing any
developer or system-administration step.

**Why this priority**: This is the product. If a user must clone the repository,
open a terminal, configure a path, run a container or start a service, the
feature has failed even if the translation engine works technically.

**Independent Test**: On a clean supported Zed profile with no project checkout,
toolchain, container, configured provider or translation binary, install the
published extension and translate one public English fixture using only Zed.

**Acceptance Scenarios**:

1. **Given** a clean supported Zed installation, **When** the user installs the
   extension from the Extension Gallery and opens a supported document, **Then**
   the direct translation action is available without configuration.
2. **Given** the extension is installed but its local translation package is
   absent, **When** the user invokes translation, **Then** required preparation
   happens automatically with visible progress and the requested translation
   completes without leaving Zed.
3. **Given** preparation completed, **When** the user translates an English
   selection or supported document, **Then** Zed shows a real Spanish read-only
   preview and the source remains byte-for-byte unchanged.
4. **Given** a supported user has never opened a terminal for this project,
   **When** the primary journey completes, **Then** no repository, command,
   binary path, service, account, API key or provider setting was required.

---

### User Story 2 - Recover Automatically From Preparation Problems (Priority: P2)

A user receives a clear in-editor status when the initial download cannot
finish because the network is unavailable, storage is insufficient, content is
incomplete or verification fails. Retrying from the normal translation flow
recovers safely without manual cleanup.

**Why this priority**: Automatic setup is only plug-and-play when common first-run
failures do not send the user to a terminal or leave a broken installation.

**Independent Test**: Simulate interruption, corruption, insufficient storage
and loss of network during first use, then restore the condition and verify that
one retry through Zed completes while no incomplete package is executed.

**Acceptance Scenarios**:

1. **Given** preparation cannot reach its source, **When** the first attempt
   fails, **Then** Zed shows a concise retryable message and preserves the
   document without exposing its content.
2. **Given** an incomplete or invalid package exists, **When** translation is
   requested again, **Then** the invalid package is never executed and the
   extension safely retries or reports the same actionable failure.
3. **Given** Zed closes during preparation, **When** it starts again and the
   user retries, **Then** preparation resumes or restarts safely without manual
   file removal.
4. **Given** a working package and a newer package that fails validation,
   **When** an update is attempted, **Then** the previous working translation
   path remains available.

---

### User Story 3 - Remain Private, Offline and Removable (Priority: P3)

After the one-time public package acquisition, a user translates with external
networking unavailable. The extension owns all of its runtime data, exposes no
document content during installation or translation and can be disabled or
removed through normal Zed controls.

**Why this priority**: The extension must behave like an ordinary trustworthy
editor extension rather than a hidden remote service or a permanent host
installation.

**Independent Test**: Prepare the extension once, disable external networking,
run the complete public fixture set, then disable and uninstall through Zed and
verify the defined extension-owned state and source documents.

**Acceptance Scenarios**:

1. **Given** preparation completed, **When** external networking is disabled,
   **Then** every supported translation flow remains functional.
2. **Given** any preparation, translation or update attempt, **When** network
   activity is inspected, **Then** only public package acquisition occurs and
   no source text, translation, workspace path, credential or secret is sent.
3. **Given** the extension is disabled, **When** a supported document is opened,
   **Then** no translator process or network operation starts.
4. **Given** the extension is uninstalled, **When** removal completes through
   Zed, **Then** no terminal command is required to remove its owned runtime and
   model data.

---

### User Story 4 - Understand Unsupported Platforms (Priority: P4)

A user on a platform outside the first supported release receives a direct,
accurate explanation rather than a failed download, a shell instruction or a
misleading configuration prompt.

**Why this priority**: A marketplace listing can be visible beyond the first
validated platform, and unsupported users must fail safely.

**Independent Test**: Present every unsupported operating-system and processor
class reported by the extension host and verify a stable in-editor unsupported
platform result with zero acquisition and zero document mutation.

**Acceptance Scenarios**:

1. **Given** an unsupported platform, **When** the extension is activated,
   **Then** it performs no package download and reports the supported platform
   class in Zed.
2. **Given** an unsupported platform, **When** the user invokes translation,
   **Then** the response never recommends cloning the repository, running a
   build command or configuring an arbitrary executable.

### Edge Cases

- Zed opens two supported workspaces concurrently during first preparation.
- Available storage becomes insufficient before or during acquisition.
- The server reports an unavailable, redirected, truncated, oversized or
  integrity-invalid package.
- Zed or the host stops immediately before a package becomes usable.
- A valid previous package exists while a replacement is incomplete or invalid.
- The user is offline before the first successful preparation.
- The user is offline after preparation and restarts Zed.
- The document contains code fences, inline code, links, Markdown structure,
  Unicode, empty selections or content near existing limits.
- The selected range becomes stale while a translation is running.
- A workspace contains sensitive paths, binary data, non-UTF-8 data or secret-like
  content.
- The extension is disabled or uninstalled while its local process is idle or
  active.
- The Extension Gallery or package source is temporarily unavailable.
- The installed extension version and locally prepared package version differ.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The extension MUST be installable from Zed's Extension Gallery.
- **FR-002**: The first supported release MUST provide real English-to-Spanish
  translation on Linux `x86_64`.
- **FR-003**: The primary user journey MUST require zero terminal commands,
  repository checkouts, developer builds, containers, locally started services,
  accounts, API keys, arbitrary executable paths and manual provider settings.
- **FR-004**: Installing the extension MUST make the existing direct translation
  action available for Markdown and plain-text documents.
- **FR-005**: The extension MUST automatically acquire any absent local package
  required for translation as part of the normal Zed flow.
- **FR-006**: First preparation and updates MUST expose visible checking,
  downloading, ready and actionable failure states in Zed.
- **FR-007**: Every acquired package MUST have a fixed expected identity, size,
  compatibility, origin and license conclusion and MUST be verified before use.
- **FR-008**: Package acquisition MUST use only fixed public sources and MUST NOT
  include document content, translations, workspace paths, credentials or
  secrets in a request.
- **FR-009**: After first successful preparation, translation, readiness and
  normal restart MUST work with external networking unavailable.
- **FR-010**: The local no-account translation path MUST be the usable default;
  Mock, remote and developer-only providers MUST NOT be required for the primary
  marketplace journey.
- **FR-011**: Incomplete, corrupt, incompatible, oversized or unlicensed
  packages MUST never be executed or become current.
- **FR-012**: A failed or interrupted first preparation MUST be retryable through
  the normal Zed action without manual cleanup.
- **FR-013**: A failed update MUST preserve the last verified working package and
  translation path.
- **FR-014**: Concurrent Zed windows MUST not corrupt, duplicate unsafely or
  partially activate shared extension-owned package state.
- **FR-015**: All installed runtime and model data MUST remain within storage
  owned by the Zed extension and MUST NOT install host-global packages,
  runtimes, services or configuration.
- **FR-016**: Disabling the extension MUST stop new translator startup and
  acquisition; uninstalling MUST offer a complete no-terminal removal path for
  extension-owned package data.
- **FR-017**: Unsupported platforms MUST perform zero acquisition and present a
  stable in-editor explanation of the first supported platform.
- **FR-018**: The extension MUST preserve the existing 20 KiB request, 4 KiB
  segment, 256-segment, 40 KiB response and 15-second translation limits.
- **FR-019**: The extension MUST preserve Markdown structure, code, links and
  ambiguous content and MUST NOT modify editor buffers or source files.
- **FR-020**: The extension MUST reject unsafe workspace paths, unsupported file
  types, binary content and non-UTF-8 content before translation.
- **FR-021**: The extension MUST NOT log or expose source text, translated text,
  segments, secrets, credentials, headers, tokens, sensitive paths or raw child
  output.
- **FR-022**: Translation processes MUST be bounded, terminable, non-listening
  and unable to perform normal-runtime package acquisition.
- **FR-023**: The distributed extension and local translation package MUST
  include or link every required license, attribution, notice and corresponding
  source obligation for their exact published contents.
- **FR-024**: Publication MUST NOT depend on an unresolved model or runtime
  license; a candidate without sufficient distribution evidence MUST be
  replaced rather than exposed as a user approval prompt.
- **FR-025**: Existing MCP, Agent Panel, configurable local and remote provider
  paths MAY remain compatible but MUST NOT appear in or block the primary
  marketplace journey.
- **FR-026**: Feature completion MUST include a clean-install interactive test
  using the marketplace-shaped package, not a development extension or local
  repository binary.
- **FR-027**: Feature completion MUST include a submitted marketplace release
  that a clean Zed installation can obtain through the normal extension
  distribution path.
- **FR-SEC-A**: The system MUST NOT modify editor buffers unless a later
  constitution amendment allows it.
- **FR-SEC-B**: The system MUST reject unsafe file paths, unsupported file types,
  non-UTF-8 input and binary content.
- **FR-SEC-C**: Public package acquisition MUST be isolated from translation
  content; any optional remote translation provider MUST remain explicitly
  configured and confirmed per request.
- **FR-SEC-D**: The system MUST NOT log source text, translated text, segments,
  secrets, headers, tokens or sensitive paths.
- **FR-TEST-A**: Behavior changes MUST begin with failing acceptance, contract or
  negative tests and MUST include a real clean-install validation before
  completion.

### Key Entities

- **Marketplace Installation**: The installed extension version, supported
  platform declaration and visible installation state presented by Zed.
- **Local Translation Package**: The complete verified runtime and language
  resources needed for offline translation, treated as one usable version.
- **Published Package Record**: The fixed identity, origin, size, compatibility,
  license and notice information for one supported package.
- **Preparation State**: Absent, checking, downloading, ready or failed state
  owned by the extension and safe across restart and concurrency.
- **Translation Invocation**: One bounded, content-protected request from the
  direct Zed action to the verified local package.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In three of three independent clean-install acceptance runs on
  supported systems, a user installs from the Extension Gallery and obtains a
  real translation with zero terminal commands and zero manual settings.
- **SC-002**: On a controlled 10 Mbps connection with sufficient storage, at
  least 19 of 20 first-use preparations reach a usable translation within five
  minutes.
- **SC-003**: After preparation, 20 of 20 public synthetic cases translate with
  external networking disabled and every individual request completes within
  15 seconds.
- **SC-004**: Across the complete real acceptance set, source documents remain
  byte-for-byte unchanged and protected Markdown/code structures remain intact
  in 100% of cases.
- **SC-005**: All tested interruption, corruption, insufficient-storage and
  failed-update scenarios execute zero invalid packages and recover through an
  in-Zed retry without manual cleanup.
- **SC-006**: Package acquisition and offline translation expose zero document
  content, translation content, credentials, secrets or sensitive paths in
  network requests, logs and user-facing diagnostics.
- **SC-007**: The complete active local translation package remains below 128
  MiB installed and normal translation remains below 1 GiB peak memory and four
  inference threads on the supported platform.
- **SC-008**: Unsupported-platform acceptance runs perform zero package
  acquisition and return one actionable in-editor result in 100% of cases.
- **SC-009**: A clean user can disable and remove the extension and its owned
  package data without a terminal, repository checkout or system package tool.
- **SC-010**: The published marketplace submission passes all repository
  quality, license, packaging and clean-install gates with no developer-only
  setup step in its user documentation.

## Assumptions

- The first release supports Linux `x86_64`; macOS, Windows, Linux `aarch64` and
  other platforms are deferred until each has its own published and validated
  package.
- A one-time automatic public download during installation or first use is
  compatible with plug-and-play. Translation content is never part of that
  download and later translation is offline.
- Zed supplies an extension-owned working area and installation status suitable
  for automatically managed local packages.
- The existing direct code action and read-only preview remain the user-facing
  translation interaction; a custom panel, clipboard mutation and automatic
  insertion are not required.
- English-to-Spanish is the only language direction in this release.
- The implementation may reuse prior prototype work only when it reduces the
  published user path; prototype lifecycle complexity is not a requirement.
- If the previously evaluated runtime or model cannot be distributed with clear
  obligations, planning selects a smaller distributable candidate rather than
  transferring that ambiguity to users.

## Scope Boundaries

### In Scope

- Extension Gallery installation and marketplace submission.
- Automatic first-use local package preparation with in-Zed status and retry.
- Real direct English-to-Spanish translation on Linux `x86_64`.
- Offline operation after preparation, safe update and no-terminal removal.
- Publication licenses, notices, integrity and clean-install acceptance.

### Out of Scope

- Manual `make`, Docker, binary-path or provider configuration as user steps.
- Agent Panel or MCP as the primary product experience.
- Accounts, subscriptions, API keys and mandatory remote translation.
- macOS, Windows, `aarch64`, additional languages and arbitrary model selection.
- Automatic edits, clipboard writes, custom panels and source-file mutation.
- Host-global packages, services, runtimes or privileged installation.
