mod pdk;

use extism_pdk::*;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use rusqlite::Connection;
use serde_json::json;
use std::sync::Once;

static DB_INIT: Once = Once::new();

fn init_db(db_path: &str) -> Result<(), Error> {
    let _conn = Connection::open_with_flags(
        db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
    )?;

    Ok(())
}

fn get_db_path() -> Result<String, Error> {
    config::get("db_path")?
        .ok_or_else(|| Error::msg("db_path configuration is required but not set"))
}

fn execute_read_query(query: &str, db_path: &str) -> Result<String, Error> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    let mut stmt = conn.prepare(query)?;
    let column_names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();

    let rows = stmt.query_map([], |row| {
        let mut map = serde_json::Map::new();
        for (i, col_name) in column_names.iter().enumerate() {
            let value = match row.get_ref(i)? {
                rusqlite::types::ValueRef::Null => serde_json::Value::Null,
                rusqlite::types::ValueRef::Integer(i) => json!(i),
                rusqlite::types::ValueRef::Real(f) => json!(f),
                rusqlite::types::ValueRef::Text(s) => json!(s),
                rusqlite::types::ValueRef::Blob(b) => json!(b),
            };
            map.insert(col_name.clone(), value);
        }
        Ok(map)
    })?;

    let results: Vec<serde_json::Map<String, serde_json::Value>> =
        rows.filter_map(Result::ok).collect();
    Ok(serde_json::to_string(&results)?)
}

fn execute_write_query(query: &str, db_path: &str) -> Result<String, Error> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    let affected = conn.execute(query, [])?;
    Ok(json!({ "rows_affected": affected }).to_string())
}

fn create_table(query: &str, db_path: &str) -> Result<String, Error> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    conn.execute(query, [])?;
    Ok(json!({ "status": "success" }).to_string())
}

fn list_tables(db_path: &str) -> Result<String, Error> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let tables: Result<Vec<String>, _> = stmt.query_map([], |row| row.get(0))?.collect();
    Ok(json!({ "tables": tables? }).to_string())
}

fn describe_table(table_name: &str, db_path: &str) -> Result<String, Error> {
    let conn = Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;

    let columns = stmt.query_map([], |row| {
        Ok(json!({
            "cid": row.get::<_, i64>(0)?,
            "name": row.get::<_, String>(1)?,
            "type": row.get::<_, String>(2)?,
            "notnull": row.get::<_, bool>(3)?,
            "dflt_value": row.get::<_, Option<String>>(4)?,
            "pk": row.get::<_, bool>(5)?
        }))
    })?;

    let schema: Vec<serde_json::Value> = columns.filter_map(Result::ok).collect();
    Ok(json!({ "schema": schema }).to_string())
}

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let db_path = get_db_path()?;
    DB_INIT.call_once(|| {
        init_db(&db_path).expect("Failed to initialize database");
    });

    match input.params.name.as_str() {
        "sqlite_read_query" => {
            let args = input.params.arguments.unwrap_or_default();
            let query = match args.get("query") {
                Some(v) if v.is_string() => v.as_str().unwrap(),
                _ => return Err(Error::msg("query parameter is required")),
            };

            let result = execute_read_query(query, &db_path)?;
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(result),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        }
        "sqlite_write_query" => {
            let args = input.params.arguments.unwrap_or_default();
            let query = match args.get("query") {
                Some(v) if v.is_string() => v.as_str().unwrap(),
                _ => return Err(Error::msg("query parameter is required")),
            };

            let result = execute_write_query(query, &db_path)?;
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(result),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        }
        "sqlite_create_table" => {
            let args = input.params.arguments.unwrap_or_default();
            let query = match args.get("query") {
                Some(v) if v.is_string() => v.as_str().unwrap(),
                _ => return Err(Error::msg("query parameter is required")),
            };

            let result = create_table(query, &db_path)?;
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(result),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        }
        "sqlite_list_tables" => {
            let result = list_tables(&db_path)?;
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(result),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
        }
        "sqlite_describe_table" => {
            let args = input.params.arguments.unwrap_or_default();
            let table_name = match args.get("table_name") {
                Some(v) if v.is_string() => v.as_str().unwrap(),
                _ => return Err(Error::msg("table_name parameter is required")),
            };

            let result = describe_table(table_name, &db_path)?;
            Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(result),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            })
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
                name: "sqlite_read_query".into(),
                description: "Execute a SELECT query on the SQLite database".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "SELECT SQL query to execute",
                        }
                    },
                    "required": ["query"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "sqlite_write_query".into(),
                description: "Execute an INSERT, UPDATE, or DELETE query on the SQLite database"
                    .into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "SQL query to execute",
                        }
                    },
                    "required": ["query"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "sqlite_create_table".into(),
                description: "Create a new table in the SQLite database".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "CREATE TABLE SQL statement",
                        }
                    },
                    "required": ["query"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "sqlite_list_tables".into(),
                description: "List all tables in the SQLite database".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": [],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "sqlite_describe_table".into(),
                description: "Get the schema information for a specific table".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "table_name": {
                            "type": "string",
                            "description": "Name of the table to describe",
                        }
                    },
                    "required": ["table_name"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
