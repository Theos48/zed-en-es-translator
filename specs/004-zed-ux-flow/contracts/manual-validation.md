# Contract: Manual Zed Validation

Manual validation is required because this feature claims an in-editor user
experience. Repository tests can prove contracts and safety boundaries, but the
reviewer must also observe the actual Zed Agent Panel flow.

## Required Evidence Fields

Record:

- Date.
- Reviewer.
- Git branch and revision.
- Zed version.
- Operating system.
- Extension path installed in Zed.
- Prepared `translator-mcp` artifact path category, redacted if needed.
- Agent Profile or equivalent tool-availability setup.
- Agent model route: `local`, `zed-hosted`, `provider-key`, `subscription`,
  `gateway`, or `unknown`.
- Tool-permission posture for built-in edit/write/delete/move/copy, terminal,
  fetch, and search tools.
- Whether `translator-en-es` exposes `translate_text` and `translate_file`.
- Whether any host prerequisite blocked validation.
- Evidence format used: synthetic canary text, hash/length metadata, and
  redacted summaries.

## Scenario 0: Agent Privacy And Permission Boundary

Goal: Prove the reviewer knows the Agent model route and has constrained
mutation-capable tools before validating translation UX.

Steps:

1. Record the Agent model route for the thread.
2. If the route is not `local` or cannot be identified, use only synthetic
   canary validation text.
3. Record whether built-in edit/write/delete/move/copy, terminal, fetch, and
   search tools are denied or require confirmation.

Pass criteria:

- The model route is recorded.
- Non-local and unknown routes use synthetic canary text only.
- Mutation-capable and external-access tools are denied or require
  confirmation.
- Global auto-approval of all Agent tools is not used.

## Scenario 1: Direct Text Success

Goal: Prove a user can translate direct text inside Zed without manually
starting the server during the request.

Steps:

1. Prepare and register the local development extension.
2. Open an Agent Panel thread where `translator-en-es` tools are available.
3. Ask for a direct English-to-Spanish translation using the approved prompt in
   [quickstart.md](../quickstart.md).
4. Confirm the visible result is readable Spanish text.
5. Confirm no source buffer or file changed.

Pass criteria:

- `translate_text` is used.
- Output is readable Spanish.
- No raw MCP or JSON-RPC protocol is the primary result.
- No manual `translator-mcp` process was started during the request.
- No mutation occurred.

## Scenario 2: Workspace File Success And No Mutation

Goal: Prove an allowed workspace file can be translated while the source file is
preserved.

Steps:

1. Create or choose a small `.md`, `.markdown`, or `.txt` fixture inside the
   authorized workspace.
2. Record its pre-translation content or hash.
3. Request translation through `translate_file`.
4. Record the visible result.
5. Compare the file content or hash after translation.

Pass criteria:

- `translate_file` is used.
- The path is inside the authorized workspace.
- Output is readable Spanish.
- The file remains byte-for-byte unchanged.

## Scenario 3: Selection Support Decision

Goal: Record whether selected editor text can be safely supported in this Zed
flow.

Steps:

1. Select a short English phrase in a Zed buffer.
2. Add the selection to the Agent Panel thread using the Zed UI path being
   evaluated.
3. Request translation through the local translator.
4. Inspect what was sent to the translation tool, using only synthetic canary
   text, length/hash metadata, and redacted summaries in persistent notes.
5. Record the decision as `validated_supported`, `unsupported`, or `deferred`.

Pass criteria for `validated_supported`:

- The tool input contains only the intended selected text.
- No unrelated file or workspace context is sent.
- The visible result is readable Spanish.
- No buffer, file, or selection is modified.
- Persistent evidence contains no real workspace text.

If the reviewer cannot prove these conditions, the feature must not claim
selection support.

## Scenario 4: Setup Failure

Goal: Prove setup failures are actionable and redacted.

Trigger one setup failure, such as missing `binary_path`, missing artifact, or a
stale artifact.

Pass criteria:

- The visible failure points to a corrective action, such as running
  `make zed-extension-prepare` and configuring `binary_path`.
- The failure does not expose full sensitive paths, environment dumps, source
  text, translated text, secrets, tokens, or headers.
- The editor remains unchanged.

## Scenario 5: Unsafe Or Unsupported Input Denial

Goal: Prove unsafe requests remain denied inside the Zed flow.

Trigger at least one denied request, such as:

- parent traversal in `file_path`;
- unsupported file extension;
- binary or non-UTF-8 content;
- sensitive hidden filename;
- provider, remote confirmation, API key, header, or base URL field.

Pass criteria:

- The request is denied.
- The visible error is actionable and redacted.
- No network/provider path is enabled.
- No source file or buffer is modified.

## Scenario 6: Redaction Inspection

Goal: Prove visible failures and diagnostics do not leak protected content.

Inspect the Agent Panel result and any repository-visible logs produced during
manual validation.

Pass criteria:

- No source text appears in errors or diagnostics.
- No translated text appears in error diagnostics.
- No secrets, tokens, headers, environment dumps, workspace roots, or sensitive
  unredacted paths appear.
- Success output contains translated text only as the expected user result.
- Persistent validation notes use synthetic canary text or redacted summaries
  rather than real workspace content.

## Closure Requirement

The feature cannot be closed until all scenarios above are either passed or
explicitly marked blocked with a reason. A blocked host prerequisite must be
recorded; it must not be resolved by installing host tooling outside the
workstation policy.
