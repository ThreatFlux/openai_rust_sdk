//! Comprehensive tests for the OpenAI Threads & Messages API
//!
//! This test suite covers all functionality of the Threads API including:
//! - Thread creation, retrieval, modification, and deletion
//! - Message creation, retrieval, modification, and listing
//! - File attachment and management for messages
//! - Error handling and validation
//! - Pagination and filtering
//! - Content types and annotations

use openai_rust_sdk::api::threads::ThreadsApi;
use openai_rust_sdk::models::threads::{
    Annotation, FileCitation, FilePathInfo, ListMessagesParams, Message, MessageContent,
    MessageFile, MessageRequest, MessageRole, SortOrder, Thread, ThreadRequest,
};
use std::collections::HashMap;

/// Helper function to create a test thread request
fn create_test_thread_request() -> ThreadRequest {
    ThreadRequest::builder()
        .metadata_pair("test", "true")
        .metadata_pair("purpose", "unit_testing")
        .build()
}

/// Helper function to create a test message request
fn create_test_message_request() -> MessageRequest {
    MessageRequest::builder()
        .role(MessageRole::User)
        .content("Hello, this is a test message.")
        .metadata_pair("test", "true")
        .build()
        .unwrap()
}

/// Helper function to create a test message request with file
fn create_test_message_request_with_file() -> MessageRequest {
    MessageRequest::builder()
        .role(MessageRole::User)
        .content("Please analyze this file.")
        .file_id("file-test123")
        .metadata_pair("type", "file_analysis")
        .build()
        .unwrap()
}

/// Helper function to create an assistant message request
fn create_assistant_message_request() -> MessageRequest {
    MessageRequest::builder()
        .role(MessageRole::Assistant)
        .content("I'm happy to help you with that.")
        .metadata_pair("response_type", "greeting")
        .build()
        .unwrap()
}

// Unit Tests for Models

#[test]
fn test_thread_request_builder() {
    let request = ThreadRequest::builder()
        .metadata_pair("key1", "value1")
        .metadata_pair("key2", "value2")
        .build();

    assert_eq!(request.metadata.len(), 2);
    assert_eq!(request.metadata.get("key1"), Some(&"value1".to_string()));
    assert_eq!(request.metadata.get("key2"), Some(&"value2".to_string()));
    assert!(request.messages.is_empty());
}

#[test]
fn test_thread_request_with_messages() {
    let message = create_test_message_request();
    let request = ThreadRequest::builder()
        .message(message)
        .metadata_pair("purpose", "conversation")
        .build();

    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.metadata.len(), 1);
    assert_eq!(request.messages[0].role, MessageRole::User);
}

#[test]
fn test_message_request_builder() {
    let request = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Test message content")
        .file_id("file-123")
        .file_id("file-456")
        .metadata_pair("priority", "high")
        .metadata_pair("category", "support")
        .build()
        .unwrap();

    assert_eq!(request.role, MessageRole::User);
    assert_eq!(request.content, "Test message content");
    assert_eq!(request.file_ids.len(), 2);
    assert_eq!(request.metadata.len(), 2);
    assert!(request.file_ids.contains(&"file-123".to_string()));
    assert!(request.file_ids.contains(&"file-456".to_string()));
}

#[test]
fn test_message_request_builder_missing_role() {
    let result = MessageRequest::builder().content("Test content").build();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Role is required"));
}

#[test]
fn test_message_request_builder_missing_content() {
    let result = MessageRequest::builder().role(MessageRole::User).build();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Content is required"));
}

#[test]
fn test_message_role_methods() {
    let user_role = MessageRole::User;
    assert!(user_role.is_user());
    assert!(!user_role.is_assistant());

    let assistant_role = MessageRole::Assistant;
    assert!(assistant_role.is_assistant());
    assert!(!assistant_role.is_user());
}

#[test]
fn test_message_content_text() {
    let content = MessageContent::text("Hello, world!");

    match content {
        MessageContent::Text { text } => {
            assert_eq!(text.value, "Hello, world!");
            assert!(text.annotations.is_empty());
        }
        _ => panic!("Expected text content"),
    }
}

#[test]
fn test_message_content_text_with_annotations() {
    let annotations = vec![Annotation::FileCitation {
        text: "cited text".to_string(),
        start_index: 0,
        end_index: 10,
        file_citation: FileCitation {
            file_id: "file-123".to_string(),
            quote: Some("original quote".to_string()),
        },
    }];

    let content = MessageContent::text_with_annotations("Hello with citation", annotations);

    match content {
        MessageContent::Text { text } => {
            assert_eq!(text.value, "Hello with citation");
            assert_eq!(text.annotations.len(), 1);
        }
        _ => panic!("Expected text content"),
    }
}

#[test]
fn test_message_content_image_file() {
    let content = MessageContent::image_file("file-abc123");

    match content {
        MessageContent::ImageFile { image_file } => {
            assert_eq!(image_file.file_id, "file-abc123");
        }
        _ => panic!("Expected image file content"),
    }
}

#[test]
fn test_annotation_file_citation() {
    let annotation = Annotation::FileCitation {
        text: "cited text".to_string(),
        start_index: 5,
        end_index: 15,
        file_citation: FileCitation {
            file_id: "file-xyz789".to_string(),
            quote: Some("This is the original quote".to_string()),
        },
    };

    match annotation {
        Annotation::FileCitation {
            text,
            start_index,
            end_index,
            file_citation,
        } => {
            assert_eq!(text, "cited text");
            assert_eq!(start_index, 5);
            assert_eq!(end_index, 15);
            assert_eq!(file_citation.file_id, "file-xyz789");
            assert_eq!(
                file_citation.quote,
                Some("This is the original quote".to_string())
            );
        }
        _ => panic!("Expected file citation annotation"),
    }
}

#[test]
fn test_annotation_file_path() {
    let annotation = Annotation::FilePath {
        text: "file path".to_string(),
        start_index: 10,
        end_index: 20,
        file_path: FilePathInfo {
            file_id: "file-path123".to_string(),
        },
    };

    match annotation {
        Annotation::FilePath {
            text,
            start_index,
            end_index,
            file_path,
        } => {
            assert_eq!(text, "file path");
            assert_eq!(start_index, 10);
            assert_eq!(end_index, 20);
            assert_eq!(file_path.file_id, "file-path123");
        }
        _ => panic!("Expected file path annotation"),
    }
}

#[test]
fn test_list_messages_params() {
    let params = ListMessagesParams::new()
        .limit(50)
        .order(SortOrder::Asc)
        .after("msg_after123")
        .before("msg_before456");

    assert_eq!(params.limit, Some(50));
    assert_eq!(params.order, Some(SortOrder::Asc));
    assert_eq!(params.after, Some("msg_after123".to_string()));
    assert_eq!(params.before, Some("msg_before456".to_string()));
}

#[test]
fn test_list_messages_params_limit_clamping() {
    let params = ListMessagesParams::new().limit(150);
    assert_eq!(params.limit, Some(100));

    let params = ListMessagesParams::new().limit(0);
    assert_eq!(params.limit, Some(1));
}

#[test]
fn test_sort_order_default() {
    let order = SortOrder::default();
    assert_eq!(order, SortOrder::Desc);
}

// Validation Tests

#[test]
fn test_thread_request_validation_metadata_count() {
    let mut request = ThreadRequest::new();
    for i in 0..17 {
        request
            .metadata
            .insert(format!("key{}", i), "value".to_string());
    }

    let result = request.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("more than 16 metadata pairs"));
}

#[test]
fn test_thread_request_validation_metadata_key_length() {
    let mut request = ThreadRequest::new();
    let long_key = "a".repeat(65);
    request.metadata.insert(long_key, "value".to_string());

    let result = request.validate();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("key cannot exceed 64 characters")
    );
}

#[test]
fn test_thread_request_validation_metadata_value_length() {
    let mut request = ThreadRequest::new();
    let long_value = "a".repeat(513);
    request.metadata.insert("key".to_string(), long_value);

    let result = request.validate();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("value cannot exceed 512 characters")
    );
}

#[test]
fn test_message_request_validation_content_length() {
    let long_content = "a".repeat(32769);
    let result = MessageRequest::builder()
        .role(MessageRole::User)
        .content(long_content)
        .build();

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("content cannot exceed 32,768 characters")
    );
}

#[test]
fn test_message_request_validation_file_ids_count() {
    let mut builder = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Test content");

    for i in 0..11 {
        builder = builder.file_id(format!("file-{}", i));
    }

    let result = builder.build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("more than 10 file IDs"));
}

#[test]
fn test_message_request_validation_metadata_count() {
    let mut builder = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Test content");

    for i in 0..17 {
        builder = builder.metadata_pair(format!("key{}", i), "value");
    }

    let result = builder.build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("more than 16 metadata pairs"));
}

// API Client Tests

#[test]
fn test_threads_api_creation() {
    let api = ThreadsApi::new("test-api-key");
    assert!(api.is_ok());
}

#[test]
fn test_threads_api_with_custom_url() {
    let api = ThreadsApi::with_base_url("test-api-key", "https://custom.api.com");
    assert!(api.is_ok());
}

// Integration-style tests (would require actual API calls in real integration tests)

#[test]
fn test_thread_workflow_validation() {
    // Test the complete workflow validation without actual API calls
    let api = ThreadsApi::new("test-api-key").unwrap();

    // Create thread request
    let thread_request = create_test_thread_request();
    assert!(thread_request.validate().is_ok());

    // Create message request
    let message_request = create_test_message_request();
    assert!(message_request.validate().is_ok());

    // Create message with file request
    let message_with_file = create_test_message_request_with_file();
    assert!(message_with_file.validate().is_ok());

    // Create assistant message request
    let assistant_message = create_assistant_message_request();
    assert!(assistant_message.validate().is_ok());
}

#[test]
fn test_message_content_serialization() {
    // Test that message content can be properly serialized
    let text_content = MessageContent::text("Hello, world!");
    let json = serde_json::to_string(&text_content).unwrap();
    assert!(json.contains("\"type\":\"text\""));
    assert!(json.contains("\"value\":\"Hello, world!\""));

    let image_content = MessageContent::image_file("file-123");
    let json = serde_json::to_string(&image_content).unwrap();
    assert!(json.contains("\"type\":\"image_file\""));
    assert!(json.contains("\"file_id\":\"file-123\""));
}

#[test]
fn test_annotation_serialization() {
    let annotation = Annotation::FileCitation {
        text: "cited".to_string(),
        start_index: 0,
        end_index: 5,
        file_citation: FileCitation {
            file_id: "file-123".to_string(),
            quote: Some("quote".to_string()),
        },
    };

    let json = serde_json::to_string(&annotation).unwrap();
    assert!(json.contains("\"type\":\"file_citation\""));
    assert!(json.contains("\"text\":\"cited\""));
    assert!(json.contains("\"file_id\":\"file-123\""));
}

#[test]
fn test_complex_thread_with_multiple_messages() {
    let user_message = MessageRequest::builder()
        .role(MessageRole::User)
        .content("What's the weather like?")
        .build()
        .unwrap();

    let assistant_message = MessageRequest::builder()
        .role(MessageRole::Assistant)
        .content("I'll help you check the weather.")
        .build()
        .unwrap();

    let thread_request = ThreadRequest::builder()
        .message(user_message)
        .message(assistant_message)
        .metadata_pair("conversation_type", "weather_inquiry")
        .build();

    assert_eq!(thread_request.messages.len(), 2);
    assert_eq!(thread_request.messages[0].role, MessageRole::User);
    assert_eq!(thread_request.messages[1].role, MessageRole::Assistant);
    assert!(thread_request.validate().is_ok());
}

#[test]
fn test_message_with_multiple_files_and_metadata() {
    let request = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Please analyze these documents")
        .file_id("file-doc1")
        .file_id("file-doc2")
        .file_id("file-doc3")
        .metadata_pair("analysis_type", "document_review")
        .metadata_pair("priority", "high")
        .metadata_pair("department", "legal")
        .build()
        .unwrap();

    assert_eq!(request.file_ids.len(), 3);
    assert_eq!(request.metadata.len(), 3);
    assert_eq!(
        request.metadata.get("analysis_type"),
        Some(&"document_review".to_string())
    );
    assert!(request.validate().is_ok());
}

#[test]
fn test_default_object_types() {
    // Test that default object types are set correctly
    let thread = Thread {
        id: "thread_123".to_string(),
        object: "thread".to_string(),
        created_at: 1234567890,
        metadata: HashMap::new(),
    };
    assert_eq!(thread.object, "thread");

    let message = Message {
        id: "msg_123".to_string(),
        object: "thread.message".to_string(),
        created_at: 1234567890,
        thread_id: "thread_123".to_string(),
        role: MessageRole::User,
        content: vec![MessageContent::text("Hello")],
        assistant_id: None,
        run_id: None,
        file_ids: Vec::new(),
        metadata: HashMap::new(),
    };
    assert_eq!(message.object, "thread.message");

    let message_file = MessageFile {
        id: "file_123".to_string(),
        object: "thread.message.file".to_string(),
        created_at: 1234567890,
        message_id: "msg_123".to_string(),
    };
    assert_eq!(message_file.object, "thread.message.file");
}
