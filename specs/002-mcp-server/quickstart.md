# Quickstart: MCP Server

## Prerequisites

- Docker available on the host.
- No global Rust installation required for this project.
- Use the project `Makefile`; it builds and runs the pinned Rust toolchain
  container.

## Scope Guard

This feature validates only the MCP server boundary:

- included: `translate_text`, `translate_file`, stdio MCP server, contract tests,
  privacy tests, adversarial file/request tests;
- excluded: Zed extension wrapper, `extension.toml`, MCP registry publication,
  real providers, network transports, remote provider configuration, and buffer
  edits.

## Planned Validation Commands

After implementation tasks complete, run:

```bash
make test
make fmt
make clippy
```

The implementation phase should also add:

```bash
make test-mcp
```

Expected result:

- all MCP contract tests pass;
- all existing core and CLI tests from feature 001 continue to pass;
- no formatter or Clippy warnings remain.

## Contract Validation Scenarios

### Tool Discovery

Start the MCP server through a test client over stdio.

Expected:

- tool list contains exactly `translate_text` and `translate_file`;
- each tool has a description and input schema;
- no resources, prompts, provider-selection tools, or remote/network tools are
  advertised.

### Direct Text Success

Call `translate_text` with:

```json
{
  "source_text": "Read the docs.",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true
}
```

Expected:

- result is not an error;
- visible text content is `Lee la documentacion.`;
- structured content contains only `translated_text`.

### File Success

Call `translate_file` for an allowed Markdown fixture inside the workspace.

Expected:

- result is not an error;
- visible text is translated;
- Markdown code regions remain unchanged;
- the source file on disk is unchanged.

### Tool Execution Error

Call `translate_file` with a traversal path such as `../secret.md`.

Expected:

- result has `isError: true`;
- structured content contains a normalized code such as `PATH_NOT_ALLOWED`;
- visible error message is redacted and actionable;
- no source path, workspace root, source text, translated text, or secret appears
  in content, stderr, logs, or diagnostics.

### Protocol Error

Call an unknown tool name or send malformed MCP request shape.

Expected:

- response is protocol-level error;
- translation work does not start;
- diagnostics are redacted.

### Session Recovery

In one test client session:

1. Send an invalid tool request.
2. Send a valid `translate_text` request.

Expected:

- first response is a safe error;
- second response succeeds without restarting the server.

## Privacy Validation

Tests must assert that these values do not appear in errors, logs, stderr, or
protocol diagnostics:

- source text;
- translated text outside successful tool payloads;
- translatable segments;
- protected Markdown/code regions;
- workspace roots;
- sensitive paths;
- secrets, tokens, headers, or environment values.

## Host Policy

Rust dependencies are project dependencies and must be resolved inside the
container workflow. Do not install `rustc`, `cargo`, Node, npm, or MCP tools
globally on Fedora for this feature.

## Implementation Notes

### User Story 1

Implemented tests:

- `crates/translator-mcp/tests/mcp_tool_discovery.rs`
- `crates/translator-mcp/tests/mcp_translate_text.rs`
- `crates/translator-mcp/tests/mcp_translate_text_errors.rs`

Validated through `make test-mcp` in the project Docker workflow.

### User Story 2

Implemented tests:

- `crates/translator-mcp/tests/mcp_translate_file.rs`
- `crates/translator-mcp/tests/mcp_markdown_preservation.rs`
- `crates/translator-mcp/tests/mcp_translate_file_errors.rs`

Validated through `make test-mcp` in the project Docker workflow.

### User Story 3

Implemented tests:

- `crates/translator-mcp/tests/mcp_protocol_errors.rs`
- `crates/translator-mcp/tests/mcp_file_attacks.rs`
- `crates/translator-mcp/tests/mcp_privacy.rs`
- `crates/translator-mcp/tests/mcp_remote_denial.rs`
- `crates/translator-mcp/tests/mcp_provider_failures.rs`
- `crates/translator-mcp/tests/mcp_session_recovery.rs`

Validated through `make test-mcp` in the project Docker workflow.

## Final Validation Summary

Executed in this session:

- `make test`: passed for the full Rust workspace, including core, CLI, and MCP
  tests.
- `make test-mcp`: passed for the MCP crate contract, privacy, adversarial, and
  lifecycle tests.
- `make fmt`: passed with `cargo fmt --all -- --check`.
- `make clippy`: passed with `cargo clippy --all-targets --all-features -- -D
  warnings`.

During validation, `make test` first exposed an `rmcp` construction issue for
`CallToolResult` because the SDK type is `#[non_exhaustive]`. The result
builders now use the SDK constructors and attach `structuredContent` through the
public field. `make clippy` then exposed a large enum variant in
`StdioServerError`; the initialization error is boxed.

Scope confirmation:

- no real provider was added;
- no network transport or HTTP listener was added;
- no Zed wrapper, `extension.toml`, or registry publication was added;
- no source-file mutation or editor buffer edit path was added;
- `translate_file` remains limited to `.md`, `.markdown`, and `.txt` through the
  existing core file boundary.
