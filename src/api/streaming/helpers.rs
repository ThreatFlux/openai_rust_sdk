//! Helper functions and utilities for streaming operations

// Re-export functions from other modules for backward compatibility
pub use crate::api::streaming::stream_operations::{collect_stream_response, ResponseStreamExt};
pub use crate::api::streaming::utilities::{
    chunk_to_events, process_stream_event, to_streaming_json,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::streaming::to_streaming_json;
    use serde::Serialize;

    #[derive(Serialize)]
    struct SimpleRequest {
        assistant_id: String,
        instructions: String,
    }

    #[derive(Serialize)]
    struct ComplexRequest {
        assistant_id: String,
        instructions: Option<String>,
        tools: Vec<String>,
        metadata: std::collections::HashMap<String, serde_json::Value>,
        existing_stream: Option<bool>,
    }

    #[test]
    fn test_to_streaming_json_simple_struct() {
        let request = SimpleRequest {
            assistant_id: "asst_123".to_string(),
            instructions: "Help me".to_string(),
        };

        let result = to_streaming_json(&request);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert_eq!(json["stream"], serde_json::Value::Bool(true));
        assert_eq!(
            json["assistant_id"],
            serde_json::Value::String("asst_123".to_string())
        );
        assert_eq!(
            json["instructions"],
            serde_json::Value::String("Help me".to_string())
        );
    }

    #[test]
    fn test_to_streaming_json_complex_struct() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(
            "key1".to_string(),
            serde_json::Value::String("value1".to_string()),
        );
        metadata.insert(
            "key2".to_string(),
            serde_json::Value::Number(serde_json::Number::from(42)),
        );

        let request = ComplexRequest {
            assistant_id: "asst_456".to_string(),
            instructions: Some("Complex task".to_string()),
            tools: vec!["function1".to_string(), "function2".to_string()],
            metadata,
            existing_stream: None,
        };

        let result = to_streaming_json(&request);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert_eq!(json["stream"], serde_json::Value::Bool(true));
        assert_eq!(
            json["assistant_id"],
            serde_json::Value::String("asst_456".to_string())
        );
        assert_eq!(
            json["instructions"],
            serde_json::Value::String("Complex task".to_string())
        );
        assert!(json["tools"].is_array());
        assert!(json["metadata"].is_object());
        assert_eq!(json["existing_stream"], serde_json::Value::Null);
    }

    #[test]
    fn test_to_streaming_json_overwrites_existing_stream_field() {
        let request = ComplexRequest {
            assistant_id: "asst_789".to_string(),
            instructions: None,
            tools: vec![],
            metadata: std::collections::HashMap::new(),
            existing_stream: Some(false), // This should be overwritten to true
        };

        let result = to_streaming_json(&request);
        assert!(result.is_ok());

        let json = result.unwrap();
        // Should overwrite the existing stream field with true
        assert_eq!(json["stream"], serde_json::Value::Bool(true));
        assert_eq!(json["existing_stream"], serde_json::Value::Bool(false));
    }

    #[test]
    fn test_to_streaming_json_with_empty_struct() {
        #[derive(Serialize)]
        struct EmptyRequest {}

        let request = EmptyRequest {};
        let result = to_streaming_json(&request);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert_eq!(json["stream"], serde_json::Value::Bool(true));
        // Should be an object with just the stream field
        assert!(json.is_object());
        assert_eq!(json.as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_to_streaming_json_preserves_all_original_fields() {
        #[derive(Serialize)]
        struct FullRequest {
            id: u64,
            name: String,
            active: bool,
            score: f64,
            tags: Vec<String>,
            optional: Option<String>,
        }

        let request = FullRequest {
            id: 123,
            name: "test_request".to_string(),
            active: true,
            score: 3.15,
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            optional: Some("optional_value".to_string()),
        };

        let result = to_streaming_json(&request);
        assert!(result.is_ok());

        let json = result.unwrap();

        // Verify streaming is added
        assert_eq!(json["stream"], serde_json::Value::Bool(true));

        // Verify all original fields are preserved
        assert_eq!(
            json["id"],
            serde_json::Value::Number(serde_json::Number::from(123))
        );
        assert_eq!(
            json["name"],
            serde_json::Value::String("test_request".to_string())
        );
        assert_eq!(json["active"], serde_json::Value::Bool(true));
        assert_eq!(
            json["score"],
            serde_json::Value::Number(serde_json::Number::from_f64(3.15).unwrap())
        );
        assert!(json["tags"].is_array());
        assert_eq!(
            json["optional"],
            serde_json::Value::String("optional_value".to_string())
        );
    }
}
