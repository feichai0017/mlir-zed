use zed::settings::LspSettings;
use zed_extension_api as zed;

const MLIR_LSP_ID: &str = "mlir-lsp";
const DEFAULT_MLIR_LSP_BINARY: &str = "mlir-lsp-server";

struct MlirExtension;

impl MlirExtension {
    fn mlir_lsp_command(&self, worktree: &zed::Worktree) -> zed::Result<zed::Command> {
        let settings = LspSettings::for_worktree(MLIR_LSP_ID, worktree)?;
        let mut command = zed::Command {
            command: DEFAULT_MLIR_LSP_BINARY.to_string(),
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
            MLIR_LSP_ID => self.mlir_lsp_command(worktree),
            other => Err(format!("unknown language server: {other}")),
        }
    }
}

zed::register_extension!(MlirExtension);
