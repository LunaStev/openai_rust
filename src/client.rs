use std::sync::OnceLock;

static API_KEY: OnceLock<String> = OnceLock::new();

/// Initializes the OpenAI client with the given API key.
/// This must be called before any API functions.
pub fn openai_init(key: &str) {
    let _ = API_KEY.set(key.to_string());
}

/// Internal helper to get the API key.
/// Returns None if `openai_init` was not called.
pub(crate) fn get_api_key() -> Option<&'static str> {
    API_KEY.get().map(|s| s.as_str())
}