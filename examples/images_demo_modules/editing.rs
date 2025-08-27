//! Image editing demonstrations
//!
//! This module contains all demos related to editing existing images using DALL-E models.

use openai_rust_sdk::{
    api::images::{ImageUtils, ImagesApi},
    error::Result,
    models::images::*,
};

/// Demo 4: Image Editing (requires sample image)
pub async fn demo_image_editing(_api: &ImagesApi) -> Result<()> {
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

/// Demonstrate ImageEditBuilder usage
pub fn demo_edit_builder() {
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
}
