//! Image generation tool configuration and types

use crate::{De, Ser};

/// Image generation configuration
#[derive(Debug, Clone, Ser, De)]
pub struct ImageGenerationConfig {
    /// Image size (e.g., "1024x1024", "512x512")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Image quality ("standard" or "hd")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,

    /// Image style ("vivid" or "natural")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// Number of images to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}
