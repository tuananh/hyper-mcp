use crate::PluginManager;

use super::types::*;
use rpc_router::{HandlerResult, IntoHandlerError};
use serde_json::json;
use tracing::error;

pub async fn tools_list(
    pm: PluginManager,
    _request: Option<ListToolsRequest>,
) -> HandlerResult<ListToolsResult> {
    let mut plugins = pm.plugins.write().await;
    let mut payload = ListToolsResult::default();
    for (key, plugin) in plugins.iter_mut() {
        match plugin.call::<&str, &str>("describe", "") {
            Ok(result) => {
                let parsed: ListToolsResult = serde_json::from_str(result).unwrap();
                payload.tools.extend(parsed.tools);
            }
            Err(e) => {
                error!("tool {} describe() error: {}", key, e.to_string());
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
    let plugin_name = request.name.as_str();

    // because extism wants it that way
    let wrapped = json!({
        "params": request,
    });

    let json_string = serde_json::to_string(&wrapped).expect("Failed to serialize request");
    if let Some(plugin) = plugins.get_mut(plugin_name) {
        match plugin.call::<&str, &str>("call", &json_string) {
            Ok(result) => match serde_json::from_str::<CallToolResult>(result) {
                Ok(parsed) => Ok(parsed),
                Err(e) => {
                    error!(
                        "Failed to deserialize data: {} with {}",
                        result,
                        e.to_string()
                    );
                    Err(
                        json!({"code": -32602, "message": "Failed to deserialized data"})
                            .into_handler_error(),
                    )
                }
            },
            Err(e) => {
                error!(
                    "Failed to execute plugin: {}, request: {:?}",
                    e.to_string(),
                    request
                );
                Err(
                    json!({"code": -32602, "message": "Failed to execute plugin"})
                        .into_handler_error(),
                )
            }
        }
    } else {
        Err(json!({"code": -32602, "message": "Plugin not found"}).into_handler_error())
    }
}
