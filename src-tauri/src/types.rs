use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::config::Config;
use crate::git::GitEngine;
use crate::sandbox::SandboxEngine;
use crate::vector::VectorStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub path: PathBuf,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileEntry {
    File {
        name: String,
        path: PathBuf,
    },
    Folder {
        name: String,
        path: PathBuf,
        children: Vec<FileEntry>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "tool")]
    Tool,
}

impl std::fmt::Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatRole::User => write!(f, "user"),
            ChatRole::Assistant => write!(f, "assistant"),
            ChatRole::System => write!(f, "system"),
            ChatRole::Tool => write!(f, "tool"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "claude")]
    Claude,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "deepseek")]
    DeepSeek,
    #[serde(rename = "ollama")]
    Ollama,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::OpenAI => write!(f, "openai"),
            Provider::Claude => write!(f, "claude"),
            Provider::Gemini => write!(f, "gemini"),
            Provider::DeepSeek => write!(f, "deepseek"),
            Provider::Ollama => write!(f, "ollama"),
        }
    }
}

impl std::str::FromStr for Provider {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(Provider::OpenAI),
            "claude" => Ok(Provider::Claude),
            "gemini" => Ok(Provider::Gemini),
            "deepseek" => Ok(Provider::DeepSeek),
            "ollama" => Ok(Provider::Ollama),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub name: String,
    #[serde(rename = "wasmPath")]
    pub wasm_path: PathBuf,
    pub permissions: Vec<String>,
    #[serde(rename = "memoryLimit")]
    pub memory_limit: usize,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub name: String,
    pub path: PathBuf,
    pub formats: Vec<String>,
    #[serde(rename = "indexOnStart")]
    pub index_on_start: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishTarget {
    pub name: String,
    #[serde(rename = "type")]
    pub target_type: String,
    pub repo: Option<String>,
    pub endpoint: Option<String>,
    pub branch: Option<String>,
    #[serde(rename = "keyId")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    #[serde(rename = "autoCommit")]
    pub auto_commit: bool,
    #[serde(rename = "commitStyle")]
    pub commit_style: String,
    pub remote: Option<RemoteConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub url: String,
    pub branch: String,
    #[serde(rename = "sshKey")]
    pub ssh_key: Option<String>,
    #[serde(rename = "autoSync")]
    pub auto_sync: bool,
    #[serde(rename = "syncInterval")]
    pub sync_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub theme: String,
    #[serde(rename = "fontSize")]
    pub font_size: u32,
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "lineHeight")]
    pub line_height: f32,
    #[serde(rename = "proseWidth")]
    pub prose_width: u32,
    #[serde(rename = "vimMode")]
    pub vim_mode: bool,
    #[serde(rename = "spellCheck")]
    pub spell_check: bool,
    #[serde(rename = "autoSave")]
    pub auto_save: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
    pub sandbox: Arc<SandboxEngine>,
    pub git: Arc<RwLock<GitEngine>>,
    pub vector: Arc<VectorStore>,
    pub vault_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: PathBuf,
    pub title: String,
    pub snippet: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitEntry {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub old_path: String,
    pub new_path: String,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub origin: char,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub ahead: usize,
    pub behind: usize,
    pub unstaged: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub provider: Provider,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub message: ChatMessage,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt: u32,
    pub completion: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    Token { content: String },
    Done { usage: Option<TokenUsage> },
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConverterInfo {
    pub name: String,
    pub description: String,
    pub input_formats: Vec<String>,
    pub output_formats: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerInfo {
    pub name: String,
    pub transport: String,
    pub status: McpStatus,
    pub tools: Vec<McpToolInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpStatus {
    Connected,
    Disconnected,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_documents: u64,
    pub total_chunks: u64,
    pub index_size_bytes: u64,
    pub last_indexed: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub name: String,
    pub path: PathBuf,
    pub vault_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub version: String,
}
