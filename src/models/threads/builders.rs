//! Common builder traits and utilities for threads and messages

use std::collections::HashMap;

/// Common trait for builders that support metadata
pub trait MetadataBuilder {
    /// Get mutable reference to the metadata HashMap
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String>;

    /// Add a metadata key-value pair
    fn add_metadata_pair(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.get_metadata_mut().insert(key.into(), value.into());
    }

    /// Set all metadata
    fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        *self.get_metadata_mut() = metadata;
    }
}

// Note: Macros are defined in the main macros module to avoid duplication
// This module only contains local helper functions and traits

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBuilder {
        metadata: HashMap<String, String>,
    }

    impl MetadataBuilder for TestBuilder {
        fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
            &mut self.metadata
        }
    }

    #[test]
    fn test_metadata_builder() {
        let mut builder = TestBuilder {
            metadata: HashMap::new(),
        };

        builder.add_metadata_pair("key1", "value1");
        assert_eq!(builder.metadata.get("key1"), Some(&"value1".to_string()));

        let mut new_metadata = HashMap::new();
        new_metadata.insert("key2".to_string(), "value2".to_string());
        builder.set_metadata(new_metadata);

        assert_eq!(builder.metadata.len(), 1);
        assert_eq!(builder.metadata.get("key2"), Some(&"value2".to_string()));
    }
}
