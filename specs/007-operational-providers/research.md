# Research: Operational Real Providers

## Decision: LibreTranslate 1.9.6 is the supported local provider

The local path will use the official CPU image pinned to the immutable
multi-architecture digest:

```text
libretranslate/libretranslate:v1.9.6@sha256:1de2d7056bb8ad607a412f4563d9abe324ff632b43b5be9428bcc8e213aebb32
```

The project will adapt the upstream Compose topology instead of using it
unchanged: publish only `127.0.0.1:5000`, load only English and Spanish,
disable the web UI and file translation, persist `/home/libretranslate/.local`,
and prohibit automatic model updates during normal operation.

**Rationale**:

- v1.9.6 is the reviewed stable release; `latest` is mutable.
- The existing Rust adapter already implements the LibreTranslate `/translate`
  contract, so the implementation can focus on safe operation rather than a
  second local protocol.
- The image is multi-architecture and avoids a Python/runtime installation on
  Fedora.

**Sources**:

- [LibreTranslate v1.9.6 release](https://github.com/LibreTranslate/LibreTranslate/releases/tag/v1.9.6)
- [Official v1.9.6 Compose file](https://github.com/LibreTranslate/LibreTranslate/blob/v1.9.6/docker-compose.yml)
- [Official installation guide](https://docs.libretranslate.com/guides/installation/)
- [Official FAQ](https://docs.libretranslate.com/guides/faq/)

**Alternatives considered**:

- Upstream Compose unchanged: rejected because it uses `latest`, publishes on
  all interfaces, and does not provide this project's offline/update/rollback
  gates.
- Host Python installation: rejected by the clean-host policy and upstream's
  containerized production guidance.
- CUDA image: rejected because CPU translation is sufficient for the
  acceptance matrix and CUDA adds size, architecture, driver, and host risk.

## Decision: Provision language artifacts explicitly, then run without egress

Initial `prepare` is the only normal operation allowed to use external
network access. It pulls the pinned image and provisions language resources in
a candidate persistent volume. Readiness requires both `GET /health` and a
fixed public English-to-Spanish synthetic probe; the upstream health script
alone is insufficient because it can report success while bootstrapping.

After successful preparation, normal `start`, `verify`, translation, `stop`,
and `rollback` use the locally available image with `pull_policy: never` on a
Docker internal network. LibreTranslate has no published port; a bounded,
fixed-destination relay in the same Compose project is the only service
reachable through loopback.

**Rationale**:

- The official image is built without bundled models and obtains them at
  bootstrap.
- Process health is not equivalent to English-to-Spanish model readiness.
- Separating acquisition from runtime makes SC-002 objectively testable.

**Sources**:

- [LibreTranslate Dockerfile v1.9.6](https://github.com/LibreTranslate/LibreTranslate/blob/v1.9.6/docker/Dockerfile)
- [LibreTranslate bootstrap logic](https://github.com/LibreTranslate/LibreTranslate/blob/v1.9.6/libretranslate/init.py)
- [Health API](https://docs.libretranslate.com/api/operations/health/)
- [Translate API](https://docs.libretranslate.com/api/operations/translate/)

## Decision: Do not redistribute the Argos English-to-Spanish model

The feature may download the model on behalf of the user into project-scoped
Docker storage, but it must not vendor the blob, commit it, publish it, or bake
it into a derivative image. The current Argos package index does not declare a
license or upstream checksum for `translate-en_es-1_0.argosmodel`, and upstream
issue #507 lists it among models with unresolved license metadata.

The implementation lock may record these locally observed artifact hashes for
integrity checking, clearly labelled as project observations rather than
upstream guarantees:

```text
en -> es 1.0  d698d0ef87ad70d5d184b7fa6965905bf4368f09a2bb9ffb165a79bac96af0c4
es -> en 1.9  c54df2b62fceaf54a3ce5d97db6bf56efd7940063329f6778f4212d2acb370d4
```

The second direction is currently pulled by LibreTranslate when restricted to
the `en,es` language set; it is an operational dependency even though product
acceptance is English-to-Spanish only.

**Rationale**:

- User-local acquisition allows F011 operation without silently asserting a
  redistribution right that upstream has not documented.
- A project-observed hash detects corruption or upstream blob replacement;
  provenance review remains required when updating it.
- This license gap blocks packaging/publication of a bundled model in F009,
  not project-local validation in F011.

**Sources**:

- [Pinned Argos package index revision](https://github.com/argosopentech/argospm-index/blob/ff90de60728f7c1338ff6b75974e4c89b2442d22/index.json)
- [Argos model license issue #507](https://github.com/argosopentech/argos-translate/issues/507)
- [LibreTranslate AGPL-3.0 license](https://github.com/LibreTranslate/LibreTranslate/blob/v1.9.6/LICENSE)

**Alternative considered**: a custom image with bundled models was rejected
because it would redistribute the model and make publication/licensing claims
that cannot currently be supported.

## Decision: Use candidate/current/previous lifecycle slots

Local state will be separated into immutable provider lock metadata and three
project-namespaced persistent slots: `candidate`, `current`, and `previous`.
Preparation or update writes only `candidate`; promotion is permitted only
after image identity, artifact integrity, health, synthetic translation, and
offline restart pass. Promotion retains the former `current` as `previous`.
Rollback changes the active slot to `previous` without downloading anything.

`start`, `status`, `verify`, and `stop` are idempotent. Destructive provider
storage removal is an explicit, confirmed `provider-local-clean` operation and
is never part of the repository's ordinary `make clean`.

**Rationale**:

- An in-place model update has no reliable rollback boundary.
- Retaining one known-good slot makes a failed update recoverable offline.
- Separating ordinary build cleanup from provider data avoids accidental
  deletion of large, prepared artifacts.

## Decision: Azure Translator Text F0 is the supported remote provider

The remote acceptance path will use one global single-service Azure Translator
resource on the F0 tier and the fixed endpoint:

```text
POST https://api.cognitive.microsofttranslator.com/translate?api-version=3.0&from=en&to=es
```

The credential is sent only as `Ocp-Apim-Subscription-Key`, sourced from the
environment variable named by `TRANSLATOR_PROVIDER_API_KEY_ENV`. The provider
mode is `azure_translator`; a configurable URL, region, custom endpoint,
redirect, proxy inheritance, or retry is forbidden.

**Rationale**:

- F0 provides a no-paid-Translator-plan acceptance path and documented quota
  with no Translator overage on that tier.
- The fixed global endpoint allows an exact host/path allowlist and avoids new
  settings beyond the existing four provider environment keys.
- Azure documents that synchronous text translation processes text at the REST
  API and does not persist submitted text.

**Privacy boundary**: the v3 global endpoint is handled by the closest
available data center and Microsoft documents that failover may route a request
outside that geography. It therefore provides no project-enforced data
residency. The current authoritative data-privacy page states no persistence
for Text Translation but does not make an explicit training-use promise; this
plan does not infer one. F011 sends public synthetic content for acceptance and
requires per-request disclosure/confirmation for any later use.

**Account caveat**: creating an Azure free account may require a phone number
and payment card. Azure documents that a free account must move to pay-as-you-go
after its introductory period to remain active; the Translator resource itself
must remain explicitly on F0 for this feature. The quickstart must present this
before the user opts into remote validation.

**Sources**:

- [Create a Translator resource](https://learn.microsoft.com/en-us/azure/ai-services/translator/how-to/create-translator-resource)
- [Translator pricing and F0 quota](https://azure.microsoft.com/en-us/pricing/details/translator/)
- [Azure account options](https://azure.microsoft.com/en-us/pricing/purchase-options/azure-account)
- [Translate API v3](https://learn.microsoft.com/en-us/azure/ai-services/translator/text-translation/reference/v3/translate)
- [Service limits](https://learn.microsoft.com/en-us/azure/ai-services/translator/service-limits)
- [Translator data privacy and security](https://learn.microsoft.com/en-us/azure/foundry/responsible-ai/translator/data-privacy-security)
- [Translator v3 base URLs and processing geography](https://learn.microsoft.com/en-us/azure/ai-services/translator/text-translation/reference/v3/reference)

**Alternatives considered**:

- DeepL API Free: rejected because current API terms permit indefinite storage
  of submitted content for service improvement unless stronger terms apply.
- Google Cloud Translation: rejected because account/billing configuration can
  transition into paid usage rather than a hard no-overage F0 resource.
- Public community LibreTranslate mirrors: rejected because endpoint ownership,
  privacy terms, availability, and quota are not controlled enough for a
  supported remote contract.

## Decision: Extend provider selection without widening configuration

Add `ProviderMode::AzureTranslator` and an `AzureTranslatorProvider` behind the
existing `Provider` trait. Keep the four existing configuration names:

```text
TRANSLATOR_PROVIDER=azure_translator
TRANSLATOR_PROVIDER_API_KEY_ENV=<safe environment variable name>
TRANSLATOR_ALLOW_REMOTE_PROVIDER=true
TRANSLATOR_PROVIDER_URL=<must be absent for azure_translator>
```

`MockProvider` remains the no-configuration default. LibreTranslate requires a
loopback URL and forbids the API-key reference for the supported local profile.
Azure requires the key reference and remote enablement, while its URL must be
absent because the endpoint is compiled into the reviewed adapter.

CLI, MCP, LSP, and Zed launch validation must derive from the same
`ProviderConfiguration`; the LSP startup must reject disagreement between the
configuration used for the locality label and the provider instance used for
execution.

**Rationale**:

- Arbitrary remote URLs would turn the adapter into an SSRF/configuration
  surface and weaken D075.
- Reusing the established keys avoids storing actual credentials in Zed
  settings and preserves the current launch boundary.
- One parsed configuration prevents CLI/Zed drift.

## Decision: Keep current normalized errors and do not retry

Configuration defects map to `PROVIDER_NOT_CONFIGURED`; request timeout and
HTTP 408 map to `PROVIDER_TIMEOUT`; DNS, TLS, authentication, quota, other HTTP
status, malformed body, cardinality mismatch, and oversized response map to
`PROVIDER_FAILED`. Unsafe targets fail under the existing configuration/input
codes before contact. Response bodies and sensitive request detail are never
included in messages.

No automatic retry is allowed. Every retry of remote content must be a new
user-confirmed request and must fit the same 15-second provider budget.

**Rationale**:

- Existing surfaces already depend on the stable error enum.
- Provider-specific public error codes would expose implementation detail
  without improving safe recovery.
- An automatic remote retry would reuse consent and can multiply disclosures.

## Decision: Establish measurable resource and change-control envelopes

The supported local profile targets the current Fedora `x86_64` workstation
with Docker and Compose already classified as transversal host tools. The
Compose profile will cap the service at 4 logical CPUs and 4 GiB RAM. Planning
allows 4 GiB free disk for the approximately 192 MiB compressed image, about
355 MiB of currently documented Argos packages plus MiniSBD, extracted layers,
candidate/current/previous slots, and operational overhead.

Prepared startup must become model-ready within 120 seconds on the target
workstation; an individual translation remains bounded by the existing
15-second client timeout. These are acceptance budgets to measure, not upstream
performance guarantees.

Before any pinned image, model, API version, host, F0 eligibility, service
privacy statement, or terms reference changes, `provider-local-update` must
stop at a review gate. Certificate failure and unexpected redirect fail closed;
free-tier removal or privacy-policy drift leaves mock/local available and
blocks remote acceptance until a new planning decision is recorded.

**Rationale**:

- Upstream publishes artifact sizes but no supported RAM/startup envelope, so
  this feature needs explicit testable budgets.
- Change control prevents a mutable upstream service or package index from
  silently changing the approved data boundary.
