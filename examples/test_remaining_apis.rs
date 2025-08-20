#![allow(clippy::pedantic, clippy::nursery)]
//! Test remaining untested APIs

use openai_rust_sdk::api::{
    assistants::AssistantsApi, audio::AudioApi, files::FilesApi, fine_tuning::FineTuningApi,
    threads::ThreadsApi, vector_stores::VectorStoresApi,
};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| "test".to_string());

    println!("\n🧪 Testing Remaining OpenAI APIs\n");
    println!("{}", "=".repeat(50));

    // Test Files API
    println!("\n📁 Testing Files API...");
    match FilesApi::new(&api_key) {
        Ok(api) => match api.list_files(None).await {
            Ok(files) => println!("✅ Files API works! Found {} files", files.data.len()),
            Err(e) => println!("❌ Files API error: {e}"),
        },
        Err(e) => println!("❌ Failed to create Files API: {e}"),
    }

    // Test Assistants API
    println!("\n🤖 Testing Assistants API...");
    match AssistantsApi::new(&api_key) {
        Ok(api) => {
            match api
                .list_assistants(Some(
                    openai_rust_sdk::models::assistants::ListAssistantsParams::default(),
                ))
                .await
            {
                Ok(assistants) => println!(
                    "✅ Assistants API works! Found {} assistants",
                    assistants.data.len()
                ),
                Err(e) => println!("❌ Assistants API error: {e}"),
            }
        }
        Err(e) => println!("❌ Failed to create Assistants API: {e}"),
    }

    // Test Threads API
    println!("\n🧵 Testing Threads API...");
    match ThreadsApi::new(&api_key) {
        Ok(api) => {
            // Try to create a thread
            use openai_rust_sdk::models::threads::ThreadRequest;
            match api.create_thread(ThreadRequest::default()).await {
                Ok(thread) => {
                    println!("✅ Thread created! ID: {}", thread.id);
                    // Clean up - delete the thread
                    let _ = api.delete_thread(&thread.id).await;
                }
                Err(e) => println!("❌ Threads API error: {e}"),
            }
        }
        Err(e) => println!("❌ Failed to create Threads API: {e}"),
    }

    // Test Vector Stores API
    println!("\n🗄️ Testing Vector Stores API...");
    match VectorStoresApi::new(&api_key) {
        Ok(api) => match api.list_vector_stores(None).await {
            Ok(stores) => println!(
                "✅ Vector Stores API works! Found {} stores",
                stores.data.len()
            ),
            Err(e) => println!("❌ Vector Stores API error: {e}"),
        },
        Err(e) => println!("❌ Failed to create Vector Stores API: {e}"),
    }

    // Test Fine-tuning API
    println!("\n🎯 Testing Fine-tuning API...");
    match FineTuningApi::new(&api_key) {
        Ok(api) => match api.list_fine_tuning_jobs(None).await {
            Ok(jobs) => println!("✅ Fine-tuning API works! Found {} jobs", jobs.data.len()),
            Err(e) => println!("❌ Fine-tuning API error: {e}"),
        },
        Err(e) => println!("❌ Failed to create Fine-tuning API: {e}"),
    }

    // Test Audio Transcription (Whisper)
    println!("\n🎤 Testing Audio Transcription API...");
    match AudioApi::new(&api_key) {
        Ok(_api) => {
            println!("⚠️  Audio transcription requires an actual audio file to test");
            // Would need: api.create_transcription(file_path, "whisper-1").await
        }
        Err(e) => println!("❌ Failed to create Audio API: {e}"),
    }

    println!("\n");
    println!("{}", "=".repeat(50));
    println!("Test complete!");
}
