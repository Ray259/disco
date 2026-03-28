use tauri::State;
use crate::core::vault::VaultManager;

#[tauri::command]
pub async fn launch_terminal_session(
    vault: State<'_, VaultManager>,
    provider: String,
    session_id: String,
) -> Result<(), String> {
    let vault_path = vault.vault_path.clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")))
        .to_string_lossy()
        .to_string();

    let cmd_str = if provider == "claude" {
        format!("claude --resume {}", session_id)
    } else if provider == "gemini" {
        format!("gemini --resume {}", session_id)
    } else {
        let codex_id = if session_id == "latest" { "--last".to_string() } else { session_id };
        format!("codex resume {}", codex_id)
    };

    let script = format!(
        "tell application \"Terminal\" to do script \"cd '{}' && clear && echo '🚀 Resuming {} session...' && {}\"",
        vault_path, provider, cmd_str
    );

    use std::io::Write;
    eprintln!("[terminal_cmd] >>> LAUNCHING NATIVE TERMINAL");
    eprintln!("[terminal_cmd] Path: {}", vault_path);
    eprintln!("[terminal_cmd] Script: {}", script);
    let _ = std::io::stderr().flush();

    std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .spawn()
        .map_err(|e| {
            eprintln!("[terminal_cmd] !!! ERROR spawning osascript: {}", e);
            let _ = std::io::stderr().flush();
            format!("Failed to launch terminal: {}", e)
        })?;

    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Terminal\" to activate")
        .spawn();

    Ok(())
}
