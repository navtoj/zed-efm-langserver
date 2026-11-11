use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct EFM;

impl zed::Extension for EFM {
	fn new() -> Self {
		Self
	}

	fn language_server_command(
		&mut self,
		language_server_id: &LanguageServerId,
		worktree: &zed::Worktree,
	) -> Result<zed::Command> {
		let binary = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
			.ok()
			.and_then(|s| s.binary);

		let command = binary
			.as_ref()
			.and_then(|b| b.path.clone())
			.or_else(|| worktree.which("efm-langserver"))
			.ok_or_else(|| "efm-langserver not found in PATH".to_string())?;

		let args = binary
			.as_ref()
			.and_then(|b| b.arguments.clone())
			.unwrap_or_default();

		let mut env = worktree.shell_env();
		if let Some(extra) = binary.and_then(|b| b.env) {
			env.extend(extra);
		}

		Ok(zed::Command { command, args, env })
	}

	fn language_server_workspace_configuration(
		&mut self,
		language_server_id: &LanguageServerId,
		worktree: &zed::Worktree,
	) -> Result<Option<zed::serde_json::Value>> {
		let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
			.ok()
			.and_then(|s| s.settings)
			.unwrap_or_else(|| zed::serde_json::json!({}));
		Ok(Some(settings))
	}
}

zed::register_extension!(EFM);
