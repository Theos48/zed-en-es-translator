//! Local Zed MCP wrapper for the existing `translator-mcp` server.

pub mod diagnostics;
pub mod launch;
pub mod settings;

use std::collections::HashMap;

use launch::{build_launch_profile, CONTEXT_SERVER_ID};
use settings::{CommandSettingsInput, LaunchSettings};
use zed_extension_api as zed;

/// Zed extension entry point.
pub struct EnEsTranslatorExtension;

impl zed::Extension for EnEsTranslatorExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &zed::ContextServerId,
        project: &zed::Project,
    ) -> zed::Result<zed::Command> {
        let zed_settings =
            zed::settings::ContextServerSettings::for_project(context_server_id.as_ref(), project)
                .map_err(redact_internal_error)?;
        let command = command_settings_input(zed_settings.command);
        let launch_settings = LaunchSettings::from_parts(zed_settings.settings.as_ref(), command)
            .map_err(|event| event.to_user_message())?;
        let profile = build_launch_profile(context_server_id.as_ref(), &launch_settings)
            .map_err(|event| event.to_user_message())?;

        Ok(zed::Command {
            command: profile.command,
            args: profile.args,
            env: profile.env,
        })
    }

    fn context_server_configuration(
        &mut self,
        context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> zed::Result<Option<zed::ContextServerConfiguration>> {
        if context_server_id.as_ref() != CONTEXT_SERVER_ID {
            return Ok(None);
        }

        Ok(Some(zed::ContextServerConfiguration {
            installation_instructions: installation_instructions(),
            settings_schema: settings_schema(),
            default_settings: default_settings(),
        }))
    }
}

fn command_settings_input(command: Option<zed::settings::CommandSettings>) -> CommandSettingsInput {
    let Some(command) = command else {
        return CommandSettingsInput::default();
    };

    let env = hashmap_to_sorted_pairs(command.env.unwrap_or_default());

    CommandSettingsInput::new(command.path, command.arguments.unwrap_or_default(), env)
}

/// Convert Zed's unordered command environment map into a deterministic,
/// sorted list of pairs. Sorting happens here (not in the caller) so the
/// function name and behavior stay in sync: callers must not have to
/// remember to sort separately, or determinism guarantees like
/// `repeated_startup_failure_revalidates_without_state` could silently break.
fn hashmap_to_sorted_pairs(values: HashMap<String, String>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = values.into_iter().collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
}

fn redact_internal_error(_error: String) -> String {
    crate::diagnostics::diagnostic_with_action(
        crate::diagnostics::DiagnosticPhase::Configuration,
        crate::diagnostics::DiagnosticCode::InternalExtensionError,
        "Zed context server settings could not be loaded.",
    )
    .to_user_message()
}

fn installation_instructions() -> String {
    "Run `make zed-extension-prepare`, then set `binary_path` to the printed translator-mcp artifact path.".to_string()
}

fn settings_schema() -> String {
    r#"{
  "type": "object",
  "additionalProperties": false,
  "required": ["binary_path"],
  "properties": {
    "binary_path": {
      "type": "string",
      "description": "Absolute path printed by make zed-extension-prepare for target/release/translator-mcp."
    },
    "provider": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "mode": {
          "type": "string",
          "enum": ["mock", "libretranslate"],
          "default": "mock"
        },
        "url": {
          "type": "string",
          "default": ""
        },
        "api_key_env": {
          "type": "string",
          "default": ""
        },
        "allow_remote": {
          "type": "boolean",
          "default": false
        }
      }
    }
  }
}"#
    .to_string()
}

fn default_settings() -> String {
    r#"{
  "binary_path": "",
  "provider": {
    "mode": "mock",
    "url": "",
    "api_key_env": "",
    "allow_remote": false
  }
}"#
    .to_string()
}

zed::register_extension!(EnEsTranslatorExtension);
