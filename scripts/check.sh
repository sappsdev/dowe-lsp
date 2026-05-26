#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$repo_root"
cargo check -p dowe-lsp-zed-extension --target wasm32-wasip2
cargo check -p dowe_language_server

old_extension_checkout="/Users/varb/Work/do""we-zed"
old_release_repository="do""we-lang/do""we-zed"
old_debug_binary="/Users/varb/Work/do""we/target/debug/do""we-language-server"

if rg -n \
  -e "$old_extension_checkout" \
  -e "$old_release_repository" \
  -e "$old_debug_binary" \
  --glob '!target/**' \
  --glob '!.zed-dev/**' \
  --glob '!Cargo.lock' \
  .; then
  exit 1
fi
