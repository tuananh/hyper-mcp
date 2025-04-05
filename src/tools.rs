use crate::PluginManager;

use super::types::*;
use rpc_router::{HandlerResult, IntoHandlerError};
use serde_json::json;

pub async fn tools_list(
    pm: PluginManager,
    _request: Option<ListToolsRequest>,
) -> HandlerResult<ListToolsResult> {
    let mut plugins = pm.plugins.write().await;
    let mut tool_cache = pm.tool_to_plugin.write().await;
    let mut payload = ListToolsResult::default();

    // Clear the existing cache when listing tools
    tool_cache.clear();

    for (plugin_name, plugin) in plugins.iter_mut() {
        match plugin.call::<&str, &str>("describe", "") {
            Ok(result) => {
                let parsed: ListToolsResult = serde_json::from_str(result).unwrap();

                // Update the tool-to-plugin cache
                for tool in &parsed.tools {
                    tool_cache.insert(tool.name.clone(), plugin_name.clone());
                }

                payload.tools.extend(parsed.tools);
            }
            Err(e) => {
                log::error!("tool {} describe() error: {}", plugin_name, e);
            }
        }
    }

    Ok(payload)
}

pub async fn tools_call(
    pm: PluginManager,
    request: ToolCallRequestParams,
) -> HandlerResult<CallToolResult> {
    let mut plugins = pm.plugins.write().await;
    let tool_cache = pm.tool_to_plugin.read().await;

    let tool_name = request.name.as_str();
    log::info!("request: {:?}", request);

    // because extism wants it that way
    let call_payload = json!({
        "params": request,
    });
    let json_string = serde_json::to_string(&call_payload).expect("Failed to serialize request");

    // Check if the tool exists in the cache
    if let Some(plugin_name) = tool_cache.get(tool_name) {
        if let Some(plugin) = plugins.get_mut(plugin_name) {
            return match plugin.call::<&str, &str>("call", &json_string) {
                Ok(result) => match serde_json::from_str::<CallToolResult>(result) {
                    Ok(parsed) => Ok(parsed),
                    Err(e) => {
                        log::error!("Failed to deserialize data: {} with {}", result, e);
                        Err(
                            json!({"code": -32602, "message": "Failed to deserialized data"})
                                .into_handler_error(),
                        )
                    }
                },
                Err(e) => {
                    log::error!(
                        "Failed to execute plugin {}: {}, request: {:?}",
                        plugin_name,
                        e,
                        request
                    );
                    Err(
                        json!({"code": -32602, "message": format!("Failed to execute plugin {}: {}", plugin_name, e)})
                            .into_handler_error(),
                    )
                }
            };
        }
    }

    // Tool not found in cache
    Err(
        json!({"code": -32602, "message": format!("Tool '{}' not found in any plugin", tool_name)})
            .into_handler_error(),
    )
}
