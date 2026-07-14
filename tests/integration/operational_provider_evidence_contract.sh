#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANUAL="$ROOT/specs/007-operational-providers/manual-validation.md"
METADATA="$ROOT/tests/fixtures/operational-providers/expected-metadata.json"

required_cases=(
  LOCAL-CLI-01
  LOCAL-ZED-01
  REMOTE-CLI-01
  REMOTE-ZED-01
  LOCAL-PREPARE-01
  LOCAL-OFFLINE-01
  LOCAL-IDEMPOTENT-01
  LOCAL-UPDATE-FAIL-01
  LOCAL-ROLLBACK-01
  LOCAL-CLEAN-01
  REMOTE-DENY-01
  REMOTE-DISMISS-01
  REMOTE-STALE-01
  REMOTE-REUSE-01
  REMOTE-SECRET-01
  REMOTE-MISSING-KEY-01
  REMOTE-AUTH-QUOTA-01
  REMOTE-TIMEOUT-01
  REMOTE-RESPONSE-01
)

for case_id in "${required_cases[@]}"; do
  rg -q "\`${case_id}\`" "$MANUAL" || {
    printf 'missing evidence case: %s\n' "$case_id" >&2
    exit 1
  }
  jq -e --arg case_id "$case_id" '.case_ids | index($case_id) != null' \
    "$METADATA" >/dev/null || {
      printf 'missing synthetic metadata case: %s\n' "$case_id" >&2
      exit 1
    }
done

jq -e '
  .schema_version == 1 and
  .language_pair == "en-es" and
  .tone == "technical_neutral" and
  .preserve_formatting == true and
  .required_evidence_fields == [
    "case_id", "timestamp_utc", "surface", "locality",
    "normalized_outcome", "within_budget", "source_unchanged",
    "buffer_unchanged", "redaction_passed", "reviewer_result"
  ] and
  .allowed_localities == ["local", "remote"] and
  .allowed_results == ["pass", "fail"] and
  (.prohibited_evidence_fields | index("source_text") != null) and
  (.prohibited_evidence_fields | index("translated_text") != null) and
  (.prohibited_evidence_fields | index("credential") != null) and
  (.prohibited_evidence_fields | index("response_body") != null) and
  (.prohibited_evidence_fields | index("workspace_root") != null) and
  (.prohibited_evidence_fields | index("local_path") != null)
' "$METADATA" >/dev/null

for header in \
  'Timestamp UTC' \
  'Surface' \
  'Locality' \
  'Actual normalized outcome' \
  'Within budget' \
  'Source unchanged' \
  'Buffer unchanged' \
  'Redaction' \
  'Result'; do
  rg -q "$header" "$MANUAL" || {
    printf 'missing evidence field: %s\n' "$header" >&2
    exit 1
  }
done

evidence_rows="$(sed -n '/^| `\(LOCAL\|REMOTE\)-/p' "$MANUAL")"
if printf '%s\n' "$evidence_rows" | rg -i -q \
  '(https?://|/home/|/workspace/|Ocp-Apim|Authorization:|Bearer[[:space:]]|api[_-]?key[=:]|source_text|translated_text|response_body)'; then
  printf 'prohibited content found in evidence rows\n' >&2
  exit 1
fi

if rg -q '\[[^]]*(SOURCE|TRANSLATION|SECRET|TOKEN|KEY VALUE|PATH VALUE)[^]]*\]' "$MANUAL"; then
  printf 'unsafe placeholder found in manual evidence\n' >&2
  exit 1
fi

printf 'operational provider evidence/privacy contract ok\n'
