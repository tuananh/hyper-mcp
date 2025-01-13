use flate2::read::GzDecoder;
use oci_distribution::Reference;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::Path;
use tar::Archive;
use tracing::info;

// Docker manifest format v2
#[derive(Debug, Serialize, Deserialize)]
struct DockerManifest {
    #[serde(rename = "schemaVersion")]
    schema_version: u32,
    #[serde(rename = "mediaType")]
    media_type: String,
    config: DockerManifestConfig,
    layers: Vec<DockerManifestLayer>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerManifestConfig {
    #[serde(rename = "mediaType")]
    media_type: String,
    size: u64,
    digest: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerManifestLayer {
    #[serde(rename = "mediaType")]
    media_type: String,
    size: u64,
    digest: String,
}

// TODO: should we use https://github.com/bytecodealliance/wasm-pkg-tools for packaging?
pub async fn pull_and_extract_oci_image(
    image_reference: &str,
    target_file_path: &str,
    local_output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(local_output_path).exists() {
        info!(
            "Plugin {} already cached at: {}. Skipping downloading.",
            image_reference, local_output_path
        );
        return Ok(());
    }

    info!("Pulling {} ...", image_reference);

    let reference = Reference::try_from(image_reference)?;

    let client = reqwest::Client::new();
    let manifest_url = format!(
        "https://{}/v2/{}/manifests/{}",
        reference.registry(),
        reference.repository(),
        reference.tag().unwrap_or("latest")
    );

    info!("Fetching manifest from: {}", manifest_url);
    let manifest_response = client
        .get(&manifest_url)
        .header(
            "Accept",
            // Request both Docker and OCI manifest formats
            "application/vnd.docker.distribution.manifest.v2+json, \
             application/vnd.oci.image.manifest.v1+json",
        )
        .send()
        .await?;

    let manifest: DockerManifest = manifest_response.json().await?;
    info!("Manifest fetched successfully");

    for (_i, layer) in manifest.layers.iter().enumerate() {
        let blob_url = format!(
            "https://{}/v2/{}/blobs/{}",
            reference.registry(),
            reference.repository(),
            layer.digest
        );

        let response = client.get(&blob_url).send().await?;
        let bytes = response.bytes().await?;

        let gz_extract = GzDecoder::new(&bytes[..]);
        let mut archive_extract = Archive::new(gz_extract);

        for entry_result in archive_extract.entries()? {
            match entry_result {
                Ok(mut entry) => {
                    if let Ok(path) = entry.path() {
                        let path_str = path.to_string_lossy();
                        if path_str.ends_with(target_file_path) || path_str.ends_with("plugin.wasm")
                        {
                            if let Some(parent) = Path::new(local_output_path).parent() {
                                fs::create_dir_all(parent)?;
                            }
                            let mut content = Vec::new();
                            entry.read_to_end(&mut content)?;
                            fs::write(local_output_path, content)?;
                            info!("Successfully extracted to: {}", local_output_path);
                            return Ok(());
                        }
                    }
                }
                Err(e) => info!("Error during extraction: {}", e),
            }
        }
    }

    Err("Target file not found in any layer".into())
}
