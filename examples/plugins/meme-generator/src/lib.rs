mod embedded;
mod pdk;

use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use base64::Engine;
use extism_pdk::*;
use image::Rgba;
use imageproc::drawing::draw_text_mut;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    text: Vec<String>,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MemeTemplate {
    id: String,
    name: String,
    lines: u32,
    overlays: u32,
    styles: Vec<String>,
    blank: String,
    example: Example,
    source: Option<String>,
    keywords: Vec<String>,
    #[serde(rename = "_self")]
    self_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateConfig {
    name: String,
    source: String,
    keywords: Vec<String>,
    text: Vec<TextConfig>,
    example: Vec<String>,
    overlay: Vec<OverlayConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TextConfig {
    style: String,
    color: String,
    font: String,
    anchor_x: f32,
    anchor_y: f32,
    angle: f32,
    scale_x: f32,
    scale_y: f32,
    align: String,
    start: f32,
    stop: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OverlayConfig {
    center_x: f32,
    center_y: f32,
    angle: f32,
    scale: f32,
}

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    match input.params.name.as_str() {
        "meme_list_templates" => list_templates(input),
        "meme_get_template" => get_template(input),
        "meme_generate" => generate_meme(input),
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

fn list_templates(_input: CallToolRequest) -> Result<CallToolResult, Error> {
    let templates_json = embedded::TEMPLATES_JSON;
    let templates: Vec<MemeTemplate> = serde_json::from_str(templates_json)?;

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(serde_json::to_string_pretty(&templates)?),
            mime_type: Some("application/json".to_string()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

fn get_template(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    let template_id = args
        .get("template_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("template_id is required"))?;

    let templates: Vec<MemeTemplate> = serde_json::from_str(embedded::TEMPLATES_JSON)?;

    let template = templates
        .iter()
        .find(|t| t.id == template_id)
        .ok_or_else(|| Error::msg("Template not found"))?;

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(serde_json::to_string_pretty(&template)?),
            mime_type: Some("application/json".to_string()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

fn generate_meme(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();

    let template_id = args
        .get("template_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("template_id is required"))?;

    let texts = args
        .get("texts")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::msg("texts array is required"))?;

    // Load template configuration
    let config = TemplateConfig::load(template_id)?;

    // Get the default image from embedded resources
    let image_name = if embedded::get_template_image(template_id, "default.jpg").is_some() {
        "default.jpg"
    } else if embedded::get_template_image(template_id, "default.png").is_some() {
        "default.png"
    } else {
        return Err(Error::msg(format!(
            "No default template image found for {}",
            template_id
        )));
    };

    let image_data = embedded::get_template_image(template_id, image_name).ok_or_else(|| {
        Error::msg(format!(
            "Template image {} {} not found",
            template_id, image_name
        ))
    })?;

    let mut image = image::load_from_memory(image_data)?.to_rgba8();
    let (image_width, image_height) = image.dimensions();

    let font = FontArc::try_from_slice(embedded::FONT_DATA)?;

    // Draw each text configuration
    for (i, text_config) in config.text.iter().enumerate() {
        if i >= texts.len() {
            break;
        }

        let text = texts[i]
            .as_str()
            .ok_or_else(|| Error::msg("Invalid text entry"))?;

        let text = if text_config.style == "upper" {
            text.to_uppercase()
        } else {
            text.to_string()
        };

        // Calculate initial desired height based on image dimensions and config scale
        let desired_height = (image_height as f32 * text_config.scale_y).max(1.0);

        // Calculate maximum available width based on alignment
        let padding = image_width as f32 * 0.05; // 5% padding on each side
        let available_width = match text_config.align.as_str() {
            "center" => image_width as f32 - (2.0 * padding),
            "left" => {
                image_width as f32 - (image_width as f32 * text_config.anchor_x) - (2.0 * padding)
            }
            "right" => (image_width as f32 * text_config.anchor_x) - (2.0 * padding),
            _ => image_width as f32 - (2.0 * padding),
        };

        // Calculate appropriate scale that prevents overflow
        let scale = calculate_max_scale(&font, &text, available_width, desired_height);

        // Calculate text width for positioning using the adjusted scale
        let text_width = calculate_text_width(&font, &text, scale);

        // Calculate x position based on anchor and alignment, now with padding
        let x = match text_config.align.as_str() {
            "center" => ((image_width as f32 - text_width) / 2.0
                + (image_width as f32 * text_config.anchor_x))
                .max(padding) as i32,
            "left" => ((image_width as f32 * text_config.anchor_x) + padding) as i32,
            "right" => ((image_width as f32 * text_config.anchor_x) - text_width - padding)
                .max(padding) as i32,
            _ => ((image_width as f32 - text_width) / 2.0).max(padding) as i32,
        };

        // Calculate y position based on anchor
        let y = (image_height as f32 * text_config.anchor_y) as i32;

        // Convert color string to RGBA
        let color = color_to_rgba(&text_config.color);

        draw_text_mut(&mut image, color, x, y, scale, &font, &text);
    }

    // Convert image to bytes
    let mut output_bytes = Vec::new();
    let dynamic_image = image::DynamicImage::ImageRgba8(image);
    dynamic_image.write_to(&mut Cursor::new(&mut output_bytes), image::ImageFormat::Png)?;

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: None,
            mime_type: Some("image/png".to_string()),
            r#type: ContentType::Image,
            data: Some(base64::engine::general_purpose::STANDARD.encode(&output_bytes)),
        }],
    })
}

fn calculate_text_width(font: &FontArc, text: &str, scale: PxScale) -> f32 {
    let scaled_font = font.as_scaled(scale);
    let mut width = 0.0;

    for c in text.chars() {
        let id = scaled_font.glyph_id(c);
        width += scaled_font.h_advance(id);

        if let Some(next_char) = text.chars().nth(1) {
            let next_id = scaled_font.glyph_id(next_char);
            width += scaled_font.kern(id, next_id);
        }
    }

    width
}

fn color_to_rgba(color: &str) -> Rgba<u8> {
    match color.to_lowercase().as_str() {
        "white" => Rgba([255, 255, 255, 255]),
        "black" => Rgba([0, 0, 0, 255]),
        "red" => Rgba([255, 0, 0, 255]),
        "green" => Rgba([0, 255, 0, 255]),
        "blue" => Rgba([0, 0, 255, 255]),
        _ => Rgba([255, 255, 255, 255]), // Fallback to white
    }
}

fn calculate_max_scale(
    font: &FontArc,
    text: &str,
    target_width: f32,
    desired_height: f32,
) -> PxScale {
    let initial_scale = PxScale::from(desired_height);
    let initial_width = calculate_text_width(font, text, initial_scale);

    if initial_width <= target_width {
        return initial_scale;
    }

    // Scale down proportionally if text is too wide
    let scale_factor = target_width / initial_width;
    PxScale::from(desired_height * scale_factor)
}

impl TemplateConfig {
    fn load(template_id: &str) -> Result<Self, Error> {
        let config_contents = embedded::get_template_config(template_id)
            .ok_or_else(|| Error::msg(format!("Template {} not found", template_id)))?;

        let config: TemplateConfig = serde_yaml::from_str(config_contents)
            .map_err(|e| Error::msg(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult {
        tools: vec![
            ToolDescription {
                name: "meme_list_templates".into(),
                description: "Lists all available meme templates".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "meme_get_template".into(),
                description: "Get details about a specific meme template".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "template_id": {
                            "type": "string",
                            "description": "The ID of the template to retrieve",
                        }
                    },
                    "required": ["template_id"]
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "meme_generate".into(),
                description: "Generate a meme using a template and custom text".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "template_id": {
                            "type": "string",
                            "description": "The ID of the template to use",
                        },
                        "texts": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Array of text strings to place on the meme",
                        }
                    },
                    "required": ["template_id", "texts"]
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
