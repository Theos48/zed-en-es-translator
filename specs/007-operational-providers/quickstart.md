# Quickstart: Operational Real Providers

This is the implemented operational path and the final acceptance procedure.
The automatic controlled checks and the approved real local CLI/direct-Zed/
offline/failed-update/rollback/cleanup runs pass. T056 and F011 are complete.

## Prerequisites

- Fedora workstation with the repository's already-classified Docker Engine,
  Docker Compose, GNU Make, Git, `curl`, and `jq` tools.
- At least 4 logical CPUs, 4 GiB RAM available to the local provider, and 4 GiB
  free disk for image/model/lifecycle slots.
- No host Python, Rust, LibreTranslate, Argos, database, or system service is
  installed by this feature.
- Local preparation/update needs Internet access; prepared local translation,
  status, stop, and rollback do not.
- No external account, subscription, API key, or remote service is required
  for supported use or F011 acceptance.

Never create or commit a real `.env` file. Never paste a provider key into Zed
settings, command arguments, a terminal transcript, or validation evidence.

## 1. Run deterministic automatic gates

These tests use controlled doubles and require neither provider download nor
Azure contact:

```bash
make workspace-storage-check
make test-operational-providers
make test-real-provider-config
make test-direct-zed-translation
make test-zed-extension
make test
make fmt
make clippy
make deny
```

The storage guard rejects a checkout or worktree on `/tmp`, `/dev/shm`,
`tmpfs`, or `ramfs` before Rust builds can consume host memory as ephemeral
build storage. Keep the main checkout and every registered worktree on the
documented persistent path and use `make worktree-audit` when adding one.

Do not execute real local preparation or write actual manual outcomes until
every automatic gate above passes. Story implementation may prepare the
redacted procedures earlier, but real execution is a final gated activity.

## 2. Prepare the local provider

Review `ops/providers/libretranslate/provider.lock` before an online operation,
then prepare the fixed candidate:

```bash
make provider-local-prepare
make provider-local-status
```

The command reports only normalized lifecycle state. It pulls LibreTranslate
v1.9.6 by digest, downloads the two lock-approved Argos packages into ignored
`provider-cache/`, verifies the project-observed SHA-256 values, installs them
into the candidate volume, proves health plus the public synthetic probe,
restarts on the internal network, proves both again, and copies only a verified
candidate into `current`. A failed candidate leaves the prior `current`
reference unchanged. Prepared readiness must complete within 120 seconds; each
translation remains within 15 seconds.

LibreTranslate itself is attached only to the internal runtime network and has
no published port. A read-only, capability-free Python relay in the same fixed
image accepts bounded health/translate requests on `127.0.0.1:5000` and uses a
fixed internal destination. It records no request content and needs no host
Python installation.

Start/verify are offline and idempotent after preparation:

```bash
make provider-local-start
make provider-local-verify
make provider-local-start
```

Expected endpoint configuration for CLI and direct Zed:

```text
TRANSLATOR_PROVIDER=libretranslate
TRANSLATOR_PROVIDER_URL=http://127.0.0.1:5000
```

Do not set remote enablement or an API-key reference for the supported local
profile.

## 3. Validate local CLI without recording content

Build the CLI through the project container, then launch it with the local
profile and a public synthetic request. Observe the JSON success ephemerally;
record only `success`, not `translated_text`.

```bash
make translator-cli-release
```

Use the request fixture documented by the eventual manual-validation file.
Confirm the provider container has no external egress during the run and that
tracked input files remain byte-for-byte unchanged.

## 4. Validate local direct Zed

```bash
make zed-direct-prepare
```

Set only the following safe values under
`lsp.en-es-translator.binary.env` in local Zed settings:

```json
{
  "TRANSLATOR_PROVIDER": "libretranslate",
  "TRANSLATOR_PROVIDER_URL": "http://127.0.0.1:5000"
}
```

Restart the language server, invoke the code action on the public synthetic
fixture, verify the `[local]` label and read-only hover preview, then verify the
buffer/file hash is unchanged. Do not capture the hover content in evidence.

## 5. Understand the optional remote adapter

The hardened Azure adapter remains in the codebase as advanced opt-in
functionality, but this quickstart does not ask the user to create an account,
resource, or key. Its exact target, safe credential-reference, per-request
confirmation, secret-blocking, timeout, response-validation and redaction
boundaries are proven by `make test-operational-providers` and documented in
`contracts/azure-translator.md`. A live remote run contributes no required
F011 acceptance row.

MCP/Agent Panel is only a compatibility bridge. Its existing regression tests
must remain green, but it is not configured or validated as an F011 product
surface and contributes no row to the real local-service acceptance matrix.

## 6. Exercise update and rollback

An update is never automatic. After reviewing and changing the versioned lock:

```bash
make provider-local-update
```

The new candidate must not replace current until online and offline probes
pass. For acceptance, simulate a failed candidate check, verify current stayed
active, then exercise the documented offline rollback:

```bash
make provider-local-rollback
make provider-local-verify
```

## 7. Stop or explicitly remove local provider data

Normal stop preserves prepared data and is safe to repeat:

```bash
make provider-local-stop
make provider-local-stop
```

Ordinary `make clean` must also preserve provider data. Complete provider
removal is separate and destructive:

```bash
make provider-local-clean CONFIRM=remove-provider-data
```

It may remove only resources belonging to the fixed provider Compose project;
it must never run a global Docker prune or remove unrelated host tools.

## 8. Record evidence

Fill `manual-validation.md` with normalized outcomes and safe identities only.
Verify that it contains both real local success rows, offline proof, rollback,
explicit cleanup, non-mutation and redaction checks. Optional remote control
results may remain as supplemental evidence. If any row would
require source text, translation, a secret, raw response, endpoint, or local
path, record the normalized observation instead.
