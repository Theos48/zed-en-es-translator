# Research: Repository Convergence and Cleanup

## Baseline Evidence

Three parallel read-only audits covered Rust/runtime dependencies,
build/operations surfaces and documentation/roadmap consistency.

- Supported runtime graph: `zed-extension -> translator-lsp -> translator-core
  -> translator-embedded-runtime`.
- `translator-mcp` has no incoming edge from the extension, LSP, package or
  marketplace scripts; it is kept alive only by workspace/Make/test history.
- `translator-cli`, LibreTranslate and Azure likewise have no role in the
  published package or Gallery journey.
- Root baseline: 4 workspace crates, 52 Make targets, 169 lock packages,
  35 integration scripts, 368 tracked files and 42,844 lines.
- Audited reproducible residue: about 31 GiB in `target/`, 1.3 GiB in
  `zed-extension/target/`, 157 MiB in `.cache/zed-local-validation/` and one
  stale 996 KiB `zed-extension/extension.wasm` containing the old MCP wrapper.
- Feature 009 permits compatibility in FR-025, while constitution 1.0.0 still
  mandates the CLI wire; cleanup therefore requires a new feature and a
  constitutional amendment rather than silently editing completed tasks.

## Decision 1: One supported product graph

**Decision**: Keep only the Gallery extension, automatic verified package,
direct LSP, safety/segmentation core, native Bergamot runner and marketplace
release/supply-chain surfaces.

**Rationale**: These are the only components present in the published package
and the only route exercised by the feature-009 no-setup contract.

**Alternatives considered**:

- Remove MCP only: rejected because CLI, remote/configurable providers and
  manual lifecycle would still leave multiple unsupported product boundaries.
- Keep compatibility behind build features: rejected because it retains
  dependency, audit, documentation and regression cost without a consumer.

## Decision 2: Embedded provider is internal, not configurable

**Decision**: The product LSP constructs the verified adjacent embedded
provider directly. It accepts no provider name, URL, API-key reference, remote
allowance or request-level remote confirmation. Mock remains an injected test
double only.

**Rationale**: The extension already fixes the package identity and the product
promise is local/offline. Runtime configuration can only widen or confuse that
boundary.

**Alternatives considered**:

- Keep the current environment selector with one accepted value: rejected
  because an unnecessary parser/error surface remains and the product plan says
  provider selection is internal.
- Keep LibreTranslate/Azure as advanced modes: rejected because they contradict
  the zero-setting single-product objective.

## Decision 3: Preserve invariants, not obsolete test surfaces

**Decision**: Retain tests for limits, Markdown/code preservation, path/UTF-8
safety, no mutation, redaction, process bounds, acquisition, package identity,
offline operation and removal. Delete tests for MCP protocol, Agent workflow,
CLI wire and retired provider behavior after shared invariants are mapped.

**Rationale**: Test count is not the objective. Coverage must express current
risks at their closest retained boundary.

## Decision 4: Git is the full history archive

**Decision**: After live constraints migrate, remove completed pre-009 Spec Kit
trees and obsolete research/tutorial files instead of moving them to another
directory. Keep ADRs and the decision index, with explicit superseded statuses,
and keep detailed `docs/feature-map.md` content as required by project policy.

**Rationale**: An in-tree archive would preserve the same search noise and
backlinks. Git already preserves exact historical artifacts; ADRs explain why
the architecture changed.

**Alternative considered**: Keep specs 001 and 006 indefinitely because they
originated the core/LSP. Rejected as a final state; their still-live contracts
must be consolidated into active 009/010 first, then Git retains their detail.

## Decision 5: Remove dependencies by consumer graph

**Decision**: Remove full retired crates and provider modules, then regenerate
locks in the pinned container. Do not hand-edit transitive packages.

**Rationale**: Cargo determines which shared transitive packages remain. The
expected direct removals are `rmcp`, `schemars`, `tokio` and `ureq`, plus their
now-unreachable subgraphs.

## Decision 6: Clean before rebuilding the exact candidate

**Decision**: Normal cleanup removes all Rust/WASM build output and stale
generated extension artifacts, but preserves the fixed Mozilla source checkout.
The exact LSP/package is then rebuilt and its identities updated before tag.

**Rationale**: The stale `extension.wasm` contains the old MCP wrapper and can
invalidate manual evidence. Conversely, deleting the locked source cache early
adds avoidable network/rebuild cost while release gates are still open.

**Alternative considered**: Delete every ignored path with `git clean -fdX`.
Rejected because it also selects agent configuration and skips/enters nested
repositories in ways that are too broad for a safe project cleanup contract.

## Decision 7: Use three disjoint implementation subagents

**Decision**: Assign Rust/source, build/operations and
documentation/traceability to separate agents; the root coordinator owns shared
locks, extension integration, exact package identities and final validation.

**Rationale**: The work is parallelizable, but Cargo locks, package locks and
cross-artifact status are high-conflict integration points and need one owner.

## Resolved Unknowns

- **CLI**: retire after constitution 2.0.0; it has no Gallery consumer.
- **MCP/Agent**: retire completely from current source and operational docs.
- **Providers**: retire LibreTranslate/Azure and all manual/configurable paths;
  keep only embedded production and Mock tests.
- **Historical specs**: migrate live invariants, then remove; Git is archive.
- **ADRs/feature map**: retain and mark statuses; do not erase detailed backlog.
- **Generated output**: normal and deep tiers with previews and allowlists.
- **Release timing**: cleanup and new hashes must land before `v0.1.0`.

No `NEEDS CLARIFICATION` marker remains.
