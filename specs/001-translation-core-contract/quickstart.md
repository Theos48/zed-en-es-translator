# Quickstart: Translation Core Contract

## Purpose

Validate the first technical feature without Zed, MCP, network access, paid
APIs, or secrets.

## Prerequisites

- Docker available on the host for the project-local Rust workflow.
- Rust is provided by the pinned Docker image in the root `Makefile`; do not
  install `rustc` or `cargo` globally for this project by default.
- Rust installation/provisioning must follow the host system policy before any
  alternative workflow is used.
- No remote provider configuration is required or expected.

Build the project Rust toolchain image:

```bash
make install
make rust-version
```

## Expected Source Layout

```text
crates/translator-core/
crates/translator-cli/
crates/translator-cli/tests/
tests/fixtures/
Cargo.toml
Cargo.lock
```

## Validation Commands

Run all Rust tests:

```bash
make test
```

Run the CLI contract manually with stdin JSON:

```bash
printf '%s\n' '{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}' \
  | docker run --rm --user "$(id -u):$(id -g)" \
      -e HOME=/workspace/.cache/home \
      -e CARGO_HOME=/workspace/.cache/cargo \
      -e CARGO_TARGET_DIR=/workspace/target \
      -v "$PWD:/workspace" \
      -w /workspace \
      rust:1.96.1-bookworm \
      cargo run -p translator-cli
```

## Required Validation Scenarios

Direct text:

- valid text returns `translated_text`;
- empty text returns `INVALID_INPUT`;
- input over 20 KiB returns `FILE_TOO_LARGE`.
- prompt-injection text is treated as content and does not alter provider,
  logging, command execution, or output contract;
- ambiguous content is preserved instead of guessed.

Markdown/text file:

- `.md`, `.markdown`, and `.txt` inside workspace are accepted;
- file requests provide `workspace_root` and `file_path`, not caller-supplied
  `source_text`;
- fenced code and inline code are preserved;
- tricky Markdown preserves nested or alternating fences, unclosed fences,
  multi-backtick inline code, links, images, blockquotes, tables, HTML blocks,
  and frontmatter;
- unsupported extensions return `UNSUPPORTED_FILE_TYPE`;
- non-UTF-8 input returns `NON_UTF8_INPUT`;
- binary content, NUL bytes, and mixed text/binary payloads return
  `NON_UTF8_INPUT` or `INVALID_INPUT`.

Workspace safety:

- traversal with `..` returns `PATH_NOT_ALLOWED`;
- absolute path outside workspace returns `PATH_NOT_ALLOWED`;
- root-prefix confusion such as `/tmp/ws` versus `/tmp/ws-evil` returns
  `PATH_NOT_ALLOWED`;
- symlink escaping workspace returns `PATH_NOT_ALLOWED`;
- directory symlink escaping workspace returns `PATH_NOT_ALLOWED`;
- chained symlink escaping workspace returns `PATH_NOT_ALLOWED`;
- TOCTOU-style validation/read swaps do not process content outside workspace;
- hidden sensitive file returns `PATH_NOT_ALLOWED`.
- hidden sensitive file with supported-looking extension returns
  `PATH_NOT_ALLOWED`.

Privacy:

- remote provider path without confirmation returns `REMOTE_CONFIRMATION_REQUIRED`;
- unconfigured or not-allowlisted remote provider attempts return
  `PROVIDER_NOT_CONFIGURED`;
- configured remote provider attempts without per-request confirmation return
  `REMOTE_CONFIRMATION_REQUIRED`;
- confirmed remote provider attempts still return `PROVIDER_NOT_CONFIGURED` in
  this feature because no remote provider is allowlisted or implemented;
- obvious secret before remote processing returns `SECRET_DETECTED`;
- API keys, bearer tokens, private key headers, and `.env` assignments are
  covered by secret-detection fixtures;
- logs and stderr do not contain source text, translated text, segments, tokens,
  headers, or sensitive paths.

Provider:

- deterministic provider gives repeatable fixture output;
- provider timeout maps to `PROVIDER_TIMEOUT`;
- provider oversized output maps to a normalized failure;
- provider diagnostics containing source text or secrets are redacted;
- malformed provider output maps to `PROVIDER_FAILED` or `INTERNAL_ERROR` with
  redacted message.

CLI:

- malformed JSON returns a normalized failure and non-zero exit;
- unknown fields, wrong types, unsupported languages, and
  `preserve_formatting=false` are rejected;
- requests mixing `source_text` with `file_path`/`workspace_root` are rejected;
- source text, secrets, or sensitive paths in argv are rejected/redacted;
- success stdout is clean and contains no logs, metadata, paths, or provider
  diagnostics;
- failure stderr is redacted and exit code is non-zero.

## Validation Log

2026-07-02 foundational contracts and limits:

- Expected TDD failure after T009-T013: `make test-core` reached Rust
  compilation and failed because `contract`, `errors`, and `limits` APIs were not
  implemented yet.
- Passing result after T014-T017: `make test-core` passed 14 tests across
  contract serialization, schema parity, error codes, limits, and segment limits.

2026-07-02 direct text US1:

- Expected TDD failure after T019-T024: `make test-core` reached Rust
  compilation and failed because `translate_text`, `Provider`, `ProviderRequest`,
  `ProviderResponse`, and `MockProvider` APIs were not implemented yet.
- Passing result after T025-T028: `make test-core` passed 21 tests across
  foundational contracts and direct-text translation, including empty input,
  oversized input, deterministic mock provider behavior, prompt-injection-as-
  content handling, and ambiguous code-like text preservation.

2026-07-02 Markdown/text file US2:

- Expected TDD failure after T030-T044: `make test-core` reached Rust
  compilation and failed because `translate_file` was not implemented yet.
- Passing result after T045-T049: `make test-core` passed 38 tests, including
  Markdown preservation, tricky Markdown regions, provider exclusion for code
  regions, traversal/root-prefix rejection, direct/directory/chained symlink
  escape rejection, TOCTOU-style symlink swap rejection, sensitive filename
  denial, UTF-8/binary/NUL validation, and no source-file mutation.

2026-07-02 failure privacy and CLI US3:

- Tests and implementation for remote default-deny, secret detection, provider
  failure redaction, provider output limits, and CLI stdin/stdout/stderr behavior
  were added.
- Passing result after T066-T070: `make test` passed the full workspace inside
  the pinned Rust container, including `translator-core` and `translator-cli`
  integration tests for stdin/stdout JSON, malformed input, argv privacy,
  redacted stderr, remote default-deny, secret detection, provider failures,
  provider timeouts, provider output limits, and redaction.

2026-07-02 final implementation validation:

- Command: `make test`
- Result: PASS.
- Additional checks: `make fmt` PASS; `make clippy` PASS.
- Scope: no real provider was added, no network call was added, no editor buffer
  edit was added, and full-file source-code support remains out of scope.
- Decision drift: no accepted limit or error code changed during
  implementation, so `docs/decisions.md` did not need a new decision entry.

2026-07-02 convergence validation:

- T078: `ProviderRequest` now exposes only segments, language pair, and tone to
  providers; `input_kind` remains internal to file loading and reconstruction.
- T079: blank `.txt` files and protected-only Markdown files now return
  `NO_TRANSLATABLE_SEGMENTS`; ambiguous direct text remains a successful
  preserved output.
- T080: raw non-ASCII JSON strings are preserved through the contract and CLI;
  invalid UTF-8 stdin maps to `NON_UTF8_INPUT`.
- Command: `make test`
- Result: PASS.

## Security And Adversarial Coverage Summary

Automated tests cover good paths, bad paths, and malicious/adversarial paths:

- direct text success, empty input, whitespace input, oversized input,
  prompt-injection-as-content, and ambiguous code-like text preservation;
- Markdown/text success with protected fenced code, inline code, frontmatter,
  HTML blocks, tricky fences, links, images, blockquotes, tables, and no source
  file mutation;
- workspace traversal, absolute path escape, root-prefix confusion, direct
  symlink escape, directory symlink escape, chained symlink escape, TOCTOU-style
  symlink swap, hidden sensitive filenames, unsupported extensions, non-UTF-8
  bytes, NUL bytes, and mixed text/binary payloads;
- remote provider default deny, unconfirmed remote state, not-allowlisted remote
  state, obvious secret detection before remote processing, provider failures,
  provider timeout, oversized provider output, malicious provider diagnostics,
  malformed CLI JSON, unknown fields, wrong types, mixed request variants, argv
  misuse, clean success stdout, redacted failure stdout/stderr, and non-zero
  failure exit codes.

## Success Criteria

The feature is valid when every required scenario above is covered by automated
tests or explicit checks and `make test` passes locally without network access
or secrets after the Rust image and Cargo dependencies are already available.
