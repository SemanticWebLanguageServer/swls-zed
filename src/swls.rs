use std::fs;

use zed_extension_api::{
    self as zed, LanguageServerId, Result, Worktree,
    serde_json::{Value, json},
    settings::{CommandSettings, LspSettings},
};

const LANGUAGE_SERVER_ID: &str = "swls";
const GITHUB_REPO: &str = "SemanticWebLanguageServer/swls";

struct SwlsBinary {
    path: String,
    args: Vec<String>,
}

struct SwlsExtension {
    cached_binary_path: Option<String>,
}

impl SwlsExtension {
    fn get_binary_settings(worktree: &Worktree) -> Option<CommandSettings> {
        LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|s| s.binary)
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<SwlsBinary> {
        let binary_settings = Self::get_binary_settings(worktree);

        let binary_args = binary_settings
            .as_ref()
            .and_then(|s| s.arguments.clone())
            .unwrap_or_default();

        // 1. User-configured binary path
        if let Some(path) = binary_settings.as_ref().and_then(|s| s.path.clone()) {
            return Ok(SwlsBinary {
                path,
                args: binary_args,
            });
        }

        // 2. Binary on PATH
        if let Some(path) = worktree.which(LANGUAGE_SERVER_ID) {
            return Ok(SwlsBinary {
                path,
                args: binary_args,
            });
        }

        // 3. Cached binary from previous download
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|s| s.is_file()) {
                return Ok(SwlsBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        // 4. Download from GitHub releases
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = match zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(_) => {
                // Try to find an existing downloaded binary
                if let Some(path) = find_existing_binary() {
                    self.cached_binary_path = Some(path.clone());
                    return Ok(SwlsBinary {
                        path,
                        args: binary_args,
                    });
                }
                return Err("failed to fetch latest GitHub release for swls".into());
            }
        };

        let (platform, arch) = zed::current_platform();

        let asset_name = format!(
            "swls-{os}-{arch}{ext}",
            os = match platform {
                zed::Os::Mac => "macos",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            arch = match (&platform, &arch) {
                (zed::Os::Mac, zed::Architecture::Aarch64) => "arm64",
                (_, zed::Architecture::Aarch64) => "aarch64",
                (_, zed::Architecture::X8664) => "x86_64",
                (_, zed::Architecture::X86) =>
                    return Err(format!("unsupported architecture: {arch:?}")),
            },
            ext = match platform {
                zed::Os::Windows => ".exe",
                _ => "",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("swls-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|e| format!("failed to create directory: {e}"))?;

        let binary_path = format!("{version_dir}/swls");

        if !fs::metadata(&binary_path).is_ok_and(|s| s.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            remove_outdated_versions(&version_dir)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(SwlsBinary {
            path: binary_path,
            args: binary_args,
        })
    }
}

impl zed::Extension for SwlsExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: binary.args,
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|s| s.initialization_options);

        if let Some(options) = settings {
            return Ok(Some(options));
        }

        // Default initialization options matching the VS Code extension defaults
        Ok(Some(json!({
            "log": "debug",
            "turtle": true,
            "trig": true,
            "sparql": false,
        })))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|s| s.settings);

        Ok(settings)
    }
}

fn find_existing_binary() -> Option<String> {
    fs::read_dir(".").ok()?.flatten().find_map(|entry| {
        let path = entry.path().join("swls");
        if path.is_file()
            && entry
                .file_name()
                .to_str()
                .is_some_and(|n| n.starts_with("swls-"))
        {
            Some(path.to_string_lossy().to_string())
        } else {
            None
        }
    })
}

fn remove_outdated_versions(current_version_dir: &str) -> Result<()> {
    let entries =
        fs::read_dir(".").map_err(|e| format!("failed to list working directory: {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("failed to load directory entry: {e}"))?;
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with("swls-") && name != current_version_dir {
                fs::remove_dir_all(entry.path()).ok();
            }
        }
    }
    Ok(())
}

zed::register_extension!(SwlsExtension);
