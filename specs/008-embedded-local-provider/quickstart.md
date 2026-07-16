# Quickstart: Embedded Local Provider

**Feature**: `008-embedded-local-provider`
**Date**: 2026-07-15
**Gate**: `BLOCKED_LICENSE_APPROVAL`; controlled implementation available, real activation disabled

## What this path is

The implemented `embedded_local` mode is designed to translate English to Spanish on the
workstation through a verified native helper and model resources. Normal
translation requires no account, API key, remote service, provider container
or long-running provider daemon. Mock remains the default.

F012 does not publish or bundle the model. The first implementation path builds
the native runner reproducibly with the project's existing containerized build
tooling and acquires exact model resources only after explicit consent into a
fixed user-scoped XDG store. After preparation, readiness, translation,
verification and rollback are offline.

## Current planning status

The feature has built Bergamot/Mozilla as the single candidate prototype, not
as a promoted supported provider. The runner build is reproducible and its CPU
baseline and ELF closure pass. Artifact-level human legal/provenance approval
is still absent, so the manifest fails closed before acquisition. Resource,
latency, zero-network, CLI and Zed real-model gates therefore remain unrun.

The existing Mock and LibreTranslate workflows remain the working paths while
implementation is pending or if any mandatory gate fails.

## Preparation flow (currently blocked before acquisition)

### 1. Inspect before any acquisition

```bash
make provider-embedded-disclose
```

The command shows only bounded safe metadata. While review is blocked it emits
`consent_available=false` and a review-lock digest, not an activation consent:

- English-to-Spanish profile and Fedora `x86_64` support scope;
- exact source/version identities and license conclusions;
- expected transfer, installed and worst-case lifecycle storage;
- fixed user-scoped destination class, not a sensitive absolute path;
- network use limited to this authorized acquisition;
- publication/bundling limitations;
- full manifest SHA-256 to use as consent.

It must not download, build, unpack, activate or modify provider state.

### 2. Accept the exact reviewed manifest

```bash
make provider-embedded-prepare CONSENT=<manifest-sha256>
```

Omitting the digest, declining or supplying a different value must leave the
active state unchanged. Preparation may contact only the exact allowlisted
artifact URLs and must verify compressed and installed hashes, sizes,
compatibility, language pair and license/provenance completeness before an
offline smoke and atomic activation.

No `sudo`, host package installation, system service or provider container is
part of this lifecycle. The repository may still use its existing Docker build
image to produce/test project binaries, as it does for all Rust/native builds;
Docker is not the provider runtime.

## Safe operations

```bash
make provider-embedded-status
make provider-embedded-verify
make provider-embedded-update CONSENT=<new-manifest-sha256>
make provider-embedded-rollback
make provider-embedded-clean CONFIRM=remove-embedded-provider-data
```

- `status` and `verify` are offline and content-free.
- `update` requires disclosure and new consent; candidate cannot replace
  current until every gate passes.
- `rollback` re-verifies previous and restores it offline.
- `provider-embedded-clean` is the only full-removal command. Generic
  `make clean` must preserve provider state.
- cleanup fails safely if artifacts are in use or ownership is ambiguous.

## CLI use after an approved preparation

Mock remains the no-configuration default:

```bash
make translator-cli-release
printf '%s' '{"text":"Public synthetic text.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"]}' \
  | target/release/translator-cli
```

After a verified set exists, explicit embedded selection uses only the
controlled provider key:

```bash
TRANSLATOR_PROVIDER=embedded_local \
  target/release/translator-cli <<'JSON'
{"text":"Public synthetic text.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve":["markdown_structure","code","links"]}
JSON
```

No executable path, model path, URL, API key, remote flag or arbitrary argument
is accepted for embedded mode. A missing/unready set returns a normalized
failure; it does not fabricate Mock success or contact another provider.

## Direct Zed use after an approved preparation

1. Build/prepare the direct development extension through the existing project
   target.
2. Set only the controlled provider value under
   `lsp.en-es-translator.binary.env`:

   ```json
   {
     "TRANSLATOR_PROVIDER": "embedded_local"
   }
   ```

3. Restart the language server after changing selection.
4. Request the existing translation code action on public synthetic Markdown
   or Plain Text.
5. Check that the action identifies the request as `[offline]` before execution
   and that the read-only hover preview appears without edits.

The extension must not start preparation or a download during language-server
startup. Artifact consent happens only through the explicit preparation flow.

## Acceptance run after approval

Before compiling, use the required storage guard:

```bash
make workspace-storage-check
```

The implementation provides controlled/native targets. Real-model acceptance
must additionally execute:

1. automated core/config/runner/lifecycle/CLI/LSP/Zed negative coverage with
   controlled doubles;
2. native source/build, CPU-baseline, ELF dependency and license/SBOM checks;
3. 20 public synthetic real-model cases using the fixed benchmark method;
4. one real CLI and one real direct-Zed run with external networking disabled;
5. invalid candidate, interrupted update and offline rollback scenarios;
6. redaction and non-mutation inspection;
7. exact cleanup-scope validation.

Actual target names beyond the lifecycle interface are fixed during task
generation/implementation. Tests must run through Make and the project
container; do not install Rust, C++, Python or benchmark tooling globally on
Fedora for this repository.

## Resource and privacy envelope

Mandatory go/no-go budgets are:

- transfer <=64 MiB;
- active installed set <=128 MiB;
- full lifecycle storage <=384 MiB, with 512 MiB free before preparation;
- peak RSS <=1 GiB and inference threads <=4;
- cold readiness <=10 seconds, warm mixed p95 <=5 seconds and every provider
  request under the existing 15-second deadline;
- zero attempted external contacts after preparation.

The reviewed upstream model resources are approximately 23.00 MiB transferred
and 34.88 MiB installed. The reproducible runner candidate is 12,000,008 bytes;
runtime memory, thread and latency results are not measured and must not be
inferred from artifact sizes.

Logs and evidence may contain safe manifest identities, sizes, timings,
locality, synthetic case IDs and normalized outcomes only. They never contain
input, translations, model bytes, raw child output, environment values,
sensitive URLs or paths.

## Recovery and no-go behavior

- Failed preparation/update leaves current unchanged.
- Missing or invalid previous state makes rollback fail without deleting
  current.
- A timeout kills and reaps the helper; there is no retry.
- An explicitly selected broken embedded provider fails closed.
- If provenance/license, build, portability, resource, latency, offline,
  quality or lifecycle gates cannot be satisfied, the feature records the
  blocking gate and keeps Mock and LibreTranslate available.
- Publication remains a separate F009 decision even after technical success.
