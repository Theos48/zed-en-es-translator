# Contract: Embedded Provider Selection

**Feature**: `008-embedded-local-provider`
**Status**: Planning contract

## Configuration matrix

| `TRANSLATOR_PROVIDER` | URL | API-key ref | Remote allow | Result |
|---|---|---|---|---|
| absent / `mock` | absent | absent | absent | Deterministic Mock |
| `embedded_local` | absent | absent | absent | Resolve the fixed embedded profile and verified `current` set |
| `embedded_local` | any | any | any | Reject conflicting configuration |
| existing local/remote modes | existing exact rules | existing exact rules | existing exact rules | Unchanged |
| unknown | any | any | any | Reject unsupported provider |

`embedded_local` adds no path, model, executable, registry, URL, argument,
proxy or arbitrary environment setting. CLI and direct Zed use the same parser.
The Zed extension continues to forward only the four controlled D075 keys.

## Resolution order

1. Parse the four controlled provider keys.
2. Reject unknown, incomplete or simultaneous mode-specific configuration.
3. For `embedded_local`, derive the product-owned profile ID and fixed XDG
   storage root internally.
4. Validate root ownership, permissions, persistence and containment without
   following unsafe links.
5. Read one supported state schema and resolve `current` by manifest digest.
6. Verify profile, runner and model identity/compatibility before constructing
   the provider; no registry/network operation is allowed.
7. Construct a provider containing only validated immutable references.

An explicitly selected embedded mode with absent, corrupt or incompatible
state fails closed. It never executes Mock or a remote provider as fabricated
success.

## Locality and user-visible label

| Mode | Locality | Direct-Zed action label |
|---|---|---|
| Mock | Offline | Existing `[offline]` label |
| Embedded local | Offline/local | `[offline]`; no model/path/version disclosure |
| LibreTranslate | Existing local classification | Unchanged |
| Azure | Remote | Existing confirmed-remote label |

Embedded selection never requests remote confirmation because the inference
path cannot contact a remote service. Artifact consent is a separate,
preparation-only operation and is never inferred from a translation action.

## Gate order for a translation request

1. Validate request schema, language direction and supported file/input.
2. Apply existing size, segment count, Markdown protection, ambiguity and
   secret gates.
3. Resolve the already verified local installation; do not prepare implicitly.
4. Acquire a shared installation lease within the total provider deadline.
5. Invoke the exact allowlisted runner using the runner-wire contract.
6. Validate UTF-8, schema, cardinality, order, non-empty entries and total
   semantic output size.
7. Return the existing normalized result or error without persistence.

## Stable outcomes

| Condition | Normalized behavior |
|---|---|
| Embedded mode is absent | Existing Mock default |
| Embedded mode explicitly selected but no current set exists | Provider not configured / preparation required |
| Root, state or artifact containment fails | Provider not configured; no child process |
| Hash, manifest, platform, license or compatibility fails | Provider not configured/readiness failure; no child process |
| Lease cannot be acquired within budget | Stable busy/timeout failure |
| Child exceeds deadline | Kill, reap and return provider timeout |
| Child launch, exit or protocol fails | Provider failed with redacted actionable class |
| Output violates existing semantic limits | Existing invalid-response/limit behavior |

Exact public error-code mapping is fixed during tests before production code;
no new error may include paths, artifact URLs, raw child output or content.

## Required tests

- exhaustive four-key selection matrix for core, CLI, LSP and Zed launch;
- Mock remains default and an explicit broken embedded mode never fakes Mock;
- no user-controlled path/URL/args/env reaches process construction;
- XDG root ownership/permission/link/containment failures stop before spawn;
- locality remains offline and never asks for remote confirmation;
- existing limits, secret blocking, Markdown protection and non-mutation remain
  identical across CLI and direct Zed;
- MCP and all existing provider modes retain compatibility coverage.
