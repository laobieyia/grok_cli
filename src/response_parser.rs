use crate::error::CliError;
use reqwest::StatusCode;
use serde_json::Value;

pub async fn parse_response(res: reqwest::Response) -> Result<(), CliError> {
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
