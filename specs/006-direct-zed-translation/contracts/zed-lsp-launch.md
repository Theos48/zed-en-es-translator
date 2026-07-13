# Contract: Zed LSP Launch

## Manifest

The extension keeps its existing context server and adds:

```toml
[language_servers.en-es-translator]
name = "English to Spanish Translator"
languages = ["Markdown", "Plain Text"]
```

The id `en-es-translator` is distinct from the compatibility context-server id
`translator-en-es`.

## Wrapper Command

`language_server_command` MUST:

1. Read `LspSettings` for `en-es-translator` and the active worktree.
2. Require a configured absolute binary path whose filename is
   `translator-lsp`.
3. Reject all binary arguments.
4. Parse provider launch values from the explicit `binary.env` allowlist.
5. Return a direct `zed::Command` with an empty argument vector and only
   allowlisted provider/RUST_LOG environment additions.

The wrapper MUST NOT invoke a shell, download an artifact, inspect document
content, request Agent, or pass arbitrary environment entries.

## User Settings Shape

```json
{
  "lsp": {
    "en-es-translator": {
      "binary": {
        "path": "/absolute/path/to/target/release/translator-lsp",
        "arguments": [],
        "env": {
          "TRANSLATOR_PROVIDER": "libretranslate",
          "TRANSLATOR_PROVIDER_URL": "https://example.invalid",
          "TRANSLATOR_ALLOW_REMOTE_PROVIDER": "true"
        }
      }
    }
  }
}
```

Omit `binary.env` for the default mock/offline provider. The only accepted
provider keys are `TRANSLATOR_PROVIDER`, `TRANSLATOR_PROVIDER_URL`,
`TRANSLATOR_PROVIDER_API_KEY_ENV`, and
`TRANSLATOR_ALLOW_REMOTE_PROVIDER`; arbitrary variables and conflicts with
nested provider settings fail closed. Real secrets MUST NOT be stored in
settings. `TRANSLATOR_PROVIDER_API_KEY_ENV` names an inherited environment
variable and never contains its value.

Real Zed 1.10.3 validation showed that custom values under the generic LSP
`settings` object did not affect the environment used to launch the extension
language server. `binary.env` is therefore the versioned launch contract for
provider selection.

## Artifact Preparation

`make zed-direct-prepare` builds the locked release `translator-lsp` artifact
inside the project container. `scripts/zed-extension/prepare-direct.sh`
validates filename, existence, regular-file status, and executable permission,
then prints only its absolute path.

The existing `make zed-extension-prepare`/`translator-mcp` behavior remains
unchanged for compatibility.
