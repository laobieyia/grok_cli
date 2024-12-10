use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrokRequest {
    pub url: String,
    pub api_key: String,
    pub model: String,
    pub messages: Vec<Messages>,
}

impl GrokRequest {
    pub fn from(url: String, api_key: String, query: String) -> Self {
        Self {
            url,
            api_key,
            model: "grok-beta".to_string(),
            messages: vec![Messages {
                role: "user".to_string(),
                content: query,
            }],
        }
    }
}
