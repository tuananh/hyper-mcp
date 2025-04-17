use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub plugins: Vec<PluginConfig>,
    #[serde(default)]
    pub insecure_skip_signature: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub path: String,
    pub runtime_config: Option<RuntimeConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeConfig {
    pub allowed_hosts: Option<Vec<String>>,
    pub allowed_paths: Option<Vec<String>>,
    pub env_vars: Option<HashMap<String, String>>,
}

pub async fn load_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Config file not found at: {}. Please create a config file first.",
            path.display()
        ));
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read config file at {}", path.display()))?;

    let config = match ext {
        "json" => serde_json::from_str(&content)?,
        "yaml" | "yml" => serde_yaml::from_str(&content)?,
        "toml" => toml::from_str(&content)?,
        _ => return Err(anyhow::anyhow!("Unsupported config format: {}", ext)),
    };

    Ok(config)
}
