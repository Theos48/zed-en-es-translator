# Cleanup Safety Requirements Checklist: Repository Convergence and Cleanup

**Purpose**: Review whether deletion scope, migration, traceability, rollback,
release reproducibility and generated-output safety requirements are complete
before tasks or implementation
**Created**: 2026-07-16
**Feature**: [spec.md](../spec.md)

**Depth / Audience / Timing**: Formal pre-implementation gate / PR reviewer /
before `speckit-tasks` and again before final removal waves

## Requirement Completeness

- [x] CHK001 Are retained product, release, validation, governance and history
  surfaces all required to have a named consumer? [Completeness, Spec FR-001;
  Plan Retained product boundary]
- [x] CHK002 Are MCP, Agent, CLI, configurable providers and manual lifecycle
  surfaces explicitly included rather than hidden behind a generic “obsolete”
  label? [Completeness, Spec FR-003-FR-005]
- [x] CHK003 Are build metadata, locks, CI, Make targets, scripts, tests,
  fixtures and documentation all included in orphan cleanup requirements?
  [Completeness, Spec FR-006]
- [x] CHK004 Are requirements defined for migrating live contracts and safety
  invariants before completed feature trees leave the working tree?
  [Completeness, Spec FR-007, FR-012]
- [x] CHK005 Are ADR/decision history and detailed feature-map retention
  distinguished from obsolete operational tutorials and full historical specs?
  [Completeness, Spec FR-008-FR-009]
- [x] CHK006 Are both tracked-source cleanup and generated-output cleanup
  covered with separate boundaries? [Completeness, Spec FR-014; US4]

## Requirement Clarity

- [x] CHK007 Is the single supported product graph stated precisely enough to
  decide whether a path has a current consumer? [Clarity, Spec US1; SC-001]
- [x] CHK008 Is Mock described unambiguously as a test double rather than a
  second supported runtime path? [Clarity, Spec FR-004; Assumptions]
- [x] CHK009 Is “remove historical specs” conditioned on migration and Git/ADR
  retention rather than interpreted as immediate unreviewed deletion?
  [Clarity, Spec FR-007-FR-008]
- [x] CHK010 Are normal and deep cleanup tiers distinguished by eligible paths,
  preservation rules and confirmation needs? [Clarity, Spec FR-014; US4]
- [x] CHK011 Is “before release” tied to the immutable public tag/package
  identity boundary? [Clarity, Spec FR-010; SC-009]
- [x] CHK012 Is the subagent model specific about disjoint ownership and one
  integration authority? [Clarity, Spec FR-016]

## Requirement Consistency

- [x] CHK013 Does retirement of CLI/MCP require a constitutional amendment so
  the spec does not silently conflict with governance? [Consistency, Spec
  FR-011; Plan Constitution Check]
- [x] CHK014 Do cleanup requirements preserve every safety, privacy, limit and
  read-only requirement from the Gallery product? [Consistency, Spec FR-002,
  FR-SEC-A-FR-SEC-D]
- [x] CHK015 Are feature-009 publication identities required to be regenerated
  after source convergence rather than reused? [Consistency, Spec FR-010;
  SC-009]
- [x] CHK016 Are roadmap alignment requirements consistent with the project
  rule that detailed feature-map content must not be deleted? [Consistency,
  Spec FR-009]
- [x] CHK017 Does the no-archive-directory assumption remain consistent with
  rollback and historical traceability requirements? [Consistency, Spec
  FR-017; Assumptions]

## Acceptance Criteria Quality

- [x] CHK018 Can the absence of retired entry points and dependencies be
  measured objectively? [Measurability, Spec SC-001-SC-004]
- [x] CHK019 Are repository-reduction outcomes quantified without allowing
  file-count reduction to override invariant preservation? [Measurability,
  Spec SC-003]
- [x] CHK020 Are documentation quality outcomes measurable through resolved
  links and explicit decision statuses? [Measurability, Spec SC-004-SC-005]
- [x] CHK021 Are product-regression outcomes tied to the complete
  project-controlled gate set and real non-Mock smoke count? [Measurability,
  Spec SC-006-SC-007]
- [x] CHK022 Is generated-residue success defined by an allowlist and preserved
  paths rather than disk-space reduction alone? [Measurability, Spec SC-008]

## Scenario and Edge-Case Coverage

- [x] CHK023 Are dirty worktrees and unrelated user changes explicitly covered
  as preservation cases? [Coverage, Spec Edge Cases; FR-015]
- [x] CHK024 Are files with hidden compile-time/test consumers covered before
  classification as obsolete? [Coverage, Spec Edge Cases; FR-013]
- [x] CHK025 Are security tests that outlive their original protocol surface
  required to migrate to a retained boundary? [Coverage, Spec Edge Cases;
  FR-012]
- [x] CHK026 Are changed package hashes, an unexpectedly created public tag and
  an in-progress Gallery submission addressed? [Coverage, Spec Edge Cases;
  FR-010]
- [x] CHK027 Are active builds, other worktrees and unrecognized ignored paths
  covered before generated deletion? [Coverage, Spec Edge Cases; FR-014-FR-015]
- [x] CHK028 Is failure of a retained invariant required to restore the removed
  candidate before work continues? [Recovery, Spec FR-017; Plan Rollback]

## Security, Privacy and Supply Chain

- [x] CHK029 Are source, agent configuration, secrets, real environment files
  and persistent user/provider data explicitly excluded from cleanup tiers?
  [Security, Spec FR-014-FR-015; US4]
- [x] CHK030 Does the spec forbid replacing removed remote providers with a
  different network translation path? [Security, Spec FR-SEC-C]
- [x] CHK031 Are log and evidence requirements content-free during boundary,
  package and cleanup failures? [Privacy, Spec FR-SEC-D; Plan Verification]
- [x] CHK032 Are exact package identity, licenses, notices, source locks and
  deterministic rebuild obligations retained after dependency pruning?
  [Supply Chain, Spec FR-002, FR-010; SC-006-SC-009]

## Dependencies and Process Gates

- [x] CHK033 Is constitution 2.0.0 an explicit blocking dependency before task
  generation or source deletion? [Dependency, Spec FR-011; Plan Constitution]
- [x] CHK034 Are Cargo lock regeneration and package-lock regeneration treated
  as coordinated outputs rather than independent manual edits? [Dependency,
  Plan Technical Context and Wave 4]
- [x] CHK035 Is the rename/migration of the retained LSP release behavior
  required before deleting historical target names? [Dependency, Removal
  Manifest Wave C]
- [x] CHK036 Are backlink repair, invariant migration and negative boundary
  coverage required before the historical-spec deletion wave? [Dependency,
  Spec FR-007, FR-012-FR-013]
- [x] CHK037 Is the sequence `constitution -> failing boundary -> parallel
  convergence -> exact rebuild -> full gates -> publication` explicit and
  non-circular? [Process, Plan Execution Waves]

## Notes

- 37/37 requirement-quality items pass on generation.
- Re-run if the retained product graph, history policy, public tag state,
  provider posture or cleanup-tier boundary changes.
- This checklist reviews the requirements and plan; implementation verification
  belongs to the quickstart and future tasks.
