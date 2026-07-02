# Security And Testing Requirements Checklist: Translation Core Contract

**Purpose**: Validate that requirements, plan, and tasks define enough good-path, bad-path, and malicious-path coverage before implementation.
**Created**: 2026-07-01
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [x] CHK001 Are good-path requirements defined for direct text translation, file translation, provider output, and CLI success output? [Completeness, Spec §User Stories]
- [x] CHK002 Are bad-path requirements defined for empty input, unsupported language pairs, unsupported file types, missing files, oversized input, malformed provider output, and provider timeout? [Completeness, Spec §FR-006]
- [x] CHK003 Are malicious-path requirements defined for traversal, symlink escape, hidden sensitive files, prompt-injection content, secret patterns, argv leaks, malformed JSON, and malicious provider diagnostics? [Gap, Spec §Edge Cases]
- [x] CHK004 Are workspace boundary requirements explicit enough to prevent prefix confusion such as `/tmp/ws` versus `/tmp/ws-evil`? [Clarity, Spec §FR-SEC-B]
- [x] CHK005 Are requirements clear about whether TOCTOU-style path changes must be rejected or revalidated before reading? [Gap, Spec §Edge Cases]
- [x] CHK006 Are requirements defined for hidden sensitive files with allowed-looking extensions such as `.env.md` or `credentials.markdown`? [Gap, Spec §Edge Cases]

## Requirement Clarity

- [x] CHK007 Is "authorized workspace" defined clearly enough for tests to distinguish allowed files from path escapes? [Clarity, Spec §Assumptions]
- [x] CHK008 Is "protected code regions" defined clearly enough for fenced code, inline code, unclosed fences, HTML blocks, and frontmatter? [Clarity, Spec §FR-003]
- [x] CHK009 Is "obvious secret pattern" defined enough to include API keys, bearer tokens, private key headers, and `.env` assignments? [Clarity, Spec §FR-006]
- [x] CHK010 Is "remote provider use" defined enough to distinguish unconfigured provider, configured but unconfirmed provider, and confirmed-but-disallowed provider? [Clarity, Spec §FR-008]
- [x] CHK011 Is "logs or raw error details" defined enough to cover stdout, stderr, error messages, debug strings, and provider diagnostics? [Clarity, Spec §FR-010]

## Requirement Consistency

- [x] CHK012 Do the spec, constitution, and plan consistently forbid buffer/file mutation during translation? [Consistency, Spec §FR-009, Constitution §I]
- [x] CHK013 Do the spec, plan, and contracts consistently require clean success output without metadata? [Consistency, Spec §FR-004, Contracts]
- [x] CHK014 Do the spec, plan, and tasks consistently keep real providers, network calls, MCP server work, and Zed wrapper work out of this feature? [Consistency, Plan §Summary]
- [x] CHK015 Do error-code requirements in the spec match the result schema and planned tests? [Consistency, Spec §FR-006, Contracts]

## Acceptance Criteria Quality

- [x] CHK016 Are success criteria measurable for every major security gate: path rejection, remote denial, redaction, code preservation, and offline validation? [Measurability, Spec §Success Criteria]
- [x] CHK017 Are negative tests required before implementation for every behavior-changing security control? [Coverage, Constitution §III]
- [x] CHK018 Are exact limits specified for total input, segment size, segment count, output size, and provider timeout? [Measurability, Spec §FR-011]

## Scenario Coverage

- [x] CHK019 Are benign direct text, benign Markdown, and benign `.txt` cases represented separately? [Coverage, Tasks §US1-US2]
- [x] CHK020 Are malformed JSON, schema violations, extra fields, wrong types, and unsupported language pairs represented for the CLI contract? [Coverage, Tasks §US3]
- [x] CHK021 Are provider failure, provider timeout, provider oversized output, and provider diagnostics containing sensitive data represented separately? [Coverage, Tasks §US3]
- [x] CHK022 Are success stdout, failure stdout, stderr, and exit-code requirements represented for CLI validation? [Coverage, Tasks §US3]
- [x] CHK023 Are prompt-injection strings explicitly required to be treated as content rather than instructions? [Coverage, Tasks §US1]

## Edge Case Coverage

- [x] CHK024 Are path traversal variants with `..`, normalized paths, absolute paths, and root prefix confusion addressed? [Edge Case, Tasks §US2]
- [x] CHK025 Are symlink direct escape, directory escape, and chained symlink escape addressed? [Edge Case, Tasks §US2]
- [x] CHK026 Are non-UTF-8 bytes, NUL bytes, binary files renamed to allowed extensions, and mixed text/binary files addressed? [Edge Case, Tasks §US2]
- [x] CHK027 Are tricky Markdown cases addressed: nested fences, unclosed fences, multi-backtick inline code, links, images, blockquotes, tables, and frontmatter? [Edge Case, Tasks §US2]
- [x] CHK028 Are no-source-file-mutation requirements defined for translating allowed files? [Edge Case, Tasks §US2]

## Dependencies & Assumptions

- [x] CHK029 Is the Rust toolchain prerequisite documented without implying global installation? [Assumption, Plan §Technical Context]
- [x] CHK030 Are malicious fixtures required to be versioned and readable as part of the TDD flow? [Traceability, Tasks §Fixtures]
- [x] CHK031 Are future provider assumptions separated from this offline/mock feature? [Assumption, Plan §Summary]

## Ambiguities & Conflicts

- [x] CHK032 Are any requirements still relying on broad words like "safe", "secure", "clear", or "sensitive" without concrete examples? [Ambiguity]
- [x] CHK033 Are any tasks too broad to prove a single security behavior independently? [Clarity, Tasks]
- [x] CHK034 Are any malicious input classes documented in tasks but absent from spec edge cases or assumptions? [Traceability]
