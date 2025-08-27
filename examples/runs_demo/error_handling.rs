//! Error handling demonstrations for the runs demo

use openai_rust_sdk::api::runs::RunsApi;
use openai_rust_sdk::models::runs::{RunRequest, SubmitToolOutputsRequest, ToolOutput};

/// Demo error handling scenarios
pub async fn error_handling_demo(api: &RunsApi) -> Result<(), Box<dyn std::error::Error>> {
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
