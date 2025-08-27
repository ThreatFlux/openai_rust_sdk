//! Message file management functionality for the Threads API
//!
//! This module contains all methods related to managing files attached to messages
//! within threads, including listing and retrieving file operations.

use super::{
    client::ThreadsApi,
    types::{ListMessageFilesResponse, MessageFile},
};
use crate::constants::endpoints;
use crate::error::Result;

impl ThreadsApi {
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        let path = endpoints::threads::message_files(&thread_id, &message_id);
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        let path = endpoints::threads::message_file_by_id(&thread_id, &message_id, &file_id);
        self.http_client.get_with_beta(&path).await
    }
}
