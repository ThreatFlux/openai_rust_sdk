#![allow(clippy::pedantic, clippy::nursery)]
//! # Audio API Demo
//!
//! This example demonstrates how to use the OpenAI Audio API for:
//! - Text-to-speech (TTS) generation
//! - Speech-to-text transcription
//! - Audio translation
//! - Different voice options and formats
//!
//! ## Usage
//!
//! Make sure you have set the OPENAI_API_KEY environment variable:
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! cargo run --example audio_demo
//! ```

use openai_rust_sdk::api::{
    audio::{AudioApi, AudioUtils},
    common::ApiClientConstructors,
};
use openai_rust_sdk::models::audio::*;
use std::env;
use std::path::Path;
use tokio::fs;

async fn demo_voice_generation(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("🗣️  Demo 1: Text-to-Speech with Different Voices");
    println!("=================================================");

    let sample_text = "Hello! Welcome to the OpenAI Audio API demonstration. This showcases the text-to-speech capabilities with different voice options.";

    for voice in AudioApi::available_voices() {
        println!("🎤 Generating speech with {voice:?} voice...");

        let request = SpeechBuilder::tts_1(sample_text, voice.clone())
            .mp3()
            .speed(1.0)
            .build();

        let response = audio_api.create_speech(&request).await?;

        let filename = format!("output_voice_{voice:?}.mp3").to_lowercase();
        response.save_to_file(&filename).await?;

        println!("✅ Saved {} ({} bytes)", filename, response.data().len());

        let cost = AudioUtils::estimate_tts_cost(sample_text, AudioModels::TTS_1);
        println!("💰 Estimated cost: ${cost:.6}");
    }

    println!();
    Ok(())
}

async fn demo_hq_formats(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Demo 2: High-Quality TTS with Different Formats");
    println!("==================================================");

    let formats = vec![
        AudioFormat::Mp3,
        AudioFormat::Opus,
        AudioFormat::Aac,
        AudioFormat::Flac,
    ];

    let hq_text = "This is a high-quality audio sample demonstrating different output formats available in the OpenAI TTS API.";

    for format in formats {
        println!("🔧 Generating {format:?} format...");

        let request = SpeechBuilder::tts_1_hd(hq_text, Voice::Nova)
            .format(format.clone())
            .speed(1.25)
            .build();

        let response = audio_api.create_speech(&request).await?;

        let extension = match format {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Opus => "opus",
            AudioFormat::Aac => "aac",
            AudioFormat::Flac => "flac",
            _ => "audio",
        };

        let filename = format!("hq_output.{extension}");
        response.save_to_file(&filename).await?;

        println!("✅ Saved {} ({} bytes)", filename, response.data().len());

        let duration =
            AudioUtils::estimate_duration_from_size(response.data().len() as u64, &format);
        println!("⏱️  Estimated duration: {duration:.1} seconds");
    }

    println!();
    Ok(())
}

async fn demo_streaming_audio(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Demo 3: Streaming Text-to-Speech");
    println!("===================================");

    let streaming_text = "This demonstrates streaming audio generation where audio data is received in chunks as it's generated.";

    let request = SpeechBuilder::tts_1(streaming_text, Voice::Fable)
        .opus()
        .build();

    println!("🌊 Starting streaming audio generation...");

    let mut stream = audio_api.create_speech_stream(&request).await?;
    let mut total_bytes = 0;
    let mut chunks = Vec::new();

    use tokio_stream::StreamExt;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        let chunk_len = chunk.len();
        total_bytes += chunk_len;
        chunks.push(chunk);
        println!("📦 Received chunk: {chunk_len} bytes (total: {total_bytes} bytes)");
    }

    let combined_data: Vec<u8> = chunks.into_iter().flatten().collect();
    fs::write("streaming_output.opus", &combined_data).await?;
    println!(
        "✅ Streaming complete! Saved streaming_output.opus ({} bytes)",
        combined_data.len()
    );

    println!();
    Ok(())
}

async fn demo_voice_recommendations(
    audio_api: &AudioApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 Demo 4: Voice Recommendations for Different Use Cases");
    println!("========================================================");

    let use_cases = vec![
        "professional",
        "friendly",
        "storytelling",
        "energetic",
        "deep",
        "customer_service",
        "marketing",
        "audiobook",
    ];

    for use_case in use_cases {
        let recommended_voice = AudioUtils::recommend_voice(use_case);
        println!("📋 For '{use_case}' use case: {recommended_voice:?} voice recommended");

        let demo_text = format!("This is a {use_case} voice demonstration.");

        let response = audio_api
            .generate_speech(demo_text, recommended_voice, Some(AudioModels::TTS_1))
            .await?;

        let filename = format!("usecase_{use_case}.mp3");
        response.save_to_file(&filename).await?;
        println!("✅ Generated sample: {filename}");
    }

    println!();
    Ok(())
}

async fn perform_basic_transcription(
    audio_api: &AudioApi,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match audio_api.transcribe(file_path, Some("en")).await {
        Ok(transcription) => {
            println!("📝 Transcription: \"{transcription}\"");
        }
        Err(e) => {
            println!("❌ Transcription failed: {e}");
        }
    }
    Ok(())
}

async fn perform_detailed_transcription(
    audio_api: &AudioApi,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let detailed_request = TranscriptionBuilder::whisper(file_path)
        .language("en")
        .verbose_json()
        .word_timestamps()
        .temperature(0.1)
        .build();

    match audio_api
        .transcribe_file(file_path, &detailed_request)
        .await
    {
        Ok(response) => {
            print_detailed_results(&response);
        }
        Err(e) => {
            println!("❌ Detailed transcription failed: {e}");
        }
    }
    Ok(())
}

fn print_detailed_results(response: &AudioTranscriptionResponse) {
    println!("📊 Detailed transcription results:");
    println!("   Text: \"{}\"", response.text);

    if let Some(duration) = response.duration() {
        println!("   Duration: {duration:.2} seconds");
        let cost = AudioUtils::estimate_whisper_cost(duration);
        println!("   💰 Estimated cost: ${cost:.6}");
    }

    if let Some(words) = response.words() {
        println!("   Word count: {}", words.len());
        if !words.is_empty() {
            let first_word = &words[0];
            println!(
                "   First word: \"{}\" ({:.2}s - {:.2}s)",
                first_word.word, first_word.start, first_word.end
            );
        }
    }
}

async fn process_audio_file(
    audio_api: &AudioApi,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        println!("⚠️  File {file_path} not found, skipping transcription");
        return Ok(());
    }

    println!("🔍 Transcribing {file_path}...");

    if !AudioApi::is_supported_format(file_path) {
        println!("❌ File format not supported");
        return Ok(());
    }

    println!("✅ File format is supported");
    perform_basic_transcription(audio_api, file_path).await?;
    perform_detailed_transcription(audio_api, file_path).await?;

    Ok(())
}

async fn demo_transcription(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎙️  Demo 5: Speech-to-Text Transcription");
    println!("=========================================");

    let test_files = vec!["output_voice_alloy.mp3", "hq_output.mp3"];

    for file_path in test_files {
        process_audio_file(audio_api, file_path).await?;
        println!();
    }

    Ok(())
}

async fn demo_response_formats(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("📄 Demo 6: Different Transcription Response Formats");
    println!("===================================================");

    if !Path::new("output_voice_alloy.mp3").exists() {
        println!();
        return Ok(());
    }

    let file_path = "output_voice_alloy.mp3";
    type FormatFn = fn(TranscriptionBuilder) -> TranscriptionBuilder;
    let formats: [(&str, FormatFn); 4] = [
        ("JSON", |b| b.json()),
        ("Plain Text", |b| b.text()),
        ("SRT Subtitles", |b| b.srt()),
        ("WebVTT Subtitles", |b| b.vtt()),
    ];

    for (name, format_fn) in formats {
        println!("📝 Testing {name} format...");

        let request = format_fn(TranscriptionBuilder::whisper(file_path).language("en")).build();

        match audio_api.transcribe_file(file_path, &request).await {
            Ok(response) => {
                let display_text = response.text.chars().take(100).collect::<String>();
                println!("✅ {name} result: \"{display_text}\"");
                if response.text.len() > 100 {
                    println!("   ... (truncated)");
                }
            }
            Err(e) => println!("❌ {name} format failed: {e}"),
        }
    }

    println!();
    Ok(())
}

async fn demo_audio_info() {
    println!("ℹ️  Demo 7: Audio Information and Utilities");
    println!("===========================================");

    println!("📋 Supported input formats:");
    for format in AudioApi::supported_input_formats() {
        println!("   • {format}");
    }

    println!("\n🎵 Supported output formats:");
    for format in AudioApi::supported_output_formats() {
        println!("   • {format:?}");
    }

    println!("\n🎤 Available voices:");
    for voice in AudioApi::available_voices() {
        println!("   • {voice:?}");
    }

    println!("\n💰 Cost Estimates:");
    let sample_text = "This is a sample text for cost estimation.";
    println!(
        "   TTS Standard: ${:.6} for \"{}\"",
        AudioUtils::estimate_tts_cost(sample_text, "tts-1"),
        sample_text
    );
    println!(
        "   TTS HD: ${:.6} for \"{}\"",
        AudioUtils::estimate_tts_cost(sample_text, "tts-1-hd"),
        sample_text
    );
    println!(
        "   Whisper: ${:.6} per minute of audio",
        AudioUtils::estimate_whisper_cost(60.0)
    );

    println!();
}

async fn demo_error_handling(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚠️  Demo 8: Error Handling Examples");
    println!("===================================");

    println!("🔍 Testing with non-existent file...");
    match audio_api.transcribe("nonexistent.mp3", None).await {
        Ok(_) => println!("😱 Unexpected success!"),
        Err(e) => println!("✅ Expected error: {e}"),
    }

    println!("🔍 Testing format validation...");
    if AudioApi::is_supported_format("test.txt") {
        println!("❌ Should not support .txt files");
    } else {
        println!("✅ Correctly rejected .txt file");
    }

    println!();
    Ok(())
}

fn demo_cleanup() {
    println!("🧹 Demo Complete!");
    println!("=================");
    println!("Generated files:");
    let possible_files = vec![
        "output_voice_alloy.mp3",
        "output_voice_echo.mp3",
        "output_voice_fable.mp3",
        "output_voice_onyx.mp3",
        "output_voice_nova.mp3",
        "output_voice_shimmer.mp3",
        "hq_output.mp3",
        "hq_output.opus",
        "hq_output.aac",
        "hq_output.flac",
        "streaming_output.opus",
        "usecase_professional.mp3",
        "usecase_friendly.mp3",
        "usecase_storytelling.mp3",
        "usecase_energetic.mp3",
        "usecase_deep.mp3",
        "usecase_customer_service.mp3",
        "usecase_marketing.mp3",
        "usecase_audiobook.mp3",
    ];

    for file in possible_files {
        if Path::new(file).exists() {
            println!("   📁 {file}");
        }
    }

    println!("\n💡 Tips:");
    println!("   • Use different voices for different use cases");
    println!("   • Choose appropriate formats based on your needs");
    println!("   • Use streaming for real-time applications");
    println!("   • Consider cost when processing large amounts of text/audio");
    println!("   • Use verbose_json for detailed transcription metadata");
}

/// Run speech synthesis demonstrations
async fn run_speech_demos(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    demo_voice_generation(audio_api).await?;
    demo_hq_formats(audio_api).await?;
    demo_streaming_audio(audio_api).await?;
    demo_voice_recommendations(audio_api).await?;
    Ok(())
}

/// Run transcription and format demonstrations
async fn run_transcription_demos(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    demo_transcription(audio_api).await?;
    demo_response_formats(audio_api).await?;
    Ok(())
}

/// Run utility and information demonstrations
async fn run_utility_demos(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    demo_audio_info().await;
    demo_error_handling(audio_api).await?;
    demo_cleanup();
    Ok(())
}

async fn run_audio_demos(audio_api: &AudioApi) -> Result<(), Box<dyn std::error::Error>> {
    run_speech_demos(audio_api).await?;
    run_transcription_demos(audio_api).await?;
    run_utility_demos(audio_api).await?;
    Ok(())
}

fn initialize_audio_api() -> Result<AudioApi, Box<dyn std::error::Error>> {
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable must be set");
    Ok(AudioApi::new(api_key)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 OpenAI Audio API Demo");
    println!("========================\n");

    let audio_api = initialize_audio_api()?;
    run_audio_demos(&audio_api).await?;

    Ok(())
}

/// Helper function to demonstrate builder patterns
#[allow(dead_code)]
async fn demonstrate_builders() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let audio_api = AudioApi::new(api_key)?;

    // Speech generation examples
    let speech_request = SpeechBuilder::tts_1_hd("Hello world", Voice::Alloy)
        .flac()
        .speed(1.2)
        .build();

    let _speech_response = audio_api.create_speech(&speech_request).await?;

    // Transcription examples
    let transcription_request = TranscriptionBuilder::whisper("audio.mp3")
        .language("en")
        .verbose_json()
        .word_timestamps()
        .temperature(0.2)
        .build();

    let _transcription_response = audio_api
        .transcribe_file("audio.mp3", &transcription_request)
        .await?;

    // Translation examples
    let translation_request = TranslationBuilder::whisper("spanish_audio.mp3")
        .prompt("Please translate this Spanish audio to English")
        .json()
        .temperature(0.0)
        .build();

    let _translation_response = audio_api
        .translate_file("spanish_audio.mp3", &translation_request)
        .await?;

    Ok(())
}

/// Helper function to demonstrate convenience methods
#[allow(dead_code)]
async fn demonstrate_convenience_methods() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let audio_api = AudioApi::new(api_key)?;

    // Simple speech generation
    let _response = audio_api
        .generate_speech("Hello world", Voice::Alloy, Some(AudioModels::TTS_1_HD))
        .await?;

    // Generate and save to file
    audio_api
        .generate_speech_to_file(
            "Save this to a file",
            Voice::Nova,
            "output.mp3",
            Some(AudioModels::TTS_1),
            Some(AudioFormat::Mp3),
        )
        .await?;

    // Simple transcription
    let _text = audio_api.transcribe("audio.mp3", Some("en")).await?;

    // Simple translation
    let _translated_text = audio_api.translate("foreign_audio.mp3").await?;

    Ok(())
}
