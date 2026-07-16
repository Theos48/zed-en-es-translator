# Data Model: Plug-and-Play Zed Marketplace Installation

## Marketplace Extension

Represents the public extension version installed by Zed.

| Field | Type | Rules |
|---|---|---|
| `extension_id` | string | Permanently `en-es-translator` |
| `version` | semantic version | Matches `extension.toml` and registry entry |
| `schema_version` | integer | Supported Zed manifest schema |
| `supported_platforms` | set | Initially only Linux `x86_64` |
| `language_server_id` | string | Permanently `en-es-translator` |
| `languages` | set | Markdown and Plain Text |
| `license` | SPDX identifier | MIT for extension code |

The published extension declares only the direct language server. Feature 010
removes every retired compatibility surface before the release is tagged.

## Published Package Lock

The immutable build-time record compiled into one extension version.

| Field | Type | Rules |
|---|---|---|
| `schema_version` | integer | Exactly `1` |
| `package_id` | string | Safe basename, unique for exact contents |
| `package_version` | semantic version | Fixed; never `latest` |
| `platform` | string | `linux-x86_64` for this feature |
| `source_language` | string | `en` |
| `target_language` | string | `es` |
| `server_archive` | Server Archive | Fixed project release identity |
| `model_resources` | ordered list | Exactly model, vocabulary and lexical shortlist |
| `budgets` | Package Budgets | Cannot exceed specification limits |
| `license_bundle` | License Bundle | Required before publication |

Changing a URL, size, hash, source revision, notice or compatibility field
creates a new `package_id` and extension version. The extension never queries a
registry to discover package identity.

## Server Archive

| Field | Type | Rules |
|---|---|---|
| `url` | HTTPS URL | Exact public project release asset; no redirect discovery |
| `archive_type` | enum | `gzip_tar` |
| `files` | ordered list | LSP, native runner and required notice/source files |

Each executable file has role, relative safe path, exact installed size,
SHA-256, executable flag, source repository/commit and SPDX conclusion.

## Model Resource

| Field | Type | Rules |
|---|---|---|
| `role` | enum | `model`, `vocabulary`, `lexical_shortlist`; exactly one each |
| `record_id` | UUID string | Exact Mozilla record |
| `url` | HTTPS URL | Exact Mozilla attachment, no caller override |
| `compressed_name` | basename | Zstandard payload |
| `compressed_size` | integer | Must match before decoding |
| `compressed_sha256` | lowercase hex | Must match before decoding |
| `installed_name` | basename | Fixed runner argument target |
| `installed_size` | integer | Must match before activation |
| `installed_sha256` | lowercase hex | Object identity and final verification |
| `spdx_conclusion` | string | `MPL-2.0` |
| `license_url` | HTTPS URL | Exact reviewed official license source |

## Package Budgets

| Field | Value |
|---|---:|
| Maximum transfer | 64 MiB |
| Maximum active installed package | 128 MiB |
| Maximum retained current + previous + staging | 384 MiB |
| Required free storage before preparation | 512 MiB |
| Peak translation RSS | 1 GiB |
| Inference threads | 4 |
| Translation deadline | 15 seconds |

The release validator computes actual values. The extension rejects declared
or observed content beyond a budget before activation.

## Local Installation State

A small extension-owned JSON record, never a user setting.

| Field | Type | Rules |
|---|---|---|
| `schema_version` | integer | Exactly `1` |
| `generation` | integer | Increases on every atomic promotion |
| `active` | package ID or null | References a verified ready directory |
| `previous` | package ID or null | Different from active; at most one |
| `last_outcome` | enum | `ready`, `failed`, `unsupported` |

The state record contains no workspace, host, account or document information.
It is written to a temporary sibling, synced and atomically renamed only after
the target package is complete.

## Installed Package

| Field | Type | Rules |
|---|---|---|
| `package_id` | string | Matches its directory and published lock |
| `platform` | string | Matches the current extension host |
| `wire_version` | integer | Compatible with the launching LSP |
| `artifacts` | ordered list | Exact installed sizes/hashes |
| `verification_state` | enum | Only `verified` is launchable |
| `verified_at_generation` | integer | Matches promotion generation |

An installed package is immutable after promotion. A missing, symlinked,
non-regular, wrong-size or wrong-hash executable/model makes it unready.

## Preparation Lock and Staging

The lock is a create-new regular file containing only schema version, package
ID and acquisition start time. It serializes preparation across Zed processes.
The staging directory is fixed by package ID and cannot be active. A retrier
may remove stale staging only after it safely owns the lock.

## Translation Invocation

The retained result/error contract is versioned by
[translate-result.schema.json](./contracts/translate-result.schema.json). For
this feature the LSP selects a verified adjacent `Installed Package`, creates one
bounded `Translation Invocation`, and passes only ordered permitted segments,
language metadata and tone through the private runner wire.

No model path, executable path, package URL or acquisition command is accepted
from a workspace or user setting.

## State Transitions

```text
absent -> checking -> downloading -> ready
   |          |             |
   |          |             +-> failed -> checking
   |          +----------------> failed -> checking
   +----------------------------> failed -> checking

ready(current) -> checking(update) -> downloading(candidate)
       |                    |                |
       |                    +-> failed ------+-> ready(current)
       +-------------------------- success ----> ready(candidate, previous=current)
```

- Unsupported platform transitions directly to `unsupported`; it creates no
  lock, staging or package download.
- Only a fully verified staging package can become active.
- Failure never changes a valid active reference.
- Uninstall is owned by Zed and removes the whole extension work directory.

## Relationships and Invariants

- One extension version has exactly one Published Package Lock per supported
  platform.
- One Local Installation State references at most two Installed Packages.
- An active Installed Package must satisfy every identity in its lock.
- A Model Resource belongs to one exact package identity even when its upstream
  URL is shared by another release.
- Translation can read only the active package; acquisition can write only
  staging and state.
- No entity stores translated/source text, paths, credentials or secrets.
