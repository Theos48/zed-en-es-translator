# LibreTranslate-Compatible Provider Contract

This contract defines the first real provider adapter. It is intentionally
limited to English-to-Spanish text translation through a configured
LibreTranslate-compatible service.

## Provider Request

The adapter sends only:

- permitted translatable segment text;
- source language: `en`;
- target language: `es`;
- format: plain text;
- optional API key value loaded from the configured secret environment variable.

For a LibreTranslate-compatible `/translate` request, the provider payload maps
to:

- `q`: one segment or an ordered list of segments;
- `source`: `en`;
- `target`: `es`;
- `format`: `text`;
- `alternatives`: `0`;
- `api_key`: omitted unless configured.

The adapter must not send:

- workspace root;
- file path;
- protected code regions;
- full unsegmented file content;
- environment dumps;
- logs;
- headers other than reviewed request headers required by the provider client;
- tokens or secrets not explicitly selected through `api_key_env`;
- unrelated editor or Agent context.

## Provider Response

Successful responses must contain translated text for every requested segment.

Validation:

- one returned segment for one requested segment;
- matching ordered segment count for batch/multi-segment requests;
- non-empty text;
- UTF-8/text-compatible response body;
- combined output within the existing output limit.

Malformed, empty, mismatched, non-textual, or oversized responses are rejected
before user-visible success output is created.

## Error Mapping

| Condition | Normalized Result |
|-----------|-------------------|
| Missing provider configuration | `PROVIDER_NOT_CONFIGURED` |
| Unsafe or unsupported provider target | `PROVIDER_NOT_CONFIGURED` or `INVALID_INPUT` |
| Non-local target without request confirmation | `REMOTE_CONFIRMATION_REQUIRED` |
| Obvious secret before non-local contact | `SECRET_DETECTED` |
| Timeout | `PROVIDER_TIMEOUT` |
| Transport failure | `PROVIDER_FAILED` |
| Provider status rejection, including throttling or quota | `PROVIDER_FAILED` |
| Malformed provider response | `PROVIDER_FAILED` |
| Mismatched segment count | `PROVIDER_FAILED` |
| Oversized provider output | `PROVIDER_FAILED` |

All mapped failures must be redacted before reaching CLI stdout/stderr, MCP
tool output, Zed diagnostics, or test evidence.

## Timeout And Limits

- Provider call timeout uses the existing 15 s provider timeout.
- Input limit remains 20 KiB.
- Segment limit remains 4 KiB per segment and 256 segments total.
- Output limit remains 40 KiB.

## HTTP Client Constraints

- Construct a reviewed client/agent with explicit timeout.
- Do not use shell execution.
- Do not inherit proxy configuration from process environment.
- Do not enable client DEBUG/TRACE logging for sensitive runs.
- Do not log raw request bodies, response bodies, headers, API keys, source
  text, translated text, segments, paths, or workspace roots.

## Test Requirements

Automated tests must cover:

- successful local provider translation with a stub or loopback service;
- request payload contains only permitted segment text and language/format
  metadata;
- default mock mode when provider config is absent;
- invalid provider mode and unsafe target rejection;
- remote denial before network contact;
- confirmed remote secret blocking before network contact;
- timeout mapping;
- provider status/rejection mapping;
- malformed response mapping;
- segment-count mismatch mapping;
- output limit enforcement;
- redaction of diagnostics and stderr.
