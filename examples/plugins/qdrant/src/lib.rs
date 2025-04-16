mod pdk;
mod qdrant_client;

use extism_pdk::*;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use qdrant_client::*;
use serde_json::json;

fn get_qdrant_client() -> Result<QdrantClient, Error> {
    let qdrant_url = config::get("QDRANT_URL")?
        .ok_or_else(|| Error::msg("QDRANT_URL configuration is required but not set"))?;

    let mut client = QdrantClient::new_with_url(qdrant_url);

    // Check if API key is set in config
    if let Ok(Some(api_key)) = config::get("QDRANT_API_KEY") {
        client.set_api_key(api_key);
    }

    Ok(client)
}

fn ensure_collection_exists(
    client: &QdrantClient,
    collection_name: &str,
    vector_size: u32,
) -> Result<(), Error> {
    // check if the collection exists. If present, delete it.
    if let Ok(true) = client.collection_exists(collection_name) {
        println!("Collection `{}` exists", collection_name);
        match client.delete_collection(collection_name) {
            Ok(_) => println!("Collection `{}` deleted", collection_name),
            Err(e) => println!("Error deleting collection: {:?}", e),
        }
    };

    // Create collection
    let create_result = client.create_collection(collection_name, vector_size);
    println!("Create collection result is {:?}", create_result);

    Ok(())
}

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    match input.params.name.as_str() {
        "qdrant_store" => qdrant_store(input),
        "qdrant_find" => qdrant_find(input),
        "qdrant_create_collection" => qdrant_create_collection(input),
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

fn qdrant_store(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();

    let collection_name = args
        .get("collection_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("collection_name parameter is required"))?;

    let vector = args
        .get("vector")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::msg("vector parameter is required"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or_default())
        .collect::<Vec<f64>>();

    let text = args
        .get("text")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("text parameter is required"))?;

    let client = get_qdrant_client()?;
    ensure_collection_exists(&client, collection_name, vector.len() as u32)?;

    let point_id = uuid::Uuid::new_v4().to_string();
    let vector: Vec<f32> = vector.into_iter().map(|x| x as f32).collect();

    let mut points = Vec::new();
    points.push(Point {
        id: PointId::Uuid(point_id.clone()),
        vector,
        payload: json!({
            "text": text,
            "metadata": {},
        })
        .as_object()
        .map(|m| m.to_owned()),
    });

    client.upsert_points(collection_name, points)?;
    println!("Upsert points result is {:?}", ());

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(format!(
                "Successfully stored document with ID: {}",
                point_id
            )),
            mime_type: None,
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

fn qdrant_find(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();

    let collection_name = args
        .get("collection_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("collection_name parameter is required"))?;

    let vector = args
        .get("vector")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::msg("vector parameter is required"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or_default())
        .collect::<Vec<f64>>();

    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(5);

    let client = get_qdrant_client()?;

    let vector_f32: Vec<f32> = vector.into_iter().map(|x| x as f32).collect();
    let search_result = client.search_points(collection_name, vector_f32, limit, None)?;

    let mut results = Vec::new();
    for point in search_result {
        if let Some(payload) = &point.payload {
            if let Some(text) = payload.get("text").and_then(|v| v.as_str()) {
                results.push(format!("Score: {:.4} - {}", point.score, text));
            }
        }
    }

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(results.join("\n")),
            mime_type: None,
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

fn qdrant_create_collection(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();

    let collection_name = args
        .get("collection_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("collection_name parameter is required"))?;

    let vector_size = args
        .get("vector_size")
        .and_then(|v| v.as_u64())
        .unwrap_or(384) as u32;

    let client = get_qdrant_client()?;
    ensure_collection_exists(&client, collection_name, vector_size)?;

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(format!(
                "Successfully created collection '{}' with vector size {}",
                collection_name, vector_size
            )),
            mime_type: None,
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult {
        tools: vec![
            ToolDescription {
                name: "qdrant_create_collection".into(),
                description: "Creates a new collection in Qdrant with specified vector size".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "collection_name": {
                            "type": "string",
                            "description": "The name of the collection to create",
                        },
                        "vector_size": {
                            "type": "integer",
                            "description": "The size of vectors to be stored in this collection",
                            "default": 384
                        }
                    },
                    "required": ["collection_name"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "qdrant_store".into(),
                description: "Stores a document with its vector embedding in Qdrant.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "collection_name": {
                            "type": "string",
                            "description": "The name of the collection to store the document in",
                        },
                        "text": {
                            "type": "string",
                            "description": "The text content to store",
                        },
                        "vector": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "The vector embedding of the text.",
                        }
                    },
                    "required": ["collection_name", "text", "vector"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "qdrant_find".into(),
                description: "Finds similar documents in Qdrant using vector similarity search"
                    .into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "collection_name": {
                            "type": "string",
                            "description": "The name of the collection to search in",
                        },
                        "vector": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "The query vector to search with.",
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of results to return",
                            "default": 5
                        }
                    },
                    "required": ["collection_name", "vector"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
