# Feature Specification: Zed UX Flow

**Feature Branch**: `004-zed-ux-flow`

**Created**: 2026-07-06

**Status**: Draft

**Input**: User description: "Promote F007 as the fourth formal Spec Kit feature after the Zed wrapper merge. Define a polished in-editor reading workflow for Zed where a user can complete an English-to-Spanish translation without leaving the editor. The feature must build on the already merged local Zed development extension and existing translation tools. It must document a low-friction Agent Panel based flow, produce readable translation results, preserve the original buffer and files without automatic edits, keep direct text and authorized workspace-file inputs bounded by the active contracts, and manually validate the real Zed selection behavior before claiming selection support. Keep real providers, remote network translation, marketplace publication, API-key setup, arbitrary provider configuration, and automatic replacement/editing out of scope. Preserve the constitution: offline/mock by default, no source mutation, no unsafe file access, no secret leakage, and no logs or diagnostics containing source text, translated text, tokens, headers, environment dumps, or sensitive paths."

## Clarifications

### Session 2026-07-06

- Q: What privacy stance applies to Zed Agent model routing during this UX validation? -> A: Local-or-synthetic validation.
- Q: What Agent tool-permission posture is required for no-mutation claims? -> A: Translation-only profile.
- Q: What evidence format is allowed for selection and tool-input validation? -> A: Synthetic canary summaries.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Complete A Translation Inside Zed (Priority: P1)

As a Zed user, I can request an English-to-Spanish translation from inside the
editor and receive a clear Spanish result in the in-editor conversational flow,
so I can understand text without switching to a terminal, starting services by
hand, or copying command-line output back into my workspace.

**Why this priority**: This is the first product-level payoff from the previous
wrapper work. The system already knows how to start the local translator from
Zed; this feature must make that capability usable as a coherent reading
workflow.

**Independent Test**: With the local extension prepared and registered, a
reviewer follows the documented Zed flow, submits supported English content,
receives a Spanish result in Zed, and confirms that no terminal command is
needed during the translation attempt.

**Acceptance Scenarios**:

1. **Given** the local extension has been prepared and registered, **When** the
   user follows the documented in-editor translation flow, **Then** Zed presents
   a translation result without requiring the user to manually start a separate
   server process.
2. **Given** the user intentionally provides direct English text through the
   supported Zed flow, **When** translation succeeds, **Then** the visible answer
   is readable Spanish text rather than raw protocol output, setup details, or
   diagnostic noise.
3. **Given** the user has an open editor buffer with saved or unsaved content,
   **When** the translation flow completes, **Then** the original buffer and
   source file remain unchanged unless the user manually copies text.

---

### User Story 2 - Know What Content Is Sent (Priority: P2)

As a privacy-conscious user, I can tell which content will be sent to the local
translator before I submit a request, so I do not accidentally translate the
wrong context, assume unsupported selection behavior, or expose more workspace
content than intended.

**Why this priority**: The translation flow handles potentially sensitive
workspace text. A low-friction experience is not safe unless the input boundary
is explicit and understandable.

**Independent Test**: A reviewer checks each supported input path in the
documented Zed flow and verifies that the submitted content matches the active
translation contracts. Selection-based behavior is either validated with real
Zed evidence or explicitly marked unsupported for this iteration.

**Acceptance Scenarios**:

1. **Given** the user wants to translate direct text, **When** they follow the
   supported flow, **Then** only the text they intentionally provided is sent to
   the local translator.
2. **Given** the user wants to translate a workspace file, **When** they use the
   supported file input path, **Then** the request remains limited to authorized
   workspace files that satisfy the active safety rules.
3. **Given** the user highlights text in Zed, **When** selection forwarding has
   not been validated for the current Zed flow, **Then** the feature does not
   claim selection support and directs the user to a validated input path.

---

### User Story 3 - Recover Safely From UX Failures (Priority: P3)

As a user or maintainer, when the in-editor translation flow fails, I can tell
whether the problem is setup, unsupported input, unsafe content, or translation
failure without exposing source text, translated text, secrets, tokens, headers,
environment values, or sensitive paths in the visible output.

**Why this priority**: The user experience must remain trustworthy when
something goes wrong. Failure messages are part of the product surface and must
preserve the same privacy guarantees as the translator itself.

**Independent Test**: A reviewer triggers representative failures from Zed:
unprepared local extension, unsupported input, unsafe file request, remote or
provider configuration attempt, and protected-only content. Each failure is
actionable, redacted, and non-mutating.

**Acceptance Scenarios**:

1. **Given** the local extension is registered but not fully prepared, **When**
   the user starts the translation flow, **Then** the visible failure points to
   a setup corrective action rather than raw startup internals.
2. **Given** the user requests content that violates the active file safety
   rules, **When** the request is evaluated, **Then** the result is a safe denial
   that does not reveal sensitive path or file content.
3. **Given** translation fails after a request is submitted, **When** the user
   inspects the visible result and diagnostics, **Then** they can identify the
   failure category without seeing protected content or secrets.

### Edge Cases

- The local Zed extension is missing, disabled, stale, or configured with values
  left over from a previous development attempt.
- The user opens the Agent Panel before the local translator has finished
  starting.
- The Agent Panel profile uses a non-local or externally routed model provider
  while the user attempts to validate sensitive workspace content.
- The Agent Panel profile has built-in editing, terminal, path mutation, URL
  fetch, or web search tools enabled while the feature is claiming no-mutation
  behavior.
- The user assumes selected editor text is automatically sent, but the current
  Zed flow does not expose selection content reliably enough to claim support.
- The user submits Markdown with code fences, inline code, links, prompt-like
  instructions, or ambiguous code-like text.
- The user requests a workspace file with spaces in the path, parent traversal,
  symlink escape, unsupported extension, binary content, non-UTF-8 bytes, or a
  sensitive filename.
- The visible result is an error object rather than a successful translation.
- A request includes provider choice, remote confirmation, API key, base URL,
  headers, or any other remote/provider configuration even though remote
  translation remains out of scope.
- The source file has unsaved editor changes while translation is running.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The feature MUST define one canonical in-editor Zed translation
  flow that starts after the local development extension has been prepared and
  registered.
- **FR-002**: Users MUST be able to obtain a readable English-to-Spanish result
  inside Zed without manually starting the translator during the request.
- **FR-003**: The feature MUST document which input paths are supported in this
  iteration and distinguish those paths from unvalidated or future UX ideas.
- **FR-004**: Direct text input through the Zed flow MUST follow the active
  direct-text translation contract, including input limits, language direction,
  formatting preservation, offline default behavior, and privacy redaction.
- **FR-005**: Workspace file input through the Zed flow MUST follow the active
  file translation contract, including workspace-only access, safe path
  handling, supported file types, text encoding validation, binary-content
  rejection, sensitive-file rejection, and no source-file mutation.
- **FR-006**: Selection-based translation MUST NOT be documented as supported
  unless the real Zed flow is manually validated and the validation evidence
  identifies what selected content is sent.
- **FR-007**: Successful translation output MUST be presented as the primary
  readable result, without raw protocol content, setup logs, implementation
  metadata, or debug diagnostics mixed into the answer.
- **FR-008**: The feature MUST preserve the read-only product boundary: no
  editor buffer, file, or selection is modified automatically.
- **FR-009**: Failure output visible in Zed MUST be actionable and redacted. It
  MUST NOT expose source text, translated text, translatable segments, secrets,
  tokens, headers, provider credentials, environment dumps, or sensitive
  unredacted paths.
- **FR-010**: Real providers, remote network translation, provider selection,
  API-key setup, marketplace publication, and automatic replacement of editor
  content MUST remain out of scope.
- **FR-011**: The feature MUST include a manual Zed validation protocol covering
  the success path, unsupported or unsafe input, setup failure, no-mutation
  behavior, and the selection-support decision.
- **FR-012**: The feature MUST define reviewable evidence for whether the
  in-editor flow is low-friction enough to unblock later provider or packaging
  work.
- **FR-013**: The feature MUST distinguish the local translator provider
  boundary from the Zed Agent model route. Validation that uses sensitive,
  proprietary, or real workspace content MUST use a local/self-hosted model path
  or be recorded as blocked; non-local model routes may be used only with
  synthetic validation text.
- **FR-014**: Manual UX validation MUST use a translation-only Agent Profile or
  equivalent tool-permission setup that allows the local `translator-en-es` MCP
  tools while denying or requiring confirmation for built-in edit, write,
  delete, move, copy, terminal, URL fetch, and web-search tools.
- **FR-015**: Selection-support and tool-input evidence MUST be recorded as
  synthetic canary text, length/hash metadata, and redacted summaries only. It
  MUST NOT persist real source text, translated text, secrets, tokens, headers,
  workspace roots, or sensitive paths.

### Key Entities *(include if feature involves data)*

- **Zed Translation Session**: A single user attempt to request translation from
  inside Zed and inspect the result without leaving the editor.
- **Supported Input Path**: A validated route for intentionally providing
  content to the local translator, such as direct text or an authorized
  workspace file.
- **Selection Support Decision**: The recorded outcome that states whether
  selected editor text is supported, unsupported, or deferred after manual
  validation.
- **Visible Translation Result**: The user-facing result shown in Zed, either a
  readable Spanish translation or a redacted actionable failure.
- **No-Mutation Evidence**: Validation evidence showing that buffers, files, and
  unsaved editor content are unchanged after the translation flow.
- **Agent Privacy Boundary**: The recorded route used by the Zed Agent model,
  separate from the local MCP translator, including whether validation content
  may leave the workstation.
- **Translation-Only Agent Profile**: The Agent Panel profile or permissions
  setup used to keep the validation flow focused on the translator tools and
  prevent unintended editor mutation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Starting from a prepared and registered local extension, a reviewer
  can complete one successful direct-text translation entirely inside Zed in 3
  minutes or less without opening a terminal during the request.
- **SC-002**: A reviewer can complete one supported workspace-file translation
  through the Zed flow and confirm the source file remains byte-for-byte
  unchanged afterward.
- **SC-003**: The documented flow identifies every supported input path and
  clearly marks selection support as validated, unsupported, or deferred for
  this iteration.
- **SC-004**: Manual validation records at least one success case, one unsafe or
  unsupported input denial, one setup failure, and one no-mutation check using
  the actual Zed experience.
- **SC-005**: In every documented validation case, visible output and
  diagnostics contain no source text beyond what the user intentionally
  submitted as the request, no translated text in error diagnostics, no secrets,
  no headers, no tokens, no environment dumps, and no sensitive unredacted paths.
- **SC-006**: A successful result is readable without requiring the user to
  inspect raw protocol output or logs.
- **SC-007**: Remote or provider configuration attempts remain denied by default
  and are not presented as part of the supported Zed UX.
- **SC-008**: The feature can be reviewed with project-scoped commands and
  manual Zed smoke steps; it does not require installing project-specific
  runtimes or services globally as part of this repository workflow.
- **SC-009**: Manual validation records the Agent model route as `local`,
  `zed-hosted`, `provider-key`, `gateway`, `subscription`, or `unknown`; any
  non-local or unknown route uses only synthetic validation text.
- **SC-010**: Manual validation records the Agent tool-permission posture and
  shows that no automatic edit/write/delete/move/copy/terminal/fetch/search tool
  action is allowed as part of the supported translation workflow.
- **SC-011**: Starting from a prepared and registered local extension, a reviewer
  can complete one supported workspace-file translation in 5 minutes or less and
  record no-mutation evidence without inspecting raw protocol logs.
- **SC-012**: Selection validation uses a unique synthetic canary phrase and
  records only redacted evidence sufficient to decide `validated_supported`,
  `unsupported`, or `deferred`.

## Assumptions

- The local development extension and local translator startup path from the
  previous feature are merged and available before this feature begins.
- The user has Zed installed and can load local development extensions; if a
  host prerequisite is missing, installation or configuration remains governed
  by the workstation policy and is not part of this feature by default.
- The default provider remains deterministic and offline.
- The Zed Agent model used to orchestrate the tool call is a separate trust
  boundary from the local translator. This feature does not configure or bless a
  remote model route for sensitive content.
- The feature optimizes for reading assistance, not automated editing,
  replacement, batch conversion, or publication.
- The Agent Panel is the expected user-facing surface unless planning or manual
  validation identifies a safer, lower-friction in-editor route.
- Selection support is deliberately gated on real validation because the system
  must not infer or send editor content that Zed does not expose clearly and
  auditable.
