use crate::{config::AuthConfig, https_auth::Authenticator};
use anyhow::{Result, anyhow};
use reqwest::Client;
use std::collections::HashMap;
use tokio::sync::OnceCell;
use url::Url;

static REQWEST_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn load_wasm(url: &Url, auths: &Option<HashMap<Url, AuthConfig>>) -> Result<Vec<u8>> {
    match url.scheme() {
        "http" => Ok(REQWEST_CLIENT
            .get_or_init(|| async { reqwest::Client::new() })
            .await
            .get(url.as_str())
            .send()
            .await?
            .bytes()
            .await?
            .to_vec()),
        "https" => Ok(REQWEST_CLIENT
            .get_or_init(|| async { reqwest::Client::new() })
            .await
            .get(url.as_str())
            .add_auth(auths, url)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec()),
        _ => Err(anyhow!("Unsupported URL scheme: {}", url.scheme())),
    }
}
