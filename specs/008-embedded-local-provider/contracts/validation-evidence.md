# Contract: Embedded Provider Validation Evidence

**Feature**: `008-embedded-local-provider`
**Status**: Required before provider promotion and F012 closure

## Evidence layers

| Layer | Uses real runner/model | Network | Purpose |
|---|---|---|---|
| Rust unit/contract | No; controlled runner doubles | None | Selection, runner protocol, error, limits and redaction |
| Lifecycle contract | Controlled local artifacts/server doubles | Exact simulated acquisition only | Consent, hashing, atomic state, concurrency and cleanup |
| Native build/supply chain | Real pinned source/output | Fetch phase separated; build offline | Reproducibility, CPU baseline, ELF closure, SBOM/licenses |
| Real benchmark | Yes | Disabled after authorized preparation | Resource/latency/quality/locality go/no-go |
| CLI acceptance | Yes | Disabled | Real non-mock translation and non-mutation |
| Direct-Zed acceptance | Yes | Disabled | Same provider boundary and offline preview |

Controlled doubles are mandatory for negative coverage but cannot close
FR-022 or the provider-promotion gate.

## Synthetic fixture matrix

Use a versioned public 20-case corpus:

- 8 short technical-neutral English texts;
- 4 Unicode and punctuation cases;
- 4 texts from 0.5 to 2 KiB;
- 2 cases near the 4 KiB segment maximum;
- 2 multi-segment Markdown cases with protected code/links/structure.

No fixture contains a real secret, user document or sensitive identifier.
Quality assertions are deterministic enough to reject empty, unchanged,
wrong-language or structurally damaged output without claiming general
linguistic perfection. A reviewer records any explicit human quality gate.

## Benchmark protocol

1. Record only Fedora/`x86_64`, logical CPU count, RAM class, fixture-set
   version and manifest digest; omit hostname, username and paths.
2. Run five warmups.
3. Run every case five times in deterministic order for three independent
   rounds.
4. Record one pre-warmup `new_process` model-load probe, then run five warmups
   and record the complete matrix as `warm_provider`; add separate CLI and LSP
   end-to-end measurements and one real Zed smoke per acceptance scenario.
5. Use monotonic time and process-only CPU/RSS/thread observations through
   project/container tools; do not install host tools.
6. Do not clear page cache; define cold as a new runner process/model load and
   record that method.
7. Execute post-preparation translation in a network-disabled environment and
   require zero IP socket/connect attempts.

`warm_provider` means repeated one-shot processes after the fixed warmups with
a warm operating-system page cache only; it does not mean a persistent
provider, retained model process, daemon or FFI lifetime. The harness does not
clear page cache and never labels persistence as measured evidence.

## Mandatory budgets

| Metric | Gate |
|---|---:|
| Transfer | <=64 MiB |
| Active installed set | <=128 MiB |
| Full lifecycle storage | <=384 MiB |
| Peak RSS | <=1 GiB |
| Inference threads | <=4 |
| Cold readiness | <=10 s |
| Warm short p95 | <=2 s |
| Warm mixed p95 | <=5 s |
| Every request | <15 s |
| Local verify/decompress | <=60 s |
| Offline rollback to verified translation | <=5 min |

Unknown or exceeded mandatory values block promotion. A planning estimate or
upstream metadata cannot be recorded as an observed runtime result.

## Redacted record schema

Allowed fields:

```text
gate_id
manifest_digest
platform_class
fixture_set_version
case_id
surface
execution_class
round
repetition
elapsed_ms
process_cpu_ms
peak_rss_bytes
thread_peak
transfer_bytes
installed_bytes
network_attempts
locality
normalized_outcome
non_mutation
reviewer_status
```

Prohibited fields:

- source text, permitted segments or translations;
- content-derived hashes or raw model/provider output;
- environment values, proxy/credential state or sensitive URLs;
- workspace roots, XDG roots, executable/model paths;
- username, hostname, machine ID or other persistent host identity;
- raw native/downloader errors.

## Acceptance gates

### Supply chain

- exact source and all recursive dependencies pinned;
- offline reproducible native build from the project container;
- portable CPU flags and ELF runtime allowlist accepted;
- every runner/model/config artifact has size, hash, source, license conclusion
  and permitted delivery recorded;
- a human project-maintainer approval explicitly covers the exact manifest and
  local-acquisition scope; automated reports alone do not authorize activation;
- MPL notices/source obligations satisfied for the reviewed delivery;
- publication remains blocked unless a separate human F009 decision explicitly
  approves the exact bundling/redistribution scope.

### Lifecycle

- disclosure precedes acquisition and consent matches manifest digest;
- rejection/cancellation changes no active state;
- corrupt/incompatible/oversized candidates never activate;
- interrupted and concurrent operations preserve current;
- verify and rollback pass offline;
- removal changes only owned state and fails safely when busy/ambiguous.

### Product boundary

- Mock remains default;
- explicit embedded selection uses the same core gates in CLI and direct Zed;
- 20/20 real synthetic cases finish within 15 seconds;
- direct Zed labels the action offline before invocation;
- protected regions and source/editor content remain unchanged;
- external networking and remote confirmation are absent;
- logs, stderr, lifecycle output and committed evidence contain no prohibited
  content.

## Go/no-go conclusion

The final record must be exactly one of:

- `PROMOTED`: all mandatory gates have reproducible accepted evidence;
- `BLOCKED_<GATE>`: the embedded provider is not enabled/shipped, with a safe
  evidence reference and Mock/LibreTranslate preserved.

Partial technical success cannot be described as a supported or publishable
embedded provider.
