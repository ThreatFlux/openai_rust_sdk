//! # Image Request Types
//!
//! Request structures for the OpenAI Images API including image generation,
//! editing, and variation endpoints.

use super::types::{ImageQuality, ImageResponseFormat, ImageSize, ImageStyle};
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Request for generating images from text prompts
#[derive(Debug, Clone, Ser, De)]
pub struct ImageGenerationRequest {
    /// The model to use for image generation (e.g., "dall-e-3", "dall-e-2")
    pub model: String,

    /// A text description of the desired image(s)
    pub prompt: String,

    /// The number of images to generate (1-10 for dall-e-2, only 1 for dall-e-3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// The quality of the image (standard or hd, dall-e-3 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,

    /// The format in which the generated images are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// The style of the generated images (vivid or natural, dall-e-3 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Request for editing images with masks
#[derive(Debug, Clone, Ser, De)]
pub struct ImageEditRequest {
    /// The model to use for image editing (e.g., "dall-e-2")
    pub model: String,

    /// The image to edit (PNG only, up to 4MB, must be square)
    #[serde(skip_serializing)]
    pub image: String,

    /// An additional image whose fully transparent areas indicate where image should be edited (PNG only, same size as image)
    #[serde(skip_serializing)]
    pub mask: Option<String>,

    /// A text description of the desired image(s)
    pub prompt: String,

    /// The number of images to generate (1-10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// The format in which the generated images are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Request for creating variations of images
#[derive(Debug, Clone, Ser, De)]
pub struct ImageVariationRequest {
    /// The model to use for creating variations (e.g., "dall-e-2")
    pub model: String,

    /// The image to create variations of (PNG only, up to 4MB, must be square)
    #[serde(skip_serializing)]
    pub image: String,

    /// The number of images to generate (1-10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// The format in which the generated images are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::images::types::{ImageQuality, ImageResponseFormat, ImageSize, ImageStyle};

    #[test]
    fn test_image_generation_request_creation() {
        let req = ImageGenerationRequest {
            model: "dall-e-3".to_string(),
            prompt: "A beautiful sunset".to_string(),
            n: None,
            quality: None,
            response_format: None,
            size: None,
            style: None,
            user: None,
        };
        assert_eq!(req.model, "dall-e-3");
        assert_eq!(req.prompt, "A beautiful sunset");
        assert_eq!(req.n, None);
    }

    #[test]
    fn test_image_edit_request_creation() {
        let req = ImageEditRequest {
            model: "dall-e-2".to_string(),
            image: "image.png".to_string(),
            mask: None,
            prompt: "Add a cat".to_string(),
            n: None,
            response_format: None,
            size: None,
            user: None,
        };
        assert_eq!(req.model, "dall-e-2");
        assert_eq!(req.image, "image.png");
        assert_eq!(req.prompt, "Add a cat");
        assert_eq!(req.mask, None);
    }

    #[test]
    fn test_image_variation_request_creation() {
        let req = ImageVariationRequest {
            model: "dall-e-2".to_string(),
            image: "image.png".to_string(),
            n: None,
            response_format: None,
            size: None,
            user: None,
        };
        assert_eq!(req.model, "dall-e-2");
        assert_eq!(req.image, "image.png");
        assert_eq!(req.n, None);
    }

    #[test]
    fn test_request_serialization() {
        let req = ImageGenerationRequest {
            model: "dall-e-3".to_string(),
            prompt: "A beautiful image".to_string(),
            n: None,
            quality: Some(ImageQuality::Hd),
            response_format: Some(ImageResponseFormat::Url),
            size: Some(ImageSize::Size1024x1024),
            style: Some(ImageStyle::Natural),
            user: None,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"model\":\"dall-e-3\""));
        assert!(json.contains("\"prompt\":\"A beautiful image\""));
        assert!(json.contains("\"quality\":\"hd\""));
        assert!(json.contains("\"size\":\"1024x1024\""));
        assert!(json.contains("\"style\":\"natural\""));
        assert!(json.contains("\"response_format\":\"url\""));
    }
}
