//! Image generation demonstrations
//!
//! This module contains all demos related to generating new images using DALL-E models.

use openai_rust_sdk::{api::images::ImagesApi, error::Result, models::images::*};

/// Demo 1: Basic Image Generation
pub async fn demo_basic_generation(api: &ImagesApi) -> Result<()> {
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
pub async fn demo_advanced_generation(api: &ImagesApi) -> Result<()> {
    println!("🎯 Demo 2: Advanced Image Generation");
    println!("------------------------------------");

    generate_hd_landscape_image(api).await?;
    generate_portrait_image(api).await?;

    println!();
    Ok(())
}

async fn generate_hd_landscape_image(api: &ImagesApi) -> Result<()> {
    println!("Generating HD quality image with natural style...");

    let request = ImageGenerationRequest::new(
        ImageModels::DALL_E_3,
        "A serene mountain lake reflecting snow-capped peaks, painted in watercolor style",
    )
    .with_size(ImageSize::Size1792x1024)
    .with_quality(ImageQuality::Hd)
    .with_style(ImageStyle::Natural)
    .with_response_format(ImageResponseFormat::B64Json);

    match api.create_image(&request).await {
        Ok(response) => {
            println!("✅ HD image generated successfully!");
            if let Some(first_image) = response.first_image() {
                if first_image.has_b64_json() {
                    println!("   Received base64 encoded image data");
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
    Ok(())
}

async fn generate_portrait_image(api: &ImagesApi) -> Result<()> {
    println!("\nGenerating portrait format image with vivid style...");

    let portrait_request = ImageGenerationRequest::new(
        ImageModels::DALL_E_3,
        "A majestic dragon soaring through clouds with rainbow wings",
    )
    .with_size(ImageSize::Size1024x1792)
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
    Ok(())
}

/// Demo 3: Image Generation with Multiple Images (DALL-E 2)
pub async fn demo_multiple_images(api: &ImagesApi) -> Result<()> {
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

/// Demonstrate ImageGenerationBuilder usage
pub fn demo_generation_builder() {
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
}

/// Demonstrate convenience method usage
pub async fn demo_convenience_method(api: &ImagesApi) -> Result<()> {
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
    Ok(())
}

/// Run all image generation related demos
pub async fn run_generation_demos(images_api: &ImagesApi) -> Result<()> {
    // Demo 1: Basic Image Generation
    demo_basic_generation(images_api).await?;

    // Demo 2: Advanced Image Generation with Different Settings
    demo_advanced_generation(images_api).await?;

    // Demo 3: Image Generation with Multiple Images
    demo_multiple_images(images_api).await?;

    Ok(())
}
