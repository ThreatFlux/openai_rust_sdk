//! Utility functions and advanced demonstrations
//!
//! This module contains utility demos, error handling, cost estimation,
//! file operations, and other advanced features.

use openai_rust_sdk::{
    api::images::{ImageRecommendationUtils, ImageSupportUtils, ImageUtils, ImagesApi},
    error::Result,
    models::images::*,
};
use std::path::PathBuf;

/// Demo 6: Utility Functions
pub async fn demo_utilities(_api: &ImagesApi) -> Result<()> {
    println!("🛠️ Demo 6: Utility Functions");
    println!("----------------------------");

    show_supported_formats();
    test_format_validation();
    show_use_case_recommendations();
    demonstrate_file_size_estimation();
    demonstrate_prompt_enhancement();

    println!();
    Ok(())
}

fn show_supported_formats() {
    println!("Supported image formats:");
    for format in ImageSupportUtils::supported_input_formats() {
        println!("   • {}", format);
    }
}

fn test_format_validation() {
    println!("\nFormat validation tests:");
    let test_files = vec!["image.png", "photo.jpg", "graphic.webp", "animation.gif"];
    for file in test_files {
        let is_supported = ImageSupportUtils::is_supported_format(file);
        println!(
            "   {} - {}",
            file,
            if is_supported {
                "✅ Supported"
            } else {
                "❌ Not supported"
            }
        );
    }
}

fn show_use_case_recommendations() {
    println!("\nRecommended settings for different use cases:");
    let use_cases = vec!["avatar", "wallpaper", "poster", "thumbnail", "concept"];
    for use_case in use_cases {
        let (model, size, quality, style) = ImageRecommendationUtils::recommend_settings(use_case);
        println!(
            "   {}: {} at {:?} (Quality: {:?}, Style: {:?})",
            use_case, model, size, quality, style
        );
    }
}

fn demonstrate_file_size_estimation() {
    println!("\nEstimated file sizes:");
    let sizes = vec![
        ImageSize::Size256x256,
        ImageSize::Size512x512,
        ImageSize::Size1024x1024,
    ];
    for size in sizes {
        let png_size = ImageUtils::estimate_file_size(&size, "png");
        let jpg_size = ImageUtils::estimate_file_size(&size, "jpg");
        println!(
            "   {:?}: PNG ~{}KB, JPG ~{}KB",
            size,
            png_size / 1024,
            jpg_size / 1024
        );
    }
}

fn demonstrate_prompt_enhancement() {
    println!("\nPrompt enhancement:");
    let basic_prompt = "A cat sitting";
    let enhanced =
        ImageUtils::enhance_prompt(basic_prompt, Some("photorealistic, studio lighting"));
    println!("   Original: {}", basic_prompt);
    println!("   Enhanced: {}", enhanced);
}

/// Demo 7: Error Handling and Validation
pub async fn demo_error_handling(_api: &ImagesApi) -> Result<()> {
    println!("⚠️ Demo 7: Error Handling and Validation");
    println!("---------------------------------------");

    // Test DALL-E 3 validation
    println!("Testing DALL-E 3 validation:");

    let mut dall_e_3_request = ImageGenerationRequest::new(ImageModels::DALL_E_3, "Test image");

    // Try to generate multiple images with DALL-E 3 (should fail)
    dall_e_3_request.n = Some(2);
    match dall_e_3_request.validate() {
        Ok(_) => println!("   ❌ Validation should have failed"),
        Err(e) => println!("   ✅ Correctly caught error: {}", e),
    }

    // Try unsupported size with DALL-E 3
    dall_e_3_request.n = Some(1);
    dall_e_3_request.size = Some(ImageSize::Size256x256);
    match dall_e_3_request.validate() {
        Ok(_) => println!("   ❌ Validation should have failed"),
        Err(e) => println!("   ✅ Correctly caught error: {}", e),
    }

    // Test DALL-E 2 validation
    println!("\nTesting DALL-E 2 validation:");

    let mut dall_e_2_request = ImageGenerationRequest::new(ImageModels::DALL_E_2, "Test image");

    // Try to use quality with DALL-E 2 (should fail)
    dall_e_2_request.quality = Some(ImageQuality::Hd);
    match dall_e_2_request.validate() {
        Ok(_) => println!("   ❌ Validation should have failed"),
        Err(e) => println!("   ✅ Correctly caught error: {}", e),
    }

    // Try to use style with DALL-E 2 (should fail)
    dall_e_2_request.quality = None;
    dall_e_2_request.style = Some(ImageStyle::Vivid);
    match dall_e_2_request.validate() {
        Ok(_) => println!("   ❌ Validation should have failed"),
        Err(e) => println!("   ✅ Correctly caught error: {}", e),
    }

    // Try unsupported size with DALL-E 2
    dall_e_2_request.style = None;
    dall_e_2_request.size = Some(ImageSize::Size1792x1024);
    match dall_e_2_request.validate() {
        Ok(_) => println!("   ❌ Validation should have failed"),
        Err(e) => println!("   ✅ Correctly caught error: {}", e),
    }

    // Valid DALL-E 2 request
    dall_e_2_request.size = Some(ImageSize::Size512x512);
    dall_e_2_request.n = Some(3);
    match dall_e_2_request.validate() {
        Ok(_) => println!("   ✅ Valid DALL-E 2 request passed validation"),
        Err(e) => println!("   ❌ Valid request failed: {}", e),
    }

    println!();
    Ok(())
}

/// Demo 8: Cost Estimation
pub fn demo_cost_estimation() {
    println!("💰 Demo 8: Cost Estimation");
    println!("---------------------------");

    // DALL-E 3 costs
    println!("DALL-E 3 cost estimates:");
    let dall_e_3_costs = vec![
        (ImageSize::Size1024x1024, Some(ImageQuality::Standard), 1),
        (ImageSize::Size1024x1024, Some(ImageQuality::Hd), 1),
        (ImageSize::Size1792x1024, Some(ImageQuality::Standard), 1),
        (ImageSize::Size1792x1024, Some(ImageQuality::Hd), 1),
    ];

    for (size, quality, count) in dall_e_3_costs {
        let cost = ImageRecommendationUtils::estimate_cost(
            ImageModels::DALL_E_3.to_string().as_str(),
            &size,
            quality.as_ref(),
            count,
        );
        println!(
            "   {:?} {:?}: ${:.3}",
            size,
            quality.unwrap_or(ImageQuality::Standard),
            cost
        );
    }

    // DALL-E 2 costs
    println!("\nDALL-E 2 cost estimates:");
    let dall_e_2_costs = vec![
        (ImageSize::Size256x256, 1),
        (ImageSize::Size512x512, 1),
        (ImageSize::Size1024x1024, 1),
        (ImageSize::Size512x512, 5), // Multiple images
    ];

    for (size, count) in dall_e_2_costs {
        let cost = ImageRecommendationUtils::estimate_cost(
            ImageModels::DALL_E_2.to_string().as_str(),
            &size,
            None,
            count,
        );
        println!("   {:?} x{}: ${:.3}", size, count, cost);
    }

    println!();
}

/// Demo 10: File Operations
pub async fn demo_file_operations(_api: &ImagesApi) -> Result<()> {
    println!("📁 Demo 10: File Operations");
    println!("---------------------------");

    demonstrate_file_generation_mock();
    test_image_data_methods();
    test_url_download_mock();

    println!();
    Ok(())
}

/// Demonstrate file generation operations (mock)
fn demonstrate_file_generation_mock() {
    let output_dir = PathBuf::from("./demo_images");
    println!("Testing file operations (mock):");
    println!(
        "   Would generate and save images to: {}",
        output_dir.display()
    );

    // The actual call would be:
    // match api.generate_and_save_images(
    //     "A collection of abstract art pieces",
    //     &output_dir,
    //     Some(ImageModels::DALL_E_2),
    //     Some(ImageSize::Size512x512),
    //     None,
    //     Some(3),
    // ).await {
    //     Ok(saved_files) => {
    //         println!("   ✅ Saved {} files:", saved_files.len());
    //         for file in saved_files {
    //             println!("      • {}", file);
    //         }
    //     }
    //     Err(e) => {
    //         println!("   ❌ Error saving files: {}", e);
    //     }
    // }
}

/// Test ImageData methods and functionality
fn test_image_data_methods() {
    println!("\nTesting ImageData methods:");
    let mock_image_data = create_mock_image_data();

    println!("   Has URL: {}", mock_image_data.has_url());
    println!("   Has base64: {}", mock_image_data.has_b64_json());
    println!("   URL: {:?}", mock_image_data.get_url());
    println!(
        "   Revised prompt: {:?}",
        mock_image_data.get_revised_prompt()
    );

    // Test base64 decoding
    match mock_image_data.decode_b64_json() {
        Ok(data) => println!(
            "   ✅ Successfully decoded {} bytes of image data",
            data.len()
        ),
        Err(e) => println!("   ❌ Failed to decode base64: {}", e),
    }
}

/// Create mock ImageData for testing
fn create_mock_image_data() -> ImageData {
    ImageData {
        url: Some("https://example.com/image.png".to_string()),
        b64_json: Some("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==".to_string()),
        revised_prompt: Some("A single pixel image for testing".to_string()),
    }
}

/// Test URL download functionality (mock)
fn test_url_download_mock() {
    println!("\nTesting URL download (mock):");
    let mock_url = "https://example.com/image.png";
    println!("   Would download from: {}", mock_url);
    // In real usage: api.download_image(url, "downloaded_image.png").await?;
    println!("   ✅ Download simulation successful");
}

/// Run utility and advanced feature demos
pub async fn run_utility_demos(images_api: &ImagesApi) -> Result<()> {
    // Demo 6: Utility Functions
    demo_utilities(images_api).await?;

    // Demo 7: Error Handling and Validation
    demo_error_handling(images_api).await?;

    // Demo 8: Cost Estimation
    demo_cost_estimation();

    // Demo 10: File Operations
    demo_file_operations(images_api).await?;

    Ok(())
}

/// Helper function to demonstrate error handling patterns
pub async fn _example_error_handling_pattern(api: &ImagesApi) -> Result<()> {
    let request = ImageGenerationRequest::new(ImageModels::DALL_E_3, "A beautiful landscape");

    match api.create_image(&request).await {
        Ok(response) => {
            println!("Success! Generated {} images", response.count());

            // Process each image
            for (i, image) in response.data.iter().enumerate() {
                match image.get_url() {
                    Some(url) => {
                        println!("Image {}: {}", i + 1, url);
                        // Download the image
                        let filename = format!("image_{}.png", i + 1);
                        match api.download_image(url, &filename).await {
                            Ok(_) => println!("Saved to {}", filename),
                            Err(e) => eprintln!("Failed to download {}: {}", filename, e),
                        }
                    }
                    None => {
                        if let Some(b64_data) = image.get_b64_json() {
                            println!("Image {} as base64 ({} chars)", i + 1, b64_data.len());
                            // Save base64 data
                            let filename = format!("image_{}.png", i + 1);
                            match image.save_b64_to_file(&filename).await {
                                Ok(_) => println!("Saved base64 to {}", filename),
                                Err(e) => eprintln!("Failed to save {}: {}", filename, e),
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error generating images: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
