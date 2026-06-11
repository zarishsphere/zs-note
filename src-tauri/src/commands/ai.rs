use std::collections::HashMap;

use futures::StreamExt;
use tauri::{AppHandle, Emitter, State};

use crate::ai::claude::ClaudeProvider;
use crate::ai::gemini::GeminiProvider;
use crate::ai::ollama::OllamaProvider;
use crate::ai::openai::OpenAIProvider;
use crate::ai::AIProvider;
use crate::types::*;
use crate::AppState;

fn resolve_provider_config(
    config: &crate::config::Config,
    provider: &ProviderInput,
) -> Result<ProviderConfig, String> {
    let provider_id = provider.id();
    let configured = config
        .ai
        .providers
        .iter()
        .find(|p| p.id == provider_id)
        .cloned()
        .ok_or_else(|| format!("AI provider '{}' is not configured", provider_id))?;

    if !configured.enabled {
        return Err(format!("AI provider '{}' is disabled", configured.name));
    }

    Ok(configured)
}

fn get_provider(provider_config: &ProviderConfig) -> Box<dyn AIProvider + Send + Sync> {
    match &provider_config.provider_type {
        Provider::OpenAI => Box::new(OpenAIProvider::new(
            provider_config.api_key.as_deref().unwrap_or(""),
            provider_config.base_url.as_deref(),
        )),
        Provider::Anthropic => Box::new(ClaudeProvider::new(
            provider_config.api_key.as_deref().unwrap_or(""),
        )),
        Provider::Google => Box::new(GeminiProvider::new(
            provider_config.api_key.as_deref().unwrap_or(""),
        )),
        Provider::DeepSeek => Box::new(OpenAIProvider::new(
            provider_config.api_key.as_deref().unwrap_or(""),
            Some(
                provider_config
                    .base_url
                    .as_deref()
                    .unwrap_or("https://api.deepseek.com/v1"),
            ),
        )),
        Provider::Ollama => Box::new(OllamaProvider::new(
            provider_config
                .base_url
                .as_deref()
                .unwrap_or("http://localhost:11434"),
        )),
        Provider::Custom => Box::new(OpenAIProvider::new(
            provider_config.api_key.as_deref().unwrap_or(""),
            provider_config.base_url.as_deref(),
        )),
    }
}

#[tauri::command]
pub async fn ai_chat(
    app: AppHandle,
    state: State<'_, AppState>,
    messages: Vec<ChatMessage>,
    provider: ProviderInput,
    model: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    top_p: Option<f32>,
) -> Result<String, String> {
    let config = state.config.read().await;
    let provider_config = resolve_provider_config(&config, &provider)?;
    let provider_impl = get_provider(&provider_config);
    drop(config);

    let request = ChatCompletionRequest {
        messages,
        model: model
            .filter(|m| !m.trim().is_empty())
            .unwrap_or_else(|| provider_config.default_model.clone()),
        provider: provider_config.provider_type.clone(),
        temperature: temperature.or(provider_config.temperature),
        max_tokens: max_tokens.or(provider_config.max_tokens),
        top_p,
        stream: true,
    };

    let mut stream = provider_impl
        .stream_chat(request)
        .await
        .map_err(|e| format!("Chat failed: {}", e))?;

    let mut assistant_text = String::new();

    while let Some(event) = stream.next().await {
        match event {
            StreamEvent::Token { content } => {
                if !content.is_empty() {
                    assistant_text.push_str(&content);
                    let _ = app.emit("ai:chunk", content);
                }
            }
            StreamEvent::Done { .. } => break,
            StreamEvent::Error { message } => return Err(message),
        }
    }

    Ok(assistant_text)
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
    provider: ProviderInput,
) -> Result<Vec<String>, String> {
    let config = state.config.read().await;
    let provider_config = resolve_provider_config(&config, &provider)?;
    let provider_impl = get_provider(&provider_config);
    drop(config);

    provider_impl
        .list_models()
        .await
        .map_err(|e| format!("Failed to list models: {}", e))
}

#[tauri::command]
pub async fn test_provider_connection(
    state: State<'_, AppState>,
    provider: ProviderInput,
) -> Result<bool, String> {
    let config = state.config.read().await;
    let provider_config = resolve_provider_config(&config, &provider)?;
    let provider_impl = get_provider(&provider_config);
    drop(config);

    provider_impl
        .test_connection()
        .await
        .map_err(|e| format!("Connection test failed: {}", e))
}
