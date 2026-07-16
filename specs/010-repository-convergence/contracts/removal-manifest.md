# Contract: Removal Manifest

## Wave A - MCP, Agent and wrapper compatibility

Remove after the negative boundary gate is observed failing:

- `crates/translator-mcp/`;
- MCP member/dependencies/lock entries and Make targets;
- `scripts/zed-extension/prepare.sh`;
- historical `tests/integration/zed_extension_*.sh`;
- `docs/zed-ux-flow.md`, `tests/integration/zed_ux_flow_*.sh` and helper;
- MCP/Agent current-use references in README, plan, feature 009 and diagrams.

Pure MCP protocol, tool discovery, schema, session and stdio tests are deleted,
not migrated. Shared path/Markdown/redaction/no-mutation invariants must already
exist at core/LSP/marketplace boundaries.

## Wave B - CLI and configurable providers

Remove only after constitution 2.0.0 and direct embedded LSP tests:

- `crates/translator-cli/`;
- `crates/translator-core/src/{azure_translator,libretranslate,privacy,provider_config}.rs`;
- remote/configurable branches in core provider and LSP state/protocol/main;
- provider/remote-only core and LSP tests;
- `ops/providers/`, `scripts/providers/`, operational-provider fixtures,
  helpers and integration scripts;
- CLI/provider Make targets and `ureq` plus unreachable lock dependencies.

Retain and simplify `secrets.rs` only if a live local/path invariant consumes
it; otherwise remove it after use analysis. Retain `MockProvider` strictly as a
test double.

## Wave C - Redundant direct-development automation

Remove:

- `scripts/zed-extension/prepare-direct.sh`;
- `tests/integration/zed_direct_*.sh`;
- old `zed-direct-*`, `zed-extension-prepare`, `test-zed-extension` and provider
  targets after their retained behavior is folded into marketplace targets;
- marketplace package fixture files proven to have no consumer.

Rename the retained `zed-direct-server-release` behavior to
`marketplace-lsp-release` or `translator-lsp-release`; update every consumer
before deleting the old name.

## Wave D - Historical Spec Kit and documentation

After live contracts migrate into 009/010 and ADR 0007 exists, remove:

- `specs/001-translation-core-contract/` through
  `specs/007-operational-providers/` (there is no retained 008 directory);
- `docs/product.md` after its current product summary moves to README/PLAN;
- obsolete `docs/research/*.md` after live constraints move to constitution,
  009/010 or ADRs.

Retain all ADR files but mark 0001-0003 and 0005 superseded, 0004 partially
superseded and 0006 accepted; ADR 0007 is the current convergence decision.
Retain detailed `docs/feature-map.md` content while correcting current states
and prohibited future buffer mutation language.

## Wave E - Generated Residue

Normal cleanup is allowlisted to:

- root and nested Rust `target/` directories;
- `zed-extension/target/` and stale `zed-extension/extension.wasm`;
- `.cache/zed-local-validation/`;
- empty/stale project temporary and removed-crate build directories identified
  by the audit.

Normal cleanup preserves `.cache/embedded-source/` and other fixed release
inputs. Deep cleanup may remove those caches only after an explicit preview and
confirmation. Neither tier may use an unbounded ignored-file sweep or select
`.agents/`, `.codex/`, `.git/`, secrets, real `.env` files or persistent data.

## Deletion Gate Per Candidate

Before deletion, record:

1. zero unreviewed dirty changes under the path;
2. incoming reference list;
3. live invariant list and retained replacement gate;
4. dependency/backlink migration;
5. expected failing negative contract;
6. rollback commit boundary.

After deletion, require the focused gate, repository-boundary gate and
`git diff --check`. A failed retained invariant restores the candidate from Git
until coverage or migration is corrected.
