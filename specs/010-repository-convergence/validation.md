# Validation: Repository Convergence and Cleanup

## Baseline (2026-07-16)

Captured before tracked source removal, after the governance-only constitution
amendment. Counts describe the working tree content, not only `HEAD`.

| Measure | Baseline | Evidence |
|---|---:|---|
| Root workspace members | 4 | core, CLI, LSP and MCP in root `Cargo.toml` |
| Tracked paths | 368 | `git ls-files` |
| Tracked lines | 42,864 | `git ls-files -z \| xargs -0 wc -l` |
| Make target declarations | 53 | unique target declaration scan |
| Top-level integration scripts | 33 | `tests/integration/*.sh` |
| Integration/helper scripts | 35 | audit includes two scripts under `tests/integration/lib/` |
| Root lock packages | 169 | `[[package]]` entries in `Cargo.lock` |
| Root Rust build output | 31 GiB | `target/` |
| Extension Rust build output | 1.3 GiB | `zed-extension/target/` |
| Local Zed validation output | 157 MiB | `.cache/zed-local-validation/` |
| Stale extension WASM | 996 KiB | `zed-extension/extension.wasm` |
| Public `v0.1.0` tag | absent | `git tag -l v0.1.0` returned empty |

`make workspace-storage-check` passed on persistent `btrfs` storage.
`make worktree-audit` passed with one registered persistent worktree. The only
pre-existing working-tree changes were the feature-010 planning artifacts and
their managed Spec Kit/AGENTS pointers; no unrelated user edits were detected.

## Governance Gate

Constitution 2.0.0 passed its amendment checks:

- major bump is justified by retirement of required CLI/MCP/provider boundaries;
- no unexplained placeholder remains;
- ratification date is preserved and amendment date is `2026-07-16`;
- plan/spec/tasks templates use the Gallery/LSP/private-runner boundary;
- ADR 0007 and D092 record the stable replacement decision.

## Checklists Before Implementation

| Checklist | Total | Completed | Incomplete | Status |
|---|---:|---:|---:|---|
| `requirements.md` | 16 | 16 | 0 | PASS |
| `cleanup-safety.md` | 37 | 37 | 0 | PASS |

## Invariant Migration Map

| Live invariant | Retired surface that may disappear | Retained evidence |
|---|---|---|
| No source/buffer mutation | MCP tool and old wrapper/Agent tests | `crates/translator-core/tests/no_source_file_mutation.rs`, LSP code-action/preview tests, `tests/integration/marketplace_no_setup.sh` |
| Markdown/code preservation | MCP contract and CLI snapshots | `crates/translator-core/tests/markdown_preservation.rs`, `markdown_tricky_preservation.rs`, `ambiguous_content.rs`, `document_snapshot.rs` |
| Input, segment and output limits | CLI/MCP limit mapping | Core direct-text, segment and embedded-provider limit tests plus marketplace benchmark/resource gate |
| Workspace/path safety | MCP `translate_file` mapping | Core `file_validation.rs`, `workspace_path_attacks.rs`, `workspace_symlink_escape.rs`, `sensitive_file_denials.rs` |
| UTF-8 and binary denial | CLI/MCP serialization/error tests | Core `contract_utf8.rs`, `file_encoding_attacks.rs` and LSP snapshot validation tests |
| Redacted diagnostics | CLI stderr, MCP error and remote-provider tests | Core `error_redaction_matrix.rs`, embedded-process/provider tests, extension `diagnostics_redaction.rs` and marketplace privacy gate |
| Bounded child process | CLI timeout/provider timeout tests | Core `embedded_process.rs`, `embedded_provider.rs` and `embedded_runner_boundary.rs` tests |
| Local/offline translation | LibreTranslate/Azure operational tests | Adjacent embedded-provider tests and `marketplace_offline_privacy.sh`, `marketplace_real_smoke.sh` |
| No configurable provider | CLI/LSP provider configuration and remote confirmation tests | Repository-boundary negative scan plus direct embedded LSP construction tests |
| Exact acquisition/package identity | Manual wrapper preparation tests | Zed extension acquisition/package tests and marketplace acquisition/package/supply-chain gates |
| Zed-owned removal/recovery | Provider-manager cleanup and wrapper tests | Extension rollback/state tests and `marketplace_removal_contract.sh` |

Pure MCP protocol/session/schema behavior, CLI JSON/exit behavior, remote
consent, Azure/LibreTranslate HTTP semantics and provider-manager lifecycle are
not live product invariants and receive no replacement test.

## Expected Pre-Removal Boundary Failure

`make test-repository-boundary` was executed before any source removal. It
failed as required with 54 content-free findings and exit status 2 from Make:

```text
repository_boundary status=fail findings=54
make: *** [Makefile:93: test-repository-boundary] Error 1
```

The findings cover unexpected root members, retired crates/modules/scripts,
runtime configuration, Make targets and current documentation. `bash -n` and
`git diff --check` pass for the new contract. This observed failure unlocks the
three parallel removal workstreams.

## Automation Convergence

The automation workstream reduced the repository interface without removing a
retained marketplace or worktree gate:

| Measure | Baseline | Converged |
|---|---:|---:|
| Make targets | 53 observed / 52 audited | 34 |
| Files under `scripts/` | 11 | 9 |
| Integration scripts | 35 including helpers | 12 |
| Test fixtures | 20 | 3 |
| GitHub automation files | 4 | 4 |

`zed-direct-server-release` was renamed to `marketplace-lsp-release` and every
package consumer was updated. `make help`, shell syntax, ShellCheck,
`marketplace_no_setup.sh`, `make test-worktree-storage`, normal/deep cleanup
previews and `git diff --check` pass. Normal and deep cleanup were not executed
during this stream.

## Documentation and History Convergence

The documentation stream removed 81 files from completed Spec Kit cycles
001-007 and six obsolete operational/strategy documents after migrating their
live constraints. Git remains the exact archive. Current guidance now names
only the Gallery/LSP/core/embedded-runtime path, while ADRs 0001-0005 carry
explicit supersession status and ADR 0006 remains accepted.

Feature 009 no longer permits compatibility retention and identifies its old
package identities as pre-convergence evidence. Its live result schema was
migrated to `specs/009-zed-marketplace-install/contracts/translate-result.schema.json`.
The retained Markdown link check passes, and current documentation produces
zero retired operational-term findings. The exact historical allowlist is
limited to `docs/decisions.md`, `docs/feature-map.md`, ADRs 0001-0007 and the
feature-010 removal specification.

## Source Convergence

The direct LSP contract was observed failing before implementation: both new
tests failed because `main.rs` selected a provider indirectly through runtime
configuration. After the source wave, the LSP constructs
`EmbeddedProcessProvider::from_current_executable()` directly and exposes no
provider, endpoint, credential, remote-confirmation or arbitrary-binary input.

CLI and MCP crates plus Azure, LibreTranslate and dynamic provider modules were
removed. The root workspace now contains exactly core and LSP. Cargo regenerated
the lock through `make workspace-lock`: package entries decreased from 169 to
59 and contain no `translator-cli`, `translator-mcp`, `rmcp`, `schemars`,
`tokio` or `ureq` package.

`make test` passes the retained core/LSP suite and the isolated extension suite,
including direct adjacent-provider, read-only preview, Markdown, limits,
workspace/path, UTF-8/binary, bounded process, redaction, acquisition, package
state and unsupported-platform coverage. The extension no longer injects any
runtime provider environment variable.

## Generated-Residue Cleanup

The normal preview selected only the root/nested Rust targets, stale removed-
crate targets, extension WASM/target, local Zed validation output and bounded
project temporary output. It explicitly preserved Cargo/home caches, the fixed
`.cache/embedded-source/` checkout, `.agents/`, `.codex/`, `.git/` and
persistent/provider data.

`make clean` removed the approximately 32.5 GiB audited residue and pruned the
now-empty CLI, MCP and provider-manager directories. A second normal preview
reports every eligible path absent, while `make test-repository-boundary`
passes with zero findings.

`make clean-deep-preview` lists only `.cache/cargo`, `.cache/home` and
`.cache/embedded-source`, repeats the prohibited path set and requires the
exact token `remove-reproducible-caches`. Deep cleanup was intentionally not
executed; fixed release inputs remain available for exact package rebuilding.

## Exact Post-Convergence Package

The retained package was rebuilt from the normal-clean state and its lock and
feature-009 evidence were updated to the following exact identities:

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `translator-lsp` | 2,018,176 | `45218fd230fb2d072ae5528be09e583c2eaf671785a29dad327d7566507491ec` |
| `translator-embedded-runtime` | 11,898,352 | `d69ffa86ff42166afb9ffe59947dea727a9cd9856177a392d35091b97e8614ac` |
| Release archive | 5,548,286 | `9cddf1ede9a19e2e5ad6cdf1c3c775d218cdc455fc27462c8922e6ffd19108d3` |

`make test-marketplace-package` verified every archive member, mode, size and
hash. `make test-marketplace-offline` passed its real network-disabled suite:
20/20 benchmark cases, 183,412 KiB peak RSS and one runner thread. A focused
rerun recorded 3/3 independent marketplace-shaped translations with
`provider=embedded_local` and `source_mutation=none`; no Mock or retired
provider path participated.

## Final Quality Gates

One dependency-aware Make invocation executed the complete project-controlled
gate set after convergence. Storage and worktree guards, `format`, `fmt`,
Clippy with warnings denied, both dependency audits, root and extension tests,
repository boundary, marketplace foundation, contract, acquisition, native
supply chain, exact package, release contents and real offline gates all pass.
`git diff --check` also passes.

The public release check was repeated from the authorized host environment to
exclude the sandbox's DNS limitation. It has the only accepted pre-publication
result: `public project tag is absent`. No tag, release asset, upstream Gallery
submission or interactive acceptance was created or represented as successful.

## Retained-Surface and Reduction Audit

- 194 obsolete tracked paths are deleted, exceeding the 150-path target;
- 25,119 tracked lines are deleted and 981 are added, before counting the 18
  intentional new/untracked source, documentation and planning paths;
- the prospective tracked tree contains 192 paths, down from 368;
- the root workspace has two members, the lock has 59 packages, Make has 34
  targets and integration/helper scripts decreased from 35 to 12;
- all remaining top-level families and their individual consumers map to
  `contracts/retained-surface.md`: product runtime, package/supply chain,
  repository quality or documentation/governance;
- `make test-repository-boundary` reports zero findings and the current
  README/docs/spec/Spec Kit Markdown link audit reports zero broken links;
- fixture-only broken links remain deliberate translation-preservation input
  and are not documentation backlinks.

## Rollback and Backup Audit

Rollback is the reviewable Git diff and its eventual atomic commit sequence.
No hidden copy, untracked archive, backup file, temporary worktree or parallel
clone was created. Generated package archives live only under ignored
`target/` while validated and are reproducible from locks; they are not a
rollback mechanism. The exact source/model caches were preserved by normal
cleanup, and deep cleanup remains unexecuted behind its explicit confirmation
token.

## Spec Kit Convergence

The required prerequisite passed with the feature directory and tasks
available. `speckit-converge` assessed 22 functional/security/test
requirements, 9 measurable outcomes, 12 acceptance scenarios, all 5
constitutional principles, the plan decisions, 35 tasks and the implemented
source/automation/documentation tree.

No missing, partial, contradictory or unrequested work was found. The only
open task during assessment was T034 itself, so no convergence tasks were
appended. After synchronizing README, roadmap and feature status, all 35/35
tasks are complete and the feature status is `Complete`.

After all hashes and gate evidence were recorded, normal cleanup ran once more
and removed the rebuilt root/core/extension targets, exact package staging and
real-smoke output. The final checkout therefore retains no generated build or
package artifact; only the explicitly preserved reproducible Cargo/home and
locked native-source caches remain. A final normal preview reports every
eligible output absent. During the subsequent ignore audit, the user explicitly
requested removal of every unnecessary repository entry; the confirmed-empty
zero-byte `provider-cache/` directory and empty retired-spec directories were
therefore removed without touching persistent data.
