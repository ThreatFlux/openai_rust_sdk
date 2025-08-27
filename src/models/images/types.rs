//! # Image Types
//!
//! Type definitions for image quality, formats, sizes, styles, and model constants.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Image generation quality levels
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    /// Standard quality (faster and cheaper)
    Standard,
    /// High definition quality (more detailed but slower and more expensive)
    Hd,
}

/// Image response formats
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    /// Return image URLs that expire after 1 hour
    Url,
    /// Return base64-encoded image data
    B64Json,
}

/// Image sizes supported by DALL-E
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
pub enum ImageSize {
    /// 256x256 pixels (dall-e-2 only)
    #[serde(rename = "256x256")]
    Size256x256,
    /// 512x512 pixels (dall-e-2 only)
    #[serde(rename = "512x512")]
    Size512x512,
    /// 1024x1024 pixels (dall-e-2 and dall-e-3)
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    /// 1792x1024 pixels (dall-e-3 only)
    #[serde(rename = "1792x1024")]
    Size1792x1024,
    /// 1024x1792 pixels (dall-e-3 only)
    #[serde(rename = "1024x1792")]
    Size1024x1792,
}

/// Image generation styles
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    /// More vivid and dramatic images
    Vivid,
    /// More natural and less hyper-real images
    Natural,
}

/// Common image models
pub struct ImageModels;

impl ImageModels {
    /// DALL-E 3 model (latest, highest quality)
    pub const DALL_E_3: &'static str = "dall-e-3";

    /// DALL-E 2 model (faster, supports more features like editing and variations)
    pub const DALL_E_2: &'static str = "dall-e-2";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_size_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageSize::Size256x256).unwrap(),
            "\"256x256\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size512x512).unwrap(),
            "\"512x512\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size1024x1024).unwrap(),
            "\"1024x1024\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size1792x1024).unwrap(),
            "\"1792x1024\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size1024x1792).unwrap(),
            "\"1024x1792\""
        );
    }

    #[test]
    fn test_image_quality_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageQuality::Standard).unwrap(),
            "\"standard\""
        );
        assert_eq!(serde_json::to_string(&ImageQuality::Hd).unwrap(), "\"hd\"");
    }

    #[test]
    fn test_image_style_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageStyle::Vivid).unwrap(),
            "\"vivid\""
        );
        assert_eq!(
            serde_json::to_string(&ImageStyle::Natural).unwrap(),
            "\"natural\""
        );
    }

    #[test]
    fn test_response_format_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageResponseFormat::Url).unwrap(),
            "\"url\""
        );
        assert_eq!(
            serde_json::to_string(&ImageResponseFormat::B64Json).unwrap(),
            "\"b64_json\""
        );
    }
}
