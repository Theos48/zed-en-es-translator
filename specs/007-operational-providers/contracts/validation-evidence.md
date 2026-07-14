# Contract: Real-Provider Validation and Evidence

## Automatic gate

`make test-operational-providers` must run controlled tests only. It must not
require Docker provider downloads, an Azure account, real credentials, Zed
interaction, or Internet access. Tests are written first and cover:

- exact configuration matrix and mock default;
- local loopback target and fixed Azure target allowlists;
- redirects/proxy inheritance disabled;
- payload minimization, fixed language mapping, internal technical-neutral
  tone/format validation, absence of invented Azure metadata, and response
  cardinality;
- local readiness and lifecycle state-machine failures using controlled
  process/HTTP boundaries;
- remote denial/dismissal/stale/mismatch/prior-consent reuse before contact;
- secret blocking after confirmation and before contact;
- timeout, TLS/DNS/HTTP/auth/quota/malformed/empty/oversized outcomes;
- shared CLI/LSP configuration and safe locality labels;
- unchanged files/buffers and redacted stderr/diagnostics/evidence fixtures;
- preservation of all existing limits, Markdown behavior and error codes.

Existing MCP suites remain part of regression compatibility only. Automatic
F011 checks add no MCP-specific Azure product flow, and MCP/Agent Panel is not
part of the real-service acceptance matrix.

Existing suites remain mandatory: core, provider config, direct Zed,
extension, formatting, Clippy, tests, and dependency audit.

## Manual acceptance matrix

Real validation is performed only after automatic gates pass and uses public,
synthetic test cases. Required success rows:

| Case | Provider | Surface | Required observation |
|---|---|---|---|
| `LOCAL-CLI-01` | prepared LibreTranslate | CLI | non-mock valid Spanish; offline runtime |
| `LOCAL-ZED-01` | prepared LibreTranslate | direct Zed | local label, read-only preview, unchanged buffer |
| `REMOTE-CLI-01` | Azure F0 | CLI | explicit one-request confirmation, valid Spanish |
| `REMOTE-ZED-01` | Azure F0 | direct Zed | remote label, Zed confirmation, read-only preview |

The reviewer observes translated output ephemerally. Translation content is
never copied into the evidence file.

Required negative families:

- mock remains default;
- local external egress disabled after preparation;
- local unavailable, readiness failure, port conflict, timeout, corrupt or
  incomplete candidate, failed update and successful offline rollback;
- remote denied, dismissed, stale/mismatched and reused consent;
- confirmed remote content with a synthetic secret blocked before contact;
- missing credential, rejected credential/quota, timeout, certificate failure,
  redirect, API failure, malformed/cardinality/oversized response;
- all source files and open buffers unchanged;
- all observable logs, diagnostics, stderr and evidence pass redaction review.

## Evidence row schema

Each row records only:

```text
case_id
timestamp_utc
surface
safe_locality
safe_provider_identity
safe_artifact_or_service_tier
expected_normalized_outcome
actual_normalized_outcome
provider_contacted (when measured by a controlled negative case)
source_unchanged
buffer_unchanged (Zed cases)
redaction_passed
reviewer_result
safe_notes
within_budget
```

Prohibited: source, translation, segment, raw provider body, key/token, header,
environment value/dump, full/sensitive URL, absolute workspace/local path,
screenshot containing content, or raw diagnostic/log dump.

## Completion rule

F011 cannot close from stubs alone. It requires all automatic tests, the four
real success rows, required negative outcomes, offline proof, rollback proof,
non-mutation review, and redaction review. Any unchecked row keeps the feature
open and F009/publication blocked.
