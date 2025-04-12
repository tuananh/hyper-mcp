mod pdk;

use std::collections::BTreeMap;

use extism_pdk::*;
use json::Value;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use serde_json::json;

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    match input.params.name.as_str() {
        "gomodule_latest_version" => latest_version(input),
        "gomodule_info" => module_info(input),
        _ => Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some(format!("Unknown tool: {}", input.params.name)),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        }),
    }
}

fn module_info(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    if let Some(Value::String(module_names)) = args.get("module_names") {
        let module_names: Vec<&str> = module_names.split(',').map(|s| s.trim()).collect();
        let mut results = Vec::new();

        for module_name in module_names {
            let mut req = HttpRequest {
                url: format!("https://proxy.golang.org/{}/@latest", module_name),
                headers: BTreeMap::new(),
                method: Some("GET".to_string()),
            };

            req.headers
                .insert("User-Agent".to_string(), "hyper-mcp/1.0".to_string());

            let res = http::request::<()>(&req, None)?;

            let body = res.body();
            let json_str = String::from_utf8_lossy(body.as_slice());

            let json: serde_json::Value = serde_json::from_str(&json_str)?;

            // TODO: figure out how to get module license
            results.push(json);
        }

        if !results.is_empty() {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(serde_json::to_string(&results)?),
                    mime_type: Some("text/plain".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some("Failed to get module information".into()),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide module names".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn latest_version(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    if let Some(Value::String(module_names)) = args.get("module_names") {
        let module_names: Vec<&str> = module_names.split(',').map(|s| s.trim()).collect();
        let mut results = BTreeMap::new();

        for module_name in module_names {
            let mut req = HttpRequest {
                url: format!("https://proxy.golang.org/{}/@latest", module_name),
                headers: BTreeMap::new(),
                method: Some("GET".to_string()),
            };

            req.headers
                .insert("User-Agent".to_string(), "hyper-mcp/1.0".to_string());

            let res = http::request::<()>(&req, None)?;

            let body = res.body();
            let json_str = String::from_utf8_lossy(body.as_slice());

            let json: serde_json::Value = serde_json::from_str(&json_str)?;

            if let Some(version) = json["Version"].as_str() {
                results.insert(module_name.to_string(), version.to_string());
            }
        }

        if !results.is_empty() {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(serde_json::to_string(&results)?),
                    mime_type: Some("text/plain".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some("Failed to get latest versions".into()),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide module names".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult {
        tools: vec![
            ToolDescription {
                name: "gomodule_latest_version".into(),
                description: "Fetches the latest version of multiple Go modules. Assume it's github.com if not specified".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "module_names": {
                            "type": "string",
                            "description": "Comma-separated list of Go module names to get the latest versions for",
                        },
                    },
                    "required": ["module_names"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gomodule_info".into(),
                description: "Fetches detailed information about multiple Go modules. Assume it's github.com if not specified".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "module_names": {
                            "type": "string",
                            "description": "Comma-separated list of Go module names to get information for",
                        },
                    },
                    "required": ["module_names"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
