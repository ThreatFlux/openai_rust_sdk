//! Common types and utilities shared across vector store modules
//!
//! This module contains shared types and utility functions used by multiple
//! vector store modules, including expiration policies, chunking strategies,
//! and file counts.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// Expiration policy for vector stores
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
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
#[derive(Debug, Clone, PartialEq, Eq, Ser, De, Default)]
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
#[derive(Debug, Clone, PartialEq, Ser, De)]
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

/// Error information for vector store file processing
#[derive(Debug, Clone, Ser, De)]
pub struct VectorStoreFileError {
    /// The error code
    pub code: String,
    /// The error message
    pub message: String,
}

/// Utility functions for formatting and conversion
pub mod utils {
    // Generate the bytes to human readable function
    crate::impl_bytes_to_human_readable!();

    /// Format Unix timestamp as a human-readable string
    #[must_use]
    pub fn format_timestamp(timestamp: u64) -> String {
        use std::time::UNIX_EPOCH;
        let datetime = UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
        format!("{datetime:?}")
    }

    /// Check if a timestamp is within the given number of seconds from now
    #[must_use]
    pub fn expires_within_seconds(expires_at: Option<u64>, seconds: u64) -> bool {
        if let Some(expires_at) = expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            expires_at <= now + seconds
        } else {
            false
        }
    }

    /// Get current Unix timestamp
    #[must_use]
    pub fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Trait for types that have status checking functionality
pub trait StatusChecker<T> {
    /// Get the current status
    fn status(&self) -> &T;
    
    /// Check if the status matches a specific value
    fn has_status(&self, status: &T) -> bool
    where 
        T: PartialEq,
    {
        self.status() == status
    }
}

/// Trait for builder patterns with optional fields
pub trait OptionalFieldBuilder<T> {
    /// Set an optional field if the value is Some
    fn set_optional<V>(&mut self, field: &mut Option<V>, value: Option<V>) -> &mut Self {
        if let Some(v) = value {
            *field = Some(v);
        }
        self
    }
}

/// Trait for managing metadata in request builders
pub trait MetadataBuilder {
    /// Get a mutable reference to the metadata field
    fn metadata_mut(&mut self) -> &mut Option<HashMap<String, String>>;
    
    /// Add a metadata key-value pair
    fn add_metadata_pair(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        if self.metadata_mut().is_none() {
            *self.metadata_mut() = Some(HashMap::new());
        }
        self.metadata_mut()
            .as_mut()
            .unwrap()
            .insert(key.into(), value.into());
        self
    }
}

/// Trait for managing file ID lists in request builders
pub trait FileIdBuilder {
    /// Get a mutable reference to the file_ids field
    fn file_ids_mut(&mut self) -> &mut Option<Vec<String>>;
    
    /// Add a file ID to the list
    fn add_file_id_to_list(&mut self, file_id: impl Into<String>) -> &mut Self {
        if self.file_ids_mut().is_none() {
            *self.file_ids_mut() = Some(Vec::new());
        }
        self.file_ids_mut()
            .as_mut()
            .unwrap()
            .push(file_id.into());
        self
    }
}

/// Trait for building query parameters
pub trait QueryParamBuilder {
    /// Convert to query parameter vector
    fn to_query_params(&self) -> Vec<(String, String)>;
    
    /// Check if any parameters are set
    fn is_empty(&self) -> bool;
}


/// Macro for implementing common builder methods
#[macro_export]
macro_rules! impl_builder_methods {
    ($builder_type:ty, {
        $(
            $method_name:ident: $field_type:ty => $field_name:ident,
        )*
    }) => {
        impl $builder_type {
            $(
                /// Set the field value
                #[must_use]
                pub fn $method_name(mut self, value: $field_type) -> Self {
                    self.request.$field_name = Some(value);
                    self
                }
            )*
        }
    };
}

/// Macro for implementing query parameter building with common fields
#[macro_export]
macro_rules! impl_query_params {
    ($struct_name:ty, {
        $(
            $field_name:ident: $field_type:ty,
        )*
    }) => {
        impl QueryParamBuilder for $struct_name {
            fn to_query_params(&self) -> Vec<(String, String)> {
                let mut params = Vec::new();
                
                $(
                    if let Some(ref value) = self.$field_name {
                        params.push((stringify!($field_name).to_string(), value.to_string()));
                    }
                )*
                
                params
            }
            
            fn is_empty(&self) -> bool {
                true $(&& self.$field_name.is_none())*
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(counts.failure_percentage(), 0.0);

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
    fn test_chunking_strategy_serialization() {
        let auto_strategy = ChunkingStrategy::auto();
        let auto_json = serde_json::to_string(&auto_strategy).unwrap();
        assert!(auto_json.contains("\"type\":\"auto\""));

        let static_strategy = ChunkingStrategy::static_chunking(512, 64);
        let static_json = serde_json::to_string(&static_strategy).unwrap();
        assert!(static_json.contains("\"type\":\"static\""));
        assert!(static_json.contains("\"max_chunk_size_tokens\":512"));
        assert!(static_json.contains("\"chunk_overlap_tokens\":64"));

        // Test deserialization
        let deserialized_auto: ChunkingStrategy = serde_json::from_str(&auto_json).unwrap();
        assert_eq!(deserialized_auto, ChunkingStrategy::Auto);

        let deserialized_static: ChunkingStrategy = serde_json::from_str(&static_json).unwrap();
        if let ChunkingStrategy::Static {
            max_chunk_size_tokens,
            chunk_overlap_tokens,
        } = deserialized_static
        {
            assert_eq!(max_chunk_size_tokens, 512);
            assert_eq!(chunk_overlap_tokens, 64);
        } else {
            panic!("Expected static chunking strategy");
        }
    }

    #[test]
    fn test_utils_bytes_to_human_readable() {
        assert_eq!(utils::bytes_to_human_readable(512), "512 B");
        assert_eq!(utils::bytes_to_human_readable(1024), "1.0 KB");
        assert_eq!(utils::bytes_to_human_readable(1_048_576), "1.0 MB");
        assert_eq!(utils::bytes_to_human_readable(1_073_741_824), "1.0 GB");
        assert_eq!(utils::bytes_to_human_readable(2048), "2.0 KB");
    }

    #[test]
    fn test_utils_expires_within_seconds() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Test expiring within 1 hour
        assert!(utils::expires_within_seconds(Some(now + 3600), 7200)); // 2 hours from now, within 7200 seconds
        assert!(!utils::expires_within_seconds(Some(now + 7201), 7200)); // Beyond threshold
        assert!(!utils::expires_within_seconds(None, 3600)); // No expiration set
    }

    #[test]
    fn test_utils_current_timestamp() {
        let timestamp = utils::current_timestamp();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Should be very close to current time (within 1 second)
        assert!((timestamp as i64 - now as i64).abs() <= 1);
    }
}
