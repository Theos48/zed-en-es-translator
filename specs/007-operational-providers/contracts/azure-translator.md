# Contract: Azure Translator Remote Provider

## Service boundary

The only supported managed service is Azure AI Translator Text, global
single-service resource, F0 tier, API version 3.0.

```http
POST /translate?api-version=3.0&from=en&to=es HTTP/1.1
Host: api.cognitive.microsofttranslator.com
Content-Type: application/json; charset=UTF-8
Ocp-Apim-Subscription-Key: <value resolved from referenced environment variable>
```

One segment is sent per request body element:

```json
[
  { "Text": "<one permitted translatable segment>" }
]
```

The implementation may batch the already validated segment list into one API
call because project limits (256 elements, 20 KiB total) remain below Azure's
documented 1,000-element and 50,000-character request limits. It must preserve
element order and require exactly one valid non-empty Spanish translation per
input element.

No workspace root, file path, URI, protected region, environment data, log,
local context, arbitrary metadata, or unselected credential is sent.

## Internal tone and formatting boundary

The internal `ProviderRequest` retains the existing technical-neutral tone and
formatting intent. The adapter validates those invariants before contact.
Formatting preservation remains the responsibility of segmentation and
reconstruction around the provider boundary.

The reviewed Azure v3 request used by this feature has no project-approved
tone or formatting field. The adapter therefore sends no invented query,
header, JSON field, or metadata for them; each external body element contains
only `Text` as shown above. A future tone or format mode must fail closed until
its protocol mapping is separately specified and tested.

## Account and consent disclosure

Before remote setup/acceptance, documentation must state:

- an Azure account and API key are required;
- account creation may require phone and payment-card information;
- the Translator resource must be explicitly created on F0;
- F0 currently documents 2 million characters/month and no Translator
  overage, but quota/service terms may change;
- continued Azure account use after the introductory free-account period may
  require conversion to pay-as-you-go even while Translator remains F0;
- Microsoft states synchronous Text Translation processes submitted text at
  the REST API and does not persist it;
- the global endpoint normally uses the closest available data center and may
  fail over outside that geography, so this profile gives no data-residency
  guarantee;
- the current authoritative privacy page does not explicitly promise that
  submitted text is excluded from training; the project makes no such claim
  and restricts acceptance to public synthetic content;
- content leaves the machine and user confirmation is required for every
  request regardless of provider privacy statements.

If F0, endpoint, retention/privacy, account eligibility, or terms no longer
match the reviewed contract, remote validation stops. Mock and local paths
remain available.

## Transport controls

- TLS is mandatory and certificates use the configured Rust TLS verifier.
- Exact host/path/query allowlisting occurs before contact.
- Redirect limit is zero.
- Inherited HTTP(S)/ALL proxy environment is disabled.
- Timeout is the existing 15-second global provider budget.
- Automatic retry is disabled.
- Response reads remain bounded by the existing 40 KiB output contract plus
  minimal JSON overhead; oversized bodies fail before exposure.
- Raw request/response debug logging is never enabled.

## Credential handling

The value of `TRANSLATOR_PROVIDER_API_KEY_ENV` is only a safe environment
variable name, for example `AZURE_TRANSLATOR_KEY`. The named variable's value:

- exists only in the launching user's secret-capable environment;
- is not placed in repository files, `.env`, Zed settings, command arguments,
  evidence, screenshots, logs, or diagnostics;
- is resolved only by the provider configuration boundary;
- is never included in `Debug` output or error context.

For direct Zed, the actual value must already exist in the parent Zed process
environment. The extension validates and emits only the reference name; it
does not read, copy, or add the secret value to `binary.env`, command arguments,
launch profiles, settings, diagnostics, or evidence.

## Response validation

For every input element, accept exactly one response object with a non-empty
first translation whose `to` is Spanish when present. Reject missing arrays,
empty translations, non-text values, mismatched cardinality/order, invalid
JSON, unexpected top-level shape, or aggregate output beyond 40 KiB.

Provider status/body detail never crosses the adapter. Stable mapping follows
the provider-selection contract; authentication, quota, rate limit, and other
service failures use generic `PROVIDER_FAILED`.
