mod pdk;

use extism_pdk::*;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use serde_json::{Value as JsonValue, json};
use urlencoding::encode;

const CONTEXT7_API_BASE_URL: &str = "https://context7.com/api"; // Guessed API base URL

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    match input.params.name.as_str() {
        "c7_resolve_library_id" => c7_resolve_library_id(input),
        "c7_get_library_docs" => c7_get_library_docs(input),
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

fn c7_resolve_library_id(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    let library_name_val = args.get("library_name").unwrap_or(&JsonValue::Null);

    if let JsonValue::String(library_name_as_query) = library_name_val {
        let encoded_query = encode(library_name_as_query);
        let url = format!(
            "{}/v1/search?query={}",
            CONTEXT7_API_BASE_URL, encoded_query
        );

        let mut req = HttpRequest::new(&url).with_method("GET");
        req.headers
            .insert("X-Context7-Source".to_string(), "mcp-server".to_string());

        match http::request::<()>(&req, None) {
            Ok(res) => {
                let body_str = String::from_utf8_lossy(&res.body()).to_string();
                if res.status_code() >= 200 && res.status_code() < 300 {
                    match serde_json::from_str::<JsonValue>(&body_str) {
                        Ok(parsed_json) => {
                            let mut results_text_parts = Vec::new();

                            // Check if the root is an object and has a "results" field which is an array
                            if let Some(results_node) = parsed_json.get("results") {
                                if let JsonValue::Array(results_array) = results_node {
                                    if results_array.is_empty() {
                                        results_text_parts.push(
                                            "No libraries found matching your query.".to_string(),
                                        );
                                    } else {
                                        for result_item in results_array {
                                            let mut item_details = Vec::new();

                                            let title = result_item
                                                .get("title")
                                                .and_then(JsonValue::as_str)
                                                .unwrap_or("N/A");
                                            item_details.push(format!("- Title: {}", title));

                                            let id = result_item
                                                .get("id")
                                                .and_then(JsonValue::as_str)
                                                .unwrap_or("N/A");
                                            item_details.push(format!(
                                                "- Context7-compatible library ID: {}",
                                                id
                                            ));

                                            let description = result_item
                                                .get("description")
                                                .and_then(JsonValue::as_str)
                                                .unwrap_or("N/A");
                                            item_details
                                                .push(format!("- Description: {}", description));

                                            if let Some(v) = result_item
                                                .get("totalSnippets")
                                                .and_then(JsonValue::as_i64)
                                                .filter(|&v| v >= 0)
                                            {
                                                item_details.push(format!("- Code Snippets: {}", v))
                                            }

                                            if let Some(v) = result_item
                                                .get("stars")
                                                .and_then(JsonValue::as_i64)
                                                .filter(|&v| v >= 0)
                                            {
                                                item_details.push(format!("- GitHub Stars: {}", v))
                                            }

                                            results_text_parts.push(item_details.join("\n"));
                                        }
                                    }
                                } else {
                                    results_text_parts.push("API response 'results' field was not an array as expected.".to_string());
                                }
                            } else {
                                results_text_parts.push(
                                    "API response did not contain a 'results' field as expected."
                                        .to_string(),
                                );
                            }

                            let header = "Available Libraries (top matches):\n\nEach result includes information like:\n- Title: Library or package name\n- Context7-compatible library ID: Identifier (format: /org/repo)\n- Description: Short summary\n- Code Snippets: Number of available code examples (if available)\n- GitHub Stars: Popularity indicator (if available)\n\nFor best results, select libraries based on name match, popularity (stars), snippet coverage, and relevance to your use case.\n\n---\n";
                            let final_text =
                                format!("{}{}", header, results_text_parts.join("\n\n"));

                            Ok(CallToolResult {
                                is_error: None,
                                content: vec![Content {
                                    annotations: None,
                                    text: Some(final_text),
                                    mime_type: Some("text/markdown".to_string()),
                                    r#type: ContentType::Text,
                                    data: None,
                                }],
                            })
                        }
                        Err(e) => {
                            // Failed to parse the JSON body
                            Ok(CallToolResult {
                                is_error: Some(true),
                                content: vec![Content {
                                    annotations: None,
                                    text: Some(format!(
                                        "Failed to parse API response JSON: {}. Body: {}",
                                        e, body_str
                                    )),
                                    mime_type: None,
                                    r#type: ContentType::Text,
                                    data: None,
                                }],
                            })
                        }
                    }
                } else {
                    Ok(CallToolResult {
                        is_error: Some(true),
                        content: vec![Content {
                            annotations: None,
                            text: Some(format!(
                                "API request failed with status {}: {}",
                                res.status_code(),
                                body_str
                            )),
                            mime_type: None,
                            r#type: ContentType::Text,
                            data: None,
                        }],
                    })
                }
            }
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("HTTP request failed: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some(
                    "Missing required parameter: library_name (or not a string)".to_string(),
                ),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn c7_get_library_docs(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    let library_id_json_val = args
        .get("context7_compatible_library_id")
        .unwrap_or(&JsonValue::Null);

    if let JsonValue::String(original_id_str) = library_id_json_val {
        let mut id_for_path = original_id_str.clone();
        let mut folders_value_opt: Option<String> = None;

        if let Some(idx) = original_id_str.rfind("?folders=") {
            let (id_part, folders_part_with_query) = original_id_str.split_at(idx);
            id_for_path = id_part.to_string();
            folders_value_opt = Some(
                folders_part_with_query
                    .trim_start_matches("?folders=")
                    .to_string(),
            );
        }

        let mut query_params_vec = vec![format!(
            "context7CompatibleLibraryID={}",
            encode(original_id_str) // Use the original, full ID string for this query parameter
        )];

        if let Some(folders_val) = &folders_value_opt {
            if !folders_val.is_empty() {
                query_params_vec.push(format!("folders={}", encode(folders_val)));
            }
        }

        if let Some(JsonValue::String(topic_str)) = args.get("topic") {
            if !topic_str.is_empty() {
                // Ensure topic is not empty before adding
                query_params_vec.push(format!("topic={}", encode(topic_str)));
            }
        }

        if let Some(JsonValue::Number(tokens_num_json)) = args.get("tokens") {
            if let Some(tokens_f64) = tokens_num_json.as_f64() {
                query_params_vec.push(format!("tokens={}", tokens_f64 as i64));
            } else if let Some(tokens_i64) = tokens_num_json.as_i64() {
                query_params_vec.push(format!("tokens={}", tokens_i64));
            }
        }

        let final_id_for_path_segment = id_for_path.strip_prefix("/").unwrap_or(&id_for_path);

        let query_params = query_params_vec.join("&");
        let url = format!(
            "{}/v1/{}/?{}", // Corrected URL: ensure '?' before query parameters
            CONTEXT7_API_BASE_URL, final_id_for_path_segment, query_params
        );

        let mut req = HttpRequest::new(&url).with_method("GET");
        req.headers
            .insert("X-Context7-Source".to_string(), "mcp-server".to_string());

        match http::request::<()>(&req, None) {
            Ok(res) => {
                let body_str = String::from_utf8_lossy(&res.body()).to_string();
                if res.status_code() >= 200 && res.status_code() < 300 {
                    // Directly use the body_str as markdown content
                    Ok(CallToolResult {
                        is_error: None,
                        content: vec![Content {
                            annotations: None,
                            text: Some(body_str),
                            mime_type: Some("text/markdown".to_string()), // Assuming it's still markdown
                            r#type: ContentType::Text,
                            data: None,
                        }],
                    })
                } else {
                    Ok(CallToolResult {
                        is_error: Some(true),
                        content: vec![Content {
                            annotations: None,
                            text: Some(format!(
                                "API request for docs (URL: {}) failed with status {}: {}",
                                url,
                                res.status_code(),
                                body_str
                            )),
                            mime_type: None,
                            r#type: ContentType::Text,
                            data: None,
                        }],
                    })
                }
            }
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("HTTP request for docs failed: {}, URL: {}", e, url)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some(
                    "Missing required parameter: context7_compatible_library_id (or not a string)"
                        .to_string(),
                ),
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
                name: "c7_resolve_library_id".into(),
                description: "Resolves a package name to a Context7-compatible library ID and returns a list of matching libraries. You MUST call this function before 'c7_get_library_docs' to obtain a valid Context7-compatible library ID. When selecting the best match, consider: - Name similarity to the query - Description relevance - Code Snippet count (documentation coverage) - GitHub Stars (popularity) Return the selected library ID and explain your choice. If there are multiple good matches, mention this but proceed with the most relevant one.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "library_name": {
                            "type": "string",
                            "description": "Library name to search for and retrieve a Context7-compatible library ID.",
                        },
                    },
                    "required": ["library_name"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "c7_get_library_docs".into(),
                description: "Fetches up-to-date documentation for a library. You must call 'c7_resolve_library_id' first to obtain the exact Context7-compatible library ID required to use this tool.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "context7_compatible_library_id": {
                            "type": "string",
                            "description": "Exact Context7-compatible library ID (e.g., 'mongodb/docs', 'vercel/nextjs') retrieved from 'c7_resolve_library_id'.",
                        },
                        "topic": {
                            "type": "string",
                            "description": "Topic to focus documentation on (e.g., 'hooks', 'routing').",
                        },
                        "tokens": {
                            "type": "integer",
                            "description": "Maximum number of tokens of documentation to retrieve (default: 10000). Higher values provide more context but consume more tokens.",
                        },
                    },
                    "required": ["context7_compatible_library_id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
