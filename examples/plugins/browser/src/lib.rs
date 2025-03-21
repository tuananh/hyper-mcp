mod pdk;

use extism_pdk::*;
use pdk::types::*;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use html2md::parse_html;
use serde_json::Map;

// Called when the tool is invoked.
pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    
    // Parse input URL from the request
    let url = args.get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("URL parameter is required"))?;

    // Get CDP endpoint from config
    let browser = if let Ok(Some(cdp_url)) = config::get("CHROME_CDP_URL") {
        info!("Connecting to CDP endpoint: {}", cdp_url);
        Browser::connect(cdp_url)
            .map_err(|e| Error::msg(format!("Failed to connect to CDP endpoint: {}", e)))?
    } else {
        info!("No CDP endpoint provided, launching a new browser instance");
        // Fallback to launching a new browser instance if CDP URL is not provided
        let options = LaunchOptionsBuilder::default()
            .headless(true)
            .sandbox(false)
            .build()
            .map_err(|e| Error::msg(format!("Failed to build browser options: {}", e)))?;

        Browser::new(options)
            .map_err(|e| Error::msg(format!("Failed to launch browser: {}", e)))?
    };

    // Create a new tab and navigate to URL
    let tab = browser.new_tab()
        .map_err(|e| Error::msg(format!("Failed to create new tab: {}", e)))?;

    tab.navigate_to(url)
        .map_err(|e| Error::msg(format!("Failed to navigate to URL: {}", e)))?;

    // Wait for page to load
    tab.wait_until_navigated()
        .map_err(|e| Error::msg(format!("Failed to wait for navigation: {}", e)))?;

    // Get page content
    let content = tab.get_content()
        .map_err(|e| Error::msg(format!("Failed to get page content: {}", e)))?;

    // Convert HTML to Markdown
    let markdown = parse_html(&content);

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(markdown),
            mime_type: Some("text/markdown".into()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    let mut url_prop: Map<String, serde_json::Value> = Map::new();
    url_prop.insert("type".into(), "string".into());
    url_prop.insert(
        "description".into(),
        "URL to fetch and convert to markdown".into(),
    );

    let mut props: Map<String, serde_json::Value> = Map::new();
    props.insert("url".into(), url_prop.into());

    let mut schema: Map<String, serde_json::Value> = Map::new();
    schema.insert("type".into(), "object".into());
    schema.insert("properties".into(), serde_json::Value::Object(props));
    schema.insert("required".into(), serde_json::Value::Array(vec!["url".into()]));

    Ok(ListToolsResult {
        tools: vec![ToolDescription {
            name: "fetchPage".into(),
            description: "Fetch a webpage and convert it to clean markdown format".into(),
            input_schema: schema,
        }],
    })
}
