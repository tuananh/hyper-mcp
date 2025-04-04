use anyhow::{Context, Result, anyhow};
use chrono::Local;
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use log::LevelFilter;
use std::{io::Write, path::Path, str::FromStr};

lazy_static! {
    static ref CONFIG_SCHEMA: JSONSchema = {
        let schema = include_str!("schema/config.json");
        let schema_value: serde_json::Value =
            serde_json::from_str(schema).expect("Failed to parse config schema");
        JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema_value)
            .expect("Failed to compile JSON schema")
    };
}

pub(crate) fn init_logger(path: Option<&str>, level: Option<&str>) -> Result<()> {
    let log_level = LevelFilter::from_str(level.unwrap_or("info"))?;

    // If the log path is not provided, use the stderr
    let log_file = match path {
        Some(p) => Box::new(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(p)?,
        ) as Box<dyn Write + Send + Sync + 'static>,
        _ => Box::new(std::io::stderr()) as Box<dyn Write + Send + Sync + 'static>,
    };

    // TODO: apply module filter
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}/{}:{} {} [{}] - {}",
                record.module_path().unwrap_or("unknown"),
                basename(record.file().unwrap_or("unknown")),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(log_file))
        .filter(None, log_level)
        .try_init()?;

    Ok(())
}

pub fn basename(path: &str) -> String {
    path.split('/').last().unwrap_or(path).to_string()
}

#[derive(Debug)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

pub(crate) fn detect_format_from_content(content: &str) -> Result<ConfigFormat> {
    // Try to determine if it's JSON by checking for { or [ at start (after whitespace)
    let trimmed = content.trim_start();
    if (trimmed.starts_with('{') || trimmed.starts_with('['))
        && serde_json::from_str::<serde_json::Value>(content).is_ok()
    {
        return Ok(ConfigFormat::Json);
    }

    // Try YAML - Check for common YAML indicators
    if (trimmed.contains(": ") || trimmed.starts_with("---"))
        && serde_yaml::from_str::<serde_yaml::Value>(content).is_ok()
    {
        return Ok(ConfigFormat::Yaml);
    }

    // Try TOML - Look for key-value pairs with = or section headers
    if (trimmed.contains('=') || trimmed.contains('['))
        && toml::from_str::<toml::Value>(content).is_ok()
    {
        return Ok(ConfigFormat::Toml);
    }

    Err(anyhow!(
        "Unable to detect config format. Content doesn't appear to be valid JSON, YAML, or TOML"
    ))
}

pub(crate) fn validate_config(content: &str) -> Result<()> {
    // First try to parse as a generic Value to check basic format
    let format = detect_format_from_content(content)?;
    let value: serde_json::Value = match format {
        ConfigFormat::Json => serde_json::from_str(content).context("Failed to parse as JSON")?,
        ConfigFormat::Yaml => {
            let yaml_value: serde_yaml::Value =
                serde_yaml::from_str(content).context("Failed to parse as YAML")?;
            serde_json::to_value(yaml_value).context("Failed to convert YAML to JSON value")?
        }
        ConfigFormat::Toml => {
            let toml_value: toml::Value =
                toml::from_str(content).context("Failed to parse as TOML")?;
            serde_json::to_value(toml_value).context("Failed to convert TOML to JSON value")?
        }
    };

    // Validate against schema
    let validation = CONFIG_SCHEMA.validate(&value);
    if let Err(errors) = validation {
        let error_messages: Vec<String> = errors.map(|error| format!("- {}", error)).collect();
        return Err(anyhow!(
            "Config validation failed:\n{}",
            error_messages.join("\n")
        ));
    }

    // Additional validation for file paths
    if let Some(plugins) = value
        .as_object()
        .and_then(|obj| obj.get("plugins"))
        .and_then(|v| v.as_array())
    {
        for plugin in plugins {
            if let Some(path) = plugin.get("path").and_then(|v| v.as_str()) {
                // Only validate local file paths (not http or oci)
                if !path.starts_with("http")
                    && !path.starts_with("oci://")
                    && !Path::new(path).exists()
                {
                    return Err(anyhow!("Local plugin path '{}' does not exist", path));
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn parse_config_from_str<T: serde::de::DeserializeOwned>(content: &str) -> Result<T> {
    // First validate the config structure
    validate_config(content)?;

    let format = detect_format_from_content(content)?;
    match format {
        ConfigFormat::Json => serde_json::from_str(content).context("Failed to parse JSON config"),
        ConfigFormat::Yaml => serde_yaml::from_str(content).context("Failed to parse YAML config"),
        ConfigFormat::Toml => toml::from_str(content).context("Failed to parse TOML config"),
    }
}

pub(crate) fn parse_config<T: serde::de::DeserializeOwned>(
    content: &str,
    file_path: &Path,
) -> Result<T> {
    if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
        // If we have a file extension, try that format first
        match extension.to_lowercase().as_str() {
            "json" => return serde_json::from_str(content).context("Failed to parse JSON config"),
            "yaml" | "yml" => {
                return serde_yaml::from_str(content).context("Failed to parse YAML config");
            }
            "toml" => return toml::from_str(content).context("Failed to parse TOML config"),
            _ => {} // Fall through to content-based detection
        }
    }

    // If no extension or unknown extension, try to detect from content
    parse_config_from_str(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
    }

    #[test]
    fn test_parse_config() {
        let test_json = r#"{"name": "test", "value": 42}"#;
        let test_yaml = "name: test\nvalue: 42";
        let test_toml = r#"name = "test"\nvalue = 42"#;

        let json_path = PathBuf::from("test.json");
        let yaml_path = PathBuf::from("test.yaml");
        let toml_path = PathBuf::from("test.toml");

        let expected = TestConfig {
            name: "test".to_string(),
            value: 42,
        };

        assert_eq!(
            parse_config::<TestConfig>(test_json, &json_path).unwrap(),
            expected
        );
        assert_eq!(
            parse_config::<TestConfig>(test_yaml, &yaml_path).unwrap(),
            expected
        );
        assert_eq!(
            parse_config::<TestConfig>(test_toml, &toml_path).unwrap(),
            expected
        );

        // Test invalid format
        let invalid_path = PathBuf::from("test.invalid");
        assert!(parse_config::<TestConfig>(test_json, &invalid_path).is_err());
    }

    #[test]
    fn test_basename() {
        assert_eq!(basename("/path/to/file.txt"), "file.txt");
        assert_eq!(basename("file.txt"), "file.txt");
        assert_eq!(basename("file"), "file");
        assert_eq!(basename("/path/to/"), "");
        assert_eq!(basename(""), "");
    }

    #[test]
    fn test_init_logger() {
        // Test with a valid path
        let result = init_logger(Some("test.log"), Some("debug"));
        assert!(result.is_ok());

        // Test with an invalid path
        let result = init_logger(Some("/invalid/path/to/log.log"), Some("debug"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_config() {
        // Valid JSON config
        let valid_json = r#"{
            "plugins": [
                {
                    "name": "test",
                    "path": "oci://test/plugin"
                }
            ]
        }"#;
        assert!(validate_config(valid_json).is_ok());

        // Missing plugins field
        let invalid_json = r#"{
            "something": []
        }"#;
        assert!(validate_config(invalid_json).is_err());

        // Invalid plugin object (missing required fields)
        let invalid_plugin = r#"{
            "plugins": [
                {
                    "name": "test"
                }
            ]
        }"#;
        assert!(validate_config(invalid_plugin).is_err());

        // Invalid runtime_config
        let invalid_runtime = r#"{
            "plugins": [
                {
                    "name": "test",
                    "path": "oci://test/plugin",
                    "runtime_config": {
                        "allowed_host": 123
                    }
                }
            ]
        }"#;
        assert!(validate_config(invalid_runtime).is_err());

        // Valid config with runtime_config
        let valid_runtime = r#"{
            "plugins": [
                {
                    "name": "test",
                    "path": "oci://test/plugin",
                    "runtime_config": {
                        "allowed_host": "example.com",
                        "allowed_paths": ["/tmp", "/var/log"]
                    }
                }
            ]
        }"#;
        assert!(validate_config(valid_runtime).is_ok());
    }
}
