//! Core demonstration functions for basic and advanced run operations

use openai_rust_sdk::api::runs::RunsApi;
use openai_rust_sdk::models::assistants::AssistantTool;
use openai_rust_sdk::models::runs::{
    CreateThreadAndRunRequest, ListRunStepsParams, ListRunsParams, ModifyRunRequest, RunRequest,
    RunStatus, SubmitToolOutputsRequest, ThreadCreateRequest, ThreadMessage,
};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use crate::utilities::{monitor_run_to_completion, process_tool_calls};

/// Run basic demonstrations of run creation and execution
pub async fn run_basic_demos(
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
pub async fn run_advanced_demos(
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
