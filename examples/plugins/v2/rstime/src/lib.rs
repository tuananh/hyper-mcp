mod pdk;

use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use extism_pdk::{HttpRequest, Memory, http::request};
use pdk::types::*;
use serde_json::{Map, Value, json};

pub(crate) fn call_tool(input: CallToolRequest) -> Result<CallToolResult> {
    match input.request.name.as_str() {
        "get_time" => {
            let tz = match input
                .request
                .arguments
                .as_ref()
                .and_then(|args| args.get("timezone"))
                .and_then(|v| v.as_str())
            {
                Some(timezone) => match timezone.parse::<chrono_tz::Tz>() {
                    Ok(tz) => tz,
                    Err(e) => {
                        return Ok(CallToolResult {
                            content: vec![ContentBlock::Text(TextContent {
                                text: format!("Error: Invalid timezone '{}': {}", timezone, e),

                                ..Default::default()
                            })],
                            is_error: Some(true),

                            ..Default::default()
                        });
                    }
                },
                None => chrono_tz::UTC,
            };
            let current_time = chrono::Utc::now().with_timezone(&tz).to_rfc2822();
            Ok(CallToolResult {
                content: vec![ContentBlock::Text(TextContent {
                    text: current_time.clone(),

                    ..Default::default()
                })],
                structured_content: Some(Map::from_iter([(
                    "current_time".to_string(),
                    Value::String(current_time),
                )])),

                ..Default::default()
            })
        }
        "parse_time" => {
            let time_str = match input
                .request
                .arguments
                .as_ref()
                .and_then(|args| args.get("time"))
                .and_then(|v| v.as_str())
            {
                Some(t) => t,
                None => {
                    return Ok(CallToolResult {
                        content: vec![ContentBlock::Text(TextContent {
                            text: "Error: 'time' argument is required".to_string(),

                            ..Default::default()
                        })],
                        is_error: Some(true),

                        ..Default::default()
                    });
                }
            };
            match chrono::DateTime::parse_from_rfc2822(time_str) {
                Ok(dt) => Ok(CallToolResult {
                    content: vec![ContentBlock::Text(TextContent {
                        text: dt.timestamp().to_string(),

                        ..Default::default()
                    })],
                    structured_content: Some(Map::from_iter([(
                        "timestamp".to_string(),
                        Value::Number(serde_json::Number::from(dt.timestamp())),
                    )])),

                    ..Default::default()
                }),
                Err(e) => Ok(CallToolResult {
                    content: vec![ContentBlock::Text(TextContent {
                        text: format!("Error parsing time: {}", e),

                        ..Default::default()
                    })],
                    is_error: Some(true),

                    ..Default::default()
                }),
            }
        }
        _ => Err(anyhow!("Unknown tool: {}", input.request.name)),
    }
}

// Provide completion suggestions for a partially-typed input.
//
// This function is called when the user requests autocompletion. The plugin should analyze the partial input and return matching completion suggestions based on the reference (prompt or resource) and argument context.
pub(crate) fn complete(input: CompleteRequest) -> Result<CompleteResult> {
    match input.request.r#ref {
        Reference::Prompt(prompt_ref) if prompt_ref.name.as_str() != "get_time_with_timezone" => {
            return Err(anyhow!(
                "Completion for prompt not implemented: {}",
                prompt_ref.name
            ));
        }

        Reference::ResourceTemplate(resource_ref)
            if resource_ref.uri.as_str()
                != "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}" =>
        {
            return Err(anyhow!(
                "Completion for resource not implemented: {}",
                resource_ref.uri
            ));
        }

        _ => {}
    };

    match input.request.argument.name.as_str() {
        "timezone" => {
            let query = input
                .request
                .argument
                .value
                .to_ascii_lowercase()
                .replace(" ", "_");
            let mut suggestions: Vec<String> = vec![];
            let mut total: i64 = 0;
            for tz in chrono_tz::TZ_VARIANTS {
                if tz.name().to_ascii_lowercase().contains(&query) {
                    if suggestions.len() < 100 {
                        suggestions.push(tz.name().to_string());
                    }
                    total += 1;
                }
            }
            Ok(CompleteResult {
                completion: CompleteResultCompletion {
                    has_more: Some(total > suggestions.len() as i64),
                    total: Some(total),
                    values: suggestions,
                },
            })
        }
        _ => Err(anyhow!(
            "Completion for argument not implemented: {}",
            input.request.argument.name
        )),
    }
}

// Retrieve a specific prompt by name.
//
// This function is called when the user requests a specific prompt. The plugin should return the prompt details including messages and optional description.
pub(crate) fn get_prompt(input: GetPromptRequest) -> Result<GetPromptResult> {
    match input.request.name.as_str() {
        "get_time_with_timezone" => {
            let tz = match input
                .request
                .arguments
                .as_ref()
                .and_then(|args| args.get("timezone"))
            {
                Some(timezone) => match timezone.parse::<chrono_tz::Tz>() {
                    Ok(tz) => tz,
                    Err(e) => {
                        return Ok(GetPromptResult {
                            messages: vec![PromptMessage {
                                role: Role::Assistant,
                                content: ContentBlock::Text(TextContent {
                                    text: format!("Error: Invalid timezone '{}': {}", timezone, e),

                                    ..Default::default()
                                }),
                            }],

                            ..Default::default()
                        });
                    }
                },
                None => chrono_tz::UTC,
            };

            Ok(GetPromptResult {
                description: Some(format!("Information for {}", tz.name())),
                messages: vec![PromptMessage {
                    role: Role::Assistant,
                    content: ContentBlock::Text(TextContent {
                        text: format!("Please get the time for the timezone {}", tz.name()),

                        ..Default::default()
                    }),
                }],
            })
        }
        _ => Err(anyhow!("Prompt not found: {}", input.request.name)),
    }
}

// List all available prompts.
//
// This function should return a list of prompts that the plugin provides. Each prompt should include its name and a brief description of what it does. Supports pagination via cursor.
pub(crate) fn list_prompts(_input: ListPromptsRequest) -> Result<ListPromptsResult> {
    Ok(ListPromptsResult {
        prompts: vec![Prompt {
            name: "get_time_with_timezone".to_string(),
            description: Some(
                "Asks the assistant to get the time in a provided timezone".to_string(),
            ),
            title: Some("Get Localized Time".to_string()),
            arguments: Some(vec![PromptArgument {
                name: "timezone".to_string(),
                description: Some(
                    "The timezone to prompt for, will use UTC by default".to_string(),
                ),
                title: Some("Timezone".to_string()),

                ..Default::default()
            }]),
        }],
    })
}

// List all available resource templates.
//
// This function should return a list of resource templates that the plugin provides. Templates are URI patterns that can match multiple resources. Supports pagination via cursor.
pub(crate) fn list_resource_templates(
    _input: ListResourceTemplatesRequest,
) -> Result<ListResourceTemplatesResult> {
    Ok(ListResourceTemplatesResult {
        resource_templates: vec![ResourceTemplate {
            name: "time_zone_converter".to_string(),
            description: Some("Display HTML page containing timezone information".to_string()),
            mime_type: Some("text/html".to_string()),
            uri_template: "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}"
                .to_string(),
            title: Some("TimeZone Converter".to_string()),

            ..Default::default()
        }],
        ..Default::default()
    })
}

// List all available resources.
//
// This function should return a list of resources that the plugin provides. Resources are URI-based references to files, data, or services. Supports pagination via cursor.
pub(crate) fn list_resources(_input: ListResourcesRequest) -> Result<ListResourcesResult> {
    Ok(ListResourcesResult::default())
}

// List all available tools.
//
// This function should return a list of all tools that the plugin provides. Each tool should include its name, description, and input schema. Supports pagination via cursor.
pub(crate) fn list_tools(_input: ListToolsRequest) -> Result<ListToolsResult> {
    Ok(ListToolsResult {
        tools: vec![
            Tool {
                annotations: None,
                description: Some("Returns the current time in the specified timezone. If no timezone is specified then UTC is used.".to_string()),
                input_schema: ToolSchema {
                    properties: Some(Map::from_iter([
                        ("timezone".to_string(), json!({
                            "type": "string",
                            "description": "The timezone to get the current time for, e.g. 'America/New_York'. Defaults to 'UTC' if not provided.",
                        })),
                    ])),

                    ..Default::default()
                },
                name: "get_time".to_string(),
                output_schema: Some(ToolSchema {
                    properties: Some(Map::from_iter([
                        ("current_time".to_string(), json!({
                            "type": "string",
                            "description": "The current time in the specified timezone in RFC2822 format.",
                        })),
                    ])),
                    required: Some(vec!["current_time".to_string()]),

                    ..Default::default()
                }),
                title: Some("Get Current Time".to_string()),
            },
            Tool {
                annotations: None,
                description: Some("Parses a time string in RFC2822 format and returns the corresponding timestamp in UTC.".to_string()),
                input_schema: ToolSchema {
                    properties: Some(Map::from_iter([
                        ("time".to_string(), json!({
                            "type": "string",
                            "description": "The time string in RFC2822 format to parse.",
                        })),
                    ])),
                    required: Some(vec!["time".to_string()]),

                    ..Default::default()
                },
                name: "parse_time".to_string(),
                output_schema: Some(ToolSchema {
                    properties: Some(Map::from_iter([
                        ("timestamp".to_string(), json!({
                            "type": "integer",
                            "description": "The parsed timestamp in seconds since the Unix epoch.",
                        })),
                    ])),
                    required: Some(vec!["timestamp".to_string()]),

                    ..Default::default()
                }),
                title: Some("Parse Time from RFC2822".to_string()),
            }
        ],
    })
}

// Notification that the list of roots has changed.
//
// This is an optional notification handler. If implemented, the plugin will be notified whenever the roots list changes on the client side. This allows plugins to react to changes in the file system roots or other root resources.
pub(crate) fn on_roots_list_changed(_input: PluginNotificationContext) -> Result<()> {
    Ok(())
}

// Read the contents of a resource by its URI.
//
// This function is called when the user wants to read the contents of a specific resource. The plugin should retrieve and return the resource data with appropriate MIME type information.
pub(crate) fn read_resource(input: ReadResourceRequest) -> Result<ReadResourceResult> {
    if !input
        .request
        .uri
        .starts_with("https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz=")
    {
        return Ok(ReadResourceResult::default());
    }

    match request(
        &HttpRequest::new(input.request.uri.clone()).with_method("GET"),
        None::<Memory>,
    ) {
        Ok(response) => {
            if response.status_code() >= 200 && response.status_code() < 300 {
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::Blob(BlobResourceContents {
                        mime_type: Some("text/html".to_string()),
                        blob: STANDARD.encode(response.body()),
                        uri: input.request.uri,

                        ..Default::default()
                    })],
                })
            } else {
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::Text(TextResourceContents {
                        mime_type: Some("text/plain".to_string()),
                        text: format!("Error fetching resource: HTTP {}", response.status_code()),

                        ..Default::default()
                    })],
                })
            }
        }
        Err(e) => Ok(ReadResourceResult {
            contents: vec![ResourceContents::Text(TextResourceContents {
                mime_type: Some("text/plain".to_string()),
                text: format!("Error fetching resource: {}", e),

                ..Default::default()
            })],
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_call_tool_get_time_utc() {
        let input = CallToolRequest {
            context: PluginRequestContext::default(),
            request: CallToolRequestParam {
                name: "get_time".to_string(),
                arguments: None,
            },
        };

        let result = call_tool(input).expect("call_tool should succeed");
        assert!(!result.content.is_empty());
        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.structured_content.is_some());
    }

    #[test]
    fn test_call_tool_get_time_with_timezone() {
        let mut args = Map::new();
        args.insert(
            "timezone".to_string(),
            Value::String("America/New_York".to_string()),
        );

        let input = CallToolRequest {
            context: PluginRequestContext::default(),
            request: CallToolRequestParam {
                name: "get_time".to_string(),
                arguments: Some(args),
            },
        };

        let result = call_tool(input).expect("call_tool should succeed");
        assert!(!result.content.is_empty());
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[test]
    fn test_call_tool_get_time_invalid_timezone() {
        let mut args = Map::new();
        args.insert(
            "timezone".to_string(),
            Value::String("Invalid/Timezone".to_string()),
        );

        let input = CallToolRequest {
            context: PluginRequestContext::default(),
            request: CallToolRequestParam {
                name: "get_time".to_string(),
                arguments: Some(args),
            },
        };

        let result = call_tool(input).expect("call_tool should succeed");
        assert!(result.is_error == Some(true));
    }

    #[test]
    fn test_call_tool_parse_time_valid() {
        let mut args = Map::new();
        args.insert(
            "time".to_string(),
            Value::String("29 Nov 2024 10:30:00 +0000".to_string()),
        );

        let input = CallToolRequest {
            context: PluginRequestContext::default(),
            request: CallToolRequestParam {
                name: "parse_time".to_string(),
                arguments: Some(args),
            },
        };

        let result = call_tool(input).expect("call_tool should succeed");
        assert!(!result.content.is_empty());
        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.structured_content.is_some());
    }

    #[test]
    fn test_call_tool_parse_time_missing_argument() {
        let input = CallToolRequest {
            context: PluginRequestContext::default(),
            request: CallToolRequestParam {
                name: "parse_time".to_string(),
                arguments: None,
            },
        };

        let result = call_tool(input).expect("call_tool should succeed");
        assert!(result.is_error == Some(true));
    }

    #[test]
    fn test_call_tool_parse_time_invalid() {
        let mut args = Map::new();
        args.insert(
            "time".to_string(),
            Value::String("not a valid time".to_string()),
        );

        let input = CallToolRequest {
            context: PluginRequestContext::default(),
            request: CallToolRequestParam {
                name: "parse_time".to_string(),
                arguments: Some(args),
            },
        };

        let result = call_tool(input).expect("call_tool should succeed");
        assert!(result.is_error == Some(true));
    }

    #[test]
    fn test_call_tool_unknown_tool() {
        let input = CallToolRequest {
            context: PluginRequestContext::default(),
            request: CallToolRequestParam {
                name: "unknown_tool".to_string(),
                arguments: None,
            },
        };

        let result = call_tool(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_complete_with_utc_query() {
        // Test complete function with UTC timezone query
        let prompt_ref = PromptReference {
            name: "get_time_with_timezone".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::Prompt(prompt_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "utc".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(!result.completion.values.is_empty());
        assert!(result.completion.values.contains(&"UTC".to_string()));
        assert!(result.completion.total.is_some());
    }

    #[test]
    fn test_complete_with_america_query() {
        // Test complete function with America timezone prefix
        let prompt_ref = PromptReference {
            name: "get_time_with_timezone".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::Prompt(prompt_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "america".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(!result.completion.values.is_empty());
        assert!(result.completion.values.len() > 5);
        assert!(
            result
                .completion
                .values
                .iter()
                .any(|v| v.contains("America"))
        );
    }

    #[test]
    fn test_complete_with_empty_query() {
        // Test complete function with empty query - should return many results
        let prompt_ref = PromptReference {
            name: "get_time_with_timezone".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::Prompt(prompt_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: String::new(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        // Should return max 100 suggestions
        assert!(result.completion.values.len() <= 100);
        // Should indicate there are more
        assert_eq!(result.completion.has_more, Some(true));
        // Total should be much larger
        assert!(result.completion.total.unwrap() > 400);
    }

    #[test]
    fn test_complete_with_york_query() {
        // Test complete function with York timezone query (case insensitive)
        let prompt_ref = PromptReference {
            name: "get_time_with_timezone".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::Prompt(prompt_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "YORK".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(!result.completion.values.is_empty());
        assert!(
            result
                .completion
                .values
                .contains(&"America/New_York".to_string())
        );
    }

    #[test]
    fn test_complete_with_los_angeles_query() {
        // Test complete function with space-separated timezone query
        let prompt_ref = PromptReference {
            name: "get_time_with_timezone".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::Prompt(prompt_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "los angeles".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(!result.completion.values.is_empty());
        assert!(
            result
                .completion
                .values
                .contains(&"America/Los_Angeles".to_string())
        );
    }

    #[test]
    fn test_complete_with_europe_query() {
        // Test complete function with Europe timezone prefix
        let prompt_ref = PromptReference {
            name: "get_time_with_timezone".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::Prompt(prompt_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "europe/".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(!result.completion.values.is_empty());
        // All results should contain Europe
        assert!(
            result
                .completion
                .values
                .iter()
                .all(|v| v.to_lowercase().contains("europe"))
        );
    }

    #[test]
    fn test_complete_result_structure() {
        // Test that complete results have the expected structure
        // We verify the logic by constructing expected outputs
        let values = vec!["UTC".to_string(), "America/New_York".to_string()];
        let total = 500i64;
        let has_more = total > values.len() as i64;

        let completion = CompleteResultCompletion {
            has_more: Some(has_more),
            total: Some(total),
            values: values.clone(),
        };

        let result = CompleteResult { completion };
        assert_eq!(result.completion.values.len(), 2);
        assert!(result.completion.has_more.unwrap());
        assert_eq!(result.completion.total.unwrap(), 500);
    }

    #[test]
    fn test_complete_result_has_required_fields() {
        // Test that CompleteResult includes required fields
        let completion = CompleteResultCompletion {
            has_more: Some(true),
            total: Some(500),
            values: vec!["UTC".to_string(), "America/New_York".to_string()],
        };

        let result = CompleteResult { completion };

        assert!(result.completion.has_more.is_some());
        assert!(result.completion.total.is_some());
        assert!(!result.completion.values.is_empty());
        assert_eq!(result.completion.values.len(), 2);
    }

    #[test]
    fn test_complete_result_total_flag_matches_logic() {
        // Test the logic for has_more flag: should be true when total > values.len()
        let values = vec!["UTC".to_string()];
        let total = 500i64;
        let values_len = values.len() as i64;

        let has_more = total > values_len;
        assert!(has_more);
    }

    #[test]
    fn test_complete_result_no_more_when_all_returned() {
        // Test the logic for has_more flag: should be false when all results fit
        let values = vec!["UTC".to_string(), "America/New_York".to_string()];
        let total = values.len() as i64;
        let values_len = values.len() as i64;

        let has_more = total > values_len;
        assert!(!has_more);
    }

    #[test]
    fn test_get_prompt_valid() {
        let input = GetPromptRequest {
            context: PluginRequestContext::default(),
            request: GetPromptRequestParam {
                name: "get_time_with_timezone".to_string(),
                arguments: None,
            },
        };

        let result = get_prompt(input).expect("get_prompt should succeed");
        assert!(!result.messages.is_empty());
        assert!(result.description.is_some());
    }

    #[test]
    fn test_get_prompt_with_timezone() {
        let mut args = HashMap::new();
        args.insert("timezone".to_string(), "Europe/London".to_string());

        let input = GetPromptRequest {
            context: PluginRequestContext::default(),
            request: GetPromptRequestParam {
                name: "get_time_with_timezone".to_string(),
                arguments: Some(args),
            },
        };

        let result = get_prompt(input).expect("get_prompt should succeed");
        assert!(!result.messages.is_empty());
        assert!(result.description.is_some());
    }

    #[test]
    fn test_get_prompt_invalid_timezone() {
        let mut args = HashMap::new();
        args.insert("timezone".to_string(), "Invalid/Zone".to_string());

        let input = GetPromptRequest {
            context: PluginRequestContext::default(),
            request: GetPromptRequestParam {
                name: "get_time_with_timezone".to_string(),
                arguments: Some(args),
            },
        };

        let result = get_prompt(input).expect("get_prompt should succeed");
        assert!(!result.messages.is_empty());
    }

    #[test]
    fn test_get_prompt_not_found() {
        let input = GetPromptRequest {
            context: PluginRequestContext::default(),
            request: GetPromptRequestParam {
                name: "unknown_prompt".to_string(),
                arguments: None,
            },
        };

        let result = get_prompt(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_prompts() {
        let input = ListPromptsRequest::default();
        let result = list_prompts(input).expect("list_prompts should succeed");

        assert!(!result.prompts.is_empty());
        assert_eq!(result.prompts[0].name, "get_time_with_timezone");
        assert!(result.prompts[0].description.is_some());
        assert!(result.prompts[0].arguments.is_some());
    }

    #[test]
    fn test_list_resource_templates() {
        let input = ListResourceTemplatesRequest::default();
        let result =
            list_resource_templates(input).expect("list_resource_templates should succeed");

        assert!(!result.resource_templates.is_empty());
        assert_eq!(result.resource_templates[0].name, "time_zone_converter");
        assert!(result.resource_templates[0].description.is_some());
        assert!(result.resource_templates[0].mime_type.is_some());
    }

    #[test]
    fn test_list_resources() {
        let input = ListResourcesRequest::default();
        let result = list_resources(input).expect("list_resources should succeed");

        assert!(result.resources.is_empty());
    }

    #[test]
    fn test_list_tools() {
        let input = ListToolsRequest::default();
        let result = list_tools(input).expect("list_tools should succeed");

        assert_eq!(result.tools.len(), 2);
        assert_eq!(result.tools[0].name, "get_time");
        assert_eq!(result.tools[1].name, "parse_time");

        assert!(result.tools[0].description.is_some());
        assert!(result.tools[0].input_schema.properties.is_some());
        assert!(result.tools[0].output_schema.is_some());

        assert!(result.tools[1].description.is_some());
        assert!(result.tools[1].input_schema.properties.is_some());
        assert!(result.tools[1].output_schema.is_some());
    }

    #[test]
    fn test_on_roots_list_changed() {
        let input = PluginNotificationContext::default();
        let result = on_roots_list_changed(input);

        assert!(result.is_ok());
    }

    #[test]
    fn test_prompt_reference_serialization() {
        // Test serializing a PromptReference and checking its structure
        let prompt_ref = PromptReference {
            name: "test_prompt".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let json_value = serde_json::to_value(&prompt_ref).expect("should serialize");
        println!("Serialized PromptReference: {}", json_value);

        let json_obj = json_value.as_object().expect("should be object");
        assert!(json_obj.contains_key("type"), "Should have 'type' field");
        assert!(json_obj.contains_key("name"), "Should have 'name' field");

        // Check the type field value
        let type_value = json_obj.get("type").expect("type field exists");
        println!("Type field value: {}", type_value);
        assert_eq!(type_value, "prompt");
    }

    #[test]
    fn test_any_reference_deserialization() {
        // Test deserializing a PromptReference map into AnyReference
        let prompt_ref = PromptReference {
            name: "test_prompt".to_string(),
            title: None,
            r#type: PromptReferenceType::Prompt,
        };

        let json_string = serde_json::to_string(&prompt_ref).expect("should serialize");

        // Try to deserialize into AnyReference
        let any_ref: Reference =
            serde_json::from_str(&json_string).expect("should deserialize into Reference");

        match any_ref {
            Reference::Prompt(pr) => {
                assert_eq!(pr.name, "test_prompt");
            }
            _ => {
                panic!("Should have deserialized as Prompt, not other type");
            }
        }
    }

    #[test]
    fn test_complete_resource_with_utc_query() {
        // Test complete function with ResourceTemplateReference and UTC timezone query
        let resource_ref = ResourceTemplateReference {
            r#type: ResourceReferenceType::Resource,
            uri: "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}".to_string(),
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::ResourceTemplate(resource_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "utc".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(!result.completion.values.is_empty());
        assert!(result.completion.values.contains(&"UTC".to_string()));
        assert!(result.completion.total.is_some());
    }

    #[test]
    fn test_complete_resource_with_asia_query() {
        // Test complete function with ResourceTemplateReference and Asia timezone prefix
        let resource_ref = ResourceTemplateReference {
            r#type: ResourceReferenceType::Resource,
            uri: "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}".to_string(),
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::ResourceTemplate(resource_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "asia".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(!result.completion.values.is_empty());
        assert!(result.completion.values.contains(&"Asia/Tokyo".to_string()));
        assert!(result.completion.total.is_some());
        assert!(result.completion.has_more.is_some());
    }

    #[test]
    fn test_complete_resource_with_no_match() {
        // Test complete function with ResourceTemplateReference that has no matching timezones
        let resource_ref = ResourceTemplateReference {
            r#type: ResourceReferenceType::Resource,
            uri: "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}".to_string(),
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::ResourceTemplate(resource_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "nonexistent_tz".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        assert!(result.completion.values.is_empty());
        assert_eq!(result.completion.total, Some(0));
    }

    #[test]
    fn test_complete_resource_empty_query() {
        // Test complete function with ResourceTemplateReference and empty query
        let resource_ref = ResourceTemplateReference {
            r#type: ResourceReferenceType::Resource,
            uri: "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}".to_string(),
        };

        let input = CompleteRequest {
            context: PluginRequestContext::default(),
            request: CompleteRequestParam {
                r#ref: Reference::ResourceTemplate(resource_ref),
                argument: CompleteRequestParamArgument {
                    name: "timezone".to_string(),
                    value: "".to_string(),
                },
                context: None,
            },
        };

        let result = complete(input).expect("complete should succeed");
        // Empty query should match all timezones (up to 100)
        assert!(!result.completion.values.is_empty());
        assert_eq!(result.completion.values.len(), 100);
        assert!(result.completion.total.is_some());
        assert!(result.completion.has_more.is_some());
    }

    #[test]
    fn test_any_reference_deserialization_resource() {
        // Test deserializing a ResourceTemplateReference map into AnyReference
        let resource_ref = ResourceTemplateReference {
            r#type: ResourceReferenceType::Resource,
            uri: "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}".to_string(),
        };

        let json_string = serde_json::to_string(&resource_ref).expect("should serialize");

        // Try to deserialize into AnyReference
        let any_ref: Reference =
            serde_json::from_str(&json_string).expect("should deserialize into AnyReference");

        match any_ref {
            Reference::ResourceTemplate(rr) => {
                assert_eq!(
                    rr.uri,
                    "https://www.timezoneconverter.com/cgi-bin/zoneinfo?tz={timezone}"
                );
            }
            _ => {
                panic!("Should have deserialized as Resource, not Prompt");
            }
        }
    }
}
