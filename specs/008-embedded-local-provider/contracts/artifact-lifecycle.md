# Contract: Embedded Artifact Lifecycle

**Feature**: `008-embedded-local-provider`
**Status**: Planning contract

## Owned scope

The lifecycle owns one fixed user-scoped XDG product root with restrictive
permissions. It does not own the repository, arbitrary XDG entries, Zed's
global data, Docker resources, system packages, services or user documents.

```text
embedded/
├── lifecycle.lock
├── objects/<sha256>/...
├── sets/<manifest-sha256>.json
├── state.json
└── staging/<operation-id>/...
```

All mutable operations reject volatile filesystems, unsafe owner/mode, links,
unexpected hard links, traversal, unknown state schemas and paths that escape
the opened root. Logical slots are digests in `state.json`, never symlinks.

## Versioned commands

| Command | Network | Mutation | Contract |
|---|---|---|---|
| `make provider-embedded-disclose` | No | No | Print redacted reviewed identity, license conclusions, sizes, scope and exact consent digest |
| `make provider-embedded-prepare CONSENT=<digest>` | Exact acquisition only | Stage and promote only after all gates | First preparation |
| `make provider-embedded-status` | No | No | Safe profile/readiness/version/resource metadata only |
| `make provider-embedded-verify` | No | No | Rehash/revalidate current set and offline synthetic smoke |
| `make provider-embedded-update CONSENT=<digest>` | Exact acquisition only | Candidate then atomic promotion | Explicit reviewed update; never registry discovery |
| `make provider-embedded-rollback` | No | Atomic state change only | Reverify previous and restore it |
| `make provider-embedded-clean CONFIRM=remove-embedded-provider-data` | No | Remove only enumerated owned objects/state | Fail `BUSY` or on unknown entries; never part of `make clean` |

These commands are planned interfaces until implementation tasks create and
test them. No command may request administrator privileges or modify host
packages/services.

## Disclosure and consent

Disclosure precedes network or state mutation and includes:

- profile/language/platform;
- exact upstream sources and reviewed version identities;
- artifact-level license/provenance conclusions and publication limitations;
- expected download, active-set and worst-case lifecycle storage budgets;
- fixed user scope and the fact that normal translation is offline;
- the exact full-manifest SHA-256 used as `CONSENT`.

Missing, malformed or mismatched consent performs no acquisition and no
activation. Consent to one digest never authorizes another. Update requires a
fresh digest even when some content-addressed objects already exist.

The manager must recompute the consent digest from the domain-separated,
fixed-order typed manifest payload and reject a recorded digest mismatch before
approval or acquisition. The payload covers schema/profile/languages/platform,
review/publication states, runner, ordered artifacts and budgets; it excludes
only the digest and approval records to avoid self-reference.

## Acquisition rules

- exact manifest-pinned HTTPS URLs only;
- no redirects, inherited proxy, mirror selection, registry lookup or retry;
- status 200, exact content length and per-artifact byte ceiling;
- compressed SHA-256 before expansion and installed SHA-256/size after it;
- one fixed output basename per Zstandard attachment, with no archive paths;
- partial files remain in the operation staging directory and are never
  executable/current;
- runner/model/config/language/platform/license compatibility is checked even
  when all hashes match.

The inference runner contains no downloader or updater and normal translation
cannot enter this acquisition path.

## Promotion

Under the exclusive lifecycle lock:

1. create a new root-contained staging directory;
2. acquire or materialize every exact object;
3. validate manifest, identity, size, hashes, permissions, compatibility,
   licenses and CPU baseline;
4. execute the public synthetic smoke with external network disabled;
5. enforce the active-set resource budgets;
6. fsync finalized objects and the immutable set record;
7. atomically replace `state.json`, moving old `current` to `previous` and
   candidate to `current`;
8. report safe normalized metadata only.

A cancellation, crash or failed check before step 7 leaves the former current
reference unchanged. An update may not delete `previous` before the new
current has passed post-promotion verification.

## Concurrency and leases

- prepare, update, rollback and removal serialize on `lifecycle.lock`;
- readers resolve state under a short shared state lock;
- inference holds a shared lease on its immutable set;
- update may stage candidate while current serves translations, but promotion
  uses the exclusive state transition;
- removal requires an exclusive lease and returns stable `BUSY` without
  deleting when a CLI/LSP process is using the set;
- additional inference waits only within the same 15-second request budget or
  fails with a stable busy/timeout result.

## Rollback

Rollback performs no network access. It validates `previous` against its exact
manifest, runs the offline synthetic smoke, and atomically swaps logical
references. If previous is missing or invalid, rollback fails without changing
current or deleting either set.

## Removal

Removal requires the exact confirmation token and an exclusive lease. It
enumerates only state-referenced sets, manifest-listed objects, known staging
records and lifecycle metadata beneath the validated root. It refuses to
delete through links or when unknown entries make complete ownership
unprovable. Successful complete removal leaves no provider-owned active state;
Mock and the repository remain unchanged.

## Resource gates

| Resource | Budget |
|---|---:|
| Network transfer | <=64 MiB |
| Active set including runner | <=128 MiB |
| Current + previous + candidate + staging | <=384 MiB |
| Required free disk before preparation/update | 512 MiB |
| Local verify/decompression | <=60 s |
| Offline rollback to verified translation | <=5 min |

Measured results are required before promotion and feature closure.

## Required failure tests

- refusal/mismatched consent produces zero download and state change;
- unavailable network, non-200, redirect, proxy, truncated/oversized/corrupt
  attachment and decompression failure;
- valid hash with wrong language, runtime version, platform, config or license;
- interrupted prepare/update at every boundary and atomic recovery;
- concurrent lifecycle operations and removal while inference holds a lease;
- current remains usable after invalid candidate and failed promotion;
- offline verify/rollback, missing/corrupt previous and post-promotion recovery;
- unsafe owner/mode/filesystem/link/hard-link/traversal and unknown cleanup file;
- exact cleanup scope and proof that generic `make clean` preserves state.
