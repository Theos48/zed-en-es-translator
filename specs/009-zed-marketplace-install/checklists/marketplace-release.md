# Marketplace Release Requirements Checklist: Plug-and-Play Zed Marketplace Installation

**Purpose**: Review whether requirements are complete, clear, consistent and
measurable for the zero-setup Gallery journey, executable/model supply chain,
offline/privacy boundary, recovery, removal and publication. This checklist
evaluates the requirements themselves, not implementation behavior.

**Created**: 2026-07-16
**Feature**: [spec.md](../spec.md)
**Depth / Audience / Timing**: Formal release gate / PR reviewer / pre-tasks and publication review

## Requirement Completeness

- [x] CHK001 Are the supported user, platform, language direction, document
  types and read-only output surface all defined without implying support for
  deferred platforms or languages? [Completeness, Spec US1, US4, FR-002,
  FR-004; Scope Boundaries]
- [x] CHK002 Is the primary journey complete from Gallery discovery through
  first real translation, including every setup step that is explicitly
  forbidden? [Completeness, Spec US1, FR-001, FR-003, FR-005]
- [x] CHK003 Are the boundaries between the marketplace product, historical
  MCP/Agent paths and developer-configurable providers stated so none can
  silently become a prerequisite? [Completeness, Spec FR-010, FR-025; Scope
  Boundaries]
- [x] CHK004 Are requirements defined for absent, checking, downloading,
  ready, retryable failure, unsupported and last-known-good states?
  [Completeness, Spec US2, US4, FR-006, FR-012, FR-013, FR-017; Data Model]
- [x] CHK005 Does the package identity requirement enumerate runtime,
  language server, model, vocabulary, lexical shortlist, platform, sizes,
  hashes, sources, compatibility and license/notice evidence? [Completeness,
  Spec FR-007, FR-011, FR-023; Package Lock Schema]
- [x] CHK006 Are publication requirements complete for the extension manifest,
  accepted extension license, public release, central registry entry, upstream
  checks and post-registry acceptance? [Completeness, Spec FR-001, FR-026,
  FR-027, SC-010; Publication Contract]

## Requirement Clarity

- [x] CHK007 Is “plug-and-play” made objective by the zero-command,
  zero-setting, zero-account and no-checkout criteria rather than left as a
  subjective adjective? [Clarity, Spec FR-003, SC-001]
- [x] CHK008 Is “real translation” distinguished from Mock output and tied to
  an observable English-to-Spanish result in the clean-install acceptance?
  [Clarity, Spec US1, FR-002, FR-010, SC-001]
- [x] CHK009 Is the one-time network boundary distinguished clearly from
  offline translation, including what data may and may not enter acquisition
  requests? [Clarity, Spec FR-008, FR-009, FR-SEC-C; Assumptions]
- [x] CHK010 Is “verified before use” defined with fixed identity, source,
  byte counts, hashes, compatibility and license rather than hash-only trust?
  [Clarity, Spec FR-007, FR-011; Acquisition Contract]
- [x] CHK011 Is “extension-owned storage” mapped unambiguously to Zed's work
  directory and contrasted with workspace, independent XDG and host-global
  state? [Clarity, Spec FR-015, FR-016; Data Model; Research Decision 2]
- [x] CHK012 Is the visible ready state clarified despite the Zed API lacking
  a separate Ready installation-status value? [Clarity, Spec FR-006; Research
  Decision 5]
- [x] CHK013 Are package/current/previous/staging terms canonical and defined
  consistently enough to prevent partial or invalid content from being called
  installed? [Clarity, Data Model State Transitions; Translation Package]

## Requirement Consistency

- [x] CHK014 Does automatic acquisition remain consistent with offline-first
  privacy by keeping all document content out of downloads and all network
  capability out of inference? [Consistency, Spec FR-005, FR-008, FR-009,
  FR-022, FR-SEC-C; Research Decision 4]
- [x] CHK015 Does the local marketplace default remain consistent with the
  constitution's offline/mock default while leaving remote providers
  explicitly configured and default-deny? [Consistency, Spec FR-010, FR-025,
  FR-SEC-C; Plan Constitution Check]
- [x] CHK016 Are no-mutation, Markdown preservation and unsafe-input rejection
  requirements consistent across clean install, retry, offline and update
  scenarios? [Consistency, Spec FR-019, FR-020, FR-SEC-A, FR-SEC-B; SC-004]
- [x] CHK017 Are the 20 KiB, 4 KiB, 256-segment, 40 KiB and 15-second limits
  identical across product requirements, package runner contract and success
  criteria? [Consistency, Spec FR-018; Translation Package; Plan Constraints]
- [x] CHK018 Is the no-terminal removal requirement consistent with the single
  extension-work storage boundary and Zed-owned uninstall behavior?
  [Consistency, Spec FR-015, FR-016, SC-009; Research Decision 2]
- [x] CHK019 Are the package-size, lifecycle-storage, memory and thread budgets
  consistent between measurable outcomes and the package contract?
  [Consistency, Spec SC-007; Data Model Package Budgets; Package Lock Schema]

## Acceptance Criteria Quality

- [x] CHK020 Can clean installation success be objectively measured without a
  development extension, repository binary or hidden manual preparation?
  [Measurability, Spec US1 Independent Test, FR-026, SC-001]
- [x] CHK021 Are first-preparation timing requirements quantified with network
  class, success percentile, upper bound and storage precondition?
  [Measurability, Spec SC-002]
- [x] CHK022 Are offline translation criteria quantified by case count,
  per-request deadline and network condition? [Measurability, Spec SC-003]
- [x] CHK023 Are non-mutation and protected-structure outcomes stated as
  byte-level and complete-set measures rather than subjective review?
  [Measurability, Spec SC-004]
- [x] CHK024 Are privacy outcomes measurable across network requests, logs and
  user diagnostics without requiring private input evidence? [Measurability,
  Spec SC-006, FR-021]
- [x] CHK025 Is publication completion distinguishable from implementation
  completion by requiring an actual central-registry submission and a clean
  Gallery acquisition? [Measurability, Spec FR-027, SC-010; Publication
  Contract]

## Scenario and Recovery Coverage

- [x] CHK026 Are primary, alternate, exception, recovery and non-functional
  scenario classes all represented by independently testable user stories or
  acceptance criteria? [Coverage, Spec US1-US4, Edge Cases]
- [x] CHK027 Are pre-first-use offline, mid-download interruption, corruption,
  oversize, insufficient storage and unavailable source outcomes all specified
  without manual cleanup? [Coverage, Spec US2, Edge Cases, FR-011, FR-012]
- [x] CHK028 Are concurrent Zed-window requirements defined for serialization,
  safe duplicate work and prohibition of partial activation? [Coverage, Spec
  Edge Cases, FR-014; Acquisition Contract]
- [x] CHK029 Is restart recovery specified for interruption before promotion
  and for a ready package when external networking remains disabled?
  [Coverage, Spec US2-AS3, US3-AS1, Edge Cases, FR-009, FR-012]
- [x] CHK030 Is failed-update behavior complete for a valid current package,
  an invalid current package and an invalid candidate? [Coverage, Spec US2-AS4,
  FR-011, FR-013; Acquisition Failure Contract]
- [x] CHK031 Are disable and uninstall requirements defined for both idle and
  active translator processes without implying that the extension owns Zed's
  process manager? [Coverage, Spec US3-AS3, US3-AS4, Edge Cases, FR-016]
- [x] CHK032 Are unsupported operating-system and processor combinations
  covered before any package request or misleading configuration guidance?
  [Coverage, Spec US4, FR-017, SC-008]

## Security, Privacy and Supply-Chain Requirements

- [x] CHK033 Are allowed acquisition hosts, redirect behavior, registry
  discovery, proxy inheritance and alternate-source fallback bounded clearly
  enough to review the network surface? [Security, Spec FR-007, FR-008;
  Acquisition Contract]
- [x] CHK034 Are archive path, link, file type, executable allowlist and
  installed-object validation requirements documented for untrusted package
  bytes? [Security, Translation Package Launchability; Package Lock Schema]
- [x] CHK035 Are child-process command, arguments, working directory,
  environment, I/O caps, timeout and kill/reap requirements complete without
  exposing raw output? [Security, Spec FR-021, FR-022; Translation Package]
- [x] CHK036 Are extension-code, native dependency and model attribution
  obligations distinguished by delivery scope and tied to exact published
  contents? [Compliance, Spec FR-023, FR-024; Research Decision 7; Publication
  Contract]
- [x] CHK037 Is the rule for replacing a runtime/model candidate with
  insufficient distribution evidence explicit, preventing a license prompt
  from becoming user setup? [Compliance, Spec FR-024; Assumptions]
- [x] CHK038 Are log and diagnostic prohibitions complete for source,
  translation, segments, secrets, headers, tokens, child output and sensitive
  paths while still allowing content-free actionable states? [Privacy, Spec
  FR-021, FR-SEC-D; Acquisition Failure Contract]

## Dependencies, Assumptions and Boundaries

- [x] CHK039 Are Zed's work-directory, platform, installation-status,
  language-server download and uninstall behaviors documented as explicit
  external dependencies with primary evidence? [Dependency, Assumptions;
  Research Decisions 1, 2]
- [x] CHK040 Is the assumption that one-time automatic public acquisition is
  acceptable stated separately from the non-negotiable prohibition on sending
  translation content? [Assumption, Spec Assumptions, FR-008, FR-009]
- [x] CHK041 Are deferred platforms, languages, custom UI, source mutation,
  host-global install and mandatory remote service clearly excluded so they
  cannot expand task scope? [Boundary, Spec Scope Boundaries]
- [x] CHK042 Is reuse of prototype work conditioned on simplifying the
  marketplace user path, with the rejected lifecycle complexity documented?
  [Boundary, Spec Assumptions; Plan Summary and Structure Decision]
- [x] CHK043 Is external upstream registry review identified as the only gate
  that can remain outside project control after a complete submitted release,
  without allowing a dev-extension test to substitute? [Dependency, Spec
  FR-026, FR-027; Publication Clean Gallery Acceptance]

## Notes

- Requirements review passed all 43 items on generation. No additional product
  clarification was needed.
- Final implementation review on 2026-07-16 kept all 43 results passing. The
  portable ONNX SGEMM backend and exact nested Eigen gitlink change native
  implementation/provenance only; they do not change the supported platform,
  package source boundary, storage owner, extension ID or publication route.
- Focus areas: zero-setup UX, executable/model supply chain, offline/privacy,
  recovery/removal and marketplace publication.
- Re-run during PR review if the package source, supported platform, model,
  storage owner, extension ID or publication route changes.
