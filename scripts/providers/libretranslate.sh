#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LOCK_FILE="$ROOT/ops/providers/libretranslate/provider.lock"
COMPOSE_FILE="$ROOT/ops/providers/libretranslate/compose.yaml"
COMPOSE_PROJECT_NAME=zed-en-es-translator-providers
READINESS_TIMEOUT_SECONDS=120
PROVIDER_TIMEOUT_SECONDS=15
DEFAULT_PROVIDER_CACHE_ROOT="$ROOT/provider-cache/libretranslate"
PROVIDER_CACHE_ROOT="${PROVIDER_CACHE_ROOT:-$DEFAULT_PROVIDER_CACHE_ROOT}"
STATE_FILE="$PROVIDER_CACHE_ROOT/state.json"
CANDIDATE_STATE_FILE="$PROVIDER_CACHE_ROOT/candidate-state.json"
ARTIFACT_DIR="$PROVIDER_CACHE_ROOT/artifacts"
DOCKER_BIN="${DOCKER_BIN:-docker}"
CURL_BIN="${CURL_BIN:-curl}"

safe_error() {
  printf 'provider_status=%s\n' "$1" >&2
}

validate_runtime_overrides() {
  local temporary_test_root=false
  case "$PROVIDER_CACHE_ROOT" in
    /tmp/zed-en-es-provider-test.*/* | /var/tmp/zed-en-es-provider-test.*/*)
      temporary_test_root=true
      ;;
    "$DEFAULT_PROVIDER_CACHE_ROOT") ;;
    *)
      safe_error 'UNSAFE_RUNTIME_OVERRIDE'
      return 1
      ;;
  esac
  case "${PROVIDER_SKIP_DISK_CHECK:-0}" in
    0) ;;
    1)
      [[ "$temporary_test_root" == true ]] || {
        safe_error 'UNSAFE_RUNTIME_OVERRIDE'
        return 1
      }
      ;;
    *)
      safe_error 'UNSAFE_RUNTIME_OVERRIDE'
      return 1
      ;;
  esac
}

require_tools() {
  local tool
  for tool in "$DOCKER_BIN" "$CURL_BIN" jq sha256sum df; do
    command -v "$tool" >/dev/null 2>&1 || {
      safe_error 'PREREQUISITE_MISSING'
      return 1
    }
  done
  require_docker_compose
}

require_docker_compose() {
  if ! command -v "$DOCKER_BIN" >/dev/null 2>&1 \
    || ! "$DOCKER_BIN" compose version >/dev/null 2>&1; then
    safe_error 'PREREQUISITE_MISSING'
    return 1
  fi
}

validate_lock() {
  jq -e '
    .schema_version == 1 and
    (.image.reference | type == "string" and contains("@sha256:")) and
    (.image.platforms["linux/amd64"] | startswith("sha256:")) and
    (.package_index.revision | test("^[0-9a-f]{40}$")) and
    (.models | length == 2) and
    (all(.models[];
      (.source | startswith("https://argos-net.com/")) and
      (.filename | test("^[a-z0-9_-]+\\.argosmodel$")) and
      (.sha256 | test("^[0-9a-f]{64}$")) and
      .checksum_authority == "project-observed")) and
    .license.redistribution == "forbidden-until-resolved"
  ' "$LOCK_FILE" >/dev/null 2>&1 || {
    safe_error 'LOCK_INVALID'
    return 1
  }
}

check_disk_budget() {
  if [[ "${PROVIDER_SKIP_DISK_CHECK:-0}" == 1 ]]; then
    return 0
  fi
  local available_kib
  available_kib="$(df -Pk "$ROOT" 2>/dev/null | awk 'NR == 2 {print $4}')"
  if [[ -z "$available_kib" || "$available_kib" -lt 4194304 ]]; then
    safe_error 'INSUFFICIENT_DISK'
    return 1
  fi
}

image_reference() {
  jq -r '.image.reference' "$LOCK_FILE"
}

image_digest() {
  jq -r '.image.index_digest' "$LOCK_FILE"
}

compose() {
  "$DOCKER_BIN" compose \
    --project-name "$COMPOSE_PROJECT_NAME" \
    --file "$COMPOSE_FILE" \
    "$@"
}

compose_up() {
  local slot="$1"
  local internal="$2"
  local image="${3:-$(image_reference)}"
  PROVIDER_SLOT="$slot" PROVIDER_NETWORK_INTERNAL="$internal" \
    PROVIDER_IMAGE_REFERENCE="$image" \
    compose up --detach --no-build --pull never
}

compose_down() {
  local slot="$1"
  local image="${2:-$(image_reference)}"
  PROVIDER_SLOT="$slot" PROVIDER_NETWORK_INTERNAL=true \
    PROVIDER_IMAGE_REFERENCE="$image" \
    compose down --remove-orphans
}

download_locked_models() {
  mkdir -p "$ARTIFACT_DIR"
  chmod 0755 "$ARTIFACT_DIR" || {
    safe_error 'ARTIFACT_PERMISSION_FAILED'
    return 1
  }
  local filename source expected temporary actual
  while IFS=$'\t' read -r filename source expected; do
    temporary="$ARTIFACT_DIR/$filename.partial"
    "$CURL_BIN" --fail --silent --location \
      --proto '=https' --tlsv1.2 --max-time "$READINESS_TIMEOUT_SECONDS" \
      --output "$temporary" "$source" 2>/dev/null || {
        rm -f "$temporary"
        safe_error 'ARTIFACT_DOWNLOAD_FAILED'
        return 1
      }
    actual="$(sha256sum "$temporary" | awk '{print $1}')"
    if [[ "$actual" != "$expected" ]]; then
      rm -f "$temporary"
      safe_error 'ARTIFACT_INTEGRITY_FAILED'
      return 1
    fi
    mv "$temporary" "$ARTIFACT_DIR/$filename"
    chmod 0644 "$ARTIFACT_DIR/$filename" || {
      safe_error 'ARTIFACT_PERMISSION_FAILED'
      return 1
    }
  done < <(jq -r '.models[] | [.filename, .source, .sha256] | @tsv' "$LOCK_FILE")
}

install_locked_models() {
  local volume="$1"
  local image filename
  image="$(image_reference)"
  while IFS= read -r filename; do
    "$DOCKER_BIN" run --rm --pull never \
      --entrypoint /app/venv/bin/python \
      --volume "zed-en-es-translator-providers-$volume:/home/libretranslate/.local" \
      --volume "$ARTIFACT_DIR:/locked-models:ro" \
      "$image" -c \
      'import sys; from argostranslate.package import install_from_path; install_from_path(sys.argv[1])' \
      "/locked-models/$filename" >/dev/null 2>&1 || {
        safe_error 'ARTIFACT_INSTALL_FAILED'
        return 1
      }
  done < <(jq -r '.models[].filename' "$LOCK_FILE")
}

validate_runtime_image_reference() {
  local image="$1"
  [[ "$image" =~ ^libretranslate/libretranslate:v[0-9]+([.][0-9]+){1,2}@sha256:[0-9a-f]{64}$ ]]
}

image_digest_from_reference() {
  local image="$1"
  printf '%s\n' "${image##*@}"
}

verify_runtime_image() {
  local image="$1"
  local expected="$2"
  local failure_status="${3:-IMAGE_NOT_AVAILABLE}"
  local actual
  validate_runtime_image_reference "$image" || {
    safe_error "$failure_status"
    return 1
  }
  actual="$("$DOCKER_BIN" image inspect --format '{{index .RepoDigests 0}}' "$image" 2>/dev/null)" || {
    safe_error "$failure_status"
    return 1
  }
  [[ "$actual" == *"$expected"* ]] || {
    safe_error "$failure_status"
    return 1
  }
}

verify_image_identity() {
  verify_runtime_image "$(image_reference)" "$(image_digest)" 'IMAGE_IDENTITY_MISMATCH'
}

probe_once() {
  "$CURL_BIN" --fail --silent --max-time "$PROVIDER_TIMEOUT_SECONDS" \
    'http://127.0.0.1:5000/health' >/dev/null 2>&1 || return 1

  local response
  response="$("$CURL_BIN" --fail --silent --max-time "$PROVIDER_TIMEOUT_SECONDS" \
    --max-filesize 4096 \
    --header 'Content-Type: application/json' \
    --data '{"q":"Read the docs.","source":"en","target":"es","format":"text"}' \
    'http://127.0.0.1:5000/translate' 2>/dev/null)" || return 1
  jq -e '
    .translatedText as $translated |
    if ($translated | type) == "string" then
      ($translated | length) > 0 and $translated != "Read the docs."
    elif ($translated | type) == "array" then
      ($translated | length) == 1 and
      ($translated[0] | type) == "string" and
      ($translated[0] | length) > 0 and
      $translated[0] != "Read the docs."
    else false end
  ' <<<"$response" >/dev/null 2>&1
}

wait_until_ready() {
  local deadline=$((SECONDS + READINESS_TIMEOUT_SECONDS))
  while (( SECONDS < deadline )); do
    if probe_once; then
      return 0
    fi
    sleep 2
  done
  safe_error 'READINESS_TIMEOUT'
  return 1
}

state_value() {
  local field="$1"
  if [[ -f "$STATE_FILE" ]]; then
    jq -r --arg field "$field" '.[$field] // empty' "$STATE_FILE" 2>/dev/null || true
  fi
}

candidate_state_value() {
  local field="$1"
  if [[ -f "$CANDIDATE_STATE_FILE" ]]; then
    jq -r --arg field "$field" '.[$field] // empty' "$CANDIDATE_STATE_FILE" \
      2>/dev/null || true
  fi
}

logical_slot_value() {
  local logical_slot="$1"
  local suffix="$2"
  state_value "${logical_slot}_${suffix}"
}

active_slot_value() {
  local suffix="$1"
  local active_slot
  active_slot="$(state_value active_slot)"
  logical_slot_value "$active_slot" "$suffix"
}

next_candidate_volume() {
  local current_volume previous_volume slot
  current_volume="$(state_value current_volume)"
  previous_volume="$(state_value previous_volume)"
  for slot in candidate current previous; do
    if [[ "$slot" != "$current_volume" && "$slot" != "$previous_volume" ]]; then
      printf '%s\n' "$slot"
      return
    fi
  done
  safe_error 'CANDIDATE_SLOT_UNAVAILABLE'
  return 1
}

commit_state_file() {
  local temporary="$1"
  local destination="$2"
  mv "$temporary" "$destination" 2>/dev/null || {
    rm -f "$temporary" 2>/dev/null || true
    safe_error 'STATE_WRITE_FAILED'
    return 1
  }
}

write_ready_state() {
  local current_volume="$1"
  local current_identity="$2"
  local current_image_reference="$3"
  local current_lock_digest="$4"
  local previous_volume="${5:-}"
  local previous_identity="${6:-}"
  local previous_image_reference="${7:-}"
  local previous_lock_digest="${8:-}"
  local candidate_volume="$9"
  local temporary="$STATE_FILE.tmp"
  mkdir -p "$PROVIDER_CACHE_ROOT"
  if ! jq -n \
    --arg active_slot current \
    --arg lifecycle_state ready \
    --arg current_volume "$current_volume" \
    --arg current_identity "$current_identity" \
    --arg current_image_reference "$current_image_reference" \
    --arg current_lock_digest "$current_lock_digest" \
    --arg previous_volume "$previous_volume" \
    --arg previous_identity "$previous_identity" \
    --arg previous_image_reference "$previous_image_reference" \
    --arg previous_lock_digest "$previous_lock_digest" \
    --arg candidate_volume "$candidate_volume" \
    '{schema_version:1, active_slot:$active_slot, lifecycle_state:$lifecycle_state,
      offline_verified:true, current_volume:$current_volume,
      current_identity:$current_identity,
      current_image_reference:$current_image_reference,
      candidate_volume:$candidate_volume,
      previous_volume:(if $previous_volume == "" then null else $previous_volume end),
      previous_identity:(if $previous_identity == "" then null else $previous_identity end),
      previous_image_reference:(if $previous_image_reference == "" then null else $previous_image_reference end),
      current_lock_digest:$current_lock_digest,
      previous_lock_digest:(if $previous_lock_digest == "" then null else $previous_lock_digest end),
      lock_digest:$current_lock_digest}' >"$temporary" 2>/dev/null; then
    safe_error 'STATE_WRITE_FAILED'
    return 1
  fi
  commit_state_file "$temporary" "$STATE_FILE"
}

write_candidate_state() {
  local status="$1"
  local volume="$2"
  local temporary="$CANDIDATE_STATE_FILE.tmp"
  mkdir -p "$PROVIDER_CACHE_ROOT"
  if ! jq -n \
    --arg status "$status" \
    --arg volume "$volume" \
    --arg identity "$(image_digest)" \
    --arg image_reference "$(image_reference)" \
    --arg lock_digest "$(sha256sum "$LOCK_FILE" | awk '{print $1}')" \
    '{schema_version:1, status:$status, volume:$volume, identity:$identity,
      image_reference:$image_reference, lock_digest:$lock_digest}' \
    >"$temporary" 2>/dev/null; then
    safe_error 'STATE_WRITE_FAILED'
    return 1
  fi
  commit_state_file "$temporary" "$CANDIDATE_STATE_FILE"
}

reset_candidate_volume() {
  local volume="$1"
  local image="$2"
  local resource="zed-en-es-translator-providers-$volume"
  compose_down "$volume" "$image" >/dev/null 2>&1 || {
    safe_error 'CANDIDATE_RESET_FAILED'
    return 1
  }
  if "$DOCKER_BIN" volume inspect "$resource" >/dev/null 2>&1; then
    "$DOCKER_BIN" volume rm "$resource" >/dev/null 2>&1 || {
      safe_error 'CANDIDATE_RESET_FAILED'
      return 1
    }
  fi
  "$DOCKER_BIN" volume create "$resource" >/dev/null 2>&1 || {
    safe_error 'CANDIDATE_RESET_FAILED'
    return 1
  }
}

prepare_candidate() {
  local image candidate_volume
  image="$(image_reference)"
  candidate_volume="$(next_candidate_volume)"
  "$DOCKER_BIN" pull "$image" >/dev/null 2>&1 || {
    safe_error 'IMAGE_ACQUISITION_FAILED'
    write_candidate_state failed "$candidate_volume"
    return 1
  }
  verify_image_identity || {
    write_candidate_state failed "$candidate_volume"
    return 1
  }
  download_locked_models || {
    write_candidate_state failed "$candidate_volume"
    return 1
  }
  reset_candidate_volume "$candidate_volume" "$image" || {
    write_candidate_state failed "$candidate_volume"
    return 1
  }
  install_locked_models "$candidate_volume" || {
    write_candidate_state failed "$candidate_volume"
    return 1
  }

  compose_up "$candidate_volume" false "$image" >/dev/null 2>&1 || {
    safe_error 'PROVIDER_START_FAILED'
    write_candidate_state failed "$candidate_volume"
    return 1
  }
  if ! wait_until_ready; then
    compose_down "$candidate_volume" "$image" >/dev/null 2>&1 || true
    write_candidate_state failed "$candidate_volume"
    return 1
  fi
  compose_down "$candidate_volume" "$image" >/dev/null 2>&1 || true

  compose_up "$candidate_volume" true "$image" >/dev/null 2>&1 || {
    safe_error 'OFFLINE_START_FAILED'
    write_candidate_state failed "$candidate_volume"
    return 1
  }
  if ! wait_until_ready; then
    compose_down "$candidate_volume" "$image" >/dev/null 2>&1 || true
    write_candidate_state failed "$candidate_volume"
    return 1
  fi
  compose_down "$candidate_volume" "$image" >/dev/null 2>&1 || true
  write_candidate_state verified "$candidate_volume"
}

promote_candidate() {
  local active_slot candidate_volume candidate_identity candidate_image_reference
  local candidate_lock_digest previous_volume previous_identity
  local previous_image_reference previous_lock_digest next_candidate
  active_slot="$(state_value active_slot)"
  candidate_volume="$(candidate_state_value volume)"
  candidate_identity="$(candidate_state_value identity)"
  candidate_image_reference="$(candidate_state_value image_reference)"
  candidate_lock_digest="$(candidate_state_value lock_digest)"
  [[ "$(candidate_state_value status)" == verified ]] || {
    safe_error 'PROMOTION_FAILED'
    return 1
  }
  case "$active_slot" in
    current | previous)
      previous_volume="$(logical_slot_value "$active_slot" volume)"
      previous_identity="$(logical_slot_value "$active_slot" identity)"
      previous_image_reference="$(logical_slot_value "$active_slot" image_reference)"
      previous_lock_digest="$(logical_slot_value "$active_slot" lock_digest)"
      ;;
    *)
      previous_volume=
      previous_identity=
      previous_image_reference=
      previous_lock_digest=
      ;;
  esac
  next_candidate="$(for slot in candidate current previous; do
    if [[ "$slot" != "$candidate_volume" && "$slot" != "$previous_volume" ]]; then
      printf '%s\n' "$slot"
      break
    fi
  done)"

  compose_up "$candidate_volume" true "$candidate_image_reference" >/dev/null 2>&1 || {
    safe_error 'PROMOTION_FAILED'
    write_candidate_state failed "$candidate_volume"
    return 1
  }
  if ! wait_until_ready; then
    compose_down "$candidate_volume" "$candidate_image_reference" >/dev/null 2>&1 || true
    safe_error 'PROMOTION_FAILED'
    write_candidate_state failed "$candidate_volume"
    return 1
  fi
  write_ready_state \
    "$candidate_volume" "$candidate_identity" "$candidate_image_reference" \
    "$candidate_lock_digest" "$previous_volume" "$previous_identity" \
    "$previous_image_reference" "$previous_lock_digest" "$next_candidate"
  rm -f "$CANDIDATE_STATE_FILE"
}

prepare_provider() {
  require_tools
  validate_lock
  check_disk_budget
  mkdir -p "$PROVIDER_CACHE_ROOT"

  prepare_candidate
  promote_candidate
  printf 'provider_status=READY\n'
}

require_ready_state() {
  local active_slot current_volume previous_volume candidate_volume
  local active_volume active_identity active_image_reference
  active_slot="$(state_value active_slot)"
  case "$active_slot" in
    current | previous) ;;
    *)
      safe_error 'PROVIDER_NOT_PREPARED'
      return 1
      ;;
  esac
  [[ "$(state_value offline_verified)" == true ]] || {
      safe_error 'PROVIDER_NOT_PREPARED'
    return 1
  }
  current_volume="$(state_value current_volume)"
  previous_volume="$(state_value previous_volume)"
  candidate_volume="$(state_value candidate_volume)"
  active_volume="$(logical_slot_value "$active_slot" volume)"
  active_identity="$(logical_slot_value "$active_slot" identity)"
  active_image_reference="$(logical_slot_value "$active_slot" image_reference)"
  case "$active_volume" in candidate | current | previous) ;; *)
    safe_error 'PROVIDER_NOT_PREPARED'
    return 1
    ;;
  esac
  [[ -n "$current_volume" && "$candidate_volume" != "$current_volume" ]] || {
    safe_error 'PROVIDER_NOT_PREPARED'
    return 1
  }
  if [[ -n "$previous_volume" ]]; then
    [[ "$previous_volume" != "$current_volume" \
      && "$candidate_volume" != "$previous_volume" ]] || {
      safe_error 'PROVIDER_NOT_PREPARED'
      return 1
    }
  fi
  validate_runtime_image_reference "$active_image_reference" \
    && [[ "$(image_digest_from_reference "$active_image_reference")" == "$active_identity" ]] || {
      safe_error 'PROVIDER_NOT_PREPARED'
      return 1
    }
}

start_provider() {
  require_tools
  validate_lock
  require_ready_state
  local active_volume active_image_reference active_identity
  active_volume="$(active_slot_value volume)"
  active_image_reference="$(active_slot_value image_reference)"
  active_identity="$(active_slot_value identity)"
  verify_runtime_image "$active_image_reference" "$active_identity" || return 1
  compose_up "$active_volume" true "$active_image_reference" >/dev/null 2>&1 || {
    safe_error 'PROVIDER_START_FAILED'
    return 1
  }
  wait_until_ready
  write_lifecycle_state ready
  printf 'provider_status=READY\n'
}

status_provider() {
  local active_slot lifecycle_state identity lock_digest offline_verified
  active_slot="$(state_value active_slot)"
  if [[ "$active_slot" != current && "$active_slot" != previous ]]; then
    printf 'provider_status=UNPREPARED\n'
    return
  fi
  validate_persisted_state
  lifecycle_state="$(state_value lifecycle_state)"
  identity="$(state_value "${active_slot}_identity")"
  lock_digest="$(state_value "${active_slot}_lock_digest")"
  offline_verified="$(state_value offline_verified)"
  printf 'provider_status=%s\n' "${lifecycle_state^^}"
  printf 'provider_active_slot=%s\n' "$active_slot"
  printf 'provider_identity=%.19s\n' "$identity"
  printf 'provider_lock_digest=%.19s\n' "$lock_digest"
  printf 'provider_offline_verified=%s\n' "$offline_verified"
}

validate_persisted_state() {
  local active_slot lifecycle_state current_volume previous_volume candidate_volume
  local current_identity current_image_reference current_lock_digest offline_verified
  active_slot="$(state_value active_slot)"
  lifecycle_state="$(state_value lifecycle_state)"
  current_volume="$(state_value current_volume)"
  previous_volume="$(state_value previous_volume)"
  candidate_volume="$(state_value candidate_volume)"
  current_identity="$(state_value current_identity)"
  current_image_reference="$(state_value current_image_reference)"
  current_lock_digest="$(state_value current_lock_digest)"
  offline_verified="$(state_value offline_verified)"
  case "$active_slot" in current | previous) ;; *)
    safe_error 'STATE_INVALID'
    return 1
    ;;
  esac
  case "$lifecycle_state" in ready | stopped) ;; *)
    safe_error 'STATE_INVALID'
    return 1
    ;;
  esac
  case "$current_volume" in candidate | current | previous) ;; *)
    safe_error 'STATE_INVALID'
    return 1
    ;;
  esac
  case "$candidate_volume" in candidate | current | previous) ;; *)
    safe_error 'STATE_INVALID'
    return 1
    ;;
  esac
  if ! [[ "$candidate_volume" != "$current_volume" \
    && "$offline_verified" == true \
    && "$current_identity" =~ ^sha256:[0-9a-f]{64}$ \
    && "$current_lock_digest" =~ ^[0-9a-f]{64}$ \
    && "$(image_digest_from_reference "$current_image_reference")" == "$current_identity" ]] \
    || ! validate_runtime_image_reference "$current_image_reference"; then
    safe_error 'STATE_INVALID'
    return 1
  fi
  if [[ -n "$previous_volume" ]]; then
    local previous_identity previous_image_reference previous_lock_digest
    previous_identity="$(state_value previous_identity)"
    previous_image_reference="$(state_value previous_image_reference)"
    previous_lock_digest="$(state_value previous_lock_digest)"
    case "$previous_volume" in candidate | current | previous) ;; *)
      safe_error 'STATE_INVALID'
      return 1
      ;;
    esac
    if ! [[ "$previous_volume" != "$current_volume" \
      && "$previous_volume" != "$candidate_volume" \
      && "$previous_identity" =~ ^sha256:[0-9a-f]{64}$ \
      && "$previous_lock_digest" =~ ^[0-9a-f]{64}$ \
      && "$(image_digest_from_reference "$previous_image_reference")" == "$previous_identity" ]] \
      || ! validate_runtime_image_reference "$previous_image_reference"; then
      safe_error 'STATE_INVALID'
      return 1
    fi
  elif [[ "$active_slot" == previous ]]; then
    safe_error 'STATE_INVALID'
    return 1
  fi
}

verify_provider() {
  require_tools
  require_ready_state
  local active_image_reference active_identity
  active_image_reference="$(active_slot_value image_reference)"
  active_identity="$(active_slot_value identity)"
  verify_runtime_image "$active_image_reference" "$active_identity" || return 1
  if probe_once; then
    write_lifecycle_state ready
    printf 'provider_status=READY\n'
  else
    safe_error 'PROVIDER_UNAVAILABLE'
    return 1
  fi
}

write_lifecycle_state() {
  local lifecycle_state="$1"
  local temporary="$STATE_FILE.tmp"
  if ! jq --arg lifecycle_state "$lifecycle_state" \
    '.lifecycle_state = $lifecycle_state' "$STATE_FILE" >"$temporary" 2>/dev/null; then
    safe_error 'STATE_WRITE_FAILED'
    return 1
  fi
  commit_state_file "$temporary" "$STATE_FILE"
}

stop_provider() {
  local active_slot active_volume active_image_reference
  active_slot="$(state_value active_slot)"
  if [[ "$active_slot" == current || "$active_slot" == previous ]]; then
    require_docker_compose
    require_ready_state
    active_volume="$(active_slot_value volume)"
    active_image_reference="$(active_slot_value image_reference)"
    compose_down "$active_volume" "$active_image_reference" >/dev/null 2>&1 || {
      safe_error 'PROVIDER_STOP_FAILED'
      return 1
    }
    write_lifecycle_state stopped
  fi
  printf 'provider_status=STOPPED\n'
}

update_provider() {
  require_tools
  validate_lock
  check_disk_budget
  require_ready_state

  local current_lock_digest reviewed_lock_digest
  current_lock_digest="$(state_value current_lock_digest)"
  if [[ -z "$current_lock_digest" ]]; then
    current_lock_digest="$(state_value lock_digest)"
  fi
  reviewed_lock_digest="$(sha256sum "$LOCK_FILE" | awk '{print $1}')"
  if [[ "$current_lock_digest" == "$reviewed_lock_digest" ]]; then
    safe_error 'UPDATE_REVIEW_REQUIRED'
    return 1
  fi

  prepare_candidate
  promote_candidate
  printf 'provider_status=UPDATED\n'
}

write_rollback_state() {
  local temporary="$STATE_FILE.tmp"
  if ! jq '.active_slot = "previous" | .lifecycle_state = "ready" | .offline_verified = true' \
    "$STATE_FILE" >"$temporary" 2>/dev/null; then
    safe_error 'STATE_WRITE_FAILED'
    return 1
  fi
  commit_state_file "$temporary" "$STATE_FILE"
}

rollback_provider() {
  require_tools
  validate_lock
  require_ready_state
  [[ -n "$(state_value previous_identity)" ]] || {
    safe_error 'ROLLBACK_NOT_AVAILABLE'
    return 1
  }

  local original_slot original_volume original_image_reference
  local previous_volume previous_image_reference previous_identity
  original_slot="$(state_value active_slot)"
  original_volume="$(logical_slot_value "$original_slot" volume)"
  original_image_reference="$(logical_slot_value "$original_slot" image_reference)"
  previous_volume="$(state_value previous_volume)"
  previous_image_reference="$(state_value previous_image_reference)"
  previous_identity="$(state_value previous_identity)"
  verify_runtime_image \
    "$previous_image_reference" "$previous_identity" 'ROLLBACK_IMAGE_UNAVAILABLE' || return 1
  compose_down "$original_volume" "$original_image_reference" >/dev/null 2>&1 || {
    safe_error 'ROLLBACK_FAILED'
    return 1
  }
  if ! compose_up "$previous_volume" true "$previous_image_reference" >/dev/null 2>&1; then
    compose_up "$original_volume" true "$original_image_reference" >/dev/null 2>&1 || true
    safe_error 'ROLLBACK_FAILED'
    return 1
  fi
  if ! wait_until_ready; then
    compose_down "$previous_volume" "$previous_image_reference" >/dev/null 2>&1 || true
    compose_up "$original_volume" true "$original_image_reference" >/dev/null 2>&1 || true
    safe_error 'ROLLBACK_FAILED'
    return 1
  fi
  write_rollback_state
  printf 'provider_status=ROLLED_BACK\n'
}

clean_provider() {
  local confirmation="${1:-}"
  if [[ "$confirmation" != remove-provider-data ]]; then
    safe_error 'CLEAN_CONFIRMATION_REQUIRED'
    return 1
  fi

  if [[ -f "$STATE_FILE" ]]; then
    validate_persisted_state
  fi

  require_docker_compose

  local slot
  for slot in candidate current previous; do
    compose_down "$slot" >/dev/null 2>&1 || {
      safe_error 'CLEAN_FAILED'
      return 1
    }
  done
  local resource
  for resource in \
    zed-en-es-translator-providers-candidate \
    zed-en-es-translator-providers-current \
    zed-en-es-translator-providers-previous; do
    if "$DOCKER_BIN" volume inspect "$resource" >/dev/null 2>&1; then
      "$DOCKER_BIN" volume rm "$resource" >/dev/null 2>&1 || {
        safe_error 'CLEAN_FAILED'
        return 1
      }
    fi
  done
  for resource in \
    zed-en-es-translator-providers-runtime \
    zed-en-es-translator-providers-edge; do
    if "$DOCKER_BIN" network inspect "$resource" >/dev/null 2>&1; then
      "$DOCKER_BIN" network rm "$resource" >/dev/null 2>&1 || {
        safe_error 'CLEAN_FAILED'
        return 1
      }
    fi
  done
  rm -rf "$PROVIDER_CACHE_ROOT" 2>/dev/null || {
    safe_error 'CLEAN_FAILED'
    return 1
  }
  printf 'provider_status=CLEANED\n'
}

validate_runtime_overrides

case "${1:-}" in
  prepare) prepare_provider ;;
  start) start_provider ;;
  status) status_provider ;;
  verify) verify_provider ;;
  stop) stop_provider ;;
  update) update_provider ;;
  rollback) rollback_provider ;;
  clean) clean_provider "${2:-}" ;;
  *)
    safe_error 'INVALID_OPERATION'
    exit 2
    ;;
esac
