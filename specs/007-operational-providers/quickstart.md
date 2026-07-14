# Quickstart: Operational Real Providers

This is the implemented operational path and the final acceptance procedure.
The automatic controlled checks and the approved real local CLI/direct-Zed/
offline/failed-update/rollback runs pass. Task T056 remains open for Azure and
the remaining manual rows.

## Prerequisites

- Fedora workstation with the repository's already-classified Docker Engine,
  Docker Compose, GNU Make, Git, `curl`, and `jq` tools.
- At least 4 logical CPUs, 4 GiB RAM available to the local provider, and 4 GiB
  free disk for image/model/lifecycle slots.
- No host Python, Rust, LibreTranslate, Argos, database, or system service is
  installed by this feature.
- Local preparation/update needs Internet access; prepared local translation,
  status, stop, and rollback do not.
- Remote validation needs an Azure account, a global single-service Translator
  resource explicitly assigned to F0, and its key. Azure account creation may
  require a phone number and payment card. Azure says a free account may need
  conversion to pay-as-you-go after its introductory period to remain active;
  the Translator resource must still remain F0. Review current
  [Translator pricing](https://azure.microsoft.com/en-us/pricing/details/translator/)
  and [Azure account options](https://azure.microsoft.com/en-us/pricing/purchase-options/azure-account)
  before opting in.
- The fixed global endpoint may process at the closest available data center
  and fail over outside that geography. Microsoft documents no persistence for
  Text Translation, but its current privacy page does not explicitly promise
  exclusion from model training. This project therefore makes no residency or
  training-use guarantee and uses public synthetic content for acceptance.

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

Do not execute real local preparation, local acceptance, Azure acceptance, or
write actual manual outcomes until every automatic gate above passes. Story
implementation may prepare the redacted procedures earlier, but real execution
is a final gated activity.

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

## 5. Configure Azure Translator F0 safely

Create the resource from Microsoft's current
[resource guide](https://learn.microsoft.com/en-us/azure/ai-services/translator/how-to/create-translator-resource).
Use the global single-service Translator resource on F0; custom/region-specific
endpoints are unsupported.

Choose a safe environment variable name and supply its value through your
existing local secret-capable session, outside repository files:

```text
AZURE_TRANSLATOR_KEY=<secret value exists only in the launching environment>
```

Provider selection uses references only:

```text
TRANSLATOR_PROVIDER=azure_translator
TRANSLATOR_PROVIDER_API_KEY_ENV=AZURE_TRANSLATOR_KEY
TRANSLATOR_ALLOW_REMOTE_PROVIDER=true
```

`TRANSLATOR_PROVIDER_URL` must be absent. Configuration alone never confirms a
request.

## 6. Validate remote CLI and direct Zed

For each surface, submit only the public synthetic fixture, confirm that the
UI/CLI identifies remote disclosure, grant exactly this one request, observe a
valid Spanish result, and retain no translated text.

Repeat with a second request and verify confirmation is required again. Then
run denial/dismissal/stale/mismatch and synthetic-secret cases and prove the
provider was not contacted. Validate timeout, missing/rejected key, quota, and
safe generic failure handling without recording raw service output.

On the reviewed Zed 1.10.3 host, dismissing `window/showMessageRequest` may be
rendered only as the generic `Error: execute command`; the redacted LSP result
must still be `REMOTE_CONFIRMATION_REQUIRED`. Record the normalized result and
the generic host rendering, never a raw log.

For Zed local settings, add only the three safe provider-selection values
above. The actual `AZURE_TRANSLATOR_KEY` value must already exist in the Zed
parent process environment. The extension passes only the reference name and
must not read or copy the value into `binary.env`, settings, arguments, the
launch profile, diagnostics, or evidence.

MCP/Agent Panel is only a compatibility bridge. Its existing regression tests
must remain green, but it is not configured or validated as an F011 product
surface and contributes no row to the real-service acceptance matrix.

## 7. Exercise update and rollback

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

## 8. Stop or explicitly remove local provider data

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

## 9. Record evidence

Fill `manual-validation.md` with normalized outcomes and safe identities only.
Verify that it contains all four real success rows, offline proof, rollback,
remote pre-contact denials, non-mutation and redaction checks. If any row would
require source text, translation, a secret, raw response, endpoint, or local
path, record the normalized observation instead.
