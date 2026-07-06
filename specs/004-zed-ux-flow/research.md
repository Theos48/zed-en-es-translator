# Research: Zed UX Flow

## Decision: Use Agent Panel plus MCP tools as the baseline UX surface

Use Zed's Agent Panel as the user-facing translation surface and the existing
`translator-en-es` MCP context server as the tool source. The user-facing flow
should guide the agent toward the existing `translate_text` and `translate_file`
tools and should not require the user to manually start `translator-mcp`.

**Rationale**:

- Zed documents MCP servers as available to the Agent Panel and supports MCP
  tools through context servers.
- The previous feature already produced a local development extension that
  returns a `context_server_command` for the prepared `translator-mcp` artifact.
- Keeping the Agent Panel path avoids inventing a custom editor UI or adding a
  second command surface before the existing integration has been validated as a
  real product workflow.

**Sources**:

- Zed MCP server extension documentation:
  <https://zed.dev/docs/extensions/mcp-extensions>
- Zed Agent Panel documentation:
  <https://zed.dev/docs/ai/agent-panel>
- Zed tools documentation:
  <https://zed.dev/docs/ai/tools>
- Prior wrapper quickstart:
  `specs/003-zed-wrapper/quickstart.md`

**Alternatives considered**:

- Custom Zed command: rejected for this iteration because the extension already
  exposes an MCP server and the feature goal is to validate a low-friction
  reading flow before adding editor-specific command behavior.
- Inline Assistant: deferred because it would introduce another UX path and
  does not remove the need to prove which content is sent.
- Separate web or terminal UI: rejected because the feature requirement is to
  complete translation inside Zed.

## Decision: Keep direct text and authorized workspace file input as the supported baseline

The supported input paths for this feature are:

- direct text through the existing `translate_text` MCP tool;
- authorized workspace files through the existing `translate_file` MCP tool.

**Rationale**:

- Both paths already have explicit MCP contracts, schemas, error shapes, and
  safety expectations.
- The workspace-file path already inherits core protections for canonicalized
  workspace access, supported file types, text encoding, binary rejection,
  sensitive-file rejection, and no source-file mutation.
- Adding a third path before validating the current tool UX would expand scope
  without improving the core reading workflow.

**Sources**:

- MCP tool contract:
  `specs/002-mcp-server/contracts/mcp-tools.md`
- Direct text schema:
  `specs/002-mcp-server/contracts/translate-text.input.schema.json`
- File input schema:
  `specs/002-mcp-server/contracts/translate-file.input.schema.json`

**Alternatives considered**:

- Accept arbitrary pasted editor context: rejected because it blurs what is
  intentionally sent to the translator.
- Accept provider or remote fields in the UX prompt: rejected because provider
  selection and remote translation remain out of scope.

## Decision: Gate selection support on real Zed validation

Selection-based translation must remain `unvalidated` until a reviewer confirms
what Zed sends when selected content is added to an Agent Panel thread and how
that context is represented when an MCP tool call is made.

**Rationale**:

- Zed documents ways to add selection as context, but this project still needs
  local evidence that the selection path maps to the intended translation input
  without sending extra workspace context or causing unintended edits.
- Zed's tool list can vary by Agent Profile, selected model provider, and Zed
  version, so the feature must record the actual environment before claiming
  support.
- The constitution requires explicit boundaries and forbids inferring or
  sending content that has not been intentionally provided.

**Sources**:

- Agent Panel selection context:
  <https://zed.dev/docs/ai/agent-panel>
- Zed tools and profile variability:
  <https://zed.dev/docs/ai/tools>
- Zed tool permissions:
  <https://zed.dev/docs/ai/tool-permissions>

**Alternatives considered**:

- Claim selection support from documentation alone: rejected because the
  project needs evidence of actual translation-tool input and no-mutation
  behavior in this extension flow.
- Permanently exclude selection: deferred rather than rejected. Selection may
  become supported if manual validation proves it is bounded and safe.

## Decision: Treat the Zed Agent model route as a separate privacy boundary

The local `translator-en-es` MCP server remains offline/mock, but the Agent
Panel still uses a selected model route to orchestrate the thread and tool
calls. Validation must therefore record whether the Agent model route is local,
Zed-hosted, provider-key based, subscription based, gateway based, or unknown.
Sensitive or proprietary workspace content may be validated only with a local
or self-hosted model route; non-local and unknown routes are limited to
synthetic canary text.

**Rationale**:

- Zed's AI privacy documentation separates model request paths from tool/MCP
  behavior.
- A local MCP translator does not by itself prove that the Agent thread content
  remains local.
- Synthetic canary text is sufficient for UX validation while avoiding
  unnecessary exposure of real workspace content.

**Sources**:

- Zed AI privacy and request paths:
  <https://zed.dev/docs/ai/privacy-and-security>
- Zed LLM providers:
  <https://zed.dev/docs/ai/llm-providers>

**Alternatives considered**:

- Treat Agent Panel as local because the MCP server is local: rejected because
  the Agent model route is a separate trust boundary.
- Require a local model for all validation: rejected for this iteration because
  it would add host setup outside the feature scope; synthetic validation keeps
  the review path practical.

## Decision: Require a translation-only Agent Profile or equivalent permissions

Manual UX validation must use a profile or tool-permission setup that keeps the
thread focused on `translator-en-es` and prevents unrelated built-in tools from
mutating files or reaching external systems automatically.

**Rationale**:

- No-mutation cannot rely only on prompt wording when Agent tools may include
  edit, write, delete, move, terminal, fetch, or web-search capabilities.
- Zed documents tool-permission controls and matching rules for built-in tools.
- Restricting the validation profile makes no-mutation evidence reviewable and
  reproducible.

**Sources**:

- Zed tool permissions:
  <https://zed.dev/docs/ai/tool-permissions>
- Zed tools documentation:
  <https://zed.dev/docs/ai/tools>

**Alternatives considered**:

- Rely on the prompt instruction "do not edit": rejected because prompts are
  advisory and do not enforce tool availability.
- Disable Agent tools entirely: rejected because the local MCP translation tool
  must remain available for the flow under validation.

## Decision: Require manual Zed smoke evidence for the UX claim

Automated repository checks remain necessary, but they are not sufficient to
prove the in-editor user experience. This feature must record manual Zed
validation evidence for the success path, unsafe/unsupported input denial,
setup failure, no-mutation behavior, redaction, and selection-support decision.

**Rationale**:

- The feature's main outcome is a user-visible workflow inside Zed.
- Automated tests can validate contracts, launch profiles, artifact
  preparation, and redaction helpers, but they cannot prove how the real Agent
  Panel displays the result or which context a reviewer added.
- Keeping the evidence contract explicit makes later provider and publication
  work less risky.

**Sources**:

- Zed Agent Panel documentation:
  <https://zed.dev/docs/ai/agent-panel>
- Zed MCP documentation:
  <https://zed.dev/docs/ai/mcp>
- Prior manual Zed smoke notes:
  `specs/003-zed-wrapper/quickstart.md`

## Decision: Keep provider, remote, publication, and automatic replacement out of scope

This feature must not add real provider support, remote network translation,
marketplace publication, API-key setup, arbitrary provider configuration, or
automatic replacement/editing of editor content.

**Rationale**:

- The constitution requires offline/mock by default and request-level privacy
  boundaries before any remote provider work.
- Zed supports both local and remote MCP server configuration, but this project
  currently exposes a local development extension and must not silently widen
  the network boundary.
- Automatic replacement would weaken the safety-first product boundary and is
  not necessary for reading assistance.

**Sources**:

- Zed MCP local and remote server configuration:
  <https://zed.dev/docs/ai/mcp>
- Project constitution:
  `.specify/memory/constitution.md`
- Zed wrapper contract:
  `specs/003-zed-wrapper/contracts/zed-extension.md`

## Decision: Track MCP extension deprecation as a publication risk, not a blocker

The current feature can continue using the local development extension path,
but later packaging/publication work must revisit Zed's note that MCP server
extensions are planned for deprecation in favor of the official MCP registry.

**Rationale**:

- F007 validates local in-editor UX, not public distribution.
- F009 owns publication and should decide whether the extension, the registry,
  or both are required.

**Sources**:

- Zed MCP server extension documentation:
  <https://zed.dev/docs/extensions/mcp-extensions>
