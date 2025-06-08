# OpenAI Rust

A minimal, modern, async-first Rust wrapper for the OpenAI API.

## Features

* Chat completion (GPT-3.5 / GPT-4)
* DALL·E image generation
* Whisper audio transcription & translation
* Fully async with `reqwest`
* No unnecessary dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openai_rust = { package = "openai_rust_sdk", version = "0.3.0" }
```

## Quickstart

### Chat Example

```rust
use openai_rust::{openai_init, openai_chat};

#[tokio::main]
async fn main() {
    openai_init("your-api-key");

    match openai_chat("Hello, who are you?").await {
        Ok(res) => println!("AI: {}", res),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```

### Image Generation

```rust
use openai_rust::{openai_init, openai_generate_image};

#[tokio::main]
async fn main() {
    openai_init("your-api-key");

    let result = openai_generate_image("A cute robot.", 2, "512x512").await;
    println!("{:?}", result);
}
```

### Audio Transcription

```rust
use openai_rust::{openai_init, openai_transcribe_audio};

#[tokio::main]
async fn main() {
    openai_init("your-api-key");

    let transcript = openai_transcribe_audio("./audio.mp3").await;
    println!("{:?}", transcript);
}
```

## Examples

You can run built-in examples using:

```bash
cargo run --example 01_chat
cargo run --example 02_image
cargo run --example 03_audio
```

## License

[MIT License](LICENSE)

---

Made with ❤️ by LunaStev
