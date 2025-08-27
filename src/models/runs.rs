//! # OpenAI Runs & Run Steps API Models
//!
//! This module provides data structures for OpenAI's Runs API, which allows you to
//! execute assistants on threads with tool calling capabilities and step-by-step execution tracking.
//!
//! ## Overview
//!
//! The Runs API supports:
//! - **Run Execution**: Execute assistants on threads with specified parameters
//! - **Tool Calling**: Support for Code Interpreter, Retrieval, and Function calling
//! - **Status Tracking**: Monitor run progress through various status states
//! - **Step Tracking**: Detailed tracking of individual execution steps
//! - **Error Handling**: Comprehensive error reporting and recovery
//! - **Token Usage**: Track token consumption for runs and steps
//! - **Required Actions**: Handle function calls that require user input
//! - **Streaming**: Real-time updates for run execution
//!
//! ## Run Lifecycle
//!
//! Runs go through several status states:
//! - `queued`: Run is waiting to be processed
//! - `in_progress`: Run is currently executing
//! - `requires_action`: Run is waiting for user input (e.g., function call results)
//! - `cancelling`: Run is being cancelled
//! - `cancelled`: Run was cancelled
//! - `failed`: Run failed with an error
//! - `completed`: Run completed successfully
//! - `expired`: Run expired without completion
//!
//! ## Run Steps
//!
//! Each run is broken down into individual steps that can be:
//! - **Message Creation**: Creating a new message in the thread
//! - **Tool Calls**: Executing tools like Code Interpreter, Retrieval, or Functions
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::runs::{RunRequest, RunStatus, ToolOutput};
//! use std::collections::HashMap;
//!
//! // Create a run request
//! let run_request = RunRequest::builder()
//!     .assistant_id("asst_abc123")
//!     .model("gpt-4")
//!     .instructions("Please analyze the data and provide insights.")
//!     .build();
//!
//! // Create tool output for function calls
//! let tool_output = ToolOutput {
//!     tool_call_id: "call_abc123".to_string(),
//!     output: "The calculation result is 42".to_string(),
//! };
//! ```

use crate::models::assistants::AssistantTool;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// Common trait for list parameter types
trait ListParams {
    /// Get the limit parameter
    fn get_limit(&self) -> Option<u32>;
    /// Get the order parameter
    fn get_order(&self) -> Option<&String>;
    /// Get the after cursor parameter
    fn get_after(&self) -> Option<&String>;
    /// Get the before cursor parameter
    fn get_before(&self) -> Option<&String>;

    /// Build query parameters for the API request
    fn build_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(limit) = self.get_limit() {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(order) = self.get_order() {
            params.push(("order".to_string(), order.clone()));
        }
        if let Some(after) = self.get_after() {
            params.push(("after".to_string(), after.clone()));
        }
        if let Some(before) = self.get_before() {
            params.push(("before".to_string(), before.clone()));
        }
        params
    }
}

/// Common fields for run configuration
#[allow(dead_code)]
trait RunConfiguration {
    /// Get the model field
    fn get_model(&self) -> Option<&String>;
    /// Get the instructions field
    fn get_instructions(&self) -> Option<&String>;
    /// Get the tools field
    fn get_tools(&self) -> Option<&Vec<AssistantTool>>;
    /// Get the file_ids field
    fn get_file_ids(&self) -> Option<&Vec<String>>;
    /// Get the metadata field
    fn get_metadata(&self) -> Option<&HashMap<String, String>>;
}

/// A run represents an execution of an Assistant on a Thread
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct Run {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always `thread.run`
    pub object: String,
    /// The Unix timestamp (in seconds) for when the run was created
    pub created_at: i64,
    /// The ID of the thread that was executed on as a part of this run
    pub thread_id: String,
    /// The ID of the assistant used for execution of this run
    pub assistant_id: String,
    /// The status of the run
    pub status: RunStatus,
    /// Details on the action required to continue the run
    pub required_action: Option<RequiredAction>,
    /// The last error associated with this run
    pub last_error: Option<RunError>,
    /// The Unix timestamp (in seconds) for when the run will expire
    pub expires_at: Option<i64>,
    /// The Unix timestamp (in seconds) for when the run was started
    pub started_at: Option<i64>,
    /// The Unix timestamp (in seconds) for when the run was cancelled
    pub cancelled_at: Option<i64>,
    /// The Unix timestamp (in seconds) for when the run failed
    pub failed_at: Option<i64>,
    /// The Unix timestamp (in seconds) for when the run completed
    pub completed_at: Option<i64>,
    /// The model that the assistant used for this run
    pub model: String,
    /// The instructions that the assistant used for this run
    pub instructions: String,
    /// The list of tools that the assistant used for this run
    #[serde(default)]
    pub tools: Vec<AssistantTool>,
    /// The list of File IDs the assistant can access (deprecated in v2)
    #[serde(default)]
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    /// Usage statistics for the completion request
    pub usage: Option<Usage>,
}

/// Request to create a new run
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct RunRequest {
    /// The ID of the assistant to use to execute this run
    pub assistant_id: String,
    /// The model to use for this run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Override the default system message of the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Override the tools the assistant can use for this run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTool>>,
    /// Override the file IDs the assistant can access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Request to create a thread and run it in one request
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct CreateThreadAndRunRequest {
    /// The ID of the assistant to use to execute this run
    pub assistant_id: String,
    /// If no thread is provided, an empty thread will be created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<ThreadCreateRequest>,
    /// The model to use for this run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Override the default system message of the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Override the tools the assistant can use for this run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTool>>,
    /// Override the file IDs the assistant can access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Thread creation request for use in `create_thread_and_run`
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct ThreadCreateRequest {
    /// A list of messages to start the thread with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ThreadMessage>>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Message for thread creation
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct ThreadMessage {
    /// The role of the message author
    pub role: String,
    /// The content of the message
    pub content: String,
    /// A list of File IDs that the message should use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// The status of a run
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    /// The run is queued and waiting to be processed
    Queued,
    /// The run is currently in progress
    InProgress,
    /// The run requires action from the user to continue
    RequiresAction,
    /// The run is being cancelled
    Cancelling,
    /// The run was cancelled
    Cancelled,
    /// The run failed
    Failed,
    /// The run completed successfully
    Completed,
    /// The run expired
    Expired,
}

/// Details on the action required to continue the run
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct RequiredAction {
    /// The type of action required
    #[serde(rename = "type")]
    pub action_type: String,
    /// Details of the action required
    pub submit_tool_outputs: SubmitToolOutputs,
}

/// Details for submitting tool outputs
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct SubmitToolOutputs {
    /// A list of the relevant tool calls
    pub tool_calls: Vec<ToolCall>,
}

/// A tool call generated by the assistant
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct ToolCall {
    /// The ID of the tool call
    pub id: String,
    /// The type of tool call
    #[serde(rename = "type")]
    pub call_type: String,
    /// The function definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionCall>,
}

/// Function call details
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct FunctionCall {
    /// The name of the function to call
    pub name: String,
    /// The arguments to call the function with, as generated by the model in JSON format
    pub arguments: String,
}

/// Tool output for submitting results of tool calls
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct ToolOutput {
    /// The ID of the tool call to which this output corresponds
    pub tool_call_id: String,
    /// The output of the tool call to be submitted back to the assistant
    pub output: String,
}

/// Request to submit tool outputs
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct SubmitToolOutputsRequest {
    /// A list of tools for which the outputs are being submitted
    pub tool_outputs: Vec<ToolOutput>,
}

/// Error information for a run
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct RunError {
    /// One of `server_error` or `rate_limit_exceeded`
    pub code: String,
    /// A human-readable description of the error
    pub message: String,
}

/// A run step represents a step in execution of a run
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct RunStep {
    /// The identifier of the run step, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always `thread.run.step`
    pub object: String,
    /// The Unix timestamp (in seconds) for when the run step was created
    pub created_at: i64,
    /// The ID of the assistant associated with the run step
    pub assistant_id: String,
    /// The ID of the thread that was run
    pub thread_id: String,
    /// The ID of the run that this run step is a part of
    pub run_id: String,
    /// The type of run step
    #[serde(rename = "type")]
    pub step_type: String,
    /// The status of the run step
    pub status: RunStepStatus,
    /// The details of the run step
    pub step_details: StepDetails,
    /// The last error associated with this run step
    pub last_error: Option<RunError>,
    /// The Unix timestamp (in seconds) for when the run step expired
    pub expired_at: Option<i64>,
    /// The Unix timestamp (in seconds) for when the run step was cancelled
    pub cancelled_at: Option<i64>,
    /// The Unix timestamp (in seconds) for when the run step failed
    pub failed_at: Option<i64>,
    /// The Unix timestamp (in seconds) for when the run step completed
    pub completed_at: Option<i64>,
    /// Set of 16 key-value pairs that can be attached to an object
    pub metadata: HashMap<String, String>,
    /// Usage statistics for this step
    pub usage: Option<Usage>,
}

/// The status of a run step
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum RunStepStatus {
    /// The run step is in progress
    InProgress,
    /// The run step was cancelled
    Cancelled,
    /// The run step failed
    Failed,
    /// The run step completed
    Completed,
    /// The run step expired
    Expired,
}

/// Details of a run step
#[derive(Debug, Clone, PartialEq, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StepDetails {
    /// Details of a message creation step
    MessageCreation {
        /// Details of the message creation
        message_creation: MessageCreation,
    },
    /// Details of a tool calls step
    ToolCalls {
        /// Details of the tool calls
        tool_calls: Vec<StepToolCall>,
    },
}

/// Details of message creation step
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct MessageCreation {
    /// The ID of the message that was created by this run step
    pub message_id: String,
}

/// A tool call within a run step
#[derive(Debug, Clone, PartialEq, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StepToolCall {
    /// Code interpreter tool call
    CodeInterpreter {
        /// The ID of the tool call
        id: String,
        /// The Code Interpreter tool call definition
        code_interpreter: CodeInterpreterCall,
    },
    /// Retrieval tool call
    Retrieval {
        /// The ID of the tool call
        id: String,
        /// The retrieval tool call definition
        retrieval: HashMap<String, serde_json::Value>,
    },
    /// Function tool call
    Function {
        /// The ID of the tool call
        id: String,
        /// The function tool call definition
        function: FunctionCall,
    },
}

/// Code interpreter tool call details
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct CodeInterpreterCall {
    /// The input to the Code Interpreter tool call
    pub input: String,
    /// The outputs from the Code Interpreter tool call
    pub outputs: Vec<CodeInterpreterOutput>,
}

/// Output from a Code Interpreter tool call
#[derive(Debug, Clone, PartialEq, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodeInterpreterOutput {
    /// Logs output from Code Interpreter
    Logs {
        /// The text output from the Code Interpreter tool call
        logs: String,
    },
    /// Image output from Code Interpreter
    Image {
        /// The image data
        image: CodeInterpreterImage,
    },
}

/// Image output from Code Interpreter
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct CodeInterpreterImage {
    /// The file ID of the image
    pub file_id: String,
}

// Generate list response and parameter structures using macros
crate::impl_list_response!(ListRunsResponse, Run, "runs");
crate::impl_list_params!(ListRunsParams, "runs");
crate::impl_list_response!(ListRunStepsResponse, RunStep, "run steps");
crate::impl_list_params!(ListRunStepsParams, "run steps");

/// Usage statistics for a run or run step
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct Usage {
    /// Number of completion tokens used
    pub completion_tokens: u32,
    /// Number of prompt tokens used
    pub prompt_tokens: u32,
    /// Total number of tokens used
    pub total_tokens: u32,
}

/// Request to modify a run
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct ModifyRunRequest {
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Trait for common run configuration builder methods
#[allow(dead_code)]
trait RunConfigurationBuilder: Sized {
    /// Get mutable reference to the model field
    fn get_model_mut(&mut self) -> &mut Option<String>;

    /// Get mutable reference to the instructions field
    fn get_instructions_mut(&mut self) -> &mut Option<String>;

    /// Get mutable reference to the tools field
    fn get_tools_mut(&mut self) -> &mut Option<Vec<AssistantTool>>;

    /// Get mutable reference to the file_ids field
    fn get_file_ids_mut(&mut self) -> &mut Option<Vec<String>>;

    /// Get mutable reference to the metadata field
    fn get_metadata_mut(&mut self) -> &mut Option<HashMap<String, String>>;

    /// Set the model
    fn model<S: Into<String>>(mut self, model: S) -> Self {
        *self.get_model_mut() = Some(model.into());
        self
    }

    /// Set the instructions
    fn instructions<S: Into<String>>(mut self, instructions: S) -> Self {
        *self.get_instructions_mut() = Some(instructions.into());
        self
    }

    /// Add a tool
    fn tool(mut self, tool: AssistantTool) -> Self {
        self.get_tools_mut().get_or_insert_with(Vec::new).push(tool);
        self
    }

    /// Set tools
    fn tools(mut self, tools: Vec<AssistantTool>) -> Self {
        *self.get_tools_mut() = Some(tools);
        self
    }

    /// Add a file ID
    fn file_id<S: Into<String>>(mut self, file_id: S) -> Self {
        self.get_file_ids_mut()
            .get_or_insert_with(Vec::new)
            .push(file_id.into());
        self
    }

    /// Set file IDs
    fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        *self.get_file_ids_mut() = Some(file_ids);
        self
    }

    /// Add metadata key-value pair
    fn metadata_pair<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.get_metadata_mut()
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Set metadata
    fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        *self.get_metadata_mut() = Some(metadata);
        self
    }
}

/// Builder for `RunRequest`
impl RunRequest {
    /// Create a new builder for `RunRequest`
    #[must_use]
    pub fn builder() -> RunRequestBuilder {
        RunRequestBuilder::default()
    }
}

/// Builder for `RunRequest`
#[derive(Debug, Default)]
pub struct RunRequestBuilder {
    /// The ID of the assistant to use for this run
    assistant_id: Option<String>,
    /// The model to use for this run, overriding the assistant's model
    model: Option<String>,
    /// Override the default system message for the assistant
    instructions: Option<String>,
    /// Override the tools available to the assistant for this run
    tools: Option<Vec<AssistantTool>>,
    /// A list of file IDs to attach to this run
    file_ids: Option<Vec<String>>,
    /// Set of key-value pairs to attach to this run
    metadata: Option<HashMap<String, String>>,
}

crate::impl_run_config_builder!(RunRequestBuilder);

impl RunRequestBuilder {
    /// Set the assistant ID
    pub fn assistant_id<S: Into<String>>(mut self, assistant_id: S) -> Self {
        self.assistant_id = Some(assistant_id.into());
        self
    }

    crate::impl_run_builder_methods!();
}

// Generate the build method for RunRequestBuilder
crate::impl_builder_build! {
    RunRequestBuilder => RunRequest {
        required: [assistant_id: "assistant_id is required"],
        optional: [model, instructions, tools, file_ids, metadata]
    }
}

/// Builder for `CreateThreadAndRunRequest`
impl CreateThreadAndRunRequest {
    /// Create a new builder for `CreateThreadAndRunRequest`
    #[must_use]
    pub fn builder() -> CreateThreadAndRunRequestBuilder {
        CreateThreadAndRunRequestBuilder::default()
    }
}

/// Builder for `CreateThreadAndRunRequest`
#[derive(Debug, Default)]
pub struct CreateThreadAndRunRequestBuilder {
    /// The ID of the assistant to use for this run
    assistant_id: Option<String>,
    /// The thread to create and run with the assistant
    thread: Option<ThreadCreateRequest>,
    /// The model to use for this run, overriding the assistant's model
    model: Option<String>,
    /// Override the default system message for the assistant
    instructions: Option<String>,
    /// Override the tools available to the assistant for this run
    tools: Option<Vec<AssistantTool>>,
    /// A list of file IDs to attach to this run
    file_ids: Option<Vec<String>>,
    /// Set of key-value pairs to attach to this run
    metadata: Option<HashMap<String, String>>,
}

crate::impl_run_config_builder!(CreateThreadAndRunRequestBuilder);

impl CreateThreadAndRunRequestBuilder {
    /// Set the assistant ID
    pub fn assistant_id<S: Into<String>>(mut self, assistant_id: S) -> Self {
        self.assistant_id = Some(assistant_id.into());
        self
    }

    /// Set the thread
    #[must_use]
    pub fn thread(mut self, thread: ThreadCreateRequest) -> Self {
        self.thread = Some(thread);
        self
    }

    crate::impl_run_builder_methods!();
}

// Generate the build method for CreateThreadAndRunRequestBuilder
crate::impl_builder_build! {
    CreateThreadAndRunRequestBuilder => CreateThreadAndRunRequest {
        required: [assistant_id: "assistant_id is required"],
        optional: [thread, model, instructions, tools, file_ids, metadata]
    }
}

/// Create default list parameters
fn create_default_list_params() -> (Option<u32>, Option<String>, Option<String>, Option<String>) {
    (Some(20), Some("desc".to_string()), None, None)
}
