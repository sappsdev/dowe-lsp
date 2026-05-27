# Dowe LSP

Dowe LSP is the dedicated Rust language-server repository for Dowe Source Format files.

This repository builds `dowe-language-server`. The language server owns diagnostics, completions, formatting, hover, symbols, navigation, and semantic editor behavior by consuming the shared Dowe compiler APIs from the sibling Dowe checkout.

The official Zed extension lives in `../dowe-zed`. Do not install the Zed dev extension from this repository for local Dowe editing; install or refresh the language-server binary here, then install the extension from `../dowe-zed`.

## Repository Split

| Repository | Responsibility |
| --- | --- |
| `../dowe` | Compiler, source parser, component model, semantic validation, target generation, and public Dowe docs |
| `../dowe-lsp` | Rust `dowe-language-server` and semantic editor features |
| `../dowe-zed` | Zed extension adapter, Tree-sitter grammar, Zed queries, icon themes, `extension.toml`, and dev-extension install surface |

This split is intentional. Keep language semantics close to the compiler and keep Zed packaging in the Zed extension repository. Tree-sitter grammar and Zed queries may classify source for editor display, but they must not duplicate compiler validation rules.

## Requirements

- Rust installed through `rustup`.
- A sibling Dowe checkout at `../dowe` for local language-server builds against `dowe_compiler`.

The language server does not require Node.js, `node_modules`, npm, Prettier, or ESLint.

## Local Development

Build and install the language server:

```sh
cargo check -p dowe_language_server
cargo install --path crates/language_server --force
```

For Zed validation after language-server changes, refresh the extension from the sibling Zed repository:

```sh
cd ../dowe-zed
cd tree-sitter-dowe
tree-sitter generate
tree-sitter test
cd ..
./scripts/bootstrap-grammar-repo.sh
./scripts/check.sh
```

Install the extension in Zed with `zed: install dev extension` and select `../dowe-zed`.

When testing local compiler or language API changes, configure Zed's Dowe language server binary path to `/Users/varb/.cargo/bin/dowe-language-server` after `cargo install`, or to this repository's `target/debug/dowe-language-server` when running a debug build directly. Restart the Dowe language server or reload the Zed window after replacing the executable.

## Zed Extension Updates

Use `../dowe-zed` for all Zed-specific changes:

- `tree-sitter-dowe/grammar.js` for syntax nodes, component names, block keywords, and recovery behavior.
- `languages/dowe/*.scm` for highlighting, indentation, outline, text objects, and bracket matching.
- `extension.toml` for Zed language, grammar, icon, and language-server registration.
- `icon_themes` and `assets` for Dowe file icons.

Use this repository for semantic changes:

- Compiler-backed diagnostics.
- Formatter behavior.
- Completion labels and source edits.
- Hover and go-to-definition behavior.
- Document symbols and workspace analysis.

When a Dowe feature changes both syntax and semantics, update both repositories in the same work.

## Language Server

`dowe-language-server` is a Rust stdio language server. It does not execute `.dowe` files, start `dowe dev`, open server ports, run handlers, require `node_modules`, or use Prettier, ESLint, npm, React, DOM, or Node.js.

The Zed adapter decides which public release repository provides managed `dowe-language-server` assets. Local development should use an explicit Zed binary setting or a `dowe-language-server` binary on `PATH` so extension tests do not depend on public releases.

Each managed release that provides language-server features needs these assets:

```text
dowe-language-server-darwin-aarch64.tar.gz
dowe-language-server-darwin-x86_64.tar.gz
dowe-language-server-linux-aarch64.tar.gz
dowe-language-server-linux-x86_64.tar.gz
dowe-language-server-windows-aarch64.zip
dowe-language-server-windows-x86_64.zip
```

Each archive should contain the executable at its root:

- `dowe-language-server` for macOS and Linux.
- `dowe-language-server.exe` for Windows.

## Repository Layout

| Path | Purpose |
| --- | --- |
| `crates/language_server` | Builds the Rust `dowe-language-server` binary |
| `Cargo.toml` | Defines the local language-server workspace and any transitional editor tooling packages |
| `README.md` | Documents the active language-server workflow |

Legacy Zed extension scaffolding may exist in older checkouts of this repository. The active Zed extension workflow is `../dowe-zed`.
