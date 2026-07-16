# Quickstart: Validate Plug-and-Play Marketplace Installation

This guide is for maintainers validating the feature. It is not an installation
guide for users. The user-facing path is only: install from Zed's Extension
Gallery, open Markdown/plain text and invoke the translation action.

## Prerequisites

- Docker available to the project Makefile.
- Persistent checkout storage accepted by the workspace guard.
- Network only for the explicit source/model acquisition and Gallery test.
- A supported Linux `x86_64` machine for real package evidence.
- Zed for the pre-submission smoke and final interactive gates; project builds
  do not install Rust/C++ toolchains on the host. The dev-extension smoke uses
  an existing `rustup` toolchain or an isolated development environment as
  described in `docs/deployment.md`.

No real secret, API key, `.env`, source document or private fixture is used.

## 1. Validate Workspace and Contracts

```bash
make workspace-storage-check
make worktree-audit
make test-marketplace-contract
```

Expected:

- the package lock matches its JSON schema;
- the manifest contains no binary-path/runtime setting or mutable/latest URL;
- every package source is fixed HTTPS and every resource has exact size/hash;
- the extension-specific MIT license and required notice/source paths exist;
- unsupported platforms are represented as zero-download failures.

The bounded embedded-runtime foundation can be exercised independently while the
release lock is still intentionally fail-closed:

```bash
make test-marketplace-foundation
make test-marketplace-native-supply-chain
```

The package-lock contract was observed failing first. It is now green against
the exact release-server sizes and SHA-256 values recorded by T027.

## 2. Build the Real Release Package

```bash
make marketplace-package
make test-marketplace-package
```

The build runs in the pinned project container, fetches only locked source,
rebuilds the portable native runner, builds `translator-lsp`, creates the
Linux `x86_64` release archive and verifies it against the package lock.

Expected:

- executable hashes/sizes match the lock;
- model compressed and installed identities match the Mozilla records;
- archive paths are safe and only two files are executable;
- native CPU/ELF checks, notices and corresponding-source instructions pass;
- the active installed package is below 128 MiB.

## 3. Run Automatic Acquisition Failure Tests

```bash
make test-marketplace-acquisition
```

The controlled server covers clean preparation, offline-before-first-use,
truncation, oversize, wrong hash, invalid Zstandard data, insufficient storage,
interruption, stale staging, two concurrent installers and failed update with
last-known-good fallback.

It also runs 20 clean preparations at a controlled 10 Mbps and requires at
least 19 to reach a usable translation within five minutes.

Expected: no invalid package becomes active or executable; a normal retry
recovers without editing a path or deleting files manually.

## 4. Run Real Offline Translation Evidence

```bash
make test-marketplace-offline
```

This target uses the real locked native runner and exact public `en -> es`
resource set. Acquisition is complete before the network-disabled test phase.
It runs 20 public fixtures through the direct embedded-runtime/LSP boundary and records
only case IDs, pass/fail and resource metrics.

Expected:

- 20/20 cases produce non-mock Spanish output;
- every request is below 15 seconds;
- no network is attempted during readiness/translation;
- source fixture bytes and Markdown/code regions remain unchanged;
- peak RSS is below 1 GiB and inference uses no more than four threads;
- logs contain no source/translation content or sensitive paths.

## 5. Run Full Repository Quality Gates

```bash
make test
make fmt
make clippy
make deny
git diff --check
```

Expected: every retained core, LSP, extension and marketplace test passes. The
repository-boundary gate confirms that no retired executable or configurable
runtime surface remains.

## 6. Validate the Tagged Public Release

```bash
make marketplace-release-check
```

Expected: the public tag, release asset URL, `extension.toml` version,
extracted identities, package lock, notices and repository commit agree. No
user documentation contains a terminal, Docker, runtime or binary-path setup
step.

## 7. Submit to the Central Registry

After all project-controlled release gates pass, submit the exact HTTPS
submodule/path/version entry to `zed-industries/extensions` and record the PR
and check state. A dev-extension smoke can reduce submission risk, but cannot
be reported as clean-install acceptance.

## 8. Clean Gallery Acceptance

Only after the `zed-industries/extensions` submission is available through the
normal registry:

1. use a clean supported Zed profile with no project checkout, configured LSP
   binary, runtime setting or development extension;
2. install English to Spanish Translator from the Extension Gallery;
3. open the public Markdown fixture and invoke the direct action;
4. record visible checking/downloading behavior and the real Spanish read-only
   preview;
5. disable external networking, restart Zed and run the public 20-case set;
6. confirm source bytes are unchanged;
7. disable the extension and confirm no server/acquisition starts;
8. uninstall in Zed and confirm its extension work directory is removed.

Record only versions, platform class, public fixture IDs, outcomes and resource
metrics in the feature validation record. If the registry PR is awaiting
upstream review, mark only this post-merge gate externally blocked; do not
replace it with a dev-extension result.
