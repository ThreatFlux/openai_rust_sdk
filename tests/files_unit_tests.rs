//! Unit tests for the Files API models and validation
//!
//! This test suite focuses on testing the Files API models, validation,
//! and helper functions without requiring network mocking.

use openai_rust_sdk::models::files::{
    File, FileDeleteResponse, FilePurpose, FileStatus, FileUploadRequest, ListFilesParams,
    ListFilesResponse, SortOrder,
};

#[test]
fn test_file_purpose_display() {
    assert_eq!(FilePurpose::FineTune.to_string(), "fine-tune");
    assert_eq!(FilePurpose::Assistants.to_string(), "assistants");
    assert_eq!(FilePurpose::Batch.to_string(), "batch");
    assert_eq!(FilePurpose::UserData.to_string(), "user_data");
    assert_eq!(FilePurpose::Responses.to_string(), "responses");
    assert_eq!(FilePurpose::Vision.to_string(), "vision");
    assert_eq!(
        FilePurpose::FineTuneResults.to_string(),
        "fine-tune-results"
    );
    assert_eq!(
        FilePurpose::AssistantsOutput.to_string(),
        "assistants_output"
    );
}

#[test]
fn test_file_purpose_from_str() {
    assert_eq!(
        FilePurpose::from_str("fine-tune"),
        Some(FilePurpose::FineTune)
    );
    assert_eq!(
        FilePurpose::from_str("assistants"),
        Some(FilePurpose::Assistants)
    );
    assert_eq!(FilePurpose::from_str("batch"), Some(FilePurpose::Batch));
    assert_eq!(
        FilePurpose::from_str("user_data"),
        Some(FilePurpose::UserData)
    );
    assert_eq!(
        FilePurpose::from_str("responses"),
        Some(FilePurpose::Responses)
    );
    assert_eq!(FilePurpose::from_str("vision"), Some(FilePurpose::Vision));
    assert_eq!(
        FilePurpose::from_str("fine-tune-results"),
        Some(FilePurpose::FineTuneResults)
    );
    assert_eq!(
        FilePurpose::from_str("assistants_output"),
        Some(FilePurpose::AssistantsOutput)
    );
    assert_eq!(FilePurpose::from_str("invalid"), None);
}

#[test]
fn test_file_purpose_all() {
    let all_purposes = FilePurpose::all();
    assert_eq!(all_purposes.len(), 8);
    assert!(all_purposes.contains(&FilePurpose::FineTune));
    assert!(all_purposes.contains(&FilePurpose::Assistants));
    assert!(all_purposes.contains(&FilePurpose::Batch));
    assert!(all_purposes.contains(&FilePurpose::UserData));
    assert!(all_purposes.contains(&FilePurpose::Responses));
    assert!(all_purposes.contains(&FilePurpose::Vision));
    assert!(all_purposes.contains(&FilePurpose::FineTuneResults));
    assert!(all_purposes.contains(&FilePurpose::AssistantsOutput));
}

#[test]
fn test_file_purpose_supports_text() {
    assert!(FilePurpose::FineTune.supports_text());
    assert!(FilePurpose::Assistants.supports_text());
    assert!(FilePurpose::Batch.supports_text());
    assert!(FilePurpose::UserData.supports_text());
    assert!(FilePurpose::Responses.supports_text());
    assert!(!FilePurpose::Vision.supports_text());
    assert!(!FilePurpose::FineTuneResults.supports_text());
    assert!(!FilePurpose::AssistantsOutput.supports_text());
}

#[test]
fn test_file_purpose_supports_images() {
    assert!(FilePurpose::Vision.supports_images());
    assert!(FilePurpose::UserData.supports_images());
    assert!(!FilePurpose::FineTune.supports_images());
    assert!(!FilePurpose::Assistants.supports_images());
    assert!(!FilePurpose::Batch.supports_images());
    assert!(!FilePurpose::Responses.supports_images());
    assert!(!FilePurpose::FineTuneResults.supports_images());
    assert!(!FilePurpose::AssistantsOutput.supports_images());
}

#[test]
fn test_file_status_display() {
    assert_eq!(FileStatus::Uploaded.to_string(), "uploaded");
    assert_eq!(FileStatus::Processed.to_string(), "processed");
    assert_eq!(FileStatus::Error.to_string(), "error");
    assert_eq!(FileStatus::Deleted.to_string(), "deleted");
}

#[test]
fn test_file_upload_request_validation_valid() {
    let valid_requests = vec![
        FileUploadRequest::new(
            b"test content".to_vec(),
            "test.jsonl".to_string(),
            FilePurpose::FineTune,
        ),
        FileUploadRequest::new(
            b"test content".to_vec(),
            "batch.jsonl".to_string(),
            FilePurpose::Batch,
        ),
        FileUploadRequest::new(
            b"test content".to_vec(),
            "doc.txt".to_string(),
            FilePurpose::Assistants,
        ),
        FileUploadRequest::new(
            b"test content".to_vec(),
            "data.txt".to_string(),
            FilePurpose::UserData,
        ),
        FileUploadRequest::new(
            b"image data".to_vec(),
            "image.png".to_string(),
            FilePurpose::Vision,
        ),
        FileUploadRequest::new(
            b"image data".to_vec(),
            "image.jpg".to_string(),
            FilePurpose::Vision,
        ),
        FileUploadRequest::new(
            b"image data".to_vec(),
            "image.jpeg".to_string(),
            FilePurpose::Vision,
        ),
        FileUploadRequest::new(
            b"image data".to_vec(),
            "image.gif".to_string(),
            FilePurpose::Vision,
        ),
        FileUploadRequest::new(
            b"image data".to_vec(),
            "image.webp".to_string(),
            FilePurpose::Vision,
        ),
    ];

    for request in valid_requests {
        assert!(
            request.validate().is_ok(),
            "Failed validation for: {} with purpose {}",
            request.filename,
            request.purpose
        );
    }
}

#[test]
fn test_file_upload_request_validation_invalid() {
    // Empty file
    let empty = FileUploadRequest::new(Vec::new(), "test.jsonl".to_string(), FilePurpose::FineTune);
    assert!(empty.validate().is_err());
    assert_eq!(empty.validate().unwrap_err(), "File cannot be empty");

    // Empty filename
    let no_name =
        FileUploadRequest::new(b"content".to_vec(), "".to_string(), FilePurpose::FineTune);
    assert!(no_name.validate().is_err());
    assert_eq!(no_name.validate().unwrap_err(), "Filename cannot be empty");

    // Wrong extension for fine-tune
    let wrong_ft = FileUploadRequest::new(
        b"content".to_vec(),
        "test.txt".to_string(),
        FilePurpose::FineTune,
    );
    assert!(wrong_ft.validate().is_err());
    assert_eq!(
        wrong_ft.validate().unwrap_err(),
        "Fine-tuning files must be in JSONL format"
    );

    // Wrong extension for batch
    let wrong_batch = FileUploadRequest::new(
        b"content".to_vec(),
        "test.txt".to_string(),
        FilePurpose::Batch,
    );
    assert!(wrong_batch.validate().is_err());
    assert_eq!(
        wrong_batch.validate().unwrap_err(),
        "Batch files must be in JSONL format"
    );

    // Wrong extension for vision
    let wrong_vision = FileUploadRequest::new(
        b"content".to_vec(),
        "test.txt".to_string(),
        FilePurpose::Vision,
    );
    assert!(wrong_vision.validate().is_err());
    assert_eq!(
        wrong_vision.validate().unwrap_err(),
        "Vision files must be images (PNG, JPG, JPEG, GIF, WebP)"
    );

    // File too large (>200MB)
    let large = FileUploadRequest::new(
        vec![0; 201 * 1024 * 1024],
        "large.jsonl".to_string(),
        FilePurpose::FineTune,
    );
    assert!(large.validate().is_err());
    assert!(large
        .validate()
        .unwrap_err()
        .contains("exceeds maximum limit"));
}

#[test]
fn test_file_upload_request_mime_types() {
    let test_cases = vec![
        ("test.jsonl", "application/jsonl"),
        ("test.json", "application/json"),
        ("test.txt", "text/plain"),
        ("test.csv", "text/csv"),
        ("test.png", "image/png"),
        ("test.jpg", "image/jpeg"),
        ("test.jpeg", "image/jpeg"),
        ("test.JPG", "image/jpeg"),
        ("test.JPEG", "image/jpeg"),
        ("test.gif", "image/gif"),
        ("test.webp", "image/webp"),
        ("test.pdf", "application/pdf"),
        ("test.unknown", "application/octet-stream"),
        ("test", "application/octet-stream"),
    ];

    for (filename, expected_mime) in test_cases {
        let request = FileUploadRequest::new(
            b"test".to_vec(),
            filename.to_string(),
            FilePurpose::UserData,
        );
        assert_eq!(
            request.mime_type(),
            expected_mime,
            "Wrong MIME type for {}",
            filename
        );
    }
}

#[test]
fn test_file_object_methods() {
    let file = File {
        id: "file-123".to_string(),
        object: "file".to_string(),
        bytes: 2048,
        created_at: 1640995200,
        filename: "train.jsonl".to_string(),
        purpose: "fine-tune".to_string(),
        status: "uploaded".to_string(),
        status_details: None,
    };

    // Test purpose enum conversion
    assert_eq!(file.purpose_enum(), Some(FilePurpose::FineTune));

    // Test purpose checking methods
    assert!(file.is_fine_tune_file());
    assert!(!file.is_assistants_file());
    assert!(!file.is_batch_file());

    // Test human-readable size
    assert_eq!(file.size_human_readable(), "2.0 KB");

    // Test with assistants file
    let assistants_file = File {
        id: "file-456".to_string(),
        object: "file".to_string(),
        bytes: 1024 * 1024 * 5,
        created_at: 1640995200,
        filename: "knowledge.txt".to_string(),
        purpose: "assistants".to_string(),
        status: "uploaded".to_string(),
        status_details: Some("Processing complete".to_string()),
    };

    assert!(assistants_file.is_assistants_file());
    assert!(!assistants_file.is_fine_tune_file());
    assert_eq!(assistants_file.size_human_readable(), "5.0 MB");

    // Test with batch file
    let batch_file = File {
        id: "file-789".to_string(),
        object: "file".to_string(),
        bytes: 1024 * 1024 * 1024 * 2,
        created_at: 1640995200,
        filename: "batch.jsonl".to_string(),
        purpose: "batch".to_string(),
        status: "processed".to_string(),
        status_details: None,
    };

    assert!(batch_file.is_batch_file());
    assert_eq!(batch_file.size_human_readable(), "2.0 GB");
}

#[test]
fn test_file_size_human_readable() {
    let test_cases = vec![
        (0, "0 B"),
        (512, "512 B"),
        (1023, "1023 B"),
        (1024, "1.0 KB"),
        (1536, "1.5 KB"),
        (1024 * 10, "10.0 KB"),
        (1024 * 1024, "1.0 MB"),
        (1024 * 1024 * 5, "5.0 MB"),
        (1024 * 1024 * 512, "512.0 MB"),
        (1024 * 1024 * 1024, "1.0 GB"),
        (1024 * 1024 * 1024 * 10, "10.0 GB"),
    ];

    for (bytes, expected) in test_cases {
        let file = File {
            id: "test".to_string(),
            object: "file".to_string(),
            bytes,
            created_at: 0,
            filename: "test.txt".to_string(),
            purpose: "user_data".to_string(),
            status: "uploaded".to_string(),
            status_details: None,
        };
        assert_eq!(
            file.size_human_readable(),
            expected,
            "Wrong formatting for {} bytes",
            bytes
        );
    }
}

#[test]
fn test_list_files_response_methods() {
    let files = vec![
        File {
            id: "file-1".to_string(),
            object: "file".to_string(),
            bytes: 1024,
            created_at: 1640995200,
            filename: "train1.jsonl".to_string(),
            purpose: "fine-tune".to_string(),
            status: "uploaded".to_string(),
            status_details: None,
        },
        File {
            id: "file-2".to_string(),
            object: "file".to_string(),
            bytes: 2048,
            created_at: 1640995300,
            filename: "train2.jsonl".to_string(),
            purpose: "fine-tune".to_string(),
            status: "uploaded".to_string(),
            status_details: None,
        },
        File {
            id: "file-3".to_string(),
            object: "file".to_string(),
            bytes: 4096,
            created_at: 1640995400,
            filename: "knowledge.txt".to_string(),
            purpose: "assistants".to_string(),
            status: "uploaded".to_string(),
            status_details: None,
        },
        File {
            id: "file-4".to_string(),
            object: "file".to_string(),
            bytes: 8192,
            created_at: 1640995500,
            filename: "batch.jsonl".to_string(),
            purpose: "batch".to_string(),
            status: "processed".to_string(),
            status_details: None,
        },
    ];

    let response = ListFilesResponse {
        object: "list".to_string(),
        data: files,
        has_more: false,
        first_id: Some("file-1".to_string()),
        last_id: Some("file-4".to_string()),
    };

    // Test filtering by purpose
    let fine_tune_files = response.files_by_purpose(&FilePurpose::FineTune);
    assert_eq!(fine_tune_files.len(), 2);
    assert_eq!(fine_tune_files[0].id, "file-1");
    assert_eq!(fine_tune_files[1].id, "file-2");

    let assistants_files = response.files_by_purpose(&FilePurpose::Assistants);
    assert_eq!(assistants_files.len(), 1);
    assert_eq!(assistants_files[0].id, "file-3");

    let batch_files = response.files_by_purpose(&FilePurpose::Batch);
    assert_eq!(batch_files.len(), 1);
    assert_eq!(batch_files[0].id, "file-4");

    let vision_files = response.files_by_purpose(&FilePurpose::Vision);
    assert_eq!(vision_files.len(), 0);

    // Test total size calculation
    assert_eq!(response.total_size(), 1024 + 2048 + 4096 + 8192);
    assert_eq!(response.total_size(), 15360);
    assert_eq!(response.total_size_human_readable(), "15.0 KB");

    // Test empty response
    let empty_response = ListFilesResponse::empty();
    assert_eq!(empty_response.data.len(), 0);
    assert!(!empty_response.has_more);
    assert_eq!(empty_response.total_size(), 0);
    assert_eq!(empty_response.total_size_human_readable(), "0 B");
}

#[test]
fn test_file_delete_response() {
    let success = FileDeleteResponse::success("file-123".to_string());
    assert_eq!(success.id, "file-123");
    assert_eq!(success.object, "file");
    assert!(success.deleted);

    let failure = FileDeleteResponse::failure("file-456".to_string());
    assert_eq!(failure.id, "file-456");
    assert_eq!(failure.object, "file");
    assert!(!failure.deleted);
}

#[test]
fn test_list_files_params() {
    let params = ListFilesParams::new();
    assert!(params.purpose.is_none());
    assert!(params.limit.is_none());
    assert!(params.after.is_none());
    assert!(params.before.is_none());
    assert!(params.order.is_none());

    let params = ListFilesParams::new()
        .with_purpose(FilePurpose::FineTune)
        .with_limit(20)
        .with_after("file-123".to_string())
        .with_before("file-456".to_string())
        .with_order(SortOrder::Desc);

    assert_eq!(params.purpose, Some(FilePurpose::FineTune));
    assert_eq!(params.limit, Some(20));
    assert_eq!(params.after, Some("file-123".to_string()));
    assert_eq!(params.before, Some("file-456".to_string()));
    assert_eq!(params.order, Some(SortOrder::Desc));

    let query_params = params.to_query_params();
    assert_eq!(query_params.len(), 5);
    assert!(query_params.contains(&("purpose".to_string(), "fine-tune".to_string())));
    assert!(query_params.contains(&("limit".to_string(), "20".to_string())));
    assert!(query_params.contains(&("after".to_string(), "file-123".to_string())));
    assert!(query_params.contains(&("before".to_string(), "file-456".to_string())));
    assert!(query_params.contains(&("order".to_string(), "desc".to_string())));
}

#[test]
fn test_sort_order_display() {
    assert_eq!(SortOrder::Asc.to_string(), "asc");
    assert_eq!(SortOrder::Desc.to_string(), "desc");
}

#[test]
fn test_file_upload_request_from_bytes() {
    let content = b"test content";
    let request = FileUploadRequest::new(
        content.to_vec(),
        "test.jsonl".to_string(),
        FilePurpose::FineTune,
    );

    assert_eq!(request.file, content.to_vec());
    assert_eq!(request.filename, "test.jsonl");
    assert_eq!(request.purpose, FilePurpose::FineTune);
}

#[tokio::test]
async fn test_file_upload_request_from_file_path() {
    use std::path::Path;
    use tokio::fs;

    // Create a temporary file
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.jsonl");
    let content = b"test content for file upload";
    fs::write(&file_path, content).await.unwrap();

    // Create request from file path
    let request = FileUploadRequest::from_file_path(&file_path, FilePurpose::FineTune)
        .await
        .unwrap();

    assert_eq!(request.file, content.to_vec());
    assert_eq!(request.filename, "test.jsonl");
    assert_eq!(request.purpose, FilePurpose::FineTune);

    // Test with non-existent file
    let non_existent = Path::new("/non/existent/file.jsonl");
    let result = FileUploadRequest::from_file_path(non_existent, FilePurpose::FineTune).await;
    assert!(result.is_err());
}

#[test]
fn test_all_file_purposes_covered() {
    // Ensure all purposes have display strings
    for purpose in FilePurpose::all() {
        let display = purpose.to_string();
        assert!(!display.is_empty());

        // Ensure round-trip conversion works
        let parsed = FilePurpose::from_str(&display);
        assert_eq!(parsed, Some(purpose.clone()));
    }
}

#[test]
fn test_file_created_at_formatted() {
    let file = File {
        id: "file-123".to_string(),
        object: "file".to_string(),
        bytes: 1024,
        created_at: 1640995200, // 2022-01-01 00:00:00 UTC
        filename: "test.txt".to_string(),
        purpose: "user_data".to_string(),
        status: "uploaded".to_string(),
        status_details: None,
    };

    let formatted = file.created_at_formatted();
    assert!(formatted.contains("SystemTime"));
    assert!(formatted.contains("1640995200"));
}
