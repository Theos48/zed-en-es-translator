# Data Model: Translation Core Contract

## TranslateRequest

Represents one translation operation.

Fields:

- `source_text`: string, required for direct text requests; omitted from public
  file requests and populated internally from a validated file read.
- `source_language`: enum, currently only `"en"`.
- `target_language`: enum, currently only `"es"`.
- `tone`: enum, currently only `"technical_neutral"`.
- `preserve_formatting`: boolean, must be `true` in this feature.
- `input_kind`: enum, `"text"` or `"markdown"` for this feature.
- `file_path`: optional string, local-only context; never sent to providers.
- `workspace_root`: optional string for file requests; must be canonicalized
  before file access.

Validation:

- total input must be at most 20 KiB UTF-8;
- direct `source_text` must contain at least one non-whitespace character;
- `preserve_formatting` must be true;
- unsupported language pairs fail with `UNSUPPORTED_LANGUAGE_PAIR`;
- direct text requests require `source_text` and must not include file context;
- file requests require `workspace_root` and `file_path` and must not include
  caller-supplied `source_text`;
- provider config is not accepted in the request;
- any provider-selection or provider-config field supplied through the public
  request is rejected by strict validation.

## TranslateSuccess

Represents a successful translation result.

Fields:

- `translated_text`: string.

Validation:

- output must be at most 40 KiB UTF-8;
- no metadata appears in normal success output.

## TranslateFailure

Represents a failed translation result.

Fields:

- `code`: `ErrorCode`.
- `message`: user-readable redacted string.

Validation:

- message must not include source text, translated text, secrets, headers,
  tokens, raw provider output, or sensitive paths.

## ErrorCode

Stable error categories:

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

## TranslatableSegment

Represents text approved for provider processing.

Fields:

- `id`: stable segment id within a request.
- `text`: UTF-8 segment text.

Validation:

- segment text must be at most 4 KiB UTF-8;
- maximum 256 segments per request;
- protected content must not appear in segment text.

## ProtectedContent

Represents content preserved unchanged during reconstruction.

Kinds:

- Markdown fenced code.
- Markdown inline code.
- Markdown unclosed or ambiguous fences.
- Markdown HTML blocks.
- Markdown frontmatter.
- Ambiguous content.
- Content containing obvious secret patterns before remote processing.

Validation:

- protected content is never sent to providers;
- reconstruction must preserve protected content byte-for-byte when possible.

## ProviderRequest

Represents the only data a provider may receive.

Fields:

- `segments`: array of strings.
- `source_language`: `"en"`.
- `target_language`: `"es"`.
- `tone`: `"technical_neutral"`.

Forbidden fields:

- file path;
- workspace root;
- protected content;
- environment variables;
- secrets;
- logs.

## ProviderResponse

Represents provider output.

Fields:

- `translated_segments`: array of strings.

Validation:

- segment count must match request segment count;
- output size must respect the 40 KiB result limit;
- malformed output maps to a normalized provider/internal error.
