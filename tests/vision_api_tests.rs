#![allow(clippy::pedantic, clippy::nursery)]
//! Tests for OpenAI Vision API functionality

use openai_rust_sdk::{
    ImageDetail, ImageFormat, ImageUtils, Message, MessageContent, MessageContentInput,
    MessageRole, OpenAIClient, ResponseRequest,
};

#[tokio::test]
async fn test_message_creation() {
    // Test basic text message creation
    let text_msg = Message::user("Hello world");
    assert_eq!(text_msg.role, MessageRole::User);
    assert_eq!(text_msg.text_content(), "Hello world");
    assert!(!text_msg.has_images());
    assert_eq!(text_msg.image_urls().len(), 0);

    // Test image message creation
    let image_msg = Message::user_with_image("Analyze this", "https://example.com/image.jpg");
    assert_eq!(image_msg.role, MessageRole::User);
    assert_eq!(image_msg.text_content(), "Analyze this");
    assert!(image_msg.has_images());
    assert_eq!(
        image_msg.image_urls(),
        vec!["https://example.com/image.jpg"]
    );
}

#[tokio::test]
async fn test_multi_image_message() {
    let urls = vec![
        "https://example.com/image1.jpg".to_string(),
        "https://example.com/image2.png".to_string(),
    ];

    let msg = Message::user_with_images("Compare these", urls.clone());

    assert_eq!(msg.text_content(), "Compare these");
    assert!(msg.has_images());
    assert_eq!(msg.image_urls().len(), 2);
    assert_eq!(
        msg.image_urls(),
        vec![
            "https://example.com/image1.jpg",
            "https://example.com/image2.png"
        ]
    );
}

#[tokio::test]
async fn test_message_content_creation() {
    // Test text content
    let text_content = MessageContent::text("Hello");
    match text_content {
        MessageContent::Text { text } => assert_eq!(text, "Hello"),
        _ => panic!("Expected text content"),
    }

    // Test image content
    let image_content = MessageContent::image_url("https://example.com/test.jpg");
    match image_content {
        MessageContent::Image { image_url } => {
            assert_eq!(image_url.url, "https://example.com/test.jpg");
            assert_eq!(image_url.detail, None);
        }
        _ => panic!("Expected image content"),
    }

    // Test image content with detail
    let detailed_image =
        MessageContent::image_url_with_detail("https://example.com/test.jpg", ImageDetail::High);
    match detailed_image {
        MessageContent::Image { image_url } => {
            assert_eq!(image_url.url, "https://example.com/test.jpg");
            assert_eq!(image_url.detail, Some(ImageDetail::High));
        }
        _ => panic!("Expected image content"),
    }
}

#[tokio::test]
async fn test_image_format_detection() {
    // Test URL format detection
    assert_eq!(ImageFormat::from_url("test.jpg"), Some(ImageFormat::Jpeg));
    assert_eq!(ImageFormat::from_url("test.jpeg"), Some(ImageFormat::Jpeg));
    assert_eq!(ImageFormat::from_url("test.png"), Some(ImageFormat::Png));
    assert_eq!(ImageFormat::from_url("test.gif"), Some(ImageFormat::Gif));
    assert_eq!(ImageFormat::from_url("test.webp"), Some(ImageFormat::Webp));
    assert_eq!(ImageFormat::from_url("test.bmp"), None);

    // Test case insensitive
    assert_eq!(ImageFormat::from_url("TEST.JPG"), Some(ImageFormat::Jpeg));
    assert_eq!(ImageFormat::from_url("Test.PNG"), Some(ImageFormat::Png));

    // Test data URL format detection
    assert_eq!(
        ImageFormat::from_data_url("data:image/jpeg;base64,abc123"),
        Some(ImageFormat::Jpeg)
    );
    assert_eq!(
        ImageFormat::from_data_url("data:image/png;base64,abc123"),
        Some(ImageFormat::Png)
    );
    assert_eq!(
        ImageFormat::from_data_url("data:image/gif;base64,abc123"),
        Some(ImageFormat::Gif)
    );
    assert_eq!(
        ImageFormat::from_data_url("data:image/webp;base64,abc123"),
        Some(ImageFormat::Webp)
    );
}

#[tokio::test]
async fn test_image_format_mime_types() {
    assert_eq!(ImageFormat::Jpeg.mime_type(), "image/jpeg");
    assert_eq!(ImageFormat::Png.mime_type(), "image/png");
    assert_eq!(ImageFormat::Gif.mime_type(), "image/gif");
    assert_eq!(ImageFormat::Webp.mime_type(), "image/webp");
}

#[tokio::test]
async fn test_base64_encoding_decoding() {
    let test_data = vec![1, 2, 3, 4, 5];

    // Test encoding
    let data_url = ImageUtils::encode_to_data_url(&test_data, &ImageFormat::Png);
    assert!(data_url.starts_with("data:image/png;base64,"));

    // Test decoding
    let decoded = ImageUtils::decode_from_data_url(&data_url).unwrap();
    assert_eq!(decoded, test_data);

    // Test invalid data URL
    let invalid_result = ImageUtils::decode_from_data_url("not-a-data-url");
    assert!(invalid_result.is_err());

    let invalid_structure = ImageUtils::decode_from_data_url("data:image/png;base64");
    assert!(invalid_structure.is_err());
}

#[tokio::test]
async fn test_image_validation() {
    // Valid formats
    assert!(ImageUtils::validate_format("https://example.com/test.jpg").is_ok());
    assert!(ImageUtils::validate_format("data:image/png;base64,abc123").is_ok());

    // Invalid formats
    assert!(ImageUtils::validate_format("https://example.com/test.bmp").is_err());
    assert!(ImageUtils::validate_format("data:image/svg+xml;base64,abc123").is_err());
}

#[tokio::test]
async fn test_token_estimation() {
    // Test detail level estimates
    assert_eq!(ImageUtils::estimate_tokens(&ImageDetail::Low), 85);
    assert_eq!(ImageUtils::estimate_tokens(&ImageDetail::High), 170);
    assert_eq!(ImageUtils::estimate_tokens(&ImageDetail::Auto), 85);

    // Test message token estimation
    let text_msg = Message::user("Hello world");
    assert!(text_msg.estimate_tokens() > 0);

    let image_msg = Message::user_with_image("Analyze", "https://example.com/test.jpg");
    assert!(image_msg.estimate_tokens() > text_msg.estimate_tokens());

    let multi_image_msg = Message::user_with_images(
        "Compare",
        vec![
            "https://example.com/1.jpg".to_string(),
            "https://example.com/2.jpg".to_string(),
        ],
    );
    assert!(multi_image_msg.estimate_tokens() > image_msg.estimate_tokens());
}

#[tokio::test]
async fn test_image_detail_levels() {
    assert_eq!(ImageDetail::default(), ImageDetail::Auto);

    // Test serialization behavior
    let low_detail = ImageDetail::Low;
    let high_detail = ImageDetail::High;
    let _auto_detail = ImageDetail::Auto;

    // These should have different token estimates
    assert_ne!(
        ImageUtils::estimate_tokens(&low_detail),
        ImageUtils::estimate_tokens(&high_detail)
    );
}

#[tokio::test]
async fn test_message_content_input() {
    // Test text input
    let text_input = MessageContentInput::Text("Hello".to_string());
    match text_input {
        MessageContentInput::Text(text) => assert_eq!(text, "Hello"),
        _ => panic!("Expected text input"),
    }

    // Test array input
    let array_input = MessageContentInput::Array(vec![
        MessageContent::text("Hello"),
        MessageContent::image_url("https://example.com/test.jpg"),
    ]);
    match array_input {
        MessageContentInput::Array(contents) => {
            assert_eq!(contents.len(), 2);
            match &contents[0] {
                MessageContent::Text { text } => assert_eq!(text, "Hello"),
                _ => panic!("Expected text content"),
            }
            match &contents[1] {
                MessageContent::Image { image_url } => {
                    assert_eq!(image_url.url, "https://example.com/test.jpg");
                }
                _ => panic!("Expected image content"),
            }
        }
        _ => panic!("Expected array input"),
    }
}

#[tokio::test]
async fn test_base64_image_creation() {
    let test_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header

    let msg = Message::user_with_image_bytes("What is this?", &test_data, &ImageFormat::Png);

    assert!(msg.has_images());
    let urls = msg.image_urls();
    assert_eq!(urls.len(), 1);
    assert!(urls[0].starts_with("data:image/png;base64,"));
}

#[tokio::test]
async fn test_backward_compatibility() {
    // Ensure existing Message creation methods still work
    let user_msg = Message::user("Hello");
    let assistant_msg = Message::assistant("Hi there");
    let developer_msg = Message::developer("System prompt");
    let system_msg = Message::system("Legacy system message");

    assert_eq!(user_msg.role, MessageRole::User);
    assert_eq!(assistant_msg.role, MessageRole::Assistant);
    assert_eq!(developer_msg.role, MessageRole::Developer);
    assert_eq!(system_msg.role, MessageRole::System);

    // All should have text content
    assert_eq!(user_msg.text_content(), "Hello");
    assert_eq!(assistant_msg.text_content(), "Hi there");
    assert_eq!(developer_msg.text_content(), "System prompt");
    assert_eq!(system_msg.text_content(), "Legacy system message");

    // None should have images
    assert!(!user_msg.has_images());
    assert!(!assistant_msg.has_images());
    assert!(!developer_msg.has_images());
    assert!(!system_msg.has_images());
}

#[tokio::test]
async fn test_api_format_conversion() {
    let client = OpenAIClient::new("test-key").unwrap();

    // Test text-only message conversion
    let text_msg = Message::user("Hello");
    let request = ResponseRequest::new_messages("gpt-4", vec![text_msg]);

    // This should not panic and should create valid JSON
    let api_format = client.responses().to_openai_format(&request);
    assert!(api_format.is_ok());

    // Test image message conversion
    let image_msg = Message::user_with_image("Analyze", "https://example.com/test.jpg");
    let image_request = ResponseRequest::new_messages("gpt-4-vision-preview", vec![image_msg]);

    let image_api_format = client.responses().to_openai_format(&image_request);
    assert!(image_api_format.is_ok());

    // Test multi-content message
    let multi_msg = Message::user_with_content(vec![
        MessageContent::text("Look at this"),
        MessageContent::image_url_with_detail("https://example.com/test.jpg", ImageDetail::High),
    ]);
    let multi_request = ResponseRequest::new_messages("gpt-4-vision-preview", vec![multi_msg]);

    let multi_api_format = client.responses().to_openai_format(&multi_request);
    assert!(multi_api_format.is_ok());
}
