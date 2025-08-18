//! # Images API Demo
//!
//! This example demonstrates the comprehensive usage of the Images API,
//! including image generation, editing, and variation creation using DALL-E models.

use openai_rust_sdk::{
    api::images::{ImageUtils, ImagesApi},
    error::Result,
    models::images::*,
};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Images API client
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable must be set");

    let images_api = ImagesApi::new(api_key)?;

    println!("🎨 OpenAI Images API Demo");
    println!("========================\n");

    // Demo 1: Basic Image Generation
    demo_basic_generation(&images_api).await?;

    // Demo 2: Advanced Image Generation with Different Settings
    demo_advanced_generation(&images_api).await?;

    // Demo 3: Image Generation with Multiple Images
    demo_multiple_images(&images_api).await?;

    // Demo 4: Image Editing (requires sample image)
    demo_image_editing(&images_api).await?;

    // Demo 5: Image Variations (requires sample image)
    demo_image_variations(&images_api).await?;

    // Demo 6: Utility Functions
    demo_utilities(&images_api).await?;

    // Demo 7: Error Handling and Validation
    demo_error_handling(&images_api).await?;

    // Demo 8: Cost Estimation
    demo_cost_estimation();

    // Demo 9: Builder Patterns
    demo_builder_patterns(&images_api).await?;

    // Demo 10: File Operations
    demo_file_operations(&images_api).await?;

    println!("\n✅ All demos completed successfully!");

    Ok(())
}

/// Demo 1: Basic Image Generation
async fn demo_basic_generation(api: &ImagesApi) -> Result<()> {
    println!("📝 Demo 1: Basic Image Generation");
    println!("----------------------------------");

    // Simple image generation with DALL-E 3
    println!("Generating a simple image with DALL-E 3...");

    let request = ImageGenerationRequest::new(
        ImageModels::DALL_E_3,
        "A futuristic cityscape at sunset with flying cars and neon lights",
    )
    .with_size(ImageSize::Size1024x1024)
    .with_quality(ImageQuality::Standard)
    .with_style(ImageStyle::Vivid)
    .with_response_format(ImageResponseFormat::Url);

    match api.create_image(&request).await {
        Ok(response) => {
            println!("✅ Image generated successfully!");
            println!("   Created: {}", response.created);
            println!("   Number of images: {}", response.count());

            if let Some(first_image) = response.first_image() {
                if let Some(url) = first_image.get_url() {
                    println!("   Image URL: {}", url);
                }
                if let Some(revised_prompt) = first_image.get_revised_prompt() {
                    println!("   Revised prompt: {}", revised_prompt);
                }
            }
        }
        Err(e) => {
            println!("❌ Error generating image: {}", e);
        }
    }

    println!();
    Ok(())
}

/// Demo 2: Advanced Image Generation with Different Settings
async fn demo_advanced_generation(api: &ImagesApi) -> Result<()> {
    println!("🎯 Demo 2: Advanced Image Generation");
    println!("------------------------------------");

    // DALL-E 3 with HD quality and natural style
    println!("Generating HD quality image with natural style...");

    let request = ImageGenerationRequest::new(
        ImageModels::DALL_E_3,
        "A serene mountain lake reflecting snow-capped peaks, painted in watercolor style",
    )
    .with_size(ImageSize::Size1792x1024) // Landscape format
    .with_quality(ImageQuality::Hd)
    .with_style(ImageStyle::Natural)
    .with_response_format(ImageResponseFormat::B64Json);

    match api.create_image(&request).await {
        Ok(response) => {
            println!("✅ HD image generated successfully!");
            if let Some(first_image) = response.first_image() {
                if first_image.has_b64_json() {
                    println!("   Received base64 encoded image data");
                    // In a real application, you would save this to a file
                    println!(
                        "   Base64 data length: ~{} characters",
                        first_image.get_b64_json().unwrap_or("").len()
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Error generating HD image: {}", e);
        }
    }

    // Portrait format with vivid style
    println!("\nGenerating portrait format image with vivid style...");

    let portrait_request = ImageGenerationRequest::new(
        ImageModels::DALL_E_3,
        "A majestic dragon soaring through clouds with rainbow wings",
    )
    .with_size(ImageSize::Size1024x1792) // Portrait format
    .with_quality(ImageQuality::Hd)
    .with_style(ImageStyle::Vivid)
    .with_response_format(ImageResponseFormat::Url);

    match api.create_image(&portrait_request).await {
        Ok(response) => {
            println!("✅ Portrait image generated successfully!");
            if let Some(url) = response.get_urls().first() {
                println!("   Portrait URL: {}", url);
            }
        }
        Err(e) => {
            println!("❌ Error generating portrait image: {}", e);
        }
    }

    println!();
    Ok(())
}

/// Demo 3: Image Generation with Multiple Images (DALL-E 2)
async fn demo_multiple_images(api: &ImagesApi) -> Result<()> {
    println!("📊 Demo 3: Multiple Image Generation");
    println!("------------------------------------");

    // DALL-E 2 can generate multiple images in one request
    println!("Generating multiple images with DALL-E 2...");

    let request = ImageGenerationRequest::new(
        ImageModels::DALL_E_2,
        "A cute robot helping in a garden, cartoon style",
    )
    .with_n(3) // Generate 3 images
    .with_size(ImageSize::Size512x512)
    .with_response_format(ImageResponseFormat::Url);

    match api.create_image(&request).await {
        Ok(response) => {
            println!("✅ Generated {} images successfully!", response.count());
            for (i, url) in response.get_urls().iter().enumerate() {
                println!("   Image {}: {}", i + 1, url);
            }
        }
        Err(e) => {
            println!("❌ Error generating multiple images: {}", e);
        }
    }

    println!();
    Ok(())
}

/// Demo 4: Image Editing (requires sample image)
async fn demo_image_editing(api: &ImagesApi) -> Result<()> {
    println!("✂️ Demo 4: Image Editing");
    println!("------------------------");

    // Note: This demo assumes you have sample images
    // In a real scenario, you would provide actual image files

    println!("Image editing requires PNG files. Creating sample request...");

    let edit_request = ImageEditRequest::new(
        ImageModels::DALL_E_2,
        "sample_image.png", // This would be an actual PNG file
        "Add a beautiful rainbow in the sky",
    )
    .with_n(2)
    .with_size(ImageSize::Size512x512)
    .with_response_format(ImageResponseFormat::Url);

    println!("Edit request created:");
    println!("   Model: {}", edit_request.model);
    println!("   Image: {}", edit_request.image);
    println!("   Prompt: {}", edit_request.prompt);
    println!("   Number of variations: {:?}", edit_request.n);

    // We'll skip the actual API call since we don't have a real image file
    println!("⚠️  Skipping actual edit call (no sample image file provided)");

    // Demonstrate validation
    match ImageUtils::validate_image_requirements("edit", "sample.jpg") {
        Ok(_) => println!("✅ Image validation passed"),
        Err(e) => println!("❌ Image validation failed: {}", e),
    }

    println!();
    Ok(())
}

/// Demo 5: Image Variations (requires sample image)
async fn demo_image_variations(api: &ImagesApi) -> Result<()> {
    println!("🔄 Demo 5: Image Variations");
    println!("---------------------------");

    println!("Creating variations requires a PNG image. Setting up request...");

    let variation_request = ImageVariationRequest::new(
        ImageModels::DALL_E_2,
        "sample_square_image.png", // Must be square and PNG
    )
    .with_n(4) // Create 4 variations
    .with_size(ImageSize::Size256x256)
    .with_response_format(ImageResponseFormat::Url);

    println!("Variation request created:");
    println!("   Model: {}", variation_request.model);
    println!("   Source image: {}", variation_request.image);
    println!("   Number of variations: {:?}", variation_request.n);
    println!("   Size: {:?}", variation_request.size);

    // Check if image appears to be square (mock check)
    match ImageUtils::is_square_image("square_image_1024x1024.png").await {
        Ok(true) => println!("✅ Image appears to be square"),
        Ok(false) => println!("⚠️  Image may not be square"),
        Err(e) => println!("❌ Error checking image: {}", e),
    }

    println!("⚠️  Skipping actual variation call (no sample image file provided)");

    println!();
    Ok(())
}

/// Demo 6: Utility Functions
async fn demo_utilities(api: &ImagesApi) -> Result<()> {
    println!("🛠️ Demo 6: Utility Functions");
    println!("----------------------------");

    // Show supported formats
    println!("Supported image formats:");
    for format in ImagesApi::supported_input_formats() {
        println!("   • {}", format);
    }

    // Test format validation
    println!("\nFormat validation tests:");
    let test_files = vec!["image.png", "photo.jpg", "graphic.webp", "animation.gif"];
    for file in test_files {
        let is_supported = ImagesApi::is_supported_format(file);
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

    // Show recommendations for different use cases
    println!("\nRecommended settings for different use cases:");
    let use_cases = vec!["avatar", "wallpaper", "poster", "thumbnail", "concept"];
    for use_case in use_cases {
        let (model, size, quality, style) = ImagesApi::recommend_settings(use_case);
        println!(
            "   {}: {} at {:?} (Quality: {:?}, Style: {:?})",
            use_case, model, size, quality, style
        );
    }

    // File size estimation
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

    // Prompt enhancement
    println!("\nPrompt enhancement:");
    let basic_prompt = "A cat sitting";
    let enhanced =
        ImageUtils::enhance_prompt(basic_prompt, Some("photorealistic, studio lighting"));
    println!("   Original: {}", basic_prompt);
    println!("   Enhanced: {}", enhanced);

    println!();
    Ok(())
}

/// Demo 7: Error Handling and Validation
async fn demo_error_handling(_api: &ImagesApi) -> Result<()> {
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
fn demo_cost_estimation() {
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
        let cost = ImagesApi::estimate_cost(ImageModels::DALL_E_3, &size, quality.as_ref(), count);
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
        let cost = ImagesApi::estimate_cost(ImageModels::DALL_E_2, &size, None, count);
        println!("   {:?} x{}: ${:.3}", size, count, cost);
    }

    println!();
}

/// Demo 9: Builder Patterns
async fn demo_builder_patterns(api: &ImagesApi) -> Result<()> {
    println!("🏗️ Demo 9: Builder Patterns");
    println!("---------------------------");

    // Image Generation Builder
    println!("Using ImageGenerationBuilder:");
    let gen_request = ImageGenerationBuilder::dall_e_3("A cyberpunk street scene")
        .hd_quality()
        .vivid_style()
        .size_1024x1024()
        .url_format()
        .user("demo-user")
        .build();

    println!(
        "   Built request: {} with {:?} quality",
        gen_request.model, gen_request.quality
    );

    // Image Edit Builder
    println!("\nUsing ImageEditBuilder:");
    let edit_request = ImageEditBuilder::dall_e_2("original.png", "Add snow falling")
        .mask("snow_mask.png")
        .n(2)
        .url_format()
        .size(ImageSize::Size512x512)
        .user("edit-user")
        .build();

    println!(
        "   Built edit request for {} images",
        edit_request.n.unwrap_or(1)
    );

    // Image Variation Builder
    println!("\nUsing ImageVariationBuilder:");
    let var_request = ImageVariationBuilder::dall_e_2("source.png")
        .n(4)
        .b64_json_format()
        .size(ImageSize::Size256x256)
        .build();

    println!(
        "   Built variation request for {} variations",
        var_request.n.unwrap_or(1)
    );

    // Demonstrate the convenience methods
    println!("\nTesting convenience generation method:");
    match api
        .generate_image(
            "A minimalist geometric pattern",
            Some(ImageModels::DALL_E_3),
            Some(ImageSize::Size1024x1024),
            Some(ImageQuality::Standard),
        )
        .await
    {
        Ok(response) => {
            println!(
                "   ✅ Convenience method successful, {} image(s) generated",
                response.count()
            );
        }
        Err(e) => {
            println!("   ❌ Convenience method failed: {}", e);
        }
    }

    println!();
    Ok(())
}

/// Demo 10: File Operations
async fn demo_file_operations(api: &ImagesApi) -> Result<()> {
    println!("📁 Demo 10: File Operations");
    println!("---------------------------");

    // Create output directory
    let output_dir = PathBuf::from("./demo_images");

    println!("Testing file operations (mock):");

    // Generate and save images (mock demonstration)
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

    // Test image data methods
    println!("\nTesting ImageData methods:");
    let mock_image_data = ImageData {
        url: Some("https://example.com/image.png".to_string()),
        b64_json: Some("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==".to_string()),
        revised_prompt: Some("A single pixel image for testing".to_string()),
    };

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

    // Test downloading from URL (mock)
    println!("\nTesting URL download (mock):");
    if let Some(url) = mock_image_data.get_url() {
        println!("   Would download from: {}", url);
        // In real usage: api.download_image(url, "downloaded_image.png").await?;
        println!("   ✅ Download simulation successful");
    }

    println!();
    Ok(())
}

/// Helper function to demonstrate error handling patterns
async fn _example_error_handling_pattern(api: &ImagesApi) -> Result<()> {
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
