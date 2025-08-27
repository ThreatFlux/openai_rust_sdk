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

    macro_rules! test_file_op {
        (read: $s:ident $t:ident, $f:ident, $d:expr, $e:expr) => { test_file_op!(@r: $s $t, $f, $d, $e); };
        (write: $s:ident $t:ident, $f:ident, $d:expr, $v:ident) => { test_file_op!(@w: $s $t, $f, $d, $v); };
        (not_found: $s:ident $t:ident, $f:ident, $o:literal) => { test_file_op!(@e: $s $t, $f, "/nonexistent/file.txt", $o); };
        (perm_denied: $s:ident $t:ident, $f:ident, $d:expr, $o:literal) => { test_file_op!(@p: $s $t, $f, $d, $o); };
        (@r: async $t:ident, $f:ident, $d:expr, $e:expr) => {
            #[tokio::test] async fn $t() { let tmp = NamedTempFile::new().unwrap(); std::fs::write(&tmp, $d).unwrap(); assert_eq!($f(&tmp).await.unwrap(), $e); }
        };
        (@r: sync $t:ident, $f:ident, $d:expr, $e:expr) => {
            #[test] fn $t() { let tmp = NamedTempFile::new().unwrap(); std::fs::write(&tmp, $d).unwrap(); assert_eq!($f(&tmp).unwrap(), $e); }
        };
        (@w: async $t:ident, $f:ident, $d:expr, $v:ident) => {
            #[tokio::test] async fn $t() { let tmp = NamedTempFile::new().unwrap(); $f(&tmp, $d).await.unwrap(); assert_eq!(std::fs::$v(&tmp).unwrap(), $d); }
        };
        (@w: sync $t:ident, $f:ident, $d:expr, $v:ident) => {
            #[test] fn $t() { let tmp = NamedTempFile::new().unwrap(); $f(&tmp, $d).unwrap(); assert_eq!(std::fs::$v(&tmp).unwrap(), $d); }
        };
        (@e: async $t:ident, $f:ident, $p:literal, $o:literal) => {
            #[tokio::test] async fn $t() { match $f($p).await.unwrap_err() { OpenAIError::FileError(msg) => { assert!(msg.contains(concat!("Failed to ", $o, " file"))); assert!(msg.contains($p)); } _ => panic!("Expected FileError"), } }
        };
        (@e: sync $t:ident, $f:ident, $p:literal, $o:literal) => {
            #[test] fn $t() { match $f($p).unwrap_err() { OpenAIError::FileError(msg) => { assert!(msg.contains(concat!("Failed to ", $o, " file"))); assert!(msg.contains($p)); } _ => panic!("Expected FileError"), } }
        };
        (@p: async $t:ident, $f:ident, $d:expr, $o:literal) => {
            #[tokio::test] async fn $t() { match $f("/root/nonexistent_dir/file.txt", $d).await.unwrap_err() { OpenAIError::FileError(msg) => { assert!(msg.contains(concat!("Failed to ", $o, " file"))); assert!(msg.contains("/root/nonexistent_dir/file.txt")); } _ => panic!("Expected FileError"), } }
        };
        (@p: sync $t:ident, $f:ident, $d:expr, $o:literal) => {
            #[test] fn $t() { match $f("/root/nonexistent_dir/file.txt", $d).unwrap_err() { OpenAIError::FileError(msg) => { assert!(msg.contains(concat!("Failed to ", $o, " file"))); assert!(msg.contains("/root/nonexistent_dir/file.txt")); } _ => panic!("Expected FileError"), } }
        };
    }

    test_file_op!(read: async test_read_bytes_success, read_bytes, b"Hello, world!", b"Hello, world!");
    test_file_op!(read: async test_read_string_success, read_string, "Hello, world!", "Hello, world!");
    test_file_op!(write: async test_write_bytes_success, write_bytes, b"Hello, world!", read);
    test_file_op!(write: async test_write_string_success, write_string, "Hello, world!", read_to_string);
    test_file_op!(read: sync test_read_bytes_sync_success, read_bytes_sync, b"Hello, world!", b"Hello, world!");
    test_file_op!(read: sync test_read_string_sync_success, read_string_sync, "Hello, world!", "Hello, world!");
    test_file_op!(write: sync test_write_bytes_sync_success, write_bytes_sync, b"Hello, world!", read);
    test_file_op!(write: sync test_write_string_sync_success, write_string_sync, "Hello, world!", read_to_string);
    test_file_op!(not_found: async test_read_bytes_not_found, read_bytes, "read");
    test_file_op!(not_found: async test_read_string_not_found, read_string, "read");
    test_file_op!(not_found: sync test_read_bytes_sync_not_found, read_bytes_sync, "read");
    test_file_op!(not_found: sync test_read_string_sync_not_found, read_string_sync, "read");
    test_file_op!(perm_denied: async test_write_bytes_permission_denied, write_bytes, b"test", "write");
    test_file_op!(perm_denied: async test_write_string_permission_denied, write_string, "test", "write");
    test_file_op!(perm_denied: sync test_write_bytes_sync_permission_denied, write_bytes_sync, b"test", "write");
    test_file_op!(perm_denied: sync test_write_string_sync_permission_denied, write_string_sync, "test", "write");

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
