# Data Model: Zed Wrapper

## Zed Development Extension

Represents the local extension package loaded by Zed during development.

Fields:

- `directory`: Project-relative extension directory. Value: `zed-extension/`.
- `manifest`: Path to the extension manifest. Value:
  `zed-extension/extension.toml`.
- `crate_manifest`: Path to the Rust/WASM crate manifest. Value:
  `zed-extension/Cargo.toml`.
- `state`: One of `unprepared`, `prepared`, `registered`, `active`, or
  `failed`.

Validation rules:

- Directory must contain `extension.toml`.
- Extension must register exactly one context server for this feature.
- Extension code must not implement buffer mutation, selection replacement,
  provider configuration, downloads, or network behavior.
- Extension code must not log source text, translated text, secrets, tokens, or
  unredacted sensitive paths.

Relationships:

- Owns one `ExtensionMetadata`.
- Owns one `ContextServerDefinition`.
- Produces one `ServerLaunchProfile` when Zed requests the context server
  command.

## ExtensionMetadata

Represents user-visible identity and required manifest metadata.

Fields:

- `id`: Stable extension id for local development. Planned value:
  `en-es-translator`.
- `name`: Human-readable extension name.
- `version`: Development extension version.
- `schema_version`: Zed extension manifest schema version.
- `authors`: Project author list.
- `description`: Short user-visible purpose.
- `repository`: Repository URL or placeholder appropriate for local development.

Validation rules:

- Metadata must be sufficient for Zed to load the local extension.
- Metadata must not include secrets, absolute local paths, or user-specific
  machine state.
- Publication-only metadata may remain minimal until F009.

Relationships:

- Serialized in `extension.toml`.

## ContextServerDefinition

Represents the context server entry declared to Zed.

Fields:

- `id`: Stable server id. Planned value: `translator-en-es`.
- `purpose`: Starts the local English-to-Spanish translator MCP server.
- `transport`: `stdio`.
- `tools`: Existing MCP server tools: `translate_text` and `translate_file`.

Validation rules:

- Must map to exactly one `context_server_command` branch.
- Must not declare HTTP, remote, provider-selection, prompt, resource, or
  publication behavior.
- Must preserve existing MCP tool contracts from `specs/002-mcp-server/`.

Relationships:

- Declared in `extension.toml`.
- Resolved by the extension into a `ServerLaunchProfile`.

## LaunchSettings

Represents local user/development settings needed to find the prepared server
artifact.

Fields:

- `binary_path`: Local path to the prepared `translator-mcp` artifact.
- `extra_args`: Not supported in this feature.
- `extra_env`: Not supported in this feature.

Validation rules:

- `binary_path` must be present before launch.
- `binary_path` must identify the `translator-mcp` artifact produced by the
  documented preparation workflow.
- Validation and diagnostics must not echo the full path back to logs or errors.
- Provider names, API keys, URLs, remote confirmation fields, and arbitrary
  environment variables are not accepted settings in this feature.

Relationships:

- Used by `ServerLaunchProfile`.
- Described by extension configuration instructions and quickstart.

## PreparedServerArtifact

Represents the local executable server that Zed will launch.

Fields:

- `path`: Local path selected by `LaunchSettings.binary_path`.
- `name`: Expected executable name: `translator-mcp`.
- `build_mode`: `release` for manual Zed smoke validation.
- `status`: One of `missing`, `not_executable`, `stale`,
  `incompatible_checkout`, `usable`, or `failed_on_start`.

Validation rules:

- Must be produced or selected by documented project commands.
- Must be executable by Zed as a direct command.
- Must communicate over stdio and keep stdout reserved for MCP messages.
- Must not require network, provider credentials, or manual pre-start in a
  separate shell.
- Stale or checkout-incompatible artifacts must be treated as unusable and must
  produce the same redacted corrective-action class as other unusable artifacts.

Relationships:

- Is launched by `ServerLaunchProfile`.
- Provides the existing `translator-mcp` MCP tools.

## ServerLaunchProfile

Represents the exact process launch returned from the Zed wrapper.

Fields:

- `command`: Prepared `translator-mcp` executable path.
- `args`: Controlled argument list. Planned default: empty.
- `env`: Allowlisted key/value pairs. Planned default: empty.
- `working_location`: The process context provided by Zed for the command.
- `diagnostic_policy`: Redacted diagnostics only.

Validation rules:

- Must not invoke a shell.
- Must preserve a configured artifact path containing spaces as one command
  value, not as shell-split tokens.
- Must not include source text, file paths to translate, translated text, or
  provider input in argv or environment.
- Must not inherit the full Zed or shell environment.
- Must fail with a stable redacted diagnostic when the artifact is missing,
  unusable, or not configured.
- Repeated startup attempts after a recoverable failure must revalidate
  settings and artifact state without accumulating duplicate generated state.

Relationships:

- Returned by `context_server_command`.
- Launches one `PreparedServerArtifact`.

## RedactedDiagnosticEvent

Represents a safe troubleshooting message produced by the wrapper or server
startup path.

Fields:

- `phase`: `configuration`, `artifact_validation`, `launch`, or
  `server_runtime`.
- `code`: Stable diagnostic category.
- `message`: Actionable redacted message.
- `duration_ms`: Optional timing value.

Validation rules:

- May include status, error category, and bounded timing.
- Must not include source text, translated text, translatable segments, secrets,
  headers, tokens, environment dumps, full local paths, or workspace roots.
- Missing artifact messages must tell the user the corrective command without
  echoing sensitive path values.

Relationships:

- Created from failed `LaunchSettings`, `PreparedServerArtifact`, or server
  startup checks.

## DevelopmentPreparationRun

Represents the project workflow that prepares local artifacts before Zed loads
the extension.

Fields:

- `commands`: Documented Make targets for this feature.
- `outputs`: Prepared server artifact and validation notes.
- `state`: `not_started`, `succeeded`, or `failed`.

Validation rules:

- Must be repeatable without duplicating generated extension state.
- Must run through project-scoped tooling.
- Must not install host runtimes, services, databases, providers, or global
  package managers.

Relationships:

- Produces `PreparedServerArtifact`.
- Supports quickstart validation.
