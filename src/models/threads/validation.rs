//! Validation utilities for threads and messages

use std::collections::HashMap;

/// Common validation functions for threads and messages
pub mod common {
    use super::HashMap;
    use std::hash::BuildHasher;

    /// Validate metadata constraints
    pub fn validate_metadata<S>(metadata: &HashMap<String, String, S>) -> Result<(), String>
    where
        S: BuildHasher,
    {
        // Validate metadata count
        if metadata.len() > 16 {
            return Err("Cannot have more than 16 metadata pairs".to_string());
        }

        // Validate metadata key/value lengths
        for (key, value) in metadata {
            if key.len() > 64 {
                return Err("Metadata key cannot exceed 64 characters".to_string());
            }
            if value.len() > 512 {
                return Err("Metadata value cannot exceed 512 characters".to_string());
            }
        }

        Ok(())
    }

    /// Validate message content length
    pub fn validate_content_length(content: &str) -> Result<(), String> {
        if content.len() > 32768 {
            return Err("Message content cannot exceed 32,768 characters".to_string());
        }
        Ok(())
    }

    /// Validate file IDs count
    pub fn validate_file_ids_count(file_ids: &[String]) -> Result<(), String> {
        if file_ids.len() > 10 {
            return Err("Message cannot have more than 10 file IDs".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::common::*;
    use std::collections::HashMap;

    #[test]
    fn test_validate_metadata_success() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        assert!(validate_metadata(&metadata).is_ok());
    }

    #[test]
    fn test_validate_metadata_too_many_pairs() {
        let mut metadata = HashMap::new();
        for i in 0..17 {
            metadata.insert(format!("key{}", i), "value".to_string());
        }

        assert!(validate_metadata(&metadata).is_err());
    }

    #[test]
    fn test_validate_metadata_key_too_long() {
        let mut metadata = HashMap::new();
        let long_key = "a".repeat(65);
        metadata.insert(long_key, "value".to_string());

        assert!(validate_metadata(&metadata).is_err());
    }

    #[test]
    fn test_validate_content_length() {
        assert!(validate_content_length("Hello").is_ok());

        let long_content = "a".repeat(32769);
        assert!(validate_content_length(&long_content).is_err());
    }

    #[test]
    fn test_validate_file_ids_count() {
        let file_ids = vec!["file1".to_string(), "file2".to_string()];
        assert!(validate_file_ids_count(&file_ids).is_ok());

        let too_many_files: Vec<String> = (0..11).map(|i| format!("file{}", i)).collect();
        assert!(validate_file_ids_count(&too_many_files).is_err());
    }
}
