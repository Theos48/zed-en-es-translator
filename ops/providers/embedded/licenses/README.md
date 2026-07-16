# Embedded provider license gate

Status: **blocked** (`BLOCKED_LICENSE_APPROVAL`).

This record separates mechanically verified license evidence from the human
conclusions and approvals required by F012/F009. It is not legal advice and it
does not authorize activation, bundling, rehosting, redistribution or
publication.

## Exact scope reviewed mechanically

- Runner source: `mozilla/translations` at
  `f31423c7c2c6ed8ae57d71a3d19a9db6f156060e` plus the recursive revisions in
  `../source.lock.json`.
- Reproducible runner: SHA-256
  `9743b4a8efbe9471145c08fcc75a42fdc3d85e6035e797023b3a623d91e886fe`,
  12,000,008 bytes.
- Model resources: the three exact Remote Settings records, URLs, compressed
  and decompressed hashes/sizes in `../provider.lock.json`.
- Delivery evaluated by F012: project-built runner plus consented acquisition
  from the exact Mozilla attachment URLs into a private user XDG root. No
  bundle, rehost or publication is in scope.

On 2026-07-15 the three official Remote Settings records were fetched by
record ID and matched every name, role, language, architecture, version,
attachment location, compressed size/hash and decompressed size/hash in the
lock. Their record schemas contain neither a `license` nor an `spdx` field.
No attachment body was downloaded during this metadata review.

## Native evidence inventory

The release link closure names Bergamot, Marian, SentencePiece, intgemm,
ssplit-cpp and static PCRE2 directly. Marian's static archive also builds
vendored yaml-cpp, pathie-cpp, zlib, Faiss, cnpy, generated ONNX/protobuf and
phf objects. Exact license texts exist in the locked tree for most components:

| Component/evidence | Repository-level text observed | Review state |
|---|---|---|
| Project runner wrapper | Workspace `MIT` | Final binary conclusion pending complete closure review |
| Mozilla Translations/Bergamot | Root `MPL-2.0` text at the exact commit | MPL notice/source obligations pending human acceptance |
| Marian fork | `MIT` text | Candidate conclusion only |
| intgemm | `MIT` text | Candidate conclusion only |
| SentencePiece | `Apache-2.0` text plus vendored Abseil, darts-clone, esaxx and protobuf-lite texts | Recursive conclusion/notice set incomplete |
| ssplit-cpp | `Apache-2.0` for C++/CMake; `LGPL-2.1` for optional nonbreaking-prefix data | Reviewer must confirm the delivered/used data subset |
| yaml-cpp, Faiss, cnpy, phf | Permissive texts in the locked tree | Candidate conclusions only |
| pathie-cpp | BSD-style text in the locked tree | SPDX conclusion pending |
| vendored zlib | License notice in its locked `README` | SPDX conclusion pending |
| static PCRE2 from the build image | Build input and link presence verified | Exact source/license notice and delivery treatment pending |
| generated ONNX/protobuf objects | Built by the locked Marian recipe; no adjacent ONNX license file found | Origin and applicable notice unresolved |
| Dynamic ELF allowlist | glibc/libstdc++/libgcc/libm/loader only | System-library treatment must be recorded by reviewer |

This inventory is evidence, not a complete SBOM or an SPDX conclusion. In
particular, the static archive contains more compiled object groups than the
small direct-library list alone reveals; a reviewer must bind the actual final
binary closure to the retained notices and source offer.

## Model evidence and unresolved attribution

Both `mozilla/translations` and the archived
`mozilla/firefox-translations-models` repository display `MPL-2.0` at repository
level, and Mozilla documents the latter as the prior source of models deployed
to Remote Settings. That is useful provenance evidence, but the current three
Remote Settings records do not repeat a license identifier or link each exact
attachment to a license text. Under the feature contract, repository-level
evidence is not silently promoted into an artifact-level conclusion.

Consequently `spdx_conclusion` and `license_source` remain `null`, and
`delivery_permission` remains `review_required`, for the model, vocabulary and
lexical-shortlist entries. The production manifest therefore remains
unparseable as an approved provider and cannot expose a consent digest.

Primary evidence:

- <https://github.com/mozilla/translations/tree/f31423c7c2c6ed8ae57d71a3d19a9db6f156060e>
- <https://github.com/mozilla/firefox-translations-models>
- <https://firefox.settings.services.mozilla.com/v1/buckets/main/collections/translations-models-v2/records>
- <https://www.mozilla.org/en-US/MPL/2.0/FAQ/>

## Approval state

| Decision | Required authority/scope | Recorded value | Result |
|---|---|---|---|
| F012 local activation | Human `project_maintainer`, exact manifest digest, `local_activation` | `local_approval=null` | Blocked |
| F009 publication | Human `f009_human_reviewer`, exact manifest digest, `publication` | `publication_approval=null` | Separately blocked |

Automated reports cannot fill either row. F012 may not activate until a human
reviewer completes all artifact/SPDX conclusions, accepts the notice/source
obligations, records an evidence digest, and signs the exact final manifest for
local activation. F009 must later review the actual delivery artifact before
any bundling, redistribution or publication claim.

## Required completion package

1. Complete an actual-binary native SBOM, including static/vendor/generated
   code and exact license texts.
2. Record accepted SPDX conclusions and notice/source-offer handling for the
   runner and every model/config artifact.
3. Obtain authoritative artifact-level attribution for the three exact Remote
   Settings attachments, or choose another reviewed artifact set.
4. Compute the final manifest digest only after those conclusions are fixed.
5. Record the human F012 local-activation approval bound to that digest.
6. Keep F009 publication blocked until its independent human decision.

Until all F012 items pass, the only valid product outcome is
`BLOCKED_LICENSE_APPROVAL`; Mock and LibreTranslate remain unchanged.
