# Phase 0 Research: Plug-and-Play Zed Marketplace Installation

**Feature**: `009-zed-marketplace-install`
**Date**: 2026-07-16
**Status**: Complete for planning

## Decision 1: Use Zed's managed language-server download flow

**Decision**: The published extension is a thin Rust/WASM integration. On the
first supported activation it checks the platform, reports installation state,
downloads the fixed native package into its Zed-owned working directory,
verifies it and returns the `translator-lsp` command. It never asks for a
binary path or project preparation command.

**Rationale**:

- Zed's publishing rules prohibit shipping a language server inside the
  extension package and explicitly require downloading it or discovering it in
  the user's environment.
- `zed_extension_api 0.7.0` exposes `current_platform`, `download_file`,
  `make_file_executable` and language-server installation states.
- Zed's own GLSL extension uses this exact checking/downloading/versioned-work
  pattern.
- Zed documents `work` as storage for extension-created files such as downloaded
  language servers.

**Alternatives considered**:

- Ship the server in `zed-extension/`: rejected by the marketplace rules.
- Require a system binary or configured path: rejected by the confirmed product
  requirement.
- Use an intermediary compatibility workflow as the main flow: rejected
  because the direct LSP action already exists and the user explicitly wants
  an ordinary extension.

**Primary sources**:

- <https://zed.dev/docs/extensions/developing-extensions>
- <https://zed.dev/docs/extensions/installing-extensions>
- <https://docs.rs/zed_extension_api/0.7.0/zed_extension_api/fn.download_file.html>
- <https://github.com/zed-industries/zed/blob/bf14327c27885b8c52588c2bf84cb2bd4f2dd72e/extensions/glsl/src/glsl.rs>

## Decision 2: Let Zed own removal and package storage

**Decision**: All native executables, models, staging and state live below the
extension's work directory. The extension creates no independent XDG root and
offers no terminal cleanup command as part of the product.

**Rationale**: Current Zed source removes both the installed extension
directory and `work/<extension-id>` during uninstall, retrying removal to cover
process shutdown races. This directly satisfies the no-terminal removal path
and prevents the lifecycle from becoming a second application.

**Alternative considered**: Reuse the F012 user-scoped lifecycle manager. It was
safe but wrong for this product: it introduced separate disclosure, consent,
prepare, status, update, rollback and cleanup commands that a Gallery user
should never need to know exist.

**Primary source**:

- <https://github.com/zed-industries/zed/blob/bf14327c27885b8c52588c2bf84cb2bd4f2dd72e/crates/extension_host/src/extension_host.rs#L953-L1015>

## Decision 3: Reuse only compact tested local inference pieces from the prototype

**Decision**: Retain the provisional Mozilla Translations/Bergamot runner,
exact Firefox `en -> es` base-memory resources and Rust bounded child-process
wire. Do not retain the F012 manager or its manual lifecycle.

The exact model set is:

| Role | Transfer bytes | Installed bytes |
|---|---:|---:|
| Model | 22,085,725 | 31,561,787 |
| Vocabulary | 348,996 | 816,054 |
| Lexical shortlist | 1,687,731 | 4,198,436 |
| **Total** | **24,122,452** | **36,576,277** |

The already reproduced Linux runner is 12,000,008 bytes. The complete active
package is therefore expected to stay comfortably below 128 MiB after adding
the Rust LSP and notices; the release gate measures the final result rather
than treating that expectation as evidence.

**Rationale**:

- Firefox uses Bergamot/Marian for on-device translation and publishes a
  direct `en -> es` set with compressed/decompressed sizes and SHA-256 values.
- The prototype already proved a generic `x86-64` build and a killable,
  environment-cleared, bounded one-shot child process.
- ONNX OPUS-MT alternatives require separate encoder and decoder artifacts
  that exceed this feature's 128 MiB package budget even in common quantized
  forms. Other previously evaluated stacks would reintroduce a runtime or
  service that users must manage.

**Alternatives considered**:

- Argos Translate: rejected because the exact `en-es` package still lacks a
  clear upstream artifact license and requires Python/native packaging.
- CTranslate2/ONNX OPUS-MT: deferred because the practical package is larger
  and creates a new conversion/provenance path without simplifying Zed.
- Mandatory remote translation: rejected by the offline/privacy requirement.
- Bergamot WASM inside the Zed extension: rejected because Firefox's artifact
  is an Emscripten/browser integration, not a documented standalone WASI
  component.

**Primary sources**:

- <https://firefox-source-docs.mozilla.org/toolkit/components/translations/resources/01_overview.html>
- <https://firefox-source-docs.mozilla.org/toolkit/components/translations/resources/03_bergamot.html>
- <https://github.com/mozilla/translations/tree/f31423c7c2c6ed8ae57d71a3d19a9db6f156060e>
- <https://firefox.settings.services.mozilla.com/v1/buckets/main/collections/translations-models-v2/records?sourceLanguage=en&targetLanguage=es>
- <https://github.com/Helsinki-NLP/Opus-MT>
- <https://github.com/argosopentech/argos-translate/issues/507>

## Decision 4: Split acquisition from inference

**Decision**: Only the extension WASM contacts fixed public package sources.
`translator-lsp`, `translator-core` and the native runner receive no download
capability. After readiness they operate with external networking disabled.

The extension downloads:

1. one fixed project GitHub release archive containing `translator-lsp`, the
   native runner and the notice/source bundle;
2. three exact Mozilla Zstandard attachments for the model set.

It verifies archive members after extraction, verifies each compressed model,
decodes with a pure-Rust Zstandard reader, verifies installed size/hash and
only then activates the package.

**Rationale**: Zed's download API keeps network access in the normal extension
installation surface. Fixed identities prevent registry drift, while the
translation processes can clear their environment and remain incapable of
repairing or downloading themselves.

**Alternative considered**: Rehost the model in the project release. Rejected
because direct Mozilla acquisition preserves upstream identity, avoids an
unnecessary redistribution copy and keeps project releases smaller.

## Decision 5: Use one atomic package state, not a product lifecycle UI

**Decision**: The extension maintains `absent`, `checking`, `downloading`,
`ready` and `failed` states internally and maps them to Zed's
`CheckingForUpdate`, `Downloading`, `None` and `Failed` installation statuses.
The availability of the direct action and running LSP is the visible ready
state because API 0.7.0 has no distinct `Ready` status variant.

Preparation uses one exclusive create-new lock, a fixed staging directory and
atomic directory/state renames. A second Zed process waits and rechecks rather
than activating partial files. A stale lock is recoverable only after its
bounded age and an absent ready package. No incomplete directory is launchable.

**Rationale**: This is sufficient for editor installation without exposing
the seven-command F012 lifecycle. It also covers crash/retry and concurrent
windows using ordinary files available inside the WASI working directory.

## Decision 6: Preserve one last verified package during updates

**Decision**: `state.json` records `active` and optional `previous` package
IDs. A new package is built completely in staging and atomically promoted. If
download or verification fails, the extension validates and launches the
previous package. Once a new package is ready, unrelated/stale package
directories are removed, retaining at most current plus previous.

**Rationale**: Zed upgrades the extension code independently from downloaded
servers. Versioned work directories therefore provide a small, conventional
rollback without a user-facing updater or manual cleanup.

## Decision 7: Record and ship the publication obligations

**Decision**: The extension code is MIT and gets an accepted license file at
the exact `zed-extension/` marketplace path. The native release contains the
complete notice bundle and a corresponding-source document for the exact MPL,
MIT, BSD, Apache and zlib-licensed components identified by the prototype native
inventory. The three exact Mozilla resources are attributed to `MPL-2.0` using
the official archived model repository and exact installed-identity bridge;
the lock links that license and the upstream attachment records.

This is a recorded project attribution conclusion for this hobby project, not
legal advice. It removes any user approval prompt and does not hide the notices.

**Rationale**: Zed requires an accepted license for extension code and notes
that downloaded tools are separate. The project still owes accurate notices
for the release it controls. Direct model acquisition plus exact attribution
is simpler and more transparent than silently copying model blobs into a
project release.

**Primary sources**:

- <https://zed.dev/docs/extensions/developing-extensions#extension-license-requirements>
- <https://github.com/mozilla/firefox-translations-models/blob/e7957fc407441a5e3e35bbcbf9d60d9b35764618/LICENSE>
- <https://www.mozilla.org/en-US/MPL/2.0/FAQ/>

## Decision 8: Publish through the official extensions repository

**Decision**: After a public tagged project release and real clean-profile
validation, submit a PR to `zed-industries/extensions` adding this public repo
as the HTTPS submodule under `extensions/en-es-translator`, adding a matching
`extensions.toml` entry and running the repository sorter/checks. The extension
ID remains `en-es-translator` permanently.

**Rationale**: Zed's documented Gallery process is a central repository PR;
merge causes packaging and registry publication. A project PR or dev-extension
install alone does not prove the product requirement.

**Primary sources**:

- <https://zed.dev/docs/extensions/developing-extensions#publishing-your-extension>
- <https://github.com/zed-industries/extensions>

## Resolved Planning Unknowns

- **Marketplace mechanism**: official auto-downloaded language server.
- **User setup**: none beyond Gallery install and normal translation action.
- **Storage/removal**: Zed extension work directory and Zed uninstall.
- **Runtime/model**: bounded native Bergamot runner plus exact compact Firefox
  `en -> es` resources.
- **Acquisition**: fixed project release plus fixed Mozilla attachments, all
  size/hash verified before activation.
- **Updates**: atomic current/previous package state; no separate user lifecycle.
- **Publication**: exact-path MIT extension license, native notice/source
  bundle and official central-registry PR.

No `NEEDS CLARIFICATION` marker remains.
