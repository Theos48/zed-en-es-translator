# Contract: Zed UX Flow

## Scope

This contract defines the user-visible Zed workflow for reading translated
English content in Spanish. It does not define a new translation engine, network
provider, storage layer, marketplace package, or automatic editor edit.

## Supported User Surface

The canonical surface is Zed's Agent Panel with the local development extension
registered as the `translator-en-es` MCP context server.

The flow starts after:

- `make zed-extension-prepare` has prepared the local `translator-mcp`
  artifact;
- the user has installed `zed-extension/` as a Zed dev extension;
- the extension has `binary_path` configured to the prepared artifact path;
- the Agent Panel thread can access the `translator-en-es` MCP tools through
  the selected profile/model combination.
- the reviewer records the Agent model route separately from the local MCP
  translator route.
- the reviewer uses a translation-only Agent Profile or equivalent permission
  setup for no-mutation validation.

## Supported Input Paths

### Direct Text

Tool: `translate_text`

Contract source:

- `specs/002-mcp-server/contracts/translate-text.input.schema.json`
- `specs/002-mcp-server/contracts/mcp-tools.md`

Required behavior:

- Translate only the direct English text intentionally supplied by the user.
- Use `source_language = "en"`.
- Use `target_language = "es"`.
- Use `tone = "technical_neutral"`.
- Use `preserve_formatting = true`.
- Keep the existing input and output limits.
- Produce readable Spanish text as the primary visible result.

Rejected behavior:

- Provider selection.
- Remote confirmation.
- File paths or workspace roots.
- Automatic editor replacement.
- Raw protocol output in the primary answer.

### Authorized Workspace File

Tool: `translate_file`

Contract source:

- `specs/002-mcp-server/contracts/translate-file.input.schema.json`
- `specs/002-mcp-server/contracts/mcp-tools.md`

Required behavior:

- Translate only a file inside the authorized workspace.
- Preserve the existing workspace-only, canonicalization, symlink, extension,
  encoding, binary-content, sensitive-file, and size-limit checks.
- Return translated content as a visible result.
- Leave the source file unchanged.

Rejected behavior:

- Reading outside the authorized workspace.
- Unsupported file types.
- Sensitive hidden files or credential-like filenames.
- Provider or remote fields.
- Source-file mutation.

### Selection Context

Status: `unvalidated` until manual evidence says otherwise.

Required behavior before claiming support:

- Record Zed version, Agent Profile, model/provider, and how the selection was
  added to the thread.
- Confirm whether the MCP tool call receives only the intended selected text.
- Confirm the flow does not include extra workspace context beyond what the
  user intentionally added.
- Confirm no editor buffer, selection, or source file is modified.

If these conditions are not met, selection support must be documented as
`unsupported` or `deferred` for this feature.

## Success Output Contract

For successful translations:

- The primary visible answer is translated Spanish text.
- The user does not need to inspect terminal output, JSON-RPC frames, raw MCP
  payloads, or debug logs.
- Any metadata shown by Zed must be secondary to the readable answer.
- Source files and editor buffers remain unchanged.

## Failure Output Contract

For failures:

- The visible result identifies the category when possible:
  `setup`, `unsupported_input`, `unsafe_file`, `translation_failure`,
  `remote_or_provider_denied`, or `unknown`.
- The result provides a corrective action when the user can act.
- The result must not expose source text, translated text, translatable
  segments, secrets, tokens, headers, environment dumps, workspace roots, or
  sensitive unredacted paths.
- A failure must not modify editor content.

## Permission And Profile Contract

The UX documentation must acknowledge that Agent Panel tool availability can
depend on Zed profile, model/provider support, and permissions. The feature may
document the minimum successful setup but must not require broad auto-approval
of unrelated tools.

For MCP tools, permission documentation should refer to the Zed `mcp:<server>:<tool>`
permission naming shape when reviewer setup needs explicit rules.

Accepted validation posture:

- `translator-en-es` tools are available for the thread.
- Built-in editor mutation tools such as `edit_file`, `write_file`,
  `delete_path`, `move_path`, `copy_path`, and `create_directory` are denied or
  require confirmation.
- `terminal`, URL fetch, and web-search tools are denied or require
  confirmation.
- Global auto-approval of all Agent tools is not accepted for no-mutation
  evidence.

## Agent Model Privacy Contract

The local translator provider boundary and the Zed Agent model route are
separate. This feature validates a local MCP translation tool, not a remote
model privacy guarantee.

Manual validation must record the Agent model route as one of:

- `local`
- `zed-hosted`
- `provider-key`
- `subscription`
- `gateway`
- `unknown`

If the route is not `local`, or cannot be identified, validation content must be
synthetic canary text. Sensitive, proprietary, or real workspace content must
not be used to validate the UX on a non-local or unknown route.

## Evidence Contract

Validation notes may contain:

- synthetic canary phrases created only for the test;
- length, hash, or count metadata;
- redacted summaries of tool inputs and visible results;
- pass/fail status for each required scenario.

Validation notes must not contain:

- real source text;
- translated text from sensitive inputs;
- secrets, tokens, headers, provider credentials, or API keys;
- workspace roots or sensitive unredacted paths;
- raw environment dumps.

## Out Of Scope

- Real translation providers.
- Remote MCP servers or network translation.
- API-key setup.
- Marketplace or registry publication.
- Automatic editor replacement.
- Batch translation or source-code full-file translation.
- Arbitrary provider, header, environment, argument, or base URL configuration.
- Blessing a remote Agent model route for sensitive translation content.
