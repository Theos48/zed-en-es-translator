# Research: Zed Wrapper

## Decision: Implement a local Zed MCP extension wrapper

Use a new `zed-extension/` directory containing `extension.toml`, a Rust
`cdylib` crate, and minimal extension code based on `zed_extension_api = "0.7.0"`.
The extension registers one context server for the existing `translator-mcp`
stdio binary.

Sources:

- Zed MCP extension docs: <https://zed.dev/docs/extensions/mcp-extensions>
- Zed development extension docs: <https://zed.dev/docs/extensions/developing-extensions>
- `zed_extension_api` 0.7.0 docs: <https://docs.rs/zed_extension_api/0.7.0/zed_extension_api/>

Rationale:

- Zed documents MCP servers as Agent Panel integrations exposed by extensions.
- Zed requires each MCP server extension to register the server in
  `extension.toml` and implement `context_server_command`.
- The current `zed_extension_api` docs show `context_server_command` returning a
  `Command` with `command`, `args`, and `env` fields, which matches the feature
  requirement for controlled startup.
- Keeping the wrapper in `zed-extension/` preserves the existing repo structure
  and avoids moving core/MCP crates into a Zed-specific layout.

Alternatives considered:

- Put `extension.toml` at repository root: rejected because the root also owns
  Spec Kit files, workspace crates, Docker files, and strategic docs.
- Publish through the MCP registry first: rejected because F006 is local
  development only and publication remains F009.
- Add a Zed slash command or direct editor action: rejected because F007 owns UX
  flow and this feature must not modify buffers or selections.

## Decision: Launch the existing `translator-mcp` binary by explicit local path

The extension will return a `zed::Command` that launches the prepared
`translator-mcp` artifact over stdio. The binary path is a local development
setting documented by the extension configuration and quickstart. Arguments are
controlled by the wrapper and default to none. Environment variables are an
explicit allowlist and default to empty or non-sensitive runtime flags only.

Rationale:

- The existing MCP server already implements tool discovery, `translate_text`,
  `translate_file`, error mapping, redaction, recovery, and stdio behavior.
- Returning a direct command avoids shell execution, avoids argv text payloads,
  and avoids inheriting the full Zed or user shell environment.
- Using a local path setting avoids relying on undocumented assumptions about
  extension-relative process resolution.
- A missing or unusable binary can fail with a stable redacted error that tells
  the user to run the documented preparation command.

Alternatives considered:

- Download the server binary from GitHub Releases inside
  `context_server_command`: rejected because this feature is offline-only and
  must not add network behavior.
- Use npm or Node tooling: rejected because the server is already Rust and the
  project host policy avoids unnecessary runtimes.
- Spawn `cargo run`: rejected because it shells through build tooling at launch
  time, depends on host Rust, and makes startup slower and less reproducible.
- Use a wrapper shell script: rejected because no shell execution is needed and
  a direct binary path is easier to reason about.

## Decision: Keep project validation in Docker and treat Zed dev prerequisites separately

Automated repository validation will use the existing Docker-based Rust
workflow. The project Docker image may add the WASM target needed to check the
extension crate. Manual validation inside Zed is documented separately and must
not install host Rust or modify system configuration from this feature.

Rationale:

- The project constitution and host policy require a minimal host footprint.
- Zed's current development docs say custom Rust extensions require Rust via
  rustup for local dev extension installation. That is a Zed toolchain
  prerequisite, not something this feature should install.
- Docker checks can validate manifest shape, launch-profile helpers, redaction,
  and WASM buildability without changing the host.
- Manual smoke testing in Zed remains necessary to prove integration, but it
  should be skipped with a documented blocker if the host lacks approved Zed dev
  prerequisites.

Alternatives considered:

- Install rustup globally as part of the feature: rejected by host policy.
- Depend only on manual Zed validation: rejected because it would make core
  regression testing host-dependent.
- Skip Zed manual validation entirely: rejected because the feature's value is
  Zed startup.

## Decision: Expose settings only for local launch path, not provider behavior

The extension configuration may expose a local `binary_path` setting for the
prepared server artifact and installation instructions. It must not expose
provider selection, remote confirmation, network URLs, API keys, or translation
behavior settings.

Rationale:

- The feature is a startup wrapper, not provider configuration.
- Provider configuration and privacy confirmation belong to F004/F008, with
  server/core enforcement.
- Keeping the settings surface small reduces accidental secret storage and keeps
  diagnostics redaction practical.

Alternatives considered:

- Expose provider or remote settings now: rejected because real providers and
  remote calls are explicitly out of scope.
- Read provider settings from environment variables: rejected because the
  wrapper must not inherit user shell secrets.

## Decision: Treat Zed MCP extension deprecation as a publication risk, not a blocker

Zed's MCP extension docs currently note a planned deprecation path in favor of
the official MCP registry. F006 remains useful because it targets local
development startup from Zed. Publication strategy stays deferred to F009.

Rationale:

- The spec explicitly targets local development use, not marketplace
  publication.
- Local dev extension validation still proves whether Zed can start and use the
  server.
- F009 can compare Zed extension publication, MCP registry publication, or both
  with newer evidence.

Alternatives considered:

- Cancel F006 and publish to MCP registry immediately: rejected because the
  project still needs local Zed startup validation.
- Build only a registry package: rejected because it would not exercise the
  extension startup contract requested for this feature.
