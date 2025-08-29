//! Builder for image generation tool configurations

use super::{EnhancedTool, ImageGenerationConfig};

/// Builder for image generation tools
pub struct ImageGenerationToolBuilder {
    /// The image generation configuration being built
    config: ImageGenerationConfig,
}

impl Default for ImageGenerationToolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageGenerationToolBuilder {
    /// Create a new ImageGenerationToolBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ImageGenerationConfig {
                size: None,
                quality: None,
                style: None,
                n: None,
            },
        }
    }

    /// Set the image size (e.g., "1024x1024", "1792x1024")
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.config.size = Some(size.into());
        self
    }

    /// Set the image quality (e.g., "standard", "hd")
    pub fn quality(mut self, quality: impl Into<String>) -> Self {
        self.config.quality = Some(quality.into());
        self
    }

    /// Set the image style (e.g., "vivid", "natural")
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.config.style = Some(style.into());
        self
    }

    /// Set the number of images to generate
    #[must_use]
    pub fn count(mut self, n: u32) -> Self {
        self.config.n = Some(n);
        self
    }

    /// Build the configured image generation tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::ImageGeneration(self.config)
    }
}
