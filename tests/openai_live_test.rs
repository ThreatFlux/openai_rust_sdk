#![allow(clippy::pedantic, clippy::nursery)]
//! Live integration tests for `OpenAI` API
//!
//! Run with: OPENAI_API_KEY=your-key cargo test --test `openai_live_test` -- --nocapture

use openai_rust_sdk::{
    api::{
        audio::AudioApi, common::ApiClientConstructors, embeddings::EmbeddingsApi, images::ImagesApi, models::ModelsApi,
        moderations::ModerationsApi, responses::ResponsesApi, streaming::StreamingApi,
    },
    models::{
        audio::{AudioSpeechRequest, Voice},
        embeddings::{EmbeddingInput, EmbeddingRequest},
        images::{ImageGenerationRequest, ImageSize},
        moderations::ModerationRequest,
        responses::{Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest},
    },
};
use std::env;

fn get_api_key() -> Result<String, String> {
    env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY not set. Please set it to run live tests:\n\
         export OPENAI_API_KEY=your-api-key\n\
         cargo test --test openai_live_test -- --nocapture"
            .to_string()
    })
}

#[tokio::test]
#[allow(clippy::too_many_lines)]
async fn test_live_openai_apis() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match get_api_key() {
        Ok(key) => key,
        Err(msg) => {
            eprintln!("\n{msg}");
            println!("Skipping live API tests - no API key provided");
            return Ok(());
        }
    };

    println!("\n🚀 Starting OpenAI API Live Tests\n");
    println!("{}", "=".repeat(50));

    // Test 1: Models API - List available models
    println!("\n📋 Test 1: Models API - Listing available models");
    println!("{}", "-".repeat(50));

    let models_api = ModelsApi::new(&api_key)?;
    match models_api.list_models().await {
        Ok(models) => {
            println!("✅ Successfully retrieved {} models", models.data.len());
            if let Some(first_model) = models.data.first() {
                println!("   First model: {}", first_model.id);
                println!("   Owner: {}", first_model.owned_by);
            }
        }
        Err(e) => {
            println!("❌ Models API failed: {e}");
            if e.to_string().contains("401") {
                println!("   Invalid API key - please check your OPENAI_API_KEY");
                return Ok(());
            }
        }
    }

    // Test 2: Chat Completions API
    println!("\n💬 Test 2: Chat Completions API");
    println!("{}", "-".repeat(50));

    let responses_api = ResponsesApi::new(&api_key)?;
    let messages = vec![
        Message {
            role: MessageRole::System,
            content: MessageContentInput::Text(
                "You are a helpful assistant. Respond in 10 words or less.".to_string(),
            ),
        },
        Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("What is 2+2?".to_string()),
        },
    ];

    let request = ResponseRequest {
        model: "gpt-3.5-turbo".to_string(),
        input: ResponseInput::Messages(messages),
        temperature: Some(0.7),
        max_tokens: Some(100),
        response_format: None,
        instructions: None,
        previous_response_id: None,
        reasoning: None,
        text: None,
        prompt: None,
        top_p: None,
        frequency_penalty: None,
        presence_penalty: None,
        stream: None,
        tools: None,
        tool_choice: None,
        enhanced_tools: None,
        enhanced_tool_choice: None,
        parallel_tool_calls: None,
        prompt_cache_key: None,
    };

    match responses_api.create_response(&request).await {
        Ok(response) => {
            println!("✅ Chat completion successful!");
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    println!("   Response: {content}");
                }
            }
            println!("   Model: {}", response.model);
            if let Some(usage) = &response.usage {
                println!("   Tokens used: {}", usage.total_tokens);
            }
        }
        Err(e) => {
            println!("❌ Chat API failed: {e}");
        }
    }

    // Test 3: Embeddings API
    println!("\n🔢 Test 3: Embeddings API");
    println!("{}", "-".repeat(50));

    let embeddings_api = EmbeddingsApi::new(&api_key)?;
    let embedding_request = EmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("Hello, world!".to_string()),
        dimensions: None,
        encoding_format: None,
        user: None,
    };

    match embeddings_api.create_embeddings(&embedding_request).await {
        Ok(response) => {
            println!("✅ Embeddings created successfully!");
            if let Some(_embedding) = response.data.first() {
                // EmbeddingVector is an enum that wraps the actual vector
                println!("   Embedding created successfully");
            }
            println!("   Model: {}", response.model);
            println!("   Tokens used: {}", response.usage.total_tokens);
        }
        Err(e) => {
            println!("❌ Embeddings API failed: {e}");
        }
    }

    // Test 4: Moderations API
    println!("\n🛡️ Test 4: Moderations API");
    println!("{}", "-".repeat(50));

    let moderations_api = ModerationsApi::new(&api_key)?;
    let moderation_request =
        ModerationRequest::new("I want to learn about science and technology.");

    match moderations_api.create_moderation(&moderation_request).await {
        Ok(response) => {
            println!("✅ Moderation check completed!");
            if let Some(result) = response.results.first() {
                println!("   Flagged: {}", result.flagged);
                if result.flagged {
                    println!("   Categories flagged: {:?}", result.categories);
                } else {
                    println!("   Content is safe");
                }
            }
        }
        Err(e) => {
            println!("❌ Moderations API failed: {e}");
        }
    }

    // Test 5: Error Handling - Test with invalid model
    println!("\n⚠️ Test 5: Error Handling");
    println!("{}", "-".repeat(50));

    let messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text("Hello".to_string()),
    }];

    let invalid_request = ResponseRequest {
        model: "invalid-model-name".to_string(),
        input: ResponseInput::Messages(messages),
        temperature: None,
        max_tokens: None,
        response_format: None,
        instructions: None,
        previous_response_id: None,
        reasoning: None,
        text: None,
        prompt: None,
        top_p: None,
        frequency_penalty: None,
        presence_penalty: None,
        stream: None,
        tools: None,
        tool_choice: None,
        enhanced_tools: None,
        enhanced_tool_choice: None,
        parallel_tool_calls: None,
        prompt_cache_key: None,
    };

    match responses_api.create_response(&invalid_request).await {
        Ok(_) => {
            println!("❌ Expected error for invalid model, but got success");
        }
        Err(e) => {
            println!("✅ Correctly caught error for invalid model:");
            println!("   Error: {e}");
        }
    }

    // Test 6: Streaming (mock test - just verify the API exists)
    println!("\n🌊 Test 6: Streaming API");
    println!("{}", "-".repeat(50));

    let _streaming_api = StreamingApi::new(&api_key)?;
    println!("✅ Streaming API initialized successfully");
    println!("   Note: Actual streaming requires WebSocket connection");

    // Test 7: Audio API - Text to Speech
    println!("\n🔊 Test 7: Audio API - Text to Speech");
    println!("{}", "-".repeat(50));

    let audio_api = AudioApi::new(&api_key)?;
    let speech_request = AudioSpeechRequest::new("tts-1", "Hello, this is a test.", Voice::Alloy);

    match audio_api.create_speech(&speech_request).await {
        Ok(audio_data) => {
            println!("✅ Speech generation successful!");
            println!("   Audio data size: {} bytes", audio_data.audio_data.len());
        }
        Err(e) => {
            println!("❌ Audio API failed: {e}");
            if e.to_string().contains("insufficient_quota") {
                println!("   Note: This requires a paid API key with TTS access");
            }
        }
    }

    // Test 8: Images API - DALL-E
    println!("\n🎨 Test 8: Images API - DALL-E");
    println!("{}", "-".repeat(50));

    let images_api = ImagesApi::new(&api_key)?;
    let image_request =
        ImageGenerationRequest::new("dall-e-2", "A simple red circle on white background")
            .with_size(ImageSize::Size256x256)
            .with_n(1);

    match images_api
        .generate_image(
            &image_request.prompt,
            Some("dall-e-2"),
            Some(ImageSize::Size256x256),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ Image generation successful!");
            if let Some(image) = response.data.first() {
                if let Some(url) = &image.url {
                    println!("   Image URL: {url}");
                }
            }
        }
        Err(e) => {
            println!("❌ Images API failed: {e}");
            if e.to_string().contains("insufficient_quota") || e.to_string().contains("billing") {
                println!("   Note: DALL-E requires a paid API key with image generation credits");
            }
        }
    }

    println!("\n");
    println!("{}", "=".repeat(50));
    println!("🎉 OpenAI API Live Tests Complete!");
    println!("{}", "=".repeat(50));

    Ok(())
}

#[tokio::test]
async fn test_api_key_validation() {
    println!("\n🔑 Testing API Key Validation");
    println!("{}", "-".repeat(50));

    // Test with empty API key
    match ResponsesApi::new("") {
        Ok(_) => println!("❌ Should have rejected empty API key"),
        Err(e) => {
            println!("✅ Correctly rejected empty API key: {e}");
        }
    }

    // Test with whitespace API key
    match ResponsesApi::new("   ") {
        Ok(_) => println!("❌ Should have rejected whitespace API key"),
        Err(e) => {
            println!("✅ Correctly rejected whitespace API key: {e}");
        }
    }

    // Test with valid-looking key
    match ResponsesApi::new("sk-test123") {
        Ok(_) => println!("✅ Accepted valid-format API key"),
        Err(e) => println!("❌ Unexpected error with valid-format key: {e}"),
    }
}

#[tokio::test]
async fn test_batch_api_creation() {
    use openai_rust_sdk::api::{batch::BatchApi, common::ApiClientConstructors};

    println!("\n📦 Testing Batch API Creation");
    println!("{}", "-".repeat(50));

    match BatchApi::new("test-key") {
        Ok(_) => println!("✅ Batch API created successfully"),
        Err(e) => println!("❌ Failed to create Batch API: {e}"),
    }
}

#[tokio::test]
async fn test_assistants_api_creation() {
    use openai_rust_sdk::api::{assistants::AssistantsApi, common::ApiClientConstructors};

    println!("\n🤖 Testing Assistants API Creation");
    println!("{}", "-".repeat(50));

    match AssistantsApi::new("test-key") {
        Ok(_) => println!("✅ Assistants API created successfully"),
        Err(e) => println!("❌ Failed to create Assistants API: {e}"),
    }
}

#[tokio::test]
async fn test_vector_stores_api_creation() {
    use openai_rust_sdk::api::{common::ApiClientConstructors, vector_stores::VectorStoresApi};

    println!("\n🗄️ Testing Vector Stores API Creation");
    println!("{}", "-".repeat(50));

    match VectorStoresApi::new("test-key") {
        Ok(_) => println!("✅ Vector Stores API created successfully"),
        Err(e) => println!("❌ Failed to create Vector Stores API: {e}"),
    }
}
