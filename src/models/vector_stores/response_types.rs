//! Response types for vector store operations
//!
//! This module contains response types for vector store API operations,
//! including list responses and delete responses.

use crate::models::vector_stores::file_types::VectorStoreFile;
use crate::models::vector_stores::status_types::{VectorStoreFileStatus, VectorStoreStatus};
use crate::models::vector_stores::store_types::VectorStore;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Response from listing vector stores
#[derive(Debug, Clone, Ser, De)]
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

    /// Create a new list response with data
    #[must_use]
    pub fn with_data(data: Vec<VectorStore>) -> Self {
        let first_id = data.first().map(|vs| vs.id.clone());
        let last_id = data.last().map(|vs| vs.id.clone());

        Self {
            object: "list".to_string(),
            data,
            first_id,
            last_id,
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

    /// Get vector stores that have failed
    #[must_use]
    pub fn failed_stores(&self) -> Vec<&VectorStore> {
        self.data.iter().filter(|vs| vs.has_failed()).collect()
    }

    /// Get the count of vector stores
    #[must_use]
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Check if the response is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get human-readable total usage
    #[must_use]
    pub fn total_usage_human_readable(&self) -> String {
        crate::models::vector_stores::common_types::utils::bytes_to_human_readable(
            self.total_usage_bytes(),
        )
    }
}

/// Response from listing vector store files
#[derive(Debug, Clone, Ser, De)]
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

    /// Create a new list response with data
    #[must_use]
    pub fn with_data(data: Vec<VectorStoreFile>) -> Self {
        let first_id = data.first().map(|f| f.id.clone());
        let last_id = data.last().map(|f| f.id.clone());

        Self {
            object: "list".to_string(),
            data,
            first_id,
            last_id,
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

    /// Get processing files
    #[must_use]
    pub fn processing_files(&self) -> Vec<&VectorStoreFile> {
        self.data
            .iter()
            .filter(|f| f.status == VectorStoreFileStatus::InProgress)
            .collect()
    }

    /// Get the count of files
    #[must_use]
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Check if the response is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get human-readable total usage
    #[must_use]
    pub fn total_usage_human_readable(&self) -> String {
        crate::models::vector_stores::common_types::utils::bytes_to_human_readable(
            self.total_usage_bytes(),
        )
    }

    /// Get files with errors
    #[must_use]
    pub fn files_with_errors(&self) -> Vec<&VectorStoreFile> {
        self.data
            .iter()
            .filter(|f| f.last_error.is_some())
            .collect()
    }
}

/// Response from deleting a vector store
#[derive(Debug, Clone, Ser, De)]
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

    /// Check if the deletion was successful
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.deleted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::vector_stores::common_types::FileCounts;
    use std::collections::HashMap;

    fn create_test_vector_store(
        id: &str,
        status: VectorStoreStatus,
        usage_bytes: u64,
    ) -> VectorStore {
        VectorStore {
            id: id.to_string(),
            object: "vector_store".to_string(),
            created_at: 1_640_995_200,
            name: Some(format!("Store {id}")),
            usage_bytes,
            file_counts: FileCounts::new(),
            status,
            expires_after: None,
            expires_at: None,
            last_active_at: None,
            metadata: HashMap::new(),
        }
    }

    fn create_test_vector_store_file(
        id: &str,
        status: VectorStoreFileStatus,
        usage_bytes: u64,
    ) -> VectorStoreFile {
        VectorStoreFile {
            id: id.to_string(),
            object: "vector_store.file".to_string(),
            usage_bytes,
            created_at: 1_640_995_200,
            vector_store_id: "vs-123".to_string(),
            status,
            last_error: None,
            chunking_strategy: None,
        }
    }

    #[test]
    fn test_list_vector_stores_response() {
        let stores = vec![
            create_test_vector_store("vs-1", VectorStoreStatus::Completed, 1024),
            create_test_vector_store("vs-2", VectorStoreStatus::InProgress, 2048),
            create_test_vector_store("vs-3", VectorStoreStatus::Failed, 512),
        ];

        let response = ListVectorStoresResponse::with_data(stores);

        assert_eq!(response.object, "list");
        assert_eq!(response.count(), 3);
        assert!(!response.is_empty());
        assert_eq!(response.total_usage_bytes(), 3584);
        assert_eq!(response.ready_stores().len(), 1);
        assert_eq!(response.processing_stores().len(), 1);
        assert_eq!(response.failed_stores().len(), 1);
        assert_eq!(response.first_id, Some("vs-1".to_string()));
        assert_eq!(response.last_id, Some("vs-3".to_string()));
    }

    #[test]
    fn test_empty_list_vector_stores_response() {
        let response = ListVectorStoresResponse::empty();

        assert_eq!(response.object, "list");
        assert_eq!(response.count(), 0);
        assert!(response.is_empty());
        assert_eq!(response.total_usage_bytes(), 0);
        assert!(response.ready_stores().is_empty());
        assert_eq!(response.first_id, None);
        assert_eq!(response.last_id, None);
        assert!(!response.has_more);
    }

    #[test]
    fn test_list_vector_store_files_response() {
        let files = vec![
            create_test_vector_store_file("file-1", VectorStoreFileStatus::Completed, 512),
            create_test_vector_store_file("file-2", VectorStoreFileStatus::Failed, 256),
            create_test_vector_store_file("file-3", VectorStoreFileStatus::InProgress, 1024),
        ];

        let response = ListVectorStoreFilesResponse::with_data(files);

        assert_eq!(response.object, "list");
        assert_eq!(response.count(), 3);
        assert!(!response.is_empty());
        assert_eq!(response.total_usage_bytes(), 1792);
        assert_eq!(response.completed_files().len(), 1);
        assert_eq!(response.failed_files().len(), 1);
        assert_eq!(response.processing_files().len(), 1);
        assert_eq!(response.first_id, Some("file-1".to_string()));
        assert_eq!(response.last_id, Some("file-3".to_string()));
    }

    #[test]
    fn test_empty_list_vector_store_files_response() {
        let response = ListVectorStoreFilesResponse::empty();

        assert_eq!(response.object, "list");
        assert_eq!(response.count(), 0);
        assert!(response.is_empty());
        assert_eq!(response.total_usage_bytes(), 0);
        assert!(response.completed_files().is_empty());
        assert_eq!(response.first_id, None);
        assert_eq!(response.last_id, None);
        assert!(!response.has_more);
    }

    #[test]
    fn test_vector_store_files_with_errors() {
        let mut file_with_error =
            create_test_vector_store_file("file-1", VectorStoreFileStatus::Failed, 512);
        file_with_error.last_error = Some(
            crate::models::vector_stores::common_types::VectorStoreFileError {
                code: "processing_error".to_string(),
                message: "Failed to process".to_string(),
            },
        );

        let files = vec![
            file_with_error,
            create_test_vector_store_file("file-2", VectorStoreFileStatus::Completed, 256),
        ];

        let response = ListVectorStoreFilesResponse::with_data(files);
        assert_eq!(response.files_with_errors().len(), 1);
    }

    #[test]
    fn test_vector_store_delete_response() {
        let success = VectorStoreDeleteResponse::success("vs-123".to_string());
        assert!(success.is_success());
        assert!(success.deleted);
        assert_eq!(success.id, "vs-123");
        assert_eq!(success.object, "vector_store.deleted");

        let failure = VectorStoreDeleteResponse::failure("vs-456".to_string());
        assert!(!failure.is_success());
        assert!(!failure.deleted);
        assert_eq!(failure.id, "vs-456");
        assert_eq!(failure.object, "vector_store.deleted");
    }

    #[test]
    fn test_response_serialization() {
        let stores = vec![create_test_vector_store(
            "vs-1",
            VectorStoreStatus::Completed,
            1024,
        )];
        let response = ListVectorStoresResponse::with_data(stores);

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("vs-1"));
        assert!(json.contains("list"));

        let deserialized: ListVectorStoresResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.count(), 1);
        assert_eq!(deserialized.object, "list");
    }

    #[test]
    fn test_response_human_readable_usage() {
        let stores = vec![
            create_test_vector_store("vs-1", VectorStoreStatus::Completed, 1024),
            create_test_vector_store("vs-2", VectorStoreStatus::Completed, 2048),
        ];
        let response = ListVectorStoresResponse::with_data(stores);

        assert_eq!(response.total_usage_human_readable(), "3.0 KB");

        let files = vec![
            create_test_vector_store_file("file-1", VectorStoreFileStatus::Completed, 1_048_576), // 1 MB
        ];
        let file_response = ListVectorStoreFilesResponse::with_data(files);

        assert_eq!(file_response.total_usage_human_readable(), "1.0 MB");
    }

    #[test]
    fn test_filtering_by_status() {
        let stores = vec![
            create_test_vector_store("vs-1", VectorStoreStatus::Completed, 1024),
            create_test_vector_store("vs-2", VectorStoreStatus::Completed, 2048),
            create_test_vector_store("vs-3", VectorStoreStatus::InProgress, 512),
        ];
        let response = ListVectorStoresResponse::with_data(stores);

        let completed = response.by_status(&VectorStoreStatus::Completed);
        assert_eq!(completed.len(), 2);

        let in_progress = response.by_status(&VectorStoreStatus::InProgress);
        assert_eq!(in_progress.len(), 1);
        assert_eq!(in_progress[0].id, "vs-3");
    }
}
