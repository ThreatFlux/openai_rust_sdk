//! Message management functionality for the Threads API
//!
//! This module contains all methods related to managing messages within threads,
//! including creation, retrieval, modification, and listing operations.

use super::{
    client::ThreadsApi,
    types::{ListMessagesParams, ListMessagesResponse, Message, MessageRequest},
};
use crate::api::base::validate_request;
use crate::constants::endpoints;
use crate::error::Result;

impl ThreadsApi {
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        validate_request(&request)?;

        let thread_id = thread_id.into();
        let path = endpoints::threads::messages(&thread_id);
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        let path = endpoints::threads::message_by_id(&thread_id, &message_id);
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        validate_request(&request)?;

        let thread_id = thread_id.into();
        let message_id = message_id.into();
        let path = endpoints::threads::message_by_id(&thread_id, &message_id);
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        let path = endpoints::threads::messages(&thread_id);

        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query_and_beta(&path, &query_params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::common::ApiClientConstructors;
    use crate::models::threads::{MessageRequest, MessageRole};

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
