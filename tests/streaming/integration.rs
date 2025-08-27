//! Tests for integration test preparation

use openai_rust_sdk::models::functions::ToolChoice;
use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest,
};

use super::test_helpers::create_test_function_tool;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_streaming_request_setup() {
        let tools = vec![create_test_function_tool()];
        let messages = vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Text(
                "Calculate 15 + 27 using the available function".to_string(),
            ),
        }];

        let request = ResponseRequest {
            model: "gpt-4".to_string(),
            input: ResponseInput::Messages(messages),
            stream: Some(true),
            instructions: Some("Use the provided function to calculate".to_string()),
            tools: Some(tools),
            tool_choice: Some(ToolChoice::Auto),
            ..Default::default()
        };

        // Verify all components are configured
        assert_eq!(request.stream, Some(true));
        assert!(request.instructions.is_some());
        assert!(request.tools.is_some());
        assert!(request.tool_choice.is_some());

        // Should serialize correctly for API calls
        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }

    #[test]
    fn test_minimal_streaming_request() {
        let request = ResponseRequest::new_text("gpt-4", "Hello").with_streaming(true);

        // Should have minimal required fields
        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.stream, Some(true));
        assert!(request.instructions.is_none());
        assert!(request.tools.is_none());

        // Should still serialize correctly
        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }
}

// Note: Actual streaming integration tests would require network access
// and are commented out for unit testing purposes

/*
#[cfg(test)]
mod integration_tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_streaming_text_response() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = StreamingApi::new(api_key).unwrap();

        let mut stream = api.create_text_stream("gpt-3.5-turbo", "Count to 5").await.unwrap();

        let mut content = String::new();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.unwrap();
            for choice in chunk.choices {
                if let Some(delta_content) = choice.delta.content {
                    content.push_str(&delta_content);
                }
                if choice.finish_reason.is_some() {
                    break;
                }
            }
        }

        assert!(!content.is_empty());
    }
}
*/
