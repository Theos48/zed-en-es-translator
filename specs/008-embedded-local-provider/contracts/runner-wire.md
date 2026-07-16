# Contract: Embedded Runner Wire and Process Boundary

**Feature**: `008-embedded-local-provider`
**Initial wire version**: `1`

## Invocation model

The parent starts one exact verified `translator-embedded-runtime` process for
one already-validated provider request. There is no shell, daemon, listening
socket, dynamic command name, user-supplied argument or runtime download.

Process construction MUST:

- open the executable from the validated immutable set and recheck its
  identity before launch;
- use fixed arguments only, including exact manifest/model basenames;
- set cwd to the validated set root;
- clear inherited environment and add only fixed locale/runtime values proven
  necessary by tests;
- exclude proxy, credential, workspace and arbitrary loader variables;
- connect only bounded stdin/stdout/stderr pipes;
- begin the existing 15-second deadline before launch/model load;
- concurrently drain stdout and stderr, then kill and reap on deadline;
- perform no retry.

The runner binary must have no updater/downloader and must pass the native
dependency and zero-network acceptance checks.

## Request

Exactly one UTF-8 JSON object is written, followed by EOF:

```json
{
  "wire_version": 1,
  "source_language": "en",
  "target_language": "es",
  "tone": "technical_neutral",
  "preserve": ["markdown_structure", "code", "links"],
  "segments": ["Public synthetic text."]
}
```

Rules:

- unknown/missing/duplicate fields and trailing data are rejected;
- language/tone/preserve are exact existing product values;
- `segments` contains 1..256 strings, each <=4 KiB, aggregate <=20 KiB;
- the core applies segmentation, Markdown protection, secret and file-safety
  gates before encoding this request;
- request bytes are ephemeral and are never logged, persisted or included in
  crash/evidence records.

## Success response

Exactly one UTF-8 JSON object is returned, followed by EOF:

```json
{
  "wire_version": 1,
  "translations": ["Texto sintetico publico."]
}
```

Rules:

- response version equals the request version;
- translations have exact request cardinality and order;
- each translated string is non-empty for a non-empty permitted segment;
- decoded semantic translation output totals <=40 KiB;
- parent transport caps allow only a small fixed JSON-framing overhead beyond
  the semantic limit and cannot weaken it;
- unknown/missing/duplicate fields, trailing bytes and non-UTF-8 fail closed.

The parent remains authoritative for reassembly and preservation of protected
Markdown regions.

## Failure response and stderr

The runner exits non-zero and may emit only a bounded machine class such as:

```json
{"wire_version":1,"error":"MODEL_LOAD_FAILED"}
```

Allowed runner classes are fixed, content-free and mapped to existing stable
provider errors by the parent. Stderr is capped at a small implementation
constant and is discarded or normalized; it cannot be forwarded verbatim.

Prohibited output includes input, translation, segment indices paired with
content, model bytes, raw native exceptions, environment, URLs, full paths,
user/workspace identifiers or sensitive host details.

## Timeout and process cleanup

- deadline covers queue wait, launch, model load, inference, decode and child
  reap;
- stdout/stderr readers have independent hard caps and cannot deadlock the
  parent when one pipe fills;
- on timeout or cap violation the parent terminates the child, waits for it,
  closes pipes and returns a normalized failure;
- the child cannot outlive the request or become a service;
- partial response bytes are never used.

## Error mapping

| Runner/parent condition | Normalized result |
|---|---|
| Verified executable/model cannot be resolved | Provider not configured/readiness failure |
| Launch denied or unexpected exit | Provider failed |
| Deadline reached | Provider timeout |
| Malformed/oversized/non-UTF-8/cardinality mismatch | Invalid provider response |
| Input wire/limit violation | Existing invalid request/limit error before spawn |
| Installation lease busy until deadline | Busy/provider timeout; no spawn |

No condition silently chooses Mock or remote translation after explicit
embedded selection.

## Required tests

- exact command/cwd/environment and no shell;
- request schema, all existing count/size limits and one ordered batch;
- malformed, duplicate, trailing, non-UTF-8 and oversized request/response;
- empty, reordered and cardinality-mismatched translations;
- stdout/stderr pipe saturation without deadlock;
- deadline kill and reap, exit signal and orphan checks;
- no retry and no fallback;
- fake runner injection through a Rust test seam, never a production env key;
- real native helper smoke with external networking disabled;
- ELF dependency allowlist, portable CPU baseline and absence of downloader
  code/network syscalls.
