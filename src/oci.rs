use docker_credential::{CredentialRetrievalError, DockerCredential};
use flate2::read::GzDecoder;
use oci_client::Reference;
use oci_client::{Client, manifest, manifest::OciDescriptor, secrets::RegistryAuth};
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

fn build_auth(reference: &Reference) -> RegistryAuth {
    let server = reference
        .resolve_registry()
        .strip_suffix('/')
        .unwrap_or_else(|| reference.resolve_registry());

    // if cli.anonymous {
    //     return RegistryAuth::Anonymous;
    // }

    match docker_credential::get_credential(server) {
        Err(CredentialRetrievalError::ConfigNotFound) => RegistryAuth::Anonymous,
        Err(CredentialRetrievalError::NoCredentialConfigured) => RegistryAuth::Anonymous,
        Err(e) => panic!("Error handling docker configuration file: {}", e),
        Ok(DockerCredential::UsernamePassword(username, password)) => {
            info!("Found docker credentials");
            RegistryAuth::Basic(username, password)
        }
        Ok(DockerCredential::IdentityToken(_)) => {
            info!(
                "Cannot use contents of docker config, identity token not supported. Using anonymous auth"
            );
            RegistryAuth::Anonymous
        }
    }
}

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

    let client_config = oci_client::client::ClientConfig::default();
    let client = Client::new(client_config);

    let reference = Reference::try_from(image_reference)?;
    let auth = build_auth(&reference);

    // Accept both OCI and Docker manifest types
    let manifest = client
        .pull(
            &reference,
            &auth,
            vec![
                manifest::IMAGE_MANIFEST_MEDIA_TYPE,
                manifest::IMAGE_DOCKER_LAYER_GZIP_MEDIA_TYPE,
            ],
        )
        .await?;

    for layer in manifest.layers.iter() {
        let mut buf = Vec::new();
        let desc = OciDescriptor {
            digest: layer.sha256_digest().clone(),
            media_type: "application/vnd.docker.image.rootfs.diff.tar.gzip".to_string(),
            ..Default::default()
        };
        client.pull_blob(&reference, &desc, &mut buf).await.unwrap();

        let gz_extract = GzDecoder::new(&buf[..]);
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
