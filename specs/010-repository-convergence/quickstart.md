# Quickstart: Validate Repository Convergence and Cleanup

This is a maintainer validation guide for applying the cleanup plan. It is not
an end-user installation guide.

## Prerequisites

- Constitution 2.0.0 approved with Gallery/LSP/private-runner boundaries.
- Clean or fully reviewed worktree; unrelated user changes preserved.
- Docker available through the project Makefile.
- Persistent checkout storage accepted by the workspace guard.
- No public `v0.1.0` tag or active upstream Gallery submission based on the old
  package identities.

## 1. Capture Baseline and Preview

```bash
git status --short --branch
make workspace-storage-check
make worktree-audit
make test-repository-boundary
make clean-preview
```

Expected baseline for this plan: four root workspace members, 52 Make targets,
169 lock packages, 35 integration scripts, 368 tracked files and approximately
32.5 GiB of normal-tier generated residue. If the implementation baseline has
changed, record the new counts instead of forcing these historical values.

## 2. Observe the Failing Boundary

```bash
make test-repository-boundary
```

Before removal this must fail because MCP, CLI, configurable providers and old
operational documentation still exist. The failure output must contain paths or
surface names, never source/translation content or secrets.

## 3. Validate Each Removal Wave

After each wave:

```bash
make fmt
make clippy
make test
make test-repository-boundary
git diff --check
```

Expected after final source convergence:

- root workspace contains only `translator-core` and `translator-lsp`;
- current source/automation/help has no MCP, Agent, CLI, LibreTranslate, Azure,
  provider URL/key/remote-confirmation or arbitrary binary path;
- retained ADR history is the only exact allowlisted location for obsolete
  architecture terms;
- no removed path has an unresolved backlink;
- Cargo locks contain no direct package used only by removed surfaces.

## 4. Run Focused Product Gates

```bash
make test-marketplace-foundation
make test-marketplace-contract
make test-marketplace-acquisition
make test-marketplace-native-supply-chain
```

These prove the embedded process, strict package/acquisition boundary, failure
recovery and native provenance before release identities are changed.

## 5. Clean and Rebuild the Exact Package

```bash
make clean-preview
make clean
make test-marketplace-package
make test-marketplace-release-contents
make test-marketplace-offline
```

Normal cleanup must remove all target directories, validation output and the
stale generated extension WASM while preserving locked native source cache.
Rebuild the LSP/runner/archive, update their exact sizes and hashes and record
three independent marketplace-shaped non-Mock translations with unchanged
source bytes.

## 6. Run Full Quality Gates

```bash
make workspace-storage-check
make worktree-audit
make format
make fmt
make clippy
make deny
make test
make test-repository-boundary
make test-marketplace-foundation
make test-marketplace-contract
make test-marketplace-acquisition
make test-marketplace-native-supply-chain
make test-marketplace-package
make test-marketplace-release-contents
make test-marketplace-offline
git diff --check
```

Expected: all project-controlled gates pass against the post-cleanup tree and
post-cleanup package identities.

## 7. Check Public Release State

```bash
make marketplace-release-check
```

Before publishing the new exact candidate, the only accepted failure is the
explicit absence of the public tag/asset. No local test may represent this as a
release success.

After review: publish the regenerated tag/asset, rerun the check, perform the
exact-package interactive Zed acceptance, submit upstream and finally run the
three clean Gallery installations required by feature 009.

## 8. Optional Deep Cleanup

```bash
make clean-deep-preview
make clean-deep CONFIRM=remove-reproducible-caches
```

Deep cleanup occurs only after package evidence is safely recorded. Its preview
must name every cache path and prove that source, `.agents/`, `.codex/`, `.git/`,
secrets and persistent data are absent from the selection.
