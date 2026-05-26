use std::fs;

use zed_extension_api::{
    self as zed, settings::LspSettings, LanguageServerId, LanguageServerInstallationStatus,
    Worktree,
};

const LANGUAGE_SERVER_BINARY: &str = "dowe-language-server";
const LANGUAGE_SERVER_REPOSITORY: &str = "dowe-lang/dowe-lsp";
const MANAGED_INSTALL_DIR: &str = "dowe-language-server";

struct DoweExtension {
    cached_binary_path: Option<String>,
}

impl DoweExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<DoweLanguageServerBinary> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.binary);
        let args = settings
            .as_ref()
            .and_then(|settings| settings.arguments.clone())
            .unwrap_or_default();

        if let Some(path) = settings.and_then(|settings| settings.path) {
            return Ok(DoweLanguageServerBinary { path, args });
        }

        let path = if let Some(path) = worktree.which(LANGUAGE_SERVER_BINARY) {
            path
        } else {
            self.zed_managed_binary_path(language_server_id)?
        };

        Ok(DoweLanguageServerBinary { path, args })
    }

    fn zed_managed_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            LANGUAGE_SERVER_REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let (asset_name, file_type) = release_asset_name()?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no release asset found matching {asset_name:?}"))?;

        let binary_name = binary_name();
        let version_dir = format!("{MANAGED_INSTALL_DIR}/{}", release.version);
        let binary_path = format!("{version_dir}/{binary_name}");

        if !fs::metadata(&binary_path).is_ok_and(|metadata| metadata.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|error| format!("failed to download {asset_name}: {error}"))?;
            zed::make_file_executable(&binary_path)
                .map_err(|error| format!("failed to make {binary_name} executable: {error}"))?;

            if let Ok(entries) = fs::read_dir(MANAGED_INSTALL_DIR) {
                for entry in entries.flatten() {
                    if entry.file_name().to_str() != Some(&release.version) {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );
        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

struct DoweLanguageServerBinary {
    path: String,
    args: Vec<String>,
}

fn release_asset_name() -> zed::Result<(String, zed::DownloadedFileType)> {
    let (os, architecture) = zed::current_platform();
    let (os, file_type, extension) = match os {
        zed::Os::Mac => ("darwin", zed::DownloadedFileType::GzipTar, "tar.gz"),
        zed::Os::Linux => ("linux", zed::DownloadedFileType::GzipTar, "tar.gz"),
        zed::Os::Windows => ("windows", zed::DownloadedFileType::Zip, "zip"),
    };
    let architecture = match architecture {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "x86_64",
        zed::Architecture::X86 => {
            return Err("x86 is not supported by dowe-language-server releases".to_string());
        }
    };

    Ok((
        format!("{LANGUAGE_SERVER_BINARY}-{os}-{architecture}.{extension}"),
        file_type,
    ))
}

fn binary_name() -> String {
    let (os, _) = zed::current_platform();
    match os {
        zed::Os::Windows => format!("{LANGUAGE_SERVER_BINARY}.exe"),
        zed::Os::Mac | zed::Os::Linux => LANGUAGE_SERVER_BINARY.to_string(),
    }
}

impl zed::Extension for DoweExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: binary.args,
            env: Vec::new(),
        })
    }
}

zed::register_extension!(DoweExtension);
