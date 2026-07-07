# Quickstart: Real Provider Configuration

This quickstart is the reviewer protocol for the real-provider feature after
implementation. It does not install a provider or any host-level runtime.

## Scope Guard

Included:

- Mock/offline remains default.
- Local LibreTranslate-compatible provider configuration.
- Direct text and authorized workspace-file translation.
- Additive remote confirmation with default denial.
- Secret blocking before non-local provider contact.
- Provider timeout, failure mapping, response validation, and redaction.
- No automatic source-file or editor-buffer mutation.

Excluded:

- Installing LibreTranslate, Python, Docker, or any provider globally.
- Managed paid provider onboarding.
- Marketplace publication.
- Custom Zed UI or direct command UX.
- Automatic replacement of editor content.

## Repository Preparation

Run from the repository root after implementation tasks exist:

```bash
make test-core
make test-mcp
make test-zed-extension
make test-real-provider-config
make test
make fmt
make clippy
```

Expected:

- Provider config tests pass with local stubs and synthetic content.
- Existing mock/offline tests still pass.
- No real secrets or provider responses appear in logs.

If Docker or other project prerequisites are missing, record the blocker. Do
not install host tooling as part of this feature without explicit approval under
the workstation policy.

## Final Automated Evidence

Recorded on 2026-07-07 with the project Docker workflow:

```bash
make test-core
make test-mcp
make test-zed-extension
make test-real-provider-config
make test
make clippy
make fmt
```

Result:

- `make test-core`: pass.
- `make test-mcp`: pass.
- `make test-zed-extension`: pass.
- `make test-real-provider-config`: pass.
- `make test`: pass.
- `make clippy`: pass.
- `make fmt`: pass.

Automated local-provider evidence uses loopback LibreTranslate-compatible test
stubs and synthetic content only:

| Run | Command | Provider Target | Evidence Scope | Result |
|-----|---------|-----------------|----------------|--------|
| 1 | `make test-real-provider-config` | loopback test stubs | focused provider payload, local direct text, local allowed file, remote denial, secret blocking, failures, Zed settings | pass |
| 2 | `make test-core` | loopback test stubs | core local provider translation, no source mutation, response validation, timeout and redaction | pass |
| 3 | `make test` | loopback test stubs plus mock default | full workspace regression with local-provider and mock/offline paths | pass |

No real API key, provider response body, full source text, translated text,
workspace root, header, token, or sensitive path is recorded in this evidence.
Use [manual-validation.md](./manual-validation.md) for reviewer smoke evidence
against an externally started local provider.

## Local Provider Prerequisite

For manual smoke validation, start a LibreTranslate-compatible service yourself
on loopback, outside this project workflow. Example target expected by this
feature:

```text
http://127.0.0.1:5000
```

Use only synthetic text for smoke validation.

## CLI Local Provider Smoke

Configure provider mode for one shell session:

```bash
export TRANSLATOR_PROVIDER=libretranslate
export TRANSLATOR_PROVIDER_URL=http://127.0.0.1:5000
```

Run a direct-text request through the project CLI after building it through the
project Docker workflow:

```bash
printf '%s\n' '{
  "source_text": "Synthetic provider canary RPC-501 says: Read the documentation before changing the code.",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true,
  "input_kind": "text"
}' | target/release/translator-cli
```

Expected:

- Exit code is `0`.
- stdout contains JSON success with Spanish text.
- Output is not the deterministic mock phrase for the same input.
- stderr contains no source text, translated text, headers, tokens, or paths.

## Workspace File Smoke

Create a small synthetic file:

```text
tmp/provider-validation.md
```

Suggested content:

````markdown
# Notes

Synthetic provider canary RPC-502 says: Open the file before editing.

```rust
fn main() {
    println!("keep code intact");
}
```
````

Run an allowed-file translation request:

```bash
printf '%s\n' '{
  "workspace_root": "/absolute/path/to/repo",
  "file_path": "tmp/provider-validation.md",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true,
  "input_kind": "markdown"
}' | target/release/translator-cli
```

Expected:

- stdout contains translated Markdown.
- Protected code remains unchanged in the translated output.
- `tmp/provider-validation.md` remains byte-for-byte unchanged.
- Evidence records only canary IDs, lengths/hashes, and redacted summaries.

## Remote Denial Smoke

Configure a non-local target without request confirmation:

```bash
export TRANSLATOR_PROVIDER=libretranslate
export TRANSLATOR_PROVIDER_URL=https://example.invalid
export TRANSLATOR_ALLOW_REMOTE_PROVIDER=true
```

Run a direct-text request without `remote_confirmed`:

```bash
printf '%s\n' '{
  "source_text": "Synthetic provider canary RPC-503 says: Read the docs.",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true,
  "input_kind": "text"
}' | target/release/translator-cli
```

Expected:

- Request fails before provider contact.
- Normalized error code is `REMOTE_CONFIRMATION_REQUIRED`.
- No source text appears in stderr or diagnostics.

## Secret Blocking Smoke

Run a confirmed non-local request with synthetic secret-like content:

```bash
printf '%s\n' '{
  "source_text": "Synthetic provider canary RPC-504 token sk-test1234567890 should not leave the machine.",
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve_formatting": true,
  "input_kind": "text",
  "remote_confirmed": true
}' | target/release/translator-cli
```

Expected:

- Request fails before provider contact.
- Normalized error code is `SECRET_DETECTED`.
- The token-like text is absent from stdout, stderr, logs, and evidence.

## Zed Extension Smoke

After `make zed-extension-prepare`, configure the local development extension
with:

```json
{
  "binary_path": "/absolute/path/to/target/release/translator-mcp",
  "provider": {
    "mode": "libretranslate",
    "url": "http://127.0.0.1:5000",
    "api_key_env": "",
    "allow_remote": false
  }
}
```

Expected:

- `translator-en-es` starts with controlled provider configuration.
- `translate_text` and `translate_file` remain available.
- Translation output appears in Zed without automatic buffer mutation.
- Zed diagnostics remain redacted.

## Evidence To Record

Record:

- Git branch/revision.
- Commands run.
- Provider mode and local/non-local classification.
- Local provider version if known.
- Synthetic canary IDs only.
- Pass/fail status for direct text, workspace file, remote denial, secret
  blocking, timeout/failure mapping, redaction, and no-mutation checks.
