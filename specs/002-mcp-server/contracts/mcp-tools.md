# MCP Tool Contract

This feature exposes only MCP tools. It does not expose MCP resources, prompts,
sampling, elicitation, HTTP endpoints, provider selection, or Zed extension
configuration.

## Capability

The server declares the MCP `tools` capability and exposes exactly:

- `translate_text`
- `translate_file`

The tool list is static for this feature.

## Tool: `translate_text`

Purpose: Translate direct English text to Spanish using the existing offline
translation behavior.

Input schema:

- [translate-text.input.schema.json](./translate-text.input.schema.json)

Successful result:

- `isError`: `false` or omitted by SDK default.
- `content`: exactly one text content item.
- `content[0].text`: translated text only.
- `structuredContent.translated_text`: translated text only.

Execution error result:

- `isError`: `true`.
- `content`: exactly one text content item.
- `content[0].text`: `CODE: redacted message`.
- `structuredContent.code`: normalized error code.
- `structuredContent.message`: redacted actionable message.

Rejected behavior:

- No provider selection.
- No remote confirmation field.
- No file path or workspace root fields.
- No logs or diagnostics in success output.

## Tool: `translate_file`

Purpose: Translate an allowed Markdown or text file inside the authorized
workspace using the existing core file safety boundary.

Input schema:

- [translate-file.input.schema.json](./translate-file.input.schema.json)

Successful result:

- `isError`: `false` or omitted by SDK default.
- `content`: exactly one text content item.
- `content[0].text`: translated content only.
- `structuredContent.translated_text`: translated content only.

Execution error result:

- `isError`: `true`.
- `content`: exactly one text content item.
- `content[0].text`: `CODE: redacted message`.
- `structuredContent.code`: normalized error code.
- `structuredContent.message`: redacted actionable message.

Rejected behavior:

- No provider selection.
- No remote confirmation field.
- No reading outside the authorized workspace.
- No unsupported file extensions beyond `.md`, `.markdown`, and `.txt`.
- No source file mutation.
- No file path, workspace root, protected segment, source text, or secret in
  errors/logs.

## Protocol Errors

Use protocol-level errors, not `isError: true` tool results, for:

- malformed MCP/JSON-RPC messages;
- invalid `tools/call` protocol shape;
- unknown tool names;
- server initialization protocol failures.

Protocol errors must be redacted and must not start translation work.

## Error Codes

The MCP error result code must preserve the existing normalized translation
error where one exists. Known codes include:

- `INVALID_INPUT`
- `UNSUPPORTED_LANGUAGE_PAIR`
- `UNSUPPORTED_FILE_TYPE`
- `FILE_TOO_LARGE`
- `FILE_NOT_FOUND`
- `PATH_NOT_ALLOWED`
- `NON_UTF8_INPUT`
- `NO_TRANSLATABLE_SEGMENTS`
- `SECRET_DETECTED`
- `PROVIDER_NOT_CONFIGURED`
- `REMOTE_CONFIRMATION_REQUIRED`
- `PROVIDER_FAILED`
- `PROVIDER_TIMEOUT`
- `INTERNAL_ERROR`

## Privacy Rules

- stdout contains MCP messages only.
- stderr/log output is allowed only when redacted.
- Success visible content may contain the requested translated text.
- Errors, logs, protocol diagnostics, and metadata must not contain source text,
  translated text, translatable segments, protected content, secrets, headers,
  tokens, workspace roots, or sensitive paths.
