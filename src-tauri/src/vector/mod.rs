pub mod index;
pub mod query;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::Utc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::types::{IndexStats, SearchResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIndex {
    pub path: PathBuf,
    pub chunks: Vec<IndexChunk>,
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexChunk {
    pub id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub start_offset: usize,
    pub end_offset: usize,
}

pub struct VectorStore {
    db_path: PathBuf,
    index: Arc<RwLock<HashMap<String, DocumentIndex>>>,
    table_path: PathBuf,
}

impl VectorStore {
    pub fn new(vault_root: &Path) -> Self {
        let db_path = vault_root.join(".znrc-vector");
        let table_path = vault_root.join(".znrc-vector").join("documents");

        std::fs::create_dir_all(&db_path).ok();

        Self {
            db_path,
            index: Arc::new(RwLock::new(HashMap::new())),
            table_path,
        }
    }

    pub fn index_document(&self, path: &Path, content: &str) -> Result<()> {
        let checksum = compute_checksum(content);
        let file_path = path.to_string_lossy().to_string();

        {
            let index = self.index.read();
            if let Some(existing) = index.get(&file_path) {
                if existing.checksum == checksum {
                    return Ok(());
                }
            }
        }

        let chunks = index::chunk_text(content, 512, 64);

        let mut doc_chunks = Vec::new();
        for (i, chunk) in chunks.iter().enumerate() {
            doc_chunks.push(IndexChunk {
                id: format!("{}:{}", file_path, i),
                content: chunk.clone(),
                embedding: None,
                start_offset: 0,
                end_offset: chunk.len(),
            });
        }

        let doc_index = DocumentIndex {
            path: path.to_path_buf(),
            chunks: doc_chunks,
            indexed_at: Utc::now(),
            checksum,
        };

        let mut index = self.index.write();
        index.insert(file_path, doc_index);

        info!("Indexed document: {:?}", path);
        Ok(())
    }

    pub fn query(&self, query_text: &str, _kb_name: &str, top_k: usize) -> Vec<SearchResult> {
        let query_lower = query_text.to_lowercase();
        let index = self.index.read();

        let mut scored: Vec<(f64, &DocumentIndex, &IndexChunk)> = Vec::new();

        for (_, doc) in index.iter() {
            for chunk in &doc.chunks {
                let chunk_lower = chunk.content.to_lowercase();
                let score = compute_similarity(&query_lower, &chunk_lower);

                if score > 0.0 {
                    scored.push((score, doc, chunk));
                }
            }
        }

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        scored
            .into_iter()
            .take(top_k)
            .map(|(score, doc, chunk)| SearchResult {
                path: doc.path.clone(),
                title: doc
                    .path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string(),
                snippet: truncate_snippet(&chunk.content, 200),
                score: (score * 1000.0).round() / 1000.0,
            })
            .collect()
    }

    pub fn delete_document(&self, path: &Path) -> Result<()> {
        let file_path = path.to_string_lossy().to_string();
        let mut index = self.index.write();
        index.remove(&file_path);
        info!("Removed document from index: {:?}", path);
        Ok(())
    }

    pub fn get_index_stats(&self) -> IndexStats {
        let index = self.index.read();
        let total_docs = index.len() as u64;
        let total_chunks = index.values().map(|d| d.chunks.len() as u64).sum();
        let last_indexed = index.values().map(|d| d.indexed_at).max();

        // Estimate size from in-memory data
        let size_bytes = index
            .values()
            .flat_map(|d| &d.chunks)
            .map(|c| c.content.len() + c.id.len())
            .sum::<usize>() as u64;

        IndexStats {
            total_documents: total_docs,
            total_chunks,
            index_size_bytes: size_bytes,
            last_indexed,
        }
    }

    pub fn rebuild_index(&self, vault_root: &Path) -> Result<()> {
        let mut index = self.index.write();
        index.clear();
        drop(index);

        let walker = walkdir::WalkDir::new(vault_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file()
                    && e.path()
                        .extension()
                        .map(|ext| matches!(ext.to_str(), Some("md" | "markdown" | "txt")))
                        .unwrap_or(false)
            });

        for entry in walker {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                if let Err(e) = self.index_document(entry.path(), &content) {
                    tracing::warn!("Failed to index {:?}: {}", entry.path(), e);
                }
            }
        }

        info!("Rebuilt vector index");
        Ok(())
    }
}

fn compute_checksum(content: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn compute_similarity(query: &str, chunk: &str) -> f64 {
    if query.is_empty() || chunk.is_empty() {
        return 0.0;
    }

    let query_words: Vec<&str> = query.split_whitespace().collect();
    let chunk_words: Vec<&str> = chunk.split_whitespace().collect();

    if query_words.is_empty() || chunk_words.is_empty() {
        return 0.0;
    }

    let mut matches = 0;
    for qw in &query_words {
        for cw in &chunk_words {
            if cw.contains(qw) || qw.contains(cw) {
                matches += 1;
                break;
            }
        }
    }

    let precision = matches as f64 / query_words.len() as f64;
    let recall = matches as f64 / chunk_words.len() as f64;

    if precision + recall == 0.0 {
        return 0.0;
    }

    2.0 * precision * recall / (precision + recall)
}

fn truncate_snippet(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }

    let mut truncated = text[..max_len].to_string();
    if let Some(last_space) = truncated.rfind(' ') {
        truncated.truncate(last_space);
    }
    truncated.push_str("...");
    truncated
}
