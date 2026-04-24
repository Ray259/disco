use serde::Serialize;
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncBufReadExt;
use tokio::process::Command;
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

    let session_id = format!("session-{}", chrono::Local::now().format("%Y-%m-%d-%H-%M"));
    println!("[claude_bridge] start_claude_session: id={}", session_id);
    let _ = app.emit(
        "claude-out",
        ClaudeOutput {
            text: format!(
                ">>> Established connection with Claude ({})...\n",
                session_id
            ),
            is_error: false,
        },
    );
    *guard = Some(ClaudeSession {
        session_id,
        is_first: true,
    });
    Ok(())
}

#[tauri::command]
pub async fn resume_claude_session(
    app: AppHandle,
    state: tauri::State<'_, ClaudeState>,
) -> Result<(), String> {
    println!("[claude_bridge] resume_claude_session called");
    let mut guard = state.lock().await;
    if guard.is_some() {
        return Ok(());
    }

    let session_id = "latest".to_string();
    let _ = app.emit(
        "claude-out",
        ClaudeOutput {
            text: ">>> Resuming latest Claude session...\n".into(),
            is_error: false,
        },
    );
    *guard = Some(ClaudeSession {
        session_id,
        is_first: false,
    });
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
        }
        None => return Err("No session running".into()),
    };
    drop(guard); // release lock while the process runs

    let app_clone = app.clone();
    let working_dir = match vault.vault_path.clone() {
        Some(path) => path,
        None => {
            return Err(
                "No vault path configured. Please set a vault path in settings first.".into(),
            )
        }
    };

    tauri::async_runtime::spawn(async move {
        let session_arg = if is_first {
            format!("--session-id {}", session_id)
        } else {
            format!("--resume {}", session_id)
        };

        let binary_path = "claude";
        println!(
            "[claude_bridge] --- Executing: script -q /dev/null {} -p '...' {}",
            binary_path, session_arg
        );

        let mut child = Command::new("script")
            .args(["-q", "/dev/null", binary_path])
            .args(&["-p", &text, &session_arg, "--verbose"])
            .current_dir(&working_dir)
            .env("TERM", "xterm-256color")
            .env("LANG", "en_US.UTF-8")
            .env("LC_ALL", "en_US.UTF-8")
            .env("FORCE_COLOR", "1")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(c) => c,
            Err(e) => {
                let _ = app_clone.emit(
                    "claude-out",
                    ClaudeOutput {
                        text: format!("Failed to run claude: {}\n", e),
                        is_error: true,
                    },
                );
                return;
            }
        };

        // Stream stdout
        if let Some(mut stdout) = child.stdout.take() {
            let app_clone = app_clone.clone();
            tokio::spawn(async move {
                eprintln!("[claude_bridge] --- STDOUT listener started.");
                let mut buffer = [0u8; 1024];
                while let Ok(n) = tokio::io::AsyncReadExt::read(&mut stdout, &mut buffer).await {
                    if n == 0 {
                        eprintln!("[claude_bridge] --- STDOUT EOF reached.");
                        break;
                    }
                    let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                    eprintln!("[claude_bridge] RECV {} bytes: {:?}", n, text);
                    let _ = app_clone.emit(
                        "claude-out",
                        ClaudeOutput {
                            text,
                            is_error: false,
                        },
                    );
                }
            });
        }

        // Stream stderr
        if let Some(mut stderr) = child.stderr.take() {
            let app_clone = app_clone.clone();
            tokio::spawn(async move {
                eprintln!("[claude_bridge] --- STDERR listener started.");
                let mut buffer = [0u8; 1024];
                while let Ok(n) = tokio::io::AsyncReadExt::read(&mut stderr, &mut buffer).await {
                    if n == 0 {
                        eprintln!("[claude_bridge] --- STDERR EOF reached.");
                        break;
                    }
                    let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                    eprintln!("[claude_bridge] ERR RECV: {:?}", text);
                    let _ = app_clone.emit(
                        "claude-out",
                        ClaudeOutput {
                            text,
                            is_error: true,
                        },
                    );
                }
            });
        }

        let status = child.wait().await;
        eprintln!(
            "[claude_bridge] --- Process exited with status: {:?}",
            status
        );
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_claude_session(state: tauri::State<'_, ClaudeState>) -> Result<(), String> {
    let mut guard = state.lock().await;
    guard.take();
    Ok(())
}

#[tauri::command]
pub async fn get_claude_session_id(state: tauri::State<'_, ClaudeState>) -> Result<String, String> {
    let guard = state.lock().await;
    match guard.as_ref() {
        Some(s) => Ok(s.session_id.clone()),
        None => Err("No active Claude session".into()),
    }
}
