use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::{Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio;

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
        #[clap(short, long, value_parser)]
        query: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Messages {
    role: String,
    content: String,
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
// 发送请求
async fn send_request(request: &GrokRequest) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(&request.url)
        .header("Authorization", format!("Bearer {}", request.api_key))
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await?;
    Ok(res)
}
async fn parse_response(res: reqwest::Response) -> Result<(), Box<dyn std::error::Error>> {
    let status = res.status();
    let text = res.text().await?;

    println!("HTTP Status: {}", status);

    if status == reqwest::StatusCode::OK {
        let parsed_response: serde_json::Value = serde_json::from_str(&text)?;

        if let Some(choices) = parsed_response["choices"].as_array() {
            if let Some(first_choice) = choices.get(0) {
                if let Some(content) = first_choice["message"]["content"].as_str() {
                    println!("AI Response: {}", content);
                } else {
                    println!("No content found in the response.");
                }
            }
        }
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Request failed with status: {} and message: {}",
                status, text
            ),
        )));
    }
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 .env 文件
    dotenv().ok();
    let api_key: String = env::var("API_KEY").expect("API_KEY must be set in .env");
    let url = env::var("GROK_URL").expect("GROK_URL must be set in .env");
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Ask { query }) => {
            let request = GrokRequest::from(url.clone(), api_key.clone(), query.clone());
            let res = send_request(&request).await?;
            parse_response(res).await?;
        }
        None => {
            // 如果没有提供子命令，则直接执行默认行为（Ask）
            println!("No command provided, please enter a query:");
            let request =
                GrokRequest::from(url.clone(), api_key.clone(), "default query".to_string());
            let res = send_request(&request).await?;
            parse_response(res).await?;
        }
    }

    Ok(())
}

