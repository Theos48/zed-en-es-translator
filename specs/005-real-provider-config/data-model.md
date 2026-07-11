# Data Model: Real Provider Configuration

## ProviderMode

Represents the active translation provider family.

Fields:

- `mock`: deterministic offline provider; default when no real provider is
  configured.
- `libretranslate`: real provider adapter for LibreTranslate-compatible
  services.

Validation:

- Missing mode resolves to `mock`.
- Unknown modes are rejected before translation starts.

## ProviderTarget

Represents where a real provider request would be sent.

Fields:

- `url`: configured provider base URL.
- `locality`: `local` or `non_local`.
- `allow_remote`: whether non-local targets are permitted by configuration.

Validation:

- Local targets must resolve to loopback-style hostnames or addresses.
- Non-local targets require explicit remote allowance and request-level
  confirmation.
- Targets must not include embedded credentials.
- Unsafe schemes, empty URLs, and unsupported hosts are rejected before provider
  contact.

## ProviderConfiguration

Represents process/server configuration used to choose a provider.

Fields:

- `mode`: `ProviderMode`.
- `target`: optional `ProviderTarget` for real provider modes.
- `api_key_env`: optional environment variable name containing a provider API
  key for deployments that require one.

Validation:

- No real secret values are stored in versioned files or Zed settings.
- `api_key_env`, when present, is an environment variable name only.
- Real provider modes require a valid target.
- `mock` ignores real provider target fields.

## RemoteConfirmation

Represents the per-request decision allowing a configured non-local provider to
receive permitted content.

Fields:

- `confirmed`: boolean, default `false` when absent.

Validation:

- Confirmation applies only to one translation request.
- Confirmation does not override secret detection or provider allowlisting.
- Local provider requests do not require remote confirmation.

## ProviderInvocation

Represents one attempted provider call after input validation and segmentation.

Fields:

- `provider_mode`.
- `target_locality`.
- `remote_confirmation`.
- `source_language`.
- `target_language`.
- `tone`.
- `segment_count`.
- `input_bytes`.
- `started_at` and duration metadata for diagnostics.
- `outcome`: success or normalized failure code.

Validation:

- Invocation metadata must not contain source text, translated text, file
  paths, workspace roots, secrets, headers, or tokens.
- Segment count must be between 1 and the existing maximum.

## ProviderPayload

Represents the content sent to the provider adapter.

Fields:

- `segments`: permitted translatable strings only.
- `source_language`: `en`.
- `target_language`: `es`.
- `format`: plain text.

Validation:

- No protected code regions, file paths, workspace roots, headers, tokens,
  secrets, or unrelated editor context.
- Each segment obeys the existing segment size limit.
- Obvious secret detection blocks non-local payloads before contact.

## ProviderResponse

Represents provider-returned translated content before it becomes public output.

Fields:

- `translated_segments`: provider-returned translated strings.
- `provider_status`: transport/status category used only for redacted
  diagnostics.

Validation:

- Segment count must match the request.
- Each translated segment must be non-empty text.
- Combined output must obey the existing output limit.
- Malformed, non-textual, oversized, or mismatched responses become normalized
  failures.

## ProviderDiagnostic

Represents redacted information about provider setup and failure.

Fields:

- `code`: normalized error code.
- `phase`: configuration, privacy gate, request, response, timeout, or
  redaction.
- `message`: actionable redacted message.

Validation:

- Must not include source text, translated text, full translatable segments,
  secrets, tokens, headers, workspace roots, sensitive paths, or raw provider
  response bodies.

## State Transitions

```text
unconfigured
  -> mock_active
  -> real_configured_local
  -> invoking_local_provider
  -> success | normalized_failure

unconfigured
  -> real_configured_non_local
  -> denied_remote_confirmation_required
  -> confirmed_for_request
  -> denied_secret_detected | invoking_non_local_provider
  -> success | normalized_failure
```
