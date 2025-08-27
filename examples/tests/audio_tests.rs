//! Audio API test module

use openai_rust_sdk::{
    api::{audio::AudioApi, common::ApiClientConstructors},
    error::Result,
    models::audio::{AudioSpeechRequest, AudioTranscriptionRequest, Voice},
};
use tokio::fs;

pub async fn run_audio_api_test(api_key: &str) -> Result<()> {
    println!("\n🔊 Test 6: Audio API");
    println!("{}", "-".repeat(70));
    test_audio_api(api_key).await
}

async fn test_audio_api(api_key: &str) -> Result<()> {
    let api = AudioApi::new(api_key)?;

    test_text_to_speech(&api).await
}

async fn test_text_to_speech(api: &AudioApi) -> Result<()> {
    println!("   🔊 Testing Text-to-Speech...");
    let speech_request = AudioSpeechRequest::new(
        "tts-1",
        "Hello, this is a test of the OpenAI text-to-speech API.",
        Voice::Alloy,
    );

    match api.create_speech(&speech_request).await {
        Ok(response) => {
            println!(
                "   ✅ Speech generated: {} bytes",
                response.audio_data.len()
            );
            println!("      Content type: {}", response.content_type);

            test_transcription_workflow(api, response.audio_data).await?;
        }
        Err(e) => {
            println!("   ❌ Speech generation failed: {e}");
        }
    }

    Ok(())
}

async fn test_transcription_workflow(api: &AudioApi, audio_data: Vec<u8>) -> Result<()> {
    // Save audio file for transcription test
    let audio_dir =
        tempfile::tempdir().map_err(openai_rust_sdk::invalid_request_err!(to_string))?;
    let audio_path = audio_dir.path().join("test_speech.mp3");

    match fs::write(&audio_path, &audio_data).await {
        Ok(_) => {
            println!("      Audio saved to: {audio_path:?}");
            test_transcription(api, &audio_path, audio_data).await?;
        }
        Err(e) => println!("   ❌ Failed to save audio: {e}"),
    }

    Ok(())
}

async fn test_transcription(
    api: &AudioApi,
    audio_path: &std::path::Path,
    audio_data: Vec<u8>,
) -> Result<()> {
    println!("   🎤 Testing Transcription...");
    let transcription_request =
        AudioTranscriptionRequest::new(audio_path.to_str().unwrap(), "whisper-1");

    match api
        .create_transcription(&transcription_request, audio_data.clone())
        .await
    {
        Ok(transcription) => {
            println!("   ✅ Transcription successful: {}", transcription.text);
        }
        Err(e) => {
            println!("   ❌ Transcription failed: {e}");
            if e.to_string().contains("400") {
                println!("      Note: May need actual audio file");
            }
        }
    }

    // Clean up
    let _ = fs::remove_file(audio_path).await;
    Ok(())
}
