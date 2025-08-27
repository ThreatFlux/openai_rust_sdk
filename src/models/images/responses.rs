//! # Image Response Types
//!
//! Response structures for image generation endpoints including image data
//! and response utilities.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Response from image generation endpoints
#[derive(Debug, Clone, Ser, De)]
pub struct ImageResponse {
    /// The Unix timestamp when the images were created
    pub created: u64,

    /// The generated images
    pub data: Vec<ImageData>,
}

/// Individual image data in the response
#[derive(Debug, Clone, Ser, De)]
pub struct ImageData {
    /// The URL of the generated image (if `response_format` is url)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The base64-encoded image data (if `response_format` is `b64_json`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,

    /// The revised prompt used for generation (DALL-E 3 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revised_prompt: Option<String>,
}

impl ImageData {
    /// Check if this image data contains a URL
    #[must_use]
    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    /// Check if this image data contains base64 data
    #[must_use]
    pub fn has_b64_json(&self) -> bool {
        self.b64_json.is_some()
    }

    /// Get the image URL if available
    #[must_use]
    pub fn get_url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// Get the base64 image data if available
    #[must_use]
    pub fn get_b64_json(&self) -> Option<&str> {
        self.b64_json.as_deref()
    }

    /// Get the revised prompt if available (DALL-E 3 only)
    #[must_use]
    pub fn get_revised_prompt(&self) -> Option<&str> {
        self.revised_prompt.as_deref()
    }

    /// Decode base64 image data to bytes
    pub fn decode_b64_json(&self) -> Result<Vec<u8>, String> {
        match &self.b64_json {
            Some(b64_data) => {
                use base64::{engine::general_purpose, Engine as _};
                general_purpose::STANDARD
                    .decode(b64_data)
                    .map_err(|e| format!("Failed to decode base64: {e}"))
            }
            None => Err("No base64 data available".to_string()),
        }
    }

    /// Save base64 image data to file
    pub async fn save_b64_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), String> {
        let data = self.decode_b64_json()?;
        tokio::fs::write(path, data)
            .await
            .map_err(|e| format!("Failed to save file: {e}"))
    }
}

impl ImageResponse {
    /// Get the number of generated images
    #[must_use]
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Check if any images have URLs
    #[must_use]
    pub fn has_urls(&self) -> bool {
        self.data.iter().any(ImageData::has_url)
    }

    /// Check if any images have base64 data
    #[must_use]
    pub fn has_b64_json(&self) -> bool {
        self.data.iter().any(ImageData::has_b64_json)
    }

    /// Get all image URLs
    #[must_use]
    pub fn get_urls(&self) -> Vec<&str> {
        self.data.iter().filter_map(|img| img.get_url()).collect()
    }

    /// Get the first image data
    #[must_use]
    pub fn first_image(&self) -> Option<&ImageData> {
        self.data.first()
    }

    /// Get all revised prompts (DALL-E 3 only)
    #[must_use]
    pub fn get_revised_prompts(&self) -> Vec<&str> {
        self.data
            .iter()
            .filter_map(|img| img.get_revised_prompt())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_data_methods() {
        let image_data = ImageData {
            url: Some("https://example.com/image.png".to_string()),
            b64_json: None,
            revised_prompt: Some("A revised prompt".to_string()),
        };

        assert!(image_data.has_url());
        assert!(!image_data.has_b64_json());
        assert_eq!(image_data.get_url(), Some("https://example.com/image.png"));
        assert_eq!(image_data.get_revised_prompt(), Some("A revised prompt"));
    }

    #[test]
    fn test_image_response_methods() {
        let response = ImageResponse {
            created: 1_234_567_890,
            data: vec![
                ImageData {
                    url: Some("https://example.com/image1.png".to_string()),
                    b64_json: None,
                    revised_prompt: None,
                },
                ImageData {
                    url: Some("https://example.com/image2.png".to_string()),
                    b64_json: None,
                    revised_prompt: None,
                },
            ],
        };

        assert_eq!(response.count(), 2);
        assert!(response.has_urls());
        assert!(!response.has_b64_json());
        assert_eq!(response.get_urls().len(), 2);
        assert!(response.first_image().is_some());
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "created": 1234567890,
            "data": [
                {
                    "url": "https://example.com/image.png",
                    "revised_prompt": "A revised prompt"
                }
            ]
        }"#;

        let response: ImageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.created, 1_234_567_890);
        assert_eq!(response.data.len(), 1);
        assert_eq!(
            response.data[0].url,
            Some("https://example.com/image.png".to_string())
        );
        assert_eq!(
            response.data[0].revised_prompt,
            Some("A revised prompt".to_string())
        );
    }
}
