//! Setup and initialization functions for the runs demo

use openai_rust_sdk::api::{
    assistants::AssistantsApi, common::ApiClientConstructors, runs::RunsApi, threads::ThreadsApi,
};
use openai_rust_sdk::models::assistants::{AssistantRequest, AssistantTool};
use openai_rust_sdk::models::functions::FunctionTool;
use openai_rust_sdk::models::threads::{MessageRequest, MessageRole, ThreadRequest};
use serde_json::json;

/// Initialize API clients with the provided API key
pub fn initialize_api_clients(
    api_key: &str,
) -> Result<(RunsApi, AssistantsApi, ThreadsApi), Box<dyn std::error::Error>> {
    let runs_api = RunsApi::new(api_key)?;
    let assistants_api = AssistantsApi::new(api_key)?;
    let threads_api = ThreadsApi::new(api_key)?;
    Ok((runs_api, assistants_api, threads_api))
}

/// Setup initial resources (assistant and thread)
pub async fn setup_demo_resources(
    assistants_api: &AssistantsApi,
    threads_api: &ThreadsApi,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Demo 1: Create an assistant with tools
    println!("📋 Demo 1: Creating an assistant with tools...");
    let assistant = create_demo_assistant(assistants_api).await?;
    println!(
        "✅ Created assistant: {} ({})",
        assistant.name.unwrap_or_else(|| "Unnamed".to_string()),
        assistant.id
    );

    // Demo 2: Create a thread
    println!("\n🧵 Demo 2: Creating a thread...");
    let thread = create_demo_thread(threads_api).await?;
    println!("✅ Created thread: {}", thread.id);

    Ok((assistant.id, thread.id))
}

/// Create a demo assistant with various tools
pub async fn create_demo_assistant(
    api: &AssistantsApi,
) -> Result<openai_rust_sdk::models::assistants::Assistant, Box<dyn std::error::Error>> {
    // Create a custom function tool
    let weather_function = FunctionTool {
        name: "get_weather".to_string(),
        description: "Get the current weather for a location".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "The temperature unit"
                }
            },
            "required": ["location"]
        }),
        strict: Some(true),
    };

    let assistant_request = AssistantRequest::builder()
        .name("Runs Demo Assistant")
        .description("A demo assistant for testing the Runs API")
        .model("gpt-4")
        .instructions("You are a helpful assistant that can analyze data, write code, and get weather information. Always be concise and helpful.")
        .tool(AssistantTool::CodeInterpreter)
        .tool(AssistantTool::Function { function: weather_function })
        .metadata_pair("demo", "true")
        .metadata_pair("purpose", "runs_api_testing")
        .build()?;

    let assistant = api.create_assistant(assistant_request).await?;
    Ok(assistant)
}

/// Create a demo thread with initial messages
pub async fn create_demo_thread(
    api: &ThreadsApi,
) -> Result<openai_rust_sdk::models::threads::Thread, Box<dyn std::error::Error>> {
    let thread_request = ThreadRequest::builder()
        .metadata_pair("demo", "true")
        .metadata_pair("purpose", "runs_demo")
        .build();

    let thread = api.create_thread(thread_request).await?;

    // Add an initial message
    let message_request = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Hello! I'm testing the Runs API. Please help me understand how it works.")
        .build()?;

    api.create_message(&thread.id, message_request).await?;

    Ok(thread)
}

/// Clean up created resources
pub async fn cleanup(
    assistants_api: &AssistantsApi,
    threads_api: &ThreadsApi,
    assistant_id: &str,
    thread_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Delete thread
    match threads_api.delete_thread(thread_id).await {
        Ok(deleted) => {
            if deleted.deleted {
                println!("  ✅ Deleted thread: {thread_id}");
            }
        }
        Err(e) => println!("  ⚠️ Failed to delete thread: {e}"),
    }

    // Delete assistant
    match assistants_api.delete_assistant(assistant_id).await {
        Ok(deleted) => {
            if deleted.deleted {
                println!("  ✅ Deleted assistant: {assistant_id}");
            }
        }
        Err(e) => println!("  ⚠️ Failed to delete assistant: {e}"),
    }

    Ok(())
}
