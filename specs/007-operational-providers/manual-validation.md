# Manual Validation: Operational Real Providers

**Status**: Partial real execution — local CLI/direct Zed/lifecycle pass; T056 incomplete

**Rule**: Replace only bracketed safe fields after implementation. Never add
source text, translated text, segments, response bodies, credentials, headers,
environment dumps, full endpoints, workspace roots, local paths, or screenshots
containing content.

## Reviewed identities

| Field | Safe value |
|---|---|
| Local provider | `LibreTranslate 1.9.6` |
| Local image | `sha256:1de2d7056bb8...` |
| Local model versions | `en-es 1.0; es-en 1.9; user-provisioned, not redistributed` |
| Remote service | `Azure AI Translator Text v3, global single-service, F0` |
| Validation window UTC | `2026-07-14T11:07:10Z / 2026-07-14T12:14:18Z` |
| Reviewer | `Codex terminal validation + user Zed observation` |

## Real success matrix

Execution prerequisite: run the complete T054/T055 automatic and supply-chain
gates, obtain explicit approval, use a clean checkout, and record only the safe
fields below. `LOCAL-CLI-01` and `LOCAL-ZED-01` use
`make provider-local-prepare`, `make provider-local-start`,
`make provider-local-verify`, and the public fixtures under
`tests/fixtures/operational-providers/`. Readiness is measured against 120
seconds and each invocation against 15 seconds. The local rows marked `pass`
below were executed against the pinned provider; bracketed rows remain
unexecuted.

`REMOTE-CLI-01` and `REMOTE-ZED-01` additionally require a reviewed global
single-service Azure Translator resource still assigned to F0. The actual key
must exist only in the parent process environment; Zed settings and CLI launch
configuration contain the safe reference name. For each row, begin a new
request, verify the remote-confirmation label/prompt, approve only that request,
measure the 15-second invocation budget, observe the result ephemerally, and
verify source/buffer hashes. Repeat denial, dismissal, mismatch, reuse, missing
key, and confirmed synthetic-secret cases while recording only normalized
contact/no-contact outcomes.

| Case | Timestamp UTC | Surface | Locality | Actual normalized outcome | Within budget | Source unchanged | Buffer unchanged | Redaction | Result |
|---|---|---|---|---|---|---|---|---|---|
| `LOCAL-CLI-01` | `2026-07-14T11:36:42Z` | CLI | local | `success at clean commit 1d1b204151d2` | `yes (1.139 s)` | `yes` | n/a | `pass` | `pass` |
| `LOCAL-ZED-01` | `2026-07-14T11:47:52Z` | direct Zed | local | `success at clean commit 02590922bd82` | `yes (<15 s, reviewer-observed)` | `yes` | `yes` | `pass` | `pass` |
| `REMOTE-CLI-01` | `[UTC]` | CLI | remote | `[success/error code]` | `[yes/no]` | `[yes/no]` | n/a | `[pass/fail]` | `[pass/fail]` |
| `REMOTE-ZED-01` | `[UTC]` | direct Zed | remote | `[success/error code]` | `[yes/no]` | `[yes/no]` | `[yes/no]` | `[pass/fail]` | `[pass/fail]` |

Reviewer attestation: translated output was observed ephemerally as valid,
non-mock English-to-Spanish output and was not copied into this record:
`pass for LOCAL-CLI-01 and LOCAL-ZED-01; remote success rows pending`.

## Local operation and recovery

| Case | Timestamp UTC | Expected condition | Actual normalized outcome | External egress | Within budget | Result |
|---|---|---|---|---|---|---|
| `LOCAL-PREPARE-01` | `2026-07-14T11:07:10Z` | pinned artifacts verified | `READY; integrity and promotion passed` | enabled only for prepare | `yes (114 s total; readiness gates enforced at 120 s)` | `pass` |
| `LOCAL-OFFLINE-01` | `2026-07-14T11:13:49Z` | health+translation after no-egress restart | `READY; provider egress blocked` | disabled | `yes (11 s readiness; invocation under 1 s)` | `pass` |
| `LOCAL-IDEMPOTENT-01` | `2026-07-14T11:09:07Z` | repeated start/stop safe | `READY after repeated stop/start/verify` | disabled | `yes (31 s sequence)` | `pass` |
| `LOCAL-UPDATE-FAIL-01` | `2026-07-14T11:55:05Z` | failed candidate leaves current | `IMAGE_IDENTITY_MISMATCH; active/current/previous unchanged` | enabled only for update | `yes (2.530 s)` | `pass` |
| `LOCAL-ROLLBACK-01` | `2026-07-14T11:11:58Z` | prior slot restored and verified | `READY; active_slot=previous` | disabled | `yes (30 s rollback+verify)` | `pass` |
| `LOCAL-CLEAN-01` | `[UTC]` | only project provider resources removed | `[safe status]` | disabled | `[yes/no]` | `[pass/fail]` |

Execute these cases only after the T054/T055 gates and explicit approval:

1. Record the safe output of `make provider-local-status`; repeat
   `make provider-local-start`, `make provider-local-verify`, and
   `make provider-local-stop` to prove idempotency without deleting data.
2. Review the proposed `provider.lock` change, then run
   `make provider-local-update`. Simulate or observe a candidate failure and
   confirm that the recorded active slot and both last-known-good identities
   are unchanged. Never record subprocess output or content.
3. After a successful promotion, disable external egress and run
   `make provider-local-rollback`, followed by
   `make provider-local-verify`. Record only normalized lifecycle state,
   active slot, shortened image identity, lock digest, offline-verification
   status, elapsed budget, and pass/fail.
4. Confirm ordinary `make clean` and `make provider-local-stop` preserve all
   provider slots. Run destructive cleanup only as
   `make provider-local-clean CONFIRM=remove-provider-data`; verify that it
   removes only the fixed Compose project resources, three provider slots,
   provider/relay containers, two allowlisted networks, and ignored provider
   metadata.

Do not run a global Docker prune, use `sudo`, change packages or services, or
copy source, translation, response, credential, path, or environment data into
this evidence file.

## Remote pre-contact and failure controls

| Case | Timestamp UTC | Condition | Provider contacted | Actual normalized outcome | Result |
|---|---|---|---|---|---|
| `REMOTE-DENY-01` | `2026-07-14T11:13:11Z` | denied | `no` | `REMOTE_CONFIRMATION_REQUIRED` | `pass` |
| `REMOTE-DISMISS-01` | `2026-07-14T12:08:49Z` | dismissed | `no (loopback-only process namespace)` | `REMOTE_CONFIRMATION_REQUIRED; host UI showed generic error` | `pass` |
| `REMOTE-STALE-01` | `2026-07-14T12:14:18Z` | document changed during consent | `no (loopback-only process namespace)` | `INVALID_INPUT; stale target rejected; host UI showed generic error` | `pass` |
| `REMOTE-REUSE-01` | `[UTC]` | prior consent reused | `[no/unknown]` | `[error code]` | `[pass/fail]` |
| `REMOTE-SECRET-01` | `2026-07-14T11:13:29Z` | synthetic secret after confirmation | `no` | `SECRET_DETECTED` | `pass` |
| `REMOTE-MISSING-KEY-01` | `2026-07-14T11:13:29Z` | missing referenced key | `no` | `PROVIDER_NOT_CONFIGURED` | `pass` |
| `REMOTE-AUTH-QUOTA-01` | `[UTC]` | rejected credential or quota | `[yes/unknown]` | `[error code]` | `[pass/fail]` |
| `REMOTE-TIMEOUT-01` | `[UTC]` | timeout | `[yes/unknown]` | `[error code]` | `[pass/fail]` |
| `REMOTE-RESPONSE-01` | `[UTC]` | invalid/oversized response | `[yes/unknown]` | `[error code]` | `[pass/fail]` |

## Known incomplete prerequisites

- `LOCAL-CLI-01` was rerun from clean commit `1d1b204151d2`, and
  `LOCAL-ZED-01` passed from clean commit `02590922bd82`; the two remote success
  rows still require the same clean-checkout discipline.
- The local direct Zed row passed; `REMOTE-ZED-01` still requires a reviewer to
  observe the preview and buffer state in the editor.
- No real Azure F0 credential was present, so remote success and real contact
  cases were not attempted.
- A reviewed negative simulation used a temporary mismatched expected image
  identity; the versioned lock was restored byte-for-byte after the failure.
- Destructive cleanup remains deferred so the prepared offline provider is
  available for the pending cleanup case.

## Final gates

- [X] All automatic suites passed before real validation.
- [X] Prepared local readiness completed within 120 seconds and every executed provider invocation completed within 15 seconds.
- [ ] Four real success rows passed.
- [X] Local provider worked with external egress disabled after preparation.
- [X] Failed update preserved current and offline rollback passed.
- [ ] Every remote invocation required fresh confirmation.
- [ ] Denial/dismissal/stale/mismatch/reuse/secret cases stopped before contact.
- [ ] Files and Zed buffers remained byte-for-byte unchanged.
- [ ] Logs, diagnostics, stderr, evidence and screenshots passed prohibited-data review.
- [X] No key, real `.env`, provider blob, source, or translation was committed.
- [ ] Reviewer result: `incomplete`.
