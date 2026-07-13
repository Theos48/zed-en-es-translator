# Contract: Direct Zed Workflow

## Entry Point

For an open Markdown or Plain Text buffer, Zed's code-action menu MUST offer one
of these titles according to the configured provider locality:

```text
Translate English to Spanish [offline]
Translate English to Spanish [local]
Translate English to Spanish [remote - confirmation required]
```

The title MUST NOT contain a provider name, model, URL, executable, or other
configuration value.

The workflow MUST be available through Zed's standard code-action command
(`Ctrl-.` on Linux) and MUST NOT require Agent Panel, Agent profiles, a prompt,
or a model.

## Target Selection

- Non-empty Zed range: translate that exact versioned selection.
- Empty range: translate the current authorized open-document snapshot.
- Multiple selections are out of scope because the LSP request supplies one
  range; the server MUST NOT guess or merge unrelated ranges.
- Invalid, stale, blank, protected, ambiguous, unsupported, unauthorized, or
  over-limit targets fail before provider contact.

## Successful Flow

1. User opens the standard code-action menu.
2. User chooses the locality-labelled `Translate English to Spanish` action.
3. The extension validates the target and provider state.
4. For a remote target only, Zed asks for confirmation for this request.
5. Translation runs through `translator-core`.
6. Zed shows the safe message `Translation preview ready. Hover the source range to read it.`
7. Hovering within the version-bound source range displays the full validated
   translation as Markdown/plain text.

The source document MUST remain unchanged throughout. The code action MUST NOT
contain a `WorkspaceEdit`.

## Remote Confirmation

The confirmation message MUST state that selected/document content would leave
the machine and identify the provider only as remote, without including URL,
path, source text, translation, token, or secret.

Actions:

- `Send this request`: continue with `remote_confirmed = true`.
- dismiss/no response/any other response: return
  `REMOTE_CONFIRMATION_REQUIRED` without provider contact.

Configuration allowlisting alone is never confirmation. The core secret gate
remains authoritative after confirmation and before network contact.

## Preview Contract

- Preview is returned only for the same URI and version that produced it.
- Selection preview is visible only while hovering inside the source range.
- Open-document preview may be visible while hovering anywhere in the current
  document.
- A document change clears the preview before processing the new content.
- No translated content appears in notifications, errors, logs, command
  arguments, diagnostics, files, or settings.
- No copy, insert, replace, or apply command is exposed in this feature.

## Safe Failure Messages

Failures expose a stable existing `ErrorCode` and one generic actionable
message. They MUST NOT expose source, translation, provider response, URI/path,
workspace root, URL, token, headers, API-key values, or environment values.
