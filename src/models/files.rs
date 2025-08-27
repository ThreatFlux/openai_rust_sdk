//! # Files API Models
//!
//! This module provides data structures for OpenAI's Files API, which allows you to
//! upload and manage files that can be used with various OpenAI services.
//!
//! ## Overview
//!
//! The Files API supports uploading files for use with:
//! - **Fine-tuning**: Training data for custom models
//! - **Assistants**: Documents for retrieval and knowledge
//! - **Batch processing**: Input files for batch API
//! - **Vision**: Images for analysis
//! - **User data**: General purpose file storage
//!
//! ## File Purposes
//!
//! Each file must be uploaded with a specific purpose that determines how it can be used:
//!
//! - `fine-tune`: For fine-tuning datasets
//! - `assistants`: For Assistants API retrieval
//! - `batch`: For Batch API input files
//! - `user_data`: For general file storage
//! - `responses`: For storing API responses
//! - `vision`: For image analysis
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::files::{FileUploadRequest, FilePurpose};
//!
//! // Create a file upload request for fine-tuning
//! let upload_request = FileUploadRequest {
//!     file: vec![/* file bytes */],
//!     filename: "training_data.jsonl".to_string(),
//!     purpose: FilePurpose::FineTune,
//! };
//! ```

use crate::api::base::Validate;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// Generate the bytes to human readable function
crate::impl_bytes_to_human_readable!();

/// Purpose for which a file is being uploaded
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    /// Fine-tuning training data
    FineTune,
    /// Files for Assistants API (retrieval, knowledge base)
    Assistants,
    /// Batch processing input files
    Batch,
    /// General user data storage
    UserData,
    /// API response storage
    Responses,
    /// Vision/image analysis files
    Vision,
    /// Fine-tuning results and outputs
    FineTuneResults,
    /// Assistants retrieval files
    AssistantsOutput,
}

crate::impl_enum_display! {
    FilePurpose {
        FineTune => "fine-tune",
        Assistants => "assistants",
        Batch => "batch",
        UserData => "user_data",
        Responses => "responses",
        Vision => "vision",
        FineTuneResults => "fine-tune-results",
        AssistantsOutput => "assistants_output",
    }
}

impl FromStr for FilePurpose {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fine-tune" => Ok(FilePurpose::FineTune),
            "assistants" => Ok(FilePurpose::Assistants),
            "batch" => Ok(FilePurpose::Batch),
            "user_data" => Ok(FilePurpose::UserData),
            "responses" => Ok(FilePurpose::Responses),
            "vision" => Ok(FilePurpose::Vision),
            "fine-tune-results" => Ok(FilePurpose::FineTuneResults),
            "assistants_output" => Ok(FilePurpose::AssistantsOutput),
            _ => Err(format!("Unknown file purpose: {}", s)),
        }
    }
}

impl FilePurpose {
    /// Returns all valid file purposes
    #[must_use]
    pub fn all() -> Vec<FilePurpose> {
        vec![
            FilePurpose::FineTune,
            FilePurpose::Assistants,
            FilePurpose::Batch,
            FilePurpose::UserData,
            FilePurpose::Responses,
            FilePurpose::Vision,
            FilePurpose::FineTuneResults,
            FilePurpose::AssistantsOutput,
        ]
    }

    /// Check if this purpose supports text files
    #[must_use]
    pub fn supports_text(&self) -> bool {
        matches!(
            self,
            FilePurpose::FineTune
                | FilePurpose::Assistants
                | FilePurpose::Batch
                | FilePurpose::UserData
                | FilePurpose::Responses
        )
    }

    /// Check if this purpose supports image files
    #[must_use]
    pub fn supports_images(&self) -> bool {
        matches!(self, FilePurpose::Vision | FilePurpose::UserData)
    }
}

/// Status of a file upload or processing
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    /// File has been uploaded successfully
    Uploaded,
    /// File is being processed
    Processed,
    /// File processing failed
    Error,
    /// File has been deleted
    Deleted,
}

crate::impl_enum_display! {
    FileStatus {
        Uploaded => "uploaded",
        Processed => "processed",
        Error => "error",
        Deleted => "deleted",
    }
}

/// Represents a file object in `OpenAI`'s system
#[derive(Debug, Clone, Ser, De)]
pub struct File {
    /// Unique identifier for the file
    pub id: String,
    /// The object type, which is always "file"
    pub object: String,
    /// The size of the file, in bytes
    pub bytes: u64,
    /// The Unix timestamp (in seconds) for when the file was created
    pub created_at: u64,
    /// The name of the file
    pub filename: String,
    /// The intended purpose of the file
    pub purpose: String,
    /// The current status of the file
    #[serde(default = "default_file_status")]
    pub status: String,
    /// Additional details about the file's status, if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

crate::impl_default_object_type!(default_file_status, "uploaded");

impl File {
    /// Get the file purpose as a typed enum
    #[must_use]
    pub fn purpose_enum(&self) -> Option<FilePurpose> {
        FilePurpose::from_str(&self.purpose).ok()
    }

    /// Check if this file can be used for fine-tuning
    #[must_use]
    pub fn is_fine_tune_file(&self) -> bool {
        self.purpose == "fine-tune"
    }

    /// Check if this file can be used with Assistants
    #[must_use]
    pub fn is_assistants_file(&self) -> bool {
        self.purpose == "assistants"
    }

    /// Check if this file can be used for batch processing
    #[must_use]
    pub fn is_batch_file(&self) -> bool {
        self.purpose == "batch"
    }

    /// Get human-readable file size
    #[must_use]
    pub fn size_human_readable(&self) -> String {
        bytes_to_human_readable(self.bytes)
    }

    /// Get the creation date as a formatted string
    #[must_use]
    pub fn created_at_formatted(&self) -> String {
        use std::time::UNIX_EPOCH;
        let datetime = UNIX_EPOCH + std::time::Duration::from_secs(self.created_at);
        format!("{datetime:?}")
    }
}

/// Request structure for uploading a file
#[derive(Debug, Clone)]
pub struct FileUploadRequest {
    /// The file content as bytes
    pub file: Vec<u8>,
    /// The name of the file being uploaded
    pub filename: String,
    /// The intended purpose of the file
    pub purpose: FilePurpose,
}

impl FileUploadRequest {
    /// Create a new file upload request
    #[must_use]
    pub fn new(file: Vec<u8>, filename: String, purpose: FilePurpose) -> Self {
        Self {
            file,
            filename,
            purpose,
        }
    }

    /// Create a file upload request from a file path
    pub async fn from_file_path(
        file_path: &std::path::Path,
        purpose: FilePurpose,
    ) -> Result<Self, std::io::Error> {
        let file = tokio::fs::read(file_path).await?;
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Self::new(file, filename, purpose))
    }

    /// Validate the file upload request
    pub fn validate(&self) -> Result<(), String> {
        if self.file.is_empty() {
            return Err("File cannot be empty".to_string());
        }

        if self.filename.is_empty() {
            return Err("Filename cannot be empty".to_string());
        }

        // Check file size limits (200MB for most purposes)
        const MAX_FILE_SIZE: usize = 200 * 1024 * 1024; // 200MB
        if self.file.len() > MAX_FILE_SIZE {
            return Err(format!(
                "File size {} exceeds maximum limit of {} bytes",
                self.file.len(),
                MAX_FILE_SIZE
            ));
        }

        // Validate file extension for certain purposes
        match self.purpose {
            FilePurpose::FineTune => {
                if !self.filename.ends_with(".jsonl") {
                    return Err("Fine-tuning files must be in JSONL format".to_string());
                }
            }
            FilePurpose::Batch => {
                if !self.filename.ends_with(".jsonl") {
                    return Err("Batch files must be in JSONL format".to_string());
                }
            }
            FilePurpose::Vision => {
                let valid_extensions = [".png", ".jpg", ".jpeg", ".gif", ".webp"];
                if !valid_extensions
                    .iter()
                    .any(|ext| self.filename.to_lowercase().ends_with(ext))
                {
                    return Err(
                        "Vision files must be images (PNG, JPG, JPEG, GIF, WebP)".to_string()
                    );
                }
            }
            _ => {} // Other purposes allow various file types
        }

        Ok(())
    }

    /// Get the MIME type for the file based on its extension
    #[must_use]
    pub fn mime_type(&self) -> &'static str {
        let filename_lower = self.filename.to_lowercase();

        if filename_lower.ends_with(".jsonl") {
            "application/jsonl"
        } else if filename_lower.ends_with(".json") {
            "application/json"
        } else if filename_lower.ends_with(".txt") {
            "text/plain"
        } else if filename_lower.ends_with(".csv") {
            "text/csv"
        } else if filename_lower.ends_with(".png") {
            "image/png"
        } else if filename_lower.ends_with(".jpg") || filename_lower.ends_with(".jpeg") {
            "image/jpeg"
        } else if filename_lower.ends_with(".gif") {
            "image/gif"
        } else if filename_lower.ends_with(".webp") {
            "image/webp"
        } else if filename_lower.ends_with(".pdf") {
            "application/pdf"
        } else {
            "application/octet-stream"
        }
    }
}

/// Implementation of Validate trait for FileUploadRequest
impl Validate for FileUploadRequest {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

/// Response from the list files API endpoint
#[derive(Debug, Clone, Ser, De)]
pub struct ListFilesResponse {
    /// The object type, which is always "list"
    pub object: String,
    /// The list of files
    pub data: Vec<File>,
    /// Whether there are more results available
    #[serde(default)]
    pub has_more: bool,
    /// Cursor for the first item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Cursor for the last item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
}

impl ListFilesResponse {
    /// Create a new empty list response
    #[must_use]
    pub fn empty() -> Self {
        Self {
            object: "list".to_string(),
            data: Vec::new(),
            has_more: false,
            first_id: None,
            last_id: None,
        }
    }

    /// Get files by purpose
    #[must_use]
    pub fn files_by_purpose(&self, purpose: &FilePurpose) -> Vec<&File> {
        let purpose_str = purpose.to_string();
        self.data
            .iter()
            .filter(|file| file.purpose == purpose_str)
            .collect()
    }

    /// Get total size of all files in bytes
    #[must_use]
    pub fn total_size(&self) -> u64 {
        self.data.iter().map(|file| file.bytes).sum()
    }

    /// Get total size in human-readable format
    #[must_use]
    pub fn total_size_human_readable(&self) -> String {
        bytes_to_human_readable(self.total_size())
    }
}

/// Response from deleting a file
#[derive(Debug, Clone, Ser, De)]
pub struct FileDeleteResponse {
    /// Unique identifier for the file that was deleted
    pub id: String,
    /// The object type, which is always "file"
    pub object: String,
    /// Whether the file was successfully deleted
    pub deleted: bool,
}

crate::impl_response_constructors!(FileDeleteResponse, id, "file");

/// Parameters for listing files
#[derive(Debug, Clone, Default)]
pub struct ListFilesParams {
    /// Filter files by purpose
    pub purpose: Option<FilePurpose>,
    /// Maximum number of files to return (default 20, max 10,000)
    pub limit: Option<u32>,
    /// Pagination cursor - list files after this ID
    pub after: Option<String>,
    /// Pagination cursor - list files before this ID  
    pub before: Option<String>,
    /// Sort order for the results
    pub order: Option<SortOrder>,
}

/// Sort order for file listings
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Sort by creation date, ascending (oldest first)
    Asc,
    /// Sort by creation date, descending (newest first)
    Desc,
}

crate::impl_enum_display! {
    SortOrder {
        Asc => "asc",
        Desc => "desc",
    }
}

impl ListFilesParams {
    /// Create new parameters with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the purpose filter
    #[must_use]
    pub fn with_purpose(mut self, purpose: FilePurpose) -> Self {
        self.purpose = Some(purpose);
        self
    }

    /// Set the limit
    #[must_use]
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the after cursor for pagination
    #[must_use]
    pub fn with_after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    /// Set the before cursor for pagination
    #[must_use]
    pub fn with_before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }

    /// Set the sort order
    #[must_use]
    pub fn with_order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// Build query parameters for the API request
    #[must_use]
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(purpose) = &self.purpose {
            params.push(("purpose".to_string(), purpose.to_string()));
        }

        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }

        if let Some(after) = &self.after {
            params.push(("after".to_string(), after.clone()));
        }

        if let Some(before) = &self.before {
            params.push(("before".to_string(), before.clone()));
        }

        if let Some(order) = &self.order {
            params.push(("order".to_string(), order.to_string()));
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_purpose_display() {
        assert_eq!(FilePurpose::FineTune.to_string(), "fine-tune");
        assert_eq!(FilePurpose::Assistants.to_string(), "assistants");
        assert_eq!(FilePurpose::Batch.to_string(), "batch");
        assert_eq!(FilePurpose::UserData.to_string(), "user_data");
    }

    #[test]
    fn test_file_purpose_from_str() {
        assert_eq!(
            FilePurpose::from_str("fine-tune"),
            Ok(FilePurpose::FineTune)
        );
        assert_eq!(
            FilePurpose::from_str("assistants"),
            Ok(FilePurpose::Assistants)
        );
        assert!(FilePurpose::from_str("invalid").is_err());
    }

    #[test]
    fn test_file_purpose_supports() {
        assert!(FilePurpose::FineTune.supports_text());
        assert!(!FilePurpose::FineTune.supports_images());
        assert!(FilePurpose::Vision.supports_images());
        assert!(!FilePurpose::Vision.supports_text());
        assert!(FilePurpose::UserData.supports_text());
        assert!(FilePurpose::UserData.supports_images());
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

        let wrong_extension_request = FileUploadRequest::new(
            b"test content".to_vec(),
            "test.txt".to_string(),
            FilePurpose::FineTune,
        );
        assert!(wrong_extension_request.validate().is_err());
    }

    #[test]
    fn test_file_upload_request_mime_type() {
        let jsonl_request = FileUploadRequest::new(
            b"test".to_vec(),
            "test.jsonl".to_string(),
            FilePurpose::FineTune,
        );
        assert_eq!(jsonl_request.mime_type(), "application/jsonl");

        let image_request = FileUploadRequest::new(
            b"test".to_vec(),
            "test.png".to_string(),
            FilePurpose::Vision,
        );
        assert_eq!(image_request.mime_type(), "image/png");
    }

    #[test]
    fn test_file_size_human_readable() {
        let file = File {
            id: "file-123".to_string(),
            object: "file".to_string(),
            bytes: 1024,
            created_at: 1_640_995_200,
            filename: "test.txt".to_string(),
            purpose: "user_data".to_string(),
            status: "uploaded".to_string(),
            status_details: None,
        };
        assert_eq!(file.size_human_readable(), "1.0 KB");
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
    fn test_list_files_response_filters() {
        let files = vec![
            File {
                id: "file-1".to_string(),
                object: "file".to_string(),
                bytes: 100,
                created_at: 1_640_995_200,
                filename: "train.jsonl".to_string(),
                purpose: "fine-tune".to_string(),
                status: "uploaded".to_string(),
                status_details: None,
            },
            File {
                id: "file-2".to_string(),
                object: "file".to_string(),
                bytes: 200,
                created_at: 1_640_995_300,
                filename: "knowledge.pdf".to_string(),
                purpose: "assistants".to_string(),
                status: "uploaded".to_string(),
                status_details: None,
            },
        ];

        let response = ListFilesResponse {
            object: "list".to_string(),
            data: files,
            has_more: false,
            first_id: None,
            last_id: None,
        };

        let fine_tune_files = response.files_by_purpose(&FilePurpose::FineTune);
        assert_eq!(fine_tune_files.len(), 1);
        assert_eq!(fine_tune_files[0].id, "file-1");

        assert_eq!(response.total_size(), 300);
        assert_eq!(response.total_size_human_readable(), "300 B");
    }

    #[test]
    fn test_file_delete_response() {
        let success = FileDeleteResponse::success("file-123".to_string());
        assert!(success.deleted);
        assert_eq!(success.id, "file-123");

        let failure = FileDeleteResponse::failure("file-456".to_string());
        assert!(!failure.deleted);
        assert_eq!(failure.id, "file-456");
    }
}
