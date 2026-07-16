# Data Model: Embedded Local Provider

**Feature**: `008-embedded-local-provider`
**Date**: 2026-07-15

The model describes reviewed identities and lifecycle state. It never stores
source text, permitted segments, translations, credentials, arbitrary
executable paths, registry responses or host identifiers.

## 1. EmbeddedProviderProfile

The versioned product decision for one supported local path.

| Field | Type | Rules |
|---|---|---|
| `schema_version` | integer | Exact supported schema; unknown versions fail closed |
| `profile_id` | fixed string | Product-owned allowlisted identifier; not user input |
| `source_language` | enum | Exactly `en` |
| `target_language` | enum | Exactly `es` |
| `platform` | enum | Initially `linux-x86_64` only |
| `cpu_baseline` | fixed string | Reviewed portable baseline; never `native` |
| `runner_manifest` | digest reference | Exact `RunnerManifest` |
| `model_set_manifest` | digest reference | Exact required resources |
| `delivery_conclusion` | enum | `local_build_plus_consented_download` for F012 |
| `publication_state` | enum | `blocked`, `review_required`, `approved`; F012 starts blocked |
| `resource_budgets` | object | Mandatory limits from the benchmark contract |
| `acceptance_matrix_version` | string | Versioned public synthetic fixture set |

Invariants:

- one profile contains one language direction and one platform;
- every referenced artifact belongs to the same reviewed compatibility set;
- the profile digest is SHA-256 over the domain
  `translator-provider-manifest-v1\0` followed by the fixed-order typed JSON
  payload containing schema/profile/languages/platform, review/publication
  states, runner, ordered artifacts and budgets;
- `artifact_set_digest`, `local_approval` and `publication_approval` are
  excluded from that payload only to avoid self-reference; the manager
  recomputes the digest and rejects any mismatch;
- production selection cannot supply or override the profile ID.

## 2. RunnerManifest

The identity and reproducible-build boundary of the native helper.

| Field | Type | Rules |
|---|---|---|
| `runner_id` | fixed string | `translator-embedded-runtime` |
| `wire_version` | integer | Exact supported request/response schema |
| `source_repository` | HTTPS URL | Reviewed official source |
| `source_commit` | SHA-1/SHA-256 text | Exact commit, never branch/tag only |
| `recursive_dependencies` | list | Exact source revisions and license conclusions |
| `build_recipe_digest` | SHA-256 | Versioned project build recipe |
| `compiler_profile` | string | Container/toolchain and portable CPU flags |
| `binary_name` | fixed string | No path separators or user substitution |
| `binary_sha256` | SHA-256 | Exact staged executable identity |
| `binary_size` | integer | Positive and within active-set budget |
| `elf_dependency_allowlist` | list | Exact permitted runtime libraries |
| `spdx_conclusion` | string | Reviewed expression, not inferred at runtime |
| `notice_sources` | list | Versioned notice/source-offer references |
| `local_review` | approval record | Human project-maintainer role, scope, date and accepted evidence digest |

Invariants:

- all recursive native sources are pinned before a release-shaped build;
- build output using an unreviewed CPU instruction baseline is ineligible;
- the executable is regular, product-owned, non-writable by other users,
  contained in its immutable object and matches hash/size before selection.

## 3. ModelArtifact

One required immutable language resource.

| Field | Type | Rules |
|---|---|---|
| `role` | enum | `model`, `vocabulary`, `lexical_shortlist`, or reviewed config role |
| `record_id` | fixed string | Exact upstream record identity |
| `record_version` | string | Exact reviewed version |
| `source_registry` | HTTPS URL | Review evidence only; not queried by runtime |
| `attachment_url` | HTTPS URL | Exact allowlisted preparation URL |
| `compressed_name` | fixed basename | No separators or archive paths |
| `installed_name` | fixed basename | Product-owned destination |
| `compression` | enum | Exact reviewed codec, initially `zstd` |
| `compressed_size` | integer | Exact expected bytes and hard maximum |
| `compressed_sha256` | SHA-256 | Required before expansion |
| `installed_size` | integer | Exact expanded bytes and hard maximum |
| `installed_sha256` | SHA-256 | Required after expansion and at verify |
| `language_pair` | fixed tuple | Exactly `en`, `es` |
| `runtime_compatibility` | string | Exact reviewed runner/model contract |
| `spdx_conclusion` | string | Reviewed artifact-level conclusion |
| `license_source` | HTTPS URL | Evidence used for conclusion |
| `delivery_permission` | enum | `consented_local_acquisition`; bundling not implied |
| `publication_review` | approval record | Separate F009 human decision; blocked/absent during F012 |

Invariants:

- every required role appears exactly once;
- compressed and installed hashes are both mandatory;
- redirects, mirror substitution and registry discovery cannot change URLs;
- a valid hash alone does not override language, license, size, platform or
  compatibility failures.
- an automated scan may support but cannot replace either required human
  approval; local activation and publication use distinct scopes.

## 4. ArtifactSet

An immutable, usable runner plus model-resource set.

| Field | Type | Rules |
|---|---|---|
| `manifest_digest` | SHA-256 | Identity of full reviewed profile |
| `object_digests` | ordered list | Runner and every required model artifact |
| `created_by_operation` | opaque safe ID | Random/non-content operation ID |
| `consent_digest` | SHA-256 | Must equal `manifest_digest` for acquired resources |
| `verification_state` | enum | `staged`, `verified`, `rejected` |
| `offline_smoke` | enum | `not_run`, `passed`, `failed` |
| `resource_gate` | enum | `not_run`, `passed`, `failed` |
| `license_gate` | enum | `complete`, `blocked` |

Invariants:

- the set is immutable after finalization;
- only a fully verified set with passed offline/resource/license gates may be
  referenced by `current` or `previous`;
- the set contains no benchmark content or user paths.

## 5. InstallationState

The only mutable persistent activation record, replaced atomically.

| Field | Type | Rules |
|---|---|---|
| `schema_version` | integer | Exact supported state schema |
| `generation` | integer | Monotonically increasing |
| `profile_id` | fixed string | Must match selected profile |
| `current` | optional manifest digest | Active verified set |
| `previous` | optional manifest digest | Last known-good verified set |
| `candidate` | optional manifest digest | Never used for translation |
| `last_operation` | safe enum | Operation name only |
| `last_outcome` | normalized enum | No raw errors/content/path |

State transitions:

```text
absent --prepare verified--> current=A
current=A --update staged--> current=A,candidate=B
current=A,candidate=B --promote--> previous=A,current=B,candidate=null
previous=A,current=B --rollback verified--> previous=B,current=A
any --failed/cancelled update--> current unchanged,candidate cleared or quarantined
current --explicit removal with exclusive lease--> absent
```

Atomicity rules:

- lifecycle mutations hold the exclusive lifecycle lock;
- staging is fsynced and finalized before state replacement;
- a crash before replacement leaves `current` unchanged;
- unknown state fields/schema versions fail closed rather than being repaired
  destructively.

## 6. ProviderSelection

The shared parsed configuration used by CLI, LSP and compatibility builds.

```text
Mock
LibreTranslate { fixed/validated local URL }
AzureTranslator { controlled secret reference and confirmation boundary }
EmbeddedLocal { fixed profile ID resolved internally }
```

For `EmbeddedLocal`:

- `TRANSLATOR_PROVIDER=embedded_local` is present;
- provider URL, API-key reference and remote-enable variables are absent;
- no artifact root, executable path, model path, URL, arguments or environment
  can be supplied by a workspace;
- missing or invalid `current` returns a stable not-configured/readiness error;
- it never silently falls back to Mock after explicit selection.

## 7. EmbeddedInvocation

Ephemeral parent-to-helper data. It is never logged or persisted.

| Field | Type | Rules |
|---|---|---|
| `wire_version` | integer | Exact version 1 initially |
| `source_language` | enum | `en` |
| `target_language` | enum | `es` |
| `tone` | enum | Existing `technical_neutral` only |
| `preserve` | list | Existing validated preservation values |
| `segments` | string list | 1..256, each <=4 KiB, aggregate <=20 KiB |

Response:

| Field | Type | Rules |
|---|---|---|
| `wire_version` | integer | Must equal request version |
| `translations` | string list | Exact cardinality/order; non-empty per segment |

The parent applies the existing semantic 40 KiB output limit after JSON
decoding. Transport byte caps include bounded framing overhead and do not
expand the semantic limit.

## 8. LifecycleOperation

An ephemeral operation descriptor.

| Field | Type | Rules |
|---|---|---|
| `operation_id` | random safe ID | Not content-derived |
| `kind` | enum | `disclose`, `prepare`, `status`, `verify`, `update`, `rollback`, `remove` |
| `profile_digest` | SHA-256 | Exact reviewed target |
| `consent_digest` | optional SHA-256 | Required for network acquisition/update |
| `network_policy` | enum | `exact_acquisition_only` or `offline_required` |
| `started_at` | timestamp | Safe metadata |
| `outcome` | normalized enum | No raw child/downloader output |

Operations never persist environment values, proxy settings, download URLs in
diagnostics, workspace roots or source/translation data.

## 9. BenchmarkRecord

Redacted evidence for one deterministic fixture and execution class.

| Field | Type | Rules |
|---|---|---|
| `manifest_digest` | SHA-256 | Safe artifact identity |
| `platform_class` | fixed enum | `fedora-linux-x86_64`; no hostname/user |
| `fixture_set_version` | string | Public synthetic corpus version |
| `case_id` | fixed ID | Not derived from content |
| `surface` | enum | `runner`, `cli`, `lsp`, `zed_manual` |
| `temperature` | enum | `new_process`, `warm_provider`; `warm_provider` means repeated one-shot launches after five warmups with a warm operating-system page cache only and does not mean a persistent provider |
| `round` / `repetition` | integers | Deterministic schedule |
| `elapsed_ms` | integer | Monotonic duration |
| `process_cpu_ms` | integer | Process CPU time |
| `peak_rss_bytes` | integer | Safe resource metric |
| `thread_peak` | integer | Safe resource metric |
| `network_attempts` | integer | Must be zero outside acquisition |
| `normalized_outcome` | enum | `passed` or stable failure class |

Prohibited fields include text, translation, content length combinations that
identify user data, content hashes, raw stderr, hostnames, usernames and paths.

`new_process` records the single pre-warmup process/model-load probe without
clearing page cache. Every matrix sample is `warm_provider`: it still launches
and reaps one process per request, but runs after the fixed warmups so the
operating-system page cache may be warm. No execution class keeps a provider
or model process alive between requests.

## 10. ValidationRecord

One redacted gate result.

| Field | Type | Rules |
|---|---|---|
| `gate_id` | fixed ID | Links to requirement/contract |
| `manifest_digest` | SHA-256 | Reviewed artifact set |
| `surface` | enum | CLI, direct Zed or lifecycle |
| `locality` | enum | `offline_local` |
| `fixture_id` | fixed ID | Public synthetic case only |
| `outcome` | enum | Stable pass/failure class |
| `non_mutation` | boolean | Required for translation cases |
| `offline` | boolean | Required after preparation |
| `reviewer_status` | enum | `pending`, `accepted`, `rejected` |

Raw provider output and any content-bearing diagnostic are never validation
evidence.
