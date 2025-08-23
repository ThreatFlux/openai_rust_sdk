//! # OpenAI Assistants API Client
//!
//! This module provides a complete implementation of OpenAI's Assistants API, which allows you to
//! build AI assistants within your applications. Assistants can call models and use tools.
//!
//! ## Features
//!
//! - **Assistant Management**: Create, retrieve, modify, and delete assistants
//! - **Tool Integration**: Support for Code Interpreter, Retrieval, and Function calling
//! - **File Management**: Attach files to assistants for retrieval and analysis
//! - **Pagination**: List assistants with cursor-based pagination
//! - **Error Handling**: Comprehensive error handling with detailed messages
//!
//! ## Assistant Capabilities
//!
//! Assistants can use three types of tools:
//! - **Code Interpreter**: Run Python code in a sandboxed environment
//! - **Retrieval**: Search through uploaded files and documents
//! - **Functions**: Call custom functions you define
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::assistants::AssistantsApi;
//! use openai_rust_sdk::models::assistants::{AssistantRequest, AssistantTool};
//!
//! # tokio_test::block_on(async {
//! let api = AssistantsApi::new("your-api-key")?;
//!
//! // Create a new assistant with code interpreter
//! let assistant_request = AssistantRequest::builder()
//!     .model("gpt-4")
//!     .name("Data Analyst")
//!     .description("Analyzes data and creates visualizations")
//!     .instructions("You are a data analyst. Help users analyze data.")
//!     .tool(AssistantTool::code_interpreter())
//!     .build()?;
//!
//! let assistant = api.create_assistant(assistant_request).await?;
//! println!("Created assistant: {}", assistant.id);
//!
//! // Retrieve the assistant
//! let retrieved = api.retrieve_assistant(&assistant.id).await?;
//! println!("Assistant name: {:?}", retrieved.name);
//!
//! // List all assistants
//! let assistants = api.list_assistants(None).await?;
//! println!("Found {} assistants", assistants.data.len());
//!
//! // Delete the assistant
//! let deleted = api.delete_assistant(&assistant.id).await?;
//! println!("Deleted: {}", deleted.deleted);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use crate::models::assistants::{
    Assistant, AssistantRequest, DeletionStatus, ListAssistantsParams, ListAssistantsResponse,
};

/// `OpenAI` Assistants API client for managing AI assistants
#[derive(Debug, Clone)]
pub struct AssistantsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl AssistantsApi {
    /// Creates a new Assistants API client
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::assistants::AssistantsApi;
    ///
    /// let api = AssistantsApi::new("your-api-key")?;
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// ```
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
        })
    }

    /// Creates a new Assistants API client with custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    /// * `base_url` - Custom base URL for the API
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Creates a new assistant
    ///
    /// Build an AI assistant that can answer questions and perform tasks using models,
    /// tools, and knowledge.
    ///
    /// # Arguments
    ///
    /// * `request` - The assistant configuration including model, instructions, and tools
    ///
    /// # Returns
    ///
    /// Returns an `Assistant` object with the created assistant's details
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::assistants::AssistantsApi;
    /// use openai_rust_sdk::models::assistants::{AssistantRequest, AssistantTool};
    ///
    /// # tokio_test::block_on(async {
    /// let api = AssistantsApi::new("your-api-key")?;
    /// let request = AssistantRequest::builder()
    ///     .model("gpt-4")
    ///     .name("Math Tutor")
    ///     .instructions("You are a personal math tutor.")
    ///     .tool(AssistantTool::code_interpreter())
    ///     .build()?;
    ///
    /// let assistant = api.create_assistant(request).await?;
    /// println!("Created assistant: {}", assistant.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_assistant(&self, request: AssistantRequest) -> Result<Assistant> {
        // Validate the request
        request.validate().map_err(OpenAIError::InvalidRequest)?;

        self.http_client
            .post_with_beta("/v1/assistants", &request)
            .await
    }

    /// Retrieves an assistant by ID
    ///
    /// # Arguments
    ///
    /// * `assistant_id` - The ID of the assistant to retrieve
    ///
    /// # Returns
    ///
    /// Returns the `Assistant` object matching the specified ID
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::assistants::AssistantsApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = AssistantsApi::new("your-api-key")?;
    /// let assistant = api.retrieve_assistant("asst_abc123").await?;
    /// println!("Assistant name: {:?}", assistant.name);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn retrieve_assistant(&self, assistant_id: impl Into<String>) -> Result<Assistant> {
        let assistant_id = assistant_id.into();
        let path = format!("/v1/assistants/{assistant_id}");
        self.http_client.get_with_beta(&path).await
    }

    /// Modifies an existing assistant
    ///
    /// # Arguments
    ///
    /// * `assistant_id` - The ID of the assistant to modify
    /// * `request` - The updated assistant configuration
    ///
    /// # Returns
    ///
    /// Returns the modified `Assistant` object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::assistants::AssistantsApi;
    /// use openai_rust_sdk::models::assistants::AssistantRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = AssistantsApi::new("your-api-key")?;
    /// let update_request = AssistantRequest::builder()
    ///     .model("gpt-4")
    ///     .name("Updated Assistant")
    ///     .instructions("Updated instructions")
    ///     .build()?;
    ///
    /// let assistant = api.modify_assistant("asst_abc123", update_request).await?;
    /// println!("Modified assistant: {}", assistant.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn modify_assistant(
        &self,
        assistant_id: impl Into<String>,
        request: AssistantRequest,
    ) -> Result<Assistant> {
        // Validate the request
        request.validate().map_err(OpenAIError::InvalidRequest)?;

        let assistant_id = assistant_id.into();
        let path = format!("/v1/assistants/{assistant_id}");
        self.http_client.post_with_beta(&path, &request).await
    }

    /// Deletes an assistant
    ///
    /// # Arguments
    ///
    /// * `assistant_id` - The ID of the assistant to delete
    ///
    /// # Returns
    ///
    /// Returns a `DeletionStatus` indicating whether the deletion was successful
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::assistants::AssistantsApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = AssistantsApi::new("your-api-key")?;
    /// let result = api.delete_assistant("asst_abc123").await?;
    /// println!("Deleted: {}", result.deleted);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn delete_assistant(
        &self,
        assistant_id: impl Into<String>,
    ) -> Result<DeletionStatus> {
        let assistant_id = assistant_id.into();
        let path = format!("/v1/assistants/{assistant_id}");
        self.http_client.delete_with_beta(&path).await
    }

    /// Lists assistants
    ///
    /// Returns a list of assistants with support for pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a `ListAssistantsResponse` containing the list of assistants
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::assistants::AssistantsApi;
    /// use openai_rust_sdk::models::assistants::{ListAssistantsParams, SortOrder};
    ///
    /// # tokio_test::block_on(async {
    /// let api = AssistantsApi::new("your-api-key")?;
    ///
    /// // List first 10 assistants
    /// let params = ListAssistantsParams::new()
    ///     .limit(10)
    ///     .order(SortOrder::Desc);
    /// let assistants = api.list_assistants(Some(params)).await?;
    /// println!("Found {} assistants", assistants.data.len());
    ///
    /// // List all assistants (using default parameters)
    /// let all_assistants = api.list_assistants(None).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_assistants(
        &self,
        params: Option<ListAssistantsParams>,
    ) -> Result<ListAssistantsResponse> {
        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query_and_beta("/v1/assistants", &query_params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::assistants::SortOrder;
    use std::collections::HashMap;

    #[test]
    fn test_assistants_api_creation() {
        let api = AssistantsApi::new("test-key").unwrap();
        assert_eq!(api.http_client.api_key(), "test-key");
        assert_eq!(api.http_client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_assistants_api_with_custom_base_url() {
        let api = AssistantsApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.http_client.api_key(), "test-key");
        assert_eq!(api.http_client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_build_headers() {
        let api = AssistantsApi::new("test-key").unwrap();
        let headers = api.http_client.build_headers_with_beta().unwrap();

        assert!(headers.contains_key("Authorization"));
        assert!(headers.contains_key("Content-Type"));
        assert!(headers.contains_key("OpenAI-Beta"));

        let auth_header = headers.get("Authorization").unwrap();
        assert_eq!(auth_header, "Bearer test-key");

        let beta_header = headers.get("OpenAI-Beta").unwrap();
        assert_eq!(beta_header, "assistants=v2");
    }

    #[test]
    fn test_list_params_query_building() {
        let params = ListAssistantsParams::new()
            .limit(10)
            .order(SortOrder::Asc)
            .after("asst_123");

        assert_eq!(params.limit, Some(10));
        assert_eq!(params.order, Some(SortOrder::Asc));
        assert_eq!(params.after, Some("asst_123".to_string()));
    }

    #[tokio::test]
    async fn test_request_validation() {
        let api = AssistantsApi::new("test-key").unwrap();

        // Test invalid request (name too long)
        let invalid_request = AssistantRequest {
            model: "gpt-4".to_string(),
            name: Some("a".repeat(257)),
            description: None,
            instructions: None,
            tools: Vec::new(),
            file_ids: Vec::new(),
            metadata: HashMap::new(),
        };

        let result = api.create_assistant(invalid_request).await;
        assert!(result.is_err());

        if let Err(OpenAIError::InvalidRequest(msg)) = result {
            assert!(msg.contains("name cannot exceed 256 characters"));
        } else {
            panic!("Expected InvalidRequest error");
        }
    }
}
