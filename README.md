# mlir-zed

`mlir-zed` is a Zed extension repository that provides MLIR and TableGen language support.

For Zed itself, keep the extension metadata simple:

- Repository name: `mlir-zed`
- Extension id: `mlir`
- Extension name: `MLIR`

## What this first version does

- Recognizes `*.mlir` files as `MLIR`
- Recognizes `*.td` files as `TableGen`
- Loads the `tree-sitter-mlir` grammar from `artagnon/tree-sitter-mlir`
- Loads the `tree-sitter-tablegen` grammar from `Flakebi/tree-sitter-tablegen`
- Starts `mlir-lsp-server` for MLIR buffers
- Starts `tblgen-lsp-server` for TableGen buffers
- Allows the user to override the language server binary path, arguments, and environment through Zed LSP settings

## Repository layout

```text
.
├── extension.toml
├── Cargo.toml
├── src/lib.rs
└── languages/
    ├── mlir/
    │   ├── config.toml
    │   └── highlights.scm
    └── tablegen/
        ├── config.toml
        └── highlights.scm
```

## Local development

Prerequisites:

- Rust installed through `rustup`
- `mlir-lsp-server` available on `PATH`, or configured explicitly in Zed settings
- `tblgen-lsp-server` available on `PATH`, or configured explicitly in Zed settings

Suggested local flow:

1. Open Zed.
2. Run `zed: install dev extension`.
3. Select this repository.
4. Open a `.mlir` or `.td` file and confirm the language mode is `MLIR` or `TableGen`.

If your `mlir-lsp-server` or `tblgen-lsp-server` is not on `PATH`, configure it in Zed settings:

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

## Versioning notes

This scaffold uses `zed_extension_api = "0.7.0"`, which matches the current latest API docs at the time this repository was generated. If your installed Zed build is older, you may need to pin an earlier `zed_extension_api` version.
