#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source_dir="$repo_root/tree-sitter-dowe"
bare_dir="$repo_root/.zed-dev/tree-sitter-dowe.git"
manifest="$repo_root/extension.toml"
tmp_dir="$(mktemp -d)"

trap 'rm -rf "$tmp_dir"' EXIT

mkdir -p "$repo_root/.zed-dev"
mkdir -p "$tmp_dir/repo"
cp -R "$source_dir/." "$tmp_dir/repo/"
rm -rf "$tmp_dir/repo/.git"

git -C "$tmp_dir/repo" init -q
git -C "$tmp_dir/repo" add grammar.js tree-sitter.json src
export GIT_AUTHOR_DATE="2000-01-01T00:00:00Z"
export GIT_COMMITTER_DATE="$GIT_AUTHOR_DATE"
git -C "$tmp_dir/repo" \
  -c user.name="Dowe LSP" \
  -c user.email="dowe@example.invalid" \
  commit -q -m "Seed Dowe tree-sitter grammar"

rm -rf "$bare_dir"
git clone --bare -q "$tmp_dir/repo" "$bare_dir"
rm -rf "$repo_root/grammars/dowe"

rev="$(git -C "$tmp_dir/repo" rev-parse HEAD)"
repo_uri="file://$bare_dir"

REPO_URI="$repo_uri" REV="$rev" perl -0pi -e '
  s#\[grammars\.dowe\]\n(?:repository = "[^"]*"\n)?(?:rev = "[^"]*"\n)?(?:path = "[^"]*"\n)?#[grammars.dowe]\nrepository = "$ENV{REPO_URI}"\nrev = "$ENV{REV}"\n#
' "$manifest"

printf '%s\n' "$rev"
