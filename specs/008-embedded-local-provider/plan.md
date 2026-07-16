# Implementation Plan: Embedded Local Provider

**Branch**: `008-embedded-local-provider` | **Date**: 2026-07-15 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from
`/specs/008-embedded-local-provider/spec.md`

## Summary

Deliver one no-account, no-key, no-service, no-provider-Docker
English-to-Spanish path through the existing CLI and direct Zed workflow,
without weakening Mock default, safety limits, Markdown protection, redaction
or non-mutation.

Phase 0 selects Mozilla Translations/Bergamot plus the exact Firefox
Translations `en -> es` resource set as the single candidate to implement and
gate. A verified project-owned native helper runs as a bounded one-shot child
process behind `translator-core::Provider`; exact model resources are acquired
only after manifest-digest consent into fixed user-scoped XDG storage. Normal
translation, readiness, verification and rollback are offline. Promotion is
blocked until reproducible native build, portable CPU, artifact-level license
and provenance, resource/latency, zero-network, lifecycle and real CLI/Zed
evidence all pass. If they do not, Mock and LibreTranslate remain supported and
no embedded/publication claim is made.

## Technical Context

**Language/Version**: Rust 2021 on Rust 1.96.1 for product and lifecycle
manager code; C++17 for the pinned native inference helper; POSIX/Bash only as
thin Make/script orchestration

**Primary Dependencies**: Existing `translator-core`, `translator-cli`,
`translator-lsp`, `translator-mcp` regression boundary, `serde`/`serde_json`,
existing blocking HTTP policy and Zed extension API 0.7.0; maintained
`mozilla/translations` inference source pinned initially at
`f31423c7c2c6ed8ae57d71a3d19a9db6f156060e` with all recursive dependencies
locked; exact Firefox Translations `en -> es` version-3.0 model resources;
project-owned Zstandard decoding and SHA-256 verification in the lifecycle
manager

**Storage**: Versioned artifact/source/license/budget manifests under
`ops/providers/embedded/`; fixed user-scoped XDG content-addressed runtime
store with immutable objects/sets, staging and atomic
candidate/current/previous digest references; no source text, translation,
credential, registry response or host identifier persistence

**Testing**: Rust unit/contract/integration tests through Make and the pinned
project container; native reproducible-build/CPU/ELF/license checks; lifecycle
contract tests with controlled artifacts and acquisition doubles; bounded
child-process tests; 20-case real offline benchmark plus real CLI and direct
Zed acceptance

**Target Platform**: Fedora KDE `x86_64`, CPU-only local inference, current
local Zed development extension; other OS/architectures explicitly unsupported
until separately locked and validated

**Project Type**: Multi-crate Rust CLI/MCP/LSP/editor extension plus one
project-built native one-shot inference helper and non-privileged artifact
lifecycle manager

**Performance Goals**: Model resources currently reviewed at 23.00 MiB
transfer / 34.88 MiB installed; full transfer <=64 MiB, active set including
runner <=128 MiB, lifecycle storage <=384 MiB, peak RSS <=1 GiB, <=4 inference
threads, cold readiness <=10 s, warm short p95 <=2 s, warm mixed p95 <=5 s,
and every provider request under the existing 15-second deadline

**Warm execution class**: `warm_provider` runs the same verified one-shot
process boundary after five deterministic warmups and means a warm
operating-system page cache only; it does not mean a persistent provider,
retained model process, daemon or FFI lifetime. `new_process` remains one
separate pre-warmup model-load probe, and no test clears the host page cache.

**Constraints**: Existing 20 KiB input, 4 KiB segment, 256-segment, 40 KiB
semantic output and 15-second timeout limits; Mock default; `en -> es` only;
no shell/arbitrary path/URL/args/env, inherited proxy, redirects, retries,
implicit acquisition, automatic update, host package/runtime/service install,
normal-runtime network, source/buffer/clipboard mutation, content logging,
unsafe symlink traversal or unreviewed redistribution; exact consent before
network acquisition

**Scale/Scope**: One provisional embedded provider profile, one platform, one
language direction, two acceptance surfaces, seven lifecycle commands, four
immutable logical states, 20 public synthetic benchmark cases and compatibility
regression for all existing provider modes/MCP

## Spec Kit Execution Record

Actual repository commands used for this cycle:

```bash
git switch -c 008-embedded-local-provider
.specify/scripts/bash/create-new-feature.sh --json --number 8 \
  --short-name embedded-local-provider \
  'Promote F012 from docs/feature-map.md: deliver the normal English-to-Spanish translation experience without an account, API key, remote service, or user-visible Docker lifecycle; select a distributable on-device runtime and model through reviewed license, provenance, integrity, resource, update, and packaging gates; integrate the selected local path with the CLI and direct Zed workflow while preserving all existing safety and privacy boundaries.'
.specify/scripts/bash/check-prerequisites.sh --json --paths-only
.specify/scripts/bash/setup-plan.sh --json
.specify/extensions/agent-context/scripts/bash/update-agent-context.sh \
  specs/008-embedded-local-provider/plan.md
.specify/scripts/bash/check-prerequisites.sh --json
.specify/scripts/bash/check-prerequisites.sh --json --require-tasks \
  --include-tasks
```

The clarification prerequisite was executed exactly once before the structured
ambiguity scan. No critical ambiguity required a user question: scope, actor,
consent, lifecycle, limits, failure behavior, privacy, evidence and no-go
behavior were already testable. Phase 0 was explicitly assigned the technical
candidate comparison. Two consistency corrections were encoded in the spec:
delivery may differ by artifact class, and planning fixes measurement method
and budgets while real measurements remain mandatory before promotion/closure.

The optional `after_plan` agent-context hook updated the managed Spec Kit block
in `AGENTS.md`. The checklist prerequisite reported `research.md`,
`data-model.md`, `contracts/` and `quickstart.md` as available design inputs;
the generated checklist reviews artifact lifecycle/supply chain and the
embedded offline/privacy boundary. The original planning-only pass stopped
before task generation. The subsequent continuation created `tasks.md`, ran
analysis, implemented every autonomous controlled task with the required Rust
practice/TDD workflow, and ran two convergence audits. Those audits identified
and closed the `warm_provider` definition and this stale gate record. Six tasks
remain intentionally open because they require human artifact/license approval
and the real/manual evidence that approval gates.

## Spec Kit Gate Status

| Gate | Status | Evidence |
|---|---|---|
| `speckit-specify` | Complete | `spec.md` plus requirements checklist 16/16 |
| `speckit-clarify` | Complete, 0 questions | Prerequisite executed once; structured scan found no critical ambiguity |
| `speckit-plan` | Complete | Research, data model, four contracts, quickstart, pre/post constitution PASS and agent context updated |
| `speckit-checklist` | Complete/generated | 48-item artifact-safety requirements checklist with traceability on every item |
| `speckit-tasks` | Complete | Dependency-ordered `tasks.md` exists; 83 total tasks after append-only convergence |
| `speckit-analyze` | Complete | Post-task cross-artifact analysis passed; a final read-only rerun is part of closeout evidence after T083 |
| `speckit-implement` | Complete to external gates | 77/83 tasks complete; the six open US4 tasks are the human license/approval and gated real/manual evidence tasks |
| `speckit-converge` | Complete | Prerequisite passed; append-only passes produced T082/T083 and both findings were implemented |

## Constitution Check

### Before Phase 0: PASS

- **Safety-first translation**: The feature adds no edit, clipboard or source
  write. Existing file, secret, ambiguity, Markdown and response gates remain
  ahead of the provider boundary.
- **Offline-first provider boundary**: Mock remains default. Embedded is
  explicit, cannot accept remote configuration, and performs inference only
  from a previously verified local set; preparation is a separately consented
  operation.
- **Test-first development**: Configuration, process, lifecycle, supply-chain,
  timeout, redaction and non-mutation tests must fail before production code.
  Controlled doubles are supplemented by mandatory real offline evidence.
- **Explicit contracts and limits**: Existing request/result/error and
  20 KiB / 4 KiB / 256 / 40 KiB / 15 s limits remain authoritative. Phase 1
  adds versioned selection, manifest, lifecycle, wire and evidence contracts.
- **Minimal host footprint**: Native/Rust builds remain in the project
  container. Prepared artifacts are non-privileged and user-scoped; no global
  runtime, package, service, database or provider container is installed.

### After Phase 1 design: PASS

The data model and four contracts preserve every principle. Native code is
kept behind a killable, bounded child process instead of weakening Rust's safe
core. The XDG store is fixed and product-owned rather than workspace-selected.
Unresolved artifact publication rights do not receive an exception: local
acquisition may proceed only after a complete manifest review, while bundling
and F009 publication remain blocked.

## Phase 0: Research Decisions

Complete in [research.md](./research.md):

1. prototype only Mozilla Translations/Bergamot with the official exact
   Firefox `en -> es` resource set;
2. pin records, dual hashes, source and recursive native dependencies rather
   than follow `latest`;
3. run inference in a bounded, killable one-shot native helper;
4. keep inference and implicit preparation out of the Zed extension WASM;
5. add `embedded_local` without a new arbitrary path/env setting;
6. use fixed user-scoped XDG content-addressed storage;
7. bind consent to the complete manifest digest;
8. use atomic immutable lifecycle sets with offline verify/rollback;
9. fix a 20-case benchmark protocol and mandatory resource/latency budgets;
10. separate local acquisition from any redistribution/publication claim.

All planning unknowns are resolved. Real native measurements and
artifact-level review are explicit implementation promotion gates, not missing
planning decisions.

The implementation convergence review later clarified decision 9 without
changing the one-shot architecture: warm measurements reuse only the
operating-system page cache after five warmups. They never reuse a live
provider/model process.

## Phase 1: Design and Contracts

- [data-model.md](./data-model.md) defines the profile, runner/model manifests,
  immutable artifact sets, atomic installation state, provider selection,
  ephemeral invocation and redacted evidence.
- [embedded-provider-selection.md](./contracts/embedded-provider-selection.md)
  fixes the exact four-key configuration matrix, locality, resolution and
  fail-closed behavior.
- [artifact-lifecycle.md](./contracts/artifact-lifecycle.md) fixes disclosure,
  digest consent, exact acquisition, immutable promotion, concurrency,
  offline rollback and exact removal.
- [runner-wire.md](./contracts/runner-wire.md) fixes the one-shot JSON protocol,
  process construction, bounded pipes, deadline, kill/reap and error mapping.
- [validation-evidence.md](./contracts/validation-evidence.md) separates
  controlled negative coverage from mandatory real-model offline acceptance.
- [quickstart.md](./quickstart.md) documents planned preparation and both
  product surfaces while making the unimplemented/planning state explicit.

## Implementation Strategy

### 1. Tests first: configuration, state and process runner

Extend exhaustive provider configuration tests with `embedded_local`, proving
all conflicting URL/key/remote fields fail and Mock stays default. Define Rust
manifest/state types and test unknown schema, corrupt digest, unsafe storage
and missing current before filesystem/process production code.

Add an injectable process-runner boundary in `translator-core`. Tests first
prove exact executable/cwd/env, one batch, bounded concurrent stdout/stderr,
all malformed-response cases, no retry/fallback, deadline kill/reap and
redacted errors. The production provider stores only validated immutable
references and remains compatible with the synchronous `Provider` trait.

### 2. Reproducible native helper and supply-chain lock

Create a minimal C++17 `translator-embedded-runtime` wrapper around the exact
pinned Mozilla inference source. Fetching source and project building are
separate: exact source/dependency objects are verified, then the native build
runs network-disabled in the existing project container with reviewed portable
`x86_64` flags. Tests reject upstream `BUILD_ARCH=native`, unexpected ELF
dependencies, missing license/SBOM/source-offer data or unreproducible output.

The runner implements only the versioned stdin/stdout translation contract. It
contains no HTTP/update code, does not log content and cannot listen as a
service.

### 3. Artifact lifecycle manager

Implement a small Rust lifecycle manager invoked by Make targets so path,
hashing, decompression, state locking and HTTP policy are testable without
privileged or global host tooling. Disclosure is read-only. Prepare/update
require exact manifest consent, acquire exact allowlisted attachments into
staging, validate dual hashes/sizes/licenses/compatibility, execute offline
smoke/resource gates and atomically promote. Status/verify/rollback are
offline. Removal requires the exact confirmation token and exclusive lease;
generic clean preserves state.

The manager and inference runner are separate binaries. Normal translation
cannot invoke downloader/update code.

### 4. Shared core, CLI and direct-Zed integration

Construct `EmbeddedProcessProvider` from the shared parsed selection in CLI and
LSP. Preserve one ordered segment batch and all existing input/output gates.
LSP locality marks embedded as offline before execution and keeps the existing
read-only hover/no-edit behavior. The Zed extension forwards only
`TRANSLATOR_PROVIDER=embedded_local`; it never accepts artifact paths or
silently prepares from `language_server_command`.

MCP receives no new product flow. Its existing build/tests remain regression
coverage for shared enum/trait behavior.

### 5. Promotion evidence and documentation

Run the fixed synthetic matrix after authorized preparation. Record real
transfer/installed size, native binary identity, peak RSS/CPU/threads,
cold/warm latency, zero network attempts, CLI and direct-Zed outcomes,
non-mutation, invalid update, offline rollback and exact cleanup. Review
license/provenance and notices for the exact artifact set.

Only all-passing evidence enables the provider and updates documentation from
provisional to supported. Otherwise record the blocking gate and retain Mock
and LibreTranslate. F009 publication remains separate.

## Project Structure

### Documentation (this feature)

```text
specs/008-embedded-local-provider/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── artifact-lifecycle.md
│   ├── embedded-provider-selection.md
│   ├── runner-wire.md
│   └── validation-evidence.md
├── checklists/
│   ├── requirements.md
│   └── embedded-artifact-safety.md
└── tasks.md                         # later /speckit-tasks output; not created now
```

### Source Code (repository root)

```text
crates/
├── translator-core/
│   └── src/
│       ├── embedded_provider.rs       # provider/process boundary
│       ├── provider.rs                # shared enum/trait integration
│       └── provider_config.rs         # exact four-key matrix
├── translator-cli/                    # existing surface
├── translator-lsp/                    # existing direct-Zed surface/locality
├── translator-mcp/                    # compatibility regression only
└── translator-provider-manager/       # disclosure/acquisition/lifecycle CLI

native/
└── translator-embedded-runtime/       # minimal C++17 wire wrapper/build files

ops/providers/embedded/
├── provider.lock.json                 # full reviewed identity/delivery lock
├── source.lock.json                   # source/submodule/build/ELF lock
├── licenses/                          # notices/conclusions/source obligations
└── README.md                          # reviewed lifecycle boundary

scripts/
├── provider-embedded.sh               # thin Make-facing orchestration
└── tests/                              # lifecycle/build/doc contract tests

tests/fixtures/embedded/
├── manifests/                         # controlled good/bad manifests
├── runner-doubles/                    # bounded protocol/error fixtures
└── synthetic-corpus.json              # public 20-case acceptance matrix

zed-extension/                         # existing allowlisted launch/config
specs/008-embedded-local-provider/      # operational requirements/design
```

**Structure Decision**: Keep provider semantics in the existing Rust core and
surfaces; isolate native inference in one process executable; isolate network
and mutable artifact lifecycle in a separate Rust manager. Version only source,
identity, license and policy metadata. Source fetch/build staging and XDG
runtime objects remain ignored and never become arbitrary workspace inputs.

## TDD and Verification Order

1. provider configuration/state/path negative tests;
2. process wire, bounded I/O, timeout kill/reap and error tests;
3. lifecycle consent/acquisition/integrity/atomicity/concurrency/cleanup tests;
4. native source lock, reproducible build, CPU/ELF/license checks;
5. core/CLI/LSP/Zed integration and all existing compatibility suites;
6. real 20-case offline benchmark and resource evidence;
7. real CLI and direct-Zed non-mutation acceptance;
8. invalid update, offline rollback, redaction and complete removal;
9. full project formatting, lint, tests and dependency/license gates through
   Make/Docker.

No implementation task may mark real acceptance complete using only a runner,
HTTP or filesystem double.

## Complexity Tracking

No constitutional violation is required. Native C++ is a bounded project
dependency, not a host-global runtime or an exception to safe Rust product
boundaries. Any later move to in-process FFI, persistent daemon, automatic
download, arbitrary path configuration or broader redistribution requires a
new architecture/constitution review rather than being treated as an
implementation detail.
