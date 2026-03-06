use zed::settings::LspSettings;
use zed_extension_api as zed;

const MLIR_LSP_ID: &str = "mlir-lsp";
const TBLGEN_LSP_ID: &str = "tblgen-lsp";
const DEFAULT_MLIR_LSP_BINARY: &str = "mlir-lsp-server";
const DEFAULT_TBLGEN_LSP_BINARY: &str = "tblgen-lsp-server";

struct MlirExtension;

impl MlirExtension {
    fn lsp_command(
        &self,
        language_server_id: &str,
        default_binary: &str,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let settings = LspSettings::for_worktree(language_server_id, worktree)?;
        let mut command = zed::Command {
            command: default_binary.to_string(),
            args: Vec::new(),
            env: Vec::new(),
        };

        if let Some(binary) = settings.binary {
            if let Some(path) = binary.path {
                command.command = path;
            }

            if let Some(arguments) = binary.arguments {
                command.args = arguments;
            }

            if let Some(env) = binary.env {
                command.env = env.into_iter().collect();
            }
        }

        Ok(command)
    }
}

impl zed::Extension for MlirExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            MLIR_LSP_ID => self.lsp_command(MLIR_LSP_ID, DEFAULT_MLIR_LSP_BINARY, worktree),
            TBLGEN_LSP_ID => self.lsp_command(TBLGEN_LSP_ID, DEFAULT_TBLGEN_LSP_BINARY, worktree),
            other => Err(format!("unknown language server: {other}")),
        }
    }
}

zed::register_extension!(MlirExtension);
