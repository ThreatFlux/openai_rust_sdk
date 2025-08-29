//! # OpenAI Vector Stores API Client
//!
//! This module provides a complete implementation of OpenAI's Vector Stores API for creating,
//! managing, and using vector stores for retrieval-augmented generation (RAG).
//!
//! ## Features
//!
//! - **Vector Store Management**: Create, retrieve, modify, and delete vector stores
//! - **File Operations**: Attach files, manage associations, and batch operations
//! - **Status Monitoring**: Track processing status and file counts
//! - **Expiration Management**: Configure automatic cleanup policies
//! - **Chunking Strategies**: Control how files are processed and embedded
//!
//! ## Key Capabilities
//!
//! - **Automatic Processing**: Files are automatically chunked and embedded when added
//! - **Batch Operations**: Upload multiple files simultaneously for efficiency
//! - **Retrieval Integration**: Vector stores integrate seamlessly with Assistants API
//! - **Flexible Configuration**: Support for custom chunking and expiration policies
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
//! use openai_rust_sdk::models::vector_stores::{VectorStoreRequest, ExpirationPolicy};
//!
//! # tokio_test::block_on(async {
//! let api = VectorStoresApi::new("your-api-key")?;
//!
//! // Create a vector store
//! let request = VectorStoreRequest::builder()
//!     .name("Knowledge Base")
//!     .expires_after(ExpirationPolicy::new_days(30))
//!     .build();
//! let vector_store = api.create_vector_store(request).await?;
//!
//! // Add files to the vector store
//! let file_ids = vec!["file-123".to_string(), "file-456".to_string()];
//! let batch = api.create_vector_store_file_batch(&vector_store.id, file_ids).await?;
//!
//! // Monitor processing status
//! let status = api.retrieve_vector_store_file_batch(&vector_store.id, &batch.id).await?;
//! println!("Batch status: {:?}", status.status);
//! # Ok::<(), openai_rust_sdk::OpenAIError>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::constants::endpoints;
use crate::error::{OpenAIError, Result};
use crate::models::vector_stores::{
    ListVectorStoreFilesParams, ListVectorStoreFilesResponse, ListVectorStoresParams,
    ListVectorStoresResponse, QueryParamBuilder, VectorStore, VectorStoreDeleteResponse,
    VectorStoreFile, VectorStoreFileBatch, VectorStoreFileBatchRequest,
    VectorStoreFileDeleteResponse, VectorStoreFileRequest, VectorStoreRequest,
};
use std::collections::HashMap;

/// `OpenAI` Vector Stores API client for vector store management operations
#[derive(Debug, Clone)]
pub struct VectorStoresApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for VectorStoresApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl VectorStoresApi {
    /// Get the API key (for testing)
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
    }

    /// Get the base URL (for testing)
    #[must_use]
    pub fn base_url(&self) -> &str {
        self.http_client.base_url()
    }

    /// Creates a vector store
    ///
    /// # Arguments
    ///
    /// * `request` - The vector store creation request
    ///
    /// # Returns
    ///
    /// Returns a `VectorStore` object containing the created vector store's metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::vector_stores::{VectorStoreRequest, ExpirationPolicy};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let request = VectorStoreRequest::builder()
    ///     .name("Knowledge Base")
    ///     .expires_after(ExpirationPolicy::new_days(30))
    ///     .build();
    /// let vector_store = api.create_vector_store(request).await?;
    /// println!("Created vector store: {}", vector_store.id);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn create_vector_store(&self, request: VectorStoreRequest) -> Result<VectorStore> {
        self.http_client.post("/v1/vector_stores", &request).await
    }

    /// Lists vector stores
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a `ListVectorStoresResponse` containing the list of vector stores
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::vector_stores::ListVectorStoresParams;
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    ///
    /// // List all vector stores
    /// let all_stores = api.list_vector_stores(None).await?;
    ///
    /// // List with pagination
    /// let params = ListVectorStoresParams::new().with_limit(10);
    /// let limited_stores = api.list_vector_stores(Some(params)).await?;
    ///
    /// println!("Found {} total stores", all_stores.data.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn list_vector_stores(
        &self,
        params: Option<ListVectorStoresParams>,
    ) -> Result<ListVectorStoresResponse> {
        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query("/v1/vector_stores", &query_params)
            .await
    }

    /// Retrieves a vector store by its ID
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `VectorStore` object containing the vector store's metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let vector_store = api.retrieve_vector_store("vs-abc123").await?;
    /// let name = vector_store.name.clone().unwrap_or_default();
    /// println!("Vector store: {} ({})", name, vector_store.usage_human_readable());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn retrieve_vector_store(
        &self,
        vector_store_id: impl Into<String>,
    ) -> Result<VectorStore> {
        let vector_store_id = vector_store_id.into();
        let path = endpoints::vector_stores::by_id(&vector_store_id);
        self.http_client.get(&path).await
    }

    /// Modifies a vector store
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to modify
    /// * `request` - The modification request containing updated fields
    ///
    /// # Returns
    ///
    /// Returns the updated `VectorStore` object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::vector_stores::VectorStoreRequest;
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let request = VectorStoreRequest::builder()
    ///     .name("Updated Knowledge Base")
    ///     .add_metadata("version", "2.0")
    ///     .build();
    /// let updated_store = api.modify_vector_store("vs-abc123", request).await?;
    /// println!("Updated vector store: {}", updated_store.name.unwrap_or_default());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn modify_vector_store(
        &self,
        vector_store_id: impl Into<String>,
        request: VectorStoreRequest,
    ) -> Result<VectorStore> {
        let vector_store_id = vector_store_id.into();
        let path = endpoints::vector_stores::by_id(&vector_store_id);
        self.http_client.post(&path, &request).await
    }

    /// Deletes a vector store
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to delete
    ///
    /// # Returns
    ///
    /// Returns a `VectorStoreDeleteResponse` indicating whether the deletion was successful
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let result = api.delete_vector_store("vs-abc123").await?;
    /// if result.deleted {
    ///     println!("Vector store {} was successfully deleted", result.id);
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn delete_vector_store(
        &self,
        vector_store_id: impl Into<String>,
    ) -> Result<VectorStoreDeleteResponse> {
        let vector_store_id = vector_store_id.into();
        let path = endpoints::vector_stores::by_id(&vector_store_id);
        self.http_client.delete(&path).await
    }

    /// Creates a vector store file by attaching a file to a vector store
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to attach the file to
    /// * `request` - The file attachment request
    ///
    /// # Returns
    ///
    /// Returns a `VectorStoreFile` object representing the file attachment
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::vector_stores::{VectorStoreFileRequest, ChunkingStrategy};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let request = VectorStoreFileRequest::new("file-abc123")
    ///     .with_chunking_strategy(ChunkingStrategy::static_chunking(512, 50));
    /// let file_attachment = api.create_vector_store_file("vs-abc123", request).await?;
    /// println!("Attached file: {}", file_attachment.id);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn create_vector_store_file(
        &self,
        vector_store_id: impl Into<String>,
        request: VectorStoreFileRequest,
    ) -> Result<VectorStoreFile> {
        let vector_store_id = vector_store_id.into();
        let path = endpoints::vector_stores::files(&vector_store_id);
        self.http_client.post(&path, &request).await
    }

    /// Lists files in a vector store
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a `ListVectorStoreFilesResponse` containing the list of files
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::vector_stores::{ListVectorStoreFilesParams, VectorStoreFileStatus};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    ///
    /// // List all files
    /// let all_files = api.list_vector_store_files("vs-abc123", None).await?;
    ///
    /// // List only completed files
    /// let params = ListVectorStoreFilesParams::new()
    ///     .with_filter(VectorStoreFileStatus::Completed);
    /// let completed_files = api.list_vector_store_files("vs-abc123", Some(params)).await?;
    ///
    /// println!("Found {} total files, {} completed",
    ///          all_files.data.len(), completed_files.data.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn list_vector_store_files(
        &self,
        vector_store_id: impl Into<String>,
        params: Option<ListVectorStoreFilesParams>,
    ) -> Result<ListVectorStoreFilesResponse> {
        let vector_store_id = vector_store_id.into();
        let path = endpoints::vector_stores::files(&vector_store_id);

        let mut query_params = Vec::new();

        // Add query parameters if provided
        if let Some(params) = params {
            query_params = params.to_query_params();
        }

        self.http_client.get_with_query(&path, &query_params).await
    }

    /// Retrieves a vector store file
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `file_id` - The ID of the file to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `VectorStoreFile` object containing the file's metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let file = api.retrieve_vector_store_file("vs-abc123", "file-def456").await?;
    /// println!("File status: {:?}", file.status);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn retrieve_vector_store_file(
        &self,
        vector_store_id: impl Into<String>,
        file_id: impl Into<String>,
    ) -> Result<VectorStoreFile> {
        let vector_store_id = vector_store_id.into();
        let file_id = file_id.into();
        let path = endpoints::vector_stores::file_by_id(&vector_store_id, &file_id);
        self.http_client.get(&path).await
    }

    /// Deletes a vector store file
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `file_id` - The ID of the file to delete
    ///
    /// # Returns
    ///
    /// Returns a `VectorStoreFileDeleteResponse` indicating whether the deletion was successful
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let result = api.delete_vector_store_file("vs-abc123", "file-def456").await?;
    /// if result.deleted {
    ///     println!("File {} was successfully removed", result.id);
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn delete_vector_store_file(
        &self,
        vector_store_id: impl Into<String>,
        file_id: impl Into<String>,
    ) -> Result<VectorStoreFileDeleteResponse> {
        let vector_store_id = vector_store_id.into();
        let file_id = file_id.into();
        let path = endpoints::vector_stores::file_by_id(&vector_store_id, &file_id);
        self.http_client.delete(&path).await
    }

    /// Creates a vector store file batch for uploading multiple files at once
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `file_ids` - Vector of file IDs to add to the vector store
    ///
    /// # Returns
    ///
    /// Returns a `VectorStoreFileBatch` object representing the batch operation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let file_ids = vec!["file-123".to_string(), "file-456".to_string(), "file-789".to_string()];
    /// let batch = api.create_vector_store_file_batch("vs-abc123", file_ids).await?;
    /// println!("Created batch: {} with {} files", batch.id, batch.file_counts.total);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn create_vector_store_file_batch(
        &self,
        vector_store_id: impl Into<String>,
        file_ids: Vec<String>,
    ) -> Result<VectorStoreFileBatch> {
        let vector_store_id = vector_store_id.into();
        let request = VectorStoreFileBatchRequest::new(file_ids);
        let path = endpoints::vector_stores::file_batches(&vector_store_id);
        self.http_client.post(&path, &request).await
    }

    /// Creates a vector store file batch with custom request
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `request` - The batch creation request with custom options
    ///
    /// # Returns
    ///
    /// Returns a `VectorStoreFileBatch` object representing the batch operation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::vector_stores::{VectorStoreFileBatchRequest, ChunkingStrategy};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let file_ids = vec!["file-123".to_string(), "file-456".to_string()];
    /// let request = VectorStoreFileBatchRequest::new(file_ids)
    ///     .with_chunking_strategy(ChunkingStrategy::static_chunking(1024, 100));
    /// let batch = api.create_vector_store_file_batch_with_request("vs-abc123", request).await?;
    /// println!("Created batch: {}", batch.id);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn create_vector_store_file_batch_with_request(
        &self,
        vector_store_id: impl Into<String>,
        request: VectorStoreFileBatchRequest,
    ) -> Result<VectorStoreFileBatch> {
        let vector_store_id = vector_store_id.into();
        let path = endpoints::vector_stores::file_batches(&vector_store_id);
        self.http_client.post(&path, &request).await
    }

    /// Retrieves a vector store file batch
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `batch_id` - The ID of the batch to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `VectorStoreFileBatch` object containing the batch status
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let batch = api.retrieve_vector_store_file_batch("vs-abc123", "batch-def456").await?;
    /// println!("Batch status: {:?}", batch.status);
    /// println!("Files processed: {}/{}", batch.file_counts.completed, batch.file_counts.total);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn retrieve_vector_store_file_batch(
        &self,
        vector_store_id: impl Into<String>,
        batch_id: impl Into<String>,
    ) -> Result<VectorStoreFileBatch> {
        let vector_store_id = vector_store_id.into();
        let batch_id = batch_id.into();
        let path = endpoints::vector_stores::file_batch_by_id(&vector_store_id, &batch_id);
        self.http_client.get(&path).await
    }

    /// Cancels a vector store file batch
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `batch_id` - The ID of the batch to cancel
    ///
    /// # Returns
    ///
    /// Returns the updated `VectorStoreFileBatch` object with cancelled status
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let cancelled_batch = api.cancel_vector_store_file_batch("vs-abc123", "batch-def456").await?;
    /// println!("Batch cancelled: {:?}", cancelled_batch.status);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn cancel_vector_store_file_batch(
        &self,
        vector_store_id: impl Into<String>,
        batch_id: impl Into<String>,
    ) -> Result<VectorStoreFileBatch> {
        let vector_store_id = vector_store_id.into();
        let batch_id = batch_id.into();
        let path = endpoints::vector_stores::cancel_file_batch(&vector_store_id, &batch_id);
        // Use an empty object for the body since this is a POST with no actual data
        let empty_body = serde_json::json!({});
        self.http_client.post(&path, &empty_body).await
    }

    /// Lists files in a vector store file batch
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `batch_id` - The ID of the batch
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a `ListVectorStoreFilesResponse` containing the files in the batch
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    /// use openai_rust_sdk::models::vector_stores::{ListVectorStoreFilesParams, VectorStoreFileStatus};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    ///
    /// // List all files in the batch
    /// let all_files = api.list_vector_store_files_in_batch("vs-abc123", "batch-def456", None).await?;
    ///
    /// // List only failed files in the batch
    /// let params = ListVectorStoreFilesParams::new()
    ///     .with_filter(VectorStoreFileStatus::Failed);
    /// let failed_files = api.list_vector_store_files_in_batch("vs-abc123", "batch-def456", Some(params)).await?;
    ///
    /// println!("Batch has {} total files, {} failed",
    ///          all_files.data.len(), failed_files.data.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn list_vector_store_files_in_batch(
        &self,
        vector_store_id: impl Into<String>,
        batch_id: impl Into<String>,
        params: Option<ListVectorStoreFilesParams>,
    ) -> Result<ListVectorStoreFilesResponse> {
        let vector_store_id = vector_store_id.into();
        let batch_id = batch_id.into();
        let path = endpoints::vector_stores::file_batch_files(&vector_store_id, &batch_id);

        let mut query_params = Vec::new();

        // Add query parameters if provided
        if let Some(params) = params {
            query_params = params.to_query_params();
        }

        self.http_client.get_with_query(&path, &query_params).await
    }

    /// Convenience method to wait for a vector store to be ready
    ///
    /// This method polls the vector store status until it's completed, failed, or cancelled.
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to wait for
    /// * `max_wait_seconds` - Maximum time to wait before giving up (default: 300 seconds)
    /// * `poll_interval_seconds` - How often to check the status (default: 5 seconds)
    ///
    /// # Returns
    ///
    /// Returns the final `VectorStore` object when processing is complete
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let vector_store = api.wait_for_vector_store_ready("vs-abc123", Some(600), Some(10)).await?;
    /// println!("Vector store is ready: {}", vector_store.is_ready());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn wait_for_vector_store_ready(
        &self,
        vector_store_id: impl Into<String>,
        max_wait_seconds: Option<u64>,
        poll_interval_seconds: Option<u64>,
    ) -> Result<VectorStore> {
        let vector_store_id = vector_store_id.into();
        let max_wait = max_wait_seconds.unwrap_or(300);
        let poll_interval = poll_interval_seconds.unwrap_or(5);
        let start_time = std::time::Instant::now();

        loop {
            let vector_store = self.retrieve_vector_store(&vector_store_id).await?;

            // Check if processing is complete
            if vector_store.is_ready() || vector_store.has_failed() {
                return Ok(vector_store);
            }

            // Check timeout
            if start_time.elapsed().as_secs() >= max_wait {
                return Err(OpenAIError::Timeout(format!(
                    "Vector store {vector_store_id} did not become ready within {max_wait} seconds"
                )));
            }

            // Wait before next poll
            tokio::time::sleep(tokio::time::Duration::from_secs(poll_interval)).await;
        }
    }

    /// Convenience method to wait for a file batch to complete
    ///
    /// This method polls the file batch status until it's completed, failed, or cancelled.
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store
    /// * `batch_id` - The ID of the batch to wait for
    /// * `max_wait_seconds` - Maximum time to wait before giving up (default: 300 seconds)
    /// * `poll_interval_seconds` - How often to check the status (default: 5 seconds)
    ///
    /// # Returns
    ///
    /// Returns the final `VectorStoreFileBatch` object when processing is complete
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let batch = api.wait_for_file_batch_complete("vs-abc123", "batch-def456", Some(600), Some(10)).await?;
    /// println!("Batch completed: {}/{} files processed",
    ///          batch.file_counts.completed, batch.file_counts.total);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn wait_for_file_batch_complete(
        &self,
        vector_store_id: impl Into<String>,
        batch_id: impl Into<String>,
        max_wait_seconds: Option<u64>,
        poll_interval_seconds: Option<u64>,
    ) -> Result<VectorStoreFileBatch> {
        let vector_store_id = vector_store_id.into();
        let batch_id = batch_id.into();
        let max_wait = max_wait_seconds.unwrap_or(300);
        let poll_interval = poll_interval_seconds.unwrap_or(5);
        let start_time = std::time::Instant::now();

        loop {
            let batch = self
                .retrieve_vector_store_file_batch(&vector_store_id, &batch_id)
                .await?;

            // Check if processing is complete
            if !matches!(
                batch.status,
                crate::models::vector_stores::VectorStoreFileBatchStatus::InProgress
            ) {
                return Ok(batch);
            }

            // Check timeout
            if start_time.elapsed().as_secs() >= max_wait {
                return Err(OpenAIError::Timeout(format!(
                    "File batch {batch_id} did not complete within {max_wait} seconds"
                )));
            }

            // Wait before next poll
            tokio::time::sleep(tokio::time::Duration::from_secs(poll_interval)).await;
        }
    }

    /// Get usage statistics for vector stores
    ///
    /// Returns statistics about vector stores in the organization.
    ///
    /// # Returns
    ///
    /// Returns a map of statistics including total stores, total usage, etc.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// let stats = api.get_usage_statistics().await?;
    /// for (key, value) in stats {
    ///     println!("{}: {}", key, value);
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn get_usage_statistics(&self) -> Result<HashMap<String, String>> {
        let stores = self.list_vector_stores(None).await?;
        let mut stats = HashMap::new();

        stats.insert("total_stores".to_string(), stores.data.len().to_string());
        stats.insert(
            "total_usage_bytes".to_string(),
            stores.total_usage_bytes().to_string(),
        );
        stats.insert(
            "ready_stores".to_string(),
            stores.ready_stores().len().to_string(),
        );
        stats.insert(
            "processing_stores".to_string(),
            stores.processing_stores().len().to_string(),
        );

        // Calculate average usage
        if !stores.data.is_empty() {
            let avg_usage = stores.total_usage_bytes() / stores.data.len() as u64;
            stats.insert("average_usage_bytes".to_string(), avg_usage.to_string());
        }

        // Total file counts across all stores
        let total_files: u32 = stores.data.iter().map(|s| s.file_counts.total).sum();
        let completed_files: u32 = stores.data.iter().map(|s| s.file_counts.completed).sum();
        let failed_files: u32 = stores.data.iter().map(|s| s.file_counts.failed).sum();

        stats.insert("total_files".to_string(), total_files.to_string());
        stats.insert("completed_files".to_string(), completed_files.to_string());
        stats.insert("failed_files".to_string(), failed_files.to_string());

        Ok(stats)
    }

    /// Validates that a vector store exists and is accessible
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to validate
    ///
    /// # Returns
    ///
    /// Returns `true` if the vector store exists and is accessible, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::{vector_stores::VectorStoresApi, common::ApiClientConstructors};
    ///
    /// # tokio_test::block_on(async {
    /// let api = VectorStoresApi::new("your-api-key")?;
    /// if api.vector_store_exists("vs-abc123").await? {
    ///     println!("Vector store exists and is accessible");
    /// } else {
    ///     println!("Vector store does not exist or is not accessible");
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn vector_store_exists(&self, vector_store_id: impl Into<String>) -> Result<bool> {
        match self.retrieve_vector_store(vector_store_id).await {
            Ok(_) => Ok(true),
            Err(OpenAIError::ApiError { status: 404, .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::vector_stores::{
        ChunkingStrategy, ExpirationPolicy, ListVectorStoreFilesParams, ListVectorStoresParams,
        VectorStoreFileRequest, VectorStoreFileStatus, VectorStoreRequest,
    };

    #[test]
    fn test_vector_stores_api_creation() {
        let api = VectorStoresApi::new("test-key").unwrap();
        assert_eq!(api.api_key(), "test-key");
        assert_eq!(api.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_vector_stores_api_custom_base_url() {
        let api = VectorStoresApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.api_key(), "test-key");
        assert_eq!(api.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_vector_store_request_building() {
        let request = VectorStoreRequest::builder()
            .name("Test Store")
            .add_file_id("file-123")
            .expires_after(ExpirationPolicy::new_days(30))
            .chunking_strategy(ChunkingStrategy::static_chunking(512, 50))
            .add_metadata("environment", "test")
            .build();

        assert_eq!(request.name, Some("Test Store".to_string()));
        assert!(request.file_ids.is_some());
        assert!(request.expires_after.is_some());
        assert!(request.chunking_strategy.is_some());
        assert!(request.metadata.is_some());
    }

    #[test]
    fn test_vector_store_file_request() {
        let request = VectorStoreFileRequest::new("file-123")
            .with_chunking_strategy(ChunkingStrategy::auto());

        assert_eq!(request.file_id, "file-123");
        assert_eq!(request.chunking_strategy, Some(ChunkingStrategy::Auto));
    }

    #[test]
    fn test_list_params_query_building() {
        let store_params = ListVectorStoresParams::new()
            .with_limit(50)
            .with_order("desc")
            .with_after("vs-123");

        let query_params = store_params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("limit".to_string(), "50".to_string())));

        let file_params = ListVectorStoreFilesParams::new()
            .with_filter(VectorStoreFileStatus::Completed)
            .with_limit(25);

        let file_query_params = file_params.to_query_params();
        assert_eq!(file_query_params.len(), 2);
        assert!(file_query_params.contains(&("filter".to_string(), "completed".to_string())));
    }

    // Integration tests would go here if we had a test API key
    // They would test actual API calls against a test environment
}
