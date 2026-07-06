# Manual Validation Evidence: Zed UX Flow

Use this template to record the manual Zed validation run. Keep evidence
redacted. Do not paste real source text, translated text from sensitive inputs,
secrets, tokens, headers, workspace roots, sensitive paths, or environment dumps.

## Run Metadata

- Date:
- Reviewer:
- Git branch:
- Git revision:
- Zed version:
- Operating system:
- Extension path: `zed-extension/`
- Prepared artifact category: `target/release/translator-mcp` or redacted local path category
- Agent Profile:
- Agent model route: `local | zed-hosted | provider-key | subscription | gateway | unknown`
- Tool-permission posture:
- `translator-en-es` tools available: `yes | no | blocked`
- Evidence format used: Synthetic canary, Hash/length metadata, Redacted summary
- Host prerequisite blocker:

## Scenario 0: Agent Privacy And Permission Boundary

- Status: `pass | fail | blocked`
- Agent model route:
- Non-local/unknown route uses synthetic canary only: `yes | no | not_applicable`
- Tool-permission posture:
  - `edit_file`:
  - `write_file`:
  - `delete_path`:
  - `move_path`:
  - `copy_path`:
  - `create_directory`:
  - `terminal`:
  - `fetch`:
  - `search`:
- Redacted summary:

## Scenario 1: Direct Text Success

- Status: `pass | fail | blocked`
- Synthetic canary:
- Hash/length metadata:
- Tool observed: `translate_text`
- Result status:
- No manual `translator-mcp` process: `yes | no | blocked`
- No-mutation evidence:
- Redacted summary:

## Scenario 2: Workspace File Success And No Mutation

- Status: `pass | fail | blocked`
- Synthetic canary:
- File category: workspace `.md`, `.markdown`, or `.txt`
- Hash before:
- Hash after:
- Hashes match: `yes | no | blocked`
- Tool observed: `translate_file`
- No-mutation evidence:
- Redacted summary:

## Scenario 3: Selection Support Decision

- Status: `pass | fail | blocked`
- Decision: `validated_supported | unsupported | deferred`
- Synthetic canary:
- Hash/length metadata:
- Context-added-by path:
- Tool observed:
- Only selected content sent: `yes | no | blocked`
- No-mutation evidence:
- Redacted summary:

## Scenario 4: Setup Failure

- Setup failure status: `pass | fail | blocked`
- Trigger used: `missing binary_path | missing artifact | stale artifact | non-executable artifact`
- Corrective action shown:
- No-mutation evidence:
- Redacted summary:

## Scenario 5: Unsafe Or Unsupported Input Denial

- Unsafe input denial status: `pass | fail | blocked`
- Remote/provider denial status: `pass | fail | blocked`
- Trigger used:
- Error category:
- Network/provider path enabled: `yes | no | blocked`
- No-mutation evidence:
- Redacted summary:

## Scenario 6: Redaction Inspection

- Status: `pass | fail | blocked`
- Source text absent from errors/logs: `yes | no | blocked`
- Translated text absent from error diagnostics: `yes | no | blocked`
- Secrets absent: `yes | no | blocked`
- Tokens absent: `yes | no | blocked`
- Headers absent: `yes | no | blocked`
- Environment dumps absent: `yes | no | blocked`
- Workspace roots absent: `yes | no | blocked`
- Sensitive paths absent: `yes | no | blocked`
- Evidence uses canary identifiers, hash/length metadata, and redacted summaries only: `yes | no | blocked`
- Redacted summary:

## Closure

- Direct text success:
- Workspace file success:
- Selection support decision:
- Setup failure:
- Unsafe input denial:
- Remote/provider denial:
- Redaction inspection:
- Overall result: `pass | fail | blocked`
- Remaining blockers:
