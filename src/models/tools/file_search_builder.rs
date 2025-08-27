//! Builder for file search tool configurations

use super::*;

/// Builder for file search configuration
pub struct FileSearchBuilder {
    /// The file search configuration being built
    config: FileSearchConfig,
}

impl FileSearchBuilder {
    /// Create a new FileSearchBuilder with the specified vector store IDs
    #[must_use]
    pub fn new(vector_store_ids: Vec<String>) -> Self {
        Self {
            config: FileSearchConfig {
                vector_store_ids,
                max_chunks: None,
                file_types: None,
            },
        }
    }

    /// Set the maximum number of chunks to search
    #[must_use]
    pub fn max_chunks(mut self, max: u32) -> Self {
        self.config.max_chunks = Some(max);
        self
    }

    /// Specify the file types to search
    #[must_use]
    pub fn file_types(mut self, types: Vec<String>) -> Self {
        self.config.file_types = Some(types);
        self
    }

    /// Build the configured file search tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::FileSearch(self.config)
    }
}
