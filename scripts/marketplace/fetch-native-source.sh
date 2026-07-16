#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
source_root="$root/.cache/embedded-source/mozilla-translations"
repository=https://github.com/mozilla/translations.git
commit=f31423c7c2c6ed8ae57d71a3d19a9db6f156060e

mkdir -p "$(dirname "$source_root")"
if [[ ! -d "$source_root/.git" ]]; then
  git clone --filter=blob:none --no-checkout "$repository" "$source_root"
fi

actual_origin=$(git -C "$source_root" remote get-url origin)
[[ "$actual_origin" == "$repository" ]] || {
  printf 'provider_status=SOURCE_ORIGIN_MISMATCH\n' >&2
  exit 1
}

git -C "$source_root" fetch --no-tags origin "$commit"
git -C "$source_root" checkout --detach --force "$commit"
git -C "$source_root" submodule sync
git -C "$source_root" submodule update --init --checkout \
  3rd_party/extract-lex \
  3rd_party/fast_align \
  3rd_party/kenlm \
  3rd_party/marian-dev \
  3rd_party/preprocess \
  inference/3rd_party/emsdk \
  inference/3rd_party/ssplit-cpp \
  inference/marian-fork/src/3rd_party/fbgemm \
  inference/marian-fork/src/3rd_party/intgemm \
  inference/marian-fork/src/3rd_party/nccl \
  inference/marian-fork/src/3rd_party/onnxjs \
  inference/marian-fork/src/3rd_party/ruy \
  inference/marian-fork/src/3rd_party/sentencepiece \
  inference/marian-fork/src/3rd_party/simd_utils \
  inference/marian-fork/src/3rd_party/simple-websocket-server

# The portable vendored SGEMM backend requires Eigen through ONNX.js. Keep this
# one nested dependency explicit instead of recursively initialising unrelated
# training/tooling submodules.
git -C "$source_root/inference/marian-fork/src/3rd_party/onnxjs" \
  submodule update --init --checkout deps/eigen

# The upstream Marian configure step traverses unrelated training submodules.
# Keep those nested worktrees uninitialised so this inference-only checkout
# returns to the exact, direct gitlinks recorded by the lock after a build.
git -C "$source_root/3rd_party/marian-dev" submodule deinit --force --all >/dev/null

[[ "$(git -C "$source_root" rev-parse HEAD)" == "$commit" ]]
[[ -z "$(git -C "$source_root" status --porcelain --untracked-files=no)" ]]
printf 'marketplace_status=source_ready commit=%s\n' "$commit"
