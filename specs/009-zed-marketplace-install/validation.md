# Validation: Plug-and-Play Zed Marketplace Installation

**Feature branch**: `009-zed-marketplace-install`
**Implementation commit**: `bd26ecf`
**Project PR**: [Theos48/zed-en-es-translator#15](https://github.com/Theos48/zed-en-es-translator/pull/15) (draft)
**Validated**: 2026-07-16
**Supported release profile**: Linux `x86_64`, English to Spanish
**Result**: Project implementation and package gates pass. Public-release,
interactive Zed and Gallery gates remain open and are not represented as local
successes.

## Exact Candidate Identities

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `translator-lsp` | 4,871,592 | `1e76ce116cfe7873fd293c23ec3b0143a89df36b48ab8f7ddb71c6484be1779d` |
| `translator-embedded-runtime` | 11,898,352 | `d69ffa86ff42166afb9ffe59947dea727a9cd9856177a392d35091b97e8614ac` |
| Release archive | 6,747,369 | `48cb992e36e5e43e7eb6352f73e0e3d60c4268c7d7bbb54ea0ad87a090e38754` |

The complete active package measured 53,364,715 bytes, below the 128 MiB
budget. The three Mozilla model resources measured 24,122,452 compressed bytes
and 36,576,277 installed bytes. Two clean native builds produced the locked
runner identity. Two consecutive archive assemblies from the final binaries
produced the same archive identity.

## Automated Gate Matrix

| Gate | Result | Evidence |
|---|---|---|
| `make workspace-storage-check` | PASS | Checkout is on persistent `btrfs` storage. |
| `make worktree-audit` | PASS | One registered worktree; no `tmpfs`/`ramfs` checkout. |
| `make format` and `make fmt` | PASS | Workspace and extension crate formatted and checked in the pinned container. |
| `make clippy` | PASS | Workspace and extension pass with warnings denied. |
| `make deny` | PASS | Advisories, bans, licenses and sources pass; duplicate transitive versions are warnings only. |
| `git diff --check` and shell syntax checks | PASS | No whitespace errors; marketplace/native integration shell scripts parse. |
| `make test` | PASS | Complete core, CLI, MCP, LSP and extension test suites pass. |
| `make test-marketplace-foundation` | PASS | Runner wire/limits plus embedded provider/configuration boundaries pass. |
| `make test-marketplace-native-supply-chain` | PASS | Locked source, nested gitlinks, offline build, CPU/ELF and reproducibility checks pass. |
| `make test-marketplace-contract` | PASS | Final package lock, no-setup manifest, extension and embedded LSP contracts pass. |
| `make test-marketplace-acquisition` | PASS | Clean preparation, concurrency, failure, rollback, state and unsupported-platform cases pass. |
| `make test-marketplace-package` | PASS | Exact archive members, modes, sizes and hashes pass. |
| `make test-marketplace-release-contents` | PASS | Size, executable allowlist, notices, source offer and pinned Zed removal behavior pass. |
| `make test-marketplace-offline` | PASS | Privacy, 3-run real smoke and 20-case real network-disabled benchmark pass. |
| `make marketplace-release-check` | BLOCKED | Correctly fails with `public project tag is absent`; `v0.1.0` has not been published. |

One early foundation invocation returned a redacted process failure. The same
gate then passed, the embedded-provider binary passed 50 consecutive
repetitions, and it passed again in the complete and final focused suites. The
failure did not recur; no speculative product change was made from a single
non-reproducing observation.

## User-Story Evidence

### Automatic preparation and real translation

- Controlled first-use acceptance: 20/20 preparations succeeded under the
  configured 10 Mbps transport, exceeding the required 19/20 threshold.
- Marketplace-shaped exact package smoke: 3/3 independent clean work
  directories returned non-Mock output through `embedded_local`, required no
  binary/provider setting and preserved the public fixture hash.
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
- Worst observed wall time, including container startup: 500 ms, below the
  15-second request deadline.
- Peak observed RSS: 194,648 KiB, below 1 GiB.
- Maximum observed runner thread count: 1, below the four-thread budget.
- Public fixture hash was unchanged. Existing Markdown, code/link preservation,
  unsafe path/type, non-UTF-8, binary and no-mutation suites also passed in
  `make test`.
- Acquisition requests are fixed public artifact requests; readiness and
  inference contain no downloader and diagnostics expose no source,
  translation, credentials, raw child output or sensitive paths.

## Manual and External Gates

These gates remain intentionally open:

| Task | State | Required action |
|---|---|---|
| T057 exact-package interactive Zed acceptance | BLOCKED | After the public candidate exists, use a clean supported Zed profile and record only public fixture/version/outcome evidence. UI interaction cannot be replaced by a dev-extension or repository binary test. |
| T058 public tag and release asset | BLOCKED | Merge the project PR, publish signed/tagged `v0.1.0` through the prepared workflow, then rerun `make marketplace-release-check`. Publishing before review would bind the release URL to unmerged code. |
| T060 central registry and Gallery acceptance | BLOCKED | Submit the exact HTTPS submodule/version change to `zed-industries/extensions`; after maintainer merge, run 3/3 independent clean Gallery installations. |

No product decision is needed to proceed. The remaining order is fixed by the
release contract: merge project PR, publish the exact asset, pass the public
release check, perform the exact-package Zed acceptance, submit upstream, wait
for registry merge, then record 3/3 clean Gallery runs. Until those steps pass,
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
- `speckit-implement`: PASS for project-controlled implementation and tests;
  publication tasks remain open above.
- `speckit-converge`: PASS; no missing, partial, contradictory or unrequested
  work was found outside the four already tracked publication tasks, so no
  empty convergence phase was appended.
