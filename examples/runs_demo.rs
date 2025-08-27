#![allow(clippy::pedantic, clippy::nursery)]
//! # OpenAI Runs API Demo
//!
//! This example demonstrates the complete OpenAI Runs API functionality including:
//! - Creating and executing runs
//! - Handling tool calls and submitting outputs
//! - Monitoring run progress through steps
//! - Handling different run statuses
//! - Demonstrating error recovery
//! - Showing streaming updates
//! - Tracking token usage
//!
//! To run this demo:
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! cargo run --example runs_demo
//! ```

use openai_rust_sdk::api::{
    assistants::AssistantsApi, common::ApiClientConstructors, runs::RunsApi, threads::ThreadsApi,
};
use openai_rust_sdk::models::assistants::{AssistantRequest, AssistantTool};
use openai_rust_sdk::models::functions::FunctionTool;
use openai_rust_sdk::models::runs::{
    CreateThreadAndRunRequest, ListRunStepsParams, ListRunsParams, ModifyRunRequest, RunRequest,
    RunStatus, SubmitToolOutputsRequest, ThreadCreateRequest, ThreadMessage, ToolOutput,
};
use openai_rust_sdk::models::threads::{MessageRequest, MessageRole, ThreadRequest};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

/// Initialize API clients with the provided API key
fn initialize_api_clients(
    api_key: &str,
) -> Result<(RunsApi, AssistantsApi, ThreadsApi), Box<dyn std::error::Error>> {
    let runs_api = RunsApi::new(api_key)?;
    let assistants_api = AssistantsApi::new(api_key)?;
    let threads_api = ThreadsApi::new(api_key)?;
    Ok((runs_api, assistants_api, threads_api))
}

/// Setup initial resources (assistant and thread)
async fn setup_demo_resources(
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

/// Run basic demonstrations of run creation and execution
async fn run_basic_demos(
    runs_api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Demo 3: Basic run creation and execution
    println!("\n▶️ Demo 3: Creating and executing a basic run...");
    let run = basic_run_demo(runs_api, thread_id, assistant_id).await?;
    println!("✅ Basic run completed: {}", run.id);

    // Demo 4: Run with tool calls
    println!("\n🔧 Demo 4: Creating a run with tool calls...");
    tool_calling_demo(runs_api, thread_id, assistant_id).await?;

    // Demo 5: Create thread and run in one call
    println!("\n🚀 Demo 5: Creating thread and run in one call...");
    thread_and_run_demo(runs_api, assistant_id).await?;

    Ok(())
}

/// Run advanced demonstrations of run management
async fn run_advanced_demos(
    runs_api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Demo 6: List runs and run steps
    println!("\n📋 Demo 6: Listing runs and run steps...");
    list_runs_demo(runs_api, thread_id).await?;

    // Demo 7: Run modification
    println!("\n✏️ Demo 7: Modifying run metadata...");
    modify_run_demo(runs_api, thread_id, assistant_id).await?;

    // Demo 8: Run cancellation
    println!("\n❌ Demo 8: Cancelling a run...");
    cancel_run_demo(runs_api, thread_id, assistant_id).await?;

    Ok(())
}

/// Run specialized demonstrations (error handling and streaming)
async fn run_specialized_demos(
    runs_api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Demo 9: Error handling
    println!("\n🚨 Demo 9: Demonstrating error handling...");
    error_handling_demo(runs_api).await?;

    // Demo 10: Streaming runs
    println!("\n📡 Demo 10: Creating streaming runs...");
    streaming_demo(runs_api, thread_id, assistant_id).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is required");

    println!("🚀 OpenAI Runs API Demo Starting...\n");

    // Initialize API clients
    let (runs_api, assistants_api, threads_api) = initialize_api_clients(&api_key)?;

    // Setup initial resources
    let (assistant_id, thread_id) = setup_demo_resources(&assistants_api, &threads_api).await?;

    // Run demo groups
    run_basic_demos(&runs_api, &thread_id, &assistant_id).await?;
    run_advanced_demos(&runs_api, &thread_id, &assistant_id).await?;
    run_specialized_demos(&runs_api, &thread_id, &assistant_id).await?;

    // Clean up
    println!("\n🧹 Cleaning up...");
    cleanup(&assistants_api, &threads_api, &assistant_id, &thread_id).await?;

    println!("\n🎉 All demos completed successfully!");
    Ok(())
}

/// Create a demo assistant with various tools
async fn create_demo_assistant(
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
async fn create_demo_thread(
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

/// Demo basic run creation and execution
async fn basic_run_demo(
    api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<openai_rust_sdk::models::runs::Run, Box<dyn std::error::Error>> {
    let run_request = RunRequest::builder()
        .assistant_id(assistant_id)
        .instructions("Please respond to the user's message in a friendly and helpful way.")
        .metadata_pair("demo", "basic_run")
        .build()?;

    let mut run = api.create_run(thread_id, run_request).await?;
    println!("  📝 Created run: {} with status: {:?}", run.id, run.status);

    // Monitor run until completion
    run = monitor_run_to_completion(api, thread_id, &run.id).await?;

    if let Some(usage) = &run.usage {
        println!(
            "  📊 Token usage - Prompt: {}, Completion: {}, Total: {}",
            usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
        );
    }

    Ok(run)
}

/// Demo run with tool calls that require user input
async fn tool_calling_demo(
    api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut run = create_tool_calling_run(api, thread_id, assistant_id).await?;
    monitor_and_handle_tool_calls(api, thread_id, &mut run).await?;
    Ok(())
}

async fn create_tool_calling_run(
    api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<openai_rust_sdk::models::runs::Run, Box<dyn std::error::Error>> {
    let run_request = RunRequest::builder()
        .assistant_id(assistant_id)
        .instructions("The user wants to know the weather in San Francisco. Use the get_weather function to help them.")
        .metadata_pair("demo", "tool_calling")
        .build()?;

    let run = api.create_run(thread_id, run_request).await?;
    println!(
        "  📝 Created tool calling run: {} with status: {:?}",
        run.id, run.status
    );
    Ok(run)
}

async fn monitor_and_handle_tool_calls(
    api: &RunsApi,
    thread_id: &str,
    run: &mut openai_rust_sdk::models::runs::Run,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        *run = api.retrieve_run(thread_id, &run.id).await?;
        println!("  🔄 Run status: {:?}", run.status);

        match run.status {
            RunStatus::RequiresAction => {
                handle_required_action(api, thread_id, run).await?;
            }
            RunStatus::Completed => {
                println!("  ✅ Tool calling run completed successfully!");
                break;
            }
            RunStatus::Failed => {
                println!("  ❌ Run failed: {:?}", run.last_error);
                break;
            }
            RunStatus::Cancelled => {
                println!("  ❌ Run was cancelled");
                break;
            }
            RunStatus::Expired => {
                println!("  ⏰ Run expired");
                break;
            }
            _ => {
                sleep(Duration::from_millis(500)).await;
            }
        }
    }
    Ok(())
}

async fn handle_required_action(
    api: &RunsApi,
    thread_id: &str,
    run: &mut openai_rust_sdk::models::runs::Run,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(required_action) = &run.required_action {
        println!("  🔧 Run requires action: submitting tool outputs");

        let tool_outputs = process_tool_calls(&required_action.submit_tool_outputs.tool_calls);
        let request = SubmitToolOutputsRequest { tool_outputs };
        *run = api.submit_tool_outputs(thread_id, &run.id, request).await?;
        println!("  ✅ Submitted tool outputs");
    }
    Ok(())
}

fn process_tool_calls(tool_calls: &[openai_rust_sdk::models::runs::ToolCall]) -> Vec<ToolOutput> {
    tool_calls
        .iter()
        .map(|call| {
            print_tool_call_info(call);
            create_weather_tool_output(&call.id)
        })
        .collect()
}

fn print_tool_call_info(call: &openai_rust_sdk::models::runs::ToolCall) {
    println!("  📞 Tool call: {} - {}", call.id, call.call_type);
    if let Some(function) = &call.function {
        println!(
            "    Function: {} with args: {}",
            function.name, function.arguments
        );
    }
}

fn create_weather_tool_output(tool_call_id: &str) -> ToolOutput {
    ToolOutput {
        tool_call_id: tool_call_id.to_string(),
        output: json!({
            "location": "San Francisco, CA",
            "temperature": 72,
            "unit": "fahrenheit",
            "description": "Sunny and pleasant",
            "humidity": 65
        })
        .to_string(),
    }
}

/// Demo creating thread and run in a single call
async fn thread_and_run_demo(
    api: &RunsApi,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let thread = ThreadCreateRequest {
        messages: Some(vec![ThreadMessage {
            role: "user".to_string(),
            content: "Calculate the factorial of 5 and explain the steps.".to_string(),
            file_ids: None,
            metadata: None,
        }]),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("demo".to_string(), "thread_and_run".to_string());
            meta
        }),
    };

    let request = CreateThreadAndRunRequest::builder()
        .assistant_id(assistant_id)
        .thread(thread)
        .instructions("Use the code interpreter to calculate the factorial and show your work.")
        .tool(AssistantTool::CodeInterpreter)
        .metadata_pair("demo", "combined_creation")
        .build()?;

    let run = api.create_thread_and_run(request).await?;
    println!(
        "  📝 Created thread {} and run {} in one call",
        run.thread_id, run.id
    );

    // Monitor to completion
    let _final_run = monitor_run_to_completion(api, &run.thread_id, &run.id).await?;
    println!("  ✅ Thread and run demo completed!");

    Ok(())
}

/// Demo listing runs and run steps
async fn list_runs_demo(api: &RunsApi, thread_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // List runs
    let params = ListRunsParams {
        limit: Some(10),
        order: Some("desc".to_string()),
        ..Default::default()
    };

    let runs_response = api.list_runs(thread_id, Some(params)).await?;
    println!("  📋 Found {} runs in thread", runs_response.data.len());

    for (i, run) in runs_response.data.iter().enumerate() {
        println!(
            "    {}. Run {} - Status: {:?}, Created: {}",
            i + 1,
            run.id,
            run.status,
            run.created_at
        );

        // List steps for this run
        let steps_params = ListRunStepsParams {
            limit: Some(5),
            ..Default::default()
        };

        let steps_response = api
            .list_run_steps(thread_id, &run.id, Some(steps_params))
            .await?;
        println!("       📝 {} steps found", steps_response.data.len());

        for (j, step) in steps_response.data.iter().enumerate() {
            println!(
                "         {}. Step {} - Type: {}, Status: {:?}",
                j + 1,
                step.id,
                step.step_type,
                step.status
            );
        }
    }

    Ok(())
}

/// Demo modifying run metadata
async fn modify_run_demo(
    api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a run first
    let run_request = RunRequest::builder()
        .assistant_id(assistant_id)
        .instructions("Just say hello briefly.")
        .metadata_pair("status", "initial")
        .build()?;

    let run = api.create_run(thread_id, run_request).await?;
    println!("  📝 Created run for modification: {}", run.id);

    // Modify the run metadata
    let mut new_metadata = HashMap::new();
    new_metadata.insert("status".to_string(), "modified".to_string());
    new_metadata.insert("priority".to_string(), "high".to_string());
    new_metadata.insert("updated_at".to_string(), chrono::Utc::now().to_rfc3339());

    let modify_request = ModifyRunRequest {
        metadata: Some(new_metadata),
    };

    let updated_run = api.modify_run(thread_id, &run.id, modify_request).await?;
    println!("  ✏️ Modified run metadata");
    println!("    Original metadata: {:?}", run.metadata);
    println!("    Updated metadata: {:?}", updated_run.metadata);

    Ok(())
}

/// Demo run cancellation
async fn cancel_run_demo(
    api: &RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a run that might take some time
    let run_request = RunRequest::builder()
        .assistant_id(assistant_id)
        .instructions("Write a very long story about space exploration with many details.")
        .metadata_pair("demo", "cancellation_test")
        .build()?;

    let run = api.create_run(thread_id, run_request).await?;
    println!("  📝 Created run for cancellation: {}", run.id);

    // Wait a bit to let it start
    sleep(Duration::from_millis(100)).await;

    // Cancel the run
    let cancelled_run = api.cancel_run(thread_id, &run.id).await?;
    println!(
        "  ❌ Cancelled run: {} - Status: {:?}",
        cancelled_run.id, cancelled_run.status
    );

    // Monitor until fully cancelled
    let mut current_run = cancelled_run;
    for _ in 0..10 {
        if matches!(
            current_run.status,
            RunStatus::Cancelled | RunStatus::Completed | RunStatus::Failed
        ) {
            break;
        }
        sleep(Duration::from_millis(500)).await;
        current_run = api.retrieve_run(thread_id, &run.id).await?;
        println!("  🔄 Cancellation status: {:?}", current_run.status);
    }

    Ok(())
}

/// Demo error handling scenarios
async fn error_handling_demo(api: &RunsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("  🚨 Testing various error scenarios...");

    // Test with invalid thread ID
    println!("    Testing invalid thread ID...");
    let run_request = RunRequest::builder().assistant_id("asst_invalid").build()?;

    match api.create_run("thread_invalid", run_request).await {
        Ok(_) => println!("    ⚠️ Unexpected success with invalid thread ID"),
        Err(e) => println!("    ✅ Expected error with invalid thread ID: {e}"),
    }

    // Test retrieving non-existent run
    println!("    Testing non-existent run retrieval...");
    match api.retrieve_run("thread_invalid", "run_invalid").await {
        Ok(_) => println!("    ⚠️ Unexpected success with invalid run ID"),
        Err(e) => println!("    ✅ Expected error with invalid run ID: {e}"),
    }

    // Test submitting tool outputs to non-existent run
    println!("    Testing tool outputs to non-existent run...");
    let tool_outputs = vec![ToolOutput {
        tool_call_id: "call_invalid".to_string(),
        output: "test".to_string(),
    }];
    let request = SubmitToolOutputsRequest { tool_outputs };

    match api
        .submit_tool_outputs("thread_invalid", "run_invalid", request)
        .await
    {
        Ok(_) => println!("    ⚠️ Unexpected success with invalid run"),
        Err(e) => println!("    ✅ Expected error with invalid run: {e}"),
    }

    Ok(())
}

/// Demo streaming functionality
async fn streaming_demo(
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

/// Helper function to monitor a run until completion
async fn monitor_run_to_completion(
    api: &RunsApi,
    thread_id: &str,
    run_id: &str,
) -> Result<openai_rust_sdk::models::runs::Run, Box<dyn std::error::Error>> {
    let mut run = api.retrieve_run(thread_id, run_id).await?;
    let mut iterations = 0;
    const MAX_ITERATIONS: u32 = 60; // 30 seconds max

    while !matches!(
        run.status,
        RunStatus::Completed | RunStatus::Failed | RunStatus::Cancelled | RunStatus::Expired
    ) {
        if iterations >= MAX_ITERATIONS {
            println!("  ⏰ Timeout waiting for run completion");
            break;
        }

        sleep(Duration::from_millis(500)).await;
        run = api.retrieve_run(thread_id, run_id).await?;
        iterations += 1;

        if iterations % 4 == 0 {
            // Print status every 2 seconds
            println!(
                "  🔄 Run status: {:?} ({}s elapsed)",
                run.status,
                iterations / 2
            );
        }
    }

    println!("  🏁 Final run status: {:?}", run.status);
    Ok(run)
}

/// Clean up created resources
async fn cleanup(
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
