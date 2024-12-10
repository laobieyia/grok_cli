use reqwest;

pub async fn send_request(
    client: &reqwest::Client,
    request: &crate::grok_request::GrokRequest,
) -> Result<reqwest::Response, reqwest::Error> {
    client
        .post(&request.url)
        .header("Authorization", format!("Bearer {}", request.api_key))
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await
}
