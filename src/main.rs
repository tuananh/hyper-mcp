use anyhow::Result;
use clap::Parser;
use rmcp::transport::sse_server::SseServer;
use rmcp::{ServiceExt, transport::stdio};
use std::path::PathBuf;
use tracing_subscriber::{self, EnvFilter};

mod config;
mod oci;
mod plugins;

pub const JSONRPC_VERSION: &str = "2.0";
pub const SERVER_NAME: &str = "hyper-mcp";
pub const SERVER_VERSION: &str = "0.1.0";
pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:3001";

#[derive(Parser)]
#[command(author = "Tuan Anh Tran <me@tuananh.org>", version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,

    #[arg(
        long = "log-level",
        value_name = "LEVEL",
        env = "HYPER_MCP_LOG_LEVEL",
        default_value = "info"
    )]
    log_level: Option<String>,

    #[arg(
        long = "transport",
        value_name = "TRANSPORT",
        env = "HYPER_MCP_TRANSPORT",
        default_value = "stdio",
        value_parser = ["stdio", "sse"]
    )]
    transport: String,

    #[arg(
        long = "bind-address",
        value_name = "ADDRESS",
        env = "HYPER_MCP_BIND_ADDRESS",
        default_value = DEFAULT_BIND_ADDRESS
    )]
    bind_address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = cli.log_level.unwrap_or_else(|| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(log_level.parse().unwrap()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting hyper-mcp server");

    // Get default config path in the user's config directory
    let default_config_path = dirs::config_dir()
        .map(|mut path| {
            path.push("hyper-mcp");
            path.push("config.json");
            path
        })
        .unwrap();

    let config_path = cli.config_file.unwrap_or(default_config_path);
    tracing::info!("Using config file at {}", config_path.display());

    let config = config::load_config(&config_path).await?;

    let plugin_service = plugins::PluginService::new(config.clone()).await?;

    match cli.transport.as_str() {
        "stdio" => {
            let service = plugin_service.serve(stdio()).await.inspect_err(|e| {
                tracing::error!("Serving error: {:?}", e);
            })?;
            service.waiting().await?;
        }
        "sse" => {
            tracing::info!("Starting SSE server at {}", cli.bind_address);
            let ct = SseServer::serve(cli.bind_address.parse()?)
                .await?
                .with_service(move || plugin_service.clone());

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
        _ => unreachable!(),
    }

    Ok(())
}
