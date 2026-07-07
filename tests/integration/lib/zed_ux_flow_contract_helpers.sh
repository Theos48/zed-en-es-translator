require_in_file() {
  local file="$1"
  local needle="$2"

  if ! grep -Fq -- "$needle" "$file"; then
    printf 'missing contract text in %s: %s\n' "$file" "$needle" >&2
    exit 1
  fi
}
