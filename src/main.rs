use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio;
mod error;
use error::CliError;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ask {
        /// Query to send to Grok API
        #[clap(short, long, value_parser, default_value = "default query")]
        query: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Messages {
    role: String,
    content: String,
}

struct Config {
    api_key: String,
    url: String,
}
impl Config {
    fn from_env() -> Result<Self, CliError> {
        dotenv().ok();
        Ok(Self {
            api_key: get_env_var("API_KEY")?,
            url: get_env_var("GROK_URL")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct GrokRequest {
    url: String,
    api_key: String,
    model: String,
    messages: Vec<Messages>,
}
impl GrokRequest {
    fn from(url: String, api_key: String, query: String) -> Self {
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
fn get_env_var(key: &str) -> Result<String, CliError> {
    env::var(key).map_err(|_| CliError::MissingEnvVar(key.to_string()))
}
// 发送请求
async fn send_request(
    client: &reqwest::Client,
    request: &GrokRequest,
) -> Result<reqwest::Response, reqwest::Error> {
    client
        .post(&request.url)
        .header("Authorization", format!("Bearer {}", request.api_key))
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await
}
async fn parse_response(res: reqwest::Response) -> Result<(), CliError> {
    let status = res.status();
    let text = res.text().await?;

    if status != StatusCode::OK {
        return Err(CliError::HttpError(status, text));
    }
    let parsed_response: Value = serde_json::from_str(&text)?;
    if let Some(content) = parsed_response["choices"]
        .as_array()
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice["message"]["content"].as_str())
    {
        println!("AI Response: {}", content);
    } else {
        println!("No content found in the response.")
    }
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 .env 文件
    dotenv().ok();
    let cli = Cli::parse();
    let config = Config::from_env()?;
    let client = reqwest::Client::new();
    match &cli.command {
        Some(Commands::Ask { query }) => {
            let request = GrokRequest::from(config.url, config.api_key, query.clone());
            let res = send_request(&client, &request).await?;
            parse_response(res).await?;
        }
        None => {
            let request =
                GrokRequest::from(config.url, config.api_key, "default query".to_string());
            let res = send_request(&client, &request).await?;
            parse_response(res).await?;
        }
    }

    Ok(())
}
