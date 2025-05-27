use url::Url;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: Url,
    #[serde(rename = "api_version", default = "default_api_version")]
    pub version: String,
    pub rate_limit_sec: u16,
    pub rate_limit_min: u16,
    pub timeout_ms: u16,
    pub client_id: Option<String>,
    pub client_secret: Option<String>
}

pub fn from_json(path_to_config_file: &str) -> ApiConfig {
    let config = std::fs::read_to_string(path_to_config_file).unwrap();
    serde_json::from_str(&config).unwrap()
}

fn default_api_version() -> String {
    "v2".to_string()
}