//! Tests for edge cases and boundary conditions

use openai_rust_sdk::models::responses::{Message, ResponseInput, ResponseRequest};

use super::test_helpers::create_test_function_tool;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input_text() {
        let request = ResponseRequest::new_text("gpt-4", "");

        match request.input {
            ResponseInput::Text(text) => assert!(text.is_empty()),
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_very_long_input() {
        let long_input = "a".repeat(10000);
        let request = ResponseRequest::new_text("gpt-4", long_input.clone());

        match request.input {
            ResponseInput::Text(text) => assert_eq!(text.len(), 10000),
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_unicode_input() {
        let unicode_input = "Hello 世界 🌍 👋";
        let request = ResponseRequest::new_text("gpt-4", unicode_input);

        match request.input {
            ResponseInput::Text(text) => {
                assert!(text.contains("世界"));
                assert!(text.contains("🌍"));
            }
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_special_characters_in_instructions() {
        let special_instructions = "Use <tags>, \"quotes\", 'apostrophes', and [brackets]";
        let request =
            ResponseRequest::new_text("gpt-4", "Test").with_instructions(special_instructions);

        assert_eq!(request.instructions, Some(special_instructions.to_string()));
    }

    #[test]
    fn test_empty_messages_list() {
        let empty_messages: Vec<Message> = vec![];
        let request = ResponseRequest::new_messages("gpt-4", empty_messages);

        match request.input {
            ResponseInput::Messages(msgs) => assert!(msgs.is_empty()),
            _ => panic!("Expected Messages input"),
        }
    }

    #[test]
    fn test_multiple_function_tools() {
        let tools = vec![
            create_test_function_tool(),
            create_test_function_tool(), // Duplicate for testing
        ];

        let request = ResponseRequest {
            model: "gpt-4".to_string(),
            input: ResponseInput::Text("Call functions".to_string()),
            tools: Some(tools),
            ..Default::default()
        };

        assert!(request.tools.is_some());
        assert_eq!(request.tools.unwrap().len(), 2);
    }
}
