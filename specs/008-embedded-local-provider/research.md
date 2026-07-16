# Phase 0 Research: Embedded Local Provider

**Feature**: `008-embedded-local-provider`
**Date**: 2026-07-15
**Status**: Complete for planning; implementation measurements remain promotion gates

## Decision 1: Prototype Mozilla Translations/Bergamot as the only F012 path

**Decision**: Use the maintained Mozilla Translations inference source and the
Firefox Translations English-to-Spanish `base-memory` resources as the single
candidate taken into implementation. The implementation lock starts from the
reviewed `mozilla/translations` source snapshot
`f31423c7c2c6ed8ae57d71a3d19a9db6f156060e`, pins every recursive native
dependency, and pins the model, vocabulary and lexical-shortlist records by
exact identity. Promotion remains conditional on the gates below.

**Rationale**:

- Firefox uses Bergamot for on-device translation, including direct `en -> es`.
- The maintained source provides a native inference path suitable for a small
  project-owned helper rather than a user-managed service.
- Mozilla Remote Settings publishes compressed and decompressed sizes and
  SHA-256 values for every current `en -> es` resource.
- Its reviewed footprint is substantially smaller than the viable OPUS-MT
  alternatives and requires no account, key, remote service or Python runtime
  during normal use.

**Mandatory promotion gates**:

1. reproduce the native build from the complete source/dependency lock inside
   the existing project build container;
2. prove a portable reviewed Fedora `x86_64` CPU baseline rather than the
   upstream `native` compiler default;
3. meet the memory, thread, disk and 15-second latency budgets in the benchmark
   contract;
4. prove zero external network attempts during readiness and translation;
5. complete the runtime, dependency and model license/provenance report,
   including MPL source-offer and notice obligations;
6. keep publication or bundling blocked until the artifact-level legal review
   explicitly supports it.

If any mandatory gate fails and no reviewed fallback passes the same gates,
F012 records a no-go result and preserves Mock and LibreTranslate rather than
shipping an unsafe provider.

**Primary sources**:

- <https://github.com/mozilla/translations>
- <https://github.com/mozilla/translations/commit/f31423c7c2c6ed8ae57d71a3d19a9db6f156060e>
- <https://firefox-source-docs.mozilla.org/toolkit/components/translations/resources/01_overview.html>
- <https://firefox-source-docs.mozilla.org/toolkit/components/translations/resources/03_bergamot.html>
- <https://firefox.settings.services.mozilla.com/v1/buckets/main/collections/translations-models-v2/records?sourceLanguage=en&targetLanguage=es>

## Decision 2: Pin the reviewed resource set; never follow `latest`

**Decision**: The versioned artifact manifest records exact Remote Settings
record IDs, attachment URLs, filenames, versions, compressed/decompressed
sizes and hashes. Production preparation consumes only those reviewed values;
it never queries a registry to discover a newer version.

The reviewed `en -> es` version-3.0 resources on 2026-07-15 are:

| Role | Transfer bytes | Installed bytes |
|---|---:|---:|
| Model | 22,085,725 | 31,561,787 |
| Vocabulary | 348,996 | 816,054 |
| Lexical shortlist | 1,687,731 | 4,198,436 |
| **Model-set total** | **24,122,452** | **36,576,277** |

The model-set totals are approximately 23.00 MiB transferred and 34.88 MiB
installed. These are upstream artifact facts, not runtime performance results.
The native runner size and full installed footprint remain measured
implementation evidence.

**Rationale**: Registry discovery at preparation time would turn an explicit
reviewed update into an automatic supply-chain decision. Exact dual hashes
also detect substitution both before and after decompression.

**Alternatives rejected**:

- Use the official runtime WASM v4 directly: deferred because it is an
  Emscripten/browser artifact with Firefox-specific JavaScript integration,
  not a documented standalone WASI component.
- Rehost or bundle the model: deferred to F009 because the registry does not
  repeat a license field on each blob and F012 does not make a publication
  claim.

## Decision 3: Use a native one-shot helper process, not in-process C++

**Decision**: Add a project-owned native helper, provisionally named
`translator-embedded-runtime`. `translator-core` invokes it once per provider
request through a versioned JSON stdin/stdout contract. The process receives
the complete already-bounded segment batch, performs no network operation,
returns one ordered response, and exits.

The parent launches the exact verified executable without a shell, clears the
environment, supplies a controlled working directory, concurrently drains
bounded stdout/stderr, applies the existing 15-second request deadline, then
kills and reaps on timeout. It never retries.

**Rationale**:

- A child process can be terminated if native inference stalls; C++ running
  inside the Rust process cannot be cancelled safely at the constitutional
  deadline.
- The C++ ABI, exceptions and ownership remain outside Rust's safe core.
- `ProviderSelection` can remain cloneable and compatible with CLI, LSP and
  MCP regression builds.
- One batch maps to Bergamot's multiple-segment API and preserves segment
  order/cardinality.

**Benchmark-driven fallback**: If one-shot cold load cannot meet the deadline,
the design may move to an LSP-owned persistent child only after a new design
review specifies process lifetime, crash recovery, concurrency, memory and
shutdown. Direct FFI is not an implicit fallback.

## Decision 4: Keep inference out of the Zed extension WASM

**Decision**: The Zed extension continues to launch the native
`translator-lsp`. The LSP and CLI select the same `EmbeddedProcessProvider`;
the extension WASM does not run inference or download artifacts implicitly
from `language_server_command`.

**Rationale**: Zed procedural extensions are WebAssembly launch/integration
components. Zed API 0.7.0 provides native-server download and launch helpers,
but no observed custom first-run consent dialog or translation request-handler
hook. Silent acquisition during LSP startup would not satisfy informed
consent.

F012 therefore uses an explicit documented preparation command. Publication
may later adapt the same locked identities to the Zed extension work directory
only after F009 defines consent and delivery UX.

**Sources**:

- <https://zed.dev/docs/extensions/developing-extensions>
- <https://zed.dev/docs/extensions/installing-extensions>
- <https://zed.dev/docs/extensions/capabilities>
- <https://docs.rs/zed_extension_api/0.7.0/zed_extension_api/fn.download_file.html>

## Decision 5: Preserve the four-key Zed configuration allowlist

**Decision**: Add exactly one provider value,
`TRANSLATOR_PROVIDER=embedded_local`. In this mode the existing URL, API-key
reference and remote-enable keys must all be absent. No user-facing executable,
model-root, URL or arbitrary-argument setting is added.

The implementation resolves the artifact profile from a fixed product-owned
XDG location and a versioned profile ID. Test injection remains internal to
the Rust test boundary, never an inherited environment option.

**Rationale**: This preserves D075, prevents a workspace from selecting an
arbitrary native executable, and keeps the CLI/direct-Zed configuration matrix
identical. Mock remains the default and a specifically requested but unready
embedded provider fails closed rather than returning mock output.

## Decision 6: Use user-scoped, content-addressed XDG storage

**Decision**: Provider-owned runtime state lives beneath a fixed
user-scoped XDG data root, not inside a checkout and not in a host-global
package/runtime directory. The logical structure is:

```text
embedded/
├── lifecycle.lock
├── objects/<sha256>/...
├── sets/<manifest-sha256>.json
├── state.json
└── staging/<operation-id>/...
```

`current`, `previous` and `candidate` are logical manifest-digest references
to immutable sets, not mutable directories or user-controlled symlinks.
Implementation must validate ownership, restrictive permissions, persistent
filesystem placement, regular files, link count and containment without
following unsafe links.

**Rationale**: A user scope works for both CLI and Zed, avoids duplicating a
large model per checkout, and prevents an untrusted workspace from choosing or
replacing native executable content. It is still non-privileged and is wholly
owned by the product lifecycle commands.

**Alternative rejected**: A workspace-local executable/model cache is easier
to inspect but gives repository contents too much influence over native code
loaded by the editor and duplicates artifacts across workspaces.

## Decision 7: Bind every acquisition to informed consent and manifest digest

**Decision**: Preparation is a two-step flow:

```text
make provider-embedded-disclose
make provider-embedded-prepare CONSENT=<manifest-sha256>
```

Disclosure shows the exact profile, sources, license conclusions, expected
transfer/installed budgets, destination scope and network behavior. A missing
or mismatched digest, rejection or cancellation produces no activation and no
change to `current`. Every reviewed update has a new digest and requires new
consent.

Downloads use only exact allowlisted HTTPS URLs, require status 200, disable
redirects, inherited proxies and retries, enforce byte limits, and validate
both compressed and expanded hashes. Each Zstandard attachment expands to one
fixed destination filename; no archive-controlled path is accepted.

**Rationale**: The digest binds the user's authorization to the artifact set
that will actually be acquired. It also prevents a disclosure for one version
from authorizing a later registry result.

## Decision 8: Make lifecycle atomic, offline and reversible

**Decision**: Provide disclosure, prepare, status, verify, update, rollback and
explicit removal commands. Prepare/update use a staging set and exclusive
lifecycle lock. Candidate identity, license/provenance completeness,
compatibility, hashes, offline smoke and budgets all pass before an atomic
state-file replacement moves `current` to `previous` and `candidate` to
`current`.

Normal translation, readiness, verify and rollback never acquire artifacts or
contact a registry. Rollback re-verifies `previous` and changes only the
logical state reference. Inference holds a shared lease; removal requires an
exclusive lease and returns a stable `BUSY` result rather than deleting files
in use. Unknown entries make cleanup fail closed. `make clean` never removes
provider state.

**Rationale**: Immutable objects plus a tiny atomic state file preserve the
known-good version across interrupted download, unpacking, validation,
promotion and cleanup.

## Decision 9: Fix evidence method and go/no-go budgets before implementation

**Decision**: Use 20 public synthetic fixtures: eight short technical texts,
four Unicode/punctuation cases, four 0.5-2 KiB cases, two near the 4 KiB
segment limit and two multi-segment Markdown cases. Run five warmups, five
repetitions per case and three deterministic rounds. Separate one-shot cold
start, warm provider, CLI end-to-end and LSP end-to-end; add a real Zed smoke.

The approved `warm_provider` class retains the one-shot architecture. It means
repeated one-shot launches after five warmups with a warm operating-system page
cache only; it does not mean a persistent provider, live model process, daemon
or FFI lifetime. The single `new_process` probe runs before those warmups and
the harness never clears the host page cache.

Record only case IDs, normalized outcomes and metrics. Never record input,
output, content-derived hashes, user/host identifiers or sensitive paths.

| Metric | Mandatory budget |
|---|---:|
| Full network transfer | <=64 MiB |
| Active installed set, runner included | <=128 MiB |
| Current + previous + candidate + staging | <=384 MiB |
| Free disk prerequisite | 512 MiB |
| Peak RSS | <=1 GiB |
| Inference threads | <=4 |
| One-shot cold readiness | <=10 s |
| Warm p95, short set | <=2 s |
| Warm p95, mixed matrix | <=5 s |
| Any complete provider request | <15 s |
| Local verify/decompression | <=60 s |
| Offline rollback to verified translation | <=5 min |

These are acceptance budgets, not measured results. The implementation cannot
promote the provider or close F012 until every mandatory measured field in
SC-004 contains reproducible evidence.

## Decision 10: Keep publication and redistribution separate

**Decision**: F012 may build the MPL native runner in the project environment
and may acquire exact official model resources into the user's approved scope.
It does not bundle the runner/model in a published extension, rehost the blobs,
or claim general redistribution rights.

The artifact manifest still records SPDX conclusions, source/license URLs,
notices, source-offer obligations and the reviewer result for every native
dependency and resource. Publication remains blocked until F009 reviews the
actual delivery artifact and satisfies all obligations.

Local activation requires an explicitly recorded human project-maintainer
approval bound to the exact manifest digest and local-acquisition scope.
Automated license/SBOM reports are evidence, not approval. Bundling,
redistribution or publication requires a separate explicit human F009 decision
bound to that delivery scope; absent or mismatched approval remains blocked.

**Rationale**: Runtime and model repositories provide materially better
license evidence than the previous Argos path, but artifact-level publication
rights and obligations must be concluded for the exact built and downloaded
set rather than inferred from a repository root.

**Sources**:

- <https://github.com/mozilla/firefox-translations-models>
- <https://www.mozilla.org/en-US/MPL/2.0/FAQ/>

## Candidate comparison

| Candidate | Decision | Main reason |
|---|---|---|
| Mozilla Translations/Bergamot + Firefox `en-es` base-memory | Prototype and gate | Best reviewed footprint, direct on-device pair, official dual hashes and maintained integration |
| CTranslate2 + Helsinki OPUS-MT base | Defer as fallback | Permissive licenses and active runtime, but converted-artifact provenance and footprint are worse and no first-party Rust boundary exists |
| OPUS-MT TC Big | Reject for normal path | Potential quality gain does not justify the much larger disk/RAM envelope without contrary benchmark evidence |
| Candle Marian | Defer | Pure Rust is attractive, but current `en-es` safetensor/tokenizer path relies on derived or third-party artifacts |
| Firefox Bergamot WASM v4 | Defer | Official and compact, but Emscripten/JS integration is not a standalone WASI runtime |
| Argos `en-es` | Reject | Upstream model license remains unresolved |
| NLLB-200 distilled 600M | Reject | Multi-gigabyte footprint and non-commercial restriction |
| MADLAD-400 3B | Reject | Resource envelope is incompatible with this product path |

## Resolved planning unknowns

- **Candidate**: one provisional Bergamot path, with explicit no-go gates.
- **Delivery**: project-built native runner plus consented exact resource
  acquisition; publication/bundling not claimed.
- **Runtime boundary**: killable one-shot child process.
- **Configuration**: one new mode, no new arbitrary env/path key.
- **Storage**: fixed user-scoped XDG content-addressed store.
- **Lifecycle**: digest-bound consent, immutable candidate/current/previous,
  offline verify/rollback and exact cleanup.
- **Resources**: fixed benchmark method and budgets; actual results are feature
  promotion evidence, not invented planning facts.

No `NEEDS CLARIFICATION` marker remains.
