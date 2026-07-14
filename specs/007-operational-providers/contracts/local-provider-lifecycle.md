# Contract: Local Provider Lifecycle

## Reviewed profile

```text
Provider: LibreTranslate
Release: 1.9.6
Image: libretranslate/libretranslate:v1.9.6
Digest: sha256:1de2d7056bb8ad607a412f4563d9abe324ff632b43b5be9428bcc8e213aebb32
Endpoint: http://127.0.0.1:5000
Languages loaded: en, es
Runtime network: provider-only internal network plus fixed loopback relay
Persistent path in container: /home/libretranslate/.local
```

The project does not redistribute model blobs. Preparation downloads reviewed
artifacts into project-namespaced Docker volumes after user initiation.

## Commands

Implementation must expose these project interfaces:

```text
make provider-local-prepare
make provider-local-start
make provider-local-status
make provider-local-verify
make provider-local-stop
make provider-local-update
make provider-local-rollback
make provider-local-clean CONFIRM=remove-provider-data
```

The Makefile delegates lifecycle logic to a versioned script under
`scripts/providers/`; it must not reproduce provider state transitions in
shell fragments across multiple targets.

## Prepare

`provider-local-prepare` is explicitly online and must:

1. read only versioned provider lock metadata;
2. pull the exact tag+digest and verify the resulting manifest identity;
3. provision only the `candidate` slot;
4. download model artifacts only from lock-approved sources;
5. verify every locally observed SHA-256 before installation;
6. start the candidate behind the project-scoped loopback-only relay;
7. require `/health` plus a fixed public synthetic `en -> es` translation;
8. restart the candidate with external egress unavailable and repeat both
   probes;
9. promote to `current` only after all checks pass.

Interrupted, corrupt, mismatched, unlicensed-for-redistribution, or incomplete
preparation leaves `current` unchanged and returns safe actionable status.

## Normal runtime

`provider-local-start` must use `pull_policy: never`, the prepared active slot,
an internal Docker network, automatic model updates disabled, and no web
UI/file translation. LibreTranslate has no published port or egress. A
read-only, capability-free relay with fixed upstream publishes only
`127.0.0.1:5000` and accepts only bounded health/translate requests. Runtime
must not download or update.

- Starting an already ready service succeeds without creating a second one.
- Port conflict fails safely without stopping or deleting unrelated resources.
- `status` reports only safe lifecycle state and artifact identity.
- `verify` requires both health and the fixed public translation probe.
- Stopping an absent/stopped instance succeeds and preserves all slots.

Prepared readiness budget: 120 seconds. Translation client budget: 15 seconds.
Compose resource limits: 4 CPUs and 4 GiB RAM. Documented free-disk
prerequisite: 4 GiB.

## Update

`provider-local-update` is an explicitly online review operation. It must stop
before download unless the versioned lock has changed and the maintainer has
reviewed release provenance, manifest digest, package-index revision, model
identity/hash/license status, and storage/resource implications.

An update provisions a fresh `candidate`; it never mutates `current` or
`previous`. Successful online and offline checks promote candidate, retain the
old current as previous, and record only safe state. Failed checks discard or
quarantine candidate without changing the active provider.

## Rollback

`provider-local-rollback` must use only a previously verified slot and locally
available image. It must not contact a registry, model host, or package index.
Rollback succeeds only after offline health and fixed translation probes pass;
otherwise it leaves the last known-good active reference unchanged and reports
a redacted failure.

## Cleanup and complete removal

Ordinary `make clean` never touches provider resources. Destructive cleanup:

- requires exact `CONFIRM=remove-provider-data`;
- stops only the fixed Compose project;
- removes its candidate/current/previous volumes, provider/relay containers,
  internal/edge networks, and ignored operational state;
- never prunes global Docker images, networks, volumes, or unrelated projects;
- never invokes `sudo`, DNF, RPM, Flatpak, systemd, or host runtime removal.

All lifecycle output excludes source/translation content, model blob data,
workspace absolute paths, sensitive URLs, environment contents, and secrets.
