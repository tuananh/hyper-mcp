use anyhow::Result;
use aws_sdk_s3::Client;
use tokio::sync::OnceCell;
use url::Url;

static S3_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn load_wasm(url: &Url) -> Result<Vec<u8>> {
    let bucket = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("S3 URL must have a valid bucket name in the host"))?;
    let key = url.path().trim_start_matches('/');
    match S3_CLIENT
        .get_or_init(|| async { Client::new(&aws_config::load_from_env().await) })
        .await
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
    {
        Ok(response) => match response.body.collect().await {
            Ok(body) => Ok(body.to_vec()),
            Err(e) => {
                tracing::error!("Failed to collect S3 object body: {e}");
                Err(anyhow::anyhow!("Failed to collect S3 object body: {e}"))
            }
        },
        Err(e) => {
            tracing::error!("Failed to get object from S3: {e}");
            Err(anyhow::anyhow!("Failed to get object from S3: {e}"))
        }
    }
}
