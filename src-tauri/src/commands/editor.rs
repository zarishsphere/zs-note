use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

use tauri::State;
use walkdir::WalkDir;

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

/// Write raw bytes to a file (used for drag-and-drop ingestion).
#[tauri::command]
pub fn write_file(
    state: State<'_, AppState>,
    path: String,
    content: Vec<u8>,
) -> Result<(), String> {
    let full_path = resolve_vault_path(&state.vault_path, &path)?;
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    std::fs::write(&full_path, &content).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

/// Get a temporary directory path for file operations.
#[tauri::command]
pub fn get_temp_dir(state: State<'_, AppState>) -> Result<String, String> {
    let temp_dir = state.vault_path.join(".znrc-temp");
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;
    Ok(temp_dir.to_string_lossy().to_string())
}

fn resolve_vault_path(vault_root: &Path, user_path: &str) -> Result<PathBuf, String> {
    if has_windows_prefix(user_path) {
        return Err("Path traversal detected".into());
    }

    let user_path = Path::new(user_path);
    let mut validated_relative = PathBuf::new();

    for component in user_path.components() {
        match component {
            Component::Normal(component) => validated_relative.push(component),
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err("Path traversal detected".into());
            }
        }
    }

    let canonical_root = vault_root
        .canonicalize()
        .map_err(|e| format!("Failed to resolve vault root: {}", e))?;
    let candidate = canonical_root.join(&validated_relative);

    if let Ok(canonical_candidate) = candidate.canonicalize() {
        if canonical_candidate.starts_with(&canonical_root) {
            return Ok(canonical_candidate);
        }
        return Err("Path traversal detected".into());
    }

    let mut existing_parent = candidate.as_path();
    let mut missing_components = Vec::new();

    while !existing_parent.exists() {
        let file_name = existing_parent
            .file_name()
            .ok_or_else(|| "Path traversal detected".to_string())?;
        missing_components.push(file_name.to_os_string());
        existing_parent = existing_parent
            .parent()
            .ok_or_else(|| "Path traversal detected".to_string())?;
    }

    let canonical_parent = existing_parent
        .canonicalize()
        .map_err(|e| format!("Failed to resolve path: {}", e))?;
    if !canonical_parent.starts_with(&canonical_root) {
        return Err("Path traversal detected".into());
    }

    let mut resolved = canonical_parent;
    for component in missing_components.iter().rev() {
        resolved.push(component);
    }

    Ok(resolved)
}

fn has_windows_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 2
        && ((bytes[0] as char).is_ascii_alphabetic() && bytes[1] == b':'
            || bytes.starts_with(br"\\"))
}

#[cfg(test)]
mod tests {
    use super::resolve_vault_path;

    #[test]
    fn rejects_parent_directory_traversal() {
        let vault = tempfile::tempdir().expect("create temp vault");

        let result = resolve_vault_path(vault.path(), "../outside.md");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_absolute_paths() {
        let vault = tempfile::tempdir().expect("create temp vault");

        let result = resolve_vault_path(vault.path(), "/tmp/file.md");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_windows_prefixes() {
        let vault = tempfile::tempdir().expect("create temp vault");

        let result = resolve_vault_path(vault.path(), r"C:\tmp\file.md");

        assert!(result.is_err());
    }

    #[test]
    fn resolves_nested_valid_paths() {
        let vault = tempfile::tempdir().expect("create temp vault");
        let nested_dir = vault.path().join("notes").join("daily");
        std::fs::create_dir_all(&nested_dir).expect("create nested directory");
        let note = nested_dir.join("today.md");
        std::fs::write(&note, "note").expect("write nested note");

        let result = resolve_vault_path(vault.path(), "notes/daily/today.md")
            .expect("resolve nested valid path");

        assert_eq!(result, note.canonicalize().expect("canonicalize note"));
    }

    #[test]
    fn resolves_non_existent_valid_files() {
        let vault = tempfile::tempdir().expect("create temp vault");
        let notes_dir = vault.path().join("notes");
        std::fs::create_dir_all(&notes_dir).expect("create notes directory");

        let result = resolve_vault_path(vault.path(), "notes/new.md")
            .expect("resolve non-existent valid file");

        assert_eq!(
            result,
            notes_dir
                .canonicalize()
                .expect("canonicalize notes dir")
                .join("new.md")
        );
    }
}
