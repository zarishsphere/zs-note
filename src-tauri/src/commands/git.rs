use crate::AppState;
use crate::types::*;
use tauri::State;

#[tauri::command]
pub fn git_commit(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let mut git = state.git.blocking_write();
    git.commit_all(&format!("Update {}", path))
        .map_err(|e| format!("Git commit failed: {}", e))
}

#[tauri::command]
pub fn git_history(state: State<'_, AppState>, path: String) -> Result<Vec<CommitEntry>, String> {
    let git = state.git.blocking_read();
    git.get_history(&path, 100)
        .map_err(|e| format!("Git history failed: {}", e))
}

#[tauri::command]
pub fn git_diff(
    state: State<'_, AppState>,
    path: String,
    rev1: String,
    rev2: String,
) -> Result<DiffResult, String> {
    let git = state.git.blocking_read();
    let diff_text = git
        .get_diff(&path, &rev1, &rev2)
        .map_err(|e| format!("Git diff failed: {}", e))?;

    let lines: Vec<&str> = diff_text.lines().collect();
    let mut hunks = Vec::new();
    let mut current_hunk_lines = Vec::new();

    for line in &lines {
        if let Some(hdr) = line.strip_prefix("@@ ") {
            if !current_hunk_lines.is_empty() {
                hunks.push(DiffHunk {
                    old_start: 0,
                    old_lines: 0,
                    new_start: 0,
                    new_lines: 0,
                    lines: std::mem::take(&mut current_hunk_lines),
                });
            }
        }

        let origin = if line.starts_with("+") {
            '+'
        } else if line.starts_with("-") {
            '-'
        } else {
            ' '
        };

        current_hunk_lines.push(DiffLine {
            origin,
            content: line.to_string(),
        });
    }

    if !current_hunk_lines.is_empty() {
        hunks.push(DiffHunk {
            old_start: 0,
            old_lines: 0,
            new_start: 0,
            new_lines: 0,
            lines: current_hunk_lines,
        });
    }

    Ok(DiffResult {
        old_path: path.clone(),
        new_path: path,
        hunks,
    })
}

#[tauri::command]
pub fn git_status(state: State<'_, AppState>) -> Result<GitStatus, String> {
    let git = state.git.blocking_read();
    git.get_status()
        .map_err(|e| format!("Git status failed: {}", e))
}

#[tauri::command]
pub fn git_push(state: State<'_, AppState>) -> Result<(), String> {
    let mut git = state.git.blocking_write();
    git.push().map_err(|e| format!("Git push failed: {}", e))
}

#[tauri::command]
pub fn git_pull(state: State<'_, AppState>) -> Result<(), String> {
    let mut git = state.git.blocking_write();
    git.pull().map_err(|e| format!("Git pull failed: {}", e))
}
