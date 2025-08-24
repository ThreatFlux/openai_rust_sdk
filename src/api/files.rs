//! # OpenAI Files API Client
//!
//! This module provides a complete implementation of OpenAI's Files API for uploading,
//! managing, and downloading files that can be used with various OpenAI services.
//!
//! ## Features
//!
//! - **File Upload**: Upload files with different purposes (fine-tuning, assistants, batch, etc.)
//! - **File Management**: List, retrieve, and delete files
//! - **File Content**: Download file content and metadata
//! - **Purpose Filtering**: Filter files by their intended use
//! - **File Validation**: Automatic validation of file types and sizes
//!
//! ## Supported File Purposes
//!
//! - `fine-tune`: Training data for custom models
//! - `assistants`: Documents for Assistants API retrieval
//! - `batch`: Input files for Batch API processing
//! - `user_data`: General purpose file storage
//! - `responses`: API response storage
//! - `vision`: Image files for analysis
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::files::FilesApi;
//! use openai_rust_sdk::models::files::{FileUploadRequest, FilePurpose};
//! use std::path::Path;
//!
//! # tokio_test::block_on(async {
//! let api = FilesApi::new("your-api-key")?;
//!
//! // Upload a file for fine-tuning
//! let upload_request = FileUploadRequest::from_file_path(
//!     Path::new("training_data.jsonl"),
//!     FilePurpose::FineTune
//! ).await?;
//! let file = api.upload_file(upload_request).await?;
//! println!("Uploaded file: {}", file.id);
//!
//! // List all files
//! let files = api.list_files(None).await?;
//! println!("Found {} files", files.data.len());
//!
//! // Download file content
//! let content = api.retrieve_file_content(&file.id).await?;
//! println!("File content: {}", content);
//!
//! // Delete the file
//! let deleted = api.delete_file(&file.id).await?;
//! println!("File deleted: {}", deleted.deleted);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::{OpenAIError, Result};
use crate::models::files::{
    File, FileDeleteResponse, FilePurpose, FileUploadRequest, ListFilesParams, ListFilesResponse,
};
use reqwest::multipart;
use std::collections::HashMap;

/// `OpenAI` Files API client for file management operations
#[derive(Debug, Clone)]
pub struct FilesApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for FilesApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl FilesApi {
    /// Creates a new Files API client with custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    /// * `base_url` - Custom base URL for the API
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Uploads a file to `OpenAI`
    ///
    /// The file will be uploaded with the specified purpose and can then be used
    /// with the appropriate `OpenAI` APIs.
    ///
    /// # Arguments
    ///
    /// * `request` - File upload request containing file data, filename, and purpose
    ///
    /// # Returns
    ///
    /// Returns a `File` object containing the uploaded file's metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    /// use openai_rust_sdk::models::files::{FileUploadRequest, FilePurpose};
    /// use std::path::Path;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let request = FileUploadRequest::from_file_path(
    ///     Path::new("data.jsonl"),
    ///     FilePurpose::FineTune
    /// ).await?;
    /// let file = api.upload_file(request).await?;
    /// println!("Uploaded file: {}", file.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn upload_file(&self, request: FileUploadRequest) -> Result<File> {
        // Validate the request
        request.validate().map_err(OpenAIError::InvalidRequest)?;

        // Create multipart form
        let mime_type = request.mime_type();
        let filename = request.filename.clone();
        let purpose = request.purpose.to_string();

        let part = multipart::Part::bytes(request.file)
            .file_name(filename)
            .mime_str(mime_type)
            .map_err(|e| OpenAIError::RequestError(format!("Failed to create file part: {e}")))?;

        let form = multipart::Form::new()
            .part("file", part)
            .text("purpose", purpose);

        self.http_client.post_multipart("/v1/files", form).await
    }

    /// Lists files belonging to the user's organization
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a `ListFilesResponse` containing the list of files
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    /// use openai_rust_sdk::models::files::{ListFilesParams, FilePurpose};
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    ///
    /// // List all files
    /// let all_files = api.list_files(None).await?;
    ///
    /// // List only fine-tuning files
    /// let params = ListFilesParams::new().with_purpose(FilePurpose::FineTune);
    /// let fine_tune_files = api.list_files(Some(params)).await?;
    ///
    /// println!("Found {} total files, {} fine-tune files",
    ///          all_files.data.len(), fine_tune_files.data.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn list_files(&self, params: Option<ListFilesParams>) -> Result<ListFilesResponse> {
        match params {
            Some(params) => {
                let query_params = params.to_query_params();
                self.http_client
                    .get_with_query("/v1/files", &query_params)
                    .await
            }
            None => self.http_client.get("/v1/files").await,
        }
    }

    /// Retrieves a file's metadata by its ID
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `File` object containing the file's metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let file = api.retrieve_file("file-abc123").await?;
    /// println!("File: {} ({})", file.filename, file.size_human_readable());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn retrieve_file(&self, file_id: &str) -> Result<File> {
        self.http_client.get(&format!("/v1/files/{file_id}")).await
    }

    /// Downloads the content of a file
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to download
    ///
    /// # Returns
    ///
    /// Returns the content of the file as a string
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let content = api.retrieve_file_content("file-abc123").await?;
    /// println!("File content length: {} characters", content.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn retrieve_file_content(&self, file_id: &str) -> Result<String> {
        self.http_client
            .get_text(&format!("/v1/files/{file_id}/content"))
            .await
    }

    /// Downloads file content as bytes
    ///
    /// This is useful for binary files like images.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to download
    ///
    /// # Returns
    ///
    /// Returns the content of the file as bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let bytes = api.retrieve_file_bytes("file-abc123").await?;
    /// println!("File size: {} bytes", bytes.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn retrieve_file_bytes(&self, file_id: &str) -> Result<Vec<u8>> {
        self.http_client
            .get_bytes(&format!("/v1/files/{file_id}/content"))
            .await
    }

    /// Downloads file content and saves it to a local file
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to download
    /// * `output_path` - Path where the file will be saved
    ///
    /// # Returns
    ///
    /// Returns the number of bytes written to the file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    /// use std::path::Path;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let bytes_written = api.download_file("file-abc123", Path::new("downloaded_file.jsonl")).await?;
    /// println!("Downloaded {} bytes", bytes_written);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn download_file(
        &self,
        file_id: &str,
        output_path: &std::path::Path,
    ) -> Result<usize> {
        let content_bytes = self.retrieve_file_bytes(file_id).await?;

        tokio::fs::write(output_path, &content_bytes)
            .await
            .map_err(|e| {
                OpenAIError::FileError(format!(
                    "Failed to write file to {}: {}",
                    output_path.display(),
                    e
                ))
            })?;

        Ok(content_bytes.len())
    }

    /// Deletes a file
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to delete
    ///
    /// # Returns
    ///
    /// Returns a `FileDeleteResponse` indicating whether the deletion was successful
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let result = api.delete_file("file-abc123").await?;
    /// if result.deleted {
    ///     println!("File {} was successfully deleted", result.id);
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn delete_file(&self, file_id: &str) -> Result<FileDeleteResponse> {
        self.http_client
            .delete(&format!("/v1/files/{file_id}"))
            .await
    }

    /// Lists files by purpose
    ///
    /// This is a convenience method for filtering files by their purpose.
    ///
    /// # Arguments
    ///
    /// * `purpose` - The purpose to filter by
    /// * `limit` - Optional limit on the number of files to return
    ///
    /// # Returns
    ///
    /// Returns a `ListFilesResponse` containing files with the specified purpose
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    /// use openai_rust_sdk::models::files::FilePurpose;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let fine_tune_files = api.list_files_by_purpose(FilePurpose::FineTune, Some(10)).await?;
    /// println!("Found {} fine-tuning files", fine_tune_files.data.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn list_files_by_purpose(
        &self,
        purpose: FilePurpose,
        limit: Option<u32>,
    ) -> Result<ListFilesResponse> {
        let mut params = ListFilesParams::new().with_purpose(purpose);
        if let Some(limit) = limit {
            params = params.with_limit(limit);
        }
        self.list_files(Some(params)).await
    }

    /// Gets file usage statistics
    ///
    /// Returns statistics about files in the user's organization.
    ///
    /// # Returns
    ///
    /// Returns a map of file purposes to their counts and total sizes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let stats = api.get_file_usage_stats().await?;
    /// for (purpose, (count, size)) in stats {
    ///     println!("{}: {} files, {} bytes", purpose, count, size);
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn get_file_usage_stats(&self) -> Result<HashMap<String, (usize, u64)>> {
        let files = self.list_files(None).await?;
        let mut stats: HashMap<String, (usize, u64)> = HashMap::new();

        for file in files.data {
            let entry = stats.entry(file.purpose.clone()).or_insert((0, 0));
            entry.0 += 1; // count
            entry.1 += file.bytes; // total size
        }

        Ok(stats)
    }

    /// Bulk deletes files by purpose
    ///
    /// This is a convenience method for deleting multiple files with the same purpose.
    ///
    /// # Arguments
    ///
    /// * `purpose` - The purpose of files to delete
    /// * `max_deletions` - Optional limit on the number of files to delete
    ///
    /// # Returns
    ///
    /// Returns a vector of file IDs that were successfully deleted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    /// use openai_rust_sdk::models::files::FilePurpose;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let deleted_ids = api.bulk_delete_files_by_purpose(FilePurpose::UserData, Some(5)).await?;
    /// println!("Deleted {} files", deleted_ids.len());
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn bulk_delete_files_by_purpose(
        &self,
        purpose: FilePurpose,
        max_deletions: Option<usize>,
    ) -> Result<Vec<String>> {
        let files = self.list_files_by_purpose(purpose, None).await?;
        let mut deleted_ids = Vec::new();
        let limit = max_deletions.unwrap_or(files.data.len());

        for file in files.data.iter().take(limit) {
            match self.delete_file(&file.id).await {
                Ok(delete_response) if delete_response.deleted => {
                    deleted_ids.push(file.id.clone());
                }
                Ok(_) => {
                    eprintln!("Failed to delete file {}: deletion not confirmed", file.id);
                }
                Err(e) => {
                    eprintln!("Failed to delete file {}: {}", file.id, e);
                }
            }
        }

        Ok(deleted_ids)
    }

    /// Validates that a file exists and is accessible
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to validate
    ///
    /// # Returns
    ///
    /// Returns `true` if the file exists and is accessible, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// if api.file_exists("file-abc123").await? {
    ///     println!("File exists and is accessible");
    /// } else {
    ///     println!("File does not exist or is not accessible");
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn file_exists(&self, file_id: &str) -> Result<bool> {
        match self.retrieve_file(file_id).await {
            Ok(_) => Ok(true),
            Err(OpenAIError::ApiError { status: 404, .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Uploads multiple files in parallel
    ///
    /// This method uploads multiple files concurrently for improved performance.
    ///
    /// # Arguments
    ///
    /// * `requests` - Vector of file upload requests
    /// * `max_concurrent` - Maximum number of concurrent uploads (default: 5)
    ///
    /// # Returns
    ///
    /// Returns a vector of results, where each result is either a successful File or an error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::files::FilesApi;
    /// use openai_rust_sdk::models::files::{FileUploadRequest, FilePurpose};
    ///
    /// # tokio_test::block_on(async {
    /// let api = FilesApi::new("your-api-key")?;
    /// let requests = vec![
    ///     FileUploadRequest::new(b"content1".to_vec(), "file1.txt".to_string(), FilePurpose::UserData),
    ///     FileUploadRequest::new(b"content2".to_vec(), "file2.txt".to_string(), FilePurpose::UserData),
    /// ];
    /// let results = api.upload_files_parallel(requests, None).await;
    /// println!("Upload results: {:?}", results);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn upload_files_parallel(
        &self,
        requests: Vec<FileUploadRequest>,
        max_concurrent: Option<usize>,
    ) -> Vec<Result<File>> {
        let semaphore =
            std::sync::Arc::new(tokio::sync::Semaphore::new(max_concurrent.unwrap_or(5)));
        let api = std::sync::Arc::new(self.clone());

        let handles: Vec<_> = requests
            .into_iter()
            .map(|request| {
                let semaphore = semaphore.clone();
                let api = api.clone();
                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await;
                    api.upload_file(request).await
                })
            })
            .collect();

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(OpenAIError::RequestError(format!(
                    "Task join error: {e}"
                )))),
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::files::{FilePurpose, ListFilesParams, SortOrder};

    #[test]
    fn test_files_api_creation() {
        let api = FilesApi::new("test-key").unwrap();
        assert_eq!(api.http_client.api_key(), "test-key");
        assert_eq!(api.http_client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_files_api_custom_base_url() {
        let api = FilesApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.http_client.api_key(), "test-key");
        assert_eq!(api.http_client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_file_upload_request_validation() {
        let valid_request = FileUploadRequest::new(
            b"test content".to_vec(),
            "test.jsonl".to_string(),
            FilePurpose::FineTune,
        );
        assert!(valid_request.validate().is_ok());

        let empty_file_request =
            FileUploadRequest::new(Vec::new(), "test.jsonl".to_string(), FilePurpose::FineTune);
        assert!(empty_file_request.validate().is_err());

        let large_file_request = FileUploadRequest::new(
            vec![0; 300 * 1024 * 1024], // 300MB
            "large.jsonl".to_string(),
            FilePurpose::FineTune,
        );
        assert!(large_file_request.validate().is_err());
    }

    #[test]
    fn test_list_files_params_query() {
        let params = ListFilesParams::new()
            .with_purpose(FilePurpose::FineTune)
            .with_limit(10)
            .with_order(SortOrder::Desc);

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("purpose".to_string(), "fine-tune".to_string())));
        assert!(query_params.contains(&("limit".to_string(), "10".to_string())));
        assert!(query_params.contains(&("order".to_string(), "desc".to_string())));
    }

    #[test]
    fn test_file_purpose_validation() {
        // Test that different file types are validated correctly for different purposes
        let jsonl_request = FileUploadRequest::new(
            b"test content".to_vec(),
            "test.jsonl".to_string(),
            FilePurpose::FineTune,
        );
        assert!(jsonl_request.validate().is_ok());

        let txt_request = FileUploadRequest::new(
            b"test content".to_vec(),
            "test.txt".to_string(),
            FilePurpose::FineTune,
        );
        assert!(txt_request.validate().is_err());

        let image_request = FileUploadRequest::new(
            b"fake image content".to_vec(),
            "test.png".to_string(),
            FilePurpose::Vision,
        );
        assert!(image_request.validate().is_ok());

        let wrong_image_request = FileUploadRequest::new(
            b"fake image content".to_vec(),
            "test.txt".to_string(),
            FilePurpose::Vision,
        );
        assert!(wrong_image_request.validate().is_err());
    }

    #[test]
    fn test_mime_type_detection() {
        let jsonl_request = FileUploadRequest::new(
            b"test".to_vec(),
            "test.jsonl".to_string(),
            FilePurpose::FineTune,
        );
        assert_eq!(jsonl_request.mime_type(), "application/jsonl");

        let regular_json_request = FileUploadRequest::new(
            b"test".to_vec(),
            "test.json".to_string(),
            FilePurpose::UserData,
        );
        assert_eq!(regular_json_request.mime_type(), "application/json");

        let png_request = FileUploadRequest::new(
            b"test".to_vec(),
            "test.png".to_string(),
            FilePurpose::Vision,
        );
        assert_eq!(png_request.mime_type(), "image/png");

        let unknown_request = FileUploadRequest::new(
            b"test".to_vec(),
            "test.unknown".to_string(),
            FilePurpose::UserData,
        );
        assert_eq!(unknown_request.mime_type(), "application/octet-stream");
    }

    #[tokio::test]
    async fn test_file_upload_request_from_path() {
        // This test would require actual file I/O, so we'll just test the structure
        let request_result = FileUploadRequest::from_file_path(
            std::path::Path::new("nonexistent.jsonl"),
            FilePurpose::FineTune,
        )
        .await;

        // Should fail because file doesn't exist
        assert!(request_result.is_err());
    }

    #[test]
    fn test_file_purpose_supports_checks() {
        assert!(FilePurpose::FineTune.supports_text());
        assert!(!FilePurpose::FineTune.supports_images());

        assert!(FilePurpose::Vision.supports_images());
        assert!(!FilePurpose::Vision.supports_text());

        assert!(FilePurpose::UserData.supports_text());
        assert!(FilePurpose::UserData.supports_images());

        assert!(FilePurpose::Assistants.supports_text());
        assert!(!FilePurpose::Assistants.supports_images());
    }

    // Integration tests would go here if we had a test API key
    // They would test actual API calls against a test environment
}
