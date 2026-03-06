use std::collections::HashMap;
use std::path::{Path, PathBuf};

use zed::settings::LspSettings;
use zed::{
    set_language_server_installation_status as set_install_status, LanguageServerId,
    LanguageServerInstallationStatus as Status,
};
use zed_extension_api as zed;

const MLIR_LSP_ID: &str = "mlir-lsp";
const TBLGEN_LSP_ID: &str = "tblgen-lsp";
const DEFAULT_MLIR_LSP_BINARY: &str = "mlir-lsp-server";
const DEFAULT_TBLGEN_LSP_BINARY: &str = "tblgen-lsp-server";
const WORKTREE_BUILD_DIRS: &[&str] = &[
    "build",
    "../build",
    "llvm-project/build",
    "../llvm-project/build",
    "third_party/llvm-project/build",
];
const HOME_BUILD_DIRS: &[&str] = &[
    "github/llvm-project/build",
    "src/llvm-project/build",
    "llvm-project/build",
];

#[derive(Clone, Copy)]
struct LanguageServerSpec {
    id: &'static str,
    binary_name: &'static str,
    build_target: &'static str,
}

struct MlirExtension;

impl MlirExtension {
    fn server_spec(language_server_id: &str) -> Option<LanguageServerSpec> {
        match language_server_id {
            MLIR_LSP_ID => Some(LanguageServerSpec {
                id: MLIR_LSP_ID,
                binary_name: DEFAULT_MLIR_LSP_BINARY,
                build_target: DEFAULT_MLIR_LSP_BINARY,
            }),
            TBLGEN_LSP_ID => Some(LanguageServerSpec {
                id: TBLGEN_LSP_ID,
                binary_name: DEFAULT_TBLGEN_LSP_BINARY,
                build_target: DEFAULT_TBLGEN_LSP_BINARY,
            }),
            _ => None,
        }
    }

    fn lsp_settings(
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<LspSettings> {
        LspSettings::for_worktree(language_server_id.as_ref(), worktree)
    }

    fn lsp_command(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let spec = Self::server_spec(language_server_id.as_ref())
            .ok_or_else(|| format!("unknown language server: {}", language_server_id.as_ref()))?;
        let settings = Self::lsp_settings(language_server_id, worktree)?;
        let (args, env) = Self::settings_args_and_env(&settings);
        let path = self.resolve_binary_path(language_server_id, worktree, spec, &settings)?;

        Ok(zed::Command {
            command: path,
            args,
            env,
        })
    }

    fn settings_args_and_env(settings: &LspSettings) -> (Vec<String>, Vec<(String, String)>) {
        let args = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_default();
        let env = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.env.clone())
            .unwrap_or_default()
            .into_iter()
            .collect();

        (args, env)
    }

    fn resolve_binary_path(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
        spec: LanguageServerSpec,
        settings: &LspSettings,
    ) -> zed::Result<String> {
        if let Some(path) = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.path.as_ref())
            .cloned()
        {
            set_install_status(language_server_id, &Status::None);
            return Ok(path);
        }

        if let Some(path) = worktree.which(spec.binary_name) {
            set_install_status(language_server_id, &Status::None);
            return Ok(path);
        }

        if let Some(path) = self.find_binary_in_known_locations(worktree, spec) {
            set_install_status(language_server_id, &Status::None);
            return Ok(path);
        }

        let error = self.missing_binary_error(worktree, spec);
        set_install_status(language_server_id, &Status::Failed(error.clone()));
        Err(error)
    }

    fn find_binary_in_known_locations(
        &self,
        worktree: &zed::Worktree,
        spec: LanguageServerSpec,
    ) -> Option<String> {
        self.candidate_build_roots(worktree)
            .into_iter()
            .map(|build_root| build_root.join("bin").join(spec.binary_name))
            .find(|candidate| candidate.is_file())
            .map(path_to_string)
    }

    fn candidate_build_roots(&self, worktree: &zed::Worktree) -> Vec<PathBuf> {
        let worktree_root = PathBuf::from(worktree.root_path());
        let mut roots: Vec<PathBuf> = WORKTREE_BUILD_DIRS
            .iter()
            .map(|relative_dir| worktree_root.join(relative_dir))
            .collect();

        if let Some(home) = self.home_directory(worktree) {
            roots.extend(
                HOME_BUILD_DIRS
                    .iter()
                    .map(|relative_dir| home.join(relative_dir)),
            );
        }

        roots
    }

    fn home_directory(&self, worktree: &zed::Worktree) -> Option<PathBuf> {
        let shell_env: HashMap<_, _> = worktree.shell_env().into_iter().collect();
        shell_env.get("HOME").map(PathBuf::from)
    }

    fn missing_binary_error(&self, worktree: &zed::Worktree, spec: LanguageServerSpec) -> String {
        if let Some(build_root) = self.first_detected_build_root(worktree) {
            return format!(
                "could not find `{binary}`. Searched Zed LSP settings, $PATH, and common LLVM build directories. Found an LLVM build tree at `{build_root}`, but `{binary}` is missing. Build it with `ninja -C {build_root} {target}` or set `lsp.{server_id}.binary.path` in Zed settings.",
                binary = spec.binary_name,
                build_root = build_root.display(),
                target = spec.build_target,
                server_id = spec.id,
            );
        }

        format!(
            "could not find `{binary}`. Searched Zed LSP settings, $PATH, and common LLVM build directories near the current worktree (`build`, `../build`, `llvm-project/build`, `../llvm-project/build`, `third_party/llvm-project/build`) plus `~/github/llvm-project/build` and `~/src/llvm-project/build`. Set `lsp.{server_id}.binary.path` in Zed settings or build `{target}` in your LLVM tree.",
            binary = spec.binary_name,
            server_id = spec.id,
            target = spec.build_target,
        )
    }

    fn first_detected_build_root(&self, worktree: &zed::Worktree) -> Option<PathBuf> {
        self.candidate_build_roots(worktree)
            .into_iter()
            .find(|build_root| Self::looks_like_llvm_build_root(build_root))
    }

    fn looks_like_llvm_build_root(build_root: &Path) -> bool {
        build_root.join("build.ninja").is_file()
            || build_root.join("CMakeCache.txt").is_file()
            || build_root.join("bin").join("mlir-opt").is_file()
            || build_root.join("bin").join("mlir-tblgen").is_file()
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
        self.lsp_command(language_server_id, worktree)
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = Self::lsp_settings(language_server_id, worktree)?
            .initialization_options
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = Self::lsp_settings(language_server_id, worktree)?
            .settings
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

fn path_to_string(path: PathBuf) -> String {
    path.to_string_lossy().into_owned()
}

zed::register_extension!(MlirExtension);
