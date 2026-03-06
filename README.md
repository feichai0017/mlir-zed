# mlir-zed

`mlir-zed` is a Zed extension repository that provides MLIR and TableGen language support for Zed.

For Zed itself, keep the extension metadata simple:

- Repository name: `mlir-zed`
- Extension id: `mlir`
- Extension name: `MLIR`

## What this extension does

- Recognizes `*.mlir` files as `MLIR`
- Recognizes `*.td` files as `TableGen`
- Loads the `tree-sitter-mlir` grammar from `artagnon/tree-sitter-mlir`
- Loads the `tree-sitter-tablegen` grammar from `Flakebi/tree-sitter-tablegen`
- Provides syntax highlighting, bracket matching, indentation hints, and outline support for both languages

## What this extension does not do

- It does not bundle or launch `mlir-lsp-server`
- It does not bundle or launch `tblgen-lsp-server`
- It does not attempt semantic understanding of project-specific dialects
- It does not manage LLVM or MLIR build artifacts

## Repository layout

```text
.
├── extension.toml
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

There is no Rust runtime in this extension and no bundled language server integration.

## Local development

Prerequisites:

- Zed
- This repository checked out locally

Suggested local flow:

1. Open Zed.
2. Run `zed: install dev extension`.
3. Select this repository.
4. Open a `.mlir` file and confirm the language mode is `MLIR`.
5. Open a `.td` file and confirm the language mode is `TableGen`.

## Scope and Design

This extension intentionally stays at the syntax layer.

That means it is a good fit when you want:

- file type detection
- readable highlighting
- basic editor structure support

It is intentionally not responsible for:

- dialect-aware completion
- project-specific semantic analysis
- locating or installing LSP servers

This keeps the extension stable for both upstream LLVM users and projects with custom MLIR/TableGen dialects.
