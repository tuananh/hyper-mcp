mod pdk;

use extism_pdk::*;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Once;
use uuid::Uuid;

static DB_INIT: Once = Once::new();

#[derive(Debug, Serialize, Deserialize)]
struct Memory {
    id: String,
    content: String,
}

fn init_db(db_path: &str) -> Result<(), Error> {
    let conn = Connection::open_with_flags(
        db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            created_at INTEGER DEFAULT (strftime('%s', 'now'))
        )",
        [],
    )?;

    Ok(())
}

fn get_db_path() -> Result<String, Error> {
    config::get("db_path")?
        .ok_or_else(|| Error::msg("db_path configuration is required but not set"))
}

fn store_memory(content: &str, db_path: &str) -> Result<String, Error> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO memories (id, content) VALUES (?, ?)",
        params![id, content],
    )?;

    Ok(id)
}

fn get_memory(id: &str, db_path: &str) -> Result<Option<Memory>, Error> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)?;

    let mut stmt = conn.prepare("SELECT id, content FROM memories WHERE id = ?")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Memory {
            id: row.get(0)?,
            content: row.get(1)?,
        }))
    } else {
        Ok(None)
    }
}

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let db_path = get_db_path()?;
    DB_INIT.call_once(|| {
        init_db(&db_path).expect("Failed to initialize database");
    });

    match input.params.name.as_str() {
        "store_memory" => {
            let args = input.params.arguments.unwrap_or_default();
            let content = match args.get("content") {
                Some(v) if v.is_string() => v.as_str().unwrap(),
                _ => return Err(Error::msg("content parameter is required")),
            };

            let id = store_memory(content, &db_path)?;

            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(json!({ "id": id }).to_string()),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        }
        "get_memory" => {
            let args = input.params.arguments.unwrap_or_default();
            let id = match args.get("id") {
                Some(v) if v.is_string() => v.as_str().unwrap(),
                _ => return Err(Error::msg("id parameter is required")),
            };

            match get_memory(id, &db_path)? {
                Some(memory) => Ok(CallToolResult {
                    is_error: None,
                    content: vec![Content {
                        annotations: None,
                        text: Some(serde_json::to_string(&memory)?),
                        mime_type: Some("application/json".to_string()),
                        r#type: ContentType::Text,
                        data: None,
                    }],
                }),
                None => Ok(CallToolResult {
                    is_error: Some(true),
                    content: vec![Content {
                        annotations: None,
                        text: Some("Memory not found".to_string()),
                        mime_type: None,
                        r#type: ContentType::Text,
                        data: None,
                    }],
                }),
            }
        }
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

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult {
        tools: vec![
            ToolDescription {
                name: "store_memory".into(),
                description: "Store content in memory and return a unique ID".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "content": {
                            "type": "string",
                            "description": "The content to store",
                        }
                    },
                    "required": ["content"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "get_memory".into(),
                description: "Retrieve content from memory by ID".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "string",
                            "description": "The ID of the content to retrieve",
                        }
                    },
                    "required": ["id"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
