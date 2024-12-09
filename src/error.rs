use reqwest::StatusCode;
use thiserror::Error;


#[derive(Debug,Error)]
pub enum CliError {
    #[error("Environment variable {0} is missing")]
    MissingEnvVar(String),
    #[error("HTTP request failed with status {0}: {1}")]
    HttpError(StatusCode, String),
    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
}