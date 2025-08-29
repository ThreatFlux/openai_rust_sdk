//! Thread management functionality for the Threads API
//!
//! This module contains all methods related to managing conversation threads,
//! including creation, retrieval, modification, and deletion operations.

use super::{
    client::ThreadsApi,
    types::{DeletionStatus, Thread, ThreadRequest},
};
use crate::api::base::validate_request;
use crate::constants::endpoints;
use crate::error::Result;

impl ThreadsApi {
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        validate_request(&request)?;
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        let path = endpoints::threads::by_id(&thread_id);
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        validate_request(&request)?;

        let thread_id = thread_id.into();
        let path = endpoints::threads::by_id(&thread_id);
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
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
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
        let path = endpoints::threads::by_id(&thread_id);
        self.http_client.delete_with_beta(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::common::ApiClientConstructors;
    use crate::models::threads::ThreadRequest;

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
}
