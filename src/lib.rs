mod client;
mod chat;
mod image;
mod audio;

pub use client::openai_init;
pub use chat::{openai_chat, openai_chat_with_model, OpenAIError};
pub use image::{openai_generate_image, ImageGenError};
pub use audio::{openai_transcribe_audio, openai_translate_audio, AudioError};