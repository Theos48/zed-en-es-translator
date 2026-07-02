# CLI Wire Contract

## Process Model

The CLI processes exactly one request per invocation.

Input:

- stdin: JSON UTF-8 `TranslateRequest`.
- argv: no source text or translated content.
- provider selection/configuration: not accepted in this feature; provider-like
  fields in stdin are schema violations, and argv must not be used to carry them.
- request shape: exactly one of direct `source_text` or file context
  `workspace_root` + `file_path` is accepted.

Output:

- stdout on success: JSON UTF-8 `TranslateSuccess`.
- stdout on expected failure: JSON UTF-8 `TranslateFailure`.
- stderr: redacted diagnostics only.

Exit codes:

- `0`: stdout contains `TranslateSuccess`.
- non-zero: stdout contains `TranslateFailure` when possible.

Timeout:

- caller enforces 15 s timeout for this feature.

## Redaction Rules

stderr and error messages must not contain:

- source text;
- translated text;
- translatable segments;
- secrets;
- headers;
- tokens;
- raw provider output;
- sensitive paths.

## Example Success

Request:

```json
{
  "source_text": "Read the documentation before changing the code.",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true,
  "input_kind": "text"
}
```

Response:

```json
{
  "translated_text": "Lee la documentacion antes de cambiar el codigo."
}
```

## Example Failure

```json
{
  "code": "FILE_TOO_LARGE",
  "message": "The input exceeds the configured size limit."
}
```
