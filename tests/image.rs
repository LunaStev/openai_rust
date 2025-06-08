use openai_rust::{openai_init, openai_generate_image};

#[tokio::main]
async fn main() {
    let api_key = "sk-...";
    let prompt = "Draw me a cute cat.";

    if api_key.is_empty() {
        eprintln!("ERROR: OpenAI API key is missing.");
        std::process::exit(1);
    }

    openai_init(api_key);

    match openai_generate_image(prompt, 2, "512x512").await {
        Ok(urls) => {
            println!("Image URLs:");
            for url in urls {
                println!("- {}", url);
            }
        }
        Err(e) => eprintln!("Failed to generate image: {:?}", e),
    }
}