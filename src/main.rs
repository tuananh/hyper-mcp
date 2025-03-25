use extism::*;
use extism::{Manifest, Wasm};
use rpc_router::{
    Error, Handler, HandlerResult, Request, Router as RpcRouter, RouterBuilder, RpcResource,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{error, info};

mod r#mod;
mod oci;
mod prompts;
mod resources;
mod tools;
mod types;

use r#mod::*;
use oci::*;
use prompts::{prompts_get, prompts_list};
use resources::{resource_read, resources_list};
use tools::{tools_call, tools_list};
use types::*;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author = "Tuan Anh Tran <me@tuananh.org>", version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,

    #[arg(short, long, value_name = "LOG_FILE")]
    log_file: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    plugins: Vec<PluginConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RuntimeConfig {
    allowed_host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginConfig {
    name: String,
    path: String,
    runtime_config: Option<RuntimeConfig>,
}

#[derive(Clone, RpcResource)]
pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<String, Plugin>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Get default config path in the user's config directory
    let default_config_path = dirs::config_dir()
        .map(|mut path| {
            path.push("mcp.json");
            path
        })
        .unwrap();

    let config_path = cli.config_file.unwrap_or(default_config_path);
    info!("using config_file at {}", config_path.display());
    let config: Config = {
        let config_content = tokio::fs::read_to_string(&config_path).await.map_err(|e| {
            error!("Failed to read config file at {:?}: {}", config_path, e);
            e
        })?;
        serde_json::from_str(&config_content)?
    };

    let plugins = Arc::new(RwLock::new(HashMap::new()));

    for plugin_cfg in &config.plugins {
        let wasm_content = if plugin_cfg.path.starts_with("http") {
            reqwest::get(&plugin_cfg.path)
                .await?
                .bytes()
                .await?
                .to_vec()
        } else if plugin_cfg.path.starts_with("oci") {
            // ref should be like oci://tuananh/qr-code
            let image_reference = plugin_cfg.path.strip_prefix("oci://").unwrap();
            let target_file_path = "/plugin.wasm";
            let mut hasher = Sha256::new();
            hasher.update(image_reference);
            let hash = hasher.finalize();
            let short_hash = &hex::encode(hash)[..7];
            let cache_dir = dirs::cache_dir()
                .map(|mut path| {
                    path.push("mcp");
                    path
                })
                .unwrap();
            std::fs::create_dir_all(&cache_dir)?;

            let local_output_path =
                cache_dir.join(format!("{}-{}.wasm", plugin_cfg.name, short_hash));
            let local_output_path = local_output_path.to_str().unwrap();

            if let Err(e) =
                pull_and_extract_oci_image(image_reference, target_file_path, local_output_path)
                    .await
            {
                eprintln!("Error pulling oci plugin: {}", e);
            }
            info!(
                "cache plugin `{}` to : {}",
                plugin_cfg.name, local_output_path
            );
            tokio::fs::read(local_output_path).await?
        } else {
            tokio::fs::read(&plugin_cfg.path).await?
        };

        let mut manifest = Manifest::new([Wasm::data(wasm_content)]);
        if let Some(runtime_cfg) = &plugin_cfg.runtime_config {
            info!("runtime_cfg: {:?}", runtime_cfg);
            if let Some(host) = &runtime_cfg.allowed_host {
                manifest = manifest.with_allowed_host(host);
            }
        }
        let plugin = Plugin::new(&manifest, [], true).unwrap();

        plugins
            .write()
            .await
            .insert(plugin_cfg.name.clone(), plugin);

        info!("Loaded plugin {}", plugin_cfg.name);
    }

    // setup router
    let rpc_router = build_rpc_router(plugins.clone());
    let input = io::stdin();
    let mut line = String::new();

    let log_dir = dirs::data_local_dir()
        .map(|mut path| {
            path.push("mcp");
            path.push("logs");
            path
        })
        .unwrap();

    std::fs::create_dir_all(&log_dir)?;

    let default_log_file_path = log_dir.join("mcp.jsonl");
    let log_file_path = cli.log_file.unwrap_or(default_log_file_path);
    info!("using log_file at {}", log_file_path.display());
    let mut logging_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .unwrap();

    while input.read_line(&mut line).unwrap() != 0 {
        let line = std::mem::take(&mut line);
        writeln!(logging_file, "receive: {}", line).unwrap();
        if !line.is_empty() {
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&line) {
                // notifications, no response required
                if json_value.is_object() && json_value.get("id").is_none() {
                    if let Some(method) = json_value.get("method") {
                        if method == "notifications/initialized" {
                            notifications_initialized();
                        } else if method == "notifications/cancelled" {
                            let params_value = json_value.get("params").unwrap();
                            let cancel_params: CancelledNotification =
                                serde_json::from_value(params_value.clone()).unwrap();
                            notifications_cancelled(cancel_params);
                        }
                    }
                } else if let Ok(mut rpc_request) = Request::from_value(json_value) {
                    // NOTE: because params is not required in ping but we need it in json-rpc
                    // https://github.com/modelcontextprotocol/specification/blob/ce55bba19fc1f5a343e45ef1b47f9ccf1801d318/docs/specification/2024-11-05/basic/utilities/ping.md#message-format
                    if rpc_request.method == "ping" {
                        rpc_request.params =
                            Some(serde_json::Value::Object(serde_json::Map::new()));
                    }

                    let id = rpc_request.id.clone();
                    match rpc_router.call(rpc_request).await {
                        Ok(call_response) => {
                            if !call_response.value.is_null() {
                                let response =
                                    JsonRpcResponse::new(id, call_response.value.clone());
                                let response_json = serde_json::to_string(&response).unwrap();
                                writeln!(logging_file, "ok: {}\n", response_json).unwrap();
                                println!("{}", response_json);
                            }
                        }
                        Err(error) => match &error.error {
                            Error::Handler(handler) => {
                                if let Some(error_value) = handler.get::<serde_json::Value>() {
                                    let json_error = json!({
                                        "jsonrpc": "2.0",
                                        "error": error_value,
                                        "id": id
                                    });
                                    let response = serde_json::to_string(&json_error).unwrap();
                                    writeln!(logging_file, "error: {}\n", response).unwrap();
                                    println!("{}", response);
                                }
                            }
                            _ => {
                                error!("Unexpected error {:?}", error);
                                let json_error = JsonRpcError::new(id, -1, "Invalid json-rpc call");
                                let response = serde_json::to_string(&json_error).unwrap();
                                writeln!(logging_file, "error: {}\n", error).unwrap();
                                println!("{}", response);
                            }
                        },
                    }
                }
            }
        }
    }
    Ok(())
}

fn build_rpc_router(plugins: Arc<RwLock<HashMap<String, Plugin>>>) -> RpcRouter {
    let plugins_clone = plugins.clone();

    RouterBuilder::default()
        .append_resource(PluginManager {
            plugins: plugins_clone,
        })
        .append_dyn("initialize", initialize.into_dyn())
        .append_dyn("ping", ping.into_dyn())
        .append_dyn("logging/setLevel", logging_set_level.into_dyn())
        .append_dyn("roots/list", roots_list.into_dyn())
        .append_dyn("prompts/list", prompts_list.into_dyn())
        .append_dyn("prompts/get", prompts_get.into_dyn())
        .append_dyn("resources/list", resources_list.into_dyn())
        .append_dyn("resources/read", resource_read.into_dyn())
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("tools/call", tools_call.into_dyn())
        .build()
}

pub fn notifications_initialized() {}
pub fn notifications_cancelled(_params: CancelledNotification) {}

pub async fn initialize(_request: InitializeRequest) -> HandlerResult<InitializeResponse> {
    let result = InitializeResponse {
        protocol_version: PROTOCOL_VERSION.to_string(),
        server_info: Implementation {
            name: SERVER_NAME.to_string(),
            version: SERVER_VERSION.to_string(),
        },
        capabilities: ServerCapabilities {
            experimental: None,
            prompts: Some(PromptCapabilities::default()),
            resources: None,
            tools: Some(json!({})),
            roots: None,
            sampling: None,
            logging: None,
        },
        instructions: None,
    };
    Ok(result)
}

pub async fn ping(_request: PingRequest) -> HandlerResult<EmptyResult> {
    Ok(EmptyResult {})
}

pub async fn logging_set_level(_request: SetLevelRequest) -> HandlerResult<LoggingResponse> {
    Ok(LoggingResponse {})
}

pub async fn roots_list(_request: Option<ListRootsRequest>) -> HandlerResult<ListRootsResult> {
    let response = ListRootsResult {
        roots: vec![Root {
            name: "my project".to_string(),
            url: "file:///home/user/projects/my-project".to_string(),
        }],
    };
    Ok(response)
}
