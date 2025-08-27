//! Message content and annotation types

use crate::{De, Ser};
use serde::{Deserialize, Serialize};

/// Content within a message
#[derive(Debug, Clone, PartialEq, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    /// Text content with optional annotations
    Text {
        /// The text content
        text: TextContent,
    },
    /// Image file content
    ImageFile {
        /// The image file details
        image_file: ImageFile,
    },
}

impl MessageContent {
    /// Create text content
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text {
            text: TextContent {
                value: value.into(),
                annotations: Vec::new(),
            },
        }
    }

    /// Create text content with annotations
    pub fn text_with_annotations(value: impl Into<String>, annotations: Vec<Annotation>) -> Self {
        Self::Text {
            text: TextContent {
                value: value.into(),
                annotations,
            },
        }
    }

    /// Create image file content
    pub fn image_file(file_id: impl Into<String>) -> Self {
        Self::ImageFile {
            image_file: ImageFile {
                file_id: file_id.into(),
            },
        }
    }
}

/// Text content with annotations
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct TextContent {
    /// The actual text content
    pub value: String,
    /// Annotations for the text content
    #[serde(default)]
    pub annotations: Vec<Annotation>,
}

/// Image file content
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct ImageFile {
    /// The file ID of the image
    pub file_id: String,
}

/// Annotations that can be applied to text content
#[derive(Debug, Clone, PartialEq, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    /// A citation of a specific quote from a file
    FileCitation {
        /// The text that was annotated
        text: String,
        /// The start index of the annotation
        start_index: u32,
        /// The end index of the annotation
        end_index: u32,
        /// The file citation details
        file_citation: FileCitation,
    },
    /// A file path annotation
    FilePath {
        /// The text that was annotated
        text: String,
        /// The start index of the annotation
        start_index: u32,
        /// The end index of the annotation
        end_index: u32,
        /// The file path details
        file_path: FilePathInfo,
    },
}

/// A citation of a specific quote from a file
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct FileCitation {
    /// The ID of the file that was cited
    pub file_id: String,
    /// The specific quote from the file
    pub quote: Option<String>,
}

/// File path information
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct FilePathInfo {
    /// The ID of the file
    pub file_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_content() {
        let text_content = MessageContent::text("Hello, world!");
        match text_content {
            MessageContent::Text { text } => {
                assert_eq!(text.value, "Hello, world!");
                assert!(text.annotations.is_empty());
            }
            _ => panic!("Expected text content"),
        }

        let image_content = MessageContent::image_file("file-123");
        match image_content {
            MessageContent::ImageFile { image_file } => {
                assert_eq!(image_file.file_id, "file-123");
            }
            _ => panic!("Expected image file content"),
        }
    }

    #[test]
    fn test_annotation_types() {
        let file_citation = Annotation::FileCitation {
            text: "cited text".to_string(),
            start_index: 0,
            end_index: 10,
            file_citation: FileCitation {
                file_id: "file-123".to_string(),
                quote: Some("original quote".to_string()),
            },
        };

        match file_citation {
            Annotation::FileCitation {
                text,
                file_citation,
                ..
            } => {
                assert_eq!(text, "cited text");
                assert_eq!(file_citation.file_id, "file-123");
                assert_eq!(file_citation.quote, Some("original quote".to_string()));
            }
            _ => panic!("Expected file citation annotation"),
        }
    }

    #[test]
    fn test_text_content_with_annotations() {
        let annotations = vec![Annotation::FileCitation {
            text: "test".to_string(),
            start_index: 0,
            end_index: 4,
            file_citation: FileCitation {
                file_id: "file-123".to_string(),
                quote: None,
            },
        }];

        let content = MessageContent::text_with_annotations("test content", annotations);
        match content {
            MessageContent::Text { text } => {
                assert_eq!(text.value, "test content");
                assert_eq!(text.annotations.len(), 1);
            }
            _ => panic!("Expected text content"),
        }
    }
}
