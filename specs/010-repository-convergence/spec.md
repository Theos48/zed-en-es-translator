# Feature Specification: Repository Convergence and Cleanup

**Feature Branch**: `010-repository-convergence`

**Created**: 2026-07-16

**Status**: Complete

**Input**: User description: "Remove obsolete code, MCP and other files that no
longer serve the product objective; align the roadmap; leave only what the
plug-and-play Zed Gallery extension needs; and plan the work so subagents can
execute it efficiently."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Maintain One Product Architecture (Priority: P1)

A maintainer can understand and change the translator through one supported
architecture: the Zed Gallery extension prepares a verified local package,
launches the direct read-only translation flow and performs English-to-Spanish
translation locally. Historical MCP, Agent, command-line and configurable
provider paths do not compete with that architecture.

**Why this priority**: Multiple obsolete product paths increase dependencies,
security review surface and release risk without helping the supported user.

**Independent Test**: Inspect every executable entry point and supported
maintainer command and establish that each is required by the Gallery product,
its release process or its safety validation.

**Acceptance Scenarios**:

1. **Given** the cleaned repository, **When** a maintainer traces the installed
   extension, **Then** there is exactly one product path from Zed to the local
   translation runtime.
2. **Given** the cleaned repository, **When** unsupported compatibility paths
   are searched, **Then** MCP, Agent Panel, standalone CLI, LibreTranslate and
   Azure are absent from executable and supported operational surfaces.
3. **Given** the cleaned repository, **When** the Gallery package is built,
   **Then** it preserves the existing direct, offline and read-only behavior.

---

### User Story 2 - Work With Focused Automation (Priority: P2)

A maintainer sees only build, test, release and cleanup commands that exercise
the retained product. Default validation does not spend time compiling or
testing retired compatibility paths, while all active security and publication
gates remain available.

**Why this priority**: Automation is the practical interface to the project;
obsolete targets make normal maintenance slower and misleading.

**Independent Test**: Enumerate every supported command, continuous-integration
gate, dependency and fixture and map each one to a retained requirement.

**Acceptance Scenarios**:

1. **Given** the cleaned automation, **When** the full project-controlled gate
   runs, **Then** it validates only retained code plus the extension and release
   package.
2. **Given** a removed component, **When** build metadata and dependency locks
   are inspected, **Then** no orphaned member, dependency, target, script, test
   or fixture remains.
3. **Given** a fresh maintainer checkout, **When** project help is displayed,
   **Then** every advertised command is current and necessary.

---

### User Story 3 - Read Current Documentation Without Losing History (Priority: P3)

A contributor lands on a concise product README and roadmap that describe the
Gallery objective and remaining release work. Superseded architectural choices
remain discoverable as clearly marked decision history, but old tutorials and
completed feature work do not remain actionable in the working tree.

**Why this priority**: Stale instructions can reintroduce the very surfaces the
cleanup removes, while deleting all decision history would make future review
harder.

**Independent Test**: Follow every documentation link and distinguish current
instructions, future roadmap and superseded decisions without consulting
untracked knowledge.

**Acceptance Scenarios**:

1. **Given** a new contributor, **When** they read the README and roadmap,
   **Then** they see the Gallery installation objective and its open release
   gates before historical implementation detail.
2. **Given** a superseded decision, **When** it is opened, **Then** its status
   and replacing decision are explicit and it cannot be mistaken for current
   guidance.
3. **Given** completed Spec Kit cycles removed from the working tree, **When**
   history is needed, **Then** Git and retained decision records provide the
   archive while all still-live requirements exist in the active features.

---

### User Story 4 - Remove Generated Residue Safely (Priority: P4)

A maintainer can remove stale build output, caches and old generated extension
artifacts without touching source, agent configuration, locked release inputs
or user data.

**Why this priority**: The audited checkout contains tens of gigabytes of
generated output and an old extension artifact that can mislead manual tests.

**Independent Test**: Preview and run the supported cleanup boundary, then
classify every remaining ignored path as an intentional reusable input or
non-project state.

**Acceptance Scenarios**:

1. **Given** generated outputs from old and current components, **When** normal
   cleanup runs, **Then** build outputs and stale generated extension binaries
   are removed from every project workspace.
2. **Given** locked native source inputs needed for a pending release, **When**
   normal cleanup runs, **Then** they are preserved unless the maintainer asks
   for the deeper cleanup tier explicitly.
3. **Given** local agent/Spec Kit configuration or persistent user data, **When**
   cleanup runs, **Then** it is never selected for deletion.

### Edge Cases

- A file appears historical but is read at compile time or by an active test.
- A removed dependency is still present transitively for a retained component.
- A security test is named after a retired surface but covers a live invariant.
- A completed feature artifact is the only copy of a live requirement or
  contract.
- A documentation link crosses from a retained ADR into a removed spec.
- Cleanup changes the release binaries and invalidates locked sizes or hashes
  before the public tag exists.
- A public tag or Gallery submission appears while cleanup is in progress.
- Ignored output belongs to another worktree or an active build process.
- A dirty worktree contains user changes in a removal candidate.
- A broad cleanup preview includes `.agents/`, `.codex/`, locked native source
  inputs, provider data or any unrecognized persistent path.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST define one explicit allowlist of retained
  product, release, validation and documentation surfaces.
- **FR-002**: The retained product MUST continue to provide automatic Gallery
  preparation, real local English-to-Spanish translation, offline operation
  after preparation, read-only previews, input limits, Markdown preservation,
  path safety, redacted diagnostics and Zed-owned removal.
- **FR-003**: MCP, Agent Panel and context-server code, contracts, preparation
  paths, tests and current-use documentation MUST be removed from the supported
  repository surface.
- **FR-004**: The standalone translation CLI, LibreTranslate, Azure and their
  configurable or manual provider lifecycles MUST be removed; a deterministic
  mock MAY remain only as a test double for retained behavior.
- **FR-005**: The shipped language-server path MUST select only the verified
  adjacent local package and MUST accept no user provider, URL, credential,
  remote-confirmation or arbitrary binary setting.
- **FR-006**: Build metadata, dependency locks, continuous integration,
  maintainer commands, scripts, tests and fixtures MUST contain no orphaned
  reference to a removed surface.
- **FR-007**: Completed pre-009 Spec Kit directories MUST leave the working tree
  only after every still-live requirement, contract and validation invariant is
  migrated into the active feature or retained decision documentation; Git is
  the archive for their full historical contents.
- **FR-008**: Stable ADR and decision history MUST be retained and marked
  accepted, partially superseded or superseded with an explicit replacement;
  historical records MUST NOT present executable setup instructions as current.
- **FR-009**: The README, project plan, feature map and architecture diagrams
  MUST describe the Gallery product, current publication sequence and future
  work consistently, without deleting the detailed future-feature map required
  by project policy.
- **FR-010**: The cleanup MUST occur before the first public release tag and
  MUST regenerate any package identity and validation evidence changed by the
  retained-source edits.
- **FR-011**: Constitutional language that requires retired CLI/MCP/provider
  boundaries MUST be amended before their implementation is removed, while
  preserving safety-first, offline-first, test-first, explicit-limit and clean
  host principles.
- **FR-012**: Each removal wave MUST start with a failing or explicit negative
  contract for the desired repository boundary and MUST preserve equivalent
  live security coverage before deleting old tests.
- **FR-013**: No retained document, build target, test, source file or release
  artifact MAY contain a broken reference to a removed path.
- **FR-014**: Project cleanup MUST provide a previewable normal tier for build
  output and a separate explicit deep tier for reproducible caches; neither tier
  may target source, agent configuration, secrets or persistent user data.
- **FR-015**: Existing dirty worktree changes and unrecognized ignored paths
  MUST be preserved and reviewed rather than deleted automatically.
- **FR-016**: Implementation MUST use parallel subagents with disjoint ownership
  for source/dependencies, automation/tests and documentation/traceability, with
  one coordinating agent responsible for integration and final gates.
- **FR-017**: The final repository MUST expose a complete rollback path through
  reviewable version-control commits and MUST NOT depend on an untracked backup
  or archive directory.
- **FR-SEC-A**: The system MUST NOT modify editor buffers or source files.
- **FR-SEC-B**: The system MUST reject unsafe file paths, unsupported file
  types, non-UTF-8 input and binary content before translation.
- **FR-SEC-C**: Translation MUST remain local and offline after fixed public
  package acquisition; removed remote-provider code MUST NOT be replaced by a
  different network translation path.
- **FR-SEC-D**: The system MUST NOT log source text, translated text, segments,
  secrets, headers, tokens, raw child output or sensitive paths.
- **FR-TEST-A**: Retained behavior and repository-boundary changes MUST have
  testable acceptance and negative coverage before implementation.

### Key Entities

- **Retained Surface**: A source, release, validation or documentation area with
  a named consumer in the supported Gallery product.
- **Removal Candidate**: A tracked or generated path whose consumers are absent,
  retired or migrated and whose deletion gates are recorded.
- **Historical Record**: A retained ADR or decision entry that explains a past
  choice without acting as current operational guidance.
- **Validation Invariant**: A safety, privacy, packaging or user-behavior rule
  that must survive even when its original test surface is removed.
- **Cleanup Tier**: A bounded set of generated paths eligible for normal or deep
  cleanup, with an explicit preservation allowlist.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every retained executable entry point maps to the single Gallery
  product path or its release validation, and zero retired product entry points
  remain.
- **SC-002**: The root product workspace decreases from four members to the two
  retained runtime members, with zero direct dependencies used only by MCP,
  Agent, CLI, LibreTranslate or Azure.
- **SC-003**: At least 150 audited obsolete tracked files leave the working tree
  without losing any requirement or validation invariant mapped to the active
  product.
- **SC-004**: Searches of current source, automation and user-facing
  documentation find zero MCP, Agent Panel, configurable-provider or manual
  provider instructions outside the explicit superseded-history allowlist.
- **SC-005**: One hundred percent of retained documentation links resolve, and
  every retained ADR/decision has a coherent current status.
- **SC-006**: All project-controlled format, lint, dependency, unit,
  integration, acquisition, package, offline, privacy, license and removal gates
  pass after cleanup.
- **SC-007**: Three independent marketplace-shaped translations still return
  real non-mock output with source bytes unchanged and no manual setting.
- **SC-008**: The normal cleanup tier removes all audited build outputs and the
  stale generated extension artifact while preserving every allowlisted source
  cache, agent file and persistent data path.
- **SC-009**: Any changed release binary, archive, size or hash is regenerated
  and recorded before the public tag, exact-package interactive test and Gallery
  submission continue.

## Assumptions

- The supported product remains the Linux `x86_64`, English-to-Spanish,
  plug-and-play Zed Gallery extension defined by feature 009.
- MCP, Agent Panel, standalone CLI, LibreTranslate and Azure have no supported
  consumer outside this repository and are intentionally retired rather than
  deprecated for a later compatibility release.
- Git history and retained ADRs are sufficient archival storage for removed
  completed Spec Kit cycles; no duplicate `archive/` tree is created.
- The active 009 feature and this 010 feature retain all current operational
  requirements; historical feature-map detail remains because project policy
  requires it as future planning context.
- Cleanup runs before `v0.1.0` is tagged, so changed package identities can be
  regenerated without breaking a published immutable release.
- Generated-output deletion is a separate final action after source cleanup and
  release-candidate verification, not a prerequisite for planning.
