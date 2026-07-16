# Implementation Plan: Repository Convergence and Cleanup

**Branch**: `main` (feature context `010-repository-convergence`) | **Date**:
2026-07-16 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from
`/specs/010-repository-convergence/spec.md`

## Summary

Converge the repository on the only supported product path:

```text
Zed Gallery extension
  -> verified extension-owned package
  -> translator-lsp
  -> translator-core safety/segmentation + embedded process boundary
  -> native Bergamot runner + fixed Mozilla en->es resources
```

Retire the standalone CLI, MCP/Agent context-server path, configurable
LibreTranslate/Azure providers, manual provider lifecycle, old wrapper flows,
orphaned tests/fixtures/targets and completed pre-009 Spec Kit trees after
migrating every live invariant. Preserve ADR/decision history with explicit
supersession, keep the detailed feature map, rebuild the exact marketplace
package before `v0.1.0`, and provide bounded normal/deep cleanup tiers for
generated output.

## Technical Context

**Language/Version**: Rust 2021 on Rust 1.96.1 for `translator-core`,
`translator-lsp` and the Zed WASM extension; C++17 for the retained native
runner; POSIX shell for repository gates

**Primary Dependencies**: Retain `lsp-server`, `lsp-types`, `serde`,
`serde_json`, `sha2`, `libc`, `ruzstd` and `zed_extension_api = 0.7.0` where
used. Remove direct dependencies whose only consumers are retired surfaces,
including `rmcp`, `schemars`, `tokio` and `ureq`; regenerate both Cargo locks
rather than editing them manually.

**Storage**: No product storage change. Runtime state stays in Zed's extension
work directory. Repository cleanup distinguishes tracked source, reproducible
build output, reusable locked source cache and prohibited persistent/user state.

**Testing**: Failing repository-boundary contracts first; retained Rust unit and
LSP integration tests; Zed extension tests; marketplace acquisition, package,
offline, privacy, license, removal and clean-smoke gates through the Makefile and
pinned Docker image

**Target Platform**: Development on Fedora through project containers;
published product remains Zed on Linux `x86_64`, CPU-only, English to Spanish

**Project Type**: Multi-language editor extension with one two-crate root Rust
workspace, one isolated Rust/WASM extension and one private native process

**Performance Goals**: No regression from feature 009: 20/20 offline cases
within 15 seconds each, package below 128 MiB, peak RSS below 1 GiB and at most
four inference threads. Maintenance goals: 52 to at most 34 Make targets, four
to two root workspace members and 35 to approximately 11 integration scripts.

**Constraints**: Preserve read-only behavior, Markdown/code protection, path and
UTF-8 safety, exact limits, log redaction, deterministic tests, package supply
chain, reproducibility and host-clean policy. Cleanup must precede the public
tag because core/LSP changes invalidate the current package identities.

**Scale/Scope**: Baseline is 368 tracked files, 42,844 lines, 169 Cargo lock
packages and approximately 32.5 GiB of audited reproducible residue. Expected
source reduction is at least 159 files/17,079 lines in the first cut and up to
184 files/19,939 lines after safe consolidation of features 001 and 006.

## Spec Kit Execution Record

Commands executed for this planning cycle:

```bash
.specify/scripts/bash/check-prerequisites.sh --json --paths-only
.specify/scripts/bash/setup-plan.sh --json
```

The feature directory was created as `specs/010-repository-convergence/` and
persisted in `.specify/feature.json`. `speckit-clarify` found no critical
ambiguity: the user requested a minimal product repository, the audit proved
the retired surfaces have no Gallery consumer, Git is the chosen full-history
archive, and ADRs remain the concise decision record.

## Constitution Check

### Before Phase 0: PASS AFTER CONSTITUTION 2.0.0

- **Safety-first translation**: PASS. The retained path remains read-only and
  preserves ambiguous/code content.
- **Single offline product boundary**: PASS. Constitution 2.0.0 names the
  Gallery/LSP/core/private-runner chain and limits Mock to deterministic tests.
- **Test-first development**: PASS. Each deletion wave begins with a negative
  boundary contract and migrates live invariants before old coverage disappears.
- **Explicit contracts and limits**: PASS. Constitution 2.0.0 retires CLI/MCP
  wire contracts while preserving active LSP, core, private-runner, package and
  limit contracts.
- **Minimal host footprint**: PASS. No host runtime, package or service is
  introduced; generated cleanup remains project-scoped.

The `speckit-constitution` amendment to version 2.0.0 passed before task
generation. The major bump records removal of required CLI/MCP/provider
boundaries; affected templates and ADR 0007 were updated in the same wave.

### After Phase 1 Design: PASS

The retained-surface and removal-manifest contracts preserve every safety,
privacy, limit, package and release obligation while deleting only surfaces
with no supported consumer. Constitution 2.0.0 adopts the Gallery/LSP/private
runner boundary and Mock-as-test-double wording, so the design passes without
exception.

## Phase 0 Research Decisions

Complete in [research.md](./research.md):

1. treat the Gallery/LSP/embedded-runner chain as the only supported product;
2. retire compatibility instead of hiding it behind optional features;
3. remove CLI/MCP and configurable local/remote providers together so the
   dependency and security surface actually converges;
4. retain Mock only as an injected deterministic test double;
5. use Git for full removed-history retention and ADRs for visible decisions;
6. migrate invariants before deleting completed pre-009 Spec Kit directories;
7. separate normal generated cleanup from explicit deep cache cleanup;
8. execute three disjoint workstreams with one integration coordinator.

No `NEEDS CLARIFICATION` marker remains.

## Phase 1 Design

### Retained product boundary

The normative allowlist is [contracts/retained-surface.md](./contracts/retained-surface.md).
Every tracked path must map to product runtime, release/supply-chain work,
active validation, project governance or current/future documentation.

### Removal boundary

The deletion waves and migration prerequisites are normative in
[contracts/removal-manifest.md](./contracts/removal-manifest.md). A path cannot
move to `removed` until its live invariant, backlink and build consumer fields
are empty or migrated.

### Data model

[data-model.md](./data-model.md) defines retained surfaces, removal candidates,
historical records, validation invariants, cleanup tiers and their state
transitions. No product runtime entity or public request shape is added.

### Interface contracts

No new user-facing API is introduced. Existing feature-009 acquisition,
translation-package and publication contracts remain authoritative. Feature 010
adds repository-maintainer contracts only:

- retained path/consumer allowlist;
- removal manifest with migration and rollback evidence;
- negative search boundary for retired surfaces;
- normal versus deep generated-cleanup boundary.

## Target Project Structure

```text
crates/
├── translator-core/                 # safety, segmentation, embedded boundary
└── translator-lsp/                  # only product server and read-only UX

native/
└── translator-embedded-runtime/     # private Bergamot runner

zed-extension/                       # Gallery WASM/acquisition/package state

ops/
└── marketplace/                     # exact locks, notices and release record

scripts/
├── marketplace/                     # build/fetch/validate exact package
└── worktrees/                        # persistent-storage guards

tests/
├── fixtures/
│   ├── markdown/
│   └── marketplace/
└── integration/                     # repository boundary + marketplace gates

specs/
├── 009-zed-marketplace-install/     # active release/publication truth
└── 010-repository-convergence/      # this cleanup cycle

docs/
├── PLAN.md                          # current work and post-v0.1 roadmap
├── feature-map.md                   # detailed backlog/history required by policy
├── decisions.md                     # decision index with explicit statuses
├── diagrams.md                      # current Gallery architecture
└── adr/                              # concise accepted/superseded history
```

The root also retains Spec Kit/agent configuration, Docker/Make/Cargo metadata,
GitHub workflows, licenses and repository policy files. It does not create an
`archive/` directory; removed history remains in Git.

## Execution Waves and Subagent Ownership

### Wave 0 - Governance and baseline

Coordinator:

1. record clean/dirty state, tracked-file/line/target/dependency metrics and
   generated-path preview;
2. apply constitution 2.0.0 through `speckit-constitution` and record ADR 0007;
3. update feature 009 to point compatibility retirement to feature 010 without
   rewriting its completed implementation history;
4. stop if the constitutional gate is not accepted.

### Wave 1 - Failing boundary contracts

Automation/tests subagent owns `tests/integration/` and introduces a single
repository-boundary gate that initially fails while the audited surfaces exist.
It must cover workspace members, dependency names, supported Make targets,
runtime provider settings, current documentation and the historical allowlist.
The coordinator runs and records the expected failure before removal begins.

### Wave 2 - Parallel source convergence

Three subagents work on disjoint paths:

- **Rust/source subagent**: owns `crates/` and focused Rust tests. It changes the
  LSP to construct the adjacent embedded provider directly, removes remote
  confirmation/configuration and the CLI/MCP/provider source trees, and keeps
  Mock only for tests.
- **Build/operations subagent**: owns `Makefile`, `scripts/`, `ops/` provider
  paths, integration tests/fixtures, CI and PR template. It removes historical
  targets, renames the retained LSP release target and implements cleanup tiers.
- **Documentation/traceability subagent**: owns `README.md`, `docs/`,
  `specs/001-*` through `specs/009-*`, AGENTS hierarchy text and Spec Kit
  templates affected by the constitution. It migrates live constraints, marks
  ADRs superseded and removes historical trees only after backlink checks.

The coordinator owns root `Cargo.toml`, regenerated locks, `zed-extension/`
integration, package identities and any cross-stream conflict. Subagents do not
edit those coordinator-owned paths.

### Wave 3 - Historical and dependency pruning

After focused tests pass:

1. remove completed pre-009 spec trees whose live constraints have migrated;
2. remove obsolete research/product/Agent documentation, retaining current
   diagrams, decision history and feature-map detail;
3. regenerate Cargo locks inside the project container;
4. require zero retired direct dependencies and zero broken backlinks;
5. make the repository-boundary contract pass.

### Wave 4 - Exact package regeneration

1. normal-clean all generated Rust/WASM outputs, including the stale local
   `zed-extension/extension.wasm`, but preserve locked native source cache;
2. rebuild `translator-lsp` and the native runner from the retained tree;
3. regenerate exact file sizes/hashes and the deterministic release archive;
4. update `ops/marketplace/package.lock.json` and feature-009 validation;
5. run three independent marketplace-shaped real translations.

### Wave 5 - Full gates and disk cleanup

Run the [quickstart](./quickstart.md) matrix. Only after the exact candidate and
evidence are reproducible may the coordinator offer the deep cleanup tier for
locked/download caches. Preview every deletion, preserve unrecognized paths and
never target `.agents/`, `.codex/`, secrets or provider/user data.

## TDD and Verification Order

1. failing constitution and repository-boundary gates;
2. direct embedded LSP tests before provider/configuration removal;
3. focused core/LSP/extension tests after each source wave;
4. dependency, Make/help, script and documentation negative contracts;
5. full format, Clippy, test and cargo-deny gates;
6. native supply-chain and deterministic package rebuild;
7. acquisition, offline/privacy/resource/removal regression gates;
8. exact-package three-run smoke and updated validation record;
9. public release check remains expected to fail only for the absent tag until
   the new candidate is reviewed and published.

## Rollback Strategy

- Keep waves in reviewable conventional commits; do not mix generated-output
  deletion with source history.
- Revert the last failing wave rather than restoring an untracked backup.
- Do not publish a tag or mutate the upstream Gallery submission until every
  project-controlled gate passes with regenerated identities.
- Preserve any dirty user changes before a candidate path is deleted.
- If a migrated invariant is missing, restore the original test/source from Git,
  add the retained-path coverage, and resume only after the focused gate passes.

## Complexity Tracking

No constitutional exception remains. Constitution 2.0.0 resolved the prior
CLI/MCP boundary conflict before task generation and implementation.
