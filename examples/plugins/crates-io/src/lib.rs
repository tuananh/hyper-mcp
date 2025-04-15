mod pdk;

use std::collections::BTreeMap;

use dom_smoothie::{Article, Config, Readability};
use extism_pdk::*;
use json::Value;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use serde_json::json;

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    match input.params.name.as_str() {
        "crates_io_latest_version" => latest_version(input),
        "crates_io_crate_info" => crate_info(input),
        "crates_io_docs" => crate_docs(input),
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

fn crate_info(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    if let Some(Value::String(crate_names)) = args.get("crate_names") {
        let crate_names: Vec<&str> = crate_names.split(',').map(|s| s.trim()).collect();
        let mut results = Vec::new();

        for crate_name in crate_names {
            // Create HTTP request to crates.io API
            let mut req = HttpRequest {
                url: format!("https://crates.io/api/v1/crates/{}", crate_name),
                headers: BTreeMap::new(),
                method: Some("GET".to_string()),
            };

            // Add a user agent header to be polite
            req.headers
                .insert("User-Agent".to_string(), "hyper-mcp/1.0".to_string());

            // Perform the request
            let res = http::request::<()>(&req, None)?;

            // Convert response body to string
            let body = res.body();
            let json_str = String::from_utf8_lossy(body.as_slice());

            // Parse the JSON response
            let json: serde_json::Value = serde_json::from_str(&json_str)?;

            if let Some(crate_info) = json["crate"].as_object() {
                // Extract relevant information with null checks
                let info = json!({
                    "name": crate_info.get("name").and_then(|v| v.as_str()),
                    "description": crate_info.get("description").and_then(|v| v.as_str()),
                    "latest_version": crate_info.get("max_version").and_then(|v| v.as_str()),
                    "downloads": crate_info.get("downloads").and_then(|v| v.as_i64()),
                    "repository": crate_info.get("repository").and_then(|v| v.as_str()),
                    "documentation": crate_info.get("documentation").and_then(|v| v.as_str()),
                    "homepage": crate_info.get("homepage").and_then(|v| v.as_str()),
                    "keywords": crate_info.get("keywords").and_then(|v| v.as_array()),
                    "categories": crate_info.get("categories").and_then(|v| v.as_array()),
                    "license": json["versions"].as_array().and_then(|v| v.first()).and_then(|v| v["license"].as_str()),
                    "created_at": crate_info.get("created_at").and_then(|v| v.as_str()),
                    "updated_at": crate_info.get("updated_at").and_then(|v| v.as_str()),
                });

                results.push(info);
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
                    text: Some("Failed to get crate information".into()),
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
                text: Some("Please provide crate names".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn latest_version(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    if let Some(Value::String(crate_names)) = args.get("crate_names") {
        let crate_names: Vec<&str> = crate_names.split(',').map(|s| s.trim()).collect();
        let mut results = BTreeMap::new();

        for crate_name in crate_names {
            // Create HTTP request to crates.io API
            let mut req = HttpRequest {
                url: format!("https://crates.io/api/v1/crates/{}", crate_name),
                headers: BTreeMap::new(),
                method: Some("GET".to_string()),
            };

            // Add a user agent header to be polite
            req.headers
                .insert("User-Agent".to_string(), "hyper-mcp/1.0".to_string());

            // Perform the request
            let res = http::request::<()>(&req, None)?;

            // Convert response body to string
            let body = res.body();
            let json_str = String::from_utf8_lossy(body.as_slice());

            // Parse the JSON response
            let json: serde_json::Value = serde_json::from_str(&json_str)?;

            if let Some(version) = json["crate"]["max_version"].as_str() {
                results.insert(crate_name.to_string(), version.to_string());
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
                text: Some("Please provide crate names".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn extract_readable_content(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cfg = Config {
        max_elements_to_parse: 9000,
        ..Default::default()
    };

    let mut readability = Readability::new(html, None, Some(cfg))?;
    let article: Article = readability.parse()?;

    let formatted = format!(
        "# {}\n\n{}\n\n---\n\n{}",
        article.title,
        article.excerpt.unwrap(),
        article.content
    );

    Ok(formatted)
}

fn crate_docs(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    if let Some(Value::String(crate_names)) = args.get("crate_names") {
        let crate_names: Vec<&str> = crate_names.split(',').map(|s| s.trim()).collect();
        let mut results = Vec::new();

        for crate_name in crate_names {
            // First get the crate info to determine documentation source
            let mut req = HttpRequest {
                url: format!("https://crates.io/api/v1/crates/{}", crate_name),
                headers: BTreeMap::new(),
                method: Some("GET".to_string()),
            };
            req.headers
                .insert("User-Agent".to_string(), "hyper-mcp/1.0".to_string());

            let res = http::request::<()>(&req, None)?;
            let body = res.body();
            let json_str = String::from_utf8_lossy(body.as_slice());
            let json: serde_json::Value = serde_json::from_str(&json_str)?;

            if let Some(crate_info) = json["crate"].as_object() {
                let mut docs_url = None;

                // First try to get custom documentation URL
                if let Some(documentation) =
                    crate_info.get("documentation").and_then(|v| v.as_str())
                {
                    docs_url = Some(documentation.to_string());
                } else {
                    // If no custom docs, use docs.rs URL with latest version
                    if let Some(version) = crate_info.get("max_version").and_then(|v| v.as_str()) {
                        docs_url = Some(format!("https://docs.rs/{}/{}", crate_name, version));
                    }
                }

                if let Some(url) = docs_url {
                    // Now fetch the actual documentation
                    let mut doc_req = HttpRequest {
                        url,
                        headers: BTreeMap::new(),
                        method: Some("GET".to_string()),
                    };
                    doc_req
                        .headers
                        .insert("User-Agent".to_string(), "hyper-mcp/1.0".to_string());

                    match http::request::<()>(&doc_req, None) {
                        Ok(doc_res) => {
                            let doc_body =
                                String::from_utf8_lossy(doc_res.body().as_slice()).into_owned();

                            // Process the HTML into readable text
                            match extract_readable_content(&doc_body) {
                                Ok(readable_content) => {
                                    // Create a result object with metadata and content
                                    let result = json!({
                                        "crate_name": crate_name,
                                        "version": crate_info.get("max_version").and_then(|v| v.as_str()),
                                        "documentation_url": doc_req.url,
                                        "content": readable_content
                                    });

                                    results.push(result);
                                }
                                Err(e) => {
                                    // If we can't process the content, include error in results
                                    let error_result = json!({
                                        "crate_name": crate_name,
                                        "error": format!("Failed to process documentation: {}", e)
                                    });
                                    results.push(error_result);
                                }
                            }
                        }
                        Err(e) => {
                            // If we can't fetch the docs, include error in results
                            let error_result = json!({
                                "crate_name": crate_name,
                                "error": format!("Failed to fetch documentation: {}", e)
                            });
                            results.push(error_result);
                        }
                    }
                } else {
                    // No documentation URL available
                    let error_result = json!({
                        "crate_name": crate_name,
                        "error": "No documentation URL available"
                    });
                    results.push(error_result);
                }
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
                    text: Some("Failed to get documentation for any crates".into()),
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
                text: Some("Please provide crate names".into()),
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
                name: "crates_io_latest_version".into(),
                description: "Fetches the latest version of multiple crates from crates.io".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "crate_names": {
                            "type": "string",
                            "description": "Comma-separated list of crate names to get the latest versions for",
                        },
                    },
                    "required": ["crate_names"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "crates_io_crate_info".into(),
                description: "Fetches detailed information about multiple crates from crates.io".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "crate_names": {
                            "type": "string",
                            "description": "Comma-separated list of crate names to get information for",
                        },
                    },
                    "required": ["crate_names"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "crates_io_docs".into(),
                description: "Fetches and extracts readable documentation content for multiple crates".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "crate_names": {
                            "type": "string",
                            "description": "Comma-separated list of crate names to get documentation for",
                        },
                    },
                    "required": ["crate_names"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
