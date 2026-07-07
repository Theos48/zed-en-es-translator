# Quickstart: Zed UX Flow

This quickstart is the reviewer protocol for the Zed UX feature. The operational
walkthrough lives in [docs/zed-ux-flow.md](../../docs/zed-ux-flow.md); this file
records the feature-level validation boundary and results.

## Scope Guard

Included:

- Agent Panel based reading workflow.
- Existing local development extension.
- Existing `translator-en-es` MCP context server.
- Existing `translate_text` and `translate_file` tools.
- Translation-only Agent Profile or equivalent tool-permission setup.
- Synthetic canary validation text when the Agent model route is non-local or
  unknown.
- Manual validation of selection behavior before claiming support.
- No-mutation and redaction evidence.

Excluded:

- Real provider setup.
- Remote network translation.
- Marketplace or registry publication.
- API keys, headers, base URLs, or provider selection.
- Automatic replacement of editor content.

## Repository Preparation

Run from the repository root:

```bash
make zed-extension-prepare
```

Expected:

- The project Docker workflow builds the release `translator-mcp` artifact.
- The prepare script prints the artifact path to configure as `binary_path`.
- No global Rust installation is required by the repository workflow.

Optional repository checks before manual UX validation:

```bash
make test-zed-extension
make test
make fmt
make clippy
```

If Docker, Zed, or Zed local extension prerequisites are missing, record the
blocker. Do not install host tooling as part of this feature without explicit
approval under the workstation policy.

## Zed Setup

1. Open Zed.
2. Run `zed: install dev extension`.
3. Select:

   ```text
   zed-extension/
   ```

4. Configure the extension `binary_path` with the path printed by:

   ```bash
   make zed-extension-prepare
   ```

5. Open the Agent Panel.
6. Use a profile/model combination where the `translator-en-es` MCP tools are
   available.

## Agent Privacy And Permission Setup

Before validating translation behavior, record the Agent model route used by
the thread:

- `local`
- `zed-hosted`
- `provider-key`
- `subscription`
- `gateway`
- `unknown`

If the route is not `local`, or cannot be identified, use only synthetic canary
text in all validation prompts. Do not use sensitive, proprietary, or real
workspace content for a non-local or unknown model route.

Use a translation-only Agent Profile or equivalent permissions:

- `translator-en-es` tools are available.
- Built-in edit/write/delete/move/copy/create-directory tools are denied or
  require confirmation.
- Terminal, URL fetch, and web-search tools are denied or require confirmation.
- Global auto-approval of all Agent tools is not used for no-mutation evidence.

## Direct Text Scenario

Reviewer guide:
[Direct Text Translation](../../docs/zed-ux-flow.md#direct-text-translation).

Use this reviewer prompt in the Agent Panel:

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

- The result appears inside Zed.
- The primary result is readable Spanish text.
- No terminal process is manually started during the request.
- No buffer or file is modified.
- No raw MCP/JSON-RPC payload is the primary output.

## Workspace File Scenario

Reviewer guide:
[Workspace File Translation](../../docs/zed-ux-flow.md#workspace-file-translation).

Create a small validation file inside the workspace, for example:

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

Use this reviewer prompt in the Agent Panel, replacing the workspace root with
the current repository path if Zed does not infer it:

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

Pass criteria:

- The visible result is readable Spanish.
- Markdown structure is preserved.
- Code remains protected in the translated result.
- `tmp/zed-ux-validation.md` remains byte-for-byte unchanged.
- Persistent validation notes record only canary identifiers, hash/length
  metadata, and redacted summaries.

## Selection Validation Scenario

Reviewer guide:
[Selection Support Decision](../../docs/zed-ux-flow.md#selection-support-decision).

Selection is not supported until this scenario passes.

1. Open a file in Zed.
2. Select a short synthetic canary phrase.
3. Add the selection to the Agent Panel thread using the Zed UI path under
   review.
4. Ask the agent to use only `translator-en-es` and `translate_text`.
5. Record whether the actual tool input contains only the intended selected
   canary text. Persist only the canary identifier, length/hash metadata, and a
   redacted summary.

Allowed outcomes:

- `validated_supported`: only the intended selected text is sent, output is
  readable, and no mutation occurs.
- `unsupported`: the current Zed flow cannot safely or reliably map selection
  to the translation tool input.
- `deferred`: host setup, profile support, model support, or Zed behavior could
  not be validated in this iteration.

Do not document selection as supported unless the first outcome is proven.

## Failure Scenarios

Reviewer guides:
[Setup Failure Recovery](../../docs/zed-ux-flow.md#setup-failure-recovery),
[Unsafe Input And Provider Denial](../../docs/zed-ux-flow.md#unsafe-input-and-provider-denial),
and [Redaction Inspection](../../docs/zed-ux-flow.md#redaction-inspection).

Run at least one setup failure:

- leave `binary_path` unset;
- set it to a missing artifact;
- use a stale or non-executable artifact.

Run at least one unsafe or unsupported input denial:

- `../outside.md`;
- unsupported extension;
- provider or remote fields;
- API key, header, base URL, or remote confirmation field.

Pass criteria:

- The error is actionable.
- The error is redacted.
- No network/provider path is enabled.
- No file or buffer is modified.

## Evidence To Record

Use
[manual-validation-template.md](./manual-validation-template.md)
for redacted evidence.

Record:

- Zed version.
- Git branch and revision.
- Commands run.
- Agent model route.
- Agent tool-permission posture.
- Whether `translator-en-es` exposed `translate_text` and `translate_file`.
- Direct text result status.
- Workspace file result status and no-mutation proof.
- Failure scenario status and redaction notes.
- Selection support decision.

## Validation Results

Repository checks recorded on 2026-07-06:

- `make test-zed-ux-flow`: pass. The target ran make-target, documentation,
  evidence-template, privacy-boundary, failure-recovery, and redaction contract
  checks.
- `make test-zed-extension`: pass. The target built and tested the Zed
  extension, prepared the `translator-mcp` artifact, and ran the existing Zed
  wrapper integration checks.
- `make test`: pass. Rust workspace tests passed inside the pinned project
  container.
- `make fmt`: pass. Workspace and Zed extension formatting checks passed inside
  the pinned project container.
- `make clippy`: pass. Workspace and Zed extension clippy checks passed with
  warnings denied inside the pinned project container.
- Manual Zed UX smoke validation: pass with selection deferred. Interactive Zed
  review confirmed `mcp.translator-en-es.translate_text` for synthetic canary
  `ZUX-407`, confirmed `mcp.translator-en-es.translate_file` for synthetic
  canary `ZUX-408`, and confirmed no mutation of `tmp/zed-ux-validation.md`
  with matching SHA-256
  `7485b0ccddd834e8a8d3ab17376cd4d76db106cae144d8d01c6b2d7fe4a70008`.
  Setup failure recovery was exercised with an invalid `binary_path`, unsafe
  path denial returned `PATH_NOT_ALLOWED` for a parent-directory request, and
  redaction inspection passed because only synthetic canary content was used
  and no secrets, tokens, headers, environment dumps, real workspace text, or
  sensitive paths were observed in persistent evidence. Selection support is
  recorded as `deferred` because the selection-to-tool-input flow was not
  validated in this run.

Manual validation evidence details recorded on 2026-07-06:

- Zed version: `1.9.0`.
- Operating system: Fedora Linux host, kernel `7.0.11-200.fc44.x86_64`.
- Git branch/revision: `004-zed-ux-flow` / `25553b4`.
- Extension path category: local `zed-extension/` development extension.
- Prepared artifact category: local `target/release/translator-mcp` artifact
  produced by `make zed-extension-prepare`.
- Agent profile: not captured as a named profile in persistent evidence.
- Agent model route: `unknown`. The run used only synthetic canary content, so
  no sensitive, proprietary, or real workspace content was validated through an
  unknown model route.
- `translator-en-es` tool availability: `translate_text` and `translate_file`
  were available and observed as `mcp.translator-en-es.translate_text` and
  `mcp.translator-en-es.translate_file`.
- Tool-permission posture: mutation-capable and external-access Agent tool
  policy was not captured as a named Zed profile. No automatic edit, write,
  delete, move, copy, terminal, fetch, or search action was observed during the
  run. No-mutation evidence is therefore accepted for the observed translator
  operations only, based on tool observation and matching file hash, while the
  profile-level permission audit remains `blocked` for this run.
- Setup failure evidence: invalid `binary_path` was exercised and reported by
  the reviewer as completed. The visible category was setup/startup failure for
  an invalid local artifact path. Corrective action was to restore the prepared
  `target/release/translator-mcp` path from `make zed-extension-prepare`.
  Evidence persisted only the redacted category and corrective action, and no
  buffer or workspace-file mutation was observed.

## Completion Boundary

This quickstart does not close the feature by itself. The feature can close only
when repository checks pass and the manual Zed UX smoke result is recorded with
synthetic or redacted evidence.
