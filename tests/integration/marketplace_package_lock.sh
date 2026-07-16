#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock="$root/ops/marketplace/package.lock.json"
model_lock="$root/ops/marketplace/model.lock.json"

fail() {
    printf 'marketplace package lock contract: %s\n' "$1" >&2
    exit 1
}

command -v jq >/dev/null || fail "jq is required"
test -f "$lock" || fail "package.lock.json is missing"
jq empty "$lock" >/dev/null || fail "package lock is not valid JSON"

jq -e '
  (keys | sort) == (["budgets", "license_bundle", "model_resources", "package_id", "package_version", "platform", "schema_version", "server_archive", "source_language", "target_language"] | sort)
  and .schema_version == 1
  and (.package_id | test("^[a-z0-9][a-z0-9._-]{0,127}$"))
  and (.package_version | test("^[0-9]+\\.[0-9]+\\.[0-9]+$"))
  and .platform == "linux-x86_64"
  and .source_language == "en"
  and .target_language == "es"
  and .server_archive.archive_type == "gzip_tar"
  and (.server_archive.url | test("^https://github.com/Theos48/zed-en-es-translator/releases/download/[^/?#]+/[^/?#]+\\.tar\\.gz$"))
  and (.server_archive.files | length >= 5 and length <= 32)
  and ([.server_archive.files[].role] | map(select(. == "language_server")) | length == 1)
  and ([.server_archive.files[].role] | map(select(. == "native_runner")) | length == 1)
  and ([.server_archive.files[] | select(.executable == true) | .role] | sort == ["language_server", "native_runner"])
  and (all(.server_archive.files[];
      (.path | test("^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$"))
      and (.installed_size > 0 and .installed_size <= 134217728)
      and (.installed_sha256 | test("^[0-9a-f]{64}$") and . != ("0" * 64))
      and (.source_url | test("^https://[^?#]+$"))))
  and (.model_resources | length == 3)
  and ([.model_resources[].role] | sort == ["lexical_shortlist", "model", "vocabulary"])
  and (all(.model_resources[];
      (.url | test("^https://[^?#]+$"))
      and (.compressed_size > 0 and .compressed_size <= 67108864)
      and (.installed_size > 0 and .installed_size <= 134217728)
      and (.compressed_sha256 | test("^[0-9a-f]{64}$") and . != ("0" * 64))
      and (.installed_sha256 | test("^[0-9a-f]{64}$") and . != ("0" * 64))
      and .spdx_conclusion == "MPL-2.0"))
  and .budgets.maximum_transfer_bytes == 67108864
  and .budgets.maximum_active_installed_bytes == 134217728
  and .budgets.maximum_lifecycle_bytes == 402653184
  and .budgets.required_free_bytes == 536870912
  and .budgets.peak_rss_bytes == 1073741824
  and .budgets.inference_threads == 4
  and .budgets.provider_deadline_ms == 15000
  and .license_bundle.extension_spdx == "MIT"
  and (.license_bundle.required_paths | unique | length >= 3)
' "$lock" >/dev/null || fail "schema or semantic validation failed"

jq -S '.model_resources' "$lock" >"${TMPDIR:-/tmp}/marketplace-lock-models.$$.json"
jq -S '.model_resources' "$model_lock" >"${TMPDIR:-/tmp}/marketplace-model-lock-models.$$.json"
trap 'rm -f "${TMPDIR:-/tmp}/marketplace-lock-models.$$.json" "${TMPDIR:-/tmp}/marketplace-model-lock-models.$$.json"' EXIT
cmp -s "${TMPDIR:-/tmp}/marketplace-lock-models.$$.json" "${TMPDIR:-/tmp}/marketplace-model-lock-models.$$.json" \
    || fail "published model identities drift from model.lock.json"

printf 'marketplace package lock contract: ok\n'
