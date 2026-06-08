use std::collections::HashMap;
use std::path::{Path, PathBuf};

use tauri::State;
use walkdir::WalkDir;

use crate::config::Config;
use crate::types::{AppState, FileEntry};

#[tauri::command]
pub fn read_file(state: State<'_, AppState>, path: String) -> Result<String, String> {
    let full_path = resolve_vault_path(&state.vault_path, &path)?;
    std::fs::read_to_string(&full_path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub fn save_file(state: State<'_, AppState>, path: String, content: String) -> Result<(), String> {
    let full_path = resolve_vault_path(&state.vault_path, &path)?;
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    std::fs::write(&full_path, &content).map_err(|e| format!("Failed to write file: {}", e))?;

    let config = state.config.blocking_read();
    if config.git.auto_commit {
        let mut git = state.git.blocking_write();
        let _ = git.auto_commit(&full_path, &content);
    }

    let vector = &state.vector;
    let _ = vector.index_document(&full_path, &content);

    Ok(())
}

#[tauri::command]
pub fn list_files(
    state: State<'_, AppState>,
    path: Option<String>,
) -> Result<Vec<FileEntry>, String> {
    let base = match path {
        Some(p) => resolve_vault_path(&state.vault_path, &p)?,
        None => state.vault_path.clone(),
    };

    let mut entries = Vec::new();
    let read_dir =
        std::fs::read_dir(&base).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in read_dir {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        if name.starts_with('.') {
            continue;
        }

        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            entries.push(FileEntry::Folder {
                name,
                path,
                children: Vec::new(),
            });
        } else {
            entries.push(FileEntry::File { name, path });
        }
    }

    entries.sort_by(|a, b| {
        let a_is_folder = matches!(a, FileEntry::Folder { .. });
        let b_is_folder = matches!(b, FileEntry::Folder { .. });
        b_is_folder.cmp(&a_is_folder).then_with(|| {
            let a_name = match a {
                FileEntry::File { name, .. } | FileEntry::Folder { name, .. } => name.clone(),
            };
            let b_name = match b {
                FileEntry::File { name, .. } | FileEntry::Folder { name, .. } => name.clone(),
            };
            a_name.to_lowercase().cmp(&b_name.to_lowercase())
        })
    });

    Ok(entries)
}

#[tauri::command]
pub fn create_file(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let full_path = resolve_vault_path(&state.vault_path, &path)?;
    if full_path.exists() {
        return Err("File already exists".into());
    }
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    std::fs::write(&full_path, "").map_err(|e| format!("Failed to create file: {}", e))
}

#[tauri::command]
pub fn create_folder(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let full_path = resolve_vault_path(&state.vault_path, &path)?;
    std::fs::create_dir_all(&full_path).map_err(|e| format!("Failed to create folder: {}", e))
}

#[tauri::command]
pub fn rename_file(
    state: State<'_, AppState>,
    old_path: String,
    new_path: String,
) -> Result<(), String> {
    let old_full = resolve_vault_path(&state.vault_path, &old_path)?;
    let new_full = resolve_vault_path(&state.vault_path, &new_path)?;
    std::fs::rename(&old_full, &new_full).map_err(|e| format!("Failed to rename: {}", e))
}

#[tauri::command]
pub fn delete_file(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let full_path = resolve_vault_path(&state.vault_path, &path)?;
    if full_path.is_dir() {
        std::fs::remove_dir_all(&full_path).map_err(|e| format!("Failed to delete folder: {}", e))
    } else {
        std::fs::remove_file(&full_path).map_err(|e| format!("Failed to delete file: {}", e))
    }
}

#[tauri::command]
pub fn duplicate_file(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let full_path = resolve_vault_path(&state.vault_path, &path)?;
    if !full_path.exists() {
        return Err("File does not exist".into());
    }
    let stem = full_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = full_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let parent = full_path.parent().unwrap_or(Path::new("."));
    let mut counter = 1;
    let new_path = loop {
        let name = format!(
            "{} (copy {}){}",
            stem,
            counter,
            if ext.is_empty() {
                String::new()
            } else {
                format!(".{}", ext)
            }
        );
        let candidate = parent.join(&name);
        if !candidate.exists() {
            break candidate;
        }
        counter += 1;
    };
    std::fs::copy(&full_path, &new_path).map_err(|e| format!("Failed to duplicate: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_tags(state: State<'_, AppState>) -> Result<Vec<(String, u32)>, String> {
    let vault = &state.vault_path;
    let mut tag_counts: HashMap<String, u32> = HashMap::new();

    for entry in WalkDir::new(vault).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" || ext == "markdown" {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        for line in content.lines() {
                            if let Some(tag_str) = line.strip_prefix("tags:") {
                                for tag in tag_str.split(',').map(|s| s.trim().to_lowercase()) {
                                    if !tag.is_empty() {
                                        *tag_counts.entry(tag).or_insert(0) += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut tags: Vec<(String, u32)> = tag_counts.into_iter().collect();
    tags.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(tags)
}

#[tauri::command]
pub fn get_recent_files(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let vault = &state.vault_path;
    let mut files: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();

    for entry in WalkDir::new(vault).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.path().metadata() {
                if let Ok(modified) = metadata.modified() {
                    files.push((entry.path().to_path_buf(), modified));
                }
            }
        }
    }

    files.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(files
        .into_iter()
        .take(20)
        .map(|(p, _)| p.to_string_lossy().to_string())
        .collect())
}

fn resolve_vault_path(vault_root: &Path, user_path: &str) -> Result<PathBuf, String> {
    let cleaned = user_path.trim_start_matches('/');
    let candidate = vault_root.join(cleaned);
    let canonical = candidate.canonicalize().unwrap_or(candidate);
    if !canonical.starts_with(vault_root) {
        return Err("Path traversal detected".into());
    }
    Ok(canonical)
}
