use std::collections::HashMap;

use anyhow::{Context, Result};
use futures::StreamExt;
use serde_json::Value;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::mpsc;

use crate::AppState;
use crate::ai::claude::ClaudeProvider;
use crate::ai::gemini::GeminiProvider;
use crate::ai::ollama::OllamaProvider;
use crate::ai::openai::OpenAIProvider;
use crate::ai::{AIProvider, ChatCompletionRequest};
use crate::types::*;

fn get_provider(
    provider_type: &Provider,
    config: &crate::config::Config,
) -> Box<dyn AIProvider + Send + Sync> {
    match provider_type {
        Provider::OpenAI => Box::new(OpenAIProvider::new(
            config.ai.api_key.as_deref().unwrap_or(""),
            config.ai.base_url.as_deref(),
        )),
        Provider::Claude => Box::new(ClaudeProvider::new(
            config.ai.api_key.as_deref().unwrap_or(""),
        )),
        Provider::Gemini => Box::new(GeminiProvider::new(
            config.ai.api_key.as_deref().unwrap_or(""),
        )),
        Provider::DeepSeek => Box::new(OpenAIProvider::new(
            config.ai.api_key.as_deref().unwrap_or(""),
            Some("https://api.deepseek.com/v1"),
        )),
        Provider::Ollama => Box::new(OllamaProvider::new(
            config
                .ai
                .base_url
                .as_deref()
                .unwrap_or("http://localhost:11434"),
        )),
    }
}

#[tauri::command]
pub async fn ai_chat(
    app: AppHandle,
    state: State<'_, AppState>,
    messages: Vec<ChatMessage>,
    provider: Provider,
    model: String,
) -> Result<(), String> {
    let config = state.config.read().await;
    let provider_impl = get_provider(&provider, &config);
    drop(config);

    let request = ChatCompletionRequest {
        messages,
        model,
        provider: provider.clone(),
        temperature: None,
        max_tokens: None,
        stream: true,
    };

    let mut stream = provider_impl
        .stream_chat(request)
        .await
        .map_err(|e| format!("Chat failed: {}", e))?;

    while let Some(event) = stream.next().await {
        let payload = serde_json::to_value(&event).unwrap_or_default();
        let _ = app.emit("ai:token", payload);
    }

    Ok(())
}

#[tauri::command]
pub fn ai_template(
    _state: State<'_, AppState>,
    template_name: String,
    variables: HashMap<String, String>,
) -> Result<String, String> {
    let templates: HashMap<&str, &str> = HashMap::from([
        (
            "summarize",
            "Please summarize the following text concisely:\n\n{content}",
        ),
        (
            "explain",
            "Explain the following in simple terms:\n\n{content}",
        ),
        (
            "rewrite",
            "Rewrite the following text to improve clarity and flow:\n\n{content}",
        ),
        (
            "translate",
            "Translate the following text to {language}:\n\n{content}",
        ),
        (
            "continue",
            "Continue writing from where this leaves off:\n\n{content}",
        ),
    ]);

    let template = templates
        .get(template_name.as_str())
        .ok_or_else(|| format!("Template '{}' not found", template_name))?;

    let mut result = template.to_string();
    for (key, value) in &variables {
        result = result.replace(&format!("{{{}}}", key), value);
    }

    Ok(result)
}

#[tauri::command]
pub async fn ai_list_models(
    state: State<'_, AppState>,
    provider: Provider,
) -> Result<Vec<String>, String> {
    let config = state.config.read().await;
    let provider_impl = get_provider(&provider, &config);
    drop(config);

    provider_impl
        .list_models()
        .await
        .map_err(|e| format!("Failed to list models: {}", e))
}

#[tauri::command]
pub async fn test_provider_connection(
    state: State<'_, AppState>,
    provider: Provider,
) -> Result<bool, String> {
    let config = state.config.read().await;
    let provider_impl = get_provider(&provider, &config);
    drop(config);

    provider_impl
        .test_connection()
        .await
        .map_err(|e| format!("Connection test failed: {}", e))
}
