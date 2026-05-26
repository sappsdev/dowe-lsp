#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/extension.toml"
repository="${DOWE_LSP_REPOSITORY:-https://github.com/dowe-lang/dowe-lsp}"

cd "$repo_root"

rev="$(git rev-parse HEAD)"

REPOSITORY="$repository" REV="$rev" perl -0pi -e '
  s#\[grammars\.dowe\]\n(?:repository = "[^"]*"\n)?(?:rev = "[^"]*"\n)?(?:path = "[^"]*"\n)?#[grammars.dowe]\nrepository = "$ENV{REPOSITORY}"\nrev = "$ENV{REV}"\npath = "tree-sitter-dowe"\n#
' "$manifest"

printf 'Prepared %s with grammar rev %s\n' "$manifest" "$rev"
