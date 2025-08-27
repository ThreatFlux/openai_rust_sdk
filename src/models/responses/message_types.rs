use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Role for message in conversation
#[derive(Debug, Clone, Ser, De, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// Developer message providing high-priority instructions
    Developer,
    /// User input message
    User,
    /// AI assistant response message
    Assistant,
    /// System message (legacy, use Developer for new code)
    System,
}

/// Detail level for image processing
#[derive(Debug, Clone, Ser, De, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ImageDetail {
    /// Auto-detect appropriate detail level
    #[default]
    Auto,
    /// Low detail for faster processing
    Low,
    /// High detail for better accuracy
    High,
}

/// Image URL specification
#[derive(Debug, Clone, Ser, De)]
pub struct ImageUrl {
    /// URL or base64-encoded image data
    pub url: String,
    /// Detail level for image processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Image content for multimodal messages
#[derive(Debug, Clone, Ser, De)]
pub struct ImageContent {
    /// Type identifier for image content
    #[serde(rename = "type")]
    pub content_type: String,
    /// Image URL or base64 data
    pub image_url: ImageUrl,
}

/// Text content for multimodal messages
#[derive(Debug, Clone, Ser, De)]
pub struct TextContent {
    /// Type identifier for text content
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content
    pub text: String,
}

/// Content types for multimodal messages
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type")]
pub enum MessageContent {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// The text content
        text: String,
    },
    /// Image content
    #[serde(rename = "image_url")]
    Image {
        /// The image URL details
        image_url: ImageUrl,
    },
}

/// Message content input - can be simple text or array of content items
#[derive(Debug, Clone, Ser, De)]
#[serde(untagged)]
pub enum MessageContentInput {
    /// Simple text content
    Text(String),
    /// Array of multimodal content items
    Array(Vec<MessageContent>),
}

/// Message in a conversation
#[derive(Debug, Clone, Ser, De)]
pub struct Message {
    /// The role of the message sender
    pub role: MessageRole,
    /// The content of the message (text or multimodal)
    pub content: MessageContentInput,
}

impl MessageContent {
    /// Create text content
    pub fn text(text: impl Into<String>) -> Self {
        MessageContent::Text { text: text.into() }
    }

    /// Create image content from URL
    pub fn image_url(url: impl Into<String>) -> Self {
        MessageContent::Image {
            image_url: ImageUrl {
                url: url.into(),
                detail: None,
            },
        }
    }

    /// Create image content from URL with detail level
    pub fn image_url_with_detail(url: impl Into<String>, detail: ImageDetail) -> Self {
        MessageContent::Image {
            image_url: ImageUrl {
                url: url.into(),
                detail: Some(detail),
            },
        }
    }

    /// Create image content from bytes and format
    #[must_use]
    pub fn image_from_bytes(
        image_data: &[u8],
        format: &crate::models::responses::ImageFormat,
    ) -> Self {
        use crate::models::responses::ImageUtils;
        let data_url = ImageUtils::encode_to_data_url(image_data, format);
        MessageContent::Image {
            image_url: ImageUrl {
                url: data_url,
                detail: None,
            },
        }
    }

    /// Create image content from bytes, format, and detail level
    #[must_use]
    pub fn image_from_bytes_with_detail(
        image_data: &[u8],
        format: &crate::models::responses::ImageFormat,
        detail: ImageDetail,
    ) -> Self {
        use crate::models::responses::ImageUtils;
        let data_url = ImageUtils::encode_to_data_url(image_data, format);
        MessageContent::Image {
            image_url: ImageUrl {
                url: data_url,
                detail: Some(detail),
            },
        }
    }
}

impl Message {
    /// Create a new user message with text content
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a new assistant message with text content
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a new developer message (high-priority instructions)
    pub fn developer(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Developer,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a new system message (legacy, use developer for new code)
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a user message with multimodal content
    #[must_use]
    pub fn user_with_content(content: Vec<MessageContent>) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(content),
        }
    }

    /// Create an assistant message with multimodal content
    #[must_use]
    pub fn assistant_with_content(content: Vec<MessageContent>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: MessageContentInput::Array(content),
        }
    }

    /// Create a user message with text and image
    pub fn user_with_image(text: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(vec![
                MessageContent::text(text),
                MessageContent::image_url(image_url),
            ]),
        }
    }

    /// Create a user message with text and image with detail level
    pub fn user_with_image_detail(
        text: impl Into<String>,
        image_url: impl Into<String>,
        detail: ImageDetail,
    ) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(vec![
                MessageContent::text(text),
                MessageContent::image_url_with_detail(image_url, detail),
            ]),
        }
    }

    /// Create a user message with text and multiple images
    pub fn user_with_images(text: impl Into<String>, image_urls: Vec<String>) -> Self {
        let mut content = vec![MessageContent::text(text)];
        for url in image_urls {
            content.push(MessageContent::image_url(url));
        }
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(content),
        }
    }

    /// Create a user message from image bytes
    pub fn user_with_image_bytes(
        text: impl Into<String>,
        image_data: &[u8],
        format: &crate::models::responses::ImageFormat,
    ) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(vec![
                MessageContent::text(text),
                MessageContent::image_from_bytes(image_data, format),
            ]),
        }
    }

    /// Get estimated token count including images
    #[must_use]
    pub fn estimate_tokens(&self) -> u32 {
        use crate::models::responses::ImageUtils;
        match &self.content {
            MessageContentInput::Text(text) => {
                // Rough estimate: 1 token per 4 characters
                (text.len() as f32 / 4.0).ceil() as u32
            }
            MessageContentInput::Array(contents) => {
                let mut total = 0;
                for content in contents {
                    match content {
                        MessageContent::Text { text } => {
                            total += (text.len() as f32 / 4.0).ceil() as u32;
                        }
                        MessageContent::Image { image_url } => {
                            let detail = image_url.detail.as_ref().unwrap_or(&ImageDetail::Auto);
                            total += ImageUtils::estimate_tokens(detail);
                        }
                    }
                }
                total
            }
        }
    }

    /// Check if message contains images
    #[must_use]
    pub fn has_images(&self) -> bool {
        match &self.content {
            MessageContentInput::Text(_) => false,
            MessageContentInput::Array(contents) => contents
                .iter()
                .any(|c| matches!(c, MessageContent::Image { .. })),
        }
    }

    /// Get text content only (concatenated if multimodal)
    #[must_use]
    pub fn text_content(&self) -> String {
        match &self.content {
            MessageContentInput::Text(text) => text.clone(),
            MessageContentInput::Array(contents) => contents
                .iter()
                .filter_map(|c| match c {
                    MessageContent::Text { text } => Some(text.as_str()),
                    MessageContent::Image { .. } => None,
                })
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    /// Get image URLs from the message
    #[must_use]
    pub fn image_urls(&self) -> Vec<&str> {
        match &self.content {
            MessageContentInput::Text(_) => vec![],
            MessageContentInput::Array(contents) => contents
                .iter()
                .filter_map(|c| match c {
                    MessageContent::Text { .. } => None,
                    MessageContent::Image { image_url } => Some(image_url.url.as_str()),
                })
                .collect(),
        }
    }
}
