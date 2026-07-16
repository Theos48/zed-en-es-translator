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

The exact compressed hashes and sizes also match three Git LFS pointers in the
official `mozilla/remote-settings-data` repository. All three pointers were
introduced together by `remote-settings-data-bot` in commit
`2cf7ff66844260317726822990a7f47a4730ec8a` on 2026-06-25:

| Role | Remote Settings attachment path | Locked LFS object |
|---|---|---|
| Model | `1d705201-9be0-40c4-a0b4-18d1e3973777.zst` | `ce7ba731…`, 22,085,725 bytes |
| Vocabulary | `b2b5907b-8759-4cc8-a721-89c283e6e45b.zst` | `76b9ef22…`, 348,996 bytes |
| Lexical shortlist | `51318160-1249-451f-80fb-12e61f8c1def.zst` | `0dd2945d…`, 1,687,731 bytes |

This closes the official-download provenance gap. It does not close the
license gap: `mozilla/remote-settings-data` has no repository license and its
LFS pointers do not carry an artifact license field.

There is one stronger identity bridge for the largest attachment. At archived
`mozilla/firefox-translations-models` commit
`e7957fc407441a5e3e35bbcbf9d60d9b35764618`, the MPL-2.0 repository's
`models/base-memory/enes/metadata.json` records the exact locked decompressed
model SHA-256 `3b1c3995…` and size 31,561,787 bytes, together with the same
language pair and architecture. The repository's three gzip LFS pointer names
also match the locked model, vocabulary and lexical-shortlist installed names.
Their compressed identities differ because the reviewed delivery uses Zstandard,
and no gzip/model body was downloaded for this comparison. The committed
metadata does not provide equivalent decompressed identities for vocabulary and
lexical shortlist, so this strengthens but does not silently complete the
three-artifact license conclusion.

## Native evidence inventory

The machine-readable actual-binary inventory is
[`native-sbom.json`](native-sbom.json). The native supply-chain gate binds its
runner hash/size and source commit to both lock files, requires unique component
IDs, and rehashes every license notice taken from the locked source tree.

A network-disabled diagnostic rebuild added a linker map and reproduced the
release size (12,000,008 bytes). Its extra linker option changed the diagnostic
binary hash, so the map is used only to enumerate archive members; the release
hash, release `build.ninja` link manifest and release-symbol scan remain the
identity evidence. Together they establish the following closure:

| Component/evidence | Repository-level text observed | Review state |
|---|---|---|
| Project wrapper, Marian, CLI11, yaml-cpp, pathie-cpp, spdlog/fmt, cnpy, Faiss, half-float, mio, PHF and zstr | MIT/BSD candidate texts or header notices, all individually hashed in the SBOM | Human conclusions and notice retention pending |
| Mozilla Translations/Bergamot | Root `MPL-2.0` text at exact commit `f31423c…` | Covered-source/notice handling pending human acceptance |
| intgemm | Exact submodule `f740151…`; runtime candidate `MIT` | Human conclusion pending; Catch test notice is outside the runtime closure |
| SentencePiece closure | Exact submodule `ae41b77…`; Apache-2.0 plus hashed Abseil, darts-clone, esaxx and protobuf-lite texts | Recursive human conclusions/notice set pending |
| ssplit-cpp | Exact submodule `a311f98…`; only `ssplit.cpp.o` and `regex.cpp.o` link under candidate `Apache-2.0` | Optional LGPL nonbreaking-prefix data is neither linked nor packaged |
| zlib, sse_mathfun and threadpool | Hashed zlib-style notices | Human conclusions/notice retention pending |
| Microsoft CNTK call-stack helper | Source header declares MIT | Referenced upstream root notice is not adjacent to the vendored file |
| Marian `any_type.h` | The final release contains its symbols; header cites external inspiration but has no explicit notice | `NOASSERTION`; attribution/conclusion is a blocking human item |
| static PCRE2 | Exact Debian package `libpcre2-dev:amd64=10.42-1`; package copyright hash recorded | Composite PCRE2/sljit/binary-exception conclusion pending |
| Dynamic ELF allowlist | Exact bookworm glibc/libstdc++/libgcc/libm/loader package versions and notice hashes recorded | System-library delivery treatment pending |

The build compiles generated ONNX/protobuf objects into `libmarian.a`, but the
linker map and release symbols show that no ONNX member enters the runner.
SentencePiece's protobuf-lite implementation does enter and is inventoried.
The source pins for fbgemm, nccl, ruy and simd-related repositories likewise do
not imply that those projects are shipped.

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
- <https://github.com/mozilla/firefox-translations-models/blob/e7957fc407441a5e3e35bbcbf9d60d9b35764618/LICENSE>
- <https://github.com/mozilla/firefox-translations-models/blob/e7957fc407441a5e3e35bbcbf9d60d9b35764618/models/base-memory/enes/metadata.json>
- <https://github.com/mozilla/remote-settings-data/commit/2cf7ff66844260317726822990a7f47a4730ec8a>
- <https://github.com/mozilla/remote-settings-data/blob/2cf7ff66844260317726822990a7f47a4730ec8a/attachments/main-workspace/translations-models-v2/1d705201-9be0-40c4-a0b4-18d1e3973777.zst>
- <https://github.com/mozilla/remote-settings-data/blob/2cf7ff66844260317726822990a7f47a4730ec8a/attachments/main-workspace/translations-models-v2/b2b5907b-8759-4cc8-a721-89c283e6e45b.zst>
- <https://github.com/mozilla/remote-settings-data/blob/2cf7ff66844260317726822990a7f47a4730ec8a/attachments/main-workspace/translations-models-v2/51318160-1249-451f-80fb-12e61f8c1def.zst>
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

The manager derives `artifact_set_digest` from a domain-separated, canonical
typed payload containing the complete artifact identities, license and
delivery conclusions, resource budgets and publication status. It rejects a
manifest when the recorded digest differs. Approval records and their digest
fields are excluded only to avoid self-reference. Therefore any later change
to a URL, hash, size, conclusion, budget or publication state requires a new
digest and fresh matching approvals.

## Required completion package

1. Human reviewer accepts or rejects the SBOM candidates, including an explicit
   conclusion for Marian `any_type.h`, PCRE2's composite terms, system-library
   treatment, retained notices and MPL covered-source handling.
2. Human reviewer accepts authoritative artifact-level attribution for the
   three exact Remote Settings attachments, or chooses another reviewed set.
3. If accepted, record the reviewed SPDX/license source and
   `local_acquisition_approved` conclusion for the runner and each resource.
4. Compute the canonical final manifest and evidence digests only after those
   conclusions are fixed.
5. Human `project_maintainer` approves or rejects F012 local activation bound
   to those exact digests and the `local_activation` scope.
6. Human `f009_human_reviewer` explicitly keeps publication blocked or records
   a separate approval; local activation never implies publication.

Until all F012 items pass, the only valid product outcome is
`BLOCKED_LICENSE_APPROVAL`; Mock and LibreTranslate remain unchanged.
