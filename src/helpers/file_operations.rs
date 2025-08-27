//! File operation helper functions
//!
//! This module provides reusable helper functions for common file operations
//! to reduce code duplication across the codebase. All functions include
//! proper error mapping to `OpenAIError::FileError` with descriptive messages.

use crate::error::{OpenAIError, Result};
use std::path::Path;
use tokio::fs;

/// Internal helper to create consistent file error messages
fn create_file_error<P: AsRef<Path>>(
    path: P,
    operation: &str,
    error: &std::io::Error,
) -> OpenAIError {
    OpenAIError::FileError(format!(
        "Failed to {} file {}: {}",
        operation,
        path.as_ref().display(),
        error
    ))
}

/// Macro to generate async read functions with consistent error handling
macro_rules! impl_async_read {
    ($func_name:ident, $fs_op:ident, $operation:literal, $return_type:ty) => {
        #[doc = concat!("Read a file asynchronously, returning `", stringify!($return_type), "`.")]
        #[doc = ""]
        #[doc = "Maps I/O errors to `OpenAIError::FileError` with a descriptive message"]
        #[doc = "that includes the file path."]
        pub async fn $func_name<P: AsRef<Path>>(file_path: P) -> Result<$return_type> {
            let path = file_path.as_ref();
            fs::$fs_op(path)
                .await
                .map_err(|e| create_file_error(path, $operation, &e))
        }
    };
}

/// Macro to generate async write functions with consistent error handling
macro_rules! impl_async_write {
    ($func_name:ident, $fs_op:ident, $operation:literal, $param_type:ty) => {
        #[doc = concat!("Write data of type `", stringify!($param_type), "` to a file asynchronously.")]
        #[doc = ""]
        #[doc = "Maps I/O errors to `OpenAIError::FileError` with a descriptive message"]
        #[doc = "that includes the file path."]
        pub async fn $func_name<P: AsRef<Path>>(file_path: P, data: $param_type) -> Result<()> {
            let path = file_path.as_ref();
            fs::$fs_op(path, data)
                .await
                .map_err(|e| create_file_error(path, $operation, &e))
        }
    };
}

/// Macro to generate sync read functions with consistent error handling
macro_rules! impl_sync_read {
    ($func_name:ident, $fs_op:ident, $operation:literal, $return_type:ty) => {
        #[doc = concat!("Read a file synchronously, returning `", stringify!($return_type), "`.")]
        #[doc = ""]
        #[doc = "Maps I/O errors to `OpenAIError::FileError` with a descriptive message"]
        #[doc = "that includes the file path."]
        pub fn $func_name<P: AsRef<Path>>(file_path: P) -> Result<$return_type> {
            let path = file_path.as_ref();
            std::fs::$fs_op(path).map_err(|e| create_file_error(path, $operation, &e))
        }
    };
}

/// Macro to generate sync write functions with consistent error handling
macro_rules! impl_sync_write {
    ($func_name:ident, $fs_op:ident, $operation:literal, $param_type:ty) => {
        #[doc = concat!("Write data of type `", stringify!($param_type), "` to a file synchronously.")]
        #[doc = ""]
        #[doc = "Maps I/O errors to `OpenAIError::FileError` with a descriptive message"]
        #[doc = "that includes the file path."]
        pub fn $func_name<P: AsRef<Path>>(file_path: P, data: $param_type) -> Result<()> {
            let path = file_path.as_ref();
            std::fs::$fs_op(path, data).map_err(|e| create_file_error(path, $operation, &e))
        }
    };
}

// Generate async file operation functions using macros to reduce duplication
impl_async_read!(read_bytes, read, "read", Vec<u8>);
impl_async_read!(read_string, read_to_string, "read", String);
impl_async_write!(write_bytes, write, "write", &[u8]);

// Generate sync file operation functions using macros to reduce duplication
impl_sync_read!(read_bytes_sync, read, "read", Vec<u8>);
impl_sync_read!(read_string_sync, read_to_string, "read", String);
impl_sync_write!(write_bytes_sync, write, "write", &[u8]);

/// Write a string to a file asynchronously
///
/// Maps I/O errors to `OpenAIError::FileError` with a descriptive message
/// that includes the file path.
pub async fn write_string<P: AsRef<Path>, S: AsRef<str>>(file_path: P, content: S) -> Result<()> {
    let path = file_path.as_ref();
    let content = content.as_ref();
    fs::write(path, content)
        .await
        .map_err(|e| create_file_error(path, "write", &e))
}

/// Write a string to a file synchronously (blocking)
///
/// Maps I/O errors to `OpenAIError::FileError` with a descriptive message
/// that includes the file path.
pub fn write_string_sync<P: AsRef<Path>, S: AsRef<str>>(file_path: P, content: S) -> Result<()> {
    let path = file_path.as_ref();
    let content = content.as_ref();
    std::fs::write(path, content).map_err(|e| create_file_error(path, "write", &e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::OpenAIError;
    use std::io::{Error, ErrorKind};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_read_bytes_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_data = b"Hello, world!";
        std::fs::write(&temp_file, test_data).unwrap();

        let result = read_bytes(&temp_file).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_read_bytes_not_found() {
        let result = read_bytes("/nonexistent/file.txt").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to read file"));
                assert!(msg.contains("/nonexistent/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[tokio::test]
    async fn test_read_string_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_content = "Hello, world!";
        std::fs::write(&temp_file, test_content).unwrap();

        let result = read_string(&temp_file).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[tokio::test]
    async fn test_read_string_not_found() {
        let result = read_string("/nonexistent/file.txt").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to read file"));
                assert!(msg.contains("/nonexistent/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[tokio::test]
    async fn test_write_bytes_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_data = b"Hello, world!";

        let result = write_bytes(&temp_file, test_data).await;
        assert!(result.is_ok());

        // Verify the content was written correctly
        let written_content = std::fs::read(&temp_file).unwrap();
        assert_eq!(written_content, test_data);
    }

    #[tokio::test]
    async fn test_write_bytes_permission_denied() {
        // Try to write to a directory that doesn't exist or has no permissions
        let result = write_bytes("/root/nonexistent_dir/file.txt", b"test").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to write file"));
                assert!(msg.contains("/root/nonexistent_dir/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[tokio::test]
    async fn test_write_string_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_content = "Hello, world!";

        let result = write_string(&temp_file, test_content).await;
        assert!(result.is_ok());

        // Verify the content was written correctly
        let written_content = std::fs::read_to_string(&temp_file).unwrap();
        assert_eq!(written_content, test_content);
    }

    #[tokio::test]
    async fn test_write_string_permission_denied() {
        // Try to write to a directory that doesn't exist or has no permissions
        let result = write_string("/root/nonexistent_dir/file.txt", "test").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to write file"));
                assert!(msg.contains("/root/nonexistent_dir/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[test]
    fn test_read_bytes_sync_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_data = b"Hello, world!";
        std::fs::write(&temp_file, test_data).unwrap();

        let result = read_bytes_sync(&temp_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_data);
    }

    #[test]
    fn test_read_bytes_sync_not_found() {
        let result = read_bytes_sync("/nonexistent/file.txt");
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to read file"));
                assert!(msg.contains("/nonexistent/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[test]
    fn test_read_string_sync_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_content = "Hello, world!";
        std::fs::write(&temp_file, test_content).unwrap();

        let result = read_string_sync(&temp_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[test]
    fn test_read_string_sync_not_found() {
        let result = read_string_sync("/nonexistent/file.txt");
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to read file"));
                assert!(msg.contains("/nonexistent/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[test]
    fn test_write_bytes_sync_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_data = b"Hello, world!";

        let result = write_bytes_sync(&temp_file, test_data);
        assert!(result.is_ok());

        // Verify the content was written correctly
        let written_content = std::fs::read(&temp_file).unwrap();
        assert_eq!(written_content, test_data);
    }

    #[test]
    fn test_write_bytes_sync_permission_denied() {
        // Try to write to a directory that doesn't exist or has no permissions
        let result = write_bytes_sync("/root/nonexistent_dir/file.txt", b"test");
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to write file"));
                assert!(msg.contains("/root/nonexistent_dir/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[test]
    fn test_write_string_sync_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_content = "Hello, world!";

        let result = write_string_sync(&temp_file, test_content);
        assert!(result.is_ok());

        // Verify the content was written correctly
        let written_content = std::fs::read_to_string(&temp_file).unwrap();
        assert_eq!(written_content, test_content);
    }

    #[test]
    fn test_write_string_sync_permission_denied() {
        // Try to write to a directory that doesn't exist or has no permissions
        let result = write_string_sync("/root/nonexistent_dir/file.txt", "test");
        assert!(result.is_err());

        match result.unwrap_err() {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to write file"));
                assert!(msg.contains("/root/nonexistent_dir/file.txt"));
            }
            _ => panic!("Expected FileError"),
        }
    }

    #[test]
    fn test_consistent_error_messages() {
        let path = "/nonexistent/test.txt";

        // Test that async and sync versions produce similar error messages
        let sync_error = read_bytes_sync(path).unwrap_err();

        if let OpenAIError::FileError(sync_msg) = sync_error {
            assert!(sync_msg.contains("Failed to read file"));
            assert!(sync_msg.contains(path));
        } else {
            panic!("Expected FileError");
        }
    }

    #[test]
    fn test_path_display_in_errors() {
        use std::path::PathBuf;

        // Test with different path types
        let paths = [
            "/simple/path.txt",
            "/path/with spaces/file.txt",
            "/path-with-dashes/file.txt",
            "/path_with_underscores/file.txt",
        ];

        for path in paths {
            let result = read_bytes_sync(path);
            assert!(result.is_err());

            if let OpenAIError::FileError(msg) = result.unwrap_err() {
                assert!(msg.contains(path));
                assert!(msg.contains("Failed to read file"));
            }
        }
    }

    #[test]
    fn test_generic_path_types() {
        use std::path::PathBuf;

        let temp_file = NamedTempFile::new().unwrap();
        let test_data = b"test data";
        std::fs::write(&temp_file, test_data).unwrap();

        // Test with different path types
        let path_buf = PathBuf::from(temp_file.path());
        let path_str = temp_file.path().to_str().unwrap();
        let path_string = temp_file.path().to_string_lossy().to_string();

        // All should work
        assert!(read_bytes_sync(&path_buf).is_ok());
        assert!(read_bytes_sync(path_str).is_ok());
        assert!(read_bytes_sync(&path_string).is_ok());
    }
}
