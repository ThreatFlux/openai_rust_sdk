//! Conversation data structures for real-time audio
//!
//! Contains types for conversation items, content parts, roles,
//! and related data structures used in real-time audio conversations.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Conversation item
#[derive(Debug, Clone, Ser, De)]
pub struct ConversationItem {
    /// Unique item identifier
    pub id: Option<String>,

    /// Object type
    pub object: String,

    /// Item type
    #[serde(rename = "type")]
    pub item_type: ConversationItemType,

    /// Item status
    pub status: ConversationItemStatus,

    /// Item role
    pub role: ConversationRole,

    /// Item content
    pub content: Vec<ContentPart>,
}

/// Types of conversation items
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConversationItemType {
    /// Message item
    Message,
    /// Function call item
    FunctionCall,
    /// Function call output item
    FunctionCallOutput,
}

/// Status of conversation items
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConversationItemStatus {
    /// Item is completed
    Completed,
    /// Item is in progress
    InProgress,
    /// Item is incomplete
    Incomplete,
}

/// Roles in conversation
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConversationRole {
    /// User role
    User,
    /// Assistant role
    Assistant,
    /// System role
    System,
}

/// Content parts of conversation items
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type")]
pub enum ContentPart {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// Text content
        text: String,
    },

    /// Audio content
    #[serde(rename = "audio")]
    Audio {
        /// Base64-encoded audio data
        audio: Option<String>,
        /// Audio transcript
        transcript: Option<String>,
    },

    /// Input audio content
    #[serde(rename = "input_audio")]
    InputAudio {
        /// Base64-encoded audio data
        audio: Option<String>,
        /// Audio transcript
        transcript: Option<String>,
    },

    /// Function call content
    #[serde(rename = "function_call")]
    FunctionCall {
        /// Call ID
        call_id: String,
        /// Function name
        name: String,
        /// Function arguments (JSON string)
        arguments: String,
    },

    /// Function call output content
    #[serde(rename = "function_call_output")]
    FunctionCallOutput {
        /// Call ID
        call_id: String,
        /// Function output
        output: String,
    },
}

impl ConversationItem {
    /// Create a new text message
    #[must_use]
    pub fn new_text_message(role: ConversationRole, text: String) -> Self {
        Self {
            id: None,
            object: "realtime.item".to_string(),
            item_type: ConversationItemType::Message,
            status: ConversationItemStatus::Completed,
            role,
            content: vec![ContentPart::Text { text }],
        }
    }

    /// Create a new audio message
    #[must_use]
    pub fn new_audio_message(
        role: ConversationRole,
        audio: Option<String>,
        transcript: Option<String>,
    ) -> Self {
        Self {
            id: None,
            object: "realtime.item".to_string(),
            item_type: ConversationItemType::Message,
            status: ConversationItemStatus::Completed,
            role,
            content: vec![ContentPart::Audio { audio, transcript }],
        }
    }

    /// Create a new function call
    #[must_use]
    pub fn new_function_call(call_id: String, name: String, arguments: String) -> Self {
        Self {
            id: None,
            object: "realtime.item".to_string(),
            item_type: ConversationItemType::FunctionCall,
            status: ConversationItemStatus::Completed,
            role: ConversationRole::Assistant,
            content: vec![ContentPart::FunctionCall {
                call_id,
                name,
                arguments,
            }],
        }
    }

    /// Create a new function call output
    #[must_use]
    pub fn new_function_call_output(call_id: String, output: String) -> Self {
        Self {
            id: None,
            object: "realtime.item".to_string(),
            item_type: ConversationItemType::FunctionCallOutput,
            status: ConversationItemStatus::Completed,
            role: ConversationRole::User,
            content: vec![ContentPart::FunctionCallOutput { call_id, output }],
        }
    }

    /// Get the text content if this is a text message
    #[must_use]
    pub fn get_text_content(&self) -> Option<&str> {
        self.content.iter().find_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
    }

    /// Get the audio transcript if this is an audio message
    #[must_use]
    pub fn get_audio_transcript(&self) -> Option<&str> {
        self.content.iter().find_map(|part| match part {
            ContentPart::Audio { transcript, .. } => transcript.as_deref(),
            ContentPart::InputAudio { transcript, .. } => transcript.as_deref(),
            _ => None,
        })
    }

    /// Check if this item contains a function call
    #[must_use]
    pub fn has_function_call(&self) -> bool {
        self.content
            .iter()
            .any(|part| matches!(part, ContentPart::FunctionCall { .. }))
    }
}

impl ContentPart {
    /// Get the content type as a string
    #[must_use]
    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Text { .. } => "text",
            Self::Audio { .. } => "audio",
            Self::InputAudio { .. } => "input_audio",
            Self::FunctionCall { .. } => "function_call",
            Self::FunctionCallOutput { .. } => "function_call_output",
        }
    }

    /// Check if this is a text content part
    #[must_use]
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    /// Check if this is an audio content part
    #[must_use]
    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio { .. } | Self::InputAudio { .. })
    }

    /// Check if this is a function call content part
    #[must_use]
    pub fn is_function_call(&self) -> bool {
        matches!(
            self,
            Self::FunctionCall { .. } | Self::FunctionCallOutput { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_item_creation() {
        let item = ConversationItem {
            id: Some("item-123".to_string()),
            object: "realtime.item".to_string(),
            item_type: ConversationItemType::Message,
            status: ConversationItemStatus::Completed,
            role: ConversationRole::User,
            content: vec![ContentPart::Text {
                text: "Hello, world!".to_string(),
            }],
        };

        assert_eq!(item.id, Some("item-123".to_string()));
        assert_eq!(item.role, ConversationRole::User);
        assert_eq!(item.content.len(), 1);
    }

    #[test]
    fn test_new_text_message() {
        let item =
            ConversationItem::new_text_message(ConversationRole::User, "Hello, world!".to_string());

        assert_eq!(item.role, ConversationRole::User);
        assert_eq!(item.item_type, ConversationItemType::Message);
        assert_eq!(item.get_text_content(), Some("Hello, world!"));
    }

    #[test]
    fn test_new_audio_message() {
        let item = ConversationItem::new_audio_message(
            ConversationRole::Assistant,
            Some("base64-audio".to_string()),
            Some("Hello there!".to_string()),
        );

        assert_eq!(item.role, ConversationRole::Assistant);
        assert_eq!(item.item_type, ConversationItemType::Message);
        assert_eq!(item.get_audio_transcript(), Some("Hello there!"));
    }

    #[test]
    fn test_new_function_call() {
        let item = ConversationItem::new_function_call(
            "call-123".to_string(),
            "get_weather".to_string(),
            r#"{"location": "New York"}"#.to_string(),
        );

        assert_eq!(item.role, ConversationRole::Assistant);
        assert_eq!(item.item_type, ConversationItemType::FunctionCall);
        assert!(item.has_function_call());
    }

    #[test]
    fn test_new_function_call_output() {
        let item = ConversationItem::new_function_call_output(
            "call-123".to_string(),
            "Sunny, 75°F".to_string(),
        );

        assert_eq!(item.role, ConversationRole::User);
        assert_eq!(item.item_type, ConversationItemType::FunctionCallOutput);
    }

    #[test]
    fn test_content_part_type_detection() {
        let text_part = ContentPart::Text {
            text: "Hello".to_string(),
        };
        let audio_part = ContentPart::Audio {
            audio: None,
            transcript: None,
        };
        let function_part = ContentPart::FunctionCall {
            call_id: "123".to_string(),
            name: "test".to_string(),
            arguments: "{}".to_string(),
        };

        assert!(text_part.is_text());
        assert!(!text_part.is_audio());
        assert!(!text_part.is_function_call());

        assert!(audio_part.is_audio());
        assert!(!audio_part.is_text());
        assert!(!audio_part.is_function_call());

        assert!(function_part.is_function_call());
        assert!(!function_part.is_text());
        assert!(!function_part.is_audio());
    }

    #[test]
    fn test_content_part_type_strings() {
        let text_part = ContentPart::Text {
            text: "Hello".to_string(),
        };
        let audio_part = ContentPart::Audio {
            audio: None,
            transcript: None,
        };
        let input_audio_part = ContentPart::InputAudio {
            audio: None,
            transcript: None,
        };
        let function_part = ContentPart::FunctionCall {
            call_id: "123".to_string(),
            name: "test".to_string(),
            arguments: "{}".to_string(),
        };
        let function_output_part = ContentPart::FunctionCallOutput {
            call_id: "123".to_string(),
            output: "result".to_string(),
        };

        assert_eq!(text_part.content_type(), "text");
        assert_eq!(audio_part.content_type(), "audio");
        assert_eq!(input_audio_part.content_type(), "input_audio");
        assert_eq!(function_part.content_type(), "function_call");
        assert_eq!(function_output_part.content_type(), "function_call_output");
    }

    #[test]
    fn test_conversation_role_serialization() {
        assert_eq!(
            serde_json::to_string(&ConversationRole::User).unwrap(),
            "\"user\""
        );
        assert_eq!(
            serde_json::to_string(&ConversationRole::Assistant).unwrap(),
            "\"assistant\""
        );
        assert_eq!(
            serde_json::to_string(&ConversationRole::System).unwrap(),
            "\"system\""
        );
    }

    #[test]
    fn test_conversation_item_status_serialization() {
        assert_eq!(
            serde_json::to_string(&ConversationItemStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&ConversationItemStatus::InProgress).unwrap(),
            "\"inprogress\""
        );
        assert_eq!(
            serde_json::to_string(&ConversationItemStatus::Incomplete).unwrap(),
            "\"incomplete\""
        );
    }

    #[test]
    fn test_conversation_item_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ConversationItemType::Message).unwrap(),
            "\"message\""
        );
        assert_eq!(
            serde_json::to_string(&ConversationItemType::FunctionCall).unwrap(),
            "\"functioncall\""
        );
        assert_eq!(
            serde_json::to_string(&ConversationItemType::FunctionCallOutput).unwrap(),
            "\"functioncalloutput\""
        );
    }
}
