//! File operation helper functions
//!
//! This module provides reusable helper functions for common file operations
//! to reduce code duplication across the codebase. All functions include
//! proper error mapping to `OpenAIError::FileError` with descriptive messages.

use crate::error::{OpenAIError, Result};
use std::path::Path;
use tokio::fs;

/// Read a file as bytes asynchronously
///
/// This helper consolidates the common pattern of reading a file as bytes
/// and mapping I/O errors to `OpenAIError::FileError` with a descriptive message
/// that includes the file path.
///
/// # Arguments
///
/// * `file_path` - Path to the file to read
///
/// # Returns
///
/// * `Result<Vec<u8>>` - The file contents as bytes, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::read_bytes;
/// # async fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// let data = read_bytes("image.png").await?;
/// println!("Read {} bytes", data.len());
/// # Ok(())
/// # }
/// ```
pub async fn read_bytes<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let path = file_path.as_ref();
    fs::read(path).await.map_err(|e| {
        OpenAIError::FileError(format!("Failed to read file {}: {}", path.display(), e))
    })
}

/// Read a file as a UTF-8 string asynchronously
///
/// This helper consolidates the common pattern of reading a file as a string
/// and mapping I/O errors to `OpenAIError::FileError` with a descriptive message
/// that includes the file path.
///
/// # Arguments
///
/// * `file_path` - Path to the file to read
///
/// # Returns
///
/// * `Result<String>` - The file contents as a string, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::read_string;
/// # async fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// let content = read_string("config.json").await?;
/// println!("Read {} characters", content.len());
/// # Ok(())
/// # }
/// ```
pub async fn read_string<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let path = file_path.as_ref();
    fs::read_to_string(path).await.map_err(|e| {
        OpenAIError::FileError(format!("Failed to read file {}: {}", path.display(), e))
    })
}

/// Write bytes to a file asynchronously
///
/// This helper consolidates the common pattern of writing bytes to a file
/// and mapping I/O errors to `OpenAIError::FileError` with a descriptive message
/// that includes the file path.
///
/// # Arguments
///
/// * `file_path` - Path to the file to write
/// * `data` - The bytes to write to the file
///
/// # Returns
///
/// * `Result<()>` - Success, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::write_bytes;
/// # async fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// let data = vec![1, 2, 3, 4];
/// write_bytes("output.bin", &data).await?;
/// println!("Written {} bytes", data.len());
/// # Ok(())
/// # }
/// ```
pub async fn write_bytes<P: AsRef<Path>>(file_path: P, data: &[u8]) -> Result<()> {
    let path = file_path.as_ref();
    fs::write(path, data).await.map_err(|e| {
        OpenAIError::FileError(format!("Failed to write file {}: {}", path.display(), e))
    })
}

/// Write a string to a file asynchronously
///
/// This helper consolidates the common pattern of writing a string to a file
/// and mapping I/O errors to `OpenAIError::FileError` with a descriptive message
/// that includes the file path.
///
/// # Arguments
///
/// * `file_path` - Path to the file to write
/// * `content` - The string content to write to the file
///
/// # Returns
///
/// * `Result<()>` - Success, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::write_string;
/// # async fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// write_string("output.txt", "Hello, world!").await?;
/// println!("File written successfully");
/// # Ok(())
/// # }
/// ```
pub async fn write_string<P: AsRef<Path>, S: AsRef<str>>(file_path: P, content: S) -> Result<()> {
    let path = file_path.as_ref();
    let content = content.as_ref();
    fs::write(path, content).await.map_err(|e| {
        OpenAIError::FileError(format!("Failed to write file {}: {}", path.display(), e))
    })
}

/// Read a file as bytes synchronously (blocking)
///
/// This helper provides a synchronous version of file reading for cases where
/// async is not available or needed. Maps I/O errors to `OpenAIError::FileError`.
///
/// # Arguments
///
/// * `file_path` - Path to the file to read
///
/// # Returns
///
/// * `Result<Vec<u8>>` - The file contents as bytes, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::read_bytes_sync;
/// # fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// let data = read_bytes_sync("image.png")?;
/// println!("Read {} bytes", data.len());
/// # Ok(())
/// # }
/// ```
pub fn read_bytes_sync<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let path = file_path.as_ref();
    std::fs::read(path).map_err(|e| {
        OpenAIError::FileError(format!("Failed to read file {}: {}", path.display(), e))
    })
}

/// Read a file as a UTF-8 string synchronously (blocking)
///
/// This helper provides a synchronous version of string file reading for cases where
/// async is not available or needed. Maps I/O errors to `OpenAIError::FileError`.
///
/// # Arguments
///
/// * `file_path` - Path to the file to read
///
/// # Returns
///
/// * `Result<String>` - The file contents as a string, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::read_string_sync;
/// # fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// let content = read_string_sync("config.json")?;
/// println!("Read {} characters", content.len());
/// # Ok(())
/// # }
/// ```
pub fn read_string_sync<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let path = file_path.as_ref();
    std::fs::read_to_string(path).map_err(|e| {
        OpenAIError::FileError(format!("Failed to read file {}: {}", path.display(), e))
    })
}

/// Write bytes to a file synchronously (blocking)
///
/// This helper provides a synchronous version of file writing for cases where
/// async is not available or needed. Maps I/O errors to `OpenAIError::FileError`.
///
/// # Arguments
///
/// * `file_path` - Path to the file to write
/// * `data` - The bytes to write to the file
///
/// # Returns
///
/// * `Result<()>` - Success, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::write_bytes_sync;
/// # fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// let data = vec![1, 2, 3, 4];
/// write_bytes_sync("output.bin", &data)?;
/// println!("Written {} bytes", data.len());
/// # Ok(())
/// # }
/// ```
pub fn write_bytes_sync<P: AsRef<Path>>(file_path: P, data: &[u8]) -> Result<()> {
    let path = file_path.as_ref();
    std::fs::write(path, data).map_err(|e| {
        OpenAIError::FileError(format!("Failed to write file {}: {}", path.display(), e))
    })
}

/// Write a string to a file synchronously (blocking)
///
/// This helper provides a synchronous version of string file writing for cases where
/// async is not available or needed. Maps I/O errors to `OpenAIError::FileError`.
///
/// # Arguments
///
/// * `file_path` - Path to the file to write
/// * `content` - The string content to write to the file
///
/// # Returns
///
/// * `Result<()>` - Success, or a FileError on failure
///
/// # Example
///
/// ```rust,no_run
/// use openai_rust_sdk::helpers::file_operations::write_string_sync;
/// # fn example() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// write_string_sync("output.txt", "Hello, world!")?;
/// println!("File written successfully");
/// # Ok(())
/// # }
/// ```
pub fn write_string_sync<P: AsRef<Path>, S: AsRef<str>>(file_path: P, content: S) -> Result<()> {
    let path = file_path.as_ref();
    let content = content.as_ref();
    std::fs::write(path, content).map_err(|e| {
        OpenAIError::FileError(format!("Failed to write file {}: {}", path.display(), e))
    })
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
