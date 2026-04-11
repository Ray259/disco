use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use tokio::sync::mpsc;

use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;

/// Starts a background file watcher on the vault directory.
///
/// Watches for Create, Modify, and Remove events on `.md` files.
/// Uses a debounce channel to avoid processing intermediate saves.
/// Returns a handle that keeps the watcher alive.
pub fn start_watcher(
    vault: Arc<VaultManager>,
    db: Arc<EncyclopediaDb>,
) -> Result<WatcherHandle, String> {
    let vault_path = vault.vault_path.clone();
    
    // Channel for debounced events
    let (tx, mut rx) = mpsc::channel::<PathBuf>(100);
    
    // Create the notify watcher
    let tx_clone = tx.clone();
    let mut watcher = RecommendedWatcher::new(
        move |result: Result<notify::Event, notify::Error>| {
            if let Ok(event) = result {
                for path in event.paths {
                    if path.extension().map_or(false, |ext| ext == "md") {
                        // Skip hidden paths
                        if path.components().any(|c| c.as_os_str().to_str().map_or(false, |s| s.starts_with('.') && s != "." && s != "..")) {
                            continue;
                        }
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                                let _ = tx_clone.blocking_send(path);
                            }
                            _ => {}
                        }
                    }
                }
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    ).map_err(|e| {
        tracing::error!("Failed to create file watcher: {}", e);
        format!("Failed to create file watcher: {}", e)
    })?;
    
    if let Some(path) = &vault_path {
        tracing::info!(path = %path.display(), "Starting file watcher on vault");
        watcher.watch(path, RecursiveMode::Recursive)
            .map_err(|e| {
                tracing::error!(path = %path.display(), "Failed to watch vault: {}", e);
                format!("Failed to watch vault directory: {}", e)
            })?;
    }
    
    // Spawn async task to process file events with debouncing
    let vault_for_task = vault.clone();
    let db_for_task = db.clone();
    
    tokio::spawn(async move {
        // Simple debounce: collect events over 300ms windows
        loop {
            match rx.recv().await {
                Some(path) => {
                    // Wait a bit for additional events (debounce)
                    tokio::time::sleep(Duration::from_millis(300)).await;
                    
                    // Drain any queued events for this path
                    let mut paths_to_process = vec![path];
                    while let Ok(extra_path) = rx.try_recv() {
                        if !paths_to_process.contains(&extra_path) {
                            paths_to_process.push(extra_path);
                        }
                    }
                    
                    // Process each unique path
                    for p in paths_to_process {
                        if p.exists() {
                            // File was created or modified
                            match vault_for_task.sync_single_file(&p, &db_for_task).await {
                                Ok(()) => {
                                    tracing::info!(path = %p.display(), "File system event: Synced");
                                }
                                Err(e) => {
                                    tracing::error!(path = %p.display(), "Error syncing file: {}", e);
                                }
                            }
                        } else {
                            // File was deleted
                            match vault_for_task.handle_file_deleted(&p, &db_for_task).await {
                                Ok(_) => {
                                    tracing::info!(path = %p.display(), "File system event: Deleted");
                                }
                                Err(e) => {
                                    tracing::error!(path = %p.display(), "Error handling deletion: {}", e);
                                }
                            }
                        }
                    }
                }
                None => break, // Channel closed
            }
        }
    });

    Ok(WatcherHandle { _watcher: watcher })
}

/// Handle that keeps the file watcher alive.
/// Drop this to stop watching.
pub struct WatcherHandle {
    _watcher: RecommendedWatcher,
}
