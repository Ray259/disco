use std::process::Stdio;
use tokio::process::Command;
use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::io::Write;

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
    let display_id = if session_id.len() >= 8 { &session_id[..8] } else { &session_id };
    println!("[gemini_bridge] start_gemini_session: id={}", session_id);
    let _ = app.emit("gemini-out", GeminiOutput {
        text: format!("Gemini Session ready ({}). Type a message.\n", display_id),
        is_error: false,
    });
    *guard = Some(GeminiSession { session_id, is_first: true });
    Ok(())
}

#[tauri::command]
pub async fn resume_gemini_session(
    app: AppHandle,
    state: tauri::State<'_, GeminiState>,
    vault: tauri::State<'_, crate::core::vault::VaultManager>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    if guard.is_some() {
        return Ok(());
    }

    let session_id = "latest".to_string();
    let _ = app.emit("gemini-out", GeminiOutput {
        text: ">>> Resuming latest Gemini session...\n".into(),
        is_error: false,
    });

    // Try to load history from the local gemini cache to fill the UI
    let app_clone = app.clone();
    let working_dir = match vault.vault_path.clone() {
        Some(path) => path,
        None => return Err("No vault path configured.".into()),
    };

    tauri::async_runtime::spawn(async move {
        let _ = load_latest_gemini_history(app_clone, working_dir).await;
    });

    *guard = Some(GeminiSession { session_id, is_first: false });
    Ok(())
}

async fn load_latest_gemini_history(app: AppHandle, _working_dir: std::path::PathBuf) -> Result<(), String> {
    use tauri::Manager;
    use std::fs;
    
    let mut root_session_dir = app.path().home_dir().map_err(|_| "No home dir")?;
    root_session_dir.push(".gemini");
    root_session_dir.push("tmp");

    if !root_session_dir.exists() {
        println!("[gemini_bridge] Root session dir missing: {:?}", root_session_dir);
        return Ok(());
    }

    let mut latest_file = None;
    let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

    let project_name = _working_dir.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_lowercase();
    println!("[gemini_bridge] >>> Scoping history search to project family: {}", project_name);

    if let Ok(mut projects) = tokio::fs::read_dir(&root_session_dir).await {
        while let Ok(Some(project)) = projects.next_entry().await {
            let name = project.file_name().to_string_lossy().to_lowercase();
            // Only look into projects that start with our name (case-insensitive)
            let is_match = name == project_name || name.starts_with(&format!("{}-", project_name));
            
            if !is_match {
                continue;
            }

            let mut chat_dir = project.path();
            chat_dir.push("chats");
            if chat_dir.exists() {
                if let Ok(mut sessions) = tokio::fs::read_dir(&chat_dir).await {
                    while let Ok(Some(session)) = sessions.next_entry().await {
                        if session.path().extension().map_or(false, |e| e == "json") {
                            if let Ok(metadata) = session.metadata().await {
                                if let Ok(mtime) = metadata.modified() {
                                    if mtime > latest_time {
                                        latest_time = mtime;
                                        latest_file = Some(session.path());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(path) = latest_file {
        println!("[gemini_bridge] >>> AUTO-RECOVERY: Loading true latest session from: {:?}", path);
        let content = tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())?;
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(messages) = json.get("messages").and_then(|v| v.as_array()) {
                for msg in messages {
                    let msg_type = msg.get("type").and_then(|v| v.as_str()).unwrap_or("system");
                    let content_val = msg.get("content");
                    
                    let text = if msg_type == "user" {
                        if let Some(content_arr) = content_val.and_then(|v| v.as_array()) {
                             content_arr.iter()
                                .find_map(|v| v.get("text").and_then(|t| t.as_str()))
                                .unwrap_or("")
                        } else {
                            ""
                        }
                    } else {
                        content_val.and_then(|v| v.as_str()).unwrap_or("")
                    };

                    // 1. Push the text intent first
                    if !text.is_empty() {
                        let display_text = if msg_type == "user" {
                            format!("> [YOU]: {}\n", text)
                        } else {
                            format!("{}\n", text)
                        };
                        let _ = app.emit("gemini-out", GeminiOutput {
                            text: display_text,
                            is_error: false,
                        });
                    }

                    // 2. Then push tool calls if this is a gemini message
                    if msg_type == "gemini" {
                        if let Some(tool_calls) = msg.get("toolCalls").and_then(|v| v.as_array()) {
                            for tc in tool_calls {
                                let display_name = tc.get("displayName").and_then(|v| v.as_str()).unwrap_or("Tool");
                                let description = tc.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                let result = tc.get("resultDisplay").and_then(|v| v.as_str()).unwrap_or("No output.");
                                
                                let separator = "────────────────────────────────────────────────────────────────────────\n";
                                let content_str = format!(
                                    "\n{}\
                                     ✓  {} {}\n\n\
                                     {}\n\
                                     {}\n",
                                    separator,
                                    display_name,
                                    description,
                                    result,
                                    separator
                                );
                                
                                let _ = app.emit("gemini-out", GeminiOutput {
                                    text: content_str,
                                    is_error: false,
                                });
                            }
                        }
                    }
                }
                let _ = app.emit("gemini-out", GeminiOutput {
                    text: ">>> [RE-ESTABLISHED FROM CACHE]\n".into(),
                    is_error: false,
                });
            }
        }
    } else {
        println!("[gemini_bridge] No history files found anywhere in ~/.gemini/tmp/");
        let _ = app.emit("gemini-out", GeminiOutput {
            text: ">>> [NO PREVIOUS SESSIONS FOUND]\n".into(),
            is_error: false,
        });
    }

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
    let (session_id, is_first) = match guard.as_mut() {
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
    let working_dir = match vault.vault_path.clone() {
        Some(path) => path,
        None => return Err("No vault path configured. Please set a vault path in settings first.".into()),
    };

    use std::io::Write;
    eprintln!("[gemini_bridge] >>> send_gemini_input CALLED.");
    eprintln!("[gemini_bridge] Session: {}, First: {}", session_id, is_first);
    let _ = std::io::stderr().flush();

    tauri::async_runtime::spawn(async move {
        let binary_path = "gemini";
        
        // Build arguments conditionally
        let mut gemini_args = vec![
            "-p".to_string(),
            text,
            "--output-format".to_string(),
            "stream-json".to_string(),
            "--raw-output".to_string(),
            "--accept-raw-output-risk".to_string(),
        ];

        if !is_first {
            gemini_args.push("-r".to_string());
            gemini_args.push(session_id);
        }

        eprintln!("[gemini_bridge] !!! ATTEMPTING DIRECT JSON EXECUTION: {:?} {:?}", binary_path, gemini_args);
        let _ = std::io::stderr().flush();

        let mut child = Command::new(binary_path)
            .args(&gemini_args)
            .current_dir(&working_dir)
            .env("TERM", "xterm-256color")
            .env("LANG", "en_US.UTF-8")
            .env("LC_ALL", "en_US.UTF-8")
            .env("FORCE_COLOR", "1")
            .env("CLICOLOR_FORCE", "1")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(c) => {
                eprintln!("[gemini_bridge] !!! SUCCESS: Child spawned with PID: {:?}", c.id());
                let _ = std::io::stderr().flush();
                c
            },
            Err(e) => {
                eprintln!("[gemini_bridge] !!! ERROR: Failed to spawn gemini: {}", e);
                let _ = std::io::stderr().flush();
                let _ = app_clone.emit("gemini-out", GeminiOutput {
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
                eprintln!("[gemini_bridge] --- STDOUT JSON reader started.");
                
                while let Ok(Some(line)) = reader.next_line().await {
                    eprintln!("[gemini_bridge] RECV JSON: {}", line);
                    let _ = std::io::stderr().flush();
                    
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                        let msg_type = json.get("type").and_then(|v| v.as_str()).unwrap_or("");
                        match msg_type {
                            "message" => {
                                if let Some(content) = json.get("content").and_then(|v| v.as_str()) {
                                    let _ = app_clone.emit("gemini-out", GeminiOutput {
                                        text: content.to_string(),
                                        is_error: false,
                                    });
                                }
                            },
                            "tool_use" => {
                                let tool_name = json.get("tool_name").and_then(|v| v.as_str()).unwrap_or("Tool");
                                let params = json.get("parameters").map(|v| v.to_string()).unwrap_or_default();
                                
                                let separator = "────────────────────────────────────────────────────────────────────────\n";
                                let content_str = format!(
                                    "\n{}\
                                     ✦  {} : {}\n",
                                    separator,
                                    tool_name,
                                    params
                                );
                                let _ = app_clone.emit("gemini-out", GeminiOutput {
                                    text: content_str,
                                    is_error: false,
                                });
                            },
                            "tool_result" => {
                                let status = json.get("status").and_then(|v| v.as_str()).unwrap_or("done");
                                let separator = "────────────────────────────────────────────────────────────────────────\n";
                                let content_str = format!(
                                    "✓  Status: {}\n{}\n",
                                    status,
                                    separator
                                );
                                let _ = app_clone.emit("gemini-out", GeminiOutput {
                                    text: content_str,
                                    is_error: false,
                                });
                            },
                            "error" => {
                                if let Some(content) = json.get("content").and_then(|v| v.as_str()) {
                                    let _ = app_clone.emit("gemini-out", GeminiOutput {
                                        text: content.to_string(),
                                        is_error: true,
                                    });
                                }
                            },
                            _ => {}
                        }
                    } else if !line.trim().is_empty() {
                        // Fallback: if not JSON, just emit raw
                        let _ = app_clone.emit("gemini-out", GeminiOutput {
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
                eprintln!("[gemini_bridge] --- STDERR listener started.");
                let mut buffer = [0u8; 8192];
                while let Ok(n) = tokio::io::AsyncReadExt::read(&mut stderr, &mut buffer).await {
                    if n == 0 { 
                        eprintln!("[gemini_bridge] --- STDERR EOF reached.");
                        break; 
                    }
                    let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                    eprintln!("[gemini_bridge] ERR RECV: {:?}", text);
                    let _ = std::io::stderr().flush();
                    let _ = app_clone.emit("gemini-out", GeminiOutput {
                        text,
                        is_error: true,
                    });
                }
            });
        }

        let status = child.wait().await;
        eprintln!("[gemini_bridge] --- Process exited with status: {:?}", status);
        let _ = std::io::stderr().flush();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        eprintln!("[gemini_bridge] --- Session cycle completed.");
        let _ = std::io::stderr().flush();
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_gemini_session(
    state: tauri::State<'_, GeminiState>,
) -> Result<(), String> {
    println!("[gemini_bridge] stop_gemini_session called");
    let mut guard = state.lock().await;
    guard.take();
    Ok(())
}

#[tauri::command]
pub async fn get_gemini_session_id(
    state: tauri::State<'_, GeminiState>,
) -> Result<String, String> {
    let guard = state.lock().await;
    match guard.as_ref() {
        Some(s) => Ok(s.session_id.clone()),
        None => Err("No active Gemini session".into()),
    }
}
