mod pdk;

use extism_pdk::*;
use pdk::types::*;
use serde_json::Map;

// Called when the tool is invoked.
pub(crate) fn call(_input: CallToolRequest) -> Result<CallToolResult, Error> {
    let request = HttpRequest::new("https://1.1.1.1/cdn-cgi/trace");
    let response = http::request::<Vec<u8>>(&request, None)
        .map_err(|e| Error::msg(format!("Failed to make HTTP request: {}", e)))?;

    let text = String::from_utf8(response.body().to_vec())
        .map_err(|e| Error::msg(format!("Failed to parse response as UTF-8: {}", e)))?;

    // Parse the response to extract IP address
    let ip = text
        .lines()
        .find(|line| line.starts_with("ip="))
        .map(|line| line.trim_start_matches("ip="))
        .ok_or_else(|| Error::msg("Could not find IP address in response"))?;

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(ip.to_string()),
            mime_type: Some("text/plain".into()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    let mut foo_prop: Map<String, serde_json::Value> = Map::new();
    foo_prop.insert("type".into(), "string".into());
    foo_prop.insert(
        "description".into(),
        "foo data parameter".into(),
    );

    let mut props: Map<String, serde_json::Value> = Map::new();
    props.insert("foo".into(), foo_prop.into());

    let mut schema: Map<String, serde_json::Value> = Map::new();
    schema.insert("type".into(), "object".into());
    schema.insert("properties".into(), serde_json::Value::Object(props));
    schema.insert("required".into(), serde_json::Value::Array(vec!["foo".into()]));

    Ok(ListToolsResult {
        tools: vec![ToolDescription {
            name: "myip".into(),
            description: "Get the current IP address using Cloudflare's service".into(),
            input_schema: schema,
        }],
    })
}
