# shellcheck shell=bash

operational_provider_fail() {
  printf 'operational provider test failed: %s\n' "$1" >&2
  exit 1
}

operational_provider_temp_dir() {
  mktemp -d "${TMPDIR:-/tmp}/zed-en-es-provider-test.XXXXXX"
}

operational_provider_write_fake_docker() {
  local bin_dir="$1"
  local calls_file="$2"

  mkdir -p "$bin_dir"
  cat >"$bin_dir/docker" <<'FAKE_DOCKER'
#!/usr/bin/env bash
set -euo pipefail
printf 'PROVIDER_NETWORK_INTERNAL=%s PROVIDER_IMAGE_REFERENCE=%s %s\n' \
  "${PROVIDER_NETWORK_INTERNAL:-}" "${PROVIDER_IMAGE_REFERENCE:-}" "$*" \
  >>"${OPERATIONAL_PROVIDER_DOCKER_CALLS:?}"
if [[ -n "${OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN:-}" ]] \
  && [[ " $* " == *"${OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN}"* ]]; then
  if [[ -z "${OPERATIONAL_PROVIDER_DOCKER_FAIL_OCCURRENCE:-}" ]]; then
    exit 1
  fi
  matching_calls="$(grep -Fc -- "${OPERATIONAL_PROVIDER_DOCKER_FAIL_PATTERN}" \
    "${OPERATIONAL_PROVIDER_DOCKER_CALLS:?}" || true)"
  if [[ "$matching_calls" -eq "$OPERATIONAL_PROVIDER_DOCKER_FAIL_OCCURRENCE" ]]; then
    exit 1
  fi
fi
case " $* " in
  *' compose '*" ps --status running "*) printf 'provider\n' ;;
  *' compose '*" ps -q "*) printf 'controlled-container-id\n' ;;
  *' image inspect '*)
    if [[ "${OPERATIONAL_PROVIDER_IMAGE_ID_FROM_ARGUMENT:-0}" == 1 ]]; then
      printf '%s\n' "${*: -1}"
    else
      printf '%s\n' "${OPERATIONAL_PROVIDER_IMAGE_ID:-controlled-image-id}"
    fi
    ;;
esac
FAKE_DOCKER
  chmod +x "$bin_dir/docker"
  : >"$calls_file"
}

operational_provider_write_fake_http() {
  local bin_dir="$1"
  local calls_file="$2"

  mkdir -p "$bin_dir"
  cat >"$bin_dir/curl" <<'FAKE_CURL'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"${OPERATIONAL_PROVIDER_HTTP_CALLS:?}"
if [[ "${OPERATIONAL_PROVIDER_HTTP_FAIL:-0}" == 1 ]]; then
  exit 7
fi
output_file=
previous=
for argument in "$@"; do
  if [[ "$previous" == --output ]]; then
    output_file="$argument"
    break
  fi
  previous="$argument"
done
if [[ -n "$output_file" ]]; then
  printf 'controlled model artifact\n' >"$output_file"
elif [[ " $* " == *'/health'* ]]; then
  printf '{"status":"ok"}\n'
else
  printf '%s\n' "${OPERATIONAL_PROVIDER_HTTP_RESPONSE:-{\"translatedText\":\"controlled-ok\"}}"
fi
FAKE_CURL
  chmod +x "$bin_dir/curl"

  cat >"$bin_dir/sha256sum" <<'FAKE_SHA256SUM'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in
  *translate-en_es-1_0.argosmodel*)
    digest=d698d0ef87ad70d5d184b7fa6965905bf4368f09a2bb9ffb165a79bac96af0c4
    ;;
  *translate-es_en-1_9.argosmodel*)
    digest=c54df2b62fceaf54a3ce5d97db6bf56efd7940063329f6778f4212d2acb370d4
    ;;
  *)
    exec /usr/bin/sha256sum "$@"
    ;;
esac
printf '%s  %s\n' "$digest" "$1"
FAKE_SHA256SUM
  chmod +x "$bin_dir/sha256sum"
  : >"$calls_file"
}

operational_provider_assert_call_count() {
  local file="$1"
  local expected="$2"
  local actual=0
  if [[ -f "$file" ]]; then
    actual="$(wc -l <"$file" | tr -d ' ')"
  fi
  [[ "$actual" == "$expected" ]] \
    || operational_provider_fail "expected $expected calls, observed $actual"
}

operational_provider_assert_redacted() {
  local output="$1"
  shift
  local prohibited
  for prohibited in "$@"; do
    [[ "$output" != *"$prohibited"* ]] \
      || operational_provider_fail 'output contains prohibited test marker'
  done
}

operational_provider_assert_no_host_mutation_commands() {
  local file="$1"
  if grep -Eiq '(^|[[:space:]])(sudo|dnf|rpm|flatpak|systemctl|docker[[:space:]]+(system[[:space:]]+prune|volume[[:space:]]+prune|network[[:space:]]+prune))([[:space:]]|$)' "$file"; then
    operational_provider_fail 'controlled command log contains a forbidden host/global mutation'
  fi
}
