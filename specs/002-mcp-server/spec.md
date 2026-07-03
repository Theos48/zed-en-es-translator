# Feature Specification: MCP Server

**Feature Branch**: `002-mcp-server`

**Created**: 2026-07-02

**Status**: Implemented

**Input**: User description: "Exponer el core de traduccion como servidor MCP invocable desde Zed con herramientas translate_text y translate_file, validacion de parametros en la frontera MCP, errores MCP accionables con isError true, lectura de archivos delegada al core o CLI sin duplicar reglas operativas, salida limpia en exito, sin red ni proveedor real, sin modificar buffers, sin enviar paths o contenido protegido al proveedor, con pruebas de contrato MCP, errores, privacidad y casos adversariales."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Translate Direct Text Through MCP (Priority: P1)

As a Zed user, I can invoke a translation tool through an MCP-compatible client
with direct English text and receive the Spanish translation as a clean tool
result.

**Why this priority**: This is the smallest end-to-end MCP slice and proves the
new server boundary can expose the existing translation contract without relying
on a Zed wrapper or a real provider.

**Independent Test**: Start the MCP server in a controlled test client, discover
the available tools, call `translate_text` with valid direct text, and verify the
response contains only the expected translation payload for the tool call.

**Acceptance Scenarios**:

1. **Given** the MCP server is running with the default offline provider, **When**
   a client calls `translate_text` with valid English text, **Then** the client
   receives a successful tool result containing the Spanish translation.
2. **Given** the MCP server is running, **When** a client lists tools, **Then**
   `translate_text` is discoverable with its required input fields and bounded
   behavior.

---

### User Story 2 - Translate Allowed Files Through MCP (Priority: P2)

As a Zed user, I can invoke a file translation tool through MCP for an allowed
Markdown or text file inside the authorized workspace and receive translated
content without the source file being modified.

**Why this priority**: File translation is the primary editor workflow, but it
must reuse the existing workspace and Markdown safety guarantees rather than
introducing a second file-reading policy.

**Independent Test**: Start the MCP server in a controlled test client, call
`translate_file` with a workspace root and allowed relative file path, and verify
that translated content is returned while the original file remains unchanged.

**Acceptance Scenarios**:

1. **Given** a workspace contains an allowed Markdown file, **When** a client
   calls `translate_file` for that file, **Then** the client receives translated
   visible text while Markdown code regions remain preserved.
2. **Given** a workspace contains an allowed text file, **When** a client calls
   `translate_file` for that file, **Then** the client receives translated text
   and the file content on disk is unchanged.
3. **Given** a client calls `translate_file` for an unsupported file type,
   **When** the request is handled, **Then** the client receives an actionable
   error result and no provider processing occurs.

---

### User Story 3 - Receive Safe Actionable Errors Through MCP (Priority: P3)

As a Zed user, when a translation request fails, I receive a structured MCP error
result that tells me what category failed without exposing source text,
translations, secrets, protected content, or sensitive paths.

**Why this priority**: The server becomes a new external boundary. It must carry
the existing privacy and error contract across MCP before any future Zed wrapper
or provider work depends on it.

**Independent Test**: Call each MCP tool with invalid parameters and adversarial
inputs, then verify tool validation/execution failures are marked as error tool
results, protocol-shape failures are returned as protocol errors, and all
diagnostics are redacted.

**Acceptance Scenarios**:

1. **Given** a client sends missing or invalid tool arguments, **When** the
   server validates the request, **Then** the client receives an MCP tool result
   with `isError: true`, a normalized validation error, and no server panic.
2. **Given** a client sends a malformed MCP request or unknown tool name,
   **When** the server validates the protocol request, **Then** the client
   receives a protocol-level error and no translation work begins.
3. **Given** a client requests a path traversal, symlink escape, binary file, or
   non-UTF-8 file, **When** `translate_file` handles the request, **Then** the
   client receives an MCP error result and no source content is leaked.
4. **Given** a client attempts to enable or request remote translation, **When**
   the MCP server handles the request in this feature, **Then** the request is
   denied by default with an actionable error result.

---

### Edge Cases

- The MCP client sends unknown tools, missing fields, wrong field types, extra
  fields, empty text, whitespace-only text, oversized text, or malformed request
  payloads.
- A tool request exceeds the existing input, segment, output, or timeout limits.
- `translate_file` receives absolute paths, parent directory traversal,
  root-prefix confusion, symlink escapes, unsupported extensions, binary data,
  NUL bytes, non-UTF-8 content, hidden sensitive files, or credential-like file
  names.
- Markdown content contains protected code regions, HTML blocks, frontmatter,
  links, nested fences, multi-line inline code, or content with no translatable
  segments.
- Provider failures, provider timeouts, malformed provider output, and output
  limit failures must be mapped without leaking source or translation content.
- The server receives a failed tool call and then another valid tool call in the
  same session.
- A future Zed client consumes the tool result; the feature must not depend on
  automatic buffer replacement or hidden editor-side state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST expose exactly two translation tools for this
  feature: `translate_text` and `translate_file`.
- **FR-002**: The system MUST make both tools discoverable to an MCP-compatible
  client with clear names, descriptions, required fields, optional fields, and
  input constraints.
- **FR-003**: `translate_text` MUST accept direct English text and return Spanish
  translated text using the current translation contract and default offline
  provider behavior.
- **FR-004**: `translate_file` MUST accept an authorized workspace root and an
  allowed file path, then delegate file authorization, file reading, content
  validation, and Markdown/text preservation to the existing translation
  boundary instead of defining a second divergent policy.
- **FR-005**: Successful tool results MUST contain the translated content needed
  by the caller and MUST NOT include logs, provider diagnostics, file paths,
  workspace roots, protected segments, or unrelated metadata.
- **FR-006**: Tool validation and execution failures MUST return MCP tool
  results with `isError: true`, a normalized error code, and an actionable
  redacted message.
- **FR-007**: The MCP boundary MUST map existing translation errors to stable MCP
  error results without changing the meaning of the underlying error code.
- **FR-008**: The system MUST reject invalid tool names, missing required
  fields, wrong field types, unsupported fields, malformed payloads, and
  conflicting direct-text/file inputs before translation work begins. Protocol
  shape failures and unknown tools MUST use protocol-level errors; valid tool
  calls with invalid arguments MUST use `isError: true` tool results.
- **FR-009**: The system MUST preserve the existing limits for total input size,
  segment size, segment count, output size, and provider timeout.
- **FR-010**: The system MUST support `translate_file` only for `.md`,
  `.markdown`, and `.txt` in this feature.
- **FR-011**: The system MUST NOT modify editor buffers or source files.
- **FR-012**: The system MUST NOT add a real provider, remote network call, or
  provider selection flow in this feature.
- **FR-013**: The system MUST NOT send file paths, workspace roots, protected
  code, environment variables, logs, or detected secrets to any provider.
- **FR-014**: Logs, stderr, protocol errors, and diagnostics MUST NOT contain
  source text, translated text outside the successful tool payload, translatable
  segments, secrets, headers, tokens, or sensitive paths.
- **FR-015**: The server MUST remain available for a later valid tool request
  after handling a validation or translation failure in the same session.
- **FR-016**: The feature MUST include contract tests for tool discovery,
  successful tool calls, failed tool calls, and error result shape.
- **FR-017**: The feature MUST include negative and adversarial tests for invalid
  parameters, unsafe file requests, malformed input, privacy redaction, remote
  denial, and protected-content preservation.

### Key Entities *(include if feature involves data)*

- **MCP Tool Definition**: Public description of a callable translation tool,
  including name, purpose, accepted fields, and constraints.
- **MCP Tool Request**: A client request to invoke one translation tool with
  structured parameters.
- **MCP Tool Result**: The response returned to the client after a tool call,
  either successful translated content or an error result.
- **MCP Error Result**: A failed tool result marked as an error, carrying a
  normalized code and redacted actionable message.
- **Direct Text Request**: A request containing source text to translate without
  file access.
- **Workspace File Request**: A request containing a workspace root and file path
  that must pass the existing workspace and file safety rules.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A test MCP client can discover exactly `translate_text` and
  `translate_file` with their required inputs and constraints.
- **SC-002**: Valid `translate_text` and `translate_file` calls return successful
  tool results using the existing offline translation behavior.
- **SC-003**: All known translation failure categories are represented as
  `isError: true` MCP tool results with normalized codes and redacted messages,
  while malformed protocol requests and unknown tools return protocol-level
  errors.
- **SC-004**: Negative tests cover invalid parameters, unsupported files,
  traversal, symlink escape, binary input, non-UTF-8 input, oversized input,
  protected-only Markdown, provider failures, and remote denial.
- **SC-005**: Privacy tests confirm that logs, stderr, protocol errors, and
  diagnostics do not expose source text, translated text outside successful
  payloads, paths, secrets, headers, tokens, or protected segments.
- **SC-006**: File mutation tests confirm `translate_file` does not modify the
  source file or any editor buffer.
- **SC-007**: A failed tool request followed by a valid tool request in the same
  server session succeeds without restarting the server.

## Assumptions

- The first MCP server feature uses the existing offline deterministic
  translation behavior; provider configuration and real network providers remain
  out of scope.
- The exact server transport, crate layout, dependency choices, and process
  startup details are planning decisions for `speckit-plan`.
- The server is intended to be usable by Zed later, but the Zed extension,
  manifest, installation workflow, and editor UX are separate future features.
- The existing core/CLI contracts remain the source of truth for translation
  behavior, limits, file safety, and redaction.
- Tool success payloads may contain translated text because that is the requested
  product output; logs, diagnostics, and error paths must not.
