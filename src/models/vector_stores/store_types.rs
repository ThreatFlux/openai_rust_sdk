//! Vector store types and implementations
//!
//! This module contains the main VectorStore type and its associated
//! request types and builder patterns.

use crate::models::vector_stores::common_types::{ChunkingStrategy, ExpirationPolicy, FileCounts};
use crate::models::vector_stores::status_types::VectorStoreStatus;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// A vector store object representing a collection of files for RAG
#[derive(Debug, Clone, Ser, De)]
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
        crate::models::vector_stores::common_types::utils::bytes_to_human_readable(self.usage_bytes)
    }

    /// Get the creation date as a formatted string
    #[must_use]
    pub fn created_at_formatted(&self) -> String {
        crate::models::vector_stores::common_types::utils::format_timestamp(self.created_at)
    }

    /// Check if the vector store will expire soon (within 24 hours)
    #[must_use]
    pub fn expires_soon(&self) -> bool {
        const TWENTY_FOUR_HOURS: u64 = 24 * 60 * 60;
        crate::models::vector_stores::common_types::utils::expires_within_seconds(
            self.expires_at,
            TWENTY_FOUR_HOURS,
        )
    }
}

/// Request to create or modify a vector store
#[derive(Debug, Clone, Ser, De, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_vector_store_fluent_interface() {
        let mut metadata = HashMap::new();
        metadata.insert("project".to_string(), "test".to_string());

        let request = VectorStoreRequest::new()
            .with_name("Fluent Test")
            .with_file_ids(vec!["file-789".to_string()])
            .with_expires_after(ExpirationPolicy::new_days(60))
            .with_chunking_strategy(ChunkingStrategy::auto())
            .with_metadata(metadata)
            .add_metadata("additional", "value");

        assert_eq!(request.name, Some("Fluent Test".to_string()));
        assert_eq!(request.file_ids, Some(vec!["file-789".to_string()]));
        assert!(request.expires_after.is_some());
        assert_eq!(request.chunking_strategy, Some(ChunkingStrategy::Auto));
        assert!(request.metadata.is_some());

        let meta = request.metadata.unwrap();
        assert_eq!(meta.len(), 2);
        assert_eq!(meta.get("project"), Some(&"test".to_string()));
        assert_eq!(meta.get("additional"), Some(&"value".to_string()));
    }

    #[test]
    fn test_vector_store_status_methods() {
        let mut store = VectorStore {
            id: "vs-123".to_string(),
            object: "vector_store".to_string(),
            created_at: 1_640_995_200,
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
    fn test_vector_store_expires_soon() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut store = VectorStore {
            id: "vs-test123".to_string(),
            object: "vector_store".to_string(),
            created_at: now - 3600,
            name: Some("Test Store".to_string()),
            usage_bytes: 1024,
            file_counts: FileCounts::new(),
            status: VectorStoreStatus::Completed,
            expires_after: None,
            expires_at: Some(now + 3600), // Expires in 1 hour
            last_active_at: Some(now),
            metadata: HashMap::new(),
        };

        assert!(store.expires_soon());

        store.expires_at = Some(now + 48 * 3600); // Expires in 2 days
        assert!(!store.expires_soon());

        store.expires_at = None;
        assert!(!store.expires_soon());
    }

    #[test]
    fn test_vector_store_serialization() {
        let store = VectorStore {
            id: "vs-123".to_string(),
            object: "vector_store".to_string(),
            created_at: 1_640_995_200,
            name: Some("Test Store".to_string()),
            usage_bytes: 1024,
            file_counts: FileCounts::new(),
            status: VectorStoreStatus::Completed,
            expires_after: Some(ExpirationPolicy::new_days(30)),
            expires_at: None,
            last_active_at: None,
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string(&store).unwrap();
        assert!(json.contains("vs-123"));
        assert!(json.contains("Test Store"));

        let deserialized: VectorStore = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, store.id);
        assert_eq!(deserialized.name, store.name);
        assert_eq!(deserialized.status, store.status);
    }
}
