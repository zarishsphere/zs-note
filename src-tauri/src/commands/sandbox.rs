use serde_json::Value;
use tauri::State;

use crate::types::AppState;

#[tauri::command]
pub fn sandbox_execute(
    state: State<'_, AppState>,
    tool_name: String,
    func_name: String,
    args: Value,
) -> Result<String, String> {
    let config = state.config.blocking_read();
    let tool = config
        .sandbox
        .tools
        .iter()
        .find(|t| t.name == tool_name)
        .cloned()
        .ok_or_else(|| format!("Tool '{}' not found in sandbox config", tool_name))?;
    drop(config);

    let sandbox = &state.sandbox;
    sandbox
        .execute(&tool, &func_name, &args.to_string())
        .map_err(|e| format!("Sandbox execution failed: {}", e))
}

#[tauri::command]
pub fn sandbox_get_tools(
    state: State<'_, AppState>,
) -> Result<Vec<crate::types::ToolConfig>, String> {
    let config = state.config.blocking_read();
    Ok(config.sandbox.tools.clone())
}

#[tauri::command]
pub fn sandbox_test_tool(state: State<'_, AppState>, tool_name: String) -> Result<bool, String> {
    let config = state.config.blocking_read();
    let tool = config
        .sandbox
        .tools
        .iter()
        .find(|t| t.name == tool_name)
        .cloned()
        .ok_or_else(|| format!("Tool '{}' not found", tool_name))?;
    drop(config);

    let sandbox = &state.sandbox;
    let wasm_bytes =
        std::fs::read(&tool.wasm_path).map_err(|e| format!("Failed to read WASM module: {}", e))?;

    sandbox
        .test_module(&wasm_bytes)
        .map(|_| true)
        .map_err(|e| format!("Tool test failed: {}", e))
}
