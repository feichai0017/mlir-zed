# mlir-zed

`mlir-zed` is a Zed extension repository that provides MLIR and TableGen language and LSP support.

For Zed itself, keep the extension metadata simple:

- Repository name: `mlir-zed`
- Extension id: `mlir`
- Extension name: `MLIR`

## What this extension does

- Recognizes `*.mlir` files as `MLIR`
- Recognizes `*.td` files as `TableGen`
- Loads the `tree-sitter-mlir` grammar from `artagnon/tree-sitter-mlir`
- Loads the `tree-sitter-tablegen` grammar from `Flakebi/tree-sitter-tablegen`
- Loads bracket matching, indentation, and outline queries for both languages
- Starts `mlir-lsp-server` for MLIR buffers
- Starts `tblgen-lsp-server` for TableGen buffers
- Resolves language servers in this order:
  1. `lsp.<server>.binary.path` from Zed settings
  2. binaries on `PATH`
  3. common LLVM build directories near the current worktree
  4. common LLVM build directories under `~/github/llvm-project/build` and `~/src/llvm-project/build`
- Allows the user to override the language server binary path, arguments, environment, initialization options, and workspace settings through Zed LSP settings

## Repository layout

```text
.
├── extension.toml
├── Cargo.toml
├── src/lib.rs
└── languages/
    ├── mlir/
    │   ├── brackets.scm
    │   ├── config.toml
    │   ├── highlights.scm
    │   ├── indents.scm
    │   └── outline.scm
    └── tablegen/
        ├── brackets.scm
        ├── config.toml
        ├── highlights.scm
        ├── indents.scm
        └── outline.scm
```

## Local development

Prerequisites:

- Rust installed through `rustup`
- One of:
  - `mlir-lsp-server` and `tblgen-lsp-server` on `PATH`
  - explicit Zed settings for both binaries
  - an LLVM build tree in one of the extension's known locations

Suggested local flow:

1. Open Zed.
2. Run `zed: install dev extension`.
3. Select this repository.
4. Open a `.mlir` or `.td` file and confirm the language mode is `MLIR` or `TableGen`.

If your servers are not on `PATH`, the extension will also look in:

- `<workspace>/build/bin`
- `<workspace>/../build/bin`
- `<workspace>/llvm-project/build/bin`
- `<workspace>/../llvm-project/build/bin`
- `<workspace>/third_party/llvm-project/build/bin`
- `~/github/llvm-project/build/bin`
- `~/src/llvm-project/build/bin`

If it finds an LLVM build tree but not the server binary, Zed will show a concrete error with the suggested `ninja -C ... mlir-lsp-server` or `tblgen-lsp-server` command.

You can always override discovery in Zed settings:

```json
{
  "lsp": {
    "mlir-lsp": {
      "binary": {
        "path": "/absolute/path/to/mlir-lsp-server",
        "arguments": []
      }
    },
    "tblgen-lsp": {
      "binary": {
        "path": "/absolute/path/to/tblgen-lsp-server",
        "arguments": []
      }
    }
  }
}
```

If you build LLVM/MLIR yourself, the typical command is:

```bash
ninja -C /path/to/llvm-project/build mlir-lsp-server tblgen-lsp-server
```

`tblgen-lsp-server` also expects the relevant TableGen compilation database in your build tree, typically `tablegen_compile_commands.yml`.

## Versioning notes

This scaffold uses `zed_extension_api = "0.7.0"`, which matches the current latest API docs at the time this repository was generated. If your installed Zed build is older, you may need to pin an earlier `zed_extension_api` version.
