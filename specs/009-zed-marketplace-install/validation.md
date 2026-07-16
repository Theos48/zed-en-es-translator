# Validation: Plug-and-Play Zed Marketplace Installation

**Feature branch**: `009-zed-marketplace-install`
**Implementation commit**: `bd26ecf`
**Project PR**: [Theos48/zed-en-es-translator#15](https://github.com/Theos48/zed-en-es-translator/pull/15) (merged)
**Convergence PR**: [Theos48/zed-en-es-translator#17](https://github.com/Theos48/zed-en-es-translator/pull/17) (merged)
**Validated**: 2026-07-16
**Supported release profile**: Linux `x86_64`, English to Spanish
**Result**: The project-controlled implementation, package and public-release
gates pass after feature-010 convergence. Interactive Zed and Gallery gates
remain open and are not represented as local successes.

## Exact Post-Convergence Candidate Identities

These are the only candidate identities eligible for exact-package acceptance
after feature 010 removed the retired CLI, MCP and configurable-provider source.

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `translator-lsp` | 2,018,176 | `45218fd230fb2d072ae5528be09e583c2eaf671785a29dad327d7566507491ec` |
| `translator-embedded-runtime` | 11,898,352 | `d69ffa86ff42166afb9ffe59947dea727a9cd9856177a392d35091b97e8614ac` |
| Release archive | 5,548,286 | `9cddf1ede9a19e2e5ad6cdf1c3c775d218cdc455fc27462c8922e6ffd19108d3` |

The complete active package measures 50,511,299 bytes, below the 128 MiB
budget. The three Mozilla model resources measured 24,122,452 compressed bytes
and 36,576,277 installed bytes. The converged build reproduced the locked
runner identity and regenerated the archive from the reduced LSP.

## Public Release Evidence

- Annotated tag `v0.1.0` resolves to convergence/fix merge
  `fb2d76c01cd51b57fbe8d6c904518d09588d8356`.
- [Release workflow 29518324568](https://github.com/Theos48/zed-en-es-translator/actions/runs/29518324568)
  passed its repository-boundary, native supply-chain, reproducibility and
  immutable-publication gates.
- [GitHub release `v0.1.0`](https://github.com/Theos48/zed-en-es-translator/releases/tag/v0.1.0)
  was published as a non-draft, non-prerelease release on 2026-07-16.
- Public asset `en-es-translator-0.1.0-linux-x86_64.tar.gz` is 5,548,286 bytes
  with SHA-256
  `9cddf1ede9a19e2e5ad6cdf1c3c775d218cdc455fc27462c8922e6ffd19108d3`,
  exactly matching `ops/marketplace/package.lock.json`.
- The published checksum sidecar is 136 bytes. GitHub reports its SHA-256 as
  `2009ef4b469514642fe7cb15379eacee3416eace8535fccf7902be55130ed898`.

## Pre-Submission Dev-Extension Smoke

This supporting smoke passed on 2026-07-16 with Zed 1.11.3 on Linux `x86_64`:

- Zed used an isolated, workspace-ignored user-data directory and compiled the
  extension WASM successfully through the existing `rustup` toolchain.
- The extension acquired and activated verified package
  `en-es-translator-0.1.0-linux-x86_64` in its Zed-owned work directory.
- Zed started `translator-lsp` from that work directory. The installed LSP and
  runner SHA-256 values were respectively
  `45218fd230fb2d072ae5528be09e583c2eaf671785a29dad327d7566507491ec`
  and
  `d69ffa86ff42166afb9ffe59947dea727a9cd9856177a392d35091b97e8614ac`,
  matching the post-convergence package identities above.
- The direct offline action produced the Spanish hover preview for the public
  Markdown fixture. Its Git blob identity remained
  `a9d9dd6344c28eeb72b64073853cb388442727a5` before and after the action.

This smoke reduces submission risk but does not close T057, FR-026 or the 3/3
clean Gallery acceptance because it intentionally used a development extension.

## Central Registry Submission Evidence

- [zed-industries/extensions#6843](https://github.com/zed-industries/extensions/pull/6843)
  submits extension version `0.1.0` at path `zed-extension` through HTTPS
  submodule commit `0e5d7f5a7be9cd4b530a9a0039981230e071d80e`.
- Upstream submission commit is
  `970a532b0722f322fb6aca89cd53a6b191c09c1a` on the public fork branch.
- [Upstream package run 29522063923](https://github.com/zed-industries/extensions/actions/runs/29522063923)
  passed extension packaging, sorted-manifest and no-Git-LFS gates in 33
  seconds.
- [Upstream Danger run 29522063706](https://github.com/zed-industries/extensions/actions/runs/29522063706)
  passed.
- `verification/cla-signed` is the only failing check. Zed requires account
  `Theos48` to sign the contributor agreement at <https://zed.dev/cla> and then
  request `@cla-bot check` on the PR. This legal acceptance cannot be completed
  by the agent.

## Automated Gate Matrix

| Gate | Result | Evidence |
|---|---|---|
| `make workspace-storage-check` | PASS | Checkout is on persistent `btrfs` storage. |
| `make worktree-audit` | PASS | One registered worktree; no `tmpfs`/`ramfs` checkout. |
| `make format` and `make fmt` | PASS | Workspace and extension crate formatted and checked in the pinned container. |
| `make clippy` | PASS | Workspace and extension pass with warnings denied. |
| `make deny` | PASS | Advisories, bans, licenses and sources pass; duplicate transitive versions are warnings only. |
| `git diff --check` and shell syntax checks | PASS | No whitespace errors; marketplace/native integration shell scripts parse. |
| `make test` | PASS | The converged core/LSP workspace and isolated extension suite pass. |
| `make test-marketplace-foundation` | PASS | Runner wire/limits plus embedded runtime boundaries pass. |
| `make test-marketplace-native-supply-chain` | PASS | Locked source, nested gitlinks, offline build, CPU/ELF and reproducibility checks pass. |
| `make test-marketplace-contract` | PASS | Final package lock, no-setup manifest, extension and embedded LSP contracts pass. |
| `make test-marketplace-acquisition` | PASS | Clean preparation, concurrency, failure, rollback, state and unsupported-platform cases pass. |
| `make test-marketplace-package` | PASS | Exact archive members, modes, sizes and hashes pass. |
| `make test-marketplace-release-contents` | PASS | Size, executable allowlist, notices, source offer and pinned Zed removal behavior pass. |
| `make test-marketplace-offline` | PASS | Privacy, 3-run real smoke and 20-case real network-disabled benchmark pass. |
| `make marketplace-release-check` | PASS | Public `v0.1.0` tag, immutable asset URL, version, size and SHA-256 exactly match the package lock. |

One early foundation invocation returned a redacted process failure. The same
gate then passed, the embedded-runtime binary passed 50 consecutive
repetitions, and it passed again in the complete and final focused suites. The
failure did not recur; no speculative product change was made from a single
non-reproducing observation.

## User-Story Evidence

### Automatic preparation and real translation

- Controlled first-use acceptance: 20/20 preparations succeeded under the
  configured 10 Mbps transport, exceeding the required 19/20 threshold.
- Marketplace-shaped exact package smoke: 3/3 independent clean work
  directories returned real local output, required no binary/runtime setting
  and preserved the public fixture hash.
- The three runs above validate the package and automatic path, not Gallery
  installation. They do not satisfy SC-001 or FR-027 until the central registry
  distributes the extension.

### Recovery and unsupported platforms

- Truncation, oversize, wrong hash, invalid Zstandard content, storage failure,
  interruption, stale staging, concurrent acquisition and invalid update all
  activated zero invalid packages.
- Last-known-good fallback and retry without manual deletion passed.
- Every non-Linux-`x86_64` Zed OS/architecture combination covered by the
  extension API stopped before downloader or storage mutation and returned the
  same in-editor message.

### Offline, privacy and resource limits

- Real benchmark: 20/20 cases passed, with each runner container using
  `--network none`.
- Worst observed wall time, including container startup: 512 ms, below the
  15-second request deadline.
- Peak observed RSS: 183,412 KiB, below 1 GiB.
- Maximum observed runner thread count: 1, below the four-thread budget.
- Public fixture hash was unchanged. Existing Markdown, code/link preservation,
  unsafe path/type, non-UTF-8, binary and no-mutation suites also passed in
  `make test`.
- Acquisition requests are fixed public artifact requests; readiness and
  inference contain no downloader and diagnostics expose no source,
  translation, credentials, raw child output or sensitive paths.

## Manual and External Gates

The interactive and Gallery gates remain intentionally open:

| Task | State | Required action |
|---|---|---|
| T057 exact-package interactive Zed acceptance | BLOCKED | After the upstream registry entry is available, install from Gallery in a clean supported Zed profile and record only public fixture/version/outcome evidence. A dev extension or repository binary cannot replace this gate. |
| T058 public tag and release asset | PASS | The reviewed `v0.1.0` release workflow published the exact locked archive and checksum; `make marketplace-release-check` passed against them. |
| T060 central registry and Gallery acceptance | SUBMITTED / USER+EXTERNAL | Upstream PR #6843 exists and technical checks pass. The contributor must sign Zed's CLA; maintainer merge and the subsequent T057 plus two additional clean Gallery installations remain external. |

No product decision remains, but the contributor must accept Zed's CLA. After
the CLA check and maintainer merge, perform T057 and record 3/3 clean Gallery
runs. Until those steps pass,
FR-001, FR-026, FR-027, SC-001, SC-009 and SC-010 remain publication-level
open gates even though their local contracts are implemented.

## Spec Kit Gates

- `speckit-specify`: PASS; specification and requirements-quality checklist
  exist.
- `speckit-clarify`: PASS; prerequisite executed and no critical ambiguity
  remained.
- `speckit-checklist`: PASS; 43/43 marketplace requirement items reviewed.
- `speckit-plan`: PASS; research, data model, contracts, quickstart and agent
  context generated.
- `speckit-tasks`: PASS; dependency-ordered tasks generated after planning.
- `speckit-analyze`: PASS before and after implementation; final analysis found
  42/42 requirements covered by tasks, with no ambiguity, duplication,
  constitution conflict or unmapped task.
- `speckit-implement`: PASS for project-controlled implementation, tests and
  public project release; T057 and T060 remain open above.
- `speckit-converge`: PASS; no missing, partial, contradictory or unrequested
  work was found outside the tracked publication tasks, so no
  empty convergence phase was appended.
