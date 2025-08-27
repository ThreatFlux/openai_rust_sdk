#![allow(clippy::pedantic, clippy::nursery)]
//! `OpenAI` Vision API Examples
//!
//! This example demonstrates the vision capabilities of the `OpenAI` SDK,
//! including image analysis, multi-image processing, base64 encoding,
//! and various detail levels.

use openai_rust_sdk::{
    from_env, ImageDetail, ImageFormat, ImageUtils, Message, OpenAIClient, ResponseRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = initialize_client()?;
    print_header();

    run_basic_examples(&client).await?;
    run_advanced_examples(&client).await?;
    run_utility_examples().await?;
    run_compatibility_examples(&client).await?;

    print_completion_message();
    Ok(())
}

/// Initialize the OpenAI client
fn initialize_client() -> Result<OpenAIClient, Box<dyn std::error::Error>> {
    if let Ok(client) = from_env() {
        Ok(client)
    } else {
        println!("OPENAI_API_KEY not set, using demo mode with test key");
        Ok(OpenAIClient::new("test-key-for-demo")?)
    }
}

/// Print the demo header
fn print_header() {
    println!("=== OpenAI Vision API Examples ===\n");
}

/// Run basic vision examples
async fn run_basic_examples(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    simple_image_analysis(client).await?;
    image_detail_levels(client).await?;
    Ok(())
}

/// Run advanced vision examples
async fn run_advanced_examples(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    multi_image_analysis(client).await?;
    base64_image_processing(client).await?;
    Ok(())
}

/// Run utility examples
async fn run_utility_examples() -> Result<(), Box<dyn std::error::Error>> {
    image_format_validation().await?;
    token_estimation().await?;
    Ok(())
}

/// Run compatibility examples
async fn run_compatibility_examples(
    client: &OpenAIClient,
) -> Result<(), Box<dyn std::error::Error>> {
    backward_compatibility(client).await?;
    Ok(())
}

/// Print completion message
fn print_completion_message() {
    println!("\n=== All vision examples completed! ===");
}

/// Example 1: Simple image analysis with URL
async fn simple_image_analysis(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🖼️  Example 1: Simple Image Analysis");
    println!("-----------------------------------");

    // Using a publicly available image URL for demonstration
    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg";

    let message = Message::user_with_image(
        "What can you see in this image? Describe it in detail.",
        image_url,
    );

    let request =
        ResponseRequest::new_messages("gpt-4-vision-preview", vec![message]).with_max_tokens(500);

    match client.create_response(&request).await {
        Ok(response) => {
            println!("Image URL: {image_url}");
            if let Some(choice) = response.choices.first() {
                println!(
                    "Analysis: {}",
                    choice.message.content.as_deref().unwrap_or("No content")
                );
                if let Some(usage) = &response.usage {
                    println!(
                        "Tokens used: {} (prompt: {}, completion: {})",
                        usage.total_tokens, usage.prompt_tokens, usage.completion_tokens
                    );
                }
            }
        }
        Err(e) => {
            println!("Error (expected in demo mode): {e}");
        }
    }

    println!();
    Ok(())
}

/// Example 2: Image analysis with different detail levels
async fn image_detail_levels(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Example 2: Image Detail Levels");
    println!("---------------------------------");

    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a7/React-icon.svg/1280px-React-icon.svg.png";

    // Test different detail levels
    let detail_levels = vec![
        (ImageDetail::Low, "low detail (faster, fewer tokens)"),
        (ImageDetail::High, "high detail (slower, more tokens)"),
        (ImageDetail::Auto, "auto detail (balanced)"),
    ];

    for (detail, description) in detail_levels {
        println!("Testing {description} detail level:");

        let message = Message::user_with_image_detail(
            "Analyze this image and tell me what programming technology it represents.",
            image_url,
            detail,
        );

        let estimated_tokens = message.estimate_tokens();
        println!("Estimated tokens for this message: {estimated_tokens}");

        let request = ResponseRequest::new_messages("gpt-4-vision-preview", vec![message])
            .with_max_tokens(200);

        match client.create_response(&request).await {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    println!(
                        "Response: {}",
                        choice.message.content.as_deref().unwrap_or("No content")
                    );
                }
            }
            Err(e) => {
                println!("Error (expected in demo mode): {e}");
            }
        }
        println!();
    }

    Ok(())
}

/// Example 3: Multi-image analysis
async fn multi_image_analysis(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📷 Example 3: Multi-Image Analysis");
    println!("----------------------------------");

    let image_urls = vec![
        "https://upload.wikimedia.org/wikipedia/commons/thumb/4/4c/Typescript_logo_2020.svg/1200px-Typescript_logo_2020.svg.png".to_string(),
        "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a7/React-icon.svg/1280px-React-icon.svg.png".to_string(),
    ];

    let message = Message::user_with_images(
        "Compare these two programming technologies. What are the differences and how do they relate to each other?",
        image_urls,
    );

    println!(
        "Analyzing {} images in a single message",
        message.image_urls().len()
    );
    println!("Image URLs: {:?}", message.image_urls());
    println!("Message contains images: {}", message.has_images());
    println!("Text content only: {}", message.text_content());

    let request =
        ResponseRequest::new_messages("gpt-4-vision-preview", vec![message]).with_max_tokens(600);

    match client.create_response(&request).await {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!(
                    "Comparison: {}",
                    choice.message.content.as_deref().unwrap_or("No content")
                );
            }
        }
        Err(e) => {
            println!("Error (expected in demo mode): {e}");
        }
    }

    println!();
    Ok(())
}

/// Example 4: Base64 image processing
async fn base64_image_processing(_client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Example 4: Base64 Image Processing");
    println!("------------------------------------");

    // Create a simple test image (1x1 pixel PNG)
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    // Encode to base64 data URL
    let data_url = ImageUtils::encode_to_data_url(&png_data, &ImageFormat::Png);
    println!("Created base64 data URL (length: {})", data_url.len());
    println!("Data URL preview: {}...", &data_url[..100]);

    // Validate the format
    match ImageUtils::validate_format(&data_url) {
        Ok(format) => {
            println!(
                "Detected format: {:?} (MIME: {})",
                format,
                format.mime_type()
            );
        }
        Err(e) => {
            println!("Format validation error: {e}");
        }
    }

    // Decode back to bytes
    match ImageUtils::decode_from_data_url(&data_url) {
        Ok(decoded_data) => {
            println!("Successfully decoded {} bytes", decoded_data.len());
            println!("Original data matches: {}", decoded_data == png_data);
        }
        Err(e) => {
            println!("Decode error: {e}");
        }
    }

    // Create a message with the base64 image
    let message =
        Message::user_with_image_bytes("What color is this pixel?", &png_data, &ImageFormat::Png);

    println!("Created message with base64 image data");
    println!("Message contains images: {}", message.has_images());

    println!();
    Ok(())
}

/// Example 5: Image format validation
async fn image_format_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ Example 5: Image Format Validation");
    println!("-------------------------------------");

    let test_urls = vec![
        "https://example.com/image.jpg",
        "https://example.com/image.png",
        "https://example.com/image.gif",
        "https://example.com/image.webp",
        "https://example.com/image.bmp", // Unsupported
        "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEASABIAAD...",
        "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==",
        "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCI+PC9zdmc+", // Unsupported
    ];

    for url in test_urls {
        print!("Testing URL: {}... ", &url[..50.min(url.len())]);

        match ImageUtils::validate_format(url) {
            Ok(format) => {
                println!("✓ Valid {} format", format.mime_type());

                // Test format detection methods
                if url.starts_with("data:") {
                    if let Some(detected) = ImageFormat::from_data_url(url) {
                        println!("  - Detected from data URL: {detected:?}");
                    }
                } else if let Some(detected) = ImageFormat::from_url(url) {
                    println!("  - Detected from URL extension: {detected:?}");
                }
            }
            Err(e) => {
                println!("✗ {e}");
            }
        }
    }

    println!();
    Ok(())
}

/// Example 6: Token estimation for images
async fn token_estimation() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 Example 6: Token Estimation");
    println!("------------------------------");

    // Test token estimation for different detail levels
    println!("Token estimates by detail level:");
    println!(
        "- Low detail: {} tokens",
        ImageUtils::estimate_tokens(&ImageDetail::Low)
    );
    println!(
        "- High detail: {} tokens",
        ImageUtils::estimate_tokens(&ImageDetail::High)
    );
    println!(
        "- Auto detail: {} tokens",
        ImageUtils::estimate_tokens(&ImageDetail::Auto)
    );

    // Create messages with different content types
    let text_message = Message::user("This is a text-only message for comparison.");
    let image_message =
        Message::user_with_image("Analyze this image", "https://example.com/image.jpg");
    let multi_image_message = Message::user_with_images(
        "Compare these images",
        vec![
            "https://example.com/image1.jpg".to_string(),
            "https://example.com/image2.jpg".to_string(),
        ],
    );

    println!("\nMessage token estimates:");
    println!("- Text only: {} tokens", text_message.estimate_tokens());
    println!("- Text + image: {} tokens", image_message.estimate_tokens());
    println!(
        "- Text + 2 images: {} tokens",
        multi_image_message.estimate_tokens()
    );

    println!();
    Ok(())
}

/// Example 7: Backward compatibility with text-only messages
async fn backward_compatibility(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("⬅️  Example 7: Backward Compatibility");
    println!("-------------------------------------");

    // Test that existing text-only functionality still works
    let text_message = Message::user("What is the capital of France?");

    println!("Text message content: {}", text_message.text_content());
    println!("Message contains images: {}", text_message.has_images());
    println!("Image URLs: {:?}", text_message.image_urls());

    let request =
        ResponseRequest::new_messages("gpt-3.5-turbo", vec![text_message]).with_max_tokens(100);

    match client.create_response(&request).await {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!(
                    "Response: {}",
                    choice.message.content.as_deref().unwrap_or("No content")
                );
            }
        }
        Err(e) => {
            println!("Error (expected in demo mode): {e}");
        }
    }

    // Test legacy builder methods
    let conversation = [
        Message::system("You are a helpful assistant."),
        Message::user("Hello!"),
        Message::assistant("Hi there! How can I help you today?"),
        Message::user("What's 2+2?"),
    ];

    println!("\nLegacy conversation format still works:");
    for (i, msg) in conversation.iter().enumerate() {
        println!("{}. {:?}: {}", i + 1, msg.role, msg.text_content());
    }

    println!();
    Ok(())
}
