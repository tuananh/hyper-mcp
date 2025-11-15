mod pdk;

use extism_pdk::*;
use pdk::types::{CallToolResult, Content, ContentType, ListToolsResult, ToolDescription};
use pdk::*;
use serde_json::json;
use std::error::Error as StdError;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for CustomError {}

// Global counter to track how many tools have been added
static TOOL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn notify_tool_list_changed();
}

// Called when a tool is invoked
pub(crate) fn call(input: types::CallToolRequest) -> Result<types::CallToolResult, Error> {
    let tool_name = &input.params.name;

    match tool_name.as_str() {
        "add_tool" => {
            // Increment the tool count
            let new_count = TOOL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

            // Notify that the tool list has changed
            match unsafe { notify_tool_list_changed() } {
                Ok(()) => Ok(CallToolResult {
                    content: vec![Content {
                        text: Some(
                            json!({
                                "message": format!("Successfully added tool_{}", new_count),
                                "tool_count": new_count,
                            })
                            .to_string(),
                        ),
                        r#type: ContentType::Text,
                        ..Default::default()
                    }],
                    is_error: Some(false),
                }),
                Err(e) => Ok(CallToolResult {
                    content: vec![Content {
                        text: Some(format!("Failed to notify host of tool list change: {}", e)),
                        r#type: ContentType::Text,
                        ..Default::default()
                    }],
                    is_error: Some(true),
                }),
            }
        }
        tool_name if tool_name.starts_with("tool_") => {
            // Handle dynamically created tools
            let tool_number = tool_name.strip_prefix("tool_").unwrap_or("unknown");

            // Validate that the tool exists by comparing to TOOL_COUNT
            if let Ok(number_str) = std::str::from_utf8(tool_number.as_bytes()) {
                if let Ok(tool_num) = number_str.parse::<usize>() {
                    let current_count = TOOL_COUNT.load(Ordering::SeqCst);
                    if tool_num < 1 || tool_num > current_count {
                        return Ok(CallToolResult {
                            content: vec![Content {
                                text: Some(format!(
                                    "Tool {} does not exist. Only tools 1 through {} have been created.",
                                    tool_num, current_count
                                )),
                                r#type: ContentType::Text,
                                ..Default::default()
                            }],
                            is_error: Some(true),
                        });
                    }
                } else {
                    return Ok(CallToolResult {
                        content: vec![Content {
                            text: Some(format!("Invalid tool number: {}", tool_number)),
                            r#type: ContentType::Text,
                            ..Default::default()
                        }],
                        is_error: Some(true),
                    });
                }
            }

            Ok(CallToolResult {
                content: vec![Content {
                    text: Some(
                        json!({
                            "message": format!("Called dynamically created tool: {}", tool_name),
                            "tool_number": tool_number,
                        })
                        .to_string(),
                    ),
                    r#type: ContentType::Text,
                    ..Default::default()
                }],
                is_error: Some(false),
            })
        }
        _ => Err(Error::new(CustomError(format!(
            "Unknown tool: {}",
            tool_name
        )))),
    }
}

pub(crate) fn describe() -> Result<types::ListToolsResult, Error> {
    let current_count = TOOL_COUNT.load(Ordering::SeqCst);
    let mut tools = vec![];

    // Always include the add_tool
    tools.push(ToolDescription {
        name: "add_tool".into(),
        description: "Adds a new dynamic tool to the plugin's tool list. Each call creates a new tool named 'tool_n' where n is the number of times this tool has been called.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        })
        .as_object()
        .unwrap()
        .clone(),
    });

    // Add all the dynamically created tools
    for i in 1..=current_count {
        tools.push(ToolDescription {
            name: format!("tool_{}", i),
            description: format!(
                "Dynamically created tool number {}. This tool was added by calling 'add_tool'.",
                i
            ),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false
            })
            .as_object()
            .unwrap()
            .clone(),
        });
    }

    Ok(ListToolsResult { tools })
}
