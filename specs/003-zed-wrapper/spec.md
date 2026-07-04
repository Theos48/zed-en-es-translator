# Feature Specification: Zed Wrapper

**Feature Branch**: `003-zed-wrapper`

**Created**: 2026-07-03

**Status**: Implemented - Manual Zed smoke passed; missing-artifact fast-fail rescheduled

**Input**: User description: "Promote F006 from docs/feature-map.md: install and start the existing MCP translation server from a Zed development extension, with an extension manifest, reproducible local build, useful redacted logs, a minimal allowlisted environment, and controlled server command, arguments, and variables. Keep real providers, network access, publication, advanced editor UX, and automatic buffer edits out of scope."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Start Translation From Zed (Priority: P1)

As a Zed user, I can install the project as a local development extension and
have Zed start the existing translation service for me, so I do not need to run
the server manually in a separate shell before requesting translation.

**Why this priority**: This is the smallest useful wrapper slice. It proves the
editor can own server startup while preserving the already validated offline MCP
translation behavior.

**Independent Test**: From a clean checkout with the already supported project
tooling, follow the local extension setup instructions, open Zed, invoke the
translator through the editor's MCP-capable flow, and verify a valid offline
translation result is returned without manually starting the server.

**Acceptance Scenarios**:

1. **Given** the project has been prepared for local development use, **When** a
   user registers the extension in Zed, **Then** Zed recognizes the extension as
   a local development extension without requiring global project runtimes or
   services.
2. **Given** the extension is registered in Zed, **When** the user invokes the
   translation service through the editor, **Then** the existing offline MCP
   server starts under extension control and returns the expected translation
   response.
3. **Given** the translation service returns a successful result, **When** the
   user reviews the original file or buffer, **Then** no source content has been
   modified automatically.

---

### User Story 2 - Reproduce The Development Package (Priority: P2)

As a maintainer, I can rebuild the local extension package from the repository
using documented project commands, so another contributor can validate the same
development setup without hidden machine state.

**Why this priority**: The wrapper is useful only if startup behavior is
repeatable. Reproducible preparation also keeps the host clean and avoids
undocumented global dependencies.

**Independent Test**: Remove generated development artifacts, rerun the
documented project preparation command, and verify the same extension metadata,
launch configuration, and local server artifact are produced or selected.

**Acceptance Scenarios**:

1. **Given** a clean checkout, **When** the maintainer runs the documented
   preparation workflow, **Then** all files required for local Zed development
   use are present and consistent with the repository state.
2. **Given** the preparation workflow is run more than once, **When** no source
   files changed, **Then** the resulting local extension setup remains stable and
   does not accumulate duplicate or conflicting generated state.
3. **Given** a contributor follows the quickstart, **When** they reach the
   validation step, **Then** they can confirm the exact translation server
   launched by the extension.

---

### User Story 3 - Diagnose Startup Safely (Priority: P3)

As a user or maintainer, when the extension cannot start the translation
service, I receive enough redacted diagnostic information to fix the setup
without exposing source text, translated text, secrets, or sensitive paths.

**Why this priority**: Startup failures are likely while packaging the editor
integration. Diagnostics must be useful, but the project constitution requires
that logs and errors remain privacy-safe.

**Independent Test**: Intentionally break common startup prerequisites, invoke
the extension from Zed, and verify the failure is visible, actionable, bounded in
time, and redacted.

**Acceptance Scenarios**:

1. **Given** the server artifact is missing or not prepared, **When** Zed tries
   to start the translation service, **Then** the user receives a clear redacted
   failure that identifies the missing preparation step.
2. **Given** the extension receives unexpected or unsafe environment data,
   **When** it starts the translation service, **Then** only documented
   allowlisted environment values are passed to the server.
3. **Given** startup or translation fails, **When** logs are inspected, **Then**
   they contain useful status, error category, and timing information without
   source text, translations, secrets, tokens, headers, or sensitive paths.

### Edge Cases

- Zed is not installed, cannot load local development extensions, or reports the
  extension manifest as invalid.
- The local server artifact is missing, stale, not executable, incompatible with
  the current checkout, or fails immediately after launch; the user receives a
  redacted corrective action and no translation work begins.
- The extension is registered more than once, started repeatedly, or restarted
  after a failed startup attempt; no duplicate generated state is created, and
  the next startup revalidates configuration and artifact state from scratch.
- Project paths contain spaces or characters that commonly break command
  invocation; the launch requirement still treats the configured artifact as one
  command value rather than shell-splitting it.
- Zed passes a larger environment than expected, including workspace variables,
  shell variables, tokens, or unrelated user configuration.
- The server writes warnings, panics, provider errors, or validation errors to
  diagnostics.
- A translation request asks for remote provider use or network behavior.
- A translation request succeeds or fails while an editor buffer is open with
  unsaved changes.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide the extension metadata required for Zed to
  load this project as a local development extension.
- **FR-002**: Users MUST be able to prepare the local extension using documented
  project commands without installing project-specific runtimes, services, or
  package managers globally on the host.
- **FR-003**: The extension MUST start the existing MCP translation service for
  local development use without requiring the user to manually run the server in
  a separate shell.
- **FR-004**: The extension MUST launch the translation service with documented
  command intent, arguments, working location, and allowlisted environment
  values.
- **FR-005**: The launched service MUST expose the existing translation behavior
  and tool contract from the completed MCP server feature without adding a new
  translation contract in this feature.
- **FR-006**: The default runtime behavior MUST remain offline and deterministic;
  this feature MUST NOT add a real provider, remote network call, provider
  selection flow, or publication workflow.
- **FR-007**: The system MUST NOT modify editor buffers, source files, or user
  selections automatically.
- **FR-008**: Startup success, startup failure, and service failure diagnostics
  MUST be useful for local troubleshooting and MUST be redacted according to the
  project constitution.
- **FR-009**: Logs, errors, and diagnostics MUST NOT contain source text,
  translated text, translatable segments, secrets, headers, tokens, environment
  dumps, or sensitive unredacted paths.
- **FR-010**: Unsafe or unexpected inherited environment values MUST be excluded
  from the launched service unless explicitly allowlisted for this feature.
- **FR-011**: The system MUST fail with an actionable redacted message when the
  server artifact is missing, stale, not executable, or cannot start. For this
  feature, "actionable" means the diagnostic includes both a stable error
  category/code and at least one concrete corrective action such as rerunning
  the preparation workflow, correcting `binary_path`, or reinstalling the local
  dev extension.
- **FR-012**: The local development setup MUST be repeatable; running the
  documented preparation workflow multiple times MUST NOT create duplicate or
  conflicting extension state.
- **FR-013**: The feature MUST include validation for successful startup,
  missing startup prerequisites, repeated preparation, environment minimization,
  log redaction, offline default behavior, and no automatic buffer or file
  mutation.

### Key Entities *(include if feature involves data)*

- **Zed Development Extension**: The local editor integration package that Zed
  can load from the repository during development.
- **Extension Metadata**: The user-visible identity and capability declaration
  that lets Zed recognize the local extension.
- **Server Launch Profile**: The documented startup intent for the translation
  service, including command purpose, arguments, working location, and allowed
  environment values.
- **Prepared Server Artifact**: The local translation service executable or
  equivalent runnable artifact selected by the project preparation workflow.
- **Redacted Diagnostic Event**: A startup or service status message that helps
  troubleshoot setup while excluding protected content and sensitive values.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A contributor can prepare and register the local Zed development
  extension from a clean checkout by following the feature quickstart in 10
  minutes or less after required project dependencies are available.
- **SC-002**: A Zed-driven translation request can start the translation service
  and receive a successful offline translation result without a manually started
  server process.
- **SC-003**: Re-running the documented preparation workflow twice without source
  changes leaves the local extension setup consistent and free of duplicate or
  conflicting generated state.
- **SC-004**: Startup failures for a missing or unusable server artifact become
  visible to the user within the Zed context-server initialization window
  (observed ~60 seconds in the current Zed WASM extension runtime, bounded by
  Zed's own MCP client initialize timeout) and include the next corrective
  action. A sub-15-second target remains a future goal, gated on a Zed
  extension API capability this feature does not currently have access to
  (see Status Notes).
- **SC-005**: Environment validation confirms the launched service receives only
  documented allowlisted values needed for this feature.
- **SC-006**: Log and diagnostic review for successful startup, failed startup,
  and failed translation confirms no source text, translated text, secrets,
  tokens, headers, environment dumps, or sensitive unredacted paths are exposed.
- **SC-007**: File and buffer mutation checks confirm that invoking translation
  through the extension leaves original editor content and files unchanged.
- **SC-008**: Remote provider or network translation attempts remain denied by
  default in this feature.

## Status Notes

- Manual Zed smoke is complete for the success path.
- Missing or unusable binary diagnostics are implemented with redacted failure
  categories and corrective actions, but the original 15-second manual
  visibility target remains rescheduled because the current Zed runtime surfaced
  a 60-second initialization timeout for the missing-artifact case.
- The feature remains acceptable within scope because offline-only behavior,
  redaction, environment minimization, and no-mutation guarantees are
  satisfied; the fast-fail timing gap is explicitly deferred for a future Zed
  API strategy or packaging approach.
- Offline-default denial of remote/provider behavior (FR-006, FR-013, SC-008)
  is validated end-to-end by `tests/integration/zed_extension_remote_denial.sh`
  against the real prepared `translator-mcp` artifact launched with the same
  command shape the wrapper uses, recorded in `quickstart.md`.
- SC-004's original 15-second target is formally amended (not just deferred)
  after confirming, via `zed_extension_api` 0.7.0 source, that no viable
  primitive exists for this feature to validate an out-of-worktree host path
  from inside the WASM sandbox: `std::fs::metadata` on `wasm32-wasip1` cannot
  reliably resolve arbitrary absolute host paths (no WASI preopen for them),
  and the crate's own `zed::process::Command::output()` host-process API
  reproduced the same Zed configuration-modal timeout as the earlier
  `/usr/bin/test` preflight attempt. Closing the gap to 15 seconds would
  require a Zed platform capability (e.g. a fast synchronous path-exists check
  or a shorter context-server initialize timeout) that is outside this
  feature's control; re-evaluate when `zed_extension_api` exposes one.

## Assumptions

- The previous MCP server feature is merged and remains the source of truth for
  translation tools, validation, offline behavior, limits, and error mapping.
- This feature targets local development use of a Zed extension, not marketplace
  publication or final distribution.
- The user already has or can install Zed outside this project workflow; this
  feature does not manage system-level Zed installation.
- Existing project commands remain the preferred preparation and validation
  entry points.
- The exact wrapper shape is a planning decision. If direct editor launch cannot
  satisfy startup, environment, and diagnostic requirements, planning may define
  a thin wrapper while preserving this specification's behavior.
