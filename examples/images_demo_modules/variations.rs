//! Image variation demonstrations
//!
//! This module contains all demos related to creating variations of existing images.

use openai_rust_sdk::{
    api::images::{ImageUtils, ImagesApi},
    error::Result,
    models::images::*,
};

/// Demo 5: Image Variations (requires sample image)
pub async fn demo_image_variations(_api: &ImagesApi) -> Result<()> {
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

/// Demonstrate ImageVariationBuilder usage
pub fn demo_variation_builder() {
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
}
