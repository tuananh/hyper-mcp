mod pdk;

use base64::Engine;
use extism_pdk::*;
use pdk::types::*;
use serde_json::{Map, Value};
use sha2::{Sha256, Sha512, Sha384, Sha224, Digest};
use sha1::Sha1;
use md5;
use base32;

// Called when the tool is invoked.
pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    extism_pdk::log!(
        LogLevel::Info,
        "called with args: {:?}",
        input.params.arguments
    );
    let args = input.params.arguments.unwrap_or_default();

    let data = match args.get("data") {
        Some(v) => v.as_str().unwrap(),
        None => return Err(Error::msg("`data` must be available")),
    };

    let algorithm = args.get("algorithm")
        .and_then(|v| v.as_str())
        .unwrap_or("base64");

    let result = match algorithm {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        },
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        },
        "sha384" => {
            let mut hasher = Sha384::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        },
        "sha224" => {
            let mut hasher = Sha224::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        },
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        },
        "md5" => {
            format!("{:x}", md5::compute(data))
        },
        "base32" => {
            base32::encode(base32::Alphabet::RFC4648 { padding: true }, data.as_bytes())
        },
        "base64" | _ => {
            base64::engine::general_purpose::STANDARD.encode(data)
        }
    };

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(result),
            mime_type: Some("text/plain".into()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    /*
    { tools: [{
        name: "base64",
        description: "base64 encode data",
        inputSchema: {
          type: "object",
          properties: {
            data: {
              type: "string",
              description: "data to convert to base64",
            },
            algorithm: {
              type: "string",
              description: "algorithm to use for hashing",
              enum: ["sha256", "sha512", "md5", "base64"],
            },
          },
          required: ["data", "algorithm"],
        }]
    }
    */
    let mut data_prop: Map<String, Value> = Map::new();
    data_prop.insert("type".into(), "string".into());
    data_prop.insert(
        "description".into(),
        "data to convert to hash or encoded format".into(),
    );

    let mut algorithm_prop: Map<String, Value> = Map::new();
    algorithm_prop.insert("type".into(), "string".into());
    algorithm_prop.insert(
        "description".into(),
        "algorithm to use for hashing or encoding".into(),
    );
    algorithm_prop.insert(
        "enum".into(),
        Value::Array(vec![
            "sha256".into(),
            "sha512".into(),
            "sha384".into(),
            "sha224".into(),
            "sha1".into(),
            "md5".into(),
            "base32".into(),
            "base64".into(),
        ])
    );

    let mut props: Map<String, Value> = Map::new();
    props.insert("data".into(), data_prop.into());
    props.insert("algorithm".into(), algorithm_prop.into());

    let mut schema: Map<String, Value> = Map::new();
    schema.insert("type".into(), "object".into());
    schema.insert("properties".into(), Value::Object(props));
    schema.insert("required".into(), Value::Array(vec!["data".into()]));

    Ok(ListToolsResult {
        tools: vec![ToolDescription {
            name: "hash".into(),
            description: "Hash data using various algorithms or encode using base32/base64".into(),
            input_schema: schema,
        }],
    })
}
