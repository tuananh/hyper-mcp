use anyhow::{Context, Result};
use chrono::Local;
use log::LevelFilter;
use std::{io::Write, path::Path, str::FromStr};

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

pub(crate) fn parse_config<T: serde::de::DeserializeOwned>(
    content: &str,
    file_path: &Path,
) -> Result<T> {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("json");

    match extension.to_lowercase().as_str() {
        "json" => serde_json::from_str(content).context("Failed to parse JSON config"),
        "yaml" | "yml" => serde_yaml::from_str(content).context("Failed to parse YAML config"),
        "toml" => toml::from_str(content).context("Failed to parse TOML config"),
        _ => Err(anyhow::anyhow!(
            "Unsupported config file format: {}",
            extension
        )),
    }
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
}
