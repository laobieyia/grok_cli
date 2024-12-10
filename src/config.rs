use crate::error::CliError;
use std::env;
pub struct Config {
    pub url: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, CliError> {
        dotenv::dotenv().ok();
        Ok(Self {
            url: get_env_var("GROK_URL")?,
            api_key: get_env_var("API_KEY")?,
        })
    }
}
fn get_env_var(key: &str) -> Result<String, CliError> {
    env::var(key).map_err(|_| CliError::MissingEnvVar(key.to_string()))
}
