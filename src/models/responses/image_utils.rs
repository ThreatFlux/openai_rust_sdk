use super::message_types::ImageDetail;
use base64::{engine::general_purpose, Engine as _};

/// Supported image formats for validation
#[derive(Debug, Clone, PartialEq)]
pub enum ImageFormat {
    /// JPEG image format
    Jpeg,
    /// PNG image format
    Png,
    /// GIF image format
    Gif,
    /// WebP image format
    Webp,
}

impl ImageFormat {
    /// Detect image format from data URL or file extension
    #[must_use]
    pub fn from_data_url(data_url: &str) -> Option<Self> {
        if data_url.starts_with("data:image/jpeg") || data_url.starts_with("data:image/jpg") {
            Some(ImageFormat::Jpeg)
        } else if data_url.starts_with("data:image/png") {
            Some(ImageFormat::Png)
        } else if data_url.starts_with("data:image/gif") {
            Some(ImageFormat::Gif)
        } else if data_url.starts_with("data:image/webp") {
            Some(ImageFormat::Webp)
        } else {
            None
        }
    }

    /// Detect image format from URL extension
    #[must_use]
    pub fn from_url(url: &str) -> Option<Self> {
        let url_lower = url.to_lowercase();
        if url_lower.ends_with(".jpg") || url_lower.ends_with(".jpeg") {
            Some(ImageFormat::Jpeg)
        } else if url_lower.ends_with(".png") {
            Some(ImageFormat::Png)
        } else if url_lower.ends_with(".gif") {
            Some(ImageFormat::Gif)
        } else if url_lower.ends_with(".webp") {
            Some(ImageFormat::Webp)
        } else {
            None
        }
    }

    /// Get MIME type for the format
    #[must_use]
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Png => "image/png",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Webp => "image/webp",
        }
    }
}

/// Image utilities for encoding and validation
pub struct ImageUtils;

impl ImageUtils {
    /// Encode image bytes to base64 data URL
    #[must_use]
    pub fn encode_to_data_url(image_data: &[u8], format: &ImageFormat) -> String {
        let base64_data = general_purpose::STANDARD.encode(image_data);
        format!("data:{};base64,{}", format.mime_type(), base64_data)
    }

    /// Decode base64 data URL to image bytes
    pub fn decode_from_data_url(data_url: &str) -> Result<Vec<u8>, String> {
        if !data_url.starts_with("data:image/") {
            return Err("Invalid data URL format".to_string());
        }

        let parts: Vec<&str> = data_url.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid data URL structure".to_string());
        }

        general_purpose::STANDARD
            .decode(parts[1])
            .map_err(|e| format!("Base64 decode error: {e}"))
    }

    /// Validate image format from URL or data URL
    pub fn validate_format(url: &str) -> Result<ImageFormat, String> {
        if url.starts_with("data:image/") {
            ImageFormat::from_data_url(url)
                .ok_or_else(|| "Unsupported image format in data URL".to_string())
        } else {
            ImageFormat::from_url(url).ok_or_else(|| "Unsupported image format in URL".to_string())
        }
    }

    /// Estimate token usage for image based on detail level
    #[must_use]
    pub fn estimate_tokens(detail: &ImageDetail) -> u32 {
        match detail {
            ImageDetail::Low => 85,   // Low detail uses 85 tokens
            ImageDetail::High => 170, // High detail can use up to 170 tokens per 512x512 tile
            ImageDetail::Auto => 85,  // Default to low estimate
        }
    }
}
