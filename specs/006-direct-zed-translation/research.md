# Research: Direct Zed Translation

## Decision 1: Use an LSP code action as the native extension action

**Decision**: Register a `translator-lsp` language server for Zed Markdown and
Plain Text buffers. Return a safely locality-labelled
`Translate English to Spanish` action from `textDocument/codeAction` and
execute it through a server-owned command.

**Rationale**: The current [Zed extension documentation](https://zed.dev/docs/extensions/developing-extensions)
lists languages, debuggers, themes, icon themes, snippets, and MCP servers as
extension surfaces. The [`zed_extension_api::Extension` 0.7.0 trait](https://docs.rs/zed_extension_api/0.7.0/zed_extension_api/trait.Extension.html)
has no custom command, editor selection, or custom UI method. Zed does support
language-server code actions and presents them from the editor through
`Ctrl-.`; `CodeActionParams` carries the requested document range.

**Alternatives considered**:

- Agent/MCP or slash command: rejected because Agent is explicitly not the
  product surface for F010.
- Workspace/global task: rejected because it is user configuration rather than
  an action owned and shipped by the extension, and it does not provide the
  required privacy/preview contract.
- Fork Zed or patch Zed core: rejected as an unpublishable host-level product
  change with excessive maintenance and footprint.

## Decision 2: Use Markdown hover as the read-only preview

**Decision**: Cache a successful translation by URI, document version, and
source range. Return the translated text as Markdown from
`textDocument/hover` only while the hovered position belongs to the current
preview range. Send a short safe `window/showMessage` notification telling the
user that the preview is ready.

**Rationale**: Zed advertises Markdown hover support to language servers. The
stable extension API does not expose a custom pane, modal, webview, or clipboard
operation; the upstream [webview extension request](https://github.com/zed-industries/zed/issues/21208)
remains open. Hover keeps output inside Zed, requires no buffer edit, and can be
covered by protocol tests.

**Alternatives considered**:

- `window/showMessage` containing the translation: rejected because a
  notification is not an appropriate surface for output up to 40 KiB.
- Diagnostics or code-lens text: rejected because translations are not errors
  or source annotations and those surfaces would misrepresent state.
- Generated preview file: rejected because it adds persistence, file cleanup,
  and possible content leakage.

## Decision 3: Do not implement copy, insert, or apply in this cycle

**Decision**: The direct result is read-only. No copy command is advertised and
no LSP workspace edit is returned.

**Rationale**: Extension API 0.7.0 exposes no clipboard operation. The project
constitution also forbids buffer modification, so insert/apply would fail the
governance gate even though LSP supports edits.

**Alternatives considered**:

- Workspace edit: rejected by the constitution and FR-005.
- Put translation in command arguments for external clipboard tooling:
  rejected because it leaks content through process arguments and would require
  platform-specific host commands.

## Decision 4: Keep editor protocol in a new native Rust process

**Decision**: Add `crates/translator-lsp/` with `lsp-server` 0.7.x and
`lsp-types` 0.97.x. It calls `translator-core` directly.

**Rationale**: `lsp-server` provides a small synchronous stdio transport and an
in-memory connection for tests. This matches the synchronous provider/core
model, avoids a new async runtime, and keeps editor protocol separate from MCP.

**Alternatives considered**:

- Add LSP to `translator-mcp`: rejected because MCP and LSP have different
  lifecycle, transport, and user-facing contracts.
- Hand-roll JSON-RPC framing: rejected because a maintained protocol scaffold
  already exists and reduces parsing/lifecycle risk.
- Async LSP framework: rejected because concurrency is not needed for one
  request at a time and would widen dependencies without product value.

## Decision 5: Treat document version and UTF-16 range as security boundaries

**Decision**: Store the full in-memory snapshot from `didOpen`/`didChange` with
its version. Command arguments contain URI, version, range, and input kind but
no text. Execution fails if the snapshot version changed or if UTF-16 positions
cannot be converted to valid UTF-8 byte offsets.

For selection translation, core selection analysis must prove that the selected
range is translatable and does not intersect a protected Markdown region. For
an empty range, the server validates the file URI against the canonical
worktree and current `.md`/`.markdown`/`.txt` rules before translating the
snapshot.

**Rationale**: This prevents races between selection and execution, invalid
Unicode slicing, translation of protected code, and bypass of workspace file
policy. Text never needs to be embedded in LSP command payloads or logs.

**Alternatives considered**:

- Read only from disk: rejected because the action targets the current editor
  snapshot and could otherwise translate content different from what the user
  selected.
- Trust the URI/range without version checks: rejected because edits between
  action listing and execution could change the translated content.

## Decision 6: Confirm remote use with a server-to-client LSP request

**Decision**: Parse provider configuration before translation. For an
allowlisted non-local target, send `window/showMessageRequest` with a single
explicit confirm action and a cancel path. Only the matching positive response
sets `remote_confirmed = true` for that invocation.

**Rationale**: Zed's LSP client advertises show-message request support. The
server remains responsible for correlating the response, while
`translator-core` remains the final authority for allowlisting, secret
detection, and denial.

**Alternatives considered**:

- Persist a global consent setting: rejected because confirmation must be per
  request.
- Trust LSP settings `allow_remote`: rejected because allowlisting is
  configuration, not user consent for the current content.

## Decision 7: Reuse validated wrapper settings and separate artifacts

**Decision**: Extend the wrapper launch model with an LSP id and an expected
`translator-lsp` executable. Accept provider selection only through the four
validated `LspSettings.binary.env` keys and add a separate
`prepare-direct.sh`/Make target while leaving the existing MCP prepare contract
intact.

**Rationale**: Zed's [`LspSettings::for_worktree`](https://docs.rs/zed_extension_api/0.7.0/zed_extension_api/settings/struct.LspSettings.html)
provides binary launch settings. Real Zed 1.10.3 validation showed that custom
nested server settings did not affect the launched process provider, while the
standard binary environment reached the command adapter. Separate artifacts
avoid silently changing existing context-server setup and tests.

**Alternatives considered**:

- Replace `translator-mcp`: rejected because F007 compatibility remains useful
  and F010 does not require breaking it.
- One multi-protocol binary: rejected because invocation mode and lifecycle
  would become implicit and harder to validate.
