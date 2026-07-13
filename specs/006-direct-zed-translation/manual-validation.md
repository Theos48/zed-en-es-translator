# Manual Validation: Direct Zed Translation

This file records only redacted, synthetic evidence. Do not include source or
translated text, provider URLs, workspace paths, environment values, headers,
tokens, or secrets.

## Environment

- Detected Zed version: `1.10.3` (`0c54c414d522234de7298039708ffe85a116892a`)
- Extension API: `0.7.0`
- Prepared artifact: automated preparation and interactive use **PASS**
- Automated validation date/reviewer: `2026-07-13` / Codex
- Interactive validation date/reviewer: `2026-07-13` / user with Codex evidence verification

## Scenario 1: Selection Preview

- Status: **PASS**
- Action title/locality observed: `Translate English to Spanish [offline]`
- Code action and hover result: pass
- Agent Panel remained unused: pass
- Source hash before/after: `0c9e0475dba73ae1c1c97e37ae847c1243fa7c85f09bf5a21f53e5bce90ad6ab` / `0c9e0475dba73ae1c1c97e37ae847c1243fa7c85f09bf5a21f53e5bce90ad6ab`
- Redacted log review: pass; no source or translation payload appeared in the
  observed success evidence

## Scenario 2: Open Markdown Preview

- Status: **PASS**
- Saved allowed document accepted: pass
- Protected Markdown preserved: pass
- Whole-document hover result: pass
- Source hash before/after: `0c9e0475dba73ae1c1c97e37ae847c1243fa7c85f09bf5a21f53e5bce90ad6ab` / `0c9e0475dba73ae1c1c97e37ae847c1243fa7c85f09bf5a21f53e5bce90ad6ab`
- Redacted log review: pass; no source or translation payload appeared in the
  observed success evidence

## Scenario 3: Privacy Or Safety Denial

- Status: **PASS**
- Synthetic denial exercised: `SECRET_DETECTED`
- Remote confirmation behavior: dismissal returned
  `REMOTE_CONFIRMATION_REQUIRED`; confirming safe synthetic prose reached the
  deliberately invalid provider and returned `PROVIDER_FAILED`; confirming the
  synthetic canary returned `SECRET_DETECTED`
- Provider contact count/evidence: zero for the denial case; secret detection
  completed before the provider boundary, consistent with the automated
  transport-counter regression
- Source hash before/after: `e21b7a530c14bd30d26fbc47c52ad9eb226455a1c72e13d2ab07fd4d71721cb0` / `e21b7a530c14bd30d26fbc47c52ad9eb226455a1c72e13d2ab07fd4d71721cb0`
- Redacted log review: pass; observed failure evidence contained only
  normalized status/error metadata and no source, translation, provider URL,
  environment value, header, token, or sensitive path

## Current Result

The exact automated chain recorded in `quickstart.md` passed, including direct
protocol/core/shell tests, wrapper and WASM tests, provider regressions, the
complete workspace suite, formatting, and Clippy with warnings denied. The
selection and open-document flows passed in real Zed with matching
before/after source hashes, and the Agent Panel remained unused. The privacy
denial returned `SECRET_DETECTED` before provider contact, and its observed
failure evidence was redacted. SC-008 is complete. The first scenario 3 attempt
exposed a provider-launch configuration gap; T047/T048 implement and verify its
remediation. The temporary remote provider configuration was removed after the
run so offline/mock is again the default.
