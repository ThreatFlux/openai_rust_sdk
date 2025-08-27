//! Tests for message creation and handling in streaming

use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello world".to_string()),
        };

        assert_eq!(message.role, MessageRole::User);
        // Verify content by matching the enum variant
        match message.content {
            MessageContentInput::Text(ref text) => assert_eq!(text, "Hello world"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_message_roles() {
        let roles = vec![
            MessageRole::Developer,
            MessageRole::User,
            MessageRole::Assistant,
            MessageRole::System,
        ];

        for role in roles {
            let message = Message {
                role: role.clone(),
                content: MessageContentInput::Text("Test message".to_string()),
            };

            // Each role should be valid
            let json = serde_json::to_string(&message);
            assert!(json.is_ok());
        }
    }

    #[test]
    fn test_message_serialization() {
        let message = Message {
            role: MessageRole::Assistant,
            content: MessageContentInput::Text("Hello! How can I help you?".to_string()),
        };

        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("assistant"));
        assert!(json.contains("Hello! How can I help you?"));

        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.role, message.role);
        // Compare content by extracting text from both
        match (&deserialized.content, &message.content) {
            (MessageContentInput::Text(t1), MessageContentInput::Text(t2)) => assert_eq!(t1, t2),
            _ => panic!("Content type mismatch"),
        }
    }

    #[test]
    fn test_messages_in_streaming_request() {
        let messages = vec![
            Message {
                role: MessageRole::User,
                content: MessageContentInput::Text("What's the weather like?".to_string()),
            },
            Message {
                role: MessageRole::Assistant,
                content: MessageContentInput::Text(
                    "I'd be happy to help you check the weather.".to_string(),
                ),
            },
        ];

        let request = ResponseRequest::new_messages("gpt-4", messages).with_streaming(true);

        match request.input {
            ResponseInput::Messages(msgs) => {
                assert_eq!(msgs.len(), 2);
                assert_eq!(msgs[0].role, MessageRole::User);
                assert_eq!(msgs[1].role, MessageRole::Assistant);
            }
            _ => panic!("Expected Messages input"),
        }
    }
}
