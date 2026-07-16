//! Plug-and-play local English-to-Spanish translation for Zed.

pub mod acquisition;
pub mod diagnostics;
pub mod package;

use std::path::Path;

use acquisition::{Acquisition, HostPlatform, ZedDownloader};
use package::PublishedPackage;
use zed_extension_api as zed;

const LANGUAGE_SERVER_ID: &str = "en-es-translator";
const PACKAGE_LOCK: &str = include_str!("../../ops/marketplace/package.lock.json");

/// Zed extension entry point. Runtime state lives only in Zed's work directory.
pub struct EnEsTranslatorExtension {
    acquisition: Acquisition<ZedDownloader>,
}

impl zed::Extension for EnEsTranslatorExtension {
    fn new() -> Self {
        Self {
            acquisition: Acquisition::new(Path::new("."), ZedDownloader),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Err("Unsupported translation language server.".to_string());
        }
        let package = PublishedPackage::parse(PACKAGE_LOCK).map_err(|error| error.to_string())?;
        let platform = host_platform();
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let preparation = match self.acquisition.ready_command(&package, platform) {
            Ok(Some(command)) => Ok(command),
            Ok(None) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );
                self.acquisition.prepare(&package, platform)
            }
            Err(error) => Err(error),
        };
        let command = match preparation {
            Ok(command) => command,
            Err(error) => {
                let message = diagnostics::acquisition_message(error);
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::Failed(message.clone()),
                );
                return Err(message);
            }
        };
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(zed::Command {
            command: command.to_string_lossy().into_owned(),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

fn host_platform() -> HostPlatform {
    match zed::current_platform() {
        (zed::Os::Linux, zed::Architecture::X8664) => HostPlatform::LinuxX8664,
        (zed::Os::Linux, zed::Architecture::Aarch64) => HostPlatform::LinuxAarch64,
        (zed::Os::Linux, zed::Architecture::X86) => HostPlatform::LinuxX86,
        (zed::Os::Mac, zed::Architecture::Aarch64) => HostPlatform::MacAarch64,
        (zed::Os::Mac, zed::Architecture::X86) => HostPlatform::MacX86,
        (zed::Os::Mac, zed::Architecture::X8664) => HostPlatform::MacX8664,
        (zed::Os::Windows, zed::Architecture::Aarch64) => HostPlatform::WindowsAarch64,
        (zed::Os::Windows, zed::Architecture::X86) => HostPlatform::WindowsX86,
        (zed::Os::Windows, zed::Architecture::X8664) => HostPlatform::WindowsX8664,
    }
}

zed::register_extension!(EnEsTranslatorExtension);
