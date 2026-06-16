use std::time::Instant;
use tokio::fs;

#[tokio::main]
async fn main() {
    let path = "test_vault";

    // Create test vault
    std::fs::create_dir_all(path).unwrap();
    for i in 0..1000 {
        let sub_path = format!("{}/dir_{}", path, i);
        std::fs::create_dir_all(&sub_path).unwrap();
        // Create an md file only in the last one
        if i == 999 {
            std::fs::write(format!("{}/test.md", sub_path), "test").unwrap();
        } else {
            std::fs::write(format!("{}/test.txt", sub_path), "test").unwrap();
        }
    }

    // Benchmark sync
    let start = Instant::now();
    let mut new_vault_has_md_files_sync = false;

    let path_buf = std::path::PathBuf::from(path);
    let vault_files: Vec<_> = std::fs::read_dir(&path_buf)
        .map(|entries| entries.filter_map(|e| e.ok()).collect())
        .unwrap_or_default();
    new_vault_has_md_files_sync = vault_files.iter().any(|entry: &std::fs::DirEntry| {
        entry.path().is_dir() && {
            std::fs::read_dir(entry.path())
                .map(|e| e.filter_map(|e| e.ok())
                    .any(|f| f.path().extension().map_or(false, |ext| ext == "md")))
                .unwrap_or(false)
        }
    });
    let sync_duration = start.elapsed();

    // Benchmark async
    let start = Instant::now();
    let mut new_vault_has_md_files_async = false;
    if let Ok(mut entries) = tokio::fs::read_dir(&path_buf).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let p = entry.path();
            if p.is_dir() {
                if let Ok(mut sub_entries) = tokio::fs::read_dir(&p).await {
                    while let Ok(Some(sub_entry)) = sub_entries.next_entry().await {
                        if sub_entry.path().extension().map_or(false, |ext| ext == "md") {
                            new_vault_has_md_files_async = true;
                            break;
                        }
                    }
                }
            }
            if new_vault_has_md_files_async {
                break;
            }
        }
    }
    let async_duration = start.elapsed();

    println!("Sync duration: {:?}", sync_duration);
    println!("Async duration: {:?}", async_duration);
    println!("Has md: {} (sync), {} (async)", new_vault_has_md_files_sync, new_vault_has_md_files_async);

    // Cleanup
    std::fs::remove_dir_all(path).unwrap();
}
