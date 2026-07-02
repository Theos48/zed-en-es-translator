# Feature Specification: Translation Core Contract

**Feature Branch**: `001-translation-core-contract`

**Created**: 2026-07-01

**Status**: Implemented

**Input**: User description: "Create the first technical Spec Kit feature for a safe EN->ES translation core contract. The feature must support an offline deterministic provider, clean translated output or normalized errors, formatting preservation, explicit limits, safe file reads for Markdown/text files, no real network provider, no buffer edits, and negative security checks."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Translate Direct Text Safely (Priority: P1)

A user provides a small English text fragment and receives Spanish text for
reading, without changing the original text and without sending content to a
remote service.

**Why this priority**: Direct text translation is the smallest useful slice and
proves the request/response contract without editor or provider complexity.

**Independent Test**: Submit a simple English text input and verify that the
system returns only translated Spanish text, with no metadata in the normal
success output and no modification of the original input.

**Acceptance Scenarios**:

1. **Given** valid English text under the configured input limit, **When** the
   user requests translation, **Then** the system returns Spanish text in the
   normal success output.
2. **Given** empty or whitespace-only input, **When** the user requests
   translation, **Then** the system returns a clear normalized validation error.
3. **Given** input over the configured limit, **When** the user requests
   translation, **Then** the system rejects it before provider processing.

---

### User Story 2 - Translate Allowed Documentation Files (Priority: P2)

A user provides a path to an allowed documentation file in the authorized
workspace and receives readable Spanish output while preserving Markdown
structure and protected code regions.

**Why this priority**: Documentation files such as README and notes are a core
use case, but file access must be bounded before any implementation reads from
disk.

**Independent Test**: Use an allowed Markdown or text fixture inside the
authorized workspace and verify translated output, preserved code regions, and
rejection of unsafe paths.

**Acceptance Scenarios**:

1. **Given** an allowed `.md`, `.markdown`, or `.txt` file inside the authorized
   workspace and under the size limit, **When** the user requests file
   translation, **Then** the system returns readable Spanish output.
2. **Given** a Markdown file containing fenced code or inline code, **When** the
   user requests translation, **Then** protected code content remains unchanged.
3. **Given** a path outside the authorized workspace, a traversal path, or a
   symlink escaping the workspace, **When** the user requests file translation,
   **Then** the system rejects the request with a normalized path error.
4. **Given** a binary or non-UTF-8 file, **When** the user requests file
   translation, **Then** the system rejects the request before translation.

---

### User Story 3 - Report Failures Without Leaking Content (Priority: P3)

A user receives clear errors when translation cannot proceed, while source
text, translated text, file paths, secrets, and protected segments remain out
of normal logs and error details.

**Why this priority**: Failure paths are where accidental leaks are most likely,
especially before real providers are added later.

**Independent Test**: Trigger validation, limit, path, secret, and timeout
errors using fixtures and verify normalized error codes and redacted diagnostics.

**Acceptance Scenarios**:

1. **Given** a request that would require a remote provider, **When** no remote
   confirmation exists, **Then** the system denies the operation without sending
   content outside the machine.
2. **Given** input containing an obvious secret pattern and a non-local provider
   path, **When** the user requests translation, **Then** the system blocks the
   operation with a normalized secret-detection error.
3. **Given** a simulated provider failure or timeout, **When** translation is
   requested, **Then** the system returns a normalized provider error without
   exposing source content or raw provider diagnostics.

### Edge Cases

- Empty, whitespace-only, or missing text input.
- Input exactly at and just above the total input size limit.
- A single segment exactly at and just above the segment size limit.
- More segments than the configured maximum.
- Markdown with fenced code, inline code, nested or alternating fences, unclosed
  fences, multi-backtick inline code, links, images, blockquotes, tables,
  headings, lists, HTML blocks, and frontmatter.
- Files with allowed extensions but binary or non-UTF-8 content, including NUL
  bytes or mixed text/binary payloads.
- Paths using `..`, unauthorized absolute paths, directory symlinks, chained
  symlinks, and symlinks that escape the authorized workspace.
- Paths that rely on root-prefix confusion such as an allowed workspace path
  with a similarly named sibling directory.
- Time-of-check/time-of-use style file changes between validation and reading.
- Hidden sensitive files such as environment or credential files.
- Hidden sensitive files with allowed-looking extensions such as `.env.md` or
  `credentials.markdown`.
- Remote provider paths that are unconfigured, configured but unconfirmed for
  the request, or confirmed but not allowlisted for this feature.
- Obvious secret patterns in input before a remote provider path, including API
  keys, bearer tokens, private key headers, and `.env` assignments.
- Prompt-injection text that attempts to change instructions, request
  environment data, force remote sending, or alter the contract.
- Malformed machine-readable input, unknown fields, wrong field types, and
  unsupported language or formatting values.
- Integration misuse where source text, secrets, or sensitive paths are supplied
  outside the defined machine-readable input channel.
- Provider timeout, malformed provider response, oversized provider output, or
  provider diagnostics that include sensitive data.
- Logs and errors generated during all failure cases.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept direct text input for English-to-Spanish
  translation.
- **FR-002**: System MUST accept file translation only for `.md`, `.markdown`,
  and `.txt` files during this feature.
- **FR-003**: System MUST preserve Markdown structure and protected code regions
  when translating documentation content.
- **FR-004**: System MUST return clean translated text as the only normal
  success output.
- **FR-005**: System MUST return normalized errors with a stable code and
  user-readable message when translation cannot proceed.
- **FR-006**: System MUST reject empty input, unsupported language pairs,
  unsupported file types, missing files, unsafe paths, oversized input,
  non-UTF-8 input, binary content, requests without translatable segments,
  detected secrets for remote paths, unconfigured providers, unconfirmed remote
  requests, provider failures, provider timeouts, and internal failures.
- **FR-007**: System MUST use an offline deterministic provider for this feature
  so tests do not require network, payment, secrets, or external services.
- **FR-008**: System MUST deny all remote provider use unless explicitly
  configured and confirmed for the individual request.
- **FR-009**: System MUST NOT modify editor buffers or source files.
- **FR-010**: System MUST NOT include source text, translated text, translatable
  segments, secrets, headers, tokens, or sensitive paths in logs or raw error
  details.
- **FR-010a**: System MUST treat prompt-injection text as translatable or
  protected content only; such text MUST NOT change provider selection,
  confirmation state, logging behavior, command execution, or output contract.
- **FR-010b**: System MUST reject attempts to carry source text, secrets, or
  sensitive file paths outside the defined machine-readable input channel.
- **FR-011**: System MUST enforce explicit limits: 20 KiB total input, 4 KiB per
  segment, 256 segments, 40 KiB output, and 15 s provider timeout.
- **FR-012**: System MUST expose a machine-readable request/response contract
  for later integration layers.
- **FR-013**: System MUST preserve ambiguous content instead of translating it
  when translation safety cannot be determined.
- **FR-014**: System MUST defend against file-read race or revalidation issues
  so content outside the authorized workspace is never processed.
- **FR-015**: System MUST reject sensitive hidden files and credential-like file
  names even when their extension otherwise looks supported.
- **FR-SEC-A**: System MUST NOT modify editor buffers unless a later
  constitution amendment allows it.
- **FR-SEC-B**: System MUST reject unsafe file paths, unsupported file types,
  non-UTF-8 input, and binary content.
- **FR-SEC-C**: System MUST deny remote provider use unless explicitly
  configured and confirmed per request.
- **FR-SEC-D**: System MUST NOT log source text, translated text, segments,
  secrets, headers, tokens, or sensitive paths.
- **FR-TEST-A**: System MUST define testable acceptance criteria and negative
  tests before implementation.

### Key Entities

- **Translation Request**: The input to a translation operation, including text,
  language pair, tone, formatting preservation intent, input kind, and optional
  local file context.
- **Translation Success**: A successful operation containing only clean
  translated text for normal user reading.
- **Translation Failure**: A failed operation containing a normalized error code
  and user-readable message.
- **Translatable Segment**: A piece of content approved for provider processing
  after protected content has been excluded.
- **Protected Content**: Markdown code, ambiguous content, sensitive content, or
  any content that must be preserved unchanged.
- **Provider**: A translation backend boundary that receives only approved
  translatable segments.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of valid direct text fixtures under 20 KiB return clean
  translated text with no metadata in normal success output.
- **SC-002**: 100% of Markdown fixtures with fenced code or inline code preserve
  protected code content unchanged.
- **SC-003**: 100% of unsafe file path fixtures are rejected before file content
  is processed.
- **SC-004**: 100% of remote-provider attempts without confirmation are denied
  before content leaves the machine.
- **SC-005**: 100% of configured negative security fixtures produce normalized
  errors without source text, translated text, secrets, or sensitive paths in
  logs or raw diagnostics.
- **SC-006**: A new contributor can run the documented validation flow and see
  all tests/checks pass without configuring network access, paid services, or
  secrets.

## Assumptions

- The first feature is a technical foundation and does not need a real
  translation provider.
- The initial file translation scope is documentation text: `.md`, `.markdown`,
  and `.txt`.
- Full-file source-code support is postponed until a reliable segmenter/parser
  and additional negative tests exist.
- The authorized workspace root is provided by the integration layer or test
  harness.
- Spanish output uses neutral technical Spanish.
- Remote provider behavior is represented only by denial and safety checks in
  this feature.
- No remote provider is allowlisted in this feature; remote-provider tests assert
  denial states and must not add network-capable provider code.
