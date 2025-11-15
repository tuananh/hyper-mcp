mod pdk;

use base64::Engine;
use extism_pdk::*;
use pdk::types::*;
use qrcode_png::{Color, QrCode, QrCodeEcc};
use serde_json::{Map, Value, json};

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    extism_pdk::log!(
        LogLevel::Info,
        "called with args: {:?}",
        input.params.arguments
    );
    let args = input.params.arguments.unwrap_or_default();
    let ecc = to_ecc(
        args.get("ecc")
            .cloned()
            .unwrap_or_else(|| json!(4))
            .as_number()
            .unwrap()
            .is_u64() as u8,
    );

    let data = match args.get("data") {
        Some(v) => v.as_str().unwrap(),
        None => return Err(Error::msg("`data` must be available")),
    };

    let mut code = QrCode::new(data, ecc)?;
    code.margin(10);
    code.zoom(10);

    let b = code.generate(Color::Grayscale(0, 255))?;
    let data = base64::engine::general_purpose::STANDARD.encode(b);

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: None,
            mime_type: Some("image/png".into()),
            r#type: ContentType::Image,
            data: Some(data),
        }],
    })
}

fn to_ecc(num: u8) -> QrCodeEcc {
    if num < 4 {
        return unsafe { std::mem::transmute::<u8, QrCodeEcc>(num) };
    }

    QrCodeEcc::High
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult {
        tools: vec![ToolDescription {
            name: "qr-code".into(),
            description: "Convert data like a message or URL to a QR code (resulting in a PNG file)".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "data": {
                        "type": "string",
                        "description": "data to convert to a QR code PNG"
                    },
                    "ecc": {
                        "type": "number",
                        "description": "Error correction level (range from 1 [low] to 4 [high], default to 4 unless user specifies)"
                    }
                },
                "required": ["data"]
            }).as_object().unwrap().clone(),
        }],
    })
}
