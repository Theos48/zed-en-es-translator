#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
test_file="$root/zed-extension/tests/acquisition_concurrency.rs"
source_file="$root/zed-extension/src/acquisition.rs"

rg -q 'concurrent_preparations_have_one_owner_and_one_retryable_busy_result' "$test_file"
rg -q 'create_new\(true\)' "$source_file"
rg -q 'AcquisitionError::Busy' "$source_file" "$test_file"
rg -q 'remove_file\(&self.path\)' "$source_file"

printf 'marketplace acquisition concurrency contract: ok\n'
