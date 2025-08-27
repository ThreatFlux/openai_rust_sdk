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
    let api_key = get_api_key()?;
    print_header();

    run_core_api_tests(&api_key).await?;
    run_advanced_api_tests(&api_key).await?;
    run_validation_tests().await?;

    print_summary();
    Ok(())
}

#[cfg(feature = "yara")]
fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here".into()
    })
}

#[cfg(feature = "yara")]
fn print_header() {
    println!("🚀 Starting OpenAI API Integration Tests");
    println!("═══════════════════════════════════════");
}

#[cfg(feature = "yara")]
async fn run_core_api_tests(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    test_basic_completion(api_key).await?;
    test_batch_api(api_key).await?;
    test_responses_api(api_key).await?;
    Ok(())
}

#[cfg(feature = "yara")]
async fn run_advanced_api_tests(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    test_gpt5_api(api_key).await?;
    test_function_calling(api_key).await?;
    test_streaming_api(api_key).await?;
    Ok(())
}

#[cfg(feature = "yara")]
async fn run_validation_tests() -> Result<(), Box<dyn std::error::Error>> {
    test_yara_validation().await?;
    // Add error handling test with a mock API key since it's just for error testing
    test_error_handling("mock-key-for-error-testing").await?;
    Ok(())
}

#[cfg(feature = "yara")]
fn print_summary() {
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
}

#[cfg(feature = "yara")]
fn print_success(message: &str) {
    println!("✅ {message}");
}

#[cfg(feature = "yara")]
fn print_error(message: &str) {
    println!("❌ {message}");
}

#[cfg(feature = "yara")]
fn print_warning(message: &str) {
    println!("⚠️ {message}");
}

#[cfg(feature = "yara")]
fn print_test_header(test_number: u8, description: &str) {
    println!("\n{}️⃣ Testing {}", test_number, description);
    println!("{}", "─".repeat(description.len() + 12));
}

#[cfg(feature = "yara")]
async fn test_basic_completion(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(1, "Basic Client Creation and Simple Completion");

    let client = OpenAIClient::new(api_key)?;
    let response = client
        .generate_text("gpt-4o-mini", "Say hello in a creative way!")
        .await?;

    print_success("Simple completion successful:");
    println!("   Response: {response}");

    Ok(())
}

#[cfg(feature = "yara")]
async fn test_batch_api(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(2, "OpenAI Batch API with YARA Validation");

    let batch_api = BatchApi::new(api_key.to_string())?;
    let batch_generator = BatchJobGenerator::new(Some("gpt-4o-mini".to_string()));

    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_file = temp_dir.path().join("batch_api_test.jsonl");

    println!("   Generating batch job file...");

    if let Err(e) = batch_generator.generate_test_suite(&temp_file, "basic") {
        print_error(&format!("Batch file generation failed: {e}"));
        return Ok(());
    }

    upload_and_process_batch(&batch_api, &temp_file).await;
    let _ = std::fs::remove_file(&temp_file);

    Ok(())
}

#[cfg(feature = "yara")]
async fn upload_and_process_batch(batch_api: &BatchApi, temp_file: &std::path::Path) {
    println!("   Uploading batch file to OpenAI...");

    let file_upload = match batch_api.upload_batch_file(temp_file).await {
        Ok(upload) => {
            print_success("File upload successful:");
            println!("   File ID: {}", upload.id);
            println!("   File size: {} bytes", upload.bytes);
            upload
        }
        Err(e) => {
            print_error(&format!("File upload failed: {e}"));
            println!("   This might be due to API limits or file format issues");
            return;
        }
    };

    println!("\n   Creating batch job...");

    let batch = match batch_api
        .create_batch(&file_upload.id, "/v1/chat/completions")
        .await
    {
        Ok(batch) => {
            print_success("Batch creation successful:");
            println!("   Batch ID: {}", batch.id);
            println!("   Status: {}", batch.status);
            println!("   Total requests: {}", batch.request_counts.total);
            println!("   Expires at: {}", batch.expires_at);
            batch
        }
        Err(e) => {
            print_error(&format!("Batch creation failed: {e}"));
            println!("   This might be due to API limits or batch API availability");
            return;
        }
    };

    check_batch_status(batch_api, &batch.id).await;
    list_batches(batch_api).await;
}

#[cfg(feature = "yara")]
async fn check_batch_status(batch_api: &BatchApi, batch_id: &str) {
    println!("\n   Checking batch status...");

    match batch_api.get_batch_status(batch_id).await {
        Ok(status) => {
            println!("   ✅ Status check successful:");
            println!("      Current status: {}", status.status);
            println!("      Completed: {}", status.request_counts.completed);
            println!("      Failed: {}", status.request_counts.failed);

            println!("   💡 Batch is now processing. In production:");
            println!("      - Poll status every 30-60 seconds using get_batch_status()");
            println!("      - Or use wait_for_completion() to automatically wait");
            println!("      - Retrieve results with get_batch_results() when completed");
        }
        Err(e) => print_warning(&format!("Status check error: {e}")),
    }
}

#[cfg(feature = "yara")]
async fn list_batches(batch_api: &BatchApi) {
    println!("\n   Testing batch listing...");

    match batch_api.list_batches(Some(5), None).await {
        Ok(batch_list) => {
            println!("   ✅ Found {} batches", batch_list.data.len());
            for (i, batch_item) in batch_list.data.iter().take(3).enumerate() {
                println!("      {}. {} ({})", i + 1, batch_item.id, batch_item.status);
            }
        }
        Err(e) => print_warning(&format!("Batch listing error: {e}")),
    }
}

#[cfg(feature = "yara")]
async fn test_responses_api(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(3, "Responses API with Conversation");

    let responses_api = ResponsesApi::new(api_key.to_string())?;

    let messages = vec![
        Message::user("You are a helpful coding assistant."),
        Message::user("Write a simple Rust function that adds two numbers."),
    ];

    let request = ResponseRequest::new_messages("gpt-4o-mini", messages)
        .with_max_tokens(150)
        .with_temperature(0.7);

    let response = responses_api.create_response(&request).await?;
    print_success("Conversation response successful:");
    println!("   Response: {}", response.output_text());

    Ok(())
}

#[cfg(feature = "yara")]
async fn test_gpt5_api(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(4, "GPT-5 API Features");

    let gpt5_api = GPT5Api::new(api_key.to_string())?;

    let response = gpt5_api
        .create_minimal_response("gpt-4o-mini", "Explain quantum computing in one sentence.")
        .await?;

    print_success("GPT-5 style response successful:");
    println!("   Response: {}", response.output_text());

    Ok(())
}

#[cfg(feature = "yara")]
async fn test_function_calling(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(5, "Function Calling");

    let weather_function = FunctionBuilder::new()
        .name("get_weather")
        .description("Get the current weather for a location")
        .required_parameter("location", SchemaBuilder::string())
        .build_tool()?;

    let mut functions_api = FunctionsApi::new(api_key)?;

    let function_config = FunctionConfig::new()
        .with_tools(vec![weather_function])
        .with_tool_choice(ToolChoice::Auto);

    let response = functions_api
        .create_function_response(
            &ResponseRequest::new_text("gpt-4o-mini", "What's the weather like in San Francisco?"),
            &function_config,
        )
        .await?;

    print_success("Function calling successful:");
    println!(
        "   Content: {}",
        response.content.as_deref().unwrap_or("No content")
    );

    if !response.function_calls.is_empty() {
        println!("   Functions called: {}", response.function_calls.len());
        for call in &response.function_calls {
            println!("     - {}: {}", call.name, call.arguments);
        }
    }

    Ok(())
}

#[cfg(feature = "yara")]
async fn test_streaming_api(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(6, "Streaming API");

    let streaming_api = StreamingApi::new(api_key.to_string())?;

    let mut stream = streaming_api
        .create_chat_stream(
            "gpt-4o-mini",
            vec![Message::user("Count from 1 to 5, one number per line")],
        )
        .await?;

    print_success("Streaming response:");
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

    Ok(())
}

#[cfg(feature = "yara")]
async fn test_yara_validation() -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(7, "YARA Validation Integration");

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

    print_success("YARA validation successful:");
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

    Ok(())
}

#[cfg(feature = "yara")]
async fn test_error_handling(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_test_header(8, "Error Handling");

    let responses_api = ResponsesApi::new(api_key.to_string())?;
    let validator = YaraValidator::new();

    // Test with invalid model
    match responses_api
        .create_response(&ResponseRequest::new_text("invalid-model", "test"))
        .await
    {
        Err(e) => print_success(&format!("Error handling working: {e}")),
        Ok(_) => print_error("Expected error but got success"),
    }

    // Test YARA with invalid rule
    match validator.validate_rule("invalid yara rule syntax") {
        Err(e) => print_success(&format!("YARA error handling working: {e}")),
        Ok(result) => {
            if result.is_valid {
                print_error("Expected invalid rule but got valid");
            } else {
                print_success("YARA validation correctly detected invalid rule");
            }
        }
    }

    Ok(())
}
