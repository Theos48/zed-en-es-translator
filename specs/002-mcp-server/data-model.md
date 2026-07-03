# Data Model: MCP Server

## MCP Tool Definition

Represents a tool exposed by the server.

Fields:

- `name`: Stable identifier. Allowed values in this feature:
  `translate_text`, `translate_file`.
- `description`: Human-readable purpose with no hidden instructions or provider
  prompts.
- `input_schema`: JSON Schema object for the accepted parameters.

Validation rules:

- Tool names must be unique.
- Tool names must be stable and lowercase ASCII with underscore separators.
- Tool descriptions must not contain source text, secrets, environment data, or
  prompt-injection style instructions.
- The server exposes only the `tools` capability in this feature.

Relationships:

- Each definition maps to exactly one tool handler.

## TranslateTextToolRequest

Represents parameters for the `translate_text` tool.

Fields:

- `source_text`: Required non-empty UTF-8 string.
- `source_language`: Optional language code; if present, must be `en`.
- `target_language`: Optional language code; if present, must be `es`.
- `tone`: Optional tone value; defaults to the existing technical neutral tone.
- `preserve_formatting`: Optional boolean; if present, must be `true`.

Validation rules:

- Reject missing, empty, whitespace-only, non-string, oversized, or conflicting
  input before provider processing.
- Preserve existing direct-text size and segment limits from feature 001.
- Do not accept provider selection or remote confirmation fields in this
  feature.

Relationships:

- Converts to the existing core direct text request path.
- Produces `TranslationToolSuccess` or `TranslationToolError`.

## TranslateFileToolRequest

Represents parameters for the `translate_file` tool.

Fields:

- `workspace_root`: Required UTF-8 string representing the authorized workspace.
- `file_path`: Required UTF-8 string representing a requested file path.
- `source_language`: Optional language code; if present, must be `en`.
- `target_language`: Optional language code; if present, must be `es`.
- `tone`: Optional tone value; defaults to the existing technical neutral tone.
- `preserve_formatting`: Optional boolean; if present, must be `true`.

Validation rules:

- Reject missing fields, wrong field types, empty strings, unsupported languages,
  and `preserve_formatting: false` at the MCP boundary.
- Delegate workspace canonicalization, extension allowlist, symlink handling,
  binary checks, UTF-8 checks, sensitive filename checks, and source-file
  mutation guarantees to the existing core boundary.
- Do not pass file paths or workspace roots to providers.
- Do not accept provider selection or remote confirmation fields in this
  feature.

Relationships:

- Converts to the existing core file translation request path.
- Produces `TranslationToolSuccess` or `TranslationToolError`.

## TranslationToolSuccess

Represents a successful tool result.

Fields:

- `content`: Exactly one MCP text content item containing the translated text.
- `structured_content.translated_text`: The same translated text for contract
  validation and machine clients.
- `is_error`: `false` or omitted according to the MCP SDK representation.

Validation rules:

- Must not include logs, provider diagnostics, file paths, workspace roots,
  protected segments, or unrelated metadata.
- Must respect the existing output-size limit.

Relationships:

- Created from the existing core success result.

## TranslationToolError

Represents a valid tool call that failed validation or translation execution.

Fields:

- `content`: Exactly one MCP text content item containing `CODE: redacted
  message`.
- `structured_content.code`: Normalized error code from the existing error
  model.
- `structured_content.message`: Redacted actionable message.
- `is_error`: `true`.

Validation rules:

- Must not include source text, translated text, translatable segments, secrets,
  headers, tokens, protected content, workspace roots, or sensitive paths.
- Must preserve the meaning of the underlying core error code.
- Must be used for valid tool calls with invalid arguments or execution failures.

Relationships:

- Created from the existing core failure result or MCP argument validation
  failure.

## ProtocolError

Represents an MCP/JSON-RPC request that cannot be treated as a valid tool call.

Fields:

- `code`: Protocol-level JSON-RPC error code selected by the MCP SDK.
- `message`: Redacted protocol message.

Validation rules:

- Use for malformed requests, invalid protocol shape, and unknown tool names.
- Must not begin translation work.
- Must not include request payload text, paths, or secrets.

Relationships:

- Does not map to a core translation request.

## Server Session

Represents one running MCP server process and client connection.

States:

- `starting`: Process launched, not yet initialized.
- `ready`: Tools can be listed and called.
- `handling_tool`: One tool call is being validated and executed.
- `recoverable_error`: A tool or protocol error was returned; server remains
  ready for the next valid request.
- `shutting_down`: Client closed transport or process received shutdown signal.

Validation rules:

- A recoverable tool or protocol error must not terminate the process.
- stdout must contain only valid MCP messages.
- stderr/log output must be redacted.
