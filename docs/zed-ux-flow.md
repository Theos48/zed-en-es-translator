# Zed UX Flow

This guide defines the supported reviewer flow for the local English to Spanish
translator inside Zed. It is intentionally a reading workflow: the translator
returns visible Spanish output, and the user decides whether to copy it. The
flow does not replace editor text, edit files, configure real providers, enable
remote translation, or publish the extension.

## Scope

Supported for this feature:

- local development extension `zed-extension/`;
- context server `translator-en-es`;
- MCP tools `translate_text` and `translate_file`;
- supported input paths for direct text, workspace file, and gated selection
  validation;
- synthetic validation text for non-local or unknown Agent model routes;
- manual selection-support decision before claiming selection support;
- redacted validation evidence.

Out of scope:

- real translation providers;
- remote MCP servers or network translation;
- API keys, headers, `base_url`, `provider`, or `remote_confirmation`;
- automatic editor replacement;
- marketplace or registry publication.

## Repository Preparation

Prepare the local server artifact from the repository root:

```bash
make zed-extension-prepare
```

The command prints the prepared `translator-mcp` artifact path. Use that value
as the extension `binary_path`.

No manual `translator-mcp` process should be started during the Zed translation
request. Zed should start the prepared artifact through the local development
extension.

## Zed Setup

1. Open Zed.
2. Run `zed: install dev extension`.
3. Select `zed-extension/`.
4. Configure `binary_path` to the path printed by `make zed-extension-prepare`.
5. Open the Agent Panel.
6. Use an Agent profile where `translator-en-es` exposes `translate_text` and
   `translate_file`.

## Agent Privacy And Permission Setup

The local MCP translator and the Zed Agent model route are separate trust
boundaries. Record the Agent model route for the thread as one of:

- `local`
- `zed-hosted`
- `provider-key`
- `subscription`
- `gateway`
- `unknown`

If the route is non-local or unknown, use only synthetic canary text. Do not
validate this flow with sensitive, proprietary, or real workspace content on a
non-local or unknown model route.

Use a translation-only Agent profile or equivalent tool permissions:

- `translator-en-es` tools are available.
- `edit_file`, `write_file`, `delete_path`, `move_path`, `copy_path`, and
  `create_directory` are denied or require confirmation.
- `terminal`, `fetch`, and `search` tools are denied or require confirmation.
- Global auto-approval of all Agent tools is not accepted for no-mutation
  evidence.

When explicit MCP tool permissions are needed, use Zed's
`mcp:<server>:<tool>` naming shape for the local translator tools.

## Direct Text Translation

Use this prompt in the Agent Panel:

```text
Use only the local translator-en-es MCP tool translate_text. Translate exactly
this English text to Spanish, preserve formatting, and do not edit any file or
buffer:

Synthetic canary ZUX-407 says: Read the docs before changing the implementation.
```

Expected tool input:

```json
{
  "source_text": "Synthetic canary ZUX-407 says: Read the docs before changing the implementation.",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true
}
```

Pass criteria:

- `translate_text` is used.
- The primary visible result is readable Spanish text.
- No manual `translator-mcp` process is started during the request.
- No raw MCP or JSON-RPC payload is the primary visible result.
- No buffer or file is modified.

## Workspace File Translation

Create a temporary validation file inside the workspace:

```text
tmp/zed-ux-validation.md
```

Suggested content:

````markdown
# Notes

Synthetic canary ZUX-408 says: Read the documentation first.

```rust
fn main() {
    println!("keep code intact");
}
```
````

Record a hash before the request:

```bash
sha256sum tmp/zed-ux-validation.md
```

Use this prompt in the Agent Panel:

```text
Use only the local translator-en-es MCP tool translate_file. Translate the
workspace file tmp/zed-ux-validation.md from English to Spanish, preserve
formatting, and do not edit the file or any open buffer.
```

Expected tool input shape:

```json
{
  "workspace_root": "<authorized workspace root>",
  "file_path": "tmp/zed-ux-validation.md",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true
}
```

Record the hash after the request and compare it with the original value.

Pass criteria:

- `translate_file` is used.
- The visible result is readable Spanish text.
- Markdown structure is preserved.
- Code remains protected in the translated result.
- The source file remains byte-for-byte unchanged.
- Persistent notes contain only canary identifiers, hash/length metadata, and
  redacted summaries.

## Selection Support Decision

Selection is not supported until the real Zed flow is validated.

1. Open a Zed buffer containing a short synthetic canary phrase.
2. Select only that phrase.
3. Add the selection to the Agent Panel thread using the UI path under review.
4. Ask the Agent to use only `translator-en-es` and `translate_text`.
5. Record whether the actual tool input contains only the intended canary text.

Allowed decisions:

- `validated_supported`: only the intended selected text is sent, the output is
  readable, and no mutation occurs.
- `unsupported`: the current Zed flow cannot safely or reliably map the
  selection to the translation tool input.
- `deferred`: host setup, profile support, model support, or Zed behavior could
  not be validated in this iteration.

Persistent evidence must use canary identifiers, hash/length metadata, and
redacted summaries. It must not persist real workspace text.

## Setup Failure Recovery

Trigger at least one setup failure and record the redacted result:

- missing `binary_path`;
- missing artifact;
- stale artifact;
- non-executable artifact.

Pass criteria:

- The visible failure names the setup category.
- The corrective action points to `make zed-extension-prepare` and configuring
  `binary_path`.
- No full sensitive path, environment dump, source text, translated text,
  secret, token, or header appears.
- The editor remains unchanged.

## Unsafe Input And Provider Denial

Trigger at least one unsafe or unsupported input denial:

- `../outside.md`;
- unsupported extension;
- binary or non-UTF-8 content;
- sensitive hidden filename;
- `provider`;
- `remote_confirmation`;
- `api_key`;
- `headers`;
- `base_url`.

Pass criteria:

- The request is denied.
- No network/provider path is enabled.
- The visible error is actionable and redacted.
- No source file or buffer is modified.

## Redaction Inspection

Inspect Agent Panel output and repository-visible logs produced during manual
validation.

Persistent evidence must not contain:

- source text from real workspace content;
- translated text from sensitive inputs;
- secrets;
- tokens;
- headers;
- environment dumps;
- workspace roots;
- sensitive paths.

Persistent evidence may contain:

- synthetic canary identifiers;
- hash/length metadata;
- redacted summaries;
- pass, fail, or blocked status for each scenario.

## Completion Boundary

The feature can close only when repository checks pass and manual Zed validation
has a recorded pass or explicit blocked reason for each scenario. A blocked host
prerequisite must be recorded without installing host tooling outside the
workstation policy.
