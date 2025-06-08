use openai_rust::{openai_init, openai_transcribe_audio, openai_translate_audio};

#[tokio::main]
async fn main() {
    let api_key = "sk-...";
    let audio_file = "alloy.wav";

    if api_key.is_empty() {
        eprintln!("ERROR: OpenAI API key is missing.");
        std::process::exit(1);
    }

    openai_init(api_key);

    match openai_transcribe_audio(audio_file).await {
        Ok(transcription) => {
            println!("Transcription result:\n{}", transcription);

            match openai_translate_audio(audio_file).await {
                Ok(translation) => println!("Translation result:\n{}", translation),
                Err(e) => eprintln!("Failed to translate audio: {:?}", e),
            }
        }
        Err(e) => eprintln!("Failed to transcribe audio: {:?}", e),
    }
}