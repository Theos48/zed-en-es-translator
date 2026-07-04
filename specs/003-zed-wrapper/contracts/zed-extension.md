# Contract: Zed Development Extension

## Extension Directory

The local development extension lives at:

```text
zed-extension/
```

It must contain:

```text
zed-extension/
├── extension.toml
├── Cargo.toml
└── src/
    └── lib.rs
```

## Manifest Contract

`zed-extension/extension.toml` must provide the required Zed extension metadata
and exactly one context server declaration for this feature.

Required metadata:

```toml
id = "en-es-translator"
name = "English to Spanish Translator"
version = "0.0.1"
schema_version = 1
authors = ["theos"]
description = "Local English to Spanish translator MCP wrapper."
repository = "https://github.com/theos/zed-en-es-translator"

[context_servers.translator-en-es]
```

Validation:

- The manifest must be valid TOML.
- The extension id and context server id must remain stable for F006.
- The manifest must not declare more than one context server in this feature.
- The manifest must not declare themes, languages, snippets, debuggers, network
  services, provider configuration, or publication-only behavior.
- The manifest must not contain absolute local paths or secrets.

## Rust/WASM Extension Contract

`zed-extension/Cargo.toml` must define a Rust `cdylib` extension crate using
`zed_extension_api = "0.7.0"`.

The extension code must:

- implement `zed::Extension`;
- register the extension with `zed::register_extension!`;
- implement `context_server_command` for `translator-en-es`;
- optionally implement `context_server_configuration` to describe local
  development settings and installation instructions;
- return errors as redacted user-actionable strings.

The extension code must not:

- translate content itself;
- read source files for translation;
- modify buffers, selections, or files;
- invoke a shell;
- download binaries or contact network services;
- accept provider, API key, URL, or remote confirmation settings;
- log source text, translated text, secrets, tokens, environment dumps, or
  unredacted sensitive paths.

## Configuration Contract

The extension configuration surface for F006 is limited to the local MCP server
artifact path.

Allowed setting:

```json
{
  "binary_path": "/absolute/path/to/target/release/translator-mcp"
}
```

Rejected settings:

- `provider`
- `api_key`
- `base_url`
- `remote`
- `remote_confirmation`
- `headers`
- `extra_env`
- `extra_args`

Validation:

- Missing `binary_path` must produce an actionable redacted diagnostic.
- Invalid or unusable `binary_path` must not be echoed in logs or errors.
- The extension must not inherit arbitrary shell or Zed environment values to
  compensate for missing settings.

## Manual Zed Contract

Manual Zed validation must install the local dev extension from:

```text
zed-extension/
```

Expected behavior:

- Zed recognizes one local development extension.
- Zed can request the `translator-en-es` context server command.
- The command launches the prepared `translator-mcp` stdio server.
- MCP tool discovery still exposes only `translate_text` and `translate_file`.
- Translation results appear through Zed's MCP/Agent Panel flow.
- Source files and editor buffers remain unchanged.

If Zed cannot build or load the dev extension because host prerequisites are
missing, validation must record the blocker and must not install system
toolchains unless separately approved under the host policy.
