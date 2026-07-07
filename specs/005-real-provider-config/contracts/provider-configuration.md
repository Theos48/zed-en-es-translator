# Provider Configuration Contract

This contract defines how the product selects a real provider without carrying
provider configuration in translated content.

## Defaults

- If no provider configuration is present, provider mode is `mock`.
- Existing translation requests without remote confirmation remain valid.
- Remote confirmation is `false` when absent.

## Process Configuration

The CLI and MCP server read provider configuration from controlled process
configuration:

- `TRANSLATOR_PROVIDER`
  - allowed: unset, `mock`, `libretranslate`
  - default: `mock`
- `TRANSLATOR_PROVIDER_URL`
  - required when `TRANSLATOR_PROVIDER=libretranslate`
  - must not contain embedded credentials
- `TRANSLATOR_PROVIDER_API_KEY_ENV`
  - optional environment variable name containing a provider API key
  - the value itself must never be versioned, printed, or included in Zed
    settings
- `TRANSLATOR_ALLOW_REMOTE_PROVIDER`
  - optional boolean
  - default: `false`
  - required before a non-local target can proceed to request-level
    confirmation

Invalid configuration fails before translation starts and returns a normalized
redacted error.

## Request-Level Confirmation

Translation request inputs may include an additive boolean:

```json
{
  "remote_confirmed": true
}
```

Rules:

- Missing `remote_confirmed` means `false`.
- The field is meaningful only for configured non-local provider targets.
- `remote_confirmed: true` does not bypass secret detection, size limits,
  provider allowlisting, response validation, or redaction.
- Existing request fields remain unchanged and backward-compatible.

## Zed Context-Server Settings

The Zed extension may expose controlled provider settings that map to the
process configuration above:

```json
{
  "binary_path": "/absolute/path/to/translator-mcp",
  "provider": {
    "mode": "mock",
    "url": "",
    "api_key_env": "",
    "allow_remote": false
  }
}
```

Rules:

- `binary_path` remains required for the local MCP server artifact.
- `provider.mode` defaults to `mock`.
- `provider.url` is required only for real provider mode.
- `provider.api_key_env` is a variable name only, not a secret value.
- `provider.allow_remote` defaults to `false`.
- Unsupported keys, headers, raw API keys, extra command arguments, and
  arbitrary extra environment variables are rejected.

## Locality Classification

Local targets are loopback-only. Accepted local examples:

- `http://127.0.0.1:<port>`
- `http://localhost:<port>`
- `http://[::1]:<port>`

Non-local targets are any other host. Non-local targets require:

- explicit provider configuration;
- `TRANSLATOR_ALLOW_REMOTE_PROVIDER=true` or equivalent controlled setting;
- request-level `remote_confirmed: true`;
- secret detection pass before contact.

## Rejected Behavior

- Provider URL, API keys, headers, or arbitrary provider options inside
  `source_text`.
- Provider URL, API keys, headers, or arbitrary provider options inside
  workspace files.
- Real `.env` files committed to the repository.
- Logging raw provider configuration, secret values, source text, translated
  text, request bodies, response bodies, workspace roots, or sensitive paths.
- Using inherited proxy environment variables for provider requests.
