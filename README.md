# Dowe LSP

Dowe LSP is the dedicated editor tooling workspace for Dowe Source Format files.

This repository contains the Rust `dowe-language-server` binary, the Zed extension adapter, language metadata, Tree-sitter queries, icon metadata, and the Dowe Tree-sitter grammar used by Zed. It recognizes `.dowe` files as `Dowe` and starts `dowe-language-server` over stdio.

Editor tooling is maintained here directly. It is not generated from, embedded in, or installed through the main Dowe repository.

## Requirements

- Zed with Rust extension development support.
- Rust installed through `rustup`.
- The `wasm32-wasip2` Rust target for local adapter builds.
- A sibling Dowe checkout at `../dowe` for local language-server builds against `dowe_compiler`.

The extension does not require Node.js, `node_modules`, npm, Prettier, or ESLint.

## Local Development

Install the Rust target used by Zed extensions if it is not already present:

```sh
rustup target add wasm32-wasip2
```

Prepare the local grammar mirror used by Zed dev extension installs:

```sh
./scripts/bootstrap-grammar-repo.sh
```

The bootstrap script creates `.zed-dev/tree-sitter-dowe.git` from the bundled `tree-sitter-dowe` directory and updates the grammar `rev` in `extension.toml`. Run it again after changing the grammar.

Build the extension adapter:

```sh
cargo check -p dowe-lsp-zed-extension --target wasm32-wasip2
```

Build or install the language server:

```sh
cargo check -p dowe_language_server
cargo install --path crates/language_server --debug --force
```

Install the extension in Zed with `zed: install dev extension` and select this repository directory.

The extension also provides the `Dowe Icons Dark` and `Dowe Icons Light` icon themes. Select one from Zed's icon theme selector to use the Dowe logo for `.dowe` files in the project panel.

Run local validation:

```sh
./scripts/check.sh
```

## View Syntax

The grammar recognizes the built-in `Input`, `Select`, and `Option` form controls. Completion and diagnostic support for their props is provided by `dowe-language-server`.

```text
Input label:"Email" placeholder:"name@example.com" labelFloating:true variant:"outlined" scheme:"primary"
Select label:"Role" placeholder:"Choose a role" labelFloating:true variant:"soft" scheme:"secondary"
  Option value:"admin" label:"Administrator" description:"Manage users"
  Option value:"viewer" label:"Viewer"
```

`Input` recognizes `label`, `placeholder`, and `labelFloating` through language-server support. `Select` recognizes the same visual props plus `bind`, `variant`, and `scheme`; direct `Option` children use `value`, `label`, and optional `description`. Every static string prop uses double quotes, including design tokens and enum values, so the language server flags `Option value:admin`, `Path fill:none`, `variant:outlined`, and `scheme:primary`. Resolved references such as `bind:profile.role` and `onClick:save` remain bare.

## Language Server

The published extension must not depend on a private Dowe checkout. The adapter uses an explicit Zed LSP binary setting when present, then a `dowe-language-server` binary on `PATH`, and otherwise asks Zed to download `dowe-language-server` from public release assets on `dowe-lang/dowe-lsp`.

Each release that should provide language-server features needs these assets:

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

For local development, install the current language server on `PATH` after compiler or language API changes:

```sh
cargo install --path crates/language_server --force
```

## Publishing

Before publishing to the Zed extension registry, `extension.toml` must point at a public grammar source. Zed allows a Tree-sitter grammar to live in a subdirectory by using `path = "tree-sitter-dowe"`.

After committing grammar changes, prepare the manifest for publication:

```sh
./scripts/prepare-publish.sh
```

This changes the grammar entry to use `https://github.com/dowe-lang/dowe-lsp`, the current `HEAD` commit, and `path = "tree-sitter-dowe"`.

Then open a PR to `zed-industries/extensions` that adds this repository as a submodule under `extensions/dowe` and adds the matching version to `extensions.toml`.

## Repository Layout

| Path | Purpose |
| --- | --- |
| `extension.toml` | Registers the Zed extension, grammar, and language server |
| `Cargo.toml` | Defines the editor tooling workspace and WebAssembly extension adapter |
| `crates/language_server` | Builds the Rust `dowe-language-server` binary |
| `src/lib.rs` | Starts `dowe-language-server` for Zed |
| `languages/dowe/config.toml` | Registers `.dowe`, tab size, and grammar metadata |
| `languages/dowe/*.scm` | Tree-sitter queries for highlighting, indentation, outline, text objects, and brackets |
| `icon_themes/dowe-icons.json` | Registers the Dowe file icon theme |
| `assets/logo.svg` | Provides the Dowe icon asset |
| `tree-sitter-dowe/grammar.js` | Tree-sitter grammar source |
| `tree-sitter-dowe/src/parser.c` | Generated Tree-sitter parser consumed by Zed |
| `scripts/bootstrap-grammar-repo.sh` | Builds the local git mirror used by Zed dev extension installs |
| `scripts/prepare-publish.sh` | Points the grammar at the public repository before publishing |
| `scripts/check.sh` | Runs local build and decoupling checks |
