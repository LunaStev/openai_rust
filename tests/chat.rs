use openai_rust::{openai_init, openai_chat_with_model};

#[tokio::main]
async fn main() {
    let api_key = "sk-...";
    let model = "gpt-3.5-turbo";
    let prompt = "Hello, who are you?";

    if api_key.is_empty() {
        eprintln!("ERROR: OpenAI API key is missing.");
        std::process::exit(1);
    }

    openai_init(api_key);

    match openai_chat_with_model(prompt, model).await {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("ERROR: Failed to get response from OpenAI: {:?}", e),
    }
}