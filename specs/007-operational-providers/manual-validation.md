# Manual Validation: Operational Real Providers

**Status**: Complete

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
| Optional remote adapter | `Azure AI Translator Text v3; non-gating` |
| Validation window UTC | `2026-07-14T11:07:10Z / 2026-07-14T12:48:30Z` |
| Reviewer | `Codex terminal validation + user Zed observation` |

## Real success matrix

Execution prerequisite: run the complete T054/T055 automatic and supply-chain
gates, obtain explicit approval, use a clean checkout, and record only the safe
fields below. `LOCAL-CLI-01` and `LOCAL-ZED-01` use
`make provider-local-prepare`, `make provider-local-start`,
`make provider-local-verify`, and the public fixtures under
`tests/fixtures/operational-providers/`. Readiness is measured against 120
seconds and each invocation against 15 seconds. Both required rows below were
executed against the pinned provider. No remote account, API key, or live
remote success row is required.

| Case | Timestamp UTC | Surface | Locality | Actual normalized outcome | Within budget | Source unchanged | Buffer unchanged | Redaction | Result |
|---|---|---|---|---|---|---|---|---|---|
| `LOCAL-CLI-01` | `2026-07-14T11:36:42Z` | CLI | local | `success at clean commit 1d1b204151d2` | `yes (1.139 s)` | `yes` | n/a | `pass` | `pass` |
| `LOCAL-ZED-01` | `2026-07-14T11:47:52Z` | direct Zed | local | `success at clean commit 02590922bd82` | `yes (<15 s, reviewer-observed)` | `yes` | `yes` | `pass` | `pass` |

Reviewer attestation: translated output was observed ephemerally as valid,
non-mock English-to-Spanish output and was not copied into this record:
`pass for LOCAL-CLI-01 and LOCAL-ZED-01`.

## Local operation and recovery

| Case | Timestamp UTC | Expected condition | Actual normalized outcome | External egress | Within budget | Result |
|---|---|---|---|---|---|---|
| `LOCAL-PREPARE-01` | `2026-07-14T11:07:10Z` | pinned artifacts verified | `READY; integrity and promotion passed` | enabled only for prepare | `yes (114 s total; readiness gates enforced at 120 s)` | `pass` |
| `LOCAL-OFFLINE-01` | `2026-07-14T11:13:49Z` | health+translation after no-egress restart | `READY; provider egress blocked` | disabled | `yes (11 s readiness; invocation under 1 s)` | `pass` |
| `LOCAL-IDEMPOTENT-01` | `2026-07-14T11:09:07Z` | repeated start/stop safe | `READY after repeated stop/start/verify` | disabled | `yes (31 s sequence)` | `pass` |
| `LOCAL-UPDATE-FAIL-01` | `2026-07-14T11:55:05Z` | failed candidate leaves current | `IMAGE_IDENTITY_MISMATCH; active/current/previous unchanged` | enabled only for update | `yes (2.530 s)` | `pass` |
| `LOCAL-ROLLBACK-01` | `2026-07-14T11:11:58Z` | prior slot restored and verified | `READY; active_slot=previous` | disabled | `yes (30 s rollback+verify)` | `pass` |
| `LOCAL-CLEAN-01` | `2026-07-14T12:48:30Z` | only project provider resources removed | `CLEANED; project resources absent; unrelated resource counts unchanged` | disabled | `yes (<1 s observed)` | `pass` |

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

## Supplemental optional-remote controls

These observations complement the controlled automatic matrix. They are
retained as security evidence but are not required live-service acceptance.

| Case | Timestamp UTC | Condition | Provider contacted | Actual normalized outcome | Result |
|---|---|---|---|---|---|
| `REMOTE-DENY-01` | `2026-07-14T11:13:11Z` | denied | `no` | `REMOTE_CONFIRMATION_REQUIRED` | `pass` |
| `REMOTE-DISMISS-01` | `2026-07-14T12:08:49Z` | dismissed | `no (loopback-only process namespace)` | `REMOTE_CONFIRMATION_REQUIRED; host UI showed generic error` | `pass` |
| `REMOTE-STALE-01` | `2026-07-14T12:14:18Z` | document changed during consent | `no (loopback-only process namespace)` | `INVALID_INPUT; stale target rejected; host UI showed generic error` | `pass` |
| `REMOTE-REUSE-01` | `2026-07-14T12:26:00Z` | prior consent reused | `no (second request; loopback-only process namespace)` | `REMOTE_CONFIRMATION_REQUIRED; fresh second prompt; host UI showed generic error` | `pass` |
| `REMOTE-SECRET-01` | `2026-07-14T11:13:29Z` | synthetic secret after confirmation | `no` | `SECRET_DETECTED` | `pass` |
| `REMOTE-MISSING-KEY-01` | `2026-07-14T11:13:29Z` | missing referenced key | `no` | `PROVIDER_NOT_CONFIGURED` | `pass` |
| `REMOTE-AUTH-QUOTA-01` | `2026-07-14T12:28:35Z` | rejected credential | `yes (single confirmed request)` | `PROVIDER_FAILED; 879 ms; redaction passed` | `pass` |

## Scope and execution notes

- `LOCAL-CLI-01` was rerun from clean commit `1d1b204151d2`, and
  `LOCAL-ZED-01` passed from clean commit `02590922bd82`.
- No real Azure F0 credential was present, needed, or used. Remote success and
  live-service failure cases are intentionally outside F011 acceptance.
- A reviewed negative simulation used a temporary mismatched expected image
  identity; the versioned lock was restored byte-for-byte after the failure.
- Destructive cleanup used only the exact documented confirmation token. The
  provider ended `UNPREPARED`; unrelated container, volume and network counts
  were unchanged.

## Final gates

- [X] All automatic suites passed before real validation.
- [X] Prepared local readiness completed within 120 seconds and every executed provider invocation completed within 15 seconds.
- [X] Both required real local success rows passed.
- [X] Local provider worked with external egress disabled after preparation.
- [X] Failed update preserved current and offline rollback passed.
- [X] Optional remote fresh-confirmation behavior passed controlled automatic tests and the recorded supplemental observations.
- [X] Optional remote denial/dismissal/stale/reuse/secret cases stopped before contact in controlled validation.
- [X] Files and Zed buffers remained byte-for-byte unchanged.
- [X] Logs, diagnostics, stderr, evidence and screenshots passed prohibited-data review.
- [X] No key, real `.env`, provider blob, source, or translation was committed.
- [X] Reviewer result: `pass`.
