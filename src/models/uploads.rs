//! # Uploads API Models
//!
//! Data structures for the OpenAI Uploads API, which allows uploading large files
//! in multiple parts (multipart uploads).
//!
//! ## Overview
//!
//! The Uploads API enables uploading files larger than the standard file upload limit
//! by splitting them into parts. The workflow is:
//!
//! 1. **Create** an upload with the target filename, purpose, size, and MIME type
//! 2. **Add parts** by uploading file chunks sequentially
//! 3. **Complete** the upload by providing the ordered part IDs (and optional MD5 checksum)
//! 4. Optionally **cancel** an in-progress upload
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::uploads::CreateUploadRequest;
//!
//! let request = CreateUploadRequest::new(
//!     "training_data.jsonl",
//!     "fine-tune",
//!     64 * 1024 * 1024, // 64 MB
//!     "application/jsonl",
//! );
//! ```

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

use super::files::File;

/// Status of an upload
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum UploadStatus {
    /// Upload has been created and is awaiting parts
    Pending,
    /// All parts have been uploaded and the upload is complete
    Completed,
    /// The upload was cancelled before completion
    Cancelled,
}

crate::impl_enum_display! {
    UploadStatus {
        Pending => "pending",
        Completed => "completed",
        Cancelled => "cancelled",
    }
}

/// Represents an upload object in OpenAI's system
#[derive(Debug, Clone, Ser, De)]
pub struct Upload {
    /// Unique identifier for the upload
    pub id: String,

    /// The Unix timestamp (in seconds) for when the upload was created
    pub created_at: u64,

    /// The name of the file to be uploaded
    pub filename: String,

    /// The intended number of bytes to be uploaded
    pub bytes: u64,

    /// The intended purpose of the file (e.g., "fine-tune", "assistants")
    pub purpose: String,

    /// The current status of the upload
    pub status: UploadStatus,

    /// The Unix timestamp (in seconds) for when the upload expires
    pub expires_at: u64,

    /// The object type, which is always "upload"
    #[serde(default = "default_upload_object")]
    pub object: String,

    /// The `File` object created after the upload is completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
}

crate::impl_default_object_type!(default_upload_object, "upload");

impl Upload {
    /// Check if the upload is still pending
    #[must_use]
    pub fn is_pending(&self) -> bool {
        self.status == UploadStatus::Pending
    }

    /// Check if the upload has been completed
    #[must_use]
    pub fn is_completed(&self) -> bool {
        self.status == UploadStatus::Completed
    }

    /// Check if the upload was cancelled
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.status == UploadStatus::Cancelled
    }
}

/// Represents a part of an upload
#[derive(Debug, Clone, Ser, De)]
pub struct UploadPart {
    /// Unique identifier for the upload part
    pub id: String,

    /// The Unix timestamp (in seconds) for when the part was created
    pub created_at: u64,

    /// The ID of the upload this part belongs to
    pub upload_id: String,

    /// The object type, which is always "upload.part"
    #[serde(default = "default_upload_part_object")]
    pub object: String,
}

crate::impl_default_object_type!(default_upload_part_object, "upload.part");

/// Request to create a new multipart upload
#[derive(Debug, Clone, Ser, De)]
pub struct CreateUploadRequest {
    /// The name of the file to upload
    pub filename: String,

    /// The intended purpose of the uploaded file (e.g., "fine-tune", "assistants")
    pub purpose: String,

    /// The number of bytes in the file you are uploading
    pub bytes: u64,

    /// The MIME type of the file (e.g., "application/jsonl", "text/plain")
    pub mime_type: String,
}

impl CreateUploadRequest {
    /// Create a new upload request
    pub fn new(
        filename: impl Into<String>,
        purpose: impl Into<String>,
        bytes: u64,
        mime_type: impl Into<String>,
    ) -> Self {
        Self {
            filename: filename.into(),
            purpose: purpose.into(),
            bytes,
            mime_type: mime_type.into(),
        }
    }

    /// Set the filename
    #[must_use]
    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = filename.into();
        self
    }

    /// Set the purpose
    #[must_use]
    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = purpose.into();
        self
    }

    /// Set the byte count
    #[must_use]
    pub fn with_bytes(mut self, bytes: u64) -> Self {
        self.bytes = bytes;
        self
    }

    /// Set the MIME type
    #[must_use]
    pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = mime_type.into();
        self
    }
}

/// Request to complete a multipart upload
#[derive(Debug, Clone, Ser, De)]
pub struct CompleteUploadRequest {
    /// The ordered list of part IDs that make up the upload
    pub part_ids: Vec<String>,

    /// Optional MD5 checksum for the entire file to verify integrity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
}

impl CompleteUploadRequest {
    /// Create a new complete upload request with the given part IDs
    pub fn new(part_ids: Vec<String>) -> Self {
        Self {
            part_ids,
            md5: None,
        }
    }

    /// Set the MD5 checksum for integrity verification
    #[must_use]
    pub fn with_md5(mut self, md5: impl Into<String>) -> Self {
        self.md5 = Some(md5.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_status_display() {
        assert_eq!(UploadStatus::Pending.to_string(), "pending");
        assert_eq!(UploadStatus::Completed.to_string(), "completed");
        assert_eq!(UploadStatus::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_create_upload_request() {
        let req = CreateUploadRequest::new(
            "training_data.jsonl",
            "fine-tune",
            64 * 1024 * 1024,
            "application/jsonl",
        );
        assert_eq!(req.filename, "training_data.jsonl");
        assert_eq!(req.purpose, "fine-tune");
        assert_eq!(req.bytes, 64 * 1024 * 1024);
        assert_eq!(req.mime_type, "application/jsonl");
    }

    #[test]
    fn test_create_upload_request_builder() {
        let req = CreateUploadRequest::new("file.txt", "assistants", 1024, "text/plain")
            .with_filename("renamed.txt")
            .with_bytes(2048);
        assert_eq!(req.filename, "renamed.txt");
        assert_eq!(req.bytes, 2048);
    }

    #[test]
    fn test_complete_upload_request() {
        let req = CompleteUploadRequest::new(vec![
            "part-1".to_string(),
            "part-2".to_string(),
            "part-3".to_string(),
        ]);
        assert_eq!(req.part_ids.len(), 3);
        assert!(req.md5.is_none());
    }

    #[test]
    fn test_complete_upload_request_with_md5() {
        let req = CompleteUploadRequest::new(vec!["part-1".to_string()])
            .with_md5("d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(
            req.md5,
            Some("d41d8cd98f00b204e9800998ecf8427e".to_string())
        );
    }

    #[test]
    fn test_upload_status_checks() {
        let upload = Upload {
            id: "upload-123".to_string(),
            created_at: 1_700_000_000,
            filename: "data.jsonl".to_string(),
            bytes: 1024,
            purpose: "fine-tune".to_string(),
            status: UploadStatus::Pending,
            expires_at: 1_700_003_600,
            object: "upload".to_string(),
            file: None,
        };
        assert!(upload.is_pending());
        assert!(!upload.is_completed());
        assert!(!upload.is_cancelled());
    }

    #[test]
    fn test_upload_serialization() {
        let req = CreateUploadRequest::new("test.jsonl", "fine-tune", 512, "application/jsonl");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"filename\":\"test.jsonl\""));
        assert!(json.contains("\"purpose\":\"fine-tune\""));
        assert!(json.contains("\"bytes\":512"));
        assert!(json.contains("\"mime_type\":\"application/jsonl\""));
    }

    #[test]
    fn test_upload_deserialization() {
        let json = r#"{
            "id": "upload-abc123",
            "created_at": 1700000000,
            "filename": "data.jsonl",
            "bytes": 2048,
            "purpose": "fine-tune",
            "status": "completed",
            "expires_at": 1700003600,
            "object": "upload",
            "file": {
                "id": "file-xyz789",
                "object": "file",
                "bytes": 2048,
                "created_at": 1700000000,
                "filename": "data.jsonl",
                "purpose": "fine-tune",
                "status": "uploaded"
            }
        }"#;
        let upload: Upload = serde_json::from_str(json).unwrap();
        assert_eq!(upload.id, "upload-abc123");
        assert!(upload.is_completed());
        assert!(upload.file.is_some());
        assert_eq!(upload.file.unwrap().id, "file-xyz789");
    }

    #[test]
    fn test_upload_part_deserialization() {
        let json = r#"{
            "id": "part-def456",
            "created_at": 1700000100,
            "upload_id": "upload-abc123",
            "object": "upload.part"
        }"#;
        let part: UploadPart = serde_json::from_str(json).unwrap();
        assert_eq!(part.id, "part-def456");
        assert_eq!(part.upload_id, "upload-abc123");
        assert_eq!(part.object, "upload.part");
    }
}
