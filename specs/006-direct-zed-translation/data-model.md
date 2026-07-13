# Data Model: Direct Zed Translation

All entities are in-memory Rust values unless explicitly described as
configuration. Source and translated text are never persisted.

## Editor Document Snapshot

Represents the latest text Zed sent for one open document.

Fields:

- `uri`: canonical LSP document URI used only for lookup and safe path
  validation; never sent to a provider or logged in full.
- `version`: monotonically increasing Zed document version.
- `language_id`: `markdown` or `plaintext`.
- `text`: UTF-8 document content, bounded by the 20 KiB input limit before a
  translation can start.
- `worktree_root`: configured process root used for canonical authorization;
  never sent to a provider or diagnostic.

Lifecycle:

- created by `textDocument/didOpen`;
- replaced atomically by `textDocument/didChange` full-content synchronization;
- removed with its preview by `textDocument/didClose`.

## Translation Target

Identifies what the user asked to translate without carrying source text in the
command payload.

Fields:

- `uri`;
- `version`;
- `range`: LSP UTF-16 start/end positions;
- `kind`: `Selection` for a non-empty range or `OpenDocument` for an empty
  range;
- `input_kind`: `Text` or `Markdown` derived from the document.

Validation:

- URI must refer to the active snapshot;
- version must match exactly;
- positions must convert to valid UTF-8 character boundaries;
- selection must be non-blank, within limits, and free of protected/ambiguous
  Markdown overlap;
- open-document targets must pass canonical workspace/file validation.

## Direct Translation Invocation

Represents one execution of `en-es-translator.translate`.

Fields:

- `target`: validated `TranslationTarget`;
- `provider_locality`: `Offline`, `Local`, or `Remote`;
- `confirmation`: `NotRequired`, `Pending`, `Confirmed`, or `Denied`;
- `started_at`: monotonic time for timeout/diagnostic metadata only;
- `outcome`: `PreviewReady` or a normalized `ErrorCode`.

State transitions:

```text
Requested
  -> Rejected
  -> Validated
       -> Translating (offline/local)
       -> AwaitingRemoteConfirmation
            -> Rejected
            -> Translating (confirmed)
       -> PreviewReady
       -> Failed
```

No invocation survives process exit and no invocation text is logged.

## Translation Preview

Read-only translated output available through hover.

Fields:

- `uri` and `version`: bind the preview to one immutable document snapshot;
- `source_range`: range in which hover may reveal the preview;
- `translated_text`: validated output, maximum 40 KiB;
- `input_kind`: controls Markdown/plain-text hover rendering;
- `provider_locality`: safe display label only, never a URL or credential.

Invalidation:

- any newer `didChange` for the URI;
- `didClose`;
- a newer successful invocation for the same URI;
- server shutdown.

## Provider Privacy State

Derived from existing provider configuration, not editor content.

Fields:

- `mode`: mock or configured real provider;
- `locality`: offline, loopback/local, or non-local;
- `allow_remote`: configuration allowlist flag;
- `request_confirmed`: ephemeral boolean, default `false`;
- `api_key_env_name`: optional validated environment variable name; its value
  is never copied into LSP messages or logs.

## Platform Capability Evidence

Documentation-only record used by manual validation.

Fields:

- Zed version and extension API version;
- code action visible/invocable;
- selected range delivered correctly;
- show-message confirmation behavior;
- hover Markdown preview behavior;
- unavailable clipboard/custom-pane capability;
- redacted observation and reviewer result.
