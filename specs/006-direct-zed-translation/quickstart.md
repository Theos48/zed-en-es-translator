# Quickstart: Direct Zed Translation

This guide is the validation contract for F010. Commands use the project
Makefile and Docker-pinned Rust toolchain; no host Rust installation is needed.

## Automated Validation

```bash
make test-direct-zed-translation
make test-zed-extension
make test
make fmt
make clippy
```

Expected outcomes:

- core selection and open-document snapshot tests pass;
- in-memory LSP lifecycle, code-action, command, hover, stale-state, privacy,
  Unicode-range, redaction, and non-mutation tests pass;
- wrapper manifest/settings/launch tests pass;
- all earlier CLI, MCP, provider, and Zed compatibility tests remain green;
- formatting and Clippy pass with warnings denied.

## Prepare The Direct Artifact

```bash
make zed-direct-prepare
```

Expected output is one absolute path ending in
`target/release/translator-lsp`. Do not place secrets in this command or its
output.

## Configure The Development Extension

Install `zed-extension/` as a local dev extension using Zed's `zed: install dev
extension` action. Add the prepared path under the `en-es-translator` LSP
binary configuration using the shape in
[contracts/zed-lsp-launch.md](./contracts/zed-lsp-launch.md). Keep `binary.env`
absent for the first mock/offline validation.

Reinstall or reload the rebuilt development extension and fully restart the
language server after changing launch settings.

## Manual Scenario 1: Selection Preview

1. Open an allowed Markdown file inside this workspace.
2. Select `Read the documentation before changing the code.`
3. Press `Ctrl-.` and choose `Translate English to Spanish [offline]`.
4. Observe the safe preview-ready notification.
5. Hover inside the original selection.

Expected:

- hover contains the deterministic Spanish result;
- Agent Panel never opens and no Agent profile/prompt/model is configured;
- source selection, buffer, and file remain byte-for-byte unchanged;
- no source or translation appears in Zed/LSP logs.

## Manual Scenario 2: Open Markdown Preview

1. Open the saved synthetic fixture
   `fixtures/manual-open-document.md`, which contains prose, a link, inline
   code, and fenced code.
2. Place one cursor without selecting text.
3. Invoke the same code action and hover anywhere in the document after the
   preview-ready message.

Expected:

- prose is translated;
- Markdown structure, links, and protected code are unchanged in the preview;
- the source document remains unchanged;
- unsupported, sensitive, outside-workspace, binary, non-UTF-8, and over-limit
  files are denied before provider contact in their automated cases.

## Manual Scenario 3: Privacy Denial

Temporarily add this controlled map under the direct LSP `binary` object:

```json
"env": {
  "TRANSLATOR_PROVIDER": "libretranslate",
  "TRANSLATOR_PROVIDER_URL": "https://example.invalid",
  "TRANSLATOR_ALLOW_REMOTE_PROVIDER": "true"
}
```

The reserved non-local target is used only to expose the remote consent flow;
the two checks below must stop before transport. Do not use proprietary text or
real secrets.

1. Select safe prose in `fixtures/manual-open-document.md`, invoke the action
   labelled `[remote - confirmation required]`, and dismiss the confirmation.
2. Open `fixtures/manual-privacy-denial.txt`, select its synthetic canary,
   invoke the same action, and choose `Send this request`.
3. Remove the three `binary.env` entries and restart the language server after
   recording the results so mock/offline becomes the default again.

Expected:

- dismissal returns `REMOTE_CONFIRMATION_REQUIRED` with no provider contact;
- confirmed secret content returns `SECRET_DETECTED` before provider contact;
- messages and logs contain only redacted status/error metadata.

On the reviewed Zed 1.10.3 host, the dismissed execute-command error may be
rendered only as `Error: execute command`; the redacted LSP result still carries
`REMOTE_CONFIRMATION_REQUIRED`. Record the normalized result and generic host
rendering rather than raw log output.

## Manual Evidence

Record Zed version, extension API version, exact scenario result, source hash
before/after, and redacted observations in `manual-validation.md`. Never record
source text, translated text, provider URL, workspace root, environment values,
headers, tokens, or secrets.

## Current Validation Status

Automated validation completed on 2026-07-13 with these exact targets:

```bash
make format
make test-zed-extension
make test-direct-zed-translation && \
make test-real-provider-config && make test && make fmt && \
make clippy
```

Result: **PASS**. The direct core/LSP/shell slice, extension wrapper and WASM
build, real-provider configuration regressions, complete workspace regression
suite, formatting checks, and Clippy with warnings denied all completed
successfully through the Docker-pinned toolchain.

All three real-Zed scenarios passed: selection and open-document previews left
their source unchanged without Agent Panel use, remote dismissal was denied,
and a confirmed synthetic secret returned `SECRET_DETECTED` before provider
contact. The observed failure evidence was redacted, SC-008 is complete, and
the temporary remote `binary.env` configuration was removed so offline/mock is
again the default.
