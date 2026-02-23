//! # Uploads API
//!
//! Client for the OpenAI Uploads API, which allows uploading large files
//! in multiple parts (multipart uploads).

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use crate::models::uploads::{CompleteUploadRequest, CreateUploadRequest, Upload, UploadPart};
use reqwest::multipart;

/// Uploads API client for managing multipart file uploads
pub struct UploadsApi {
    /// Shared HTTP client for making requests
    client: HttpClient,
}

impl ApiClientConstructors for UploadsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self {
            client: http_client,
        }
    }
}

impl UploadsApi {
    /// Create a new multipart upload
    pub async fn create_upload(&self, request: &CreateUploadRequest) -> Result<Upload> {
        self.client.post("/v1/uploads", request).await
    }

    /// Add a part to an in-progress upload
    ///
    /// The data is sent as multipart form data.
    pub async fn add_upload_part(
        &self,
        upload_id: impl AsRef<str>,
        data: Vec<u8>,
    ) -> Result<UploadPart> {
        let path = format!("/v1/uploads/{}/parts", upload_id.as_ref());
        let part = multipart::Part::bytes(data).file_name("part");
        let form = multipart::Form::new().part("data", part);
        self.client.post_multipart(&path, form).await
    }

    /// Complete an upload by providing the ordered list of part IDs
    pub async fn complete_upload(
        &self,
        upload_id: impl AsRef<str>,
        request: &CompleteUploadRequest,
    ) -> Result<Upload> {
        let path = format!("/v1/uploads/{}/complete", upload_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Cancel an in-progress upload
    pub async fn cancel_upload(&self, upload_id: impl AsRef<str>) -> Result<Upload> {
        let path = format!("/v1/uploads/{}/cancel", upload_id.as_ref());
        self.client.post(&path, &()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uploads_api_creation() {
        let api = UploadsApi::new("test-key").unwrap();
        assert_eq!(api.client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_uploads_api_creation_with_base_url() {
        let api = UploadsApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_uploads_api_empty_key_fails() {
        let result = UploadsApi::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_upload_request_serialization() {
        let req = CreateUploadRequest::new("data.jsonl", "fine-tune", 1024, "application/jsonl");
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["filename"], "data.jsonl");
        assert_eq!(json["purpose"], "fine-tune");
        assert_eq!(json["bytes"], 1024);
        assert_eq!(json["mime_type"], "application/jsonl");
    }

    #[test]
    fn test_complete_upload_request_serialization() {
        let req = CompleteUploadRequest::new(vec!["part-1".to_string(), "part-2".to_string()])
            .with_md5("abc123");
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["part_ids"], serde_json::json!(["part-1", "part-2"]));
        assert_eq!(json["md5"], "abc123");
    }

    #[test]
    fn test_complete_upload_request_without_md5() {
        let req = CompleteUploadRequest::new(vec!["part-1".to_string()]);
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("md5").is_none());
    }

    #[test]
    fn test_upload_deserialization() {
        let json = r#"{
            "id": "upload-abc",
            "created_at": 1700000000,
            "filename": "data.jsonl",
            "bytes": 2048,
            "purpose": "fine-tune",
            "status": "pending",
            "expires_at": 1700003600,
            "object": "upload"
        }"#;
        let upload: Upload = serde_json::from_str(json).unwrap();
        assert_eq!(upload.id, "upload-abc");
        assert!(upload.is_pending());
        assert!(!upload.is_completed());
        assert!(!upload.is_cancelled());
        assert!(upload.file.is_none());
    }

    #[test]
    fn test_upload_part_deserialization() {
        let json = r#"{
            "id": "part-abc",
            "created_at": 1700000000,
            "upload_id": "upload-abc",
            "object": "upload.part"
        }"#;
        let part: UploadPart = serde_json::from_str(json).unwrap();
        assert_eq!(part.id, "part-abc");
        assert_eq!(part.upload_id, "upload-abc");
    }
}
