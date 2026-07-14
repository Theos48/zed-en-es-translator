# Data Model: Operational Real Providers

This feature adds operational state and one provider adapter; it does not add a
database. Versioned records are declarative files, project-local runtime state
is disposable metadata, and secrets/content are never persisted.

## OperationalProviderProfile

Reviewed identity of a supported provider.

| Field | Type | Rules |
|---|---|---|
| `id` | enum | `mock`, `libretranslate_local`, or `azure_translator` |
| `locality` | enum | `offline`, `local`, or `remote` |
| `software_identity` | string | Safe release/service identity; never a URL containing user data |
| `language_pair` | tuple | Exactly `en -> es` |
| `endpoint_policy` | enum | none, loopback-only, or fixed Azure global host |
| `credential_reference_required` | bool | False for supported local profile; true for Azure |
| `runtime_artifacts` | list | Safe lock references, never source/translation/credentials |
| `readiness_policy` | enum | mock-ready, local health+translation, or remote configuration-only |

### Invariants

- Absence of explicit configuration selects `mock`.
- `local` accepts only HTTP loopback and the reviewed local port.
- `remote` accepts only fixed HTTPS host/path with redirects and inherited
  proxies disabled.
- Provider branding does not appear in the user-facing locality label.

## LocalProviderLock

Versioned supply-chain inputs for reproducible preparation.

| Field | Type | Rules |
|---|---|---|
| `schema_version` | integer | Starts at 1; unknown versions fail closed |
| `image_reference` | string | Includes stable tag and immutable multi-arch digest |
| `image_platforms` | map | Reviewed platform manifest digests |
| `package_index_revision` | commit ID | Exact reviewed Argos index revision |
| `models` | list | Direction, version, source URL and project-observed SHA-256 |
| `license_status` | enum | `declared` or `unresolved`; unresolved forbids redistribution |
| `reviewed_at` | date | Metadata only |

### Invariants

- An artifact is not prepared until its computed digest matches the lock.
- A lock update is manual and reviewable; runtime never rewrites it.
- A project-observed model hash is not represented as upstream-attested.

## LocalProviderEnvironment

Project-namespaced Docker/Compose resources and safe runtime metadata.

| Field | Type | Rules |
|---|---|---|
| `compose_project` | constant | `zed-en-es-translator-providers` |
| `active_slot` | enum | `current` or `previous`; never `candidate` |
| `slots` | map | Candidate/current/previous artifact identity and safe verification state |
| `container_state` | enum | absent, created, starting, ready, stopped, or failed |
| `offline_verified` | bool | True only after no-egress restart plus health+translation probe |
| `last_known_good` | slot reference | Must have passed full verification |
| `safe_failure_code` | optional enum | Normalized status only; no raw provider detail |

### Lifecycle states

```text
UNPREPARED
  -> CANDIDATE_PREPARING
  -> CANDIDATE_READY
  -> CANDIDATE_OFFLINE_VERIFIED
  -> CURRENT_READY
  -> STOPPED

CURRENT_READY
  -> UPDATE_CANDIDATE_PREPARING
  -> UPDATE_CANDIDATE_VERIFIED
  -> CURRENT_READY (promotion; old current becomes previous)

UPDATE_*_FAILED
  -> CURRENT_READY (unchanged)

CURRENT_FAILED
  -> ROLLBACK_VERIFYING
  -> PREVIOUS_READY

any stopped state
  -> CLEANED (explicit destructive confirmation only)
```

### Invariants

- `candidate` preparation never mutates `current` or `previous`.
- Promotion requires matching locks, `/health`, fixed public translation
  probe, offline restart, and second successful probe.
- `previous` remains available until a later successful promotion or explicit
  cleanup.
- Start/stop/status/verify are safe to repeat.
- Repository `make clean` cannot remove provider volumes or state.

## RemoteAccessConfiguration

Parsed process configuration for Azure Translator.

| Field | Type | Rules |
|---|---|---|
| `mode` | enum | Exactly `azure_translator` |
| `allow_remote` | bool | Must be explicitly true |
| `api_key_env` | safe identifier | Names an inherited environment variable; is not the secret value |
| `endpoint` | constant | `https://api.cognitive.microsofttranslator.com` |
| `api_version` | constant | `3.0` |
| `from` / `to` | constant | `en` / `es` |
| `url_override` | absent | Any configured provider URL is invalid |

### Invariants

- Missing/empty key values fail before contact.
- Configuration does not constitute request confirmation.
- The actual key is resolved only when constructing the provider and is never
  copied to diagnostics, `Debug`, evidence, files, or Zed settings.
- Proxy inheritance, redirects, region/custom endpoint, and retry are disabled.

## ProviderInvocation

Ephemeral execution state shared by real-provider paths.

| Field | Type | Rules |
|---|---|---|
| `provider_locality` | enum | Safe locality only |
| `segments` | list of UTF-8 text | Ephemeral; 1..256; each <=4 KiB |
| `input_bytes` | integer | <=20 KiB |
| `source` / `target` | enum | Fixed English/Spanish |
| `tone` | enum | Existing technical-neutral mapping |
| `remote_confirmed` | bool | Fresh per invocation; ignored for local/mock |
| `normalized_outcome` | enum | Existing success/error contract |
| `output_bytes` | integer | <=40 KiB on success |

### Processing order

1. Validate request, file boundary, formatting and limits.
2. Segment/protect content.
3. Parse and validate provider configuration/locality.
4. For remote: validate fresh confirmation.
5. For remote: run secret detection.
6. Validate exact target and minimize payload.
7. Contact provider within the 15-second budget.
8. Validate response type, cardinality, non-empty text and output limit.
9. Reconstruct an ephemeral result without mutating the source.

## ValidationRecord

Human-reviewed, redacted evidence row. It may be committed only after its
privacy checks pass.

| Field | Required | Example-safe value |
|---|---|---|
| `case_id` | yes | `LOCAL-CLI-01` |
| `timestamp_utc` | yes | ISO-8601 timestamp |
| `surface` | yes | `cli` or `zed-direct` |
| `locality` | yes | `local` or `remote` |
| `provider_identity` | yes | Safe release/service/tier label |
| `artifact_identity` | local only | Digest prefix/model versions, no host path |
| `expected_outcome` | yes | Normalized condition |
| `actual_outcome` | yes | `success` or existing `ErrorCode` |
| `provider_contacted` | negative cases | yes/no derived from controlled observation |
| `source_unchanged` | yes | boolean |
| `buffer_unchanged` | Zed | boolean |
| `redaction_passed` | yes | boolean |
| `within_budget` | yes | boolean for the 120-second prepared-readiness or 15-second invocation budget |
| `reviewer_result` | yes | pass/fail |
| `notes` | optional | Safe status vocabulary only |

### Prohibited fields/content

Source text, translated text, segments, response bodies, tokens, secret
values, headers, environment dumps, full endpoints, workspace roots, local
paths, screenshots containing content, or raw logs are never evidence.
