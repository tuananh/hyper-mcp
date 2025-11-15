use anyhow::{Error, anyhow, bail};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointId {
    Uuid(String),
    Num(u64),
}
impl From<u64> for PointId {
    fn from(num: u64) -> Self {
        PointId::Num(num)
    }
}
impl From<String> for PointId {
    fn from(uuid: String) -> Self {
        PointId::Uuid(uuid)
    }
}
impl Display for PointId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointId::Uuid(uuid) => write!(f, "{}", uuid),
            PointId::Num(num) => write!(f, "{}", num),
        }
    }
}

/// The point struct.
/// A point is a record consisting of a vector and an optional payload.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    /// Id of the point
    pub id: PointId,

    /// Vectors
    pub vector: Vec<f32>,

    /// Additional information along with vectors
    pub payload: Option<Map<String, Value>>,
}

/// The point struct with the score returned by searching
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoredPoint {
    /// Id of the point
    pub id: PointId,

    /// Vectors
    pub vector: Option<Vec<f32>>,

    /// Additional information along with vectors
    pub payload: Option<Map<String, Value>>,

    /// Points vector distance to the query vector
    pub score: f32,
}

pub struct QdrantClient {
    url_base: String,
    api_key: Option<String>,
}

impl QdrantClient {
    pub fn new_with_url(url_base_: String) -> QdrantClient {
        QdrantClient {
            url_base: url_base_,
            api_key: None,
        }
    }

    pub fn new() -> QdrantClient {
        QdrantClient::new_with_url("http://localhost:6333".to_string())
    }

    pub fn set_api_key(&mut self, api_key: impl Into<String>) {
        self.api_key = Some(api_key.into());
    }
}

impl Default for QdrantClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Shortcut functions
impl QdrantClient {
    /// Shortcut functions
    pub fn collection_info(&self, collection_name: &str) -> Result<u64, Error> {
        let v = self.collection_info_api(collection_name)?;
        v.get("result")
            .and_then(|v| v.get("points_count"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("[qdrant] Invalid response format"))
    }

    pub fn create_collection(&self, collection_name: &str, size: u32) -> Result<(), Error> {
        match self.collection_exists(collection_name)? {
            false => (),
            true => {
                let err_msg = format!("Collection '{}' already exists", collection_name);
                bail!(err_msg);
            }
        }

        let params = json!({
            "vectors": {
                "size": size,
                "distance": "Cosine",
                "on_disk": true,
            }
        });
        if !self.create_collection_api(collection_name, &params)? {
            bail!("Failed to create collection '{}'", collection_name);
        }
        Ok(())
    }

    pub fn list_collections(&self) -> Result<Vec<String>, Error> {
        self.list_collections_api()
    }

    pub fn collection_exists(&self, collection_name: &str) -> Result<bool, Error> {
        let collection_names = self.list_collections()?;
        Ok(collection_names.contains(&collection_name.to_string()))
    }

    pub fn delete_collection(&self, collection_name: &str) -> Result<(), Error> {
        match self.collection_exists(collection_name)? {
            true => (),
            false => {
                let err_msg = format!("Not found collection '{}'", collection_name);
                bail!(err_msg);
            }
        }

        if !self.delete_collection_api(collection_name)? {
            bail!("Failed to delete collection '{}'", collection_name);
        }
        Ok(())
    }

    pub fn upsert_points(&self, collection_name: &str, points: Vec<Point>) -> Result<(), Error> {
        let params = json!({
            "points": points,
        });
        self.upsert_points_api(collection_name, &params)
    }

    pub fn search_points(
        &self,
        collection_name: &str,
        vector: Vec<f32>,
        limit: u64,
        score_threshold: Option<f32>,
    ) -> Result<Vec<ScoredPoint>, Error> {
        let score_threshold = score_threshold.unwrap_or(0.0);

        let params = json!({
            "vector": vector,
            "limit": limit,
            "with_payload": true,
            "with_vector": true,
            "score_threshold": score_threshold,
        });

        match self.search_points_api(collection_name, &params) {
            Ok(v) => match v.get("result") {
                Some(v) => match v.as_array() {
                    Some(rs) => {
                        let mut sps: Vec<ScoredPoint> = Vec::<ScoredPoint>::new();
                        for r in rs {
                            let sp: ScoredPoint = serde_json::from_value(r.clone())?;
                            sps.push(sp);
                        }
                        Ok(sps)
                    }
                    None => {
                        bail!(
                            "[qdrant] The value corresponding to the 'result' key is not an array."
                        )
                    }
                },
                None => Ok(vec![]),
            },
            Err(_) => Ok(vec![]),
        }
    }

    pub fn get_points(&self, collection_name: &str, ids: &[PointId]) -> Result<Vec<Point>, Error> {
        let params = json!({
            "ids": ids,
            "with_payload": true,
            "with_vector": true,
        });

        let v = self.get_points_api(collection_name, &params)?;
        let rs = v
            .get("result")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("[qdrant] Invalid response format"))?;

        let mut ps: Vec<Point> = Vec::new();
        for r in rs {
            let p: Point = serde_json::from_value(r.clone())?;
            ps.push(p);
        }
        Ok(ps)
    }

    pub fn get_point(&self, collection_name: &str, id: &PointId) -> Result<Point, Error> {
        let v = self.get_point_api(collection_name, id)?;
        let r = v
            .get("result")
            .ok_or_else(|| anyhow!("[qdrant] Invalid response format"))?;
        Ok(serde_json::from_value(r.clone())?)
    }

    pub fn delete_points(&self, collection_name: &str, ids: &[PointId]) -> Result<(), Error> {
        let params = json!({
            "points": ids,
        });
        self.delete_points_api(collection_name, &params)
    }

    /// REST API functions
    pub fn collection_info_api(&self, collection_name: &str) -> Result<Value, Error> {
        let url = format!("{}/collections/{}", self.url_base, collection_name);

        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let response: HttpResponse = http::request::<()>(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("GET".to_string()),
            },
            None,
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        Ok(json)
    }

    pub fn create_collection_api(
        &self,
        collection_name: &str,
        params: &Value,
    ) -> Result<bool, Error> {
        let url = format!("{}/collections/{}", self.url_base, collection_name);
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let body = serde_json::to_vec(params)?;
        let response = http::request::<Vec<u8>>(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("PUT".to_string()),
            },
            Some(body),
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        let success = json
            .get("result")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| anyhow!("[qdrant] Invalid response format"))?;
        Ok(success)
    }

    pub fn list_collections_api(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/collections", self.url_base);
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let response = http::request::<()>(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("GET".to_string()),
            },
            None,
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;

        match json.get("result") {
            Some(result) => match result.get("collections") {
                Some(collections) => match collections.as_array() {
                    Some(collections) => {
                        let mut collection_names = Vec::new();
                        for collection in collections {
                            if let Some(name) = collection.get("name").and_then(|n| n.as_str()) {
                                collection_names.push(name.to_string());
                            }
                        }
                        Ok(collection_names)
                    }
                    None => bail!(
                        "[qdrant] The value corresponding to the 'collections' key is not an array."
                    ),
                },
                None => bail!("[qdrant] The given key 'collections' does not exist."),
            },
            None => bail!("[qdrant] The given key 'result' does not exist."),
        }
    }

    pub fn collection_exists_api(&self, collection_name: &str) -> Result<bool, Error> {
        let url = format!("{}/collections/{}/exists", self.url_base, collection_name);
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let response = http::request::<()>(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("GET".to_string()),
            },
            None,
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        match json.get("result") {
            Some(result) => {
                let exists = result
                    .get("exists")
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| anyhow!("[qdrant] Invalid response format"))?;
                Ok(exists)
            }
            None => Err(anyhow!("[qdrant] Failed to check collection existence")),
        }
    }

    pub fn delete_collection_api(&self, collection_name: &str) -> Result<bool, Error> {
        let url = format!("{}/collections/{}", self.url_base, collection_name);
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let response = http::request::<()>(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("DELETE".to_string()),
            },
            None,
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        let success = json
            .get("result")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| anyhow!("[qdrant] Invalid response format"))?;
        Ok(success)
    }

    pub fn upsert_points_api(&self, collection_name: &str, params: &Value) -> Result<(), Error> {
        let url = format!(
            "{}/collections/{}/points?wait=true",
            self.url_base, collection_name,
        );
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let body = serde_json::to_vec(params)?;
        let response = http::request(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("PUT".to_string()),
            },
            Some(&body),
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        let status = json
            .get("status")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("[qdrant] Invalid response format"))?;

        if status == "ok" {
            Ok(())
        } else {
            Err(anyhow!(
                "[qdrant] Failed to upsert points. Status = {}",
                status
            ))
        }
    }

    pub fn search_points_api(&self, collection_name: &str, params: &Value) -> Result<Value, Error> {
        let url = format!(
            "{}/collections/{}/points/search",
            self.url_base, collection_name,
        );
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let body = serde_json::to_vec(params)?;
        let response = http::request(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("POST".to_string()),
            },
            Some(&body),
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        Ok(json)
    }

    pub fn get_points_api(&self, collection_name: &str, params: &Value) -> Result<Value, Error> {
        let url = format!("{}/collections/{}/points", self.url_base, collection_name);
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let body = serde_json::to_vec(params)?;
        let response = http::request(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("POST".to_string()),
            },
            Some(&body),
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        Ok(json)
    }

    pub fn get_point_api(&self, collection_name: &str, id: &PointId) -> Result<Value, Error> {
        let url = format!(
            "{}/collections/{}/points/{}",
            self.url_base, collection_name, id,
        );
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let response = http::request::<()>(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("GET".to_string()),
            },
            None,
        )?;

        let json: Value = serde_json::from_slice(&response.body())?;
        Ok(json)
    }

    pub fn delete_points_api(&self, collection_name: &str, params: &Value) -> Result<(), Error> {
        let url = format!(
            "{}/collections/{}/points/delete?wait=true",
            self.url_base, collection_name,
        );
        let mut headers = BTreeMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if let Some(api_key) = &self.api_key {
            headers.insert("api-key".to_string(), api_key.clone());
        }

        let body = serde_json::to_vec(params)?;
        let response = http::request(
            &HttpRequest {
                url: url.clone(),
                headers,
                method: Some("POST".to_string()),
            },
            Some(&body),
        )?;

        Ok(())
    }
}
