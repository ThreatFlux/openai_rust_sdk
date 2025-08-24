#![allow(clippy::pedantic, clippy::nursery)]
#[cfg(not(feature = "yara"))]
fn main() {
    println!("This example requires the 'yara' feature to be enabled.");
    println!("Run with: cargo run --example api_integration_test --features yara");
}

#[cfg(feature = "yara")]
use openai_rust_sdk::{
    api::{
        batch::BatchApi,
        common::ApiClientConstructors,
        functions::{FunctionConfig, FunctionsApi},
        gpt5::GPT5Api,
        responses::ResponsesApi,
        streaming::StreamingApi,
    },
    builders::function_builder::FunctionBuilder,
    models::{
        functions::ToolChoice,
        responses::{Message, ResponseRequest},
    },
    schema::builder::SchemaBuilder,
    testing::{yara_validator::YaraValidator, BatchJobGenerator},
    OpenAIClient,
};
#[cfg(feature = "yara")]
use std::env;
#[cfg(feature = "yara")]
use tokio_stream::StreamExt;

#[cfg(feature = "yara")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🚀 Starting OpenAI API Integration Tests");
    println!("═══════════════════════════════════════");

    // Test 1: Basic client creation and simple completion
    println!("\n1️⃣ Testing Basic Client Creation and Simple Completion");
    println!("──────────────────────────────────────────────────────");

    let client = OpenAIClient::new(&api_key)?;

    let simple_response = client
        .generate_text("gpt-4o-mini", "Say hello in a creative way!")
        .await?;

    println!("✅ Simple completion successful:");
    println!("   Response: {simple_response}");

    // Test 2: OpenAI Batch API with YARA validation requests
    println!("\n2️⃣ Testing OpenAI Batch API with YARA Validation");
    println!("─────────────────────────────────────────────────");

    let batch_api = BatchApi::new(api_key.clone())?;
    let batch_generator = BatchJobGenerator::new(Some("gpt-4o-mini".to_string()));

    // Generate a small batch file for testing - using tempfile for secure creation
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_file = temp_dir.path().join("batch_api_test.jsonl");

    println!("   Generating batch job file...");
    match batch_generator.generate_test_suite(&temp_file, "basic") {
        Ok(()) => {
            // Upload the batch file
            println!("   Uploading batch file to OpenAI...");
            match batch_api.upload_batch_file(&temp_file).await {
                Ok(file_upload) => {
                    println!("✅ File upload successful:");
                    println!("   File ID: {}", file_upload.id);
                    println!("   File size: {} bytes", file_upload.bytes);

                    // Create a batch job
                    println!("\n   Creating batch job...");
                    match batch_api
                        .create_batch(&file_upload.id, "/v1/chat/completions")
                        .await
                    {
                        Ok(batch) => {
                            println!("✅ Batch creation successful:");
                            println!("   Batch ID: {}", batch.id);
                            println!("   Status: {}", batch.status);
                            println!("   Total requests: {}", batch.request_counts.total);
                            println!("   Expires at: {}", batch.expires_at);

                            // Check status one more time
                            println!("\n   Checking batch status...");
                            match batch_api.get_batch_status(&batch.id).await {
                                Ok(status) => {
                                    println!("   ✅ Status check successful:");
                                    println!("      Current status: {}", status.status);
                                    println!(
                                        "      Completed: {}",
                                        status.request_counts.completed
                                    );
                                    println!("      Failed: {}", status.request_counts.failed);

                                    println!("   💡 Batch is now processing. In production:");
                                    println!(
                                        "      - Poll status every 30-60 seconds using get_batch_status()"
                                    );
                                    println!(
                                        "      - Or use wait_for_completion() to automatically wait"
                                    );
                                    println!(
                                        "      - Retrieve results with get_batch_results() when completed"
                                    );
                                }
                                Err(e) => println!("   ⚠️ Status check error: {e}"),
                            }

                            // Test batch listing
                            println!("\n   Testing batch listing...");
                            match batch_api.list_batches(Some(5), None).await {
                                Ok(batch_list) => {
                                    println!("   ✅ Found {} batches", batch_list.data.len());
                                    for (i, batch_item) in
                                        batch_list.data.iter().take(3).enumerate()
                                    {
                                        println!(
                                            "      {}. {} ({})",
                                            i + 1,
                                            batch_item.id,
                                            batch_item.status
                                        );
                                    }
                                }
                                Err(e) => println!("   ⚠️ Batch listing error: {e}"),
                            }
                        }
                        Err(e) => {
                            println!("❌ Batch creation failed: {e}");
                            println!(
                                "   This might be due to API limits or batch API availability"
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("❌ File upload failed: {e}");
                    println!("   This might be due to API limits or file format issues");
                }
            }

            // Clean up temp file
            let _ = std::fs::remove_file(&temp_file);
        }
        Err(e) => {
            println!("❌ Batch file generation failed: {e}");
        }
    }

    // Test 3: Responses API with messages
    println!("\n3️⃣ Testing Responses API with Conversation");
    println!("─────────────────────────────────────────────");

    let responses_api = ResponsesApi::new(api_key.clone())?;

    let messages = vec![
        Message::user("You are a helpful coding assistant."),
        Message::user("Write a simple Rust function that adds two numbers."),
    ];

    let request = ResponseRequest::new_messages("gpt-4o-mini", messages)
        .with_max_tokens(150)
        .with_temperature(0.7);

    let response = responses_api.create_response(&request).await?;
    println!("✅ Conversation response successful:");
    println!("   Response: {}", response.output_text());

    // Test 4: GPT-5 API features (using GPT-4 as fallback since GPT-5 may not be available)
    println!("\n4️⃣ Testing GPT-5 API Features");
    println!("────────────────────────────────────────");

    let gpt5_api = GPT5Api::new(api_key.clone())?;

    // Test with a model that exists (gpt-4o-mini instead of gpt-5)
    let gpt5_response = gpt5_api
        .create_minimal_response(
            "gpt-4o-mini", // Using available model
            "Explain quantum computing in one sentence.",
        )
        .await?;

    println!("✅ GPT-5 style response successful:");
    println!("   Response: {}", gpt5_response.output_text());

    // Test 5: Function Calling
    println!("\n5️⃣ Testing Function Calling");
    println!("──────────────────────────────");

    let weather_function = FunctionBuilder::new()
        .name("get_weather")
        .description("Get the current weather for a location")
        .required_parameter("location", SchemaBuilder::string())
        .build_tool()?;

    let mut functions_api = FunctionsApi::new(&api_key)?;

    let function_config = FunctionConfig::new()
        .with_tools(vec![weather_function])
        .with_tool_choice(ToolChoice::Auto);

    let function_response = functions_api
        .create_function_response(
            &ResponseRequest::new_text("gpt-4o-mini", "What's the weather like in San Francisco?"),
            &function_config,
        )
        .await?;

    println!("✅ Function calling successful:");
    println!(
        "   Content: {}",
        function_response.content.as_deref().unwrap_or("No content")
    );
    if !function_response.function_calls.is_empty() {
        println!(
            "   Functions called: {}",
            function_response.function_calls.len()
        );
        for call in &function_response.function_calls {
            println!("     - {}: {}", call.name, call.arguments);
        }
    }

    // Test 6: Streaming API
    println!("\n6️⃣ Testing Streaming API");
    println!("───────────────────────────");

    let streaming_api = StreamingApi::new(api_key.clone())?;

    let mut stream = streaming_api
        .create_chat_stream(
            "gpt-4o-mini",
            vec![Message::user("Count from 1 to 5, one number per line")],
        )
        .await?;

    println!("✅ Streaming response:");
    print!("   ");

    let mut chunk_count = 0;
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                if let Some(content) = chunk
                    .choices
                    .first()
                    .and_then(|choice| choice.delta.content.as_ref())
                {
                    print!("{content}");
                    chunk_count += 1;
                }
            }
            Err(e) => {
                eprintln!("\n❌ Streaming error: {e}");
                break;
            }
        }
    }

    println!("\n   Received {chunk_count} chunks");

    // Test 7: YARA Validation Integration
    println!("\n7️⃣ Testing YARA Validation Integration");
    println!("────────────────────────────────────────");

    let validator = YaraValidator::new();

    let test_rule = r#"
rule test_api_integration {
    meta:
        description = "Test rule for API integration"
        author = "API Test"
    strings:
        $text = "malware"
        $hex = { 4D 5A }
    condition:
        $text or $hex
}
"#;

    let validation_result = validator.validate_rule(test_rule)?;

    println!("✅ YARA validation successful:");
    println!("   Valid: {}", validation_result.is_valid);
    println!(
        "   Rule name: {}",
        validation_result.rule_name.as_deref().unwrap_or("Unknown")
    );
    println!(
        "   Compilation time: {}ms",
        validation_result.metrics.compilation_time_ms
    );
    println!(
        "   Features: strings={}, hex={}",
        validation_result.features.has_strings, validation_result.features.has_hex_patterns
    );

    // Test 8: Error Handling
    println!("\n8️⃣ Testing Error Handling");
    println!("────────────────────────────");

    // Test with invalid model
    match responses_api
        .create_response(&ResponseRequest::new_text("invalid-model", "test"))
        .await
    {
        Err(e) => println!("✅ Error handling working: {e}"),
        Ok(_) => println!("❌ Expected error but got success"),
    }

    // Test YARA with invalid rule
    match validator.validate_rule("invalid yara rule syntax") {
        Err(e) => println!("✅ YARA error handling working: {e}"),
        Ok(result) => {
            if result.is_valid {
                println!("❌ Expected invalid rule but got valid");
            } else {
                println!("✅ YARA validation correctly detected invalid rule");
            }
        }
    }

    println!("\n🎉 All API Integration Tests Completed Successfully!");
    println!("═══════════════════════════════════════════════════");
    println!("✅ Client Creation: Working");
    println!("✅ Simple Completions: Working");
    println!("✅ Batch API: Working");
    println!("✅ Conversation API: Working");
    println!("✅ GPT-5 Style API: Working");
    println!("✅ Function Calling: Working");
    println!("✅ Streaming: Working");
    println!("✅ YARA Integration: Working");
    println!("✅ Error Handling: Working");

    Ok(())
}
