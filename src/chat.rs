use reqwest::Client;
use serde_json::json;
use crate::client::get_api_key;

#[derive(Debug)]
pub enum OpenAIError {
    MissingApiKey,
    RequestError(reqwest::Error),
    InvalidResponse,
}

impl From<reqwest::Error> for OpenAIError {
    fn from(err: reqwest::Error) -> Self {
        OpenAIError::RequestError(err)
    }
}

/// Sends a prompt using a specified model (e.g., "gpt-3.5-turbo").
pub async fn openai_chat_with_model(prompt: &str, model: &str) -> Result<String, OpenAIError> {
    let api_key = get_api_key().ok_or(OpenAIError::MissingApiKey)?;

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
        Ok(content.to_string())
    } else {
        Err(OpenAIError::InvalidResponse)
    }
}

/// Sends a prompt using the default model "gpt-3.5-turbo".
pub async fn openai_chat(prompt: &str) -> Result<String, OpenAIError> {
    openai_chat_with_model(prompt, "gpt-3.5-turbo").await
}