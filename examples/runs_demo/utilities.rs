//! Utility functions for the runs demo

use openai_rust_sdk::api::runs::RunsApi;
use openai_rust_sdk::models::runs::{RunStatus, ToolOutput};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

/// Helper function to monitor a run until completion
pub async fn monitor_run_to_completion(
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

/// Process tool calls and generate responses
pub fn process_tool_calls(
    tool_calls: &[openai_rust_sdk::models::runs::ToolCall],
) -> Vec<ToolOutput> {
    tool_calls
        .iter()
        .map(|call| {
            print_tool_call_info(call);
            create_weather_tool_output(&call.id)
        })
        .collect()
}

/// Print information about a tool call
fn print_tool_call_info(call: &openai_rust_sdk::models::runs::ToolCall) {
    println!("  📞 Tool call: {} - {}", call.id, call.call_type);
    if let Some(function) = &call.function {
        println!(
            "    Function: {} with args: {}",
            function.name, function.arguments
        );
    }
}

/// Create a mock weather tool output
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
