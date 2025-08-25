//! # OpenAI Runs & Run Steps API Client
//!
//! This module provides a complete implementation of OpenAI's Runs API, which allows you to
//! execute assistants on threads with tool calling capabilities and step-by-step execution tracking.
//!
//! ## Features
//!
//! - **Run Management**: Create, retrieve, modify, list, and cancel runs
//! - **Tool Calling**: Submit tool outputs for function calls and other tools
//! - **Step Tracking**: List and retrieve individual run steps
//! - **Thread Integration**: Create threads and runs in a single request
//! - **Status Monitoring**: Track run progress through various status states
//! - **Error Handling**: Comprehensive error handling with detailed messages
//! - **Streaming Support**: Real-time updates for run execution (when available)
//! - **Usage Tracking**: Monitor token consumption for runs and steps
//!
//! ## Run Lifecycle
//!
//! 1. **Create**: Start a new run on a thread with an assistant
//! 2. **Monitor**: Check status and handle required actions
//! 3. **Interact**: Submit tool outputs when required
//! 4. **Complete**: Run finishes with success, failure, or cancellation
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
//! use openai_rust_sdk::models::runs::{RunRequest, ToolOutput, SubmitToolOutputsRequest};
//!
//! # tokio_test::block_on(async {
//! let api = RunsApi::new("your-api-key")?;
//!
//! // Create a new run
//! let run_request = RunRequest::builder()
//!     .assistant_id("asst_abc123")
//!     .instructions("Please analyze the uploaded data.")
//!     .build()?;
//!
//! let run = api.create_run("thread_abc123", run_request).await?;
//! println!("Created run: {}", run.id);
//!
//! // Monitor run status
//! loop {
//!     let run = api.retrieve_run("thread_abc123", &run.id).await?;
//!     match run.status {
//!         openai_rust_sdk::models::runs::RunStatus::RequiresAction => {
//!             // Handle tool calls
//!             if let Some(required_action) = run.required_action {
//!                 let tool_outputs: Vec<ToolOutput> = required_action
//!                     .submit_tool_outputs
//!                     .tool_calls
//!                     .into_iter()
//!                     .map(|call| ToolOutput {
//!                         tool_call_id: call.id,
//!                         output: "Tool result".to_string(),
//!                     })
//!                     .collect();
//!
//!                 let request = SubmitToolOutputsRequest { tool_outputs };
//!                 api.submit_tool_outputs("thread_abc123", &run.id, request).await?;
//!             }
//!         }
//!         openai_rust_sdk::models::runs::RunStatus::Completed => {
//!             println!("Run completed successfully!");
//!             break;
//!         }
//!         openai_rust_sdk::models::runs::RunStatus::Failed => {
//!             println!("Run failed: {:?}", run.last_error);
//!             break;
//!         }
//!         _ => {
//!             tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//!         }
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::{OpenAIError, Result};
use crate::models::runs::{
    CreateThreadAndRunRequest, ListRunStepsParams, ListRunStepsResponse, ListRunsParams,
    ListRunsResponse, ModifyRunRequest, Run, RunRequest, RunStep, SubmitToolOutputsRequest,
};

/// `OpenAI` Runs API client for managing assistant run execution
#[derive(Debug, Clone)]
pub struct RunsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for RunsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl RunsApi {
    /// Creates a new Runs API client with a custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    /// * `base_url` - Custom API base URL
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    ///
    /// let api = RunsApi::with_base_url("your-api-key", "https://api.openai.com")?;
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// ```
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Create a run
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to run
    /// * `request` - The run request parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::RunRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let request = RunRequest::builder()
    ///     .assistant_id("asst_abc123")
    ///     .instructions("Please analyze the data.")
    ///     .build()?;
    ///
    /// let run = api.create_run("thread_abc123", request).await?;
    /// println!("Created run: {}", run.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_run<S: AsRef<str>>(
        &self,
        thread_id: S,
        request: RunRequest,
    ) -> Result<Run> {
        self.http_client
            .post_with_beta(
                &format!("/v1/threads/{}/runs", thread_id.as_ref()),
                &request,
            )
            .await
    }

    /// Create a thread and run it in one request
    ///
    /// # Arguments
    ///
    /// * `request` - The thread and run request parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::CreateThreadAndRunRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let request = CreateThreadAndRunRequest::builder()
    ///     .assistant_id("asst_abc123")
    ///     .instructions("Please help me with this task.")
    ///     .build()?;
    ///
    /// let run = api.create_thread_and_run(request).await?;
    /// println!("Created thread and run: {} in thread {}", run.id, run.thread_id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_thread_and_run(&self, request: CreateThreadAndRunRequest) -> Result<Run> {
        self.http_client
            .post_with_beta("/v1/threads/runs", &request)
            .await
    }

    /// Retrieve a run
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `run_id` - The ID of the run to retrieve
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let run = api.retrieve_run("thread_abc123", "run_abc123").await?;
    /// println!("Run status: {:?}", run.status);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn retrieve_run<S: AsRef<str>, R: AsRef<str>>(
        &self,
        thread_id: S,
        run_id: R,
    ) -> Result<Run> {
        self.http_client
            .get_with_beta(&format!(
                "/v1/threads/{}/runs/{}",
                thread_id.as_ref(),
                run_id.as_ref()
            ))
            .await
    }

    /// Modify a run
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `run_id` - The ID of the run to modify
    /// * `request` - The modification request
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::ModifyRunRequest;
    /// use std::collections::HashMap;
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let mut metadata = HashMap::new();
    /// metadata.insert("updated".to_string(), "true".to_string());
    /// let request = ModifyRunRequest { metadata: Some(metadata) };
    ///
    /// let run = api.modify_run("thread_abc123", "run_abc123", request).await?;
    /// println!("Modified run: {}", run.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn modify_run<S: AsRef<str>, R: AsRef<str>>(
        &self,
        thread_id: S,
        run_id: R,
        request: ModifyRunRequest,
    ) -> Result<Run> {
        self.http_client
            .post_with_beta(
                &format!(
                    "/v1/threads/{}/runs/{}",
                    thread_id.as_ref(),
                    run_id.as_ref()
                ),
                &request,
            )
            .await
    }

    /// List runs in a thread
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::ListRunsParams;
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let params = ListRunsParams { limit: Some(10), ..Default::default() };
    /// let response = api.list_runs("thread_abc123", Some(params)).await?;
    /// println!("Found {} runs", response.data.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_runs<S: AsRef<str>>(
        &self,
        thread_id: S,
        params: Option<ListRunsParams>,
    ) -> Result<ListRunsResponse> {
        let path = format!("/v1/threads/{}/runs", thread_id.as_ref());

        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query_and_beta(&path, &query_params)
            .await
    }

    /// Submit tool outputs to a run
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `run_id` - The ID of the run
    /// * `request` - The tool outputs to submit
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::{SubmitToolOutputsRequest, ToolOutput};
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let request = SubmitToolOutputsRequest {
    ///     tool_outputs: vec![
    ///         ToolOutput {
    ///             tool_call_id: "call_abc123".to_string(),
    ///             output: "The result is 42".to_string(),
    ///         }
    ///     ],
    /// };
    ///
    /// let run = api.submit_tool_outputs("thread_abc123", "run_abc123", request).await?;
    /// println!("Submitted tool outputs to run: {}", run.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn submit_tool_outputs<S: AsRef<str>, R: AsRef<str>>(
        &self,
        thread_id: S,
        run_id: R,
        request: SubmitToolOutputsRequest,
    ) -> Result<Run> {
        self.http_client
            .post_with_beta(
                &format!(
                    "/v1/threads/{}/runs/{}/submit_tool_outputs",
                    thread_id.as_ref(),
                    run_id.as_ref()
                ),
                &request,
            )
            .await
    }

    /// Cancel a run
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `run_id` - The ID of the run to cancel
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let run = api.cancel_run("thread_abc123", "run_abc123").await?;
    /// println!("Cancelled run: {}", run.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn cancel_run<S: AsRef<str>, R: AsRef<str>>(
        &self,
        thread_id: S,
        run_id: R,
    ) -> Result<Run> {
        // For cancel operations, we need to send an empty POST request
        let empty_request: serde_json::Value = serde_json::json!({});
        self.http_client
            .post_with_beta(
                &format!(
                    "/v1/threads/{}/runs/{}/cancel",
                    thread_id.as_ref(),
                    run_id.as_ref()
                ),
                &empty_request,
            )
            .await
    }

    /// List run steps in a run
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `run_id` - The ID of the run
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::ListRunStepsParams;
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let params = ListRunStepsParams { limit: Some(10), ..Default::default() };
    /// let response = api.list_run_steps("thread_abc123", "run_abc123", Some(params)).await?;
    /// println!("Found {} run steps", response.data.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_run_steps<S: AsRef<str>, R: AsRef<str>>(
        &self,
        thread_id: S,
        run_id: R,
        params: Option<ListRunStepsParams>,
    ) -> Result<ListRunStepsResponse> {
        let endpoint = format!(
            "/v1/threads/{}/runs/{}/steps",
            thread_id.as_ref(),
            run_id.as_ref()
        );

        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query(&endpoint, &query_params)
            .await
    }

    /// Retrieve a run step
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `run_id` - The ID of the run
    /// * `step_id` - The ID of the run step to retrieve
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let step = api.retrieve_run_step("thread_abc123", "run_abc123", "step_abc123").await?;
    /// println!("Run step status: {:?}", step.status);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn retrieve_run_step<S: AsRef<str>, R: AsRef<str>, T: AsRef<str>>(
        &self,
        thread_id: S,
        run_id: R,
        step_id: T,
    ) -> Result<RunStep> {
        let endpoint = format!(
            "/v1/threads/{}/runs/{}/steps/{}",
            thread_id.as_ref(),
            run_id.as_ref(),
            step_id.as_ref()
        );
        self.http_client.get(&endpoint).await
    }

    /// Submit tool outputs with streaming
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread
    /// * `run_id` - The ID of the run
    /// * `request` - The tool outputs to submit
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::{SubmitToolOutputsRequest, ToolOutput};
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let request = SubmitToolOutputsRequest {
    ///     tool_outputs: vec![
    ///         ToolOutput {
    ///             tool_call_id: "call_abc123".to_string(),
    ///             output: "The result is 42".to_string(),
    ///         }
    ///     ],
    /// };
    ///
    /// let run = api.submit_tool_outputs_stream("thread_abc123", "run_abc123", request).await?;
    /// println!("Submitted tool outputs with streaming to run: {}", run.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn submit_tool_outputs_stream<S: AsRef<str>, R: AsRef<str>>(
        &self,
        thread_id: S,
        run_id: R,
        request: SubmitToolOutputsRequest,
    ) -> Result<Run> {
        let endpoint = format!(
            "/v1/threads/{}/runs/{}/submit_tool_outputs",
            thread_id.as_ref(),
            run_id.as_ref()
        );

        // Add stream: true to the JSON body
        let mut request_json = serde_json::to_value(&request).map_err(OpenAIError::Json)?;
        request_json["stream"] = serde_json::Value::Bool(true);

        self.http_client.post(&endpoint, &request_json).await
    }

    /// Create a run with streaming
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to run
    /// * `request` - The run request parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::RunRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let request = RunRequest::builder()
    ///     .assistant_id("asst_abc123")
    ///     .instructions("Please analyze the data.")
    ///     .build()?;
    ///
    /// let run = api.create_run_stream("thread_abc123", request).await?;
    /// println!("Created streaming run: {}", run.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_run_stream<S: AsRef<str>>(
        &self,
        thread_id: S,
        request: RunRequest,
    ) -> Result<Run> {
        let endpoint = format!("/v1/threads/{}/runs", thread_id.as_ref());

        // Add stream: true to the JSON body
        let mut request_json = serde_json::to_value(&request).map_err(OpenAIError::Json)?;
        request_json["stream"] = serde_json::Value::Bool(true);

        self.http_client.post(&endpoint, &request_json).await
    }

    /// Create a thread and run with streaming
    ///
    /// # Arguments
    ///
    /// * `request` - The thread and run request parameters
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{runs::RunsApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::runs::CreateThreadAndRunRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = RunsApi::new("your-api-key")?;
    /// let request = CreateThreadAndRunRequest::builder()
    ///     .assistant_id("asst_abc123")
    ///     .instructions("Please help me with this task.")
    ///     .build()?;
    ///
    /// let run = api.create_thread_and_run_stream(request).await?;
    /// println!("Created streaming thread and run: {} in thread {}", run.id, run.thread_id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_thread_and_run_stream(
        &self,
        request: CreateThreadAndRunRequest,
    ) -> Result<Run> {
        // Add stream: true to the JSON body
        let mut request_json = serde_json::to_value(&request).map_err(OpenAIError::Json)?;
        request_json["stream"] = serde_json::Value::Bool(true);

        self.http_client
            .post("/v1/threads/runs", &request_json)
            .await
    }
}
