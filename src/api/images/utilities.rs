//! Image utilities and helper functions

use crate::error::{OpenAIError, Result};
use crate::models::images::{ImageModels, ImageQuality, ImageSize, ImageStyle};
use std::path::Path;

/// Image utilities
pub struct ImageUtils;

impl ImageUtils {
    /// Check if an image is square (required for edits/variations)
    pub async fn is_square_image(image_path: impl AsRef<Path>) -> Result<bool> {
        // This is a basic implementation that would need an image processing library
        // For now, we'll just assume PNG files with "square" in the name are square
        let filename = image_path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        Ok(filename.contains("square") || filename.contains("1024x1024"))
    }

    /// Estimate image file size in bytes
    #[must_use]
    pub fn estimate_file_size(size: &ImageSize, format: &str) -> u64 {
        let pixel_count = match size {
            ImageSize::Size256x256 => 256 * 256,
            ImageSize::Size512x512 => 512 * 512,
            ImageSize::Size1024x1024 => 1024 * 1024,
            ImageSize::Size1792x1024 => 1792 * 1024,
            ImageSize::Size1024x1792 => 1024 * 1792,
        };

        match format.to_lowercase().as_str() {
            "png" => pixel_count * 3, // Rough estimate: 3 bytes per pixel for PNG
            "jpg" | "jpeg" => pixel_count / 10, // JPEG compression ratio ~10:1
            "webp" => pixel_count / 15, // WebP compression ratio ~15:1
            _ => pixel_count * 3,     // Default to PNG estimate
        }
    }

    /// Generate prompt suggestions for better results
    #[must_use]
    pub fn enhance_prompt(basic_prompt: &str, style_hints: Option<&str>) -> String {
        let mut enhanced = basic_prompt.to_string();

        if let Some(hints) = style_hints {
            enhanced = format!("{enhanced}, {hints}");
        }

        // Add common quality enhancers
        if !enhanced.contains("high quality") && !enhanced.contains("detailed") {
            enhanced = format!("{enhanced}, high quality, detailed");
        }

        enhanced
    }

    /// Validate image requirements for different operations
    pub fn validate_image_requirements(
        operation: &str,
        image_path: impl AsRef<Path>,
    ) -> Result<()> {
        let path = image_path.as_ref();

        // Check file extension
        if !ImageSupportUtils::is_supported_format(path) {
            return Err(OpenAIError::invalid_request(
                "Image must be in PNG, JPG, JPEG, or WebP format",
            ));
        }

        // Check file name for operation-specific requirements
        match operation {
            "edit" | "variation" => {
                // These operations require PNG format and square images
                if let Some(ext) = path.extension() {
                    if ext.to_str().unwrap_or("").to_lowercase() != "png" {
                        return Err(OpenAIError::invalid_request(
                            "Image edits and variations require PNG format",
                        ));
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Image format and support utilities
pub struct ImageSupportUtils;

impl ImageSupportUtils {
    /// Get supported image formats for input
    #[must_use]
    pub fn supported_input_formats() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg", "webp"]
    }

    /// Validate image file format
    pub fn is_supported_format(file_path: impl AsRef<Path>) -> bool {
        if let Some(extension) = file_path.as_ref().extension() {
            if let Some(ext_str) = extension.to_str() {
                return Self::supported_input_formats().contains(&ext_str.to_lowercase().as_str());
            }
        }
        false
    }
}

/// Image recommendation and pricing utilities
pub struct ImageRecommendationUtils;

impl ImageRecommendationUtils {
    /// Get recommended settings for different use cases
    #[must_use]
    pub fn recommend_settings(
        use_case: &str,
    ) -> (String, ImageSize, Option<ImageQuality>, Option<ImageStyle>) {
        match use_case.to_lowercase().as_str() {
            "avatar" | "profile" => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1024x1024,
                Some(ImageQuality::Hd),
                Some(ImageStyle::Natural),
            ),
            "wallpaper" | "background" => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1792x1024,
                Some(ImageQuality::Hd),
                Some(ImageStyle::Vivid),
            ),
            "poster" | "vertical" => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1024x1792,
                Some(ImageQuality::Hd),
                Some(ImageStyle::Vivid),
            ),
            "thumbnail" | "icon" => (
                ImageModels::DALL_E_2.to_string(),
                ImageSize::Size256x256,
                None,
                None,
            ),
            "concept" | "draft" => (
                ImageModels::DALL_E_2.to_string(),
                ImageSize::Size512x512,
                None,
                None,
            ),
            _ => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1024x1024,
                Some(ImageQuality::Standard),
                Some(ImageStyle::Natural),
            ),
        }
    }

    /// Estimate cost for image generation
    /// Based on `OpenAI` pricing (as of 2024)
    #[must_use]
    pub fn estimate_cost(
        model: &str,
        size: &ImageSize,
        quality: Option<&ImageQuality>,
        count: u32,
    ) -> f64 {
        let base_cost = match model {
            "dall-e-3" => {
                match size {
                    ImageSize::Size1024x1024 => {
                        match quality {
                            Some(ImageQuality::Hd) => 0.080, // $0.080 per image
                            _ => 0.040,                      // $0.040 per image (standard)
                        }
                    }
                    ImageSize::Size1792x1024 | ImageSize::Size1024x1792 => {
                        match quality {
                            Some(ImageQuality::Hd) => 0.120, // $0.120 per image
                            _ => 0.080,                      // $0.080 per image (standard)
                        }
                    }
                    _ => 0.040, // Default to standard 1024x1024
                }
            }
            "dall-e-2" => {
                match size {
                    ImageSize::Size1024x1024 => 0.020, // $0.020 per image
                    ImageSize::Size512x512 => 0.018,   // $0.018 per image
                    ImageSize::Size256x256 => 0.016,   // $0.016 per image
                    _ => 0.020,                        // Default to 1024x1024
                }
            }
            _ => 0.040, // Default to DALL-E 3 standard pricing
        };

        base_cost * f64::from(count)
    }
}
