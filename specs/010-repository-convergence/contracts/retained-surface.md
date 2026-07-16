# Contract: Retained Repository Surface

## Product Runtime

The only supported runtime chain is:

```text
zed-extension/
  -> crates/translator-lsp/
  -> crates/translator-core/
  -> native/translator-embedded-runtime/
```

`translator-core` retains only modules required for contract types used by the
LSP/private runner, limits, Markdown segmentation/reconstruction, workspace
safety, redaction, the Provider trait/test Mock and the embedded process,
protocol and provider. The exact post-refactor module list is enforced by use
and lint gates rather than by preserving unused compatibility modules.

## Package and Supply Chain

Retain:

- `zed-extension/{Cargo.toml,Cargo.lock,LICENSE,extension.toml,src/,tests/}`;
- `ops/marketplace/`;
- `scripts/marketplace/`;
- `docker/rust-toolchain.Dockerfile`;
- `.github/workflows/marketplace-package.yml`;
- marketplace fixtures and `tests/integration/marketplace_*.sh`;
- exact source/model/package locks, license bundle and source instructions.

## Repository Quality

Retain:

- root `Cargo.toml`/`Cargo.lock` with only core and LSP members;
- `Makefile`, `deny.toml`, `.dockerignore`, `.gitignore`;
- CI, Dependabot and the current PR template;
- `scripts/cleanup/generated.sh` as the bounded normal/deep cleanup interface;
- `scripts/worktrees/` and `tests/integration/worktree_storage_guard.sh`;
- focused core/LSP/extension tests and their Markdown/marketplace fixtures for
  live invariants;
- one repository-boundary integration gate.

## Documentation and Governance

Retain:

- `README.md` as concise product/current-status documentation;
- `CONTRIBUTING.md` and `docs/deployment.md` as the non-duplicative operational
  guides for development, modification, release and Gallery publication;
- `.specify/` as the active Spec Kit workflow, constitution, templates,
  integrations and scripts, plus `skills-lock.json` as its reproducible skill
  resolution;
- `AGENTS.md` managed current-plan section and project workflow rules;
- `specs/009-zed-marketplace-install/` and
  `specs/010-repository-convergence/`;
- `docs/PLAN.md`, `docs/feature-map.md`, `docs/decisions.md`, current
  `docs/diagrams.md` and `docs/adr/`.

Old ADRs remain only as explicit accepted/superseded history. They are excluded
from negative keyword scans through an exact allowlist, not a broad `docs/**`
exception.

## Prohibited Product Surfaces

Current source, automation and user guidance must not expose:

- MCP server or Agent Panel/context-server integration;
- standalone translator CLI or its JSON wire;
- LibreTranslate, Azure or generic remote/configurable provider behavior;
- provider URL, API-key reference, remote allowance/confirmation or arbitrary
  binary configuration;
- manual dev-extension preparation as a user or release path;
- a second runtime/model lifecycle outside Zed's extension work directory.

## Acceptance Boundary

A retained path is valid only if it has a named consumer and passes its focused
gate. The repository-boundary test fails on an unclassified tracked file in a
removed path family, a retired dependency/target, a broken backlink or current
operational wording for a prohibited surface.
