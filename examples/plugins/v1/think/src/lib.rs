mod pdk;

use extism_pdk::*;
use json::Value;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use serde_json::json;

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    match input.params.name.as_str() {
        "think" => think(input),
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

fn think(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    if let Some(Value::String(thought)) = args.get("thought") {
        Ok(CallToolResult {
            is_error: None,
            content: vec![Content {
                annotations: None,
                text: Some(thought.clone()),
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
                text: Some("Please provide a 'thought' string.".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult{
        tools: vec![
            ToolDescription {
                name: "think".into(),
                description: "Use the tool to think about something. It will not obtain new information or change the database, but just append the thought to the log. Use it when complex reasoning or some cache memory is needed.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "thought": {
                            "type": "string",
                            "description": "A thought to think about.",
                        },
                    },
                    "required": ["thought"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
