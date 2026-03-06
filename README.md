# mlir-zed

`mlir-zed` is a Zed extension repository that provides MLIR language support.

For Zed itself, keep the extension metadata simple:

- Repository name: `mlir-zed`
- Extension id: `mlir`
- Extension name: `MLIR`

## What this first version does

- Recognizes `*.mlir` files as `MLIR`
- Loads the `tree-sitter-mlir` grammar from `drom/tree-sitter-mlir`
- Starts `mlir-lsp-server` for MLIR buffers
- Allows the user to override the language server binary path, arguments, and environment through Zed LSP settings

## Repository layout

```text
.
├── extension.toml
├── Cargo.toml
├── src/lib.rs
└── languages/mlir/
    ├── config.toml
    └── highlights.scm
```

## Local development

Prerequisites:

- Rust installed through `rustup`
- `mlir-lsp-server` available on `PATH`, or configured explicitly in Zed settings

Suggested local flow:

1. Open Zed.
2. Run `zed: install dev extension`.
3. Select this repository.
4. Open a `.mlir` file and confirm the language mode is `MLIR`.

If your `mlir-lsp-server` is not on `PATH`, configure it in Zed settings:

```json
{
  "lsp": {
    "mlir-lsp": {
      "binary": {
        "path": "/absolute/path/to/mlir-lsp-server",
        "arguments": []
      }
    }
  }
}
```

## Publishing to the Zed extension registry

The normal path is to publish into the registry, not to move the repository under direct Zed team ownership.

Current registry flow:

1. Push this repository to GitHub.
2. Fork `https://github.com/zed-industries/extensions`.
3. Add this repository there as a git submodule under `extensions/mlir`.
4. Register the extension in the top-level `extensions.toml`.
5. Run `pnpm sort-extensions`.
6. Open a PR to `zed-industries/extensions`.

Important details:

- Use an HTTPS submodule URL in the registry PR, even if your personal clone uses SSH.
- Keep a root `LICENSE` file in this repository, or the registry checks will fail.
- Getting into the registry is the standard supported route. It does not automatically mean the extension becomes Zed-team-maintained.

## Versioning notes

This scaffold uses `zed_extension_api = "0.7.0"`, which matches the current latest API docs at the time this repository was generated. If your installed Zed build is older, you may need to pin an earlier `zed_extension_api` version.
