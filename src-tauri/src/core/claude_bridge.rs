use std::process::Stdio;
use tokio::io::AsyncBufReadExt;
use tokio::process::Command;
use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug)]
pub struct ClaudeOutput {
    pub text: String,
    pub is_error: bool,
}

/// Holds the session ID and whether it's the very first message.
pub struct ClaudeSession {
    pub session_id: String,
    pub is_first: bool,
}

pub type ClaudeState = Arc<Mutex<Option<ClaudeSession>>>;

pub fn init_claude_state() -> ClaudeState {
    Arc::new(Mutex::new(None))
}

#[tauri::command]
pub async fn start_claude_session(
    app: AppHandle,
    state: tauri::State<'_, ClaudeState>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    if guard.is_some() {
        return Ok(());
    }

    // Proactive Auth Check
    let auth_check = Command::new("claude")
        .args(&["auth", "status"])
        .output()
        .await;

    if let Ok(output) = auth_check {
        let out_str = String::from_utf8_lossy(&output.stdout);
        if out_str.contains("\"loggedIn\": false") || out_str.contains("Not logged in") {
            let _ = app.emit("claude-out", ClaudeOutput {
                text: "⚠️ Claude CLI is not logged in.\n\nPlease open your normal MacOS Terminal (outside of this app) and run:\n    claude auth login\n\nReturn here securely once done.\n".to_string(),
                is_error: true,
            });
            // Don't create session since they can't chat anyway
            return Ok(());
        }
    }

    let session_id = Uuid::new_v4().to_string();
    let _ = app.emit("claude-out", ClaudeOutput {
        text: format!("Session ready ({}). Type a message.\n", &session_id[..8]),
        is_error: false,
    });
    *guard = Some(ClaudeSession { session_id, is_first: true });
    Ok(())
}

/// Each message spawns `claude -p "message"` with the correct session argument
#[tauri::command]
pub async fn send_claude_input(
    app: AppHandle,
    state: tauri::State<'_, ClaudeState>,
    vault: tauri::State<'_, crate::core::vault::VaultManager>,
    text: String,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let (session_id, is_first) = match guard.as_mut() {
        Some(s) => {
            let id = s.session_id.clone();
            let first = s.is_first;
            s.is_first = false; // Next message will be a resume
            (id, first)
        },
        None => return Err("No session running".into()),
    };
    drop(guard); // release lock while the process runs

    let app_clone = app.clone();
    let vault_path = vault.vault_path.clone().unwrap_or_else(|| std::env::current_dir().unwrap());

    tauri::async_runtime::spawn(async move {
        let session_arg = if is_first {
            format!("--session-id {}", session_id)
        } else {
            format!("--resume {}", session_id)
        };

        let args = format!(
            "claude -p '{}' {} --output-format text --verbose < /dev/null",
            text.replace("'", "'\\''"),
            session_arg
        );

        let child = Command::new("sh")
            .arg("-c")
            .arg(&args)
            .current_dir(&vault_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(c) => c,
            Err(e) => {
                let _ = app_clone.emit("claude-out", ClaudeOutput {
                    text: format!("Failed to run claude: {}\n", e),
                    is_error: true,
                });
                return;
            }
        };

        // Stream stdout
        if let Some(stdout) = child.stdout.take() {
            let reader = tokio::io::BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_clone.emit("claude-out", ClaudeOutput {
                    text: format!("{}\n", line),
                    is_error: false,
                });
            }
        }

        // Collect stderr
        if let Some(stderr) = child.stderr.take() {
            let reader = tokio::io::BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_clone.emit("claude-out", ClaudeOutput {
                    text: format!("{}\n", line),
                    is_error: true,
                });
            }
        }

        let _ = child.wait().await;
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_claude_session(
    state: tauri::State<'_, ClaudeState>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    guard.take();
    Ok(())
}
