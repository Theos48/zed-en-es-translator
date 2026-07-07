# Manual Validation: Real Provider Configuration

Use this template for three reviewer smoke runs against a local
LibreTranslate-compatible provider that the reviewer starts outside this
project workflow. Do not install provider tooling through this repository, and
do not record real secrets, full source text, translated text, provider bodies,
headers, tokens, workspace roots, or sensitive paths.

## Prerequisites

- Local provider target is loopback, for example `http://127.0.0.1:5000`.
- Test input is synthetic English only.
- Shell environment uses only controlled provider variables:
  - `TRANSLATOR_PROVIDER=libretranslate`
  - `TRANSLATOR_PROVIDER_URL=<loopback-url>`
  - `TRANSLATOR_PROVIDER_API_KEY_ENV=<optional-env-var-name>`
  - `TRANSLATOR_ALLOW_REMOTE_PROVIDER=false`
- `target/release/translator-cli` was built through the project Makefile/Docker
  workflow.

## Evidence Rules

Record only:

- date and git revision;
- provider family and local/non-local classification;
- provider version if known;
- command name, not full payload;
- synthetic canary ID;
- input/output byte lengths or hashes;
- normalized error code when applicable;
- pass/fail for redaction, no-mutation, and remote gates.

Never record:

- real source text or translated text;
- API keys or key values;
- full provider response bodies;
- headers, tokens, full URLs with credentials, workspace roots, or local paths;
- real user documents.

## Run 1: Direct Text Local Provider

| Field | Value |
|-------|-------|
| Date | |
| Git revision | |
| Provider target classification | loopback local |
| Provider version | |
| Command | `translator-cli` direct text JSON on stdin |
| Synthetic canary ID | RPC-501 |
| Input byte length | |
| Output byte length or hash | |
| Non-mock Spanish output observed | pass/fail |
| stderr redaction | pass/fail |
| Notes | |

## Run 2: Allowed Workspace File Local Provider

| Field | Value |
|-------|-------|
| Date | |
| Git revision | |
| Provider target classification | loopback local |
| Provider version | |
| Command | `translator-cli` allowed Markdown file JSON on stdin |
| Synthetic canary ID | RPC-502 |
| Input file byte length or hash | |
| Output byte length or hash | |
| Protected code unchanged in output | pass/fail |
| Source file unchanged byte-for-byte | pass/fail |
| stdout/stderr redaction | pass/fail |
| Notes | |

## Run 3: Local Provider Failure Redaction

| Field | Value |
|-------|-------|
| Date | |
| Git revision | |
| Provider target classification | loopback local |
| Provider state | unavailable/timeout/malformed synthetic response |
| Command | `translator-cli` direct text JSON on stdin |
| Synthetic canary ID | RPC-505 |
| Expected code | `PROVIDER_FAILED` or `PROVIDER_TIMEOUT` |
| Observed code | |
| stdout/stderr redaction | pass/fail |
| No source or provider body recorded | pass/fail |
| Notes | |

## Optional Remote Gate Check

| Field | Value |
|-------|-------|
| Date | |
| Git revision | |
| Provider target classification | non-local synthetic target |
| Command A | unconfirmed non-local `translator-cli` request |
| Expected code A | `REMOTE_CONFIRMATION_REQUIRED` |
| Observed code A | |
| Command B | confirmed non-local request with synthetic secret-like canary |
| Expected code B | `SECRET_DETECTED` |
| Observed code B | |
| Provider contact prevented | pass/fail |
| stdout/stderr redaction | pass/fail |
| Notes | |
