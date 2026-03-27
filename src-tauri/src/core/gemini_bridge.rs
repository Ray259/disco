use std::process::Stdio;
use tokio::io::AsyncBufReadExt;
use tokio::process::Command;
use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug)]
pub struct GeminiOutput {
    pub text: String,
    pub is_error: bool,
}

pub struct GeminiSession {
    pub session_id: String,
    pub is_first: bool,
}

pub type GeminiState = Arc<Mutex<Option<GeminiSession>>>;

pub fn init_gemini_state() -> GeminiState {
    Arc::new(Mutex::new(None))
}

#[tauri::command]
pub async fn start_gemini_session(
    app: AppHandle,
    state: tauri::State<'_, GeminiState>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    if guard.is_some() {
        return Ok(());
    }

    let session_id = Uuid::new_v4().to_string();
    let _ = app.emit("gemini-out", GeminiOutput {
        text: format!("Gemini Session ready ({}). Type a message.\n", &session_id[..8]),
        is_error: false,
    });
    *guard = Some(GeminiSession { session_id, is_first: true });
    Ok(())
}

#[tauri::command]
pub async fn send_gemini_input(
    app: AppHandle,
    state: tauri::State<'_, GeminiState>,
    vault: tauri::State<'_, crate::core::vault::VaultManager>,
    text: String,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let (_session_id, is_first) = match guard.as_mut() {
        Some(s) => {
            let id = s.session_id.clone();
            let first = s.is_first;
            s.is_first = false; 
            (id, first)
        },
        None => return Err("No Gemini session running".into()),
    };
    drop(guard);

    let app_clone = app.clone();
    let vault_path = vault.vault_path.clone().unwrap_or_else(|| std::env::current_dir().unwrap());

    tauri::async_runtime::spawn(async move {
        // We use `--resume latest` for all follow-up messages to maintain continuity.
        // For the first message, we just run without resume flags.
        let args = if is_first {
            format!("gemini -p '{}' --output-format text < /dev/null", text.replace("'", "'\\''"))
        } else {
            format!("gemini -p '{}' --resume latest --output-format text < /dev/null", text.replace("'", "'\\''"))
        };

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
                let _ = app_clone.emit("gemini-out", GeminiOutput {
                    text: format!("Failed to run gemini: {}\n", e),
                    is_error: true,
                });
                return;
            }
        };

        if let Some(stdout) = child.stdout.take() {
            let reader = tokio::io::BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_clone.emit("gemini-out", GeminiOutput {
                    text: format!("{}\n", line),
                    is_error: false,
                });
            }
        }

        if let Some(stderr) = child.stderr.take() {
            let reader = tokio::io::BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_clone.emit("gemini-out", GeminiOutput {
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
pub async fn stop_gemini_session(
    state: tauri::State<'_, GeminiState>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    guard.take();
    Ok(())
}
