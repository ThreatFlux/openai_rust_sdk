//! Streaming demonstrations for the runs demo

use crate::utilities::monitor_run_to_completion;
use openai_rust_sdk::api::runs::RunsApi;
use openai_rust_sdk::models::assistants::AssistantTool;
use openai_rust_sdk::models::runs::{
    CreateThreadAndRunRequest, RunRequest, ThreadCreateRequest, ThreadMessage,
};

/// Demo streaming functionality
pub async fn streaming_demo(
    api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📡 Testing streaming run creation...");

    let run_request = RunRequest::builder()
        .assistant_id(assistant_id)
        .instructions("Count from 1 to 5 and explain each number briefly.")
        .metadata_pair("demo", "streaming")
        .build()?;

    // Create run with streaming
    let run = api.create_run_stream(thread_id, run_request).await?;
    println!("  🚀 Created streaming run: {}", run.id);

    // Monitor the streaming run
    let final_run = monitor_run_to_completion(api, thread_id, &run.id).await?;
    println!("  ✅ Streaming run completed: {}", final_run.id);

    // Test streaming with thread and run creation
    println!("  📡 Testing streaming thread and run creation...");

    let thread = ThreadCreateRequest {
        messages: Some(vec![ThreadMessage {
            role: "user".to_string(),
            content: "What's 2 + 2? Please show your calculation.".to_string(),
            file_ids: None,
            metadata: None,
        }]),
        metadata: None,
    };

    let stream_request = CreateThreadAndRunRequest::builder()
        .assistant_id(assistant_id)
        .thread(thread)
        .instructions("Show your mathematical reasoning step by step.")
        .tool(AssistantTool::CodeInterpreter)
        .build()?;

    let stream_run = api.create_thread_and_run_stream(stream_request).await?;
    println!("  🚀 Created streaming thread and run: {}", stream_run.id);

    let final_stream_run =
        monitor_run_to_completion(api, &stream_run.thread_id, &stream_run.id).await?;
    println!(
        "  ✅ Streaming thread and run completed: {}",
        final_stream_run.id
    );

    Ok(())
}
