mod commands;
mod config;
mod error;
mod grok_request;
mod http_client;
mod response_parser;

use clap::Parser;
use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let cli = commands::Cli::parse();
    let config = config::Config::from_env()?;
    let client = reqwest::Client::new();

    match &cli.command {
        Some(commands::Commands::Ask { query }) => {
            let request =
                grok_request::GrokRequest::from(config.url, config.api_key, query.clone());
            let res = http_client::send_request(&client, &request).await?;
            response_parser::parse_response(res).await?;
        }
        None => {
            let request = grok_request::GrokRequest::from(
                config.url,
                config.api_key,
                "default query".to_string(),
            );
            let res = http_client::send_request(&client, &request).await?;
            response_parser::parse_response(res).await?;
        }
    }

    Ok(())
}
