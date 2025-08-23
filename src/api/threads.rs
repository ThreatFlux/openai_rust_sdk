//! # OpenAI Threads & Messages API Client
//!
//! This module provides a complete implementation of OpenAI's Threads API, which allows you to
//! create conversation threads and manage messages within those threads.
//!
//! ## Features
//!
//! - **Thread Management**: Create, retrieve, modify, and delete conversation threads
//! - **Message Management**: Add, retrieve, modify, and list messages within threads
//! - **File Management**: Attach files to messages and manage message files
//! - **Pagination**: List messages with cursor-based pagination
//! - **Content Types**: Support for text and image content in messages
//! - **Annotations**: Handle file citations and file paths within message content
//! - **Error Handling**: Comprehensive error handling with detailed messages
//!
//! ## Thread Capabilities
//!
//! Threads provide a way to organize conversations and can:
//! - Store multiple messages in chronological order
//! - Maintain conversation context across multiple interactions
//! - Support file attachments for analysis and processing
//! - Store custom metadata for organization and tracking
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::threads::ThreadsApi;
//! use openai_rust_sdk::models::threads::{ThreadRequest, MessageRequest, MessageRole};
//!
//! # tokio_test::block_on(async {
//! let api = ThreadsApi::new("your-api-key")?;
//!
//! // Create a new thread
//! let thread_request = ThreadRequest::builder()
//!     .metadata_pair("purpose", "customer_support")
//!     .build();
//!
//! let thread = api.create_thread(thread_request).await?;
//! println!("Created thread: {}", thread.id);
//!
//! // Add a message to the thread
//! let message_request = MessageRequest::builder()
//!     .role(MessageRole::User)
//!     .content("Hello, I need help with my account.")
//!     .build()?;
//!
//! let message = api.create_message(&thread.id, message_request).await?;
//! println!("Created message: {}", message.id);
//!
//! // List messages in the thread
//! let messages = api.list_messages(&thread.id, None).await?;
//! println!("Found {} messages", messages.data.len());
//!
//! // Delete the thread
//! let deleted = api.delete_thread(&thread.id).await?;
//! println!("Deleted: {}", deleted.deleted);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use crate::models::threads::{
    DeletionStatus, ListMessageFilesResponse, ListMessagesParams, ListMessagesResponse, Message,
    MessageFile, MessageRequest, Thread, ThreadRequest,
};

/// `OpenAI` Threads API client for managing conversation threads and messages
#[derive(Debug, Clone)]
pub struct ThreadsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ThreadsApi {
    /// Creates a new Threads API client
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    ///
    /// let api = ThreadsApi::new("your-api-key")?;
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// ```
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
        })
    }

    /// Creates a new Threads API client with custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    /// * `base_url` - Custom base URL for the API
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    ///
    /// let api = ThreadsApi::with_base_url("your-api-key", "https://custom-api.example.com")?;
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// ```
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    // Thread Management Methods

    /// Create a new conversation thread
    ///
    /// # Arguments
    ///
    /// * `request` - The thread creation request
    ///
    /// # Returns
    ///
    /// Returns the created thread object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    /// use openai_rust_sdk::models::threads::ThreadRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let thread_request = ThreadRequest::builder()
    ///     .metadata_pair("purpose", "customer_support")
    ///     .build();
    ///
    /// let thread = api.create_thread(thread_request).await?;
    /// println!("Created thread: {}", thread.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_thread(&self, request: ThreadRequest) -> Result<Thread> {
        // Validate request
        request.validate().map_err(OpenAIError::InvalidRequest)?;
        self.http_client
            .post_with_beta("/v1/threads", &request)
            .await
    }

    /// Retrieve a thread by its ID
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to retrieve
    ///
    /// # Returns
    ///
    /// Returns the thread object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let thread = api.retrieve_thread("thread_abc123").await?;
    /// println!("Thread metadata: {:?}", thread.metadata);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn retrieve_thread(&self, thread_id: impl Into<String>) -> Result<Thread> {
        let thread_id = thread_id.into();
        let path = format!("/v1/threads/{thread_id}");
        self.http_client.get_with_beta(&path).await
    }

    /// Modify a thread's metadata
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to modify
    /// * `request` - The thread modification request
    ///
    /// # Returns
    ///
    /// Returns the modified thread object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    /// use openai_rust_sdk::models::threads::ThreadRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let thread_request = ThreadRequest::builder()
    ///     .metadata_pair("status", "resolved")
    ///     .build();
    ///
    /// let thread = api.modify_thread("thread_abc123", thread_request).await?;
    /// println!("Modified thread: {}", thread.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn modify_thread(
        &self,
        thread_id: impl Into<String>,
        request: ThreadRequest,
    ) -> Result<Thread> {
        // Validate request
        request.validate().map_err(OpenAIError::InvalidRequest)?;

        let thread_id = thread_id.into();
        let path = format!("/v1/threads/{thread_id}");
        self.http_client.post_with_beta(&path, &request).await
    }

    /// Delete a thread
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to delete
    ///
    /// # Returns
    ///
    /// Returns the deletion status
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let deleted = api.delete_thread("thread_abc123").await?;
    /// println!("Deleted: {}", deleted.deleted);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn delete_thread(&self, thread_id: impl Into<String>) -> Result<DeletionStatus> {
        let thread_id = thread_id.into();
        let path = format!("/v1/threads/{thread_id}");
        self.http_client.delete_with_beta(&path).await
    }

    // Message Management Methods

    /// Create a message in a thread
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to add the message to
    /// * `request` - The message creation request
    ///
    /// # Returns
    ///
    /// Returns the created message object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    /// use openai_rust_sdk::models::threads::{MessageRequest, MessageRole};
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let message_request = MessageRequest::builder()
    ///     .role(MessageRole::User)
    ///     .content("Hello, I need help with my account.")
    ///     .build()?;
    ///
    /// let message = api.create_message("thread_abc123", message_request).await?;
    /// println!("Created message: {}", message.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_message(
        &self,
        thread_id: impl Into<String>,
        request: MessageRequest,
    ) -> Result<Message> {
        // Validate request
        request.validate().map_err(OpenAIError::InvalidRequest)?;

        let thread_id = thread_id.into();
        let path = format!("/v1/threads/{thread_id}/messages");
        self.http_client.post_with_beta(&path, &request).await
    }

    /// Retrieve a message from a thread
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread that contains the message
    /// * `message_id` - The ID of the message to retrieve
    ///
    /// # Returns
    ///
    /// Returns the message object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let message = api.retrieve_message("thread_abc123", "msg_abc123").await?;
    /// println!("Message content: {:?}", message.content);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn retrieve_message(
        &self,
        thread_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Result<Message> {
        let thread_id = thread_id.into();
        let message_id = message_id.into();
        let path = format!("/v1/threads/{thread_id}/messages/{message_id}");
        self.http_client.get_with_beta(&path).await
    }

    /// Modify a message's metadata
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread that contains the message
    /// * `message_id` - The ID of the message to modify
    /// * `request` - The message modification request
    ///
    /// # Returns
    ///
    /// Returns the modified message object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    /// use openai_rust_sdk::models::threads::{MessageRequest, MessageRole};
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let message_request = MessageRequest::builder()
    ///     .role(MessageRole::User)
    ///     .content("Updated content")
    ///     .metadata_pair("priority", "high")
    ///     .build()?;
    ///
    /// let message = api.modify_message("thread_abc123", "msg_abc123", message_request).await?;
    /// println!("Modified message: {}", message.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn modify_message(
        &self,
        thread_id: impl Into<String>,
        message_id: impl Into<String>,
        request: MessageRequest,
    ) -> Result<Message> {
        // Validate request
        request.validate().map_err(OpenAIError::InvalidRequest)?;

        let thread_id = thread_id.into();
        let message_id = message_id.into();
        let path = format!("/v1/threads/{thread_id}/messages/{message_id}");
        self.http_client.post_with_beta(&path, &request).await
    }

    /// List messages in a thread
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to list messages from
    /// * `params` - Optional parameters for pagination and filtering
    ///
    /// # Returns
    ///
    /// Returns a list of message objects with pagination information
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    /// use openai_rust_sdk::models::threads::{ListMessagesParams, SortOrder};
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let params = ListMessagesParams::new()
    ///     .limit(10)
    ///     .order(SortOrder::Desc);
    ///
    /// let messages = api.list_messages("thread_abc123", Some(params)).await?;
    /// println!("Found {} messages", messages.data.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_messages(
        &self,
        thread_id: impl Into<String>,
        params: Option<ListMessagesParams>,
    ) -> Result<ListMessagesResponse> {
        let thread_id = thread_id.into();
        let path = format!("/v1/threads/{thread_id}/messages");

        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query_and_beta(&path, &query_params)
            .await
    }

    // Message File Management Methods

    /// List files attached to a message
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread that contains the message
    /// * `message_id` - The ID of the message to list files for
    ///
    /// # Returns
    ///
    /// Returns a list of message file objects
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let files = api.list_message_files("thread_abc123", "msg_abc123").await?;
    /// println!("Found {} files", files.data.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_message_files(
        &self,
        thread_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Result<ListMessageFilesResponse> {
        let thread_id = thread_id.into();
        let message_id = message_id.into();
        let path = format!("/v1/threads/{thread_id}/messages/{message_id}/files");
        self.http_client.get_with_beta(&path).await
    }

    /// Retrieve a specific file attached to a message
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread that contains the message
    /// * `message_id` - The ID of the message that contains the file
    /// * `file_id` - The ID of the file to retrieve
    ///
    /// # Returns
    ///
    /// Returns the message file object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::threads::ThreadsApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = ThreadsApi::new("your-api-key")?;
    /// let file = api.retrieve_message_file("thread_abc123", "msg_abc123", "file_abc123").await?;
    /// println!("File created at: {}", file.created_at);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn retrieve_message_file(
        &self,
        thread_id: impl Into<String>,
        message_id: impl Into<String>,
        file_id: impl Into<String>,
    ) -> Result<MessageFile> {
        let thread_id = thread_id.into();
        let message_id = message_id.into();
        let file_id = file_id.into();
        let path = format!("/v1/threads/{thread_id}/messages/{message_id}/files/{file_id}");
        self.http_client.get_with_beta(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::threads::{MessageRole, ThreadRequest};

    #[test]
    fn test_threads_api_creation() {
        let api = ThreadsApi::new("test-api-key").unwrap();
        assert_eq!(api.http_client.api_key(), "test-api-key");
        assert_eq!(api.http_client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_threads_api_with_custom_url() {
        let api = ThreadsApi::with_base_url("test-api-key", "https://custom.api.com").unwrap();
        assert_eq!(api.http_client.api_key(), "test-api-key");
        assert_eq!(api.http_client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_create_headers() {
        let api = ThreadsApi::new("test-api-key").unwrap();
        let headers = api.http_client.build_headers_with_beta().unwrap();

        assert!(headers.contains_key("Content-Type"));
        assert!(headers.contains_key("Authorization"));
        assert!(headers.contains_key("OpenAI-Beta"));
    }

    #[test]
    fn test_thread_request_validation() {
        let _api = ThreadsApi::new("test-api-key").unwrap();

        // Valid request should not error during validation
        let valid_request = ThreadRequest::builder()
            .metadata_pair("test", "value")
            .build();

        // This would be tested in an async context in real integration tests
        assert!(valid_request.validate().is_ok());
    }

    #[test]
    fn test_message_request_validation() {
        let _api = ThreadsApi::new("test-api-key").unwrap();

        // Valid request should not error during validation
        let valid_request = MessageRequest::builder()
            .role(MessageRole::User)
            .content("Test message")
            .build()
            .unwrap();

        assert!(valid_request.validate().is_ok());
    }
}
