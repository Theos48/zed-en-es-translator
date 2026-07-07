# Data Model: Zed UX Flow

This feature does not add a database or persistent application state. The model
below defines the reviewable concepts used by the UX contract and manual
validation protocol.

## ZedTranslationSession

A single attempt to request translation from inside Zed and inspect the result
without leaving the editor.

Fields:

- `session_id`: Reviewer-chosen identifier for validation notes.
- `zed_version`: Zed version used for manual validation.
- `extension_state`: `prepared`, `registered`, `missing`, `disabled`, `stale`,
  or `misconfigured`.
- `context_server_id`: Expected value: `translator-en-es`.
- `agent_model_route`: `local`, `zed-hosted`, `provider-key`,
  `subscription`, `gateway`, or `unknown`.
- `tool_permission_posture`: Reference to the translation-only permission
  setup used during validation.
- `input_path`: Reference to the supported input path used.
- `result`: Reference to the visible translation result.
- `mutation_evidence`: Reference to no-mutation evidence.

Validation rules:

- A successful session must use the local development extension and must not
  require a manually started server process during the translation attempt.
- A session must not include provider, API key, base URL, headers, remote
  confirmation, or arbitrary environment configuration.
- If `agent_model_route` is not `local`, the session must use synthetic canary
  validation text only.

## SupportedInputPath

A validated route for intentionally providing content to the local translator.

Fields:

- `kind`: `direct_text`, `workspace_file`, or `selection_context`.
- `tool_name`: `translate_text` or `translate_file` for supported paths.
- `source_language`: `en`.
- `target_language`: `es`.
- `tone`: `technical_neutral`.
- `preserve_formatting`: `true`.
- `limit_source`: Reference to the active input limit or file safety boundary.
- `status`: `supported`, `unvalidated`, `unsupported`, or `deferred`.

Validation rules:

- `direct_text` is supported only through the active `translate_text` schema.
- `workspace_file` is supported only through the active `translate_file` schema.
- `selection_context` cannot be marked `supported` until manual validation
  identifies exactly what selected content is sent and proves no mutation.
- Unsupported or unvalidated paths must not be described as working UX.

## SelectionSupportDecision

The recorded outcome for selected editor text in the current Zed UX.

Fields:

- `state`: `unvalidated`, `validated_supported`, `unsupported`, or `deferred`.
- `zed_version`: Zed version used for the decision.
- `agent_profile`: Profile used during validation, if known.
- `tool_availability`: Whether `translator-en-es` tools were available in the
  thread.
- `context_added_by`: `keybinding`, `agent_action`, `plus_menu`, `paste`, or
  `not_attempted`.
- `observed_tool_input`: Redacted summary of what was sent to the translation
  tool.
- `synthetic_canary`: Unique synthetic phrase used to identify the intended
  selected text without persisting real workspace content.
- `evidence_hash`: Optional hash or length metadata for the observed input.
- `decision_reason`: Brief evidence-based rationale.

Validation rules:

- `validated_supported` requires manual evidence that only the selected text
  intended for translation was passed to the translator.
- Evidence must be recorded as synthetic canary text, length/hash metadata, and
  redacted summaries only.
- `unsupported` requires manual evidence that the current path cannot safely or
  reliably pass the selection.
- `deferred` is valid when the reviewer cannot validate the behavior without
  extra host setup, a different Zed version, or a later product decision.

## VisibleTranslationResult

The user-facing result shown in Zed.

Fields:

- `status`: `success` or `failure`.
- `surface`: Expected value: `Agent Panel`.
- `primary_text`: Spanish translated text for success, redacted actionable
  message for failure.
- `error_code`: Normalized error code for failures, when available.
- `raw_protocol_visible`: Must be `false` for accepted UX.
- `diagnostic_noise_visible`: Must be `false` for accepted UX.
- `redaction_status`: `passed`, `failed`, or `not_applicable`.

Validation rules:

- Success output may contain the translated text as the primary result.
- Failure output must not expose source text, translated text, translatable
  segments, secrets, tokens, headers, environment dumps, workspace roots, or
  sensitive unredacted paths.
- Raw MCP/JSON-RPC payloads must not be the normal user-facing result.

## NoMutationEvidence

Evidence that the translation workflow did not change user content.

Fields:

- `buffer_state_before`: Reviewer note or checksum for relevant content.
- `buffer_state_after`: Reviewer note or checksum for relevant content.
- `file_hash_before`: Optional hash for saved workspace files.
- `file_hash_after`: Optional hash for saved workspace files.
- `unsaved_changes_preserved`: `true`, `false`, or `not_applicable`.
- `review_changes_shown`: Whether Zed surfaced agent edits.

Validation rules:

- For file translation, byte-for-byte file content must remain unchanged.
- For unsaved buffers, visible editor content must remain unchanged.
- If Zed shows reviewable edits, the session fails the no-mutation contract
  unless the edits are unrelated and clearly not produced by this flow.

## ManualValidationRun

The complete evidence bundle for a reviewer run.

Fields:

- `date`: Validation date.
- `reviewer`: Person or role recording the run.
- `zed_version`: Zed version.
- `repo_revision`: Git revision or branch.
- `commands_run`: Repository commands run before manual validation.
- `agent_model_route`: Recorded model request route for the Agent Panel thread.
- `tool_permission_posture`: Translation-only profile or equivalent permission
  setup used during validation.
- `scenarios`: List of validated scenarios from
  [manual-validation.md](./contracts/manual-validation.md).
- `selection_decision`: Reference to the selection support decision.
- `open_issues`: Any blocking or deferred evidence.

Validation rules:

- A run cannot close the feature unless it includes at least one success case,
  one unsafe or unsupported denial, one setup failure, one no-mutation check,
  one redaction inspection, and one selection-support decision.
- If host prerequisites block manual Zed validation, the run must record the
  blocker instead of installing tools outside policy.

## AgentPrivacyBoundary

The model route used by Zed Agent while orchestrating MCP tool calls.

Fields:

- `route`: `local`, `zed-hosted`, `provider-key`, `subscription`, `gateway`, or
  `unknown`.
- `sensitive_content_allowed`: `true` only for `local`.
- `validation_content_policy`: `synthetic_only` or `sensitive_allowed`.
- `source`: Reviewer note explaining how the route was identified.

Validation rules:

- Non-local and unknown routes are allowed only with synthetic canary text.
- This feature does not configure provider credentials or bless remote model
  use for sensitive content.

## TranslationOnlyAgentProfile

The Agent Panel profile or permission posture used to keep validation focused
on the translator tools.

Fields:

- `translator_tools_available`: Whether `translator-en-es` exposes
  `translate_text` and `translate_file`.
- `edit_tools_posture`: `denied`, `confirm`, or `unknown`.
- `filesystem_mutation_tools_posture`: `denied`, `confirm`, or `unknown`.
- `terminal_posture`: `denied`, `confirm`, or `unknown`.
- `network_tools_posture`: `denied`, `confirm`, or `unknown`.

Validation rules:

- No-mutation evidence is accepted only when mutation-capable tools are denied
  or require confirmation.
- A profile with automatic edit/write/delete/move/copy, terminal, fetch, or
  search actions enabled cannot close this feature.
