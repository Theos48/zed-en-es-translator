# Research: Translation Core Contract

## Decision 1: First feature remains offline and deterministic

**Decision**: Use only an offline deterministic provider in this feature.

**Rationale**: The constitution requires privacy default deny and TDD without
network, paid APIs, or secrets. A deterministic provider lets tests verify
segmentation, reconstruction, error mapping, and CLI behavior before selecting
any real translation engine.

**Alternatives considered**:

- Local real model: rejected for this feature because model/runtime selection is
  not decided and may require host policy approval.
- Free remote endpoint: rejected for this feature because cost-free does not
  imply privacy-safe and would require consent and provider review.

## Decision 2: Rust core plus Rust CLI for first implementation slice

**Decision**: Implement `translator-core` as the behavior owner and
`translator-cli` as the process boundary used by future integration layers.

**Rationale**: Existing project decisions choose Rust for the core and wrapper,
with a CLI bridge as the simplest MVP boundary. The CLI contract can be tested
without Zed or MCP.

**Alternatives considered**:

- TypeScript-only core: rejected because project decisions assign preservation
  and safety behavior to Rust.
- Long-running local service: rejected because it adds lifecycle, port, and
  sandbox complexity before the core contract is stable.
- WASM boundary: rejected for the first feature because it is more complex than
  a one-request CLI process.

## Decision 3: JSON UTF-8 stdin/stdout wire contract

**Decision**: The CLI receives one JSON UTF-8 `TranslateRequest` on stdin and
returns JSON UTF-8 success or failure on stdout.

**Rationale**: Passing text through stdin avoids shell quoting and argv leaks.
One request per process simplifies timeouts, failure isolation, and fixtures.

**Alternatives considered**:

- Command-line arguments for text: rejected because source text may leak in
  process listings or shell history.
- JSON lines multi-request process: rejected because state and lifecycle are not
  needed for this feature.

## Decision 4: Markdown/text only for file translation in this feature

**Decision**: `translate_file` accepts `.md`, `.markdown`, and `.txt` only.

**Rationale**: Source-code full-file support requires language-aware
segmentation to avoid translating code. This feature focuses on safe file reads
and Markdown/text preservation first.

**Alternatives considered**:

- Include Rust/TypeScript/Python/shell comments now: rejected until a reliable
  segmenter/parser and negative fixtures exist.
- Accept arbitrary text-like extensions: rejected because configuration and
  data files can be destructively translated or leak secrets.

## Decision 5: Workspace-only file reads

**Decision**: File reads require an authorized workspace root and a requested
path that canonicalizes inside that root.

**Rationale**: MCP tools can be invoked with model-provided arguments. Without a
workspace boundary, `translate_file` risks reading unrelated user files.

**Alternatives considered**:

- Allow absolute paths: rejected unless they canonicalize inside the authorized
  workspace.
- Allow symlinks blindly: rejected because symlinks can escape the workspace.

## Decision 6: Redacted diagnostics only

**Decision**: Logs and stderr may include request ids, error codes, sizes,
durations, and redacted status, but never source text, translated text,
segments, secrets, headers, tokens, or sensitive paths.

**Rationale**: Failure paths are a common source of accidental data leaks, and
Zed/MCP logs may be easy to inspect or share.

**Alternatives considered**:

- Verbose debug logs with source text: rejected for privacy.
- Full raw provider diagnostics: rejected because future providers may include
  request contents, headers, or tokens in errors.

## Decision 7: Rust toolchain prerequisite is deferred to implementation setup

**Decision**: This plan records Rust as required, but does not install it.

**Rationale**: The current environment has Node/npm but no `rustc`/`cargo`.
Installing a Rust toolchain is a project dependency decision and must follow
the host system policy before implementation.

**Alternatives considered**:

- Install Rust globally now: rejected because the user requires explicit policy
  handling for system/toolchain changes.
- Change stack away from Rust: rejected because the accepted architecture uses
  Rust for the core.
