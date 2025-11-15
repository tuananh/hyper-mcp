use anyhow::Result;
use clap::Parser;
use rmcp::transport::sse_server::SseServer;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use rmcp::{ServiceExt, transport::stdio};
use std::path::PathBuf;
use tokio::{runtime::Handle, task::block_in_place};

mod config;
mod https_auth;
mod logging;
mod oci;
mod plugin;
mod service;

pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:3001";

#[derive(Parser, Clone)]
#[command(author = "Tuan Anh Tran <me@tuananh.org>", version = env!("CARGO_PKG_VERSION"), about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,

    #[arg(
        long = "transport",
        value_name = "TRANSPORT",
        env = "HYPER_MCP_TRANSPORT",
        default_value = "stdio",
        value_parser = ["stdio", "sse", "streamable-http"]
    )]
    transport: String,

    #[arg(
        long = "bind-address",
        value_name = "ADDRESS",
        env = "HYPER_MCP_BIND_ADDRESS",
        default_value = DEFAULT_BIND_ADDRESS
    )]
    bind_address: String,

    #[arg(
        long = "insecure-skip-signature",
        help = "Skip OCI image signature verification",
        env = "HYPER_MCP_INSECURE_SKIP_SIGNATURE",
        default_value = "false"
    )]
    insecure_skip_signature: bool,

    #[arg(
        long = "use-sigstore-tuf-data",
        help = "Use Sigstore TUF data for verification",
        env = "HYPER_MCP_USE_SIGSTORE_TUF_DATA",
        default_value = "true"
    )]
    use_sigstore_tuf_data: bool,

    #[arg(
        long = "rekor-pub-keys",
        help = "Path to Rekor public keys for verification",
        env = "HYPER_MCP_REKOR_PUB_KEYS"
    )]
    rekor_pub_keys: Option<PathBuf>,

    #[arg(
        long = "fulcio-certs",
        help = "Path to Fulcio certificates for verification",
        env = "HYPER_MCP_FULCIO_CERTS"
    )]
    fulcio_certs: Option<PathBuf>,

    #[arg(
        long = "cert-issuer",
        help = "Certificate issuer to verify against",
        env = "HYPER_MCP_CERT_ISSUER"
    )]
    cert_issuer: Option<String>,

    #[arg(
        long = "cert-email",
        help = "Certificate email to verify against",
        env = "HYPER_MCP_CERT_EMAIL"
    )]
    cert_email: Option<String>,

    #[arg(
        long = "cert-url",
        help = "Certificate URL to verify against",
        env = "HYPER_MCP_CERT_URL"
    )]
    cert_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing::info!("Starting hyper-mcp server");

    match cli.transport.as_str() {
        "stdio" => {
            tracing::info!("Starting hyper-mcp with stdio transport");
            let service = service::PluginService::new(&cli)
                .await?
                .serve(stdio())
                .await
                .inspect_err(|e| {
                    tracing::error!("Serving error: {:?}", e);
                })?;
            service.waiting().await?;
        }
        "sse" => {
            tracing::info!(
                "Starting hyper-mcp with SSE transport at {}",
                cli.bind_address
            );
            let ct = SseServer::serve(cli.bind_address.parse()?)
                .await?
                .with_service({
                    move || {
                        block_in_place(|| {
                            Handle::current()
                                .block_on(async { service::PluginService::new(&cli).await })
                        })
                        .expect("Failed to create plugin service")
                    }
                });

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
        "streamable-http" => {
            let bind_address = cli.bind_address.clone();
            tracing::info!(
                "Starting hyper-mcp with streamable-http transport at {}/mcp",
                bind_address
            );

            let service = StreamableHttpService::new(
                {
                    move || {
                        block_in_place(|| {
                            Handle::current()
                                .block_on(async { service::PluginService::new(&cli).await })
                        })
                        .map_err(std::io::Error::other)
                    }
                },
                LocalSessionManager::default().into(),
                Default::default(),
            );

            let router = axum::Router::new().nest_service("/mcp", service);

            let _ = axum::serve(tokio::net::TcpListener::bind(bind_address).await?, router)
                .with_graceful_shutdown(async {
                    tokio::signal::ctrl_c().await.unwrap();
                    tracing::info!("Received Ctrl+C, shutting down hyper-mcp server...");
                    // Give the log a moment to flush
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    std::process::exit(0);
                })
                .await;
        }
        _ => unreachable!(),
    }

    Ok(())
}
