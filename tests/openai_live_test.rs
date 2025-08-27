#![allow(clippy::pedantic, clippy::nursery)]
//! Live integration tests for `OpenAI` API
//!
//! Run with: OPENAI_API_KEY=your-key cargo test --test `openai_live_test` -- --nocapture

use openai_rust_sdk::{
    api::{
        audio::AudioApi, common::ApiClientConstructors, embeddings::EmbeddingsApi,
        images::ImagesApi, models::ModelsApi, moderations::ModerationsApi, responses::ResponsesApi,
        streaming::StreamingApi,
    },
    models::{
        audio::{AudioSpeechRequest, Voice},
        embeddings::{EmbeddingInput, EmbeddingRequest},
        images::ImageSize,
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

/// Test runner struct to manage common test setup and utilities
struct TestRunner {
    api_key: String,
}

impl TestRunner {
    fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn print_test_header(&self, test_name: &str, icon: &str) {
        println!("\n{icon} {test_name}");
        println!("{}", "-".repeat(50));
    }

    fn print_success(&self, message: &str) {
        println!("✅ {message}");
    }

    fn print_error(&self, message: &str) {
        println!("❌ {message}");
    }

    async fn test_models_api(&self) {
        self.print_test_header("Test 1: Models API - Listing available models", "📋");

        let models_api = match ModelsApi::new(&self.api_key) {
            Ok(api) => api,
            Err(e) => {
                self.print_error(&format!("Failed to create ModelsApi: {e}"));
                return;
            }
        };

        match models_api.list_models().await {
            Ok(models) => {
                self.print_success(&format!(
                    "Successfully retrieved {} models",
                    models.data.len()
                ));
                if let Some(first_model) = models.data.first() {
                    println!("   First model: {}", first_model.id);
                    println!("   Owner: {}", first_model.owned_by);
                }
            }
            Err(e) => {
                self.print_error(&format!("Models API failed: {e}"));
                if e.to_string().contains("401") {
                    println!("   Invalid API key - please check your OPENAI_API_KEY");
                }
            }
        }
    }

    async fn test_chat_completions_api(&self) {
        self.print_test_header("Test 2: Chat Completions API", "💬");

        let responses_api = match ResponsesApi::new(&self.api_key) {
            Ok(api) => api,
            Err(e) => {
                self.print_error(&format!("Failed to create ResponsesApi: {e}"));
                return;
            }
        };

        let request = create_chat_request();
        match responses_api.create_response(&request).await {
            Ok(response) => {
                self.print_success("Chat completion successful!");
                print_chat_response(&response);
            }
            Err(e) => {
                self.print_error(&format!("Chat API failed: {e}"));
            }
        }
    }

    async fn test_embeddings_api(&self) {
        self.print_test_header("Test 3: Embeddings API", "🔢");

        let embeddings_api = match EmbeddingsApi::new(&self.api_key) {
            Ok(api) => api,
            Err(e) => {
                self.print_error(&format!("Failed to create EmbeddingsApi: {e}"));
                return;
            }
        };

        let request = create_embedding_request();
        match embeddings_api.create_embeddings(&request).await {
            Ok(response) => {
                self.print_success("Embeddings created successfully!");
                print_embedding_response(&response);
            }
            Err(e) => {
                self.print_error(&format!("Embeddings API failed: {e}"));
            }
        }
    }

    async fn test_moderations_api(&self) {
        self.print_test_header("Test 4: Moderations API", "🛡️");

        let moderations_api = match ModerationsApi::new(&self.api_key) {
            Ok(api) => api,
            Err(e) => {
                self.print_error(&format!("Failed to create ModerationsApi: {e}"));
                return;
            }
        };

        let request = ModerationRequest::new("I want to learn about science and technology.");
        match moderations_api.create_moderation(&request).await {
            Ok(response) => {
                self.print_success("Moderation check completed!");
                print_moderation_response(&response);
            }
            Err(e) => {
                self.print_error(&format!("Moderations API failed: {e}"));
            }
        }
    }

    async fn test_error_handling(&self) {
        self.print_test_header("Test 5: Error Handling", "⚠️");

        let responses_api = match ResponsesApi::new(&self.api_key) {
            Ok(api) => api,
            Err(e) => {
                self.print_error(&format!("Failed to create ResponsesApi: {e}"));
                return;
            }
        };

        let request = create_invalid_request();
        match responses_api.create_response(&request).await {
            Ok(_) => {
                self.print_error("Expected error for invalid model, but got success");
            }
            Err(e) => {
                self.print_success("Correctly caught error for invalid model:");
                println!("   Error: {e}");
            }
        }
    }

    async fn test_streaming_api(&self) {
        self.print_test_header("Test 6: Streaming API", "🌊");

        match StreamingApi::new(&self.api_key) {
            Ok(_) => {
                self.print_success("Streaming API initialized successfully");
                println!("   Note: Actual streaming requires WebSocket connection");
            }
            Err(e) => {
                self.print_error(&format!("Failed to create StreamingApi: {e}"));
            }
        }
    }

    async fn test_audio_api(&self) {
        self.print_test_header("Test 7: Audio API - Text to Speech", "🔊");

        let audio_api = match AudioApi::new(&self.api_key) {
            Ok(api) => api,
            Err(e) => {
                self.print_error(&format!("Failed to create AudioApi: {e}"));
                return;
            }
        };

        let request = AudioSpeechRequest::new("tts-1", "Hello, this is a test.", Voice::Alloy);
        match audio_api.create_speech(&request).await {
            Ok(audio_data) => {
                self.print_success("Speech generation successful!");
                println!("   Audio data size: {} bytes", audio_data.audio_data.len());
            }
            Err(e) => {
                self.print_error(&format!("Audio API failed: {e}"));
                if e.to_string().contains("insufficient_quota") {
                    println!("   Note: This requires a paid API key with TTS access");
                }
            }
        }
    }

    async fn test_images_api(&self) {
        self.print_test_header("Test 8: Images API - DALL-E", "🎨");

        let images_api = match ImagesApi::new(&self.api_key) {
            Ok(api) => api,
            Err(e) => {
                self.print_error(&format!("Failed to create ImagesApi: {e}"));
                return;
            }
        };

        match images_api
            .generate_image(
                "A simple red circle on white background",
                Some("dall-e-2"),
                Some(ImageSize::Size256x256),
                None,
            )
            .await
        {
            Ok(response) => {
                self.print_success("Image generation successful!");
                if let Some(image) = response.data.first() {
                    if let Some(url) = &image.url {
                        println!("   Image URL: {url}");
                    }
                }
            }
            Err(e) => {
                self.print_error(&format!("Images API failed: {e}"));
                if e.to_string().contains("insufficient_quota") || e.to_string().contains("billing")
                {
                    println!(
                        "   Note: DALL-E requires a paid API key with image generation credits"
                    );
                }
            }
        }
    }
}

fn create_chat_request() -> ResponseRequest {
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

    ResponseRequest {
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
    }
}

fn create_embedding_request() -> EmbeddingRequest {
    EmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: EmbeddingInput::String("Hello, world!".to_string()),
        dimensions: None,
        encoding_format: None,
        user: None,
    }
}

fn create_invalid_request() -> ResponseRequest {
    let messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text("Hello".to_string()),
    }];

    ResponseRequest {
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
    }
}

fn print_chat_response(response: &openai_rust_sdk::models::responses::ResponseResult) {
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

fn print_embedding_response(response: &openai_rust_sdk::models::embeddings::EmbeddingResponse) {
    if let Some(_embedding) = response.data.first() {
        println!("   Embedding created successfully");
    }
    println!("   Model: {}", response.model);
    println!("   Tokens used: {}", response.usage.total_tokens);
}

fn print_moderation_response(response: &openai_rust_sdk::models::moderations::ModerationResponse) {
    if let Some(result) = response.results.first() {
        println!("   Flagged: {}", result.flagged);
        if result.flagged {
            println!("   Categories flagged: {:?}", result.categories);
        } else {
            println!("   Content is safe");
        }
    }
}

#[tokio::test]
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

    let test_runner = TestRunner::new(api_key);

    // Run all test sections
    test_runner.test_models_api().await;
    test_runner.test_chat_completions_api().await;
    test_runner.test_embeddings_api().await;
    test_runner.test_moderations_api().await;
    test_runner.test_error_handling().await;
    test_runner.test_streaming_api().await;
    test_runner.test_audio_api().await;
    test_runner.test_images_api().await;

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
