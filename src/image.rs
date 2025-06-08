use reqwest::Client;
use serde_json::json;
use crate::client::get_api_key;

#[derive(Debug)]
pub enum ImageGenError {
    MissingApiKey,
    InvalidCount,
    RequestError(reqwest::Error),
    InvalidResponse,
}

impl From<reqwest::Error> for ImageGenError {
    fn from(err: reqwest::Error) -> Self {
        ImageGenError::RequestError(err)
    }
}

/// Generates image(s) from a prompt using DALL·E.
pub async fn openai_generate_image(prompt: &str, n: u32, size: &str) -> Result<Vec<String>, ImageGenError> {
    if n == 0 || n > 10 {
        return Err(ImageGenError::InvalidCount);
    }

    let api_key = get_api_key().ok_or(ImageGenError::MissingApiKey)?;

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .bearer_auth(api_key)
        .json(&json!({
            "prompt": prompt,
            "n": n,
            "size": size
        }))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    if let Some(array) = json["data"].as_array() {
        let urls: Vec<String> = array
            .iter()
            .filter_map(|item| item["url"].as_str().map(|s| s.to_string()))
            .collect();
        Ok(urls)
    } else {
        Err(ImageGenError::InvalidResponse)
    }
}