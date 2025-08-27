//! File search tool configuration and types

use crate::{De, Ser};

/// File search configuration
#[derive(Debug, Clone, Ser, De)]
pub struct FileSearchConfig {
    /// Vector store IDs to search
    pub vector_store_ids: Vec<String>,

    /// Maximum number of file chunks to retrieve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chunks: Option<u32>,

    /// File types to search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_types: Option<Vec<String>>,
}
