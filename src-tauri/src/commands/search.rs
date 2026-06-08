use tauri::State;
use walkdir::WalkDir;

use crate::types::{AppState, SearchResult};

#[derive(Debug, serde::Deserialize)]
pub struct SearchFilters {
    pub file_types: Option<Vec<String>>,
    pub max_size: Option<u64>,
    pub min_size: Option<u64>,
    pub modified_after: Option<String>,
    pub modified_before: Option<String>,
    pub case_sensitive: Option<bool>,
}

#[tauri::command]
pub fn search_files(
    state: State<'_, AppState>,
    query: String,
    scope: Option<String>,
    filters: Option<SearchFilters>,
) -> Result<Vec<SearchResult>, String> {
    let vault = &state.vault_path;
    let search_dir = match &scope {
        Some(s) => vault.join(s.trim_start_matches('/')),
        None => vault.clone(),
    };

    let case_sensitive = filters
        .as_ref()
        .and_then(|f| f.case_sensitive)
        .unwrap_or(false);
    let file_types = filters.as_ref().and_then(|f| f.file_types.clone());

    let query_lower = query.to_lowercase();
    let mut results: Vec<(SearchResult, u32)> = Vec::new();

    for entry in WalkDir::new(&search_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if let Some(types) = &file_types {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !types.iter().any(|t| t == ext || t == &format!(".{}", ext)) {
                continue;
            }
        }

        if let Some(ext) = path.extension() {
            if ext == "md" || ext == "markdown" || ext == "txt" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let content_for_search = if case_sensitive {
                        content.clone()
                    } else {
                        content.to_lowercase()
                    };
                    let search_term = if case_sensitive { &query } else { &query_lower };

                    if let Some(pos) = content_for_search.find(search_term) {
                        let snippet_start = pos.saturating_sub(80);
                        let snippet_end = (pos + query.len() + 80).min(content.len());
                        let snippet = if snippet_start > 0 {
                            format!("...{}...", &content[snippet_start..snippet_end])
                        } else {
                            content[snippet_start..snippet_end].to_string()
                        };

                        let count = content_for_search.matches(search_term).count() as u32;
                        let title = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();

                        results.push((
                            SearchResult {
                                path: path.to_path_buf(),
                                title,
                                snippet,
                                score: count as f64,
                            },
                            count,
                        ));
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(results.into_iter().map(|(r, _)| r).take(50).collect())
}
