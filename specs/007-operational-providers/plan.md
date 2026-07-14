# Implementation Plan: Operational Real Providers

**Branch**: `007-operational-providers` | **Date**: 2026-07-14 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from
`/specs/007-operational-providers/spec.md`

## Summary

Operationalize exactly two reviewed real translation paths while preserving
the existing safe core and direct Zed workflow:

- local/offline: LibreTranslate 1.9.6 CPU image pinned by digest, provisioned
  by project commands into candidate/current/previous Docker storage, isolated
  on an internal network behind a fixed project relay published only on
  `127.0.0.1:5000`, and proven to translate after external egress is disabled;
- remote/online: Azure AI Translator Text v3, global F0 resource, fixed HTTPS
  host/path, API key resolved through an existing safe environment reference,
  no redirect/proxy/retry, and fresh confirmation before every request.

`MockProvider` remains the deterministic default. Both paths continue through
`translator-core`, its segmentation/limits/secret gate/response validation,
and the same CLI and direct Zed surfaces. Automated tests use controlled
doubles; feature completion additionally requires redacted manual evidence
against both real services.

MCP/Agent Panel remains a compatibility bridge under D065/D072. F011 adds no
provider-specific MCP product flow or real-service acceptance surface; existing
MCP regression suites remain mandatory only to detect compatibility breakage.

## Technical Context

**Language/Version**: Rust 2021 on Rust 1.96.1 for product code; POSIX/Bash
project scripts; Docker Compose configuration

**Primary Dependencies**: Existing `ureq` 3.3 blocking HTTP client and
`serde`/`serde_json`; existing `translator-core`, `translator-cli`,
`translator-lsp`, Zed extension API 0.7.0; Docker Engine 29.5.3 and Compose
5.1.4 already present on the reviewed workstation

**Storage**: Versioned provider lock/Compose files; project-namespaced Docker
candidate/current/previous volumes; ignored safe runtime metadata under
`provider-cache/libretranslate/`; no database, source content, translation, or
credential persistence

**Testing**: Rust unit/contract/integration tests through Make/Docker; shell
contract tests for lifecycle and documentation; controlled HTTP/process
doubles for automation; four real-service manual acceptance runs through CLI
and direct Zed

**Target Platform**: Fedora KDE `x86_64`, Docker/Compose project isolation,
CPU-only local provider, current local Zed development extension

**Project Type**: Multi-crate Rust CLI/MCP/LSP/editor extension with a
project-scoped provider container and hardened loopback relay

**Performance Goals**: Prepared local provider model-ready in <=120 seconds on
the reviewed workstation; every provider invocation within the existing
15-second budget; Azure request remains within 256 elements, 20 KiB input and
40 KiB validated output

**Constraints**: 20 KiB input, 4 KiB/segment, 256 segments, 40 KiB output,
15-second timeout; local service capped at 4 CPUs/4 GiB RAM with 4 GiB free disk
prerequisite; no normal-runtime egress; remote exact-host HTTPS; mock default;
no redirects, inherited proxy, retries, arbitrary endpoints, host installs,
source/buffer mutation, recorded content, or model redistribution

**Scale/Scope**: One English-to-Spanish local provider, one
English-to-Spanish remote provider, three provider modes, two user-facing
surfaces, eight local lifecycle commands, four real success runs plus negative
privacy/recovery matrix

## Spec Kit Execution Record

Actual repository commands used for this planning cycle:

```bash
git switch -c 007-operational-providers
.specify/scripts/bash/create-new-feature.sh --json --number 7 \
  --short-name operational-providers \
  'Promote F011 from docs/feature-map.md: configure and validate one real local/offline English-to-Spanish provider and one real remote/online provider from both the CLI and the direct Zed workflow. Keep MockProvider as the deterministic default; isolate the local provider inside the project with documented lifecycle, persistence, verification, updates, and rollback; require HTTPS allowlisting, secrets outside the repository, and per-request confirmation for remote use; preserve existing limits, segmentation, Markdown protection, normalized errors, non-mutation, secret blocking, and redaction; require redacted manual evidence against both real services; exclude publication, paid-only providers, global host installation, and buffer mutation.'
.specify/scripts/bash/check-prerequisites.sh --json --paths-only
SPECIFY_FEATURE=007-operational-providers \
  .specify/scripts/bash/setup-plan.sh --json
SPECIFY_FEATURE=007-operational-providers \
  .specify/scripts/bash/check-prerequisites.sh --json
```

Clarification closed one material question: the remote path may require a free
account and API key, but not a paid subscription. The provider/privacy domain
checklist was then added because this feature crosses external service,
credential, artifact, lifecycle, and evidence boundaries.

## Constitution Check

### Before Phase 0: PASS

- **Safety-first translation**: No buffer/file/clipboard mutation is added;
  provider responses remain ephemeral preview/CLI results and ambiguity fails
  closed.
- **Offline-first provider boundary**: Mock is still default. Local runtime has
  no external egress after explicit preparation. Azure is disabled unless
  explicitly configured and separately confirmed per invocation.
- **Test-first development**: Tasks must begin with failing provider adapter,
  target, lifecycle, privacy, error, non-mutation and redaction tests. Real
  service checks supplement rather than replace controlled automated tests.
- **Explicit contracts and limits**: Existing request/result/error/segment
  contracts and 20 KiB / 4 KiB / 256 / 40 KiB / 15 s limits remain unchanged;
  this plan adds provider-selection, lifecycle, Azure and evidence contracts.
- **Minimal host footprint**: Provider/runtime dependencies stay in a
  project-namespaced Compose environment. No host runtime/service install,
  privileged script, real `.env`, or versioned credential is planned.

### After Phase 1 design: PASS

The data model and contracts preserve all five principles. The unresolved
Argos `en -> es` model license is handled conservatively: user-local verified
acquisition is allowed for F011, while vendoring, derivative-image bundling,
redistribution and F009 publication remain blocked until licensing is clear.
This is an external gate, not a constitutional exception.

## Phase 0: Research Decisions

Complete in [research.md](./research.md):

1. pin LibreTranslate 1.9.6 by multi-architecture digest;
2. separate online artifact preparation from no-egress normal runtime;
3. do not redistribute the Argos model while its license is unresolved;
4. use candidate/current/previous slots for update and rollback;
5. use Azure Translator v3 global F0 for remote acceptance;
6. add `azure_translator` without adding provider settings or arbitrary URLs;
7. retain stable normalized errors and never retry remote requests;
8. set measurable resource budgets and upstream change-control gates.

All `NEEDS CLARIFICATION` markers are resolved.

## Phase 1: Design and Contracts

- [data-model.md](./data-model.md) defines provider profiles, lock metadata,
  lifecycle state, remote configuration, ephemeral invocation, and redacted
  validation records.
- [provider-selection.md](./contracts/provider-selection.md) fixes the exact
  configuration matrix, locality, target validation, gate order, and errors.
- [local-provider-lifecycle.md](./contracts/local-provider-lifecycle.md)
  specifies preparation, offline operation, update, rollback, cleanup, and
  resource budgets.
- [azure-translator.md](./contracts/azure-translator.md) fixes request,
  credential, privacy, transport, and response rules.
- [validation-evidence.md](./contracts/validation-evidence.md) separates
  controlled automated tests from mandatory real-service evidence.
- [quickstart.md](./quickstart.md) defines the post-implementation acceptance
  path and account/privacy warnings.
- [manual-validation.md](./manual-validation.md) is the redacted evidence
  template; it remains explicitly unexecuted at the planning gate.

## Implementation Strategy

### 1. Tests first and shared configuration

Extend provider configuration tests before production changes. Add
`azure_translator` as the only new mode; keep all four established environment
keys and make mode-specific validation exhaustive. Refactor LSP locality so
startup constructs both label and `ProviderSelection` from one parsed
configuration, eliminating drift.

### 2. Project-scoped LibreTranslate lifecycle

Add an operational directory with pinned Compose and provider lock metadata.
A single lifecycle script owns state transitions and is exercised through
Make targets. It uses a fixed Compose project name, exact image digest,
provider-only internal network, fixed hardened loopback relay, resource caps,
named lifecycle slots, safe readiness probes, no automatic updates,
`pull_policy: never` in normal runtime, and an exact destructive confirmation
token.

Preparation/update may acquire artifacts only from the lock, verify observed
hashes, and populate candidate. Promotion is transactional at the state level;
rollback is offline and uses previous. The model blobs and runtime metadata
remain ignored and are never packaged.

### 3. Azure adapter behind `Provider`

Create a focused `AzureTranslatorProvider` in `translator-core`. Reuse the
blocking `ureq` boundary with global 15-second timeout and proxy disabled; set
redirects to zero. Construct the endpoint internally, batch only validated
segments, add the key header from the referenced environment variable, and
parse/bound/cardinality-check the response. No request/response structs derive
or emit unsafe `Debug` output. Keep the existing public `ErrorCode` enum.
The internal request must retain the existing technical-neutral tone and
formatting intent. Because the reviewed Azure request exposes no tone/format
field used by this project, the adapter validates those invariants before
contact and sends no invented parameter, header, or metadata.

### 4. CLI and direct Zed operational validation

Add `translator-cli-release` and the focused automatic gate. Update safe Zed
launch validation for the new mode while keeping the existing allowlist and
ensuring the extension emits only the key-reference name. The actual key value
must already exist in the parent Zed process environment and is resolved by the
provider; `settings.rs`, `binary.env`, launch arguments, and the generated
profile never contain that value.
Preserve the direct workflow's request-specific `showMessageRequest`, secret
gate and read-only hover.

MCP is not extended in this step. Its existing tests run as compatibility
regressions against the shared core boundary, not as F011 acceptance evidence.

### 5. Evidence and documentation

Automatic tests never call real providers. Once implementation gates pass, a
reviewer performs the four real-service runs, offline and rollback checks, and
remote negative matrix using public synthetic fixtures. Only normalized safe
fields enter the manual record. F009 remains blocked until F011 evidence is
complete and the model redistribution license is resolved or publication
chooses a legally reviewed alternative.

## Project Structure

### Documentation (this feature)

```text
specs/007-operational-providers/
├── spec.md
├── plan.md
├── tasks.md
├── research.md
├── data-model.md
├── quickstart.md
├── manual-validation.md
├── contracts/
│   ├── provider-selection.md
│   ├── local-provider-lifecycle.md
│   ├── azure-translator.md
│   └── validation-evidence.md
└── checklists/
    ├── requirements.md
    └── provider-operations-privacy.md
```

`tasks.md` was generated at the approved `speckit-tasks` gate and is the
dependency-ordered implementation source for this feature.

### Planned source changes (repository root)

```text
.dockerignore
.gitignore
Makefile
ops/
└── providers/
    └── libretranslate/
        ├── compose.yaml
        └── provider.lock
scripts/
└── providers/
    └── libretranslate.sh
provider-cache/                 # ignored; safe local state, no content/secrets
crates/
├── translator-core/
│   ├── src/
│   │   ├── azure_translator.rs
│   │   ├── provider.rs
│   │   ├── provider_config.rs
│   │   └── lib.rs
│   └── tests/
│       ├── azure_translator_provider.rs
│       ├── azure_translator_failures.rs
│       ├── operational_provider_configuration.rs
│       └── operational_provider_redaction.rs
├── translator-cli/
│   └── tests/
│       └── cli_operational_providers.rs
└── translator-lsp/
    ├── src/state.rs
    └── tests/
        └── operational_provider_locality.rs
zed-extension/
├── src/settings.rs
└── tests/direct_lsp.rs
tests/
├── fixtures/
│   └── operational-providers/  # public synthetic inputs/expected metadata
└── integration/
    ├── operational_provider_make_targets.sh
    ├── provider_local_lifecycle.sh
    ├── provider_local_offline.sh
    ├── provider_local_rollback.sh
    ├── provider_local_update_cleanup.sh
    ├── operational_provider_evidence_contract.sh
    └── operational_provider_contract.sh
```

**Structure Decision**: Keep provider execution in the existing core and
surfaces; add no new Rust crate. Operational service assets are isolated under
`ops/providers/libretranslate/`, while a single script owns Docker lifecycle
and the Makefile remains the public interface. This avoids host services,
duplicate provider logic, and state transitions embedded across Make recipes.

## TDD Boundaries and Verification Order

Implementation work must follow this order:

1. failing project-interface plus configuration/locality/target tests;
2. minimal shared configuration and safe-locality implementation;
3. failing local lifecycle/integrity/offline/CLI tests;
4. minimal Compose lock and local lifecycle implementation;
5. failing Azure payload/transport/consent/redaction tests;
6. minimal Azure adapter and CLI/LSP/Zed integration;
7. failing update/rollback/cleanup and cross-surface failure tests;
8. minimal recovery, error mapping and redaction implementation;
9. complete automatic regression, formatting, lint and supply-chain gates;
10. only then execute real local and Azure CLI/direct-Zed validation, including
   no-egress, rollback and negative consent/secret cases;
11. redaction audit and final manual record review.

Expected automatic verification interface after implementation:

```bash
make test-operational-providers
make test-real-provider-config
make test-direct-zed-translation
make test-zed-extension
make test
make fmt
make clippy
make deny
```

No real provider, real credential, or translated content is used by automatic
tests.

## Supply Chain, Privacy, and Failure Gates

- Image tag and manifest digest, model index revision and project-observed
  hashes are versioned; mutable `latest`/index resolution is forbidden at
  runtime.
- Model licensing remains explicitly unresolved; no redistribution or bundling.
- Local normal operation and rollback have no external egress.
- Azure host/path/API version are constants; a certificate failure, redirect,
  custom URL or proxy path fails closed.
- Azure F0/account/privacy/terms are reviewed before each real validation; any
  material change blocks remote acceptance without disabling mock/local.
- Raw response/provider debug detail is never user-visible or evidence.
- A failed update cannot mutate current; a failed rollback cannot destroy the
  last known-good reference.
- Ordinary cleanup cannot remove persistent provider slots.

## Gate Status after Automatic Implementation

| Gate | Status | Evidence / prerequisite |
|---|---|---|
| `speckit-specify` | complete | `spec.md` plus 16/16 requirements checklist |
| `speckit-clarify` | complete | prerequisite resolved feature; 2/2 material answers encoded in spec, including MCP/Agent compatibility-only scope |
| `speckit-checklist` | complete | provider operations/privacy checklist evaluated against design artifacts |
| `speckit-plan` | complete | `setup-plan`, research, data model, contracts, quickstart, evidence template, constitution re-check |
| `speckit-tasks` | complete | `setup-tasks --json` resolved the active feature; `tasks.md` contains 57 dependency-ordered TDD/security tasks with 22 explicit parallel opportunities |
| `speckit-analyze` | complete after remediation | non-destructive rerun resolves 39/39 requirements with no CRITICAL/HIGH finding; TDD interface checks, final-only real validation, tone/format, key-reference, clean-checkout, budget, evidence and MCP scope corrections are encoded |
| `speckit-implement` | automatic scope complete through convergence T062; T056 partial | TDD implementation and all automatic Rust/shell/security/supply-chain gates pass; approved real local CLI/direct Zed/offline/idempotency/failed-update/rollback and pre-contact negatives pass, while Azure real, clean and remaining clean-checkout evidence remain pending |
| `speckit-converge` | clean after remediation | Phase 8 added and completed T058-T062 for probe validation, atomic physical-slot rotation, prior-image rollback, fail-safe cleanup and safe state/override handling; follow-up assessment found no uncovered buildable work, while existing T056 remains the explicit external acceptance gate |

T054 passed `workspace-storage-check`, `test-operational-providers`,
`test-real-provider-config`, `test-direct-zed-translation`,
`test-zed-extension`, the full `test` suite, `fmt`, and `clippy`. T055 passed
`cargo-deny` for both workspaces with only allowed transitive duplicate-version
warnings. Manual review found no mutable provider image/index reference or
Cargo dependency drift; model redistribution remains
`forbidden-until-resolved`. Subsequent T056 execution used the real pinned
local provider without credentials; no Azure credential was present or used.

## Complexity Tracking

No constitution violation requires an exception. The two real provider
boundaries are required by FR-001 and remain behind the existing `Provider`
trait. Docker lifecycle state is the minimum reversible structure needed for
offline update/rollback. The unresolved model license is tracked as a
publication/redistribution gate rather than weakened or silently accepted.
