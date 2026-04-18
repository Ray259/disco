use std::io::Write;
use tauri::{AppHandle, Emitter, Manager};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Clone, Debug)]
pub struct CodexOutput {
    pub text: String,
    pub is_error: bool,
}

pub struct CodexSession {
    pub session_id: String,
    pub is_first: bool,
}

pub type CodexState = Arc<Mutex<Option<CodexSession>>>;

pub fn init_codex_state() -> CodexState {
    Arc::new(Mutex::new(None))
}

#[tauri::command]
pub async fn start_codex_session(
    app: AppHandle,
    state: tauri::State<'_, CodexState>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    if guard.is_some() {
        return Ok(());
    }

    let session_id = Uuid::new_v4().to_string();
    let display_id = if session_id.len() >= 8 { &session_id[..8] } else { &session_id };
    println!("[codex_bridge] start_codex_session: id={}", session_id);
    let _ = app.emit("codex-out", CodexOutput {
        text: format!("Codex Session ready ({}). Type a message.\n", display_id),
        is_error: false,
    });
    *guard = Some(CodexSession { session_id, is_first: true });
    Ok(())
}

#[tauri::command]
pub async fn resume_codex_session(
    app: AppHandle,
    state: tauri::State<'_, CodexState>,
    vault: tauri::State<'_, crate::core::vault::VaultManager>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    if guard.is_some() {
        return Ok(());
    }

    let session_id = "latest".to_string();
    println!("[codex_bridge] resume_codex_session called");
    let _ = app.emit("codex-out", CodexOutput {
        text: ">>> Resuming latest Codex session...\n".into(),
        is_error: false,
    });

    let app_clone = app.clone();
    let working_dir = match vault.vault_path.clone() {
        Some(path) => path,
        None => return Err("No vault path configured.".into()),
    };

    tauri::async_runtime::spawn(async move {
        let _ = load_latest_codex_history(app_clone, working_dir).await;
    });

    *guard = Some(CodexSession { session_id, is_first: false });
    Ok(())
}

async fn load_latest_codex_history(app: AppHandle, _working_dir: PathBuf) -> Result<(), String> {
    let mut root_session_dir = app.path().home_dir().map_err(|_| "No home dir")?;
    root_session_dir.push(".codex");
    root_session_dir.push("sessions");

    if !root_session_dir.exists() {
        return Ok(());
    }

    let project_name = _working_dir.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_lowercase();
    println!("[codex_bridge] >>> Scoping history search to project family: {}", project_name);

    use std::time::SystemTime;

    let mut dirs_to_visit = vec![root_session_dir];
    let mut latest_file = None;
    let mut latest_time = SystemTime::UNIX_EPOCH;

    while let Some(dir) = dirs_to_visit.pop() {
        if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_dir() {
                    dirs_to_visit.push(path);
                } else if path.extension().map_or(false, |e| e == "jsonl") {
                    if let Ok(metadata) = entry.metadata().await {
                        if let Ok(mtime) = metadata.modified() {
                            if mtime > latest_time {
                                latest_time = mtime;
                                latest_file = Some(path);
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(path) = latest_file {
        println!("[codex_bridge] >>> AUTO-RECOVERY: Loading latest session from: {:?}", path);
        let content = tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())?;
        
        for line in content.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                let msg_type = json.get("type").and_then(|v| v.as_str()).unwrap_or("");
                
                if msg_type == "event_msg" {
                    if let Some(payload) = json.get("payload") {
                        if payload.get("type").and_then(|v| v.as_str()) == Some("user_message") {
                            if let Some(text) = payload.get("message").and_then(|v| v.as_str()) {
                                let _ = app.emit("codex-out", CodexOutput {
                                    text: format!("> [YOU]: {}\n", text),
                                    is_error: false,
                                });
                            }
                        }
                    }
                } else if msg_type == "response_item" {
                    if let Some(payload) = json.get("payload") {
                        let inner_type = payload.get("type").and_then(|v| v.as_str()).unwrap_or("");
                        match inner_type {
                            "message" => {
                                if let Some(contents) = payload.get("content").and_then(|v| v.as_array()) {
                                    for item in contents {
                                        if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                                            let role = payload.get("role").and_then(|v| v.as_str()).unwrap_or("");
                                            let display_text = if role == "user" {
                                                format!("> [YOU]: {}\n", text)
                                            } else {
                                                format!("{}\n", text)
                                            };
                                            let _ = app.emit("codex-out", CodexOutput {
                                                text: display_text,
                                                is_error: false,
                                            });
                                        }
                                    }
                                }
                            },
                            "custom_tool_call" => {
                                let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Tool");
                                let input = payload.get("input").and_then(|v| v.as_str()).unwrap_or("");
                                let separator = "────────────────────────────────────────────────────────────────────────\n";
                                let content_str = format!("\n{}\n✦  {} : {}\n", separator, name, input);
                                let _ = app.emit("codex-out", CodexOutput { text: content_str, is_error: false });
                            },
                            "custom_tool_call_output" => {
                                let output = payload.get("output").and_then(|v| v.as_str()).unwrap_or("");
                                let separator = "────────────────────────────────────────────────────────────────────────\n";
                                let content_str = format!("✓  Result: {}\n{}\n", output, separator);
                                let _ = app.emit("codex-out", CodexOutput { text: content_str, is_error: false });
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        let _ = app.emit("codex-out", CodexOutput {
            text: ">>> [RE-ESTABLISHED FROM CACHE]\n".into(),
            is_error: false,
        });
    }

    Ok(())
}

#[tauri::command]
pub async fn send_codex_input(
    app: AppHandle,
    state: tauri::State<'_, CodexState>,
    vault: tauri::State<'_, crate::core::vault::VaultManager>,
    text: String,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let (session_id, is_first) = match guard.as_mut() {
        Some(s) => {
            let id = s.session_id.clone();
            let first = s.is_first;
            s.is_first = false; 
            (id, first)
        },
        None => return Err("No Codex session running".into()),
    };
    drop(guard);

    let app_clone = app.clone();
    let working_dir = match vault.vault_path.clone() {
        Some(path) => path,
        None => return Err("No vault path configured. Please set a vault path in settings first.".into()),
    };

    tauri::async_runtime::spawn(async move {
        let binary_path = "codex";
        
        let mut codex_args = vec!["exec".to_string()];

        if !is_first {
            codex_args.push("resume".to_string());
            if session_id == "latest" {
                codex_args.push("--last".to_string());
            } else {
                codex_args.push(session_id);
            }
        }

        codex_args.push("--json".to_string());
        codex_args.push("--skip-git-repo-check".to_string());
        codex_args.push(text.clone());

        println!("[codex_bridge] Running: {} {:?}", binary_path, codex_args);
        
        let child = tokio::process::Command::new(binary_path)
            .args(&codex_args)
            .current_dir(&working_dir)
            .env("TERM", "xterm-256color")
            .env("LANG", "en_US.UTF-8")
            .env("LC_ALL", "en_US.UTF-8")
            .env("FORCE_COLOR", "1")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(c) => {
                eprintln!("[codex_bridge] !!! SUCCESS: Child spawned with PID: {:?}", c.id());
                c
            },
            Err(e) => {
                eprintln!("[codex_bridge] !!! ERROR: Failed to spawn codex: {}", e);
                let _ = app_clone.emit("codex-out", CodexOutput {
                    text: format!("!!! [FATAL SPAWN ERROR]: {}\n", e),
                    is_error: true,
                });
                return;
            }
        };

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        if let Some(stdout) = stdout {
            let app_clone = app_clone.clone();
            tokio::spawn(async move {
                use tokio::io::AsyncBufReadExt;
                let mut reader = tokio::io::BufReader::new(stdout).lines();
                eprintln!("[codex_bridge] --- STDOUT JSON reader started.");
                
                while let Ok(Some(line)) = reader.next_line().await {
                    eprintln!("[codex_bridge] RECV JSON: {}", line);
                    let _ = std::io::stderr().flush();
                    
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                        let msg_type = json.get("type").and_then(|v| v.as_str()).unwrap_or("");
                        match msg_type {
                            "item.completed" => {
                                if let Some(item) = json.get("item") {
                                    if let Some(content) = item.get("text").and_then(|v| v.as_str()) {
                                        let _ = app_clone.emit("codex-out", CodexOutput {
                                            text: content.to_string(),
                                            is_error: false,
                                        });
                                    }
                                }
                            },
                            "turn.started" => {
                                // Potentially clear thinking state here
                            },
                            "error" => {
                                if let Some(content) = json.get("message").and_then(|v| v.as_str()) {
                                    let _ = app_clone.emit("codex-out", CodexOutput {
                                        text: format!("!!! [BRIDGE ERROR]: {}\n", content),
                                        is_error: true,
                                    });
                                }
                            },
                            _ => {}
                        }
                    } else if !line.trim().is_empty() {
                        // Fallback: if not JSON, just emit raw
                        let _ = app_clone.emit("codex-out", CodexOutput {
                            text: format!("{}\n", line),
                            is_error: false,
                        });
                    }
                }
            });
        }

        if let Some(mut stderr) = stderr {
            let app_clone = app_clone.clone();
            tokio::spawn(async move {
                eprintln!("[codex_bridge] --- STDERR listener started.");
                let mut buffer = [0u8; 8192];
                while let Ok(n) = tokio::io::AsyncReadExt::read(&mut stderr, &mut buffer).await {
                    if n == 0 { 
                        eprintln!("[codex_bridge] --- STDERR EOF reached.");
                        break; 
                    }
                    let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                    eprintln!("[codex_bridge] ERR RECV: {:?}", text);
                    let _ = std::io::stderr().flush();
                    let _ = app_clone.emit("codex-out", CodexOutput {
                        text,
                        is_error: true,
                    });
                }
            });
        }

        let status = child.wait().await;
        eprintln!("[codex_bridge] --- Process exited with status: {:?}", status);
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_codex_session(
    state: tauri::State<'_, CodexState>,
) -> Result<(), String> {
    println!("[codex_bridge] stop_codex_session called");
    let mut guard = state.lock().await;
    guard.take();
    Ok(())
}

#[tauri::command]
pub async fn get_codex_session_id(
    state: tauri::State<'_, CodexState>,
) -> Result<String, String> {
    let guard = state.lock().await;
    match guard.as_ref() {
        Some(s) => Ok(s.session_id.clone()),
        None => Err("No active Codex session".into()),
    }
}
