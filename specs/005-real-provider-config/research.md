# Research: Real Provider Configuration

## Decision: Use a LibreTranslate-compatible local provider first

The first real provider path will target a local self-hosted
LibreTranslate-compatible service. The product will treat the service as a
reviewer/user prerequisite and will not install it globally.

**Rationale**:

- LibreTranslate documents itself as free/open-source, self-hosted, and powered
  by Argos Translate rather than proprietary translation services.
- Its quickstart documents a local `localhost:5000` translation flow for
  English-to-Spanish text.
- Its `/translate` operation has the exact conceptual inputs this product
  already models: source text, source language, target language, format, and an
  optional API key for deployments that require one.
- This keeps the first real provider useful without making a remote paid
  service or API-key account part of the default path.

**Sources**:

- LibreTranslate documentation: <https://docs.libretranslate.com/>
- LibreTranslate installation guide: <https://docs.libretranslate.com/guides/installation/>
- LibreTranslate `/translate` reference: <https://docs.libretranslate.com/api/operations/translate/>
- LibreTranslate GitHub repository: <https://github.com/LibreTranslate/LibreTranslate>

**Alternatives considered**:

- Managed remote provider first: rejected because it would make network/privacy
  review the first useful path and conflicts with offline-first defaults.
- Provider-specific CLI process first: rejected for this iteration because it
  would add command allowlisting and process execution risk before a simple
  local service adapter.
- Generic arbitrary HTTP provider: rejected because arbitrary endpoints and
  payload shapes are too broad for the first real provider feature.

## Decision: Keep provider configuration outside translation requests

Provider target, mode, and optional secret reference will be process/server
configuration. The existing translation request stays backward-compatible. The
only additive request-level data is remote confirmation, which defaults to
false when absent.

**Rationale**:

- The provider should not receive workspace roots, file paths, logs, or local
  context; keeping configuration separate from source content makes that easier
  to verify.
- Existing CLI and MCP callers should continue to work without adding provider
  fields to every request.
- Per-request remote confirmation must be explicit, but it does not need to
  carry provider URL, headers, API keys, or other configuration.

**Alternatives considered**:

- Add provider URL and API key fields to every translation request: rejected
  because it mixes configuration with content and increases leakage risk.
- Configure provider only through Zed settings: rejected because CLI and MCP
  tests also need the same provider boundary outside Zed.
- Use real `.env` files in the repository: rejected because secrets and local
  provider settings must not be versioned.

## Decision: Implement the provider behind the existing Rust `Provider` boundary

The real provider adapter belongs in `translator-core` and implements the
existing `Provider` trait. `translator-cli` and `translator-mcp` select the
configured provider and continue to call the core translation functions.

**Rationale**:

- The current core already performs segmentation, limits, redaction, file
  safety, and output reconstruction before/after provider calls.
- Keeping the adapter behind `Provider` preserves testability with local stubs
  and avoids duplicating provider logic in MCP or the Zed wrapper.
- Rust best-practice guidance favors `Result` for fallible operations, precise
  error mapping in libraries, descriptive tests, and static dispatch where the
  concrete provider is known.

**Alternatives considered**:

- Implement provider calls only in `translator-mcp`: rejected because the CLI
  would not exercise the same real provider behavior.
- Implement provider calls in `zed-extension/`: rejected because the wrapper
  should only launch/pass controlled configuration and must not duplicate core
  translation logic.
- Replace `Provider` with an untyped callback or shell command: rejected due to
  privacy, process, and testability risk.

## Decision: Use a minimal blocking HTTP client with explicit timeout controls

Use a small blocking Rust HTTP client in the core provider adapter, planned as
`ureq` 3.x with JSON support, configured with the existing provider timeout and
without inherited proxy behavior.

**Rationale**:

- The current `Provider` trait is synchronous, so a blocking client avoids
  forcing async through `translator-core`.
- `ureq` documents blocking I/O, low-overhead usage, JSON support, `Result`
  errors, configurable global timeout through an `Agent`, and optional TLS.
- The plan must explicitly avoid inherited proxy behavior because the Zed
  context-server process can inherit environment variables from Zed.
- The HTTP client's DEBUG/TRACE logging cannot be relied on for redaction, so
  provider code and validation must keep those logs disabled for sensitive
  runs.

**Sources**:

- `ureq` crate docs: <https://docs.rs/ureq/latest/ureq/>

**Alternatives considered**:

- `reqwest` async client: rejected for this iteration because the core provider
  trait is synchronous and an async dependency would add runtime complexity.
- Hand-written HTTP over TCP: rejected because protocol details, TLS, and
  response parsing would be error-prone.
- Shelling out to `curl`: rejected because shell/process execution violates the
  provider sandbox direction and complicates redaction.

## Decision: Validate provider payload and response at the segment boundary

The adapter will send only translatable segments plus `en`, `es`, and
technical-neutral tone-derived formatting metadata. It will validate segment
count, non-empty translated content, UTF-8/text shape, and output limits before
returning success.

**Rationale**:

- The constitution requires providers to receive only permitted segments and
  metadata.
- Existing reconstruction safety depends on getting one translated segment per
  requested segment.
- Provider failure modes need deterministic error mapping so CLI/MCP/Zed users
  receive actionable redacted feedback.

**Alternatives considered**:

- Send full files to the provider: rejected because protected code regions,
  file paths, and workspace context must not leave the core boundary.
- Trust provider output without validation: rejected because malformed or
  oversized provider output can corrupt user-visible translation.

## Decision: Remote provider behavior is default-deny and testable without real remote calls

Non-local targets require explicit configuration and per-request confirmation.
Unconfirmed remote attempts fail before provider contact. Confirmed remote
attempts still run secret detection before contact. Automated tests may verify
these gates with non-routable or stub targets; manual remote smoke is optional
and synthetic-only.

**Rationale**:

- The important safety property is that denial happens before content leaves
  the machine.
- The project does not need a live remote service to prove default-deny,
  confirmation, secret blocking, or redaction.
- This keeps the feature useful for local real translation while preserving a
  future path for explicit remote configuration.

**Alternatives considered**:

- Exclude remote targets entirely: rejected because the constitution and prior
  decisions already require a confirmed remote path to be modeled safely.
- Require a real remote smoke test: rejected because it would make network
  access, account setup, and provider availability part of acceptance.
