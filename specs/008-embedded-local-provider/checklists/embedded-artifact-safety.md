# Embedded Artifact Safety Requirements Checklist

**Purpose**: Review whether the F012 requirements are complete, clear,
consistent and measurable for artifact supply chain/lifecycle and the embedded
offline/privacy boundary before task generation or PR approval. This checklist
evaluates the requirements themselves, not implementation behavior.

**Created**: 2026-07-15
**Feature**: [spec.md](../spec.md)
**Depth / Audience / Timing**: Standard / reviewer / pre-tasks and PR review

## Candidate and Scope Completeness

- [x] CHK001 Are the conditions that make Bergamot a provisional candidate,
  a promoted provider, or a no-go outcome explicitly distinguished without
  implying implementation or publication success? [Spec FR-001, FR-025;
  Research Decisions 1, 10]
- [x] CHK002 Are all mandatory comparison dimensions stated for every viable
  candidate, including quality, maintenance, platform, legal, supply-chain,
  operational and cross-surface criteria? [Spec FR-004; Research Candidate
  comparison]
- [x] CHK003 Is the fallback rule complete for both “primary candidate fails”
  and “no alternative passes,” including preservation of Mock and
  LibreTranslate? [Spec FR-003, FR-025; Plan Summary]
- [x] CHK004 Are the supported platform, CPU class, language direction and two
  acceptance surfaces explicit enough to prevent implied support expansion?
  [Spec Scope Boundaries, Assumptions; Plan Technical Context]
- [x] CHK005 Is “no user-visible Docker lifecycle” distinguished clearly from
  the repository's existing containerized reproducible build process? [Spec
  FR-002, Assumptions; Quickstart What this path is]

## Artifact Identity, Provenance, and License Requirements

- [x] CHK006 Does the manifest requirement enumerate every artifact class that
  can affect execution, including runner, recursive native dependencies,
  model, vocabulary, shortlist, configuration, build recipe and notices?
  [Spec FR-006; Data Model RunnerManifest, ModelArtifact]
- [x] CHK007 Are exact source revision, record ID, URL, compressed/installed
  size, dual hash, platform, language and compatibility requirements defined
  for each applicable artifact? [Spec FR-006, FR-009; Data Model
  ModelArtifact]
- [x] CHK008 Are requirements explicit that a valid hash is insufficient when
  license, provenance, language, platform or compatibility fails? [Spec
  FR-007, FR-009; Artifact Lifecycle Acquisition rules]
- [x] CHK009 Is the boundary between repository-level license evidence and an
  artifact-level delivery/publication conclusion stated without assuming
  rights from metadata that lacks a per-blob license field? [Spec FR-007;
  Research Decisions 2, 10]
- [x] CHK010 Are source-offer, notice, modification and transitive native
  dependency obligations required for the exact built runner rather than only
  named generally? [Spec FR-006, FR-024; Data Model RunnerManifest]
- [x] CHK011 Is the authority and acceptance evidence for changing a license
  conclusion or publication state defined clearly enough for review? [Gap;
  Spec FR-027; Data Model EmbeddedProviderProfile, RunnerManifest,
  ModelArtifact]
- [x] CHK012 Are requirements for source/dependency lock review and change
  control complete when upstream commit, model record, URL, hash, license or
  maintenance state changes? [Spec Edge Cases, FR-017; Research Decision 2]

## Disclosure, Consent, and Acquisition Quality

- [x] CHK013 Does disclosure require purpose, exact identity/source, license
  conclusion, transfer/installed/worst-case size, destination scope, network
  use and publication limits before any mutation? [Spec US2-AS1, FR-008;
  Artifact Lifecycle Disclosure]
- [x] CHK014 Is consent bound unambiguously to the full manifest digest, with
  rejection rules for missing, malformed, stale and mismatched values? [Spec
  FR-008; Research Decision 7; Artifact Lifecycle Disclosure and consent]
- [x] CHK015 Are cancellation and interruption requirements defined at every
  acquisition, decompression, validation and activation boundary, not only
  before download begins? [Spec Edge Cases, FR-008, FR-018; Artifact Lifecycle
  Promotion]
- [x] CHK016 Are acquisition requirements complete for exact HTTPS source,
  status, redirects, inherited proxy, retries, content length, byte ceiling,
  compressed hash, expanded hash and fixed destination name? [Spec FR-009,
  FR-021; Artifact Lifecycle Acquisition rules]
- [x] CHK017 Is the behavior for already-present content-addressed objects
  specified without allowing object reuse to bypass fresh update consent or
  verification? [Artifact Lifecycle Disclosure and consent, Promotion]
- [x] CHK018 Is it explicit that normal provider selection, LSP startup,
  readiness, verify and rollback cannot trigger implicit acquisition or
  registry lookup? [Spec FR-011, FR-016, FR-017; Provider Selection Resolution
  order; Quickstart Direct Zed use]

## Storage, State, Concurrency, and Removal Requirements

- [x] CHK019 Are XDG scope, owner, mode, persistent-filesystem, regular-file,
  hard-link, symlink and path-containment requirements specified for both reads
  and mutations? [Spec FR-010, FR-018; Research Decision 6; Data Model
  RunnerManifest]
- [x] CHK020 Are immutable object/set semantics and the exact meaning of
  candidate, current and previous consistent across the data model and
  lifecycle contract? [Data Model ArtifactSet, InstallationState; Artifact
  Lifecycle Owned scope, Promotion]
- [x] CHK021 Are atomicity requirements complete for crash points before and
  after fsync, set finalization and state replacement? [Spec FR-018; Data Model
  InstallationState; Artifact Lifecycle Promotion]
- [x] CHK022 Are concurrency requirements measurable for lifecycle locks,
  state locks, inference leases, update while current serves, queueing and busy
  outcomes within the 15-second deadline? [Spec Edge Cases, FR-018; Artifact
  Lifecycle Concurrency and leases]
- [x] CHK023 Is rollback behavior defined for valid previous, missing previous,
  corrupt previous and failed offline smoke without destroying current or the
  remaining known-good set? [Spec US3-AS3, Edge Cases; Artifact Lifecycle
  Rollback]
- [x] CHK024 Are cleanup ownership and refusal requirements complete for
  unknown files, active leases, links, partial staging and ambiguous state?
  [Spec US3-AS4, FR-018; Artifact Lifecycle Removal]
- [x] CHK025 Is successful “complete removal” measurable while excluding repo
  files, editor buffers, unrelated XDG data, Docker state, packages and system
  services? [Spec SC-009; Artifact Lifecycle Owned scope, Removal]
- [x] CHK026 Is the relationship between provider cleanup and generic
  `make clean` explicit and consistent in every user-facing planning artifact?
  [Spec FR-016; Artifact Lifecycle Versioned commands; Quickstart Safe
  operations]

## Native Process and Offline Boundary Quality

- [x] CHK027 Does the runner contract fully specify executable identity,
  fixed arguments, cwd, environment clearing, pipe caps, deadline start,
  kill/reap, exit handling and no retry? [Spec FR-014, FR-019; Runner Wire
  Invocation model, Timeout]
- [x] CHK028 Are semantic 40 KiB output and transport-framing caps clearly
  distinguished so JSON overhead neither causes ambiguity nor weakens the
  constitutional limit? [Spec FR-014; Data Model EmbeddedInvocation; Runner
  Wire Success response]
- [x] CHK029 Are request/response schema rules complete for versions, unknown
  or duplicate fields, trailing data, UTF-8, cardinality, order, emptiness and
  aggregate limits? [Spec FR-014, FR-019; Runner Wire Request, Success
  response]
- [x] CHK030 Are the normal inference binary's no-downloader/no-updater/no-
  listener requirements and allowed native dynamic dependency boundary stated
  in auditable terms? [Spec FR-011; Research Decisions 3, 8; Runner Wire
  Invocation model]
- [x] CHK031 Is zero-network behavior defined for readiness, inference,
  verification and rollback, including an observable definition of “attempted
  contact” rather than only successful requests? [Spec FR-011, SC-002; Evidence
  Benchmark protocol]
- [x] CHK032 Is the one-shot-to-persistent/FFI fallback boundary explicit about
  the new design review required if cold-start performance fails? [Research
  Decision 3; Plan Complexity Tracking]

## Measurement and Acceptance-Criteria Quality

- [x] CHK033 Are all resource and latency budgets stated with units, execution
  class and percentile/max semantics, including the relationship between cold
  readiness and the total 15-second provider deadline? [Spec SC-003, SC-004;
  Evidence Mandatory budgets]
- [x] CHK034 Is the 20-case corpus composition, warmup/repetition/round method,
  cache condition and CLI/LSP/Zed split sufficiently precise for reproducible
  evidence? [Spec FR-022, SC-003; Research Decision 9; Evidence Benchmark
  protocol]
- [x] CHK035 Are upstream artifact sizes clearly labeled as reviewed facts and
  separated from unmeasured native runner, RSS, CPU and latency results?
  [Spec SC-004; Research Decision 2; Quickstart Resource envelope]
- [x] CHK036 Are go/no-go requirements explicit that every mandatory measured
  field must be known and within budget before promotion or feature closure?
  [Spec SC-004, SC-008; Evidence Go/no-go conclusion]
- [x] CHK037 Are real-model CLI and direct-Zed evidence required in addition to
  controlled doubles, with public synthetic inputs, external networking
  disabled and source/buffer non-mutation? [Spec FR-021, FR-022, SC-001;
  Evidence Evidence layers, Product boundary]
- [x] CHK038 Are linguistic acceptance requirements precise enough to reject
  empty, unchanged, wrong-language or structurally damaged output without
  making an unbounded translation-quality claim? [Spec US1, FR-004, FR-022;
  Evidence Synthetic fixture matrix]

## Cross-Surface Privacy and Consistency

- [x] CHK039 Are CLI and direct Zed required to share selection, safety,
  segmentation, limits, response validation, errors and redaction, while MCP
  remains compatibility-only? [Spec FR-012, FR-026; Provider Selection Gate
  order; Plan Integration Strategy]
- [x] CHK040 Is the direct-Zed offline label requirement clear about timing
  and prohibited model/path/version disclosure before execution? [Spec
  FR-013; Provider Selection Locality]
- [x] CHK041 Are all forbidden diagnostic/evidence fields consistently listed
  across spec, runner stderr, lifecycle output, benchmark and validation
  records? [Spec FR-020, FR-023; Data Model BenchmarkRecord; Runner Wire
  Failure response; Evidence Redacted record schema]
- [x] CHK042 Are requirements explicit that an explicitly selected broken
  embedded provider cannot silently use Mock, LibreTranslate or remote output?
  [Spec FR-019, FR-025; Provider Selection Stable outcomes]
- [x] CHK043 Is the boundary between artifact consent and per-request remote
  confirmation clear enough that neither can substitute for the other? [Spec
  FR-008, FR-013, FR-SEC-C; Provider Selection Locality]
- [x] CHK044 Are non-mutation requirements consistent for files, buffers,
  clipboard, lifecycle storage and evidence, including failure and rollback
  scenarios? [Spec FR-015, FR-020, SC-009; Evidence Product boundary]

## Dependencies, Assumptions, and Ambiguities

- [x] CHK045 Are assumptions about Fedora `x86_64`, existing containerized
  build tooling, Zed API 0.7.0 and user-scoped XDG storage made visible wherever
  they affect acceptance or portability claims? [Spec Assumptions; Plan
  Technical Context; Research Decisions 4, 6]
- [x] CHK046 Is the distinction between “project-built runner,” “user-acquired
  resources,” “supported local use,” and “published/bundled extension”
  consistent across spec, plan, quickstart and ADR? [Spec FR-005, FR-007;
  Research Decision 10; ADR 0006]
- [x] CHK047 Are the required reviewer/authority and artifact-level evidence
  for clearing the publication block defined, or is the unresolved ownership
  explicitly tracked before F009? [Spec US4, FR-023, FR-027; Research Decision
  10]
- [x] CHK048 Are all intentionally deferred choices—other platforms,
  persistent process, FFI, Zed-managed acquisition and redistribution—bounded
  by an explicit new-review condition rather than left as implementation
  discretion? [Spec Scope Boundaries; Research Decisions 3, 4, 10; ADR 0006]

## Notes

- Requirements review completed 48/48 on 2026-07-15 before task generation.
  CHK011 and CHK047 caused FR-027 and the separate local/publication human
  approval records to be added; no implementation evidence was used to mark a
  requirement adequate.
- Mark an item complete only when the cited requirements are adequate as
  written; implementation evidence does not repair an ambiguous requirement.
- Record requirement changes in the feature artifacts and stable architecture
  changes in `docs/decisions.md` or an ADR.
- Review gaps must be resolved or explicitly assigned to the separate F009
  publication gate before F012 closure.
- Re-run on 2026-07-15: 48/48 requirement-quality items remain satisfied.
  Implementation evidence is tracked separately; human artifact approval,
  real-model measurements and publication review remain blocked/open.
