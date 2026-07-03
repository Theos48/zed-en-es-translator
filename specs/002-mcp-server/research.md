# Research: MCP Server

## Decision: Implement the MCP server in Rust with `rmcp`

Use a new Rust workspace member, `translator-mcp`, built on `rmcp = "2.1.0"`.
`cargo search` and `cargo info` were checked on 2026-07-02 from the project
Rust container; `rmcp` is described as the Rust SDK for the Model Context
Protocol, with repository `modelcontextprotocol/rust-sdk`.

Rationale:

- The project already has a Rust core that owns translation, segmentation,
  limits, file safety, redaction, and provider behavior.
- Calling `translator-core` directly avoids an extra TypeScript-to-CLI boundary,
  avoids spawning subprocesses per request, and reduces privacy/logging surfaces.
- The official Rust SDK documents stdio server support, tool discovery, tool
  calling, macros, and generated schemas.
- This choice avoids adding a project Node runtime for F005 and keeps the host
  footprint aligned with the existing Docker Rust workflow.

Alternatives considered:

- TypeScript MCP server calling `translator-cli`: previously planned, but it
  would add a Node toolchain and a second JSON boundary before the Zed wrapper
  exists.
- Hand-written JSON-RPC/MCP framing: rejected because MCP is evolving and a
  maintained SDK reduces protocol drift risk.
- HTTP MCP server: rejected for this feature because it opens a network surface
  and triggers additional origin/authentication requirements.

## Decision: Use stdio transport only

Use MCP stdio transport for the first MCP server feature.

Rationale:

- The MCP transport spec defines stdio as client-launched subprocess
  communication over stdin/stdout, and requires stdout to contain only valid MCP
  messages.
- Zed MCP server extensions start a command and return command, args, and env
  from `context_server_command`; a stdio server is the natural binary target for
  the future wrapper.
- Stdio avoids local ports, HTTP origin validation, DNS rebinding concerns, and
  remote/network confusion in this offline-only feature.

Alternatives considered:

- Streamable HTTP: useful later for remote or multi-client scenarios, but out of
  scope because this feature must not add network behavior.
- Custom transport: rejected because stdio is already standard and sufficient.

## Decision: Expose tools only, not MCP resources or prompts

Implement only the `tools` capability with `translate_text` and
`translate_file`.

Rationale:

- The feature goal is invocation from Zed/Agent Panel, not publishing resources
  or prompt templates.
- MCP tools provide discoverable names, descriptions, input schemas, and
  call-tool behavior, matching the product contract.
- Keeping the tool list to exactly two tools reduces tool-selection ambiguity
  and makes review of tool metadata tractable.

Alternatives considered:

- Resources for translated files: rejected because translation output is a tool
  result and resource URIs would introduce path/metadata privacy concerns.
- Prompts for translation instructions: rejected because the translation rules
  are deterministic product behavior, not user-editable prompt templates.

## Decision: Map errors according to MCP error categories

Use protocol-level errors for malformed MCP requests and unknown tools. Use
`isError: true` tool results for valid tool calls that fail validation or
translation execution.

Rationale:

- The MCP tools spec distinguishes protocol errors from tool execution errors.
- Tool execution errors are actionable and can include normalized translation
  error codes for client/user correction.
- Protocol-shape failures are less recoverable and should remain JSON-RPC
  errors.
- All error messages must pass the existing redaction rules and must not include
  source text, translated text, protected segments, secrets, or sensitive paths.

Alternatives considered:

- Return every failure as `isError: true`: rejected because it conflicts with
  MCP's protocol-vs-execution distinction.
- Return only generic JSON-RPC errors: rejected because it loses normalized
  domain error codes and makes safe recovery harder.

## Decision: Use translated text as the visible content, structured content for tests

Successful tool results return one text content item with the translated text.
They may also include structured content containing `translated_text` for stable
contract tests and machine clients. Error tool results return one text content
item with `CODE: redacted message` and structured content containing `code` and
`message`.

Rationale:

- Zed users need a clean readable result.
- Structured content gives contract tests a stable shape without adding logs or
  unrelated metadata to the visible content.
- The tool contract can remain stable even if future UI wrappers render the
  structured content differently.

Alternatives considered:

- Text-only output: simplest for users, but weaker for contract tests and client
  integrations.
- JSON-only text output: machine-friendly but worse for the Agent Panel reading
  workflow.

## Decision: Defer Zed extension packaging to F006

Do not add `extension.toml`, extension Rust/WASM code, install-dev-extension
workflow, or registry publication in F005.

Rationale:

- Zed documents MCP server extensions as command-launching wrappers and also
  notes a deprecation path toward the official MCP registry. The server binary
  should be validated independently before deciding packaging.
- Keeping F005 server-only preserves a narrow TDD surface and avoids mixing MCP
  protocol work with extension packaging.

Alternatives considered:

- Build the Zed wrapper immediately: rejected because it combines F005 and F006
  and makes failures harder to isolate.
- Publish to registry first: rejected because the server has not been validated
  locally yet.
