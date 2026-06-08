use tauri::State;

use crate::AppState;
use crate::mcp::MCPClient;
use crate::types::*;

#[tauri::command]
pub async fn mcp_list_servers(state: State<'_, AppState>) -> Result<Vec<McpServerInfo>, String> {
    let config = state.config.read().await;
    let mcp_configs = config
        .plugins
        .settings
        .get("mcp")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut servers = Vec::new();
    for cfg in &mcp_configs {
        let name = cfg
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let transport = cfg
            .get("transport")
            .and_then(|v| v.as_str())
            .unwrap_or("stdio")
            .to_string();
        let tools = cfg
            .get("tools")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        Some(McpToolInfo {
                            name: t.get("name").and_then(|v| v.as_str())?.to_string(),
                            description: t
                                .get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            input_schema: t
                                .get("inputSchema")
                                .cloned()
                                .unwrap_or(serde_json::Value::Null),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        servers.push(McpServerInfo {
            name,
            transport,
            status: McpStatus::Connected,
            tools,
        });
    }

    Ok(servers)
}

#[tauri::command]
pub async fn mcp_add_server(
    state: State<'_, AppState>,
    config_data: serde_json::Value,
) -> Result<(), String> {
    let mut config = state.config.write().await;
    let mcp_entry = config
        .plugins
        .settings
        .entry("mcp".to_string())
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));

    if let Some(arr) = mcp_entry.as_array_mut() {
        arr.push(config_data);
    }

    config
        .save(&state.vault_path)
        .map_err(|e| format!("Failed to save config: {}", e))
}

#[tauri::command]
pub async fn mcp_remove_server(state: State<'_, AppState>, name: String) -> Result<(), String> {
    let mut config = state.config.write().await;
    if let Some(arr) = config
        .plugins
        .settings
        .get_mut("mcp")
        .and_then(|v| v.as_array_mut())
    {
        arr.retain(|entry| entry.get("name").and_then(|v| v.as_str()) != Some(&name));
    }

    config
        .save(&state.vault_path)
        .map_err(|e| format!("Failed to save config: {}", e))
}

#[tauri::command]
pub async fn mcp_test_connection(name: String) -> Result<bool, String> {
    let client = MCPClient::new(&name);
    client
        .test_connection()
        .await
        .map_err(|e| format!("Connection test failed: {}", e))
}

#[tauri::command]
pub async fn mcp_call_tool(
    server: String,
    tool: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let client = MCPClient::new(&server);
    client
        .call_tool(&server, &tool, args)
        .await
        .map_err(|e| format!("Tool call failed: {}", e))
}
