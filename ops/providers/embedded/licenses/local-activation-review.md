# F012 local activation review

**Date prepared**: 2026-07-16

**Scope**: private local acquisition and activation of the exact locked
`bergamot-en-es-linux-x86_64-v1` set

**Status**: `PENDING_MAINTAINER_APPROVAL`

**Publication**: F009 remains blocked

This record presents the remaining human decisions without treating an
automated inventory as legal approval. Acceptance authorizes only the exact
manifest for private local use. It does not authorize bundling, rehosting,
binary distribution or publication.

## Decision already recorded

- Marian `any_type.h`: accepted as `MIT`. The exact file is carried by the
  original Marian project and the locked Mozilla Marian fork under Marian's
  project-level MIT license. Its original inspiration links remain intact.

## Proposed native conclusions

### PCRE2

- Exact linked package: `libpcre2-dev:amd64=10.42-1` from the pinned Bookworm
  build image.
- Proposed expression:
  `LicenseRef-PCRE2-BSD-3-Clause-with-binary-exception AND BSD-2-Clause`.
- Basis: the upstream PCRE2 10.42 license contains the three-clause terms and
  binary-library-like exception. The release binary contains PCRE2 JIT/sljit
  symbols, and sljit carries separate two-clause BSD terms.
- Local treatment: retain the complete PCRE2 license in the reviewed notice
  evidence. F009 must reassess notice delivery before distributing a binary.

Primary evidence:

- <https://github.com/PCRE2Project/pcre2/blob/pcre2-10.42/LICENCE>
- the exact Debian package copyright hash recorded in `native-sbom.json`;
- the release binary symbol/link evidence recorded by the native supply-chain
  test.

### Other native components and notices

The proposed local conclusion accepts the exact candidates already enumerated
and hashed in `native-sbom.json`:

- `MPL-2.0`: Mozilla Translations/Bergamot;
- `MIT`: the project wrapper, Marian, cnpy, Faiss, intgemm, mio, PHF, spdlog,
  fmt, yaml-cpp, zstr, CNTK call-stack helper and Marian `any_type.h`;
- `BSD-3-Clause`: CLI11, half-float, darts-clone and protobuf-lite;
- `BSD-2-Clause`: pathie-cpp;
- `Apache-2.0`: SentencePiece, its Abseil subset and linked ssplit code;
- `Zlib`: zlib, sse_mathfun and threadpool;
- the PCRE2 composite expression above.

For F012 the runner is built locally from the exact fetched source rather than
distributed by the project. The fetched tree preserves the upstream license
texts, every linked notice is hashed in `native-sbom.json`, and the complete
source commit/dependency graph is in `source.lock.json`. This satisfies the
proposed private local-use handling. If F009 later distributes the executable,
it must ship a notice bundle and make the exact MPL covered source available;
this F012 decision does not pre-approve that work.

The ELF `NEEDED` entries (`libc`, `libm`, `libstdc++`, `libgcc_s` and the
loader) are proposed as external system libraries, not project-delivered
artifacts. The SBOM keeps the pinned build-image package notices as build
evidence but does not claim that those Debian library binaries are bundled.
F009 must review the actual publication target and its runtime packaging.

## Proposed Mozilla resource conclusion

Proposed SPDX conclusion for the exact model, vocabulary and lexical shortlist:
`MPL-2.0`, with license source:

<https://github.com/mozilla/firefox-translations-models/blob/e7957fc407441a5e3e35bbcbf9d60d9b35764618/LICENSE>

This conclusion is an explicit maintainer inference from the following chain,
because the current Remote Settings records do not contain a `license` field:

1. The archived official model repository is licensed `MPL-2.0` and contains
   the `models/base-memory/enes` model, vocabulary and lexical-shortlist set.
2. Its metadata records the exact installed model hash and size locked here.
3. Mozilla's official `remote-settings-data` commit
   `2cf7ff66844260317726822990a7f47a4730ec8a` introduced both the current three
   Zstandard attachments and uncompressed LFS objects whose OIDs and sizes are
   the exact three installed identities in `provider.lock.json`:

   | Role | Exact installed identity evidence |
   |---|---|
   | Model | [`3b1c3995…`, 31,561,787 bytes](https://github.com/mozilla/remote-settings-data/blob/2cf7ff66844260317726822990a7f47a4730ec8a/attachments/main-workspace/translations-models/a4ba0e94-16de-4058-9a44-5bbbbb3c8640.bin) |
   | Vocabulary | [`5ae254fa…`, 816,054 bytes](https://github.com/mozilla/remote-settings-data/blob/2cf7ff66844260317726822990a7f47a4730ec8a/attachments/main-workspace/translations-models/5ac5c966-d9b3-4e89-a972-4c23e82e7157.spm) |
   | Lexical shortlist | [`7d51237c…`, 4,198,436 bytes](https://github.com/mozilla/remote-settings-data/blob/2cf7ff66844260317726822990a7f47a4730ec8a/attachments/main-workspace/translations-models/1834a61e-0331-4c4a-bbc0-dda02afa8188.bin) |

4. The current records independently bind the compressed Zstandard hashes,
   decompressed hashes, sizes, language direction, architecture and version.

Acceptance permits acquisition only from the exact Mozilla HTTPS attachment
URLs and only into the private XDG store. It does not permit rehosting or
publication.

## Approval statement

To accept the proposals, the project maintainer must explicitly approve all of
the following for F012 local activation:

1. the proposed PCRE2 composite conclusion and notice retention;
2. the native candidate licenses, local-build notice/source handling and
   external-system-library treatment;
3. `MPL-2.0` attribution for the three exact Mozilla resources using the
   identity chain above;
4. `local_acquisition_approved` for the runner and each exact resource;
5. continued `blocked` status for F009 publication.

Only after that decision may `provider.lock.json` be finalized, its canonical
digest computed and a separate human `local_activation` approval be bound to
that digest. Acquisition still requires disclosure plus explicit consent to
the final digest.
