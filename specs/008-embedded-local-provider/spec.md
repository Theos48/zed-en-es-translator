# Feature Specification: Embedded Local Provider

**Feature Branch**: `008-embedded-local-provider`

**Created**: 2026-07-15

**Status**: Draft

**Input**: User description: "Promote F012 from `docs/feature-map.md`: deliver
the normal English-to-Spanish translation experience without an account, API
key, remote service, or user-visible Docker lifecycle; select a distributable
on-device runtime and model through reviewed license, provenance, integrity,
resource, update, and packaging gates; integrate the selected local path with
the CLI and direct Zed workflow while preserving all existing safety and
privacy boundaries."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Translate Locally Without a Managed Service (Priority: P1)

A developer can translate permitted English text into Spanish from the command
line or the direct Zed workflow without creating an account, supplying a key,
contacting a remote translation service, or starting and maintaining Docker or
another long-running provider service.

**Why this priority**: Removing external accounts and visible service
operations is the core product improvement over the existing local-provider
path while keeping translation private and offline.

**Independent Test**: From a prepared supported installation, disable network
access and translate public synthetic text and Markdown through both supported
surfaces; verify real Spanish output, local-only execution, preserved protected
regions, and unchanged source content.

**Acceptance Scenarios**:

1. **Given** the reviewed local artifacts are prepared and the embedded local
   path is explicitly selected, **When** the user translates permitted English
   text with networking disabled, **Then** a real Spanish translation is
   returned without a separately managed provider service.
2. **Given** the same prepared local artifacts, **When** equivalent synthetic
   input is translated through the command line and direct Zed workflow,
   **Then** both surfaces use the same local safety boundary and identify the
   operation as offline before execution.
3. **Given** permitted Markdown containing prose and protected regions,
   **When** local translation is requested, **Then** only permitted segments
   are translated, protected regions remain unchanged, and neither the file
   nor editor buffer is modified.
4. **Given** the embedded local path has not been explicitly selected or is not
   ready, **When** translation is requested without another explicit provider
   choice, **Then** deterministic mock behavior remains the safe default and no
   artifact download or network contact begins implicitly.

---

### User Story 2 - Prepare Reviewed Artifacts With Informed Consent (Priority: P2)

A developer can understand the source, license, size, storage location, network
need, and privacy effect of the selected runtime and language resources before
any optional first-time acquisition, then prepare only the reviewed artifacts
without administrator privileges or host-global installation.

**Why this priority**: A convenient local model is not safe to adopt unless its
rights, origin, integrity, and footprint are explicit before it reaches the
workstation.

**Independent Test**: Start from a clean checkout with no prepared embedded
artifacts, inspect the documented preparation disclosure, accept or reject the
operation, and verify that rejection changes nothing while acceptance prepares
only integrity-verified project- or user-scoped artifacts.

**Acceptance Scenarios**:

1. **Given** the selected artifacts are not present, **When** the user reviews
   preparation, **Then** the product discloses source, license, version,
   expected transfer and installed sizes, destination scope, and whether
   network access is required before asking for consent.
2. **Given** the user declines first-time acquisition, **When** preparation is
   cancelled, **Then** no artifact is downloaded or activated, existing local
   state remains unchanged, and mock translation remains available.
3. **Given** the user authorizes preparation, **When** an artifact is acquired
   or unpacked, **Then** its identity and integrity are verified before it can
   become active.
4. **Given** an artifact is incomplete, corrupt, substituted, incompatible, or
   outside the reviewed license/provenance boundary, **When** preparation
   validates it, **Then** activation is denied with a redacted actionable
   outcome and the last known-good state is preserved.

---

### User Story 3 - Update, Recover, and Remove the Local Path (Priority: P3)

A maintainer can inspect, verify, update, roll back, and completely remove the
embedded local provider through documented project commands without automatic
updates, administrator privileges, lost rollback capability, or deletion of
source files and unrelated user data.

**Why this priority**: On-device artifacts are part of the product supply chain
and need a reversible lifecycle, not a one-time opaque download.

**Independent Test**: Prepare a known-good version, stage a controlled invalid
update, verify that promotion is denied, restore the known-good version without
network access, and remove only the selected provider artifacts using the
documented explicit cleanup operation.

**Acceptance Scenarios**:

1. **Given** a known-good local installation, **When** status and verification
   are requested, **Then** they report only safe version, integrity, readiness,
   locality, and resource metadata without exposing content or sensitive host
   paths.
2. **Given** a reviewed update is available, **When** the maintainer explicitly
   authorizes it, **Then** the candidate is acquired and verified separately
   before it can replace the active version.
3. **Given** update verification or post-update translation fails, **When**
   recovery is invoked, **Then** the last known-good version is restored and
   can translate the synthetic acceptance sample without network access.
4. **Given** the maintainer explicitly requests complete removal, **When** the
   documented cleanup finishes, **Then** only provider-owned artifacts and
   metadata are removed and the repository, source files, editor buffers,
   unrelated caches, and global host state remain unchanged.

---

### User Story 4 - Review Product Fit and Publication Readiness (Priority: P4)

A reviewer can compare viable on-device candidates using consistent evidence,
understand the selected delivery strategy and resource envelope, and determine
whether the provider may be distributed or whether publication must remain
blocked.

**Why this priority**: A technically functional model is not a responsible
product dependency unless its language support, maintenance, legal basis,
integrity, footprint, and performance all pass explicit gates.

**Independent Test**: Review the candidate decision record, reproduce the
documented benchmark matrix on the target workstation, trace every active
artifact to its version, source, integrity value, and license, and confirm that
the publication conclusion follows the recorded evidence.

**Acceptance Scenarios**:

1. **Given** multiple plausible on-device candidates, **When** they are
   evaluated, **Then** each is assessed against the same English-to-Spanish
   capability, license, provenance, integrity, maintenance, size, memory, CPU,
   latency, preparation, update, and integration criteria.
2. **Given** one candidate satisfies every mandatory gate, **When** it is
   selected, **Then** the decision records why it is suitable and why the
   alternatives were rejected or deferred.
3. **Given** no candidate has sufficient distribution rights, verifiable
   provenance, acceptable operation, or a safe update path, **When** the review
   concludes, **Then** no unsafe provider is shipped, the existing safe paths
   remain available, and publication stays explicitly blocked.
4. **Given** a candidate is approved for one delivery strategy only, **When**
   release readiness is assessed, **Then** the project makes no broader
   bundling, redistribution, or licensing claim than the evidence supports.

### Edge Cases

- The first preparation is attempted without network access when the selected
  strategy requires an explicitly authorized download.
- The user cancels acquisition after partial transfer, or the process is
  interrupted during download, verification, unpacking, or activation.
- A bundled or acquired artifact has a valid file hash but mismatched runtime,
  model, language pair, architecture, version, or manifest identity.
- The artifact source, license text, published integrity metadata, or upstream
  maintenance status changes after selection.
- The active artifact disappears, becomes unreadable, is corrupt, or exceeds
  the documented resource envelope between readiness and translation.
- A candidate returns empty, malformed, reordered, mismatched, non-textual, or
  oversized output, or does not finish within the existing timeout.
- An update succeeds technically but fails the synthetic English-to-Spanish
  quality check, offline check, resource budget, or cross-surface validation.
- Rollback metadata exists but its referenced artifact is missing, corrupt, or
  incompatible; recovery must not destroy the remaining known-good state.
- Two preparation or update operations are requested concurrently.
- The storage destination resolves through a link or path outside the approved
  project/user scope, or cleanup encounters unrelated files.
- Network access becomes available during normal translation after preparation;
  the embedded path must neither require nor attempt external contact.
- Diagnostics or evidence encounter source text, translation output, model
  content, sensitive paths, environment values, or identifiers that must not
  be recorded.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The product MUST deliver exactly one supported embedded local
  English-to-Spanish provider path in this feature, unless every evaluated
  candidate fails a mandatory safety, legal, provenance, integrity, resource,
  or operational gate.
- **FR-002**: Supported embedded translation MUST require no external account,
  subscription, API key, remote translation service, user-managed Docker
  lifecycle, or separately managed long-running provider service.
- **FR-003**: Deterministic mock behavior MUST remain the default when no real
  provider is explicitly selected and MUST remain available when embedded
  artifacts are absent, rejected, unavailable, or removed.
- **FR-004**: Before selection, candidate runtimes and language resources MUST
  be evaluated consistently for real English-to-Spanish support, license and
  distribution rights, provenance, integrity metadata, maintenance status,
  supported host architecture, transfer and installed size, peak memory and
  CPU use, preparation time, translation latency, update strategy, rollback,
  removal, and integration with both supported user surfaces.
- **FR-005**: The selection record MUST identify the delivery strategy for each
  artifact class—bundled, explicitly downloaded, or externally acquired into
  an approved local scope—and MUST state the rights, consent, network, storage,
  integrity, update, and publication implications of the resulting combination.
- **FR-006**: Every active runtime, model, tokenizer, vocabulary, configuration,
  or other required artifact MUST have a recorded version, reviewed source,
  license conclusion, expected identity, and integrity value appropriate to
  its delivery strategy.
- **FR-007**: The project MUST NOT bundle, vendor, publish, redistribute, or
  imply redistribution rights for any artifact whose applicable license or
  provenance is missing, ambiguous, incompatible, or unsupported by the
  recorded review.
- **FR-008**: Any first-time network acquisition MUST require explicit informed
  consent after disclosing artifact purpose, source, license, expected transfer
  and installed sizes, destination scope, and network use; cancellation MUST
  leave the active state unchanged.
- **FR-009**: Acquired, unpacked, or bundled artifacts MUST pass identity,
  integrity, compatibility, and language-pair validation before activation;
  partial or failed preparation MUST NOT become active.
- **FR-010**: All provider runtime and model state MUST remain project- or
  user-scoped and MUST NOT require administrator privileges, host-global
  runtime/package installation, system-service changes, or writes to unrelated
  host locations.
- **FR-011**: After any authorized preparation, 100% of supported embedded
  translation, readiness, verification, and rollback operations MUST complete
  with external networking disabled and MUST NOT attempt remote contact.
- **FR-012**: The command-line and direct Zed workflows MUST use the same
  provider-selection, safety, segmentation, limit, response-validation,
  redaction, and normalized-error boundaries for the embedded local path.
- **FR-013**: Before an embedded request executes, the direct Zed workflow MUST
  identify it as offline/local without exposing model names, artifact URLs,
  executable paths, sensitive storage paths, or environment details.
- **FR-014**: The embedded path MUST preserve the existing 20 KiB input, 4 KiB
  segment, 256-segment, 40 KiB output, and 15-second provider timeout limits,
  plus existing Markdown protection, supported-file validation, secret
  detection, and ambiguity-preservation behavior.
- **FR-015**: Embedded translation MUST NOT modify editor buffers, write source
  files, place translated content on the clipboard, or make Agent Panel the
  primary workflow.
- **FR-016**: The project MUST provide documented, repeatable operations for
  disclosure and preparation, safe status, readiness verification, explicit
  update, rollback to a last known-good version, and complete provider-artifact
  removal without requiring the user to operate a provider service.
- **FR-017**: Updates MUST be user-initiated, reviewed, and verified separately
  from the active version; automatic model/runtime updates and in-place
  replacement without a recoverable known-good state are prohibited.
- **FR-018**: Preparation, verification, update, rollback, and removal MUST be
  idempotent or fail safely, serialize conflicting lifecycle operations, and
  preserve both the active state and unrelated data when completion cannot be
  proven.
- **FR-019**: Missing, corrupt, incompatible, unlicensed, mis-scoped,
  resource-exhausting, unavailable, timed-out, or invalid-response provider
  states MUST fail closed with stable actionable errors and MUST NOT silently
  fall back from a requested real translation to fabricated success.
- **FR-020**: Logs, diagnostics, standard error, lifecycle output, benchmark
  records, and validation evidence MUST NOT contain source text, translated
  text, permitted segments, model data, raw provider output, environment
  contents, secrets, sensitive URLs, workspace roots, or sensitive host paths.
- **FR-021**: Automated checks MUST cover selection/configuration, mock default,
  consent and cancellation, integrity and compatibility failures, partial
  preparation, offline operation, lifecycle concurrency, update isolation,
  rollback, resource/timeout enforcement, invalid responses, cross-surface
  consistency, redaction, and non-mutation using controlled artifacts and
  doubles where appropriate.
- **FR-022**: Real acceptance MUST use public synthetic English-to-Spanish
  inputs with the selected on-device artifacts through both the command line
  and direct Zed while external networking is disabled; test doubles alone
  MUST NOT close the embedded path.
- **FR-023**: The candidate and acceptance evidence MUST record only safe
  artifact identities, license/provenance conclusions, delivery strategy,
  measured transfer and installed sizes, peak memory and CPU, preparation and
  translation timings, surface, locality, synthetic case identifier,
  normalized outcome, and reviewer result.
- **FR-024**: Documentation MUST explain prerequisites, supported host scope,
  artifact disclosure and consent, storage, privacy boundary, resource
  envelope, preparation, offline use, update, rollback, failure recovery,
  validation, complete removal, and publication limitations.
- **FR-025**: If no candidate passes every mandatory gate, the feature MUST
  preserve current mock and Docker-based local paths, publish the blocking
  evidence, and leave embedded-provider and publication claims incomplete.
- **FR-026**: This feature MUST NOT expand remote providers, add language pairs,
  introduce a new Agent Panel product flow, weaken existing compatibility
  regressions, or change the constitutional prohibition on source/buffer
  mutation.
- **FR-027**: Local activation MUST require a recorded artifact-level review by
  a human project maintainer, while any bundling, redistribution, or publication
  approval MUST be a separate explicitly recorded human decision in F009; an
  absent, anonymous, automated-only, or scope-mismatched approval MUST leave the
  corresponding activation or publication state blocked.
- **FR-SEC-A**: As the security traceability alias of FR-015, the product MUST
  NOT modify editor buffers unless a later constitution amendment allows it.
- **FR-SEC-B**: The product MUST reject unsafe file paths, unsupported file
  types, non-UTF-8 input, and binary content.
- **FR-SEC-C**: Remote provider use remains denied unless explicitly configured
  and confirmed per request; the embedded path MUST NOT weaken that separate
  boundary.
- **FR-SEC-D**: As the security traceability alias of FR-020, the product MUST
  NOT log source text, translated text, segments, secrets, headers, tokens, or
  sensitive paths.
- **FR-TEST-A**: As the test traceability alias of FR-021, the product MUST
  define testable acceptance criteria and negative checks before implementation.

### Key Entities

- **Embedded Provider Candidate**: A possible on-device runtime and
  English-to-Spanish language-resource set with version, platform support,
  license/provenance findings, integrity sources, maintenance status, delivery
  options, resource measurements, lifecycle design, and gate result.
- **Artifact Manifest**: The reviewed identity of every required runtime,
  model, tokenizer, vocabulary, configuration, or supporting artifact,
  including version, source, license conclusion, expected integrity value,
  compatibility, language pair, size, and permitted delivery strategy.
- **Prepared Local Installation**: The approved project- or user-scoped active
  artifact set, readiness and integrity state, delivery consent reference,
  resource envelope, active version, and known-good recovery reference; it
  contains no translation content or secrets.
- **Lifecycle Candidate**: A separately prepared proposed update with review,
  consent, identity, integrity, compatibility, benchmark, offline, and
  translation-validation outcomes before promotion.
- **Rollback Point**: The last known-good artifact-manifest and installation
  reference that can be restored and verified offline without source content,
  translations, credentials, or sensitive paths.
- **Benchmark Record**: A redacted measurement set for target platform,
  transfer and installed sizes, peak memory and CPU, first preparation,
  readiness, and repeated translation latency using public synthetic cases.
- **Validation Record**: A redacted result for one supported surface and
  synthetic case, including safe artifact identity, locality, timestamp,
  normalized outcome, non-mutation result, offline result, and reviewer status.

### Scope Boundaries

In scope:

- evaluating viable on-device English-to-Spanish candidates and selecting one
  only if every mandatory gate passes;
- implementing one embedded local path without a user-managed provider service;
- a reviewed bundled, consented-download, or approved local-acquisition
  strategy with integrity verification and scoped storage;
- command-line and direct Zed use, real offline evidence, resource benchmarks,
  reversible updates, rollback, removal, and publication-gate documentation;
- preserving mock default/fallback, existing translation safety, privacy,
  limits, redaction, non-mutation, and compatibility regressions.

Out of scope:

- publishing the extension or declaring F009 complete;
- assuming or manufacturing model/runtime distribution rights;
- adding remote-provider functionality, accounts, API keys, paid services,
  arbitrary endpoints, automatic discovery, or additional language pairs;
- requiring Docker or another long-running provider service for normal embedded
  use, or installing runtimes, packages, databases, or services globally;
- changing the direct Zed preview surface, adding clipboard/apply/replace
  behavior, or making Agent Panel the primary experience;
- translating unsupported full source-code files, weakening secret detection,
  changing existing limits, or storing translation content in artifacts or
  evidence.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reviewer completes 2 real embedded-provider acceptance runs—one
  through the command line and one through direct Zed—using public synthetic
  English input and receiving valid non-mock Spanish output with source content
  unchanged.
- **SC-002**: After authorized preparation, 100% of the embedded-provider
  acceptance, readiness, update-verification, and rollback scenarios complete
  with external networking disabled and zero attempted remote contacts.
- **SC-003**: Across at least 20 representative public synthetic translations
  on the target workstation, 100% finish within the existing 15-second timeout,
  preserve protected content and limits, and produce no buffer or file changes.
- **SC-004**: The plan fixes a reproducible method and acceptance budget for
  100% of the required license, provenance, integrity, maintenance, delivery,
  transfer size, installed size, peak memory, peak CPU, first-preparation,
  readiness, and repeated-translation evidence; no mandatory measured field
  remains unknown before the selected path is promoted or the feature closes.
- **SC-005**: A first-time user can review and either reject or authorize
  preparation through one documented flow without administrator privileges;
  rejection produces zero active artifacts and acceptance activates only a
  fully verified set.
- **SC-006**: A controlled invalid update changes zero active artifact
  references, and a controlled post-promotion failure can be rolled back to a
  verified synthetic translation within 5 minutes without network access.
- **SC-007**: Automated negative coverage passes for 100% of the failure classes
  named in FR-021, and inspection of logs, lifecycle output, benchmark records,
  and evidence finds zero prohibited content disclosures.
- **SC-008**: Every active artifact has a traceable version, reviewed source,
  license conclusion, integrity value, and delivery authorization; any artifact
  lacking one of these fields is prevented from activation and distribution.
- **SC-009**: Complete removal leaves zero provider-owned active artifacts or
  lifecycle metadata while changing zero repository source files, editor
  buffers, unrelated user data, global packages, or system services.

## Assumptions

- The target acceptance workstation remains Fedora KDE on `x86_64`; support for
  other operating systems or architectures requires separate evidence and is
  not implied by this feature.
- English-to-Spanish remains the only supported language pair and the existing
  technical-neutral translation behavior remains unchanged.
- The existing command-line and direct Zed read-only preview surfaces are the
  acceptance surfaces; MCP/Agent Panel remains compatibility infrastructure.
- Network access may be used only during an explicitly authorized preparation
  or update when the selected delivery strategy requires acquisition. Normal
  translation, verification, and rollback remain offline.
- Existing project-scoped development/build isolation may continue unchanged;
  the no-Docker requirement applies to the end-user provider lifecycle and
  normal translation path, not to the repository's reproducible build tests.
- The existing Docker-based LibreTranslate path remains available until the
  embedded path passes all gates and is not removed by this feature.
- Publication remains a separate F009 cycle even if the selected artifacts have
  compatible distribution rights; this feature supplies evidence but does not
  publish a release. The project maintainer may approve reviewed local
  acquisition for F012, but only the separate human F009 publication review may
  approve bundling or redistribution.
