use reqwest::{Client, multipart};
use crate::client::get_api_key;

#[derive(Debug)]
pub enum AudioError {
    MissingApiKey,
    RequestError(reqwest::Error),
    InvalidResponse,
}

impl From<reqwest::Error> for AudioError {
    fn from(err: reqwest::Error) -> Self {
        AudioError::RequestError(err)
    }
}

/// Transcribes an audio file using Whisper.
pub async fn openai_transcribe_audio(filepath: &str) -> Result<String, AudioError> {
    send_audio_request(filepath, "https://api.openai.com/v1/audio/transcriptions").await
}

/// Translates an audio file to English using Whisper.
pub async fn openai_translate_audio(filepath: &str) -> Result<String, AudioError> {
    send_audio_request(filepath, "https://api.openai.com/v1/audio/translations").await
}

/// Shared internal function to send audio requests.
async fn send_audio_request(filepath: &str, url: &str) -> Result<String, AudioError> {
    let api_key = get_api_key().ok_or(AudioError::MissingApiKey)?;

    let client = Client::new();

    let file_part = multipart::Part::file(filepath).await.map_err(|_| AudioError::InvalidResponse)?;
    let form = multipart::Form::new()
        .part("file", file_part)
        .text("model", "whisper-1");

    let response = client
        .post(url)
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    if let Some(text) = json["text"].as_str() {
        Ok(text.to_string())
    } else {
        Err(AudioError::InvalidResponse)
    }
}