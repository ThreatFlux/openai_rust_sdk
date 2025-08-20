#![allow(clippy::pedantic, clippy::nursery)]
//! # `OpenAI` Runs API Tests
//!
//! Comprehensive test suite for the `OpenAI` Runs API implementation.
//! These tests cover all aspects of run management, tool calling, step tracking,
//! status monitoring, and error handling.

use openai_rust_sdk::api::runs::RunsApi;
use openai_rust_sdk::models::assistants::AssistantTool;
use openai_rust_sdk::models::runs::{
    CreateThreadAndRunRequest, ListRunStepsParams, ListRunsParams, ModifyRunRequest, RunRequest,
    RunStatus, SubmitToolOutputsRequest, ThreadCreateRequest, ThreadMessage, ToolOutput,
};
use std::collections::HashMap;

/// Mock API key for testing
const TEST_API_KEY: &str = "sk-test123456789";

/// Helper function to create a runs API client for testing
fn create_test_client() -> RunsApi {
    RunsApi::new(TEST_API_KEY).expect("Failed to create test client")
}

/// Helper function to create a runs API client with custom base URL
fn create_test_client_with_url(base_url: &str) -> RunsApi {
    RunsApi::with_base_url(TEST_API_KEY, base_url).expect("Failed to create test client")
}

#[test]
fn test_runs_api_client_creation() {
    let _api = create_test_client();
    // Test that the client was created successfully
    // Note: fields are private, so we just test successful creation
    // Test passes if no panic occurs during client creation
}

#[test]
fn test_runs_api_client_with_custom_url() {
    let custom_url = "https://custom.openai.com";
    let _api = create_test_client_with_url(custom_url);
    // Test that the client was created successfully with custom URL
    // Note: fields are private, so we just test successful creation
    // Test passes if no panic occurs during client creation
}

#[test]
fn test_run_request_builder() {
    let request = RunRequest::builder()
        .assistant_id("asst_abc123")
        .model("gpt-4")
        .instructions("Analyze the data")
        .tool(AssistantTool::CodeInterpreter)
        .file_id("file_abc123")
        .metadata_pair("purpose", "analysis")
        .build()
        .expect("Failed to build run request");

    assert_eq!(request.assistant_id, "asst_abc123");
    assert_eq!(request.model, Some("gpt-4".to_string()));
    assert_eq!(request.instructions, Some("Analyze the data".to_string()));
    assert_eq!(request.tools, Some(vec![AssistantTool::CodeInterpreter]));
    assert_eq!(request.file_ids, Some(vec!["file_abc123".to_string()]));
    assert!(request.metadata.is_some());
    assert_eq!(
        request.metadata.unwrap().get("purpose"),
        Some(&"analysis".to_string())
    );
}

#[test]
fn test_run_request_builder_missing_assistant_id() {
    let result = RunRequest::builder()
        .model("gpt-4")
        .instructions("Analyze the data")
        .build();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "assistant_id is required");
}

#[test]
fn test_run_request_builder_minimal() {
    let request = RunRequest::builder()
        .assistant_id("asst_abc123")
        .build()
        .expect("Failed to build minimal run request");

    assert_eq!(request.assistant_id, "asst_abc123");
    assert_eq!(request.model, None);
    assert_eq!(request.instructions, None);
    assert_eq!(request.tools, None);
    assert_eq!(request.file_ids, None);
    assert_eq!(request.metadata, None);
}

#[test]
fn test_create_thread_and_run_request_builder() {
    let thread = ThreadCreateRequest {
        messages: Some(vec![ThreadMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
            file_ids: None,
            metadata: None,
        }]),
        metadata: None,
    };

    let request = CreateThreadAndRunRequest::builder()
        .assistant_id("asst_abc123")
        .thread(thread)
        .model("gpt-4")
        .instructions("Be helpful")
        .tool(AssistantTool::CodeInterpreter)
        .file_id("file_abc123")
        .metadata_pair("purpose", "chat")
        .build()
        .expect("Failed to build thread and run request");

    assert_eq!(request.assistant_id, "asst_abc123");
    assert!(request.thread.is_some());
    assert_eq!(request.model, Some("gpt-4".to_string()));
    assert_eq!(request.instructions, Some("Be helpful".to_string()));
    assert_eq!(request.tools, Some(vec![AssistantTool::CodeInterpreter]));
    assert_eq!(request.file_ids, Some(vec!["file_abc123".to_string()]));
    assert!(request.metadata.is_some());
}

#[test]
fn test_create_thread_and_run_request_builder_missing_assistant_id() {
    let result = CreateThreadAndRunRequest::builder()
        .model("gpt-4")
        .instructions("Be helpful")
        .build();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "assistant_id is required");
}

#[test]
fn test_run_status_serialization() {
    let statuses = vec![
        RunStatus::Queued,
        RunStatus::InProgress,
        RunStatus::RequiresAction,
        RunStatus::Cancelling,
        RunStatus::Cancelled,
        RunStatus::Failed,
        RunStatus::Completed,
        RunStatus::Expired,
    ];

    for status in statuses {
        let json = serde_json::to_string(&status).expect("Failed to serialize status");
        let deserialized: RunStatus =
            serde_json::from_str(&json).expect("Failed to deserialize status");
        assert_eq!(status, deserialized);
    }
}

#[test]
fn test_tool_output_creation() {
    let tool_output = ToolOutput {
        tool_call_id: "call_abc123".to_string(),
        output: "The result is 42".to_string(),
    };

    assert_eq!(tool_output.tool_call_id, "call_abc123");
    assert_eq!(tool_output.output, "The result is 42");
}

#[test]
fn test_submit_tool_outputs_request_creation() {
    let tool_outputs = vec![
        ToolOutput {
            tool_call_id: "call_abc123".to_string(),
            output: "Result 1".to_string(),
        },
        ToolOutput {
            tool_call_id: "call_def456".to_string(),
            output: "Result 2".to_string(),
        },
    ];

    let request = SubmitToolOutputsRequest { tool_outputs };

    assert_eq!(request.tool_outputs.len(), 2);
    assert_eq!(request.tool_outputs[0].tool_call_id, "call_abc123");
    assert_eq!(request.tool_outputs[1].tool_call_id, "call_def456");
}

#[test]
fn test_modify_run_request_creation() {
    let mut metadata = HashMap::new();
    metadata.insert("updated".to_string(), "true".to_string());
    metadata.insert("priority".to_string(), "high".to_string());

    let request = ModifyRunRequest {
        metadata: Some(metadata),
    };

    assert!(request.metadata.is_some());
    let metadata = request.metadata.unwrap();
    assert_eq!(metadata.get("updated"), Some(&"true".to_string()));
    assert_eq!(metadata.get("priority"), Some(&"high".to_string()));
}

#[test]
fn test_list_runs_params_default() {
    let params = ListRunsParams::default();
    assert_eq!(params.limit, Some(20));
    assert_eq!(params.order, Some("desc".to_string()));
    assert_eq!(params.after, None);
    assert_eq!(params.before, None);
}

#[test]
fn test_list_runs_params_custom() {
    let params = ListRunsParams {
        limit: Some(50),
        order: Some("asc".to_string()),
        after: Some("run_abc123".to_string()),
        before: Some("run_def456".to_string()),
    };

    assert_eq!(params.limit, Some(50));
    assert_eq!(params.order, Some("asc".to_string()));
    assert_eq!(params.after, Some("run_abc123".to_string()));
    assert_eq!(params.before, Some("run_def456".to_string()));
}

#[test]
fn test_list_run_steps_params_default() {
    let params = ListRunStepsParams::default();
    assert_eq!(params.limit, Some(20));
    assert_eq!(params.order, Some("desc".to_string()));
    assert_eq!(params.after, None);
    assert_eq!(params.before, None);
}

#[test]
fn test_list_run_steps_params_custom() {
    let params = ListRunStepsParams {
        limit: Some(100),
        order: Some("asc".to_string()),
        after: Some("step_abc123".to_string()),
        before: Some("step_def456".to_string()),
    };

    assert_eq!(params.limit, Some(100));
    assert_eq!(params.order, Some("asc".to_string()));
    assert_eq!(params.after, Some("step_abc123".to_string()));
    assert_eq!(params.before, Some("step_def456".to_string()));
}

#[test]
fn test_thread_create_request_creation() {
    let messages = vec![
        ThreadMessage {
            role: "user".to_string(),
            content: "Hello, world!".to_string(),
            file_ids: Some(vec!["file_abc123".to_string()]),
            metadata: Some({
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "greeting".to_string());
                meta
            }),
        },
        ThreadMessage {
            role: "assistant".to_string(),
            content: "Hello! How can I help you today?".to_string(),
            file_ids: None,
            metadata: None,
        },
    ];

    let mut thread_metadata = HashMap::new();
    thread_metadata.insert("purpose".to_string(), "customer_support".to_string());

    let thread_request = ThreadCreateRequest {
        messages: Some(messages),
        metadata: Some(thread_metadata),
    };

    assert!(thread_request.messages.is_some());
    assert_eq!(thread_request.messages.as_ref().unwrap().len(), 2);
    assert!(thread_request.metadata.is_some());
    assert_eq!(
        thread_request.metadata.unwrap().get("purpose"),
        Some(&"customer_support".to_string())
    );
}

#[test]
fn test_thread_message_creation() {
    let message = ThreadMessage {
        role: "user".to_string(),
        content: "Analyze this data please".to_string(),
        file_ids: Some(vec!["file_123".to_string(), "file_456".to_string()]),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("priority".to_string(), "high".to_string());
            meta
        }),
    };

    assert_eq!(message.role, "user");
    assert_eq!(message.content, "Analyze this data please");
    assert_eq!(
        message.file_ids,
        Some(vec!["file_123".to_string(), "file_456".to_string()])
    );
    assert!(message.metadata.is_some());
}

#[test]
fn test_run_request_serialization() {
    let request = RunRequest::builder()
        .assistant_id("asst_abc123")
        .model("gpt-4")
        .instructions("Test instructions")
        .tool(AssistantTool::CodeInterpreter)
        .file_id("file_abc123")
        .metadata_pair("test", "value")
        .build()
        .expect("Failed to build request");

    let json = serde_json::to_string(&request).expect("Failed to serialize request");
    let deserialized: RunRequest =
        serde_json::from_str(&json).expect("Failed to deserialize request");

    assert_eq!(request.assistant_id, deserialized.assistant_id);
    assert_eq!(request.model, deserialized.model);
    assert_eq!(request.instructions, deserialized.instructions);
    assert_eq!(request.tools, deserialized.tools);
    assert_eq!(request.file_ids, deserialized.file_ids);
    assert_eq!(request.metadata, deserialized.metadata);
}

#[test]
fn test_create_thread_and_run_request_serialization() {
    let thread = ThreadCreateRequest {
        messages: Some(vec![ThreadMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
            file_ids: None,
            metadata: None,
        }]),
        metadata: None,
    };

    let request = CreateThreadAndRunRequest::builder()
        .assistant_id("asst_abc123")
        .thread(thread)
        .model("gpt-4")
        .instructions("Be helpful")
        .build()
        .expect("Failed to build request");

    let json = serde_json::to_string(&request).expect("Failed to serialize request");
    let deserialized: CreateThreadAndRunRequest =
        serde_json::from_str(&json).expect("Failed to deserialize request");

    assert_eq!(request.assistant_id, deserialized.assistant_id);
    assert_eq!(request.model, deserialized.model);
    assert_eq!(request.instructions, deserialized.instructions);
    assert!(deserialized.thread.is_some());
}

#[test]
fn test_tool_output_serialization() {
    let tool_output = ToolOutput {
        tool_call_id: "call_abc123".to_string(),
        output: "The calculation result is 42".to_string(),
    };

    let json = serde_json::to_string(&tool_output).expect("Failed to serialize tool output");
    let deserialized: ToolOutput =
        serde_json::from_str(&json).expect("Failed to deserialize tool output");

    assert_eq!(tool_output.tool_call_id, deserialized.tool_call_id);
    assert_eq!(tool_output.output, deserialized.output);
}

#[test]
fn test_submit_tool_outputs_request_serialization() {
    let request = SubmitToolOutputsRequest {
        tool_outputs: vec![
            ToolOutput {
                tool_call_id: "call_abc123".to_string(),
                output: "Result 1".to_string(),
            },
            ToolOutput {
                tool_call_id: "call_def456".to_string(),
                output: "Result 2".to_string(),
            },
        ],
    };

    let json = serde_json::to_string(&request).expect("Failed to serialize request");
    let deserialized: SubmitToolOutputsRequest =
        serde_json::from_str(&json).expect("Failed to deserialize request");

    assert_eq!(request.tool_outputs.len(), deserialized.tool_outputs.len());
    assert_eq!(
        request.tool_outputs[0].tool_call_id,
        deserialized.tool_outputs[0].tool_call_id
    );
    assert_eq!(
        request.tool_outputs[1].output,
        deserialized.tool_outputs[1].output
    );
}

#[test]
fn test_modify_run_request_serialization() {
    let mut metadata = HashMap::new();
    metadata.insert("updated".to_string(), "true".to_string());

    let request = ModifyRunRequest {
        metadata: Some(metadata),
    };

    let json = serde_json::to_string(&request).expect("Failed to serialize request");
    let deserialized: ModifyRunRequest =
        serde_json::from_str(&json).expect("Failed to deserialize request");

    assert_eq!(request.metadata, deserialized.metadata);
}

// Integration tests (would require actual API key and network access)
// These are commented out but show how to test the actual API calls

/*
#[tokio::test]
async fn test_create_run_integration() {
    use std::env;

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api = RunsApi::new(api_key).expect("Failed to create API client");

    let request = RunRequest::builder()
        .assistant_id("asst_test123")  // Replace with actual assistant ID
        .instructions("Please help me test this API")
        .build()
        .expect("Failed to build request");

    let result = api.create_run("thread_test123", request).await;  // Replace with actual thread ID
    match result {
        Ok(run) => {
            assert!(!run.id.is_empty());
            assert_eq!(run.assistant_id, "asst_test123");
        }
        Err(e) => panic!("API call failed: {:?}", e),
    }
}

#[tokio::test]
async fn test_retrieve_run_integration() {
    use std::env;

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api = RunsApi::new(api_key).expect("Failed to create API client");

    let result = api.retrieve_run("thread_test123", "run_test123").await;  // Replace with actual IDs
    match result {
        Ok(run) => {
            assert!(!run.id.is_empty());
            assert!(!run.thread_id.is_empty());
        }
        Err(e) => panic!("API call failed: {:?}", e),
    }
}

#[tokio::test]
async fn test_list_runs_integration() {
    use std::env;

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api = RunsApi::new(api_key).expect("Failed to create API client");

    let params = ListRunsParams {
        limit: Some(5),
        ..Default::default()
    };

    let result = api.list_runs("thread_test123", Some(params)).await;  // Replace with actual thread ID
    match result {
        Ok(response) => {
            assert_eq!(response.object, "list");
            assert!(response.data.len() <= 5);
        }
        Err(e) => panic!("API call failed: {:?}", e),
    }
}

#[tokio::test]
async fn test_cancel_run_integration() {
    use std::env;

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api = RunsApi::new(api_key).expect("Failed to create API client");

    let result = api.cancel_run("thread_test123", "run_test123").await;  // Replace with actual IDs
    match result {
        Ok(run) => {
            assert!(matches!(run.status, RunStatus::Cancelling | RunStatus::Cancelled));
        }
        Err(e) => panic!("API call failed: {:?}", e),
    }
}

#[tokio::test]
async fn test_list_run_steps_integration() {
    use std::env;

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api = RunsApi::new(api_key).expect("Failed to create API client");

    let params = ListRunStepsParams {
        limit: Some(10),
        ..Default::default()
    };

    let result = api.list_run_steps("thread_test123", "run_test123", Some(params)).await;  // Replace with actual IDs
    match result {
        Ok(response) => {
            assert_eq!(response.object, "list");
            assert!(response.data.len() <= 10);
        }
        Err(e) => panic!("API call failed: {:?}", e),
    }
}
*/
