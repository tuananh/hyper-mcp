mod pdk;

use std::collections::BTreeMap;

use base64::prelude::*;
use extism_pdk::*;
use json::Value;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use serde_json::json;
use termtree::Tree;

// Helper struct for deserializing GitLab API response
// https://docs.gitlab.com/api/repositories/#list-repository-tree
#[derive(serde::Deserialize)]
struct GitLabRepoEntry {
    id: String,
    name: String,
    r#type: String, // "tree" or "blob"
    path: String,
    mode: String,
}

// Helper struct for building the tree
#[derive(Debug)]
struct FileTreeNode {
    name: String,
    children: BTreeMap<String, FileTreeNode>,
}

impl FileTreeNode {
    fn new(name: &str) -> Self {
        FileTreeNode {
            name: name.to_string(),
            children: BTreeMap::new(),
        }
    }

    fn insert_path(&mut self, path_segments: &[&str]) {
        if path_segments.is_empty() {
            return;
        }
        let current_segment = path_segments[0];
        let node = self
            .children
            .entry(current_segment.to_string())
            .or_insert_with(|| FileTreeNode::new(current_segment));

        if path_segments.len() > 1 {
            node.insert_path(&path_segments[1..]);
        }
    }
}

// Renamed and modified function to convert FileTreeNode to termtree::Tree<String>
fn convert_file_tree_to_termtree(file_node: &FileTreeNode) -> Tree<String> {
    let mut tree_node = Tree::new(file_node.name.clone());
    for child_file_node in file_node.children.values() {
        // Iterate over sorted children
        tree_node.push(convert_file_tree_to_termtree(child_file_node));
    }
    tree_node
}

// New function to build and format the tree
fn build_and_format_tree_from_entries(
    entries: Vec<GitLabRepoEntry>,
    requested_path_opt: Option<&str>,
    project_id_str: &str,
) -> Result<String, String> {
    if entries.is_empty() {
        return Ok("Repository tree is empty or path not found.".to_string());
    }

    let root_display_name = match requested_path_opt {
        Some(req_path) if !req_path.is_empty() => req_path
            .split('/')
            .next_back()
            .unwrap_or("root")
            .to_string(),
        _ => project_id_str
            .split('/')
            .next_back()
            .unwrap_or("root")
            .to_string(),
    };

    let mut root_node = FileTreeNode::new(&root_display_name);

    for entry in entries {
        let effective_path = match requested_path_opt {
            Some(base_path_val)
                if !base_path_val.is_empty() && entry.path.starts_with(base_path_val) =>
            {
                entry
                    .path
                    .strip_prefix(base_path_val)
                    .unwrap_or(&entry.path)
                    .trim_start_matches('/')
                    .to_string()
            }
            _ => entry.path.clone(),
        };

        if effective_path.is_empty() {
            continue;
        }

        let path_segments: Vec<&str> = effective_path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        if !path_segments.is_empty() {
            root_node.insert_path(&path_segments);
        }
    }

    let termtree_root = convert_file_tree_to_termtree(&root_node); // Use the new conversion function
    Ok(termtree_root.to_string())
}

fn get_gitlab_config() -> Result<(String, String), Error> {
    let token = config::get("GITLAB_TOKEN")?
        .ok_or_else(|| Error::msg("GITLAB_TOKEN configuration is required but not set"))?;

    let url = config::get("GITLAB_URL")?.unwrap_or_else(|| "https://gitlab.com/api/v4".to_string());

    Ok((token, url))
}

/// Helper function to check if an HTTP status code represents success (200-299)
fn is_success_status(status_code: u16) -> bool {
    (200..300).contains(&status_code)
}

fn urlencode_if_needed(input: &str) -> String {
    if input.contains("/") {
        urlencoding::encode(input).to_string()
    } else {
        input.to_string()
    }
}

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    info!("call: {:?}", input);
    match input.params.name.as_str() {
        // Issues
        "gl_create_issue" => create_issue(input),
        "gl_get_issue" => get_issue(input),
        "gl_update_issue" => update_issue(input),
        "gl_add_issue_comment" => add_issue_comment(input),
        "gl_list_issues" => gl_list_issues(input),

        // Files
        "gl_get_file_contents" => get_file_contents(input),
        "gl_create_or_update_file" => create_or_update_file(input),
        "gl_delete_file" => delete_file(input),

        // Branches
        "gl_create_branch" => create_branch(input),
        "gl_list_branches" => gl_list_branches(input),
        "gl_create_merge_request" => create_merge_request(input),
        "gl_update_merge_request" => update_merge_request(input),
        "gl_get_merge_request" => gl_get_merge_request(input),

        // Snippets (GitLab equivalent of Gists)
        "gl_create_snippet" => create_snippet(input),
        "gl_update_snippet" => update_snippet(input),
        "gl_get_snippet" => get_snippet(input),
        "gl_delete_snippet" => delete_snippet(input),

        // Repository tree
        "gl_get_repo_tree" => gl_get_repo_tree(input),

        // Repository members
        "gl_get_repo_members" => gl_get_repo_members(input),

        _ => Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some(format!("Unknown operation: {}", input.params.name)),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        }),
    }
}

fn create_issue(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(project_id)),
        Some(Value::String(title)),
        Some(Value::String(description)),
    ) = (
        args.get("project_id"),
        args.get("title"),
        args.get("description"),
    ) {
        let url = format!(
            "{}/projects/{}/issues",
            gitlab_url,
            urlencode_if_needed(project_id)
        );
        let mut body = json!({
            "title": title,
            "description": description,
        });

        // Add labels if provided
        if let Some(Value::String(labels)) = args.get("labels") {
            body.as_object_mut()
                .unwrap()
                .insert("labels".to_string(), Value::String(labels.clone()));
        }

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("POST".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to create issue: {}", res.status_code())),
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
                text: Some("Please provide project_id, title, and description".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn get_issue(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (Some(Value::String(project_id)), Some(Value::String(issue_iid))) =
        (args.get("project_id"), args.get("issue_iid"))
    {
        let url = format!(
            "{}/projects/{}/issues/{}",
            gitlab_url,
            urlencode_if_needed(project_id),
            issue_iid
        );

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("GET".to_string()),
        };

        let res = http::request::<()>(&req, None)?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to get issue: {}", res.status_code())),
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
                text: Some("Please provide project_id and issue_iid".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn update_issue(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (Some(Value::String(project_id)), Some(Value::String(issue_iid))) =
        (args.get("project_id"), args.get("issue_iid"))
    {
        let url = format!(
            "{}/projects/{}/issues/{}",
            gitlab_url,
            urlencode_if_needed(project_id),
            issue_iid
        );

        let mut body_map = serde_json::Map::new();
        if let Some(Value::String(title)) = args.get("title") {
            body_map.insert("title".to_string(), json!(title));
        }
        if let Some(Value::String(description)) = args.get("description") {
            body_map.insert("description".to_string(), json!(description));
        }
        if let Some(Value::String(add_labels)) = args.get("add_labels") {
            body_map.insert("add_labels".to_string(), json!(add_labels));
        }
        if let Some(Value::String(remove_labels)) = args.get("remove_labels") {
            body_map.insert("remove_labels".to_string(), json!(remove_labels));
        }
        if let Some(Value::String(due_date)) = args.get("due_date") {
            body_map.insert("due_date".to_string(), json!(due_date));
        }

        if body_map.is_empty() {
            return Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some("Please provide at least one field to update (e.g., title, description, add_labels, remove_labels, due_date)".into()),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            });
        }

        let body = Value::Object(body_map);

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("PUT".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
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
                        "Failed to update issue: {} - Response: {}",
                        res.status_code(),
                        String::from_utf8_lossy(&res.body())
                    )),
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
                text: Some("Please provide project_id, issue_iid, and at least one field to update (title, description, add_labels, remove_labels, or due_date)".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn add_issue_comment(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(project_id)),
        Some(Value::String(issue_iid)),
        Some(Value::String(comment)),
    ) = (
        args.get("project_id"),
        args.get("issue_iid"),
        args.get("comment"),
    ) {
        let url = format!(
            "{}/projects/{}/issues/{}/notes",
            gitlab_url,
            urlencode_if_needed(project_id),
            issue_iid
        );
        let body = json!({
            "body": comment,
        });

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("POST".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to add comment: {}", res.status_code())),
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
                text: Some("Please provide project_id, issue_iid, and comment".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn get_file_contents(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (Some(Value::String(project_id)), Some(Value::String(file_path))) =
        (args.get("project_id"), args.get("file_path"))
    {
        let ref_name = args.get("ref").and_then(|v| v.as_str()).unwrap_or("HEAD");

        let url = format!(
            "{}/projects/{}/repository/files/{}?ref={}",
            gitlab_url,
            urlencode_if_needed(project_id),
            urlencode_if_needed(file_path),
            ref_name
        );

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url: url.clone(),
            headers,
            method: Some("GET".to_string()),
        };

        let res = http::request::<()>(&req, None)?;

        if is_success_status(res.status_code()) {
            // Parse the response to get the file content from the "content" field
            if let Ok(json) = serde_json::from_slice::<Value>(&res.body()) {
                if let Some(content) = json.get("content").and_then(|v| v.as_str()) {
                    // Decode base64 content
                    match BASE64_STANDARD.decode(content.as_bytes()) {
                        Ok(decoded_bytes) => {
                            if let Ok(decoded_content) = String::from_utf8(decoded_bytes) {
                                return Ok(CallToolResult {
                                    is_error: None,
                                    content: vec![Content {
                                        annotations: None,
                                        text: Some(decoded_content),
                                        mime_type: Some("text/plain".to_string()),
                                        r#type: ContentType::Text,
                                        data: None,
                                    }],
                                });
                            }
                        }
                        Err(e) => {
                            return Ok(CallToolResult {
                                is_error: Some(true),
                                content: vec![Content {
                                    annotations: None,
                                    text: Some(format!("Failed to decode base64 content: {}", e)),
                                    mime_type: None,
                                    r#type: ContentType::Text,
                                    data: None,
                                }],
                            });
                        }
                    }
                }
            }

            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some("Failed to parse file contents from response".into()),
                    mime_type: None,
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
                        "Failed to get file contents: {} {}",
                        url.clone(),
                        res.status_code()
                    )),
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
                text: Some("Please provide project_id and file_path".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn delete_file(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(project_id)),
        Some(Value::String(file_path)),
        Some(Value::String(branch)),
    ) = (
        args.get("project_id"),
        args.get("file_path"),
        args.get("branch"),
    ) {
        let commit_message = args
            .get("commit_message")
            .and_then(|v| v.as_str())
            .unwrap_or("Delete file via API")
            .to_string();

        let url = format!(
            "{}/projects/{}/repository/files/{}",
            gitlab_url,
            urlencode_if_needed(project_id),
            urlencode_if_needed(file_path)
        );

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let mut body_map = serde_json::Map::new();
        body_map.insert("branch".to_string(), json!(branch));
        body_map.insert("commit_message".to_string(), json!(commit_message));
        if let Some(Value::String(author_email)) = args.get("author_email") {
            body_map.insert("author_email".to_string(), json!(author_email));
        }
        if let Some(Value::String(author_name)) = args.get("author_name") {
            body_map.insert("author_name".to_string(), json!(author_name));
        }
        let body = Value::Object(body_map);

        let req = HttpRequest {
            url,
            headers,
            method: Some("DELETE".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
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
                        "Failed to delete file (status {}): {}",
                        res.status_code(),
                        String::from_utf8_lossy(&res.body())
                    )),
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
                text: Some("Please provide project_id, file_path, and branch".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn create_or_update_file(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(project_id)),
        Some(Value::String(file_path)),
        Some(Value::String(content)),
        Some(Value::String(branch)),
    ) = (
        args.get("project_id"),
        args.get("file_path"),
        args.get("content"),
        args.get("branch"),
    ) {
        let commit_message = args
            .get("commit_message")
            .and_then(|v| v.as_str())
            .unwrap_or("Update file via API")
            .to_string();

        // URL for checking file existence. Note: GitLab GET file API needs ref in query.
        let check_file_url = format!(
            "{}/projects/{}/repository/files/{}?ref={}",
            gitlab_url,
            urlencode_if_needed(project_id),
            urlencode_if_needed(file_path),
            branch
        );

        let mut headers_check = BTreeMap::new();
        headers_check.insert("PRIVATE-TOKEN".to_string(), token.clone());
        headers_check.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let check_req = HttpRequest {
            url: check_file_url,
            headers: headers_check,
            method: Some("GET".to_string()),
        };

        let check_res = http::request::<()>(&check_req, None)?;

        let http_method = match check_res.status_code() {
            200 => "PUT",  // File exists, so update
            404 => "POST", // File does not exist, so create
            _ => {
                return Ok(CallToolResult {
                    is_error: Some(true),
                    content: vec![Content {
                        annotations: None,
                        text: Some(format!(
                            "Failed to check file existence (status {} on GET {}): {}",
                            check_res.status_code(),
                            check_req.url,
                            String::from_utf8_lossy(&check_res.body())
                        )),
                        mime_type: None,
                        r#type: ContentType::Text,
                        data: None,
                    }],
                });
            }
        };

        // URL for POST/PUT operations (does not have ref in query string, ref is in body via 'branch' parameter)
        // Ensure file_path is URL encoded for this URL as well.
        let operation_url = format!(
            "{}/projects/{}/repository/files/{}",
            gitlab_url,
            urlencode_if_needed(project_id),
            urlencode_if_needed(file_path)
        );

        let mut body_map = serde_json::Map::new();
        body_map.insert("branch".to_string(), json!(branch));
        body_map.insert("content".to_string(), json!(content));
        body_map.insert("commit_message".to_string(), json!(commit_message));

        if let Some(Value::String(author_email)) = args.get("author_email") {
            body_map.insert("author_email".to_string(), json!(author_email));
        }
        if let Some(Value::String(author_name)) = args.get("author_name") {
            body_map.insert("author_name".to_string(), json!(author_name));
        }
        // Note: For 'POST' (create), 'encoding' can be 'base64'.
        // GitLab API often expects content to be base64 encoded for new files if not plain text.
        // For simplicity, we assume content is plain text, and GitLab handles it.
        // If issues arise with binary or special characters, 'content' might need explicit base64 encoding
        // and adding "encoding": "base64" to body_map.

        let body = Value::Object(body_map);

        let mut headers_op = BTreeMap::new();
        headers_op.insert("PRIVATE-TOKEN".to_string(), token);
        headers_op.insert("Content-Type".to_string(), "application/json".to_string());
        headers_op.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url: operation_url.clone(),
            headers: headers_op,
            method: Some(http_method.to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
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
                        "Failed to {} file (method {}, status {} on {}): Response: {}",
                        if http_method == "POST" {
                            "create"
                        } else {
                            "update"
                        },
                        http_method,
                        res.status_code(),
                        req.url,
                        String::from_utf8_lossy(&res.body())
                    )),
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
                text: Some("Please provide project_id, file_path, content, and branch".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn create_branch(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(project_id)),
        Some(Value::String(branch_name)),
        Some(Value::String(ref_name)),
    ) = (
        args.get("project_id"),
        args.get("branch_name"),
        args.get("ref"),
    ) {
        let url = format!(
            "{}/projects/{}/repository/branches",
            gitlab_url,
            urlencode_if_needed(project_id)
        );
        let body = json!({
            "branch": branch_name,
            "ref": ref_name
        });

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("POST".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to create branch: {}", res.status_code())),
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
                text: Some("Please provide project_id, branch_name, and ref".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn create_merge_request(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(project_id)),
        Some(Value::String(source_branch)),
        Some(Value::String(target_branch)),
    ) = (
        args.get("project_id"),
        args.get("source_branch"),
        args.get("target_branch"),
    ) {
        let url = format!(
            "{}/projects/{}/merge_requests",
            gitlab_url,
            urlencode_if_needed(project_id)
        );

        // Use provided title if present, otherwise use default format
        let title = args
            .get("title")
            .and_then(|t| t.as_str())
            .map(|t| t.to_string())
            .unwrap_or_else(|| format!("Merge {} into {}", source_branch, target_branch));

        let body = json!({
            "source_branch": source_branch,
            "target_branch": target_branch,
            "title": title,
        });

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("POST".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
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
                        "Failed to create merge request: {}",
                        res.status_code()
                    )),
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
                text: Some("Please provide project_id, source_branch, and target_branch".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn update_merge_request(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(project_id)),
        Some(Value::String(merge_request_iid)),
        Some(Value::String(title)),
        Some(Value::String(description)),
    ) = (
        args.get("project_id"),
        args.get("merge_request_iid"),
        args.get("title"),
        args.get("description"),
    ) {
        let url = format!(
            "{}/projects/{}/merge_requests/{}",
            gitlab_url,
            urlencode_if_needed(project_id),
            merge_request_iid
        );

        let mut body_map = serde_json::Map::new();
        body_map.insert("title".to_string(), json!(title));
        body_map.insert("description".to_string(), json!(description));

        let body = Value::Object(body_map);

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("PUT".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
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
                        "Failed to update merge request: {} - Response: {}",
                        res.status_code(),
                        String::from_utf8_lossy(&res.body())
                    )),
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
                text: Some(
                    "Please provide project_id, merge_request_iid, title, and description".into(),
                ),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn gl_get_merge_request(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (Some(Value::String(project_id)), Some(Value::String(merge_request_iid))) =
        (args.get("project_id"), args.get("merge_request_iid"))
    {
        let url = format!(
            "{}/projects/{}/merge_requests/{}",
            gitlab_url,
            urlencode_if_needed(project_id),
            merge_request_iid
        );

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("GET".to_string()),
        };

        let res = http::request::<()>(&req, None)?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
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
                        "Failed to get merge request: {} - Response: {}",
                        res.status_code(),
                        String::from_utf8_lossy(&res.body())
                    )),
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
                text: Some("Please provide project_id and merge_request_iid".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn create_snippet(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (Some(Value::String(title)), Some(Value::String(content))) =
        (args.get("title"), args.get("content"))
    {
        let url = format!("{}/snippets", gitlab_url);

        // Get visibility from args or default to "private"
        let visibility = args
            .get("visibility")
            .and_then(|v| v.as_str())
            .unwrap_or("private");

        let body = json!({
            "title": title,
            "file_name": format!("{}.txt", title.to_lowercase().replace(" ", "_")),
            "content": content,
            "visibility": visibility
        });

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("POST".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to create snippet: {}", res.status_code())),
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
                text: Some("Please provide title and content".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn update_snippet(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let (
        Some(Value::String(snippet_id)),
        Some(Value::String(title)),
        Some(Value::String(content)),
    ) = (
        args.get("snippet_id"),
        args.get("title"),
        args.get("content"),
    ) {
        let url = format!("{}/snippets/{}", gitlab_url, snippet_id);
        let body = json!({
            "title": title,
            "file_name": format!("{}.txt", title.to_lowercase().replace(" ", "_")),
            "content": content,
        });

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("PUT".to_string()),
        };

        let res = http::request(&req, Some(&body.to_string()))?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to update snippet: {}", res.status_code())),
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
                text: Some("Please provide snippet_id, title, and content".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn get_snippet(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let Some(Value::String(snippet_id)) = args.get("snippet_id") {
        let url = format!("{}/snippets/{}", gitlab_url, snippet_id);

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("GET".to_string()),
        };

        let res = http::request::<()>(&req, None)?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to get snippet: {}", res.status_code())),
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
                text: Some("Please provide snippet_id".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn delete_snippet(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let Some(Value::String(snippet_id)) = args.get("snippet_id") {
        let url = format!("{}/snippets/{}", gitlab_url, snippet_id);

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("DELETE".to_string()),
        };

        let res = http::request::<()>(&req, None)?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some("Snippet deleted successfully".into()),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to delete snippet: {}", res.status_code())),
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
                text: Some("Please provide snippet_id".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn gl_list_branches(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let Some(Value::String(project_id)) = args.get("project_id") {
        let url = format!(
            "{}/projects/{}/repository/branches",
            gitlab_url,
            urlencode_if_needed(project_id)
        );

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("GET".to_string()),
        };

        let res = http::request::<()>(&req, None)?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to list branches: {}", res.status_code())),
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
                text: Some("Please provide project_id".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn gl_list_issues(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let Some(Value::String(project_id)) = args.get("project_id") {
        let mut url_params = vec![];

        if let Some(Value::String(state)) = args.get("state") {
            url_params.push(format!("state={}", state));
        }
        if let Some(Value::String(labels)) = args.get("labels") {
            url_params.push(format!("labels={}", urlencoding::encode(labels)));
        }

        let query_string = if url_params.is_empty() {
            "".to_string()
        } else {
            format!("?{}", url_params.join("&"))
        };

        let url = format!(
            "{}/projects/{}/issues{}",
            gitlab_url,
            urlencode_if_needed(project_id),
            query_string
        );

        let mut headers = BTreeMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token);
        headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

        let req = HttpRequest {
            url,
            headers,
            method: Some("GET".to_string()),
        };

        let res = http::request::<()>(&req, None)?;

        if is_success_status(res.status_code()) {
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(String::from_utf8_lossy(&res.body()).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        } else {
            Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to list issues: {}", res.status_code())),
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
                text: Some("Please provide project_id".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn gl_get_repo_tree(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let Some(Value::String(project_id_val)) = args.get("project_id") {
        let project_id = project_id_val.as_str();
        let requested_path_opt = args.get("path").and_then(|v| v.as_str());
        let ref_name_opt = args.get("ref").and_then(|v| v.as_str());
        let recursive_opt = args.get("recursive").and_then(|v| v.as_bool());

        let mut all_entries: Vec<GitLabRepoEntry> = Vec::new();
        let mut current_page_number: u32 = 1;
        const PER_PAGE_COUNT: u32 = 100; // GitLab's typical max per_page
        const MAX_PAGES: u32 = 100; // Safety break: 100 pages * 100 items/page = 10,000 items

        loop {
            if current_page_number > MAX_PAGES {
                // Log this or return a partial result with a warning if desired
                // For now, just break and use what we have.
                // Consider returning an error if this limit is hit.
                break;
            }

            let mut url_params = vec![
                format!("per_page={}", PER_PAGE_COUNT),
                format!("page={}", current_page_number),
            ];

            if let Some(path_str) = requested_path_opt {
                if !path_str.is_empty() {
                    url_params.push(format!("path={}", urlencoding::encode(path_str)));
                }
            }
            if let Some(ref_name_str) = ref_name_opt {
                url_params.push(format!("ref={}", urlencoding::encode(ref_name_str)));
            }
            if let Some(recursive_bool) = recursive_opt {
                if recursive_bool {
                    url_params.push("recursive=true".to_string());
                }
            }

            let query_string = format!("?{}", url_params.join("&"));
            let url = format!(
                "{}/projects/{}/repository/tree{}",
                gitlab_url,
                urlencode_if_needed(project_id),
                query_string
            );

            let mut headers = BTreeMap::new();
            headers.insert("PRIVATE-TOKEN".to_string(), token.clone()); // Clone token for loop
            headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

            let req = HttpRequest {
                url: url.clone(),
                headers,
                method: Some("GET".to_string()),
            };

            let res = http::request::<()>(&req, None)?;

            if !is_success_status(res.status_code()) {
                return Ok(CallToolResult {
                    is_error: Some(true),
                    content: vec![Content {
                        annotations: None,
                        text: Some(format!(
                            "Failed to get repository tree page {} from {}: {} - Response: {}",
                            current_page_number,
                            req.url,
                            res.status_code(),
                            String::from_utf8_lossy(&res.body())
                        )),
                        mime_type: None,
                        r#type: ContentType::Text,
                        data: None,
                    }],
                });
            }

            match serde_json::from_slice::<Vec<GitLabRepoEntry>>(&res.body()) {
                Ok(page_entries) => {
                    let num_fetched = page_entries.len();
                    all_entries.extend(page_entries);

                    if num_fetched < PER_PAGE_COUNT as usize {
                        break; // Last page fetched
                    }
                }
                Err(e) => {
                    return Ok(CallToolResult {
                        is_error: Some(true),
                        content: vec![Content {
                            annotations: None,
                            text: Some(format!(
                                "Failed to parse repository tree data from GitLab API (page {}): {}",
                                current_page_number, e
                            )),
                            mime_type: None,
                            r#type: ContentType::Text,
                            data: None,
                        }],
                    });
                }
            }
            current_page_number += 1;
        }

        // Proceed with building the tree from all_entries
        match build_and_format_tree_from_entries(all_entries, requested_path_opt, project_id) {
            Ok(tree_string) => Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(tree_string),
                    mime_type: Some("text/plain".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
            Err(e_str) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(e_str),
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
                text: Some("Please provide project_id".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn gl_get_repo_members(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    let (token, gitlab_url) = get_gitlab_config()?;

    if let Some(Value::String(project_id_val)) = args.get("project_id") {
        let project_id = project_id_val.as_str();
        let include_inherited = args
            .get("include_inherited_members")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let query_opt = args.get("query").and_then(|v| v.as_str());

        let members_path = if include_inherited {
            "members/all"
        } else {
            "members"
        };

        let mut all_members_json: Vec<Value> = Vec::new();
        let mut current_page_number: u32 = 1;
        const PER_PAGE_COUNT: u32 = 100; // GitLab's typical max per_page
        const MAX_PAGES: u32 = 100; // Safety break: 100 pages * 100 items/page = 10,000 members

        loop {
            if current_page_number > MAX_PAGES {
                // Log this or return a partial result with a warning if desired
                break;
            }

            let mut url_params = vec![
                format!("per_page={}", PER_PAGE_COUNT),
                format!("page={}", current_page_number),
            ];

            if let Some(query_str) = query_opt {
                url_params.push(format!("query={}", urlencoding::encode(query_str)));
            }

            let query_string = format!("?{}", url_params.join("&"));
            let url = format!(
                "{}/projects/{}/{}{}",
                gitlab_url,
                urlencode_if_needed(project_id),
                members_path,
                query_string
            );

            let mut headers = BTreeMap::new();
            headers.insert("PRIVATE-TOKEN".to_string(), token.clone());
            headers.insert("User-Agent".to_string(), "hyper-mcp/0.1.0".to_string());

            let req = HttpRequest {
                url: url.clone(),
                headers,
                method: Some("GET".to_string()),
            };

            let res = http::request::<()>(&req, None)?;

            if !is_success_status(res.status_code()) {
                return Ok(CallToolResult {
                    is_error: Some(true),
                    content: vec![Content {
                        annotations: None,
                        text: Some(format!(
                            "Failed to get repository members page {} from {}: {} - Response: {}",
                            current_page_number,
                            req.url,
                            res.status_code(),
                            String::from_utf8_lossy(&res.body())
                        )),
                        mime_type: None,
                        r#type: ContentType::Text,
                        data: None,
                    }],
                });
            }

            match serde_json::from_slice::<Vec<Value>>(&res.body()) {
                Ok(page_members) => {
                    let num_fetched = page_members.len();
                    all_members_json.extend(page_members);

                    if num_fetched < PER_PAGE_COUNT as usize {
                        break; // Last page fetched
                    }
                }
                Err(e) => {
                    return Ok(CallToolResult {
                        is_error: Some(true),
                        content: vec![Content {
                            annotations: None,
                            text: Some(format!(
                                "Failed to parse repository members data from GitLab API (page {}): {}",
                                current_page_number, e
                            )),
                            mime_type: None,
                            r#type: ContentType::Text,
                            data: None,
                        }],
                    });
                }
            }
            current_page_number += 1;
        }

        Ok(CallToolResult {
            is_error: None,
            content: vec![Content {
                annotations: None,
                text: Some(serde_json::to_string(&all_members_json)?),
                mime_type: Some("application/json".to_string()),
                r#type: ContentType::Text,
                data: None,
            }],
        })
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide project_id".into()),
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
                name: "gl_delete_file".into(),
                description: "Delete a file in a GitLab project repository. Requires project_id, file_path, branch, and optional commit_message.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file in the project",
                        },
                        "branch": {
                            "type": "string",
                            "description": "The name of the branch to delete the file from",
                        },
                        "commit_message": {
                            "type": "string",
                            "description": "The commit message. Optional, defaults to 'Delete file via API'",
                        },
                        "author_email": {
                            "type": "string",
                            "description": "The email of the commit author. Optional.",
                        },
                        "author_name": {
                            "type": "string",
                            "description": "The name of the commit author. Optional.",
                        },
                    },
                    "required": ["project_id", "file_path", "branch"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_create_issue".into(),
                description: "Create a new issue in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "title": {
                            "type": "string",
                            "description": "The title of the issue",
                        },
                        "description": {
                            "type": "string",
                            "description": "The description of the issue",
                        },
                        "labels": {
                            "type": "string",
                            "description": "Comma-separated list of labels",
                        },
                    },
                    "required": ["project_id", "title", "description"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_get_issue".into(),
                description: "Get details of a specific issue".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "issue_iid": {
                            "type": "string",
                            "description": "The internal ID of the issue",
                        },
                    },
                    "required": ["project_id", "issue_iid"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_update_issue".into(),
                description: "Update an existing issue in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "issue_iid": {
                            "type": "string",
                            "description": "The internal ID of the issue",
                        },
                        "title": {
                            "type": "string",
                            "description": "The new title of the issue",
                        },
                        "description": {
                            "type": "string",
                            "description": "The new description of the issue",
                        },
                        "add_labels": {
                            "type": "string",
                            "description": "Comma-separated list of labels to add to the issue",
                        },
                        "remove_labels": {
                            "type": "string",
                            "description": "Comma-separated list of labels to remove from the issue",
                        },
                        "due_date": {
                            "type": "string",
                            "description": "The due date of the issue in YYYY-MM-DD format (e.g., 2024-03-11)",
                        },
                    },
                    "required": ["project_id", "issue_iid"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_add_issue_comment".into(),
                description: "Add a comment to an issue in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "issue_iid": {
                            "type": "string",
                            "description": "The internal ID of the issue",
                        },
                        "comment": {
                            "type": "string",
                            "description": "The comment to add to the issue",
                        },
                    },
                    "required": ["project_id", "issue_iid", "comment"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_list_issues".into(),
                description: "List issues for a project in GitLab. Supports filtering by state and labels.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "state": {
                            "type": "string",
                            "description": "Filter by state: 'opened', 'closed', or 'all'. Defaults to 'opened' if not specified by GitLab.",
                        },
                        "labels": {
                            "type": "string",
                            "description": "Comma-separated list of label names to filter by.",
                        },
                    },
                    "required": ["project_id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_get_file_contents".into(),
                description: "Get the contents of a file in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file in the project",
                        },
                        "ref": {
                            "type": "string",
                            "description": "The name of the branch, tag or commit (defaults to HEAD)",
                        },
                    },
                    "required": ["project_id", "file_path"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_create_or_update_file".into(),
                description: "Create or update a file in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file in the project",
                        },
                        "content": {
                            "type": "string",
                            "description": "The content to write to the file",
                        },
                        "branch": {
                            "type": "string",
                            "description": "The name of the branch to create or update the file in",
                        },
                        "author_email": {
                            "type": "string",
                            "description": "The email of the commit author",
                        },
                        "author_name": {
                            "type": "string",
                            "description": "The name of the commit author",
                        },
                        "commit_message": {
                            "type": "string",
                            "description": "The commit message. Defaults to 'Update file via API' if not specified.",
                        },
                    },
                    "required": ["project_id", "file_path", "content", "branch"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_create_branch".into(),
                description: "Create a new branch in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "branch_name": {
                            "type": "string",
                            "description": "The name of the new branch",
                        },
                        "ref": {
                            "type": "string",
                            "description": "The branch name or commit SHA to create the new branch from",
                        },
                    },
                    "required": ["project_id", "branch_name", "ref"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_create_merge_request".into(),
                description: "Create a new merge request in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "source_branch": {
                            "type": "string",
                            "description": "The name of the source branch",
                        },
                        "target_branch": {
                            "type": "string",
                            "description": "The name of the target branch",
                        },
                    },
                    "required": ["project_id", "source_branch", "target_branch"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_update_merge_request".into(),
                description: "Update an existing merge request in a GitLab project.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "merge_request_iid": {
                            "type": "string",
                            "description": "The internal ID (IID) of the merge request to update",
                        },
                        "title": {
                            "type": "string",
                            "description": "The new title for the merge request.",
                        },
                        "description": {
                            "type": "string",
                            "description": "The new description for the merge request.",
                        },
                        // Consider adding other common updatable fields like:
                        // "target_branch": { "type": "string", "description": "The target branch" },
                        // "state_event": { "type": "string", "description": "Event to change MR state (e.g., 'close', 'reopen')" }
                    },
                    "required": ["project_id", "merge_request_iid", "title", "description"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_get_merge_request".into(),
                description: "Get details of a specific merge request in a GitLab project.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "merge_request_iid": {
                            "type": "string",
                            "description": "The internal ID (IID) of the merge request",
                        },
                    },
                    "required": ["project_id", "merge_request_iid"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_create_snippet".into(),
                description: "Create a new snippet".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "The title of the snippet",
                        },
                        "content": {
                            "type": "string",
                            "description": "The content of the snippet",
                        },
                        "visibility": {
                            "type": "string",
                            "description": "The visibility level of the snippet (private, internal, or public). Defaults to private if not specified.",
                        },
                    },
                    "required": ["title", "content"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_update_snippet".into(),
                description: "Update an existing snippet".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "snippet_id": {
                            "type": "string",
                            "description": "The ID of the snippet",
                        },
                        "title": {
                            "type": "string",
                            "description": "The new title of the snippet",
                        },
                        "content": {
                            "type": "string",
                            "description": "The new content of the snippet",
                        },
                    },
                    "required": ["snippet_id", "title", "content"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_get_snippet".into(),
                description: "Get details of a specific snippet".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "snippet_id": {
                            "type": "string",
                            "description": "The ID of the snippet",
                        },
                    },
                    "required": ["snippet_id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_delete_snippet".into(),
                description: "Delete a snippet".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "snippet_id": {
                            "type": "string",
                            "description": "The ID of the snippet",
                        },
                    },
                    "required": ["snippet_id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_list_branches".into(),
                description: "List all branches in a GitLab project".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                    },
                    "required": ["project_id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_get_repo_tree".into(),
                description: "Get the list of files and directories in a project repository. Handles pagination internally.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "path": {
                            "type": "string",
                            "description": "The path inside the repository. Used to get content of subdirectories. Optional.",
                        },
                        "ref": {
                            "type": "string",
                            "description": "The name of a repository branch or tag or if not given the default branch. Optional.",
                        },
                        "recursive": {
                            "type": "boolean",
                            "description": "Boolean value used to get a recursive tree. If you want a complete tree, set this to true. Default is false. Optional.",
                        },
                    },
                    "required": ["project_id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "gl_get_repo_members".into(),
                description: "Get a list of members for a GitLab project. Supports fetching direct or inherited members and filtering by query. Handles pagination internally.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "project_id": {
                            "type": "string",
                            "description": "The project identifier - can be a numeric project ID (e.g. '123') or a URL-encoded path (e.g. 'group%2Fproject')",
                        },
                        "include_inherited_members": {
                            "type": "boolean",
                            "description": "Set to true to include inherited members (e.g., from groups). Defaults to false (direct members only). Optional.",
                        },
                        "query": {
                            "type": "string",
                            "description": "Filter by username, name, or public email. Optional.",
                        },
                    },
                    "required": ["project_id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
