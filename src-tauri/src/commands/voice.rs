use serde::{Deserialize, Serialize};
use tauri::State;

use crate::types::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceRecognitionResult {
    pub text: String,
    pub confidence: f32,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceCommand {
    pub command_type: String,
    pub parameters: std::collections::HashMap<String, String>,
    pub confidence: f32,
}

#[tauri::command]
pub fn voice_start_recording(_state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Voice recording started");
    Ok(())
}

#[tauri::command]
pub fn voice_stop_recording(_state: State<'_, AppState>) -> Result<VoiceRecognitionResult, String> {
    tracing::info!("Voice recording stopped");
    Ok(VoiceRecognitionResult {
        text: String::new(),
        confidence: 0.0,
        language: "en-US".into(),
    })
}

#[tauri::command]
pub fn voice_process_command(
    _state: State<'_, AppState>,
    text: String,
) -> Result<VoiceCommand, String> {
    let lower = text.to_lowercase();

    let (command_type, parameters) = if lower.contains("save") {
        ("save".into(), std::collections::HashMap::new())
    } else if lower.contains("search") {
        let query = text.split("search").nth(1).unwrap_or("").trim().to_string();
        let mut params = std::collections::HashMap::new();
        params.insert("query".into(), query);
        ("search".into(), params)
    } else if lower.contains("open") {
        let file = text.split("open").nth(1).unwrap_or("").trim().to_string();
        let mut params = std::collections::HashMap::new();
        params.insert("file".into(), file);
        ("open".into(), params)
    } else {
        let mut params = std::collections::HashMap::new();
        params.insert("text".into(), text);
        ("dictate".into(), params)
    };

    Ok(VoiceCommand {
        command_type,
        parameters,
        confidence: 0.9,
    })
}
