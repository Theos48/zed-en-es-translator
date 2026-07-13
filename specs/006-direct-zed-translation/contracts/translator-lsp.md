# Contract: translator-lsp

## Transport And Lifecycle

- Protocol: Language Server Protocol JSON-RPC over stdin/stdout.
- One long-lived process per Zed worktree/language-server instance.
- No network listener and no non-LSP stdout output.
- stderr may contain only redacted code/status metadata.
- Graceful `shutdown`/`exit` handling is required.

## Server Capabilities

The initialize result advertises only:

- full document text synchronization (`openClose = true`, `change = Full`);
- `codeActionProvider` for `refactor` actions;
- `executeCommandProvider.commands = ["en-es-translator.translate"]`;
- `hoverProvider = true`.

No completion, diagnostics, formatting, rename, file operation, or workspace
edit capability is advertised.

## Inbound Notifications

### `textDocument/didOpen`

Store URI, version, language id, and full text. Rejecting a future translation
because of limits is allowed; the server MUST NOT log the text.

### `textDocument/didChange`

Accept exactly one full-content change. Replace the snapshot and invalidate any
preview for the URI. Incremental or version-regressing changes fail closed and
must not retain a stale preview.

### `textDocument/didClose`

Remove snapshot and preview for the URI.

## Inbound Requests

### `textDocument/codeAction`

When a supported snapshot/version/range can be represented safely, return one
`CodeAction`:

```json
{
  "title": "Translate English to Spanish [offline]",
  "kind": "refactor",
  "command": {
    "title": "Translate English to Spanish [offline]",
    "command": "en-es-translator.translate",
    "arguments": [
      {
        "uri": "<document-uri>",
        "version": 1,
        "range": {
          "start": { "line": 0, "character": 0 },
          "end": { "line": 0, "character": 14 }
        },
        "input_kind": "markdown"
      }
    ]
  }
}
```

The safe suffix is `[offline]`, `[local]`, or
`[remote - confirmation required]` according to provider locality. The action
MUST have no edit and arguments MUST contain no source text. Unsupported
languages return an empty action list.

### `workspace/executeCommand`

Accept only command `en-es-translator.translate` with exactly one argument
matching the payload above and the current snapshot. All unknown fields,
commands, stale versions, invalid ranges, and extra arguments return a redacted
invalid-request response without provider contact.

Success result is JSON null. The translated text is retained only in preview
state, never returned in the command response.

### `textDocument/hover`

Return `null` when no current preview covers the requested position. Otherwise
return `MarkupContent { kind: "markdown", value: translated_text }` plus the
source range. Plain text is escaped/represented so it cannot become unintended
Markdown structure.

## Outbound Messages

- `window/showMessageRequest`: remote confirmation only.
- `window/showMessage`: safe preview-ready or normalized failure status only.

No source or translated text is placed in either message.

## Error Mapping

Translation failures preserve the existing `ErrorCode` set. Protocol parsing,
stale state, invalid Unicode position, or unsupported message shapes map to
`INVALID_INPUT` or JSON-RPC invalid-params as appropriate. Unexpected internal
failures map to `INTERNAL_ERROR`. All messages pass through redaction.
