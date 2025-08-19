//! # Vector Stores API Models
//!
//! This module provides data structures for OpenAI's Vector Stores API, which allows you to
//! store and manage large amounts of file-based data in a vector database optimized for RAG.
//!
//! ## Overview
//!
//! Vector stores are used to store file data in a format optimized for retrieval-augmented generation (RAG).
//! They automatically process uploaded files, chunk the content, and create embeddings that can be
//! searched efficiently. Vector stores integrate seamlessly with the Assistants API.
//!
//! ## Key Features
//!
//! - **Automatic Processing**: Files are automatically chunked and embedded
//! - **Efficient Retrieval**: Optimized for similarity search and retrieval
//! - **File Management**: Support for attaching, listing, and removing files
//! - **Batch Operations**: Upload multiple files simultaneously
//! - **Expiration Policies**: Automatic cleanup with configurable expiration
//! - **Chunking Strategies**: Configurable text chunking for optimal embedding
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::vector_stores::{VectorStoreRequest, ExpirationPolicy, ChunkingStrategy};
//!
//! // Create a vector store with expiration policy
//! let request = VectorStoreRequest::builder()
//!     .name("Knowledge Base")
//!     .expires_after(ExpirationPolicy::new_days(30))
//!     .build();
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Status of a vector store
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreStatus {
    /// Vector store is being created and set up
    InProgress,
    /// Vector store is ready for use
    Completed,
    /// Vector store creation failed
    Failed,
    /// Vector store has been cancelled
    Cancelled,
    /// Vector store has expired and is being cleaned up
    Expired,
}

impl fmt::Display for VectorStoreStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            VectorStoreStatus::InProgress => "in_progress",
            VectorStoreStatus::Completed => "completed",
            VectorStoreStatus::Failed => "failed",
            VectorStoreStatus::Cancelled => "cancelled",
            VectorStoreStatus::Expired => "expired",
        };
        write!(f, "{status}")
    }
}

/// Expiration policy for vector stores
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpirationPolicy {
    /// The anchor timestamp after which the vector store will expire
    pub anchor: String,
    /// Number of days after the anchor when the vector store expires
    pub days: u32,
}

impl ExpirationPolicy {
    /// Create a new expiration policy with the specified number of days
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days after the last activity before expiration
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::models::vector_stores::ExpirationPolicy;
    ///
    /// let policy = ExpirationPolicy::new_days(30);
    /// ```
    #[must_use]
    pub fn new_days(days: u32) -> Self {
        Self {
            anchor: "last_active_at".to_string(),
            days,
        }
    }

    /// Create a new expiration policy with custom anchor
    ///
    /// # Arguments
    ///
    /// * `anchor` - The anchor timestamp ("`last_active_at`" or "`created_at`")
    /// * `days` - Number of days after the anchor when the vector store expires
    pub fn new_with_anchor(anchor: impl Into<String>, days: u32) -> Self {
        Self {
            anchor: anchor.into(),
            days,
        }
    }
}

/// File counts within a vector store
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FileCounts {
    /// Number of files currently being processed
    pub in_progress: u32,
    /// Number of files that have been successfully processed
    pub completed: u32,
    /// Number of files that failed to process
    pub failed: u32,
    /// Number of files that were cancelled during processing
    pub cancelled: u32,
    /// Total number of files in the vector store
    pub total: u32,
}

impl FileCounts {
    /// Create a new empty file counts structure
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if all files have been processed (completed or failed)
    #[must_use]
    pub fn is_processing_complete(&self) -> bool {
        self.in_progress == 0
    }

    /// Get the percentage of files that have been successfully completed
    #[must_use]
    pub fn completion_percentage(&self) -> f64 {
        if self.total == 0 {
            return 100.0;
        }
        (f64::from(self.completed) / f64::from(self.total)) * 100.0
    }

    /// Get the percentage of files that failed processing
    #[must_use]
    pub fn failure_percentage(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (f64::from(self.failed) / f64::from(self.total)) * 100.0
    }
}

/// Chunking strategy for processing files in vector stores
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChunkingStrategy {
    /// Automatic chunking with default settings
    #[serde(rename = "auto")]
    Auto,
    /// Static chunking with fixed parameters
    #[serde(rename = "static")]
    Static {
        /// Maximum number of tokens in each chunk
        max_chunk_size_tokens: u32,
        /// Number of tokens to overlap between chunks
        chunk_overlap_tokens: u32,
    },
}

impl Default for ChunkingStrategy {
    fn default() -> Self {
        Self::Auto
    }
}

impl ChunkingStrategy {
    /// Create a new automatic chunking strategy
    #[must_use]
    pub fn auto() -> Self {
        Self::Auto
    }

    /// Create a new static chunking strategy
    ///
    /// # Arguments
    ///
    /// * `max_chunk_size_tokens` - Maximum number of tokens per chunk
    /// * `chunk_overlap_tokens` - Number of overlapping tokens between chunks
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::models::vector_stores::ChunkingStrategy;
    ///
    /// let strategy = ChunkingStrategy::static_chunking(512, 50);
    /// ```
    #[must_use]
    pub fn static_chunking(max_chunk_size_tokens: u32, chunk_overlap_tokens: u32) -> Self {
        Self::Static {
            max_chunk_size_tokens,
            chunk_overlap_tokens,
        }
    }
}

/// A vector store object representing a collection of files for RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStore {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always "`vector_store`"
    pub object: String,
    /// The Unix timestamp (in seconds) for when the vector store was created
    pub created_at: u64,
    /// The name of the vector store
    pub name: Option<String>,
    /// The total number of bytes used by the files in the vector store
    pub usage_bytes: u64,
    /// File counts for different processing states
    pub file_counts: FileCounts,
    /// The status of the vector store
    pub status: VectorStoreStatus,
    /// The expiration policy for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ExpirationPolicy>,
    /// The Unix timestamp (in seconds) for when the vector store will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the vector store was last active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<u64>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl VectorStore {
    /// Check if the vector store is ready for use
    #[must_use]
    pub fn is_ready(&self) -> bool {
        matches!(self.status, VectorStoreStatus::Completed)
    }

    /// Check if the vector store is still being processed
    #[must_use]
    pub fn is_processing(&self) -> bool {
        matches!(self.status, VectorStoreStatus::InProgress)
    }

    /// Check if the vector store has failed
    #[must_use]
    pub fn has_failed(&self) -> bool {
        matches!(self.status, VectorStoreStatus::Failed)
    }

    /// Check if the vector store has expired
    #[must_use]
    pub fn has_expired(&self) -> bool {
        matches!(self.status, VectorStoreStatus::Expired)
    }

    /// Get human-readable usage size
    #[must_use]
    pub fn usage_human_readable(&self) -> String {
        let bytes = self.usage_bytes as f64;
        if bytes < 1024.0 {
            format!("{bytes} B")
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.1} KB", bytes / 1024.0)
        } else if bytes < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB", bytes / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", bytes / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Get the creation date as a formatted string
    #[must_use]
    pub fn created_at_formatted(&self) -> String {
        use std::time::UNIX_EPOCH;
        let datetime = UNIX_EPOCH + std::time::Duration::from_secs(self.created_at);
        format!("{datetime:?}")
    }

    /// Check if the vector store will expire soon (within 24 hours)
    #[must_use]
    pub fn expires_soon(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let twenty_four_hours = 24 * 60 * 60;
            expires_at <= now + twenty_four_hours
        } else {
            false
        }
    }
}

/// Request to create or modify a vector store
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VectorStoreRequest {
    /// A list of file IDs that the vector store should use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// The name of the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The expiration policy for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ExpirationPolicy>,
    /// The chunking strategy used to chunk the file(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<ChunkingStrategy>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl VectorStoreRequest {
    /// Create a new empty vector store request
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a builder for constructing a vector store request
    #[must_use]
    pub fn builder() -> VectorStoreRequestBuilder {
        VectorStoreRequestBuilder::new()
    }

    /// Set the file IDs for the vector store
    #[must_use]
    pub fn with_file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
        self
    }

    /// Set the name for the vector store
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the expiration policy for the vector store
    #[must_use]
    pub fn with_expires_after(mut self, expires_after: ExpirationPolicy) -> Self {
        self.expires_after = Some(expires_after);
        self
    }

    /// Set the chunking strategy for the vector store
    #[must_use]
    pub fn with_chunking_strategy(mut self, chunking_strategy: ChunkingStrategy) -> Self {
        self.chunking_strategy = Some(chunking_strategy);
        self
    }

    /// Set metadata for the vector store
    #[must_use]
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Add a single metadata key-value pair
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        self.metadata
            .as_mut()
            .unwrap()
            .insert(key.into(), value.into());
        self
    }
}

/// Builder for creating vector store requests
#[derive(Debug, Default)]
pub struct VectorStoreRequestBuilder {
    /// The underlying vector store request being built
    request: VectorStoreRequest,
}

impl VectorStoreRequestBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the file IDs for the vector store
    #[must_use]
    pub fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.request.file_ids = Some(file_ids);
        self
    }

    /// Add a single file ID to the vector store
    pub fn add_file_id(mut self, file_id: impl Into<String>) -> Self {
        if self.request.file_ids.is_none() {
            self.request.file_ids = Some(Vec::new());
        }
        self.request.file_ids.as_mut().unwrap().push(file_id.into());
        self
    }

    /// Set the name for the vector store
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    /// Set the expiration policy for the vector store
    #[must_use]
    pub fn expires_after(mut self, expires_after: ExpirationPolicy) -> Self {
        self.request.expires_after = Some(expires_after);
        self
    }

    /// Set the chunking strategy for the vector store
    #[must_use]
    pub fn chunking_strategy(mut self, chunking_strategy: ChunkingStrategy) -> Self {
        self.request.chunking_strategy = Some(chunking_strategy);
        self
    }

    /// Set metadata for the vector store
    #[must_use]
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.request.metadata = Some(metadata);
        self
    }

    /// Add a single metadata key-value pair
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.request.metadata.is_none() {
            self.request.metadata = Some(HashMap::new());
        }
        self.request
            .metadata
            .as_mut()
            .unwrap()
            .insert(key.into(), value.into());
        self
    }

    /// Build the vector store request
    #[must_use]
    pub fn build(self) -> VectorStoreRequest {
        self.request
    }
}

/// A vector store file represents the association between a file and a vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreFile {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always "`vector_store.file`"
    pub object: String,
    /// The total vector store usage in bytes
    pub usage_bytes: u64,
    /// The Unix timestamp (in seconds) for when the vector store file was created
    pub created_at: u64,
    /// The ID of the vector store that the file is attached to
    pub vector_store_id: String,
    /// The status of the vector store file
    pub status: VectorStoreFileStatus,
    /// The last error associated with this vector store file, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<VectorStoreFileError>,
    /// The strategy used to chunk the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<ChunkingStrategy>,
}

/// Status of a vector store file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreFileStatus {
    /// File is being processed
    InProgress,
    /// File has been successfully processed
    Completed,
    /// File processing was cancelled
    Cancelled,
    /// File processing failed
    Failed,
}

impl fmt::Display for VectorStoreFileStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            VectorStoreFileStatus::InProgress => "in_progress",
            VectorStoreFileStatus::Completed => "completed",
            VectorStoreFileStatus::Cancelled => "cancelled",
            VectorStoreFileStatus::Failed => "failed",
        };
        write!(f, "{status}")
    }
}

/// Error information for vector store file processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreFileError {
    /// The error code
    pub code: String,
    /// The error message
    pub message: String,
}

/// Request to create a vector store file association
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreFileRequest {
    /// A file ID that the vector store should use
    pub file_id: String,
    /// The chunking strategy used to chunk the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<ChunkingStrategy>,
}

impl VectorStoreFileRequest {
    /// Create a new vector store file request
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            file_id: file_id.into(),
            chunking_strategy: None,
        }
    }

    /// Set the chunking strategy for the file
    #[must_use]
    pub fn with_chunking_strategy(mut self, chunking_strategy: ChunkingStrategy) -> Self {
        self.chunking_strategy = Some(chunking_strategy);
        self
    }
}

/// A vector store file batch represents a batch operation on multiple files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreFileBatch {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always "`vector_store.files_batch`"
    pub object: String,
    /// The Unix timestamp (in seconds) for when the vector store files batch was created
    pub created_at: u64,
    /// The ID of the vector store that the files are being added to
    pub vector_store_id: String,
    /// The status of the vector store files batch
    pub status: VectorStoreFileBatchStatus,
    /// The file counts for different processing states
    pub file_counts: FileCounts,
}

/// Status of a vector store file batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreFileBatchStatus {
    /// Batch is being processed
    InProgress,
    /// Batch has been completed
    Completed,
    /// Batch was cancelled
    Cancelled,
    /// Batch failed
    Failed,
}

impl fmt::Display for VectorStoreFileBatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            VectorStoreFileBatchStatus::InProgress => "in_progress",
            VectorStoreFileBatchStatus::Completed => "completed",
            VectorStoreFileBatchStatus::Cancelled => "cancelled",
            VectorStoreFileBatchStatus::Failed => "failed",
        };
        write!(f, "{status}")
    }
}

/// Request to create a vector store file batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreFileBatchRequest {
    /// A list of file IDs to be added to the vector store
    pub file_ids: Vec<String>,
    /// The chunking strategy used to chunk the file(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<ChunkingStrategy>,
}

impl VectorStoreFileBatchRequest {
    /// Create a new vector store file batch request
    #[must_use]
    pub fn new(file_ids: Vec<String>) -> Self {
        Self {
            file_ids,
            chunking_strategy: None,
        }
    }

    /// Set the chunking strategy for the files
    #[must_use]
    pub fn with_chunking_strategy(mut self, chunking_strategy: ChunkingStrategy) -> Self {
        self.chunking_strategy = Some(chunking_strategy);
        self
    }
}

/// Response from listing vector stores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVectorStoresResponse {
    /// The object type, which is always "list"
    pub object: String,
    /// The list of vector stores
    pub data: Vec<VectorStore>,
    /// Cursor for the first item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Cursor for the last item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available
    #[serde(default)]
    pub has_more: bool,
}

impl ListVectorStoresResponse {
    /// Create a new empty list response
    #[must_use]
    pub fn empty() -> Self {
        Self {
            object: "list".to_string(),
            data: Vec::new(),
            first_id: None,
            last_id: None,
            has_more: false,
        }
    }

    /// Get total usage bytes of all vector stores
    #[must_use]
    pub fn total_usage_bytes(&self) -> u64 {
        self.data.iter().map(|vs| vs.usage_bytes).sum()
    }

    /// Get vector stores by status
    #[must_use]
    pub fn by_status(&self, status: &VectorStoreStatus) -> Vec<&VectorStore> {
        self.data.iter().filter(|vs| vs.status == *status).collect()
    }

    /// Get vector stores that are ready for use
    #[must_use]
    pub fn ready_stores(&self) -> Vec<&VectorStore> {
        self.data.iter().filter(|vs| vs.is_ready()).collect()
    }

    /// Get vector stores that are still processing
    #[must_use]
    pub fn processing_stores(&self) -> Vec<&VectorStore> {
        self.data.iter().filter(|vs| vs.is_processing()).collect()
    }
}

/// Response from listing vector store files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVectorStoreFilesResponse {
    /// The object type, which is always "list"
    pub object: String,
    /// The list of vector store files
    pub data: Vec<VectorStoreFile>,
    /// Cursor for the first item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Cursor for the last item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available
    #[serde(default)]
    pub has_more: bool,
}

impl ListVectorStoreFilesResponse {
    /// Create a new empty list response
    #[must_use]
    pub fn empty() -> Self {
        Self {
            object: "list".to_string(),
            data: Vec::new(),
            first_id: None,
            last_id: None,
            has_more: false,
        }
    }

    /// Get total usage bytes of all files
    #[must_use]
    pub fn total_usage_bytes(&self) -> u64 {
        self.data.iter().map(|f| f.usage_bytes).sum()
    }

    /// Get files by status
    #[must_use]
    pub fn by_status(&self, status: &VectorStoreFileStatus) -> Vec<&VectorStoreFile> {
        self.data.iter().filter(|f| f.status == *status).collect()
    }

    /// Get completed files
    #[must_use]
    pub fn completed_files(&self) -> Vec<&VectorStoreFile> {
        self.data
            .iter()
            .filter(|f| f.status == VectorStoreFileStatus::Completed)
            .collect()
    }

    /// Get failed files
    #[must_use]
    pub fn failed_files(&self) -> Vec<&VectorStoreFile> {
        self.data
            .iter()
            .filter(|f| f.status == VectorStoreFileStatus::Failed)
            .collect()
    }
}

/// Response from deleting a vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreDeleteResponse {
    /// The ID of the deleted vector store
    pub id: String,
    /// The object type, which is always "`vector_store.deleted`"
    pub object: String,
    /// Whether the vector store was successfully deleted
    pub deleted: bool,
}

impl VectorStoreDeleteResponse {
    /// Create a successful delete response
    #[must_use]
    pub fn success(id: String) -> Self {
        Self {
            id,
            object: "vector_store.deleted".to_string(),
            deleted: true,
        }
    }

    /// Create a failed delete response
    #[must_use]
    pub fn failure(id: String) -> Self {
        Self {
            id,
            object: "vector_store.deleted".to_string(),
            deleted: false,
        }
    }
}

/// Response from deleting a vector store file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreFileDeleteResponse {
    /// The ID of the deleted vector store file
    pub id: String,
    /// The object type, which is always "`vector_store.file.deleted`"
    pub object: String,
    /// Whether the vector store file was successfully deleted
    pub deleted: bool,
}

impl VectorStoreFileDeleteResponse {
    /// Create a successful delete response
    #[must_use]
    pub fn success(id: String) -> Self {
        Self {
            id,
            object: "vector_store.file.deleted".to_string(),
            deleted: true,
        }
    }

    /// Create a failed delete response
    #[must_use]
    pub fn failure(id: String) -> Self {
        Self {
            id,
            object: "vector_store.file.deleted".to_string(),
            deleted: false,
        }
    }
}

/// Parameters for listing vector stores
#[derive(Debug, Clone, Default)]
pub struct ListVectorStoresParams {
    /// Maximum number of vector stores to return (default 20, max 100)
    pub limit: Option<u32>,
    /// Sort order for the results (desc for descending, asc for ascending)
    pub order: Option<String>,
    /// Pagination cursor - list vector stores after this ID
    pub after: Option<String>,
    /// Pagination cursor - list vector stores before this ID
    pub before: Option<String>,
}

impl ListVectorStoresParams {
    /// Create new parameters with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    #[must_use]
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the sort order
    pub fn with_order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set the after cursor for pagination
    pub fn with_after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the before cursor for pagination
    pub fn with_before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Build query parameters for the API request
    #[must_use]
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }

        if let Some(order) = &self.order {
            params.push(("order".to_string(), order.clone()));
        }

        if let Some(after) = &self.after {
            params.push(("after".to_string(), after.clone()));
        }

        if let Some(before) = &self.before {
            params.push(("before".to_string(), before.clone()));
        }

        params
    }
}

/// Parameters for listing vector store files
#[derive(Debug, Clone, Default)]
pub struct ListVectorStoreFilesParams {
    /// Maximum number of files to return (default 20, max 100)
    pub limit: Option<u32>,
    /// Sort order for the results (desc for descending, asc for ascending)
    pub order: Option<String>,
    /// Pagination cursor - list files after this ID
    pub after: Option<String>,
    /// Pagination cursor - list files before this ID
    pub before: Option<String>,
    /// Filter files by status
    pub filter: Option<VectorStoreFileStatus>,
}

impl ListVectorStoreFilesParams {
    /// Create new parameters with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    #[must_use]
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the sort order
    pub fn with_order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set the after cursor for pagination
    pub fn with_after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the before cursor for pagination
    pub fn with_before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the status filter
    #[must_use]
    pub fn with_filter(mut self, filter: VectorStoreFileStatus) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Build query parameters for the API request
    #[must_use]
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }

        if let Some(order) = &self.order {
            params.push(("order".to_string(), order.clone()));
        }

        if let Some(after) = &self.after {
            params.push(("after".to_string(), after.clone()));
        }

        if let Some(before) = &self.before {
            params.push(("before".to_string(), before.clone()));
        }

        if let Some(filter) = &self.filter {
            params.push(("filter".to_string(), filter.to_string()));
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_store_status_display() {
        assert_eq!(VectorStoreStatus::InProgress.to_string(), "in_progress");
        assert_eq!(VectorStoreStatus::Completed.to_string(), "completed");
        assert_eq!(VectorStoreStatus::Failed.to_string(), "failed");
        assert_eq!(VectorStoreStatus::Cancelled.to_string(), "cancelled");
        assert_eq!(VectorStoreStatus::Expired.to_string(), "expired");
    }

    #[test]
    fn test_expiration_policy() {
        let policy = ExpirationPolicy::new_days(30);
        assert_eq!(policy.anchor, "last_active_at");
        assert_eq!(policy.days, 30);

        let custom_policy = ExpirationPolicy::new_with_anchor("created_at", 7);
        assert_eq!(custom_policy.anchor, "created_at");
        assert_eq!(custom_policy.days, 7);
    }

    #[test]
    fn test_file_counts() {
        let mut counts = FileCounts::new();
        assert_eq!(counts.total, 0);
        assert!(counts.is_processing_complete());
        assert_eq!(counts.completion_percentage(), 100.0);

        counts.total = 10;
        counts.completed = 7;
        counts.failed = 2;
        counts.in_progress = 1;

        assert!(!counts.is_processing_complete());
        assert_eq!(counts.completion_percentage(), 70.0);
        assert_eq!(counts.failure_percentage(), 20.0);
    }

    #[test]
    fn test_chunking_strategy() {
        let auto_strategy = ChunkingStrategy::auto();
        assert_eq!(auto_strategy, ChunkingStrategy::Auto);

        let static_strategy = ChunkingStrategy::static_chunking(512, 50);
        if let ChunkingStrategy::Static {
            max_chunk_size_tokens,
            chunk_overlap_tokens,
        } = static_strategy
        {
            assert_eq!(max_chunk_size_tokens, 512);
            assert_eq!(chunk_overlap_tokens, 50);
        } else {
            panic!("Expected static chunking strategy");
        }
    }

    #[test]
    fn test_vector_store_request_builder() {
        let request = VectorStoreRequest::builder()
            .name("Test Store")
            .add_file_id("file-123")
            .add_file_id("file-456")
            .expires_after(ExpirationPolicy::new_days(30))
            .chunking_strategy(ChunkingStrategy::static_chunking(512, 50))
            .add_metadata("environment", "test")
            .add_metadata("version", "1.0")
            .build();

        assert_eq!(request.name, Some("Test Store".to_string()));
        assert_eq!(
            request.file_ids,
            Some(vec!["file-123".to_string(), "file-456".to_string()])
        );
        assert!(request.expires_after.is_some());
        assert!(request.chunking_strategy.is_some());
        assert!(request.metadata.is_some());

        let metadata = request.metadata.unwrap();
        assert_eq!(metadata.get("environment"), Some(&"test".to_string()));
        assert_eq!(metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_vector_store_file_request() {
        let request = VectorStoreFileRequest::new("file-123")
            .with_chunking_strategy(ChunkingStrategy::static_chunking(256, 25));

        assert_eq!(request.file_id, "file-123");
        assert!(request.chunking_strategy.is_some());
    }

    #[test]
    fn test_vector_store_file_batch_request() {
        let request =
            VectorStoreFileBatchRequest::new(vec!["file-1".to_string(), "file-2".to_string()])
                .with_chunking_strategy(ChunkingStrategy::auto());

        assert_eq!(request.file_ids.len(), 2);
        assert_eq!(request.chunking_strategy, Some(ChunkingStrategy::Auto));
    }

    #[test]
    fn test_list_vector_stores_params() {
        let params = ListVectorStoresParams::new()
            .with_limit(50)
            .with_order("desc")
            .with_after("vs-123");

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("limit".to_string(), "50".to_string())));
        assert!(query_params.contains(&("order".to_string(), "desc".to_string())));
        assert!(query_params.contains(&("after".to_string(), "vs-123".to_string())));
    }

    #[test]
    fn test_list_vector_store_files_params() {
        let params = ListVectorStoreFilesParams::new()
            .with_limit(25)
            .with_filter(VectorStoreFileStatus::Completed)
            .with_order("asc");

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("limit".to_string(), "25".to_string())));
        assert!(query_params.contains(&("filter".to_string(), "completed".to_string())));
        assert!(query_params.contains(&("order".to_string(), "asc".to_string())));
    }

    #[test]
    fn test_vector_store_status_methods() {
        let mut store = VectorStore {
            id: "vs-123".to_string(),
            object: "vector_store".to_string(),
            created_at: 1640995200,
            name: Some("Test Store".to_string()),
            usage_bytes: 1024,
            file_counts: FileCounts::new(),
            status: VectorStoreStatus::Completed,
            expires_after: None,
            expires_at: None,
            last_active_at: None,
            metadata: HashMap::new(),
        };

        assert!(store.is_ready());
        assert!(!store.is_processing());
        assert!(!store.has_failed());
        assert!(!store.has_expired());
        assert_eq!(store.usage_human_readable(), "1.0 KB");
        assert!(!store.expires_soon());

        store.status = VectorStoreStatus::InProgress;
        assert!(!store.is_ready());
        assert!(store.is_processing());

        store.status = VectorStoreStatus::Failed;
        assert!(store.has_failed());

        store.status = VectorStoreStatus::Expired;
        assert!(store.has_expired());
    }

    #[test]
    fn test_list_responses_filtering() {
        let stores = vec![
            VectorStore {
                id: "vs-1".to_string(),
                object: "vector_store".to_string(),
                created_at: 1640995200,
                name: Some("Store 1".to_string()),
                usage_bytes: 500,
                file_counts: FileCounts::new(),
                status: VectorStoreStatus::Completed,
                expires_after: None,
                expires_at: None,
                last_active_at: None,
                metadata: HashMap::new(),
            },
            VectorStore {
                id: "vs-2".to_string(),
                object: "vector_store".to_string(),
                created_at: 1640995300,
                name: Some("Store 2".to_string()),
                usage_bytes: 1500,
                file_counts: FileCounts::new(),
                status: VectorStoreStatus::InProgress,
                expires_after: None,
                expires_at: None,
                last_active_at: None,
                metadata: HashMap::new(),
            },
        ];

        let response = ListVectorStoresResponse {
            object: "list".to_string(),
            data: stores,
            first_id: None,
            last_id: None,
            has_more: false,
        };

        assert_eq!(response.total_usage_bytes(), 2000);
        assert_eq!(response.ready_stores().len(), 1);
        assert_eq!(response.processing_stores().len(), 1);
        assert_eq!(response.by_status(&VectorStoreStatus::Completed).len(), 1);
    }

    #[test]
    fn test_delete_responses() {
        let success = VectorStoreDeleteResponse::success("vs-123".to_string());
        assert!(success.deleted);
        assert_eq!(success.id, "vs-123");
        assert_eq!(success.object, "vector_store.deleted");

        let failure = VectorStoreDeleteResponse::failure("vs-456".to_string());
        assert!(!failure.deleted);
        assert_eq!(failure.id, "vs-456");

        let file_success = VectorStoreFileDeleteResponse::success("file-123".to_string());
        assert!(file_success.deleted);
        assert_eq!(file_success.object, "vector_store.file.deleted");
    }
}
