//! # OpenAI Evals API Models
//!
//! This module provides data structures for OpenAI's Evals API, which allows you to
//! create and manage evaluations for measuring model performance.
//!
//! ## Overview
//!
//! The Evals API supports:
//! - **Eval Management**: Create, retrieve, update, delete, and list evals
//! - **Run Management**: Create, retrieve, delete, list, and cancel eval runs
//! - **Output Items**: Retrieve and list output items from eval runs
//!
//! ## Endpoints
//!
//! - `POST /v1/evals` - Create an eval
//! - `GET /v1/evals/{id}` - Retrieve an eval
//! - `POST /v1/evals/{id}` - Update an eval
//! - `DELETE /v1/evals/{id}` - Delete an eval
//! - `GET /v1/evals` - List evals
//! - `POST /v1/evals/{id}/runs` - Create a run
//! - `GET /v1/evals/{id}/runs/{run_id}` - Retrieve a run
//! - `DELETE /v1/evals/{id}/runs/{run_id}` - Delete a run
//! - `GET /v1/evals/{id}/runs` - List runs
//! - `POST /v1/evals/{id}/runs/{run_id}/cancel` - Cancel a run
//! - `GET /v1/evals/{id}/runs/{run_id}/output_items/{item_id}` - Retrieve output item
//! - `GET /v1/evals/{id}/runs/{run_id}/output_items` - List output items

use crate::{De, Ser};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Eval types
// ---------------------------------------------------------------------------

/// An eval object representing an evaluation configuration
#[derive(Debug, Clone, Ser, De)]
pub struct Eval {
    /// Unique identifier for the eval
    pub id: String,

    /// The object type (e.g., "eval")
    pub object: String,

    /// The Unix timestamp (in seconds) when the eval was created
    pub created_at: i64,

    /// The name of the eval
    pub name: String,

    /// A description of the eval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Metadata key-value pairs attached to the eval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Configuration for the data source used by the eval
    pub data_source_config: serde_json::Value,

    /// The testing criteria for the eval
    pub testing_criteria: Vec<serde_json::Value>,
}

/// Request to create a new eval
#[derive(Debug, Clone, Ser, De)]
pub struct CreateEvalRequest {
    /// The name of the eval
    pub name: String,

    /// A description of the eval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Metadata key-value pairs to attach to the eval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Configuration for the data source used by the eval
    pub data_source_config: serde_json::Value,

    /// The testing criteria for the eval
    pub testing_criteria: Vec<serde_json::Value>,
}

/// Request to update an existing eval
#[derive(Debug, Clone, Ser, De)]
pub struct UpdateEvalRequest {
    /// Updated name for the eval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Updated description for the eval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Updated metadata key-value pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Response from deleting an eval
#[derive(Debug, Clone, Ser, De)]
pub struct EvalDeleteResponse {
    /// The identifier of the deleted eval
    pub id: String,

    /// The object type
    pub object: String,

    /// Whether the eval was successfully deleted
    pub deleted: bool,
}

/// Paginated list of evals
#[derive(Debug, Clone, Ser, De)]
pub struct EvalList {
    /// The object type, which is "list"
    pub object: String,

    /// The list of eval objects
    pub data: Vec<Eval>,

    /// Cursor for the first item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// Cursor for the last item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more results available
    pub has_more: bool,
}

// ---------------------------------------------------------------------------
// Eval run types
// ---------------------------------------------------------------------------

/// An eval run representing a single execution of an eval
#[derive(Debug, Clone, Ser, De)]
pub struct EvalRun {
    /// Unique identifier for the eval run
    pub id: String,

    /// The object type (e.g., "eval.run")
    pub object: String,

    /// The identifier of the parent eval
    pub eval_id: String,

    /// The Unix timestamp (in seconds) when the run was created
    pub created_at: i64,

    /// The status of the eval run (e.g., "queued", "running", "completed", "failed", "cancelled")
    pub status: String,

    /// The model used for the eval run
    pub model: String,

    /// The name of the eval run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Metadata key-value pairs attached to the eval run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Counts of results by status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_counts: Option<serde_json::Value>,

    /// Per-model token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_model_usage: Option<Vec<serde_json::Value>>,

    /// Per-testing-criteria result breakdowns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_testing_criteria_results: Option<Vec<serde_json::Value>>,

    /// URL for the eval run report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,

    /// Error details if the run failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

/// Request to create a new eval run
#[derive(Debug, Clone, Ser, De)]
pub struct CreateEvalRunRequest {
    /// The model to use for the eval run
    pub model: String,

    /// The name of the eval run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Metadata key-value pairs to attach to the eval run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// The data source configuration for the eval run
    pub data_source: serde_json::Value,
}

/// Paginated list of eval runs
#[derive(Debug, Clone, Ser, De)]
pub struct EvalRunList {
    /// The object type, which is "list"
    pub object: String,

    /// The list of eval run objects
    pub data: Vec<EvalRun>,

    /// Cursor for the first item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// Cursor for the last item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more results available
    pub has_more: bool,
}

// ---------------------------------------------------------------------------
// Eval run output item types
// ---------------------------------------------------------------------------

/// A single output item from an eval run
#[derive(Debug, Clone, Ser, De)]
pub struct EvalRunOutputItem {
    /// Unique identifier for the output item
    pub id: String,

    /// The object type (e.g., "eval.run.output_item")
    pub object: String,

    /// The identifier of the parent eval
    pub eval_id: String,

    /// The identifier of the parent eval run
    pub run_id: String,

    /// The Unix timestamp (in seconds) when the output item was created
    pub created_at: i64,

    /// The status of the output item
    pub status: String,

    /// The identifier of the data source item that produced this output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource_item_id: Option<String>,

    /// The evaluation results for this output item
    pub results: Vec<serde_json::Value>,

    /// The sample data (input/output) for this output item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<serde_json::Value>,
}

/// Paginated list of eval run output items
#[derive(Debug, Clone, Ser, De)]
pub struct EvalRunOutputItemList {
    /// The object type, which is "list"
    pub object: String,

    /// The list of output item objects
    pub data: Vec<EvalRunOutputItem>,

    /// Cursor for the first item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// Cursor for the last item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more results available
    pub has_more: bool,
}

// ---------------------------------------------------------------------------
// Query parameter types
// ---------------------------------------------------------------------------

/// Parameters for listing evals
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListEvalsParams {
    /// Maximum number of evals to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order for the results ("asc" or "desc")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Pagination cursor - list evals after this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination cursor - list evals before this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Parameters for listing eval runs
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListEvalRunsParams {
    /// Maximum number of runs to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order for the results ("asc" or "desc")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Pagination cursor - list runs after this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination cursor - list runs before this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Parameters for listing eval run output items
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListEvalRunOutputItemsParams {
    /// Maximum number of output items to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order for the results ("asc" or "desc")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Pagination cursor - list output items after this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination cursor - list output items before this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Filter output items by status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_eval_request_serialization() {
        let request = CreateEvalRequest {
            name: "my-eval".to_string(),
            description: Some("Test evaluation".to_string()),
            metadata: None,
            data_source_config: json!({
                "type": "custom",
                "item_schema": {
                    "type": "object",
                    "properties": {
                        "prompt": { "type": "string" },
                        "expected": { "type": "string" }
                    }
                }
            }),
            testing_criteria: vec![json!({
                "type": "string_check",
                "name": "exact_match",
                "input": "{{ sample.output_text }}",
                "reference": "{{ item.expected }}"
            })],
        };

        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("my-eval"));
        assert!(json_str.contains("Test evaluation"));
        assert!(json_str.contains("string_check"));
    }

    #[test]
    fn test_eval_deserialization() {
        let json_str = r#"{
            "id": "eval_abc123",
            "object": "eval",
            "created_at": 1700000000,
            "name": "my-eval",
            "data_source_config": { "type": "custom" },
            "testing_criteria": []
        }"#;

        let eval: Eval = serde_json::from_str(json_str).unwrap();
        assert_eq!(eval.id, "eval_abc123");
        assert_eq!(eval.object, "eval");
        assert_eq!(eval.name, "my-eval");
        assert!(eval.description.is_none());
        assert!(eval.metadata.is_none());
    }

    #[test]
    fn test_eval_delete_response() {
        let response = EvalDeleteResponse {
            id: "eval_abc123".to_string(),
            object: "eval".to_string(),
            deleted: true,
        };

        assert!(response.deleted);
        assert_eq!(response.id, "eval_abc123");
    }

    #[test]
    fn test_eval_list_deserialization() {
        let json_str = r#"{
            "object": "list",
            "data": [],
            "has_more": false
        }"#;

        let list: EvalList = serde_json::from_str(json_str).unwrap();
        assert_eq!(list.object, "list");
        assert!(list.data.is_empty());
        assert!(!list.has_more);
        assert!(list.first_id.is_none());
        assert!(list.last_id.is_none());
    }

    #[test]
    fn test_update_eval_request_optional_fields() {
        let request = UpdateEvalRequest {
            name: Some("updated-name".to_string()),
            description: None,
            metadata: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("updated-name"));
        assert!(!json_str.contains("description"));
        assert!(!json_str.contains("metadata"));
    }

    #[test]
    fn test_create_eval_run_request_serialization() {
        let request = CreateEvalRunRequest {
            model: "gpt-4o".to_string(),
            name: Some("test-run".to_string()),
            metadata: None,
            data_source: json!({
                "type": "completions",
                "source": {
                    "type": "file_content",
                    "content": []
                }
            }),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("gpt-4o"));
        assert!(json_str.contains("test-run"));
        assert!(json_str.contains("completions"));
    }

    #[test]
    fn test_eval_run_deserialization() {
        let json_str = r#"{
            "id": "run_abc123",
            "object": "eval.run",
            "eval_id": "eval_abc123",
            "created_at": 1700000000,
            "status": "completed",
            "model": "gpt-4o"
        }"#;

        let run: EvalRun = serde_json::from_str(json_str).unwrap();
        assert_eq!(run.id, "run_abc123");
        assert_eq!(run.eval_id, "eval_abc123");
        assert_eq!(run.status, "completed");
        assert_eq!(run.model, "gpt-4o");
        assert!(run.name.is_none());
        assert!(run.error.is_none());
    }

    #[test]
    fn test_eval_run_output_item_deserialization() {
        let json_str = r#"{
            "id": "item_abc123",
            "object": "eval.run.output_item",
            "eval_id": "eval_abc123",
            "run_id": "run_abc123",
            "created_at": 1700000000,
            "status": "pass",
            "results": [{"score": 1.0}]
        }"#;

        let item: EvalRunOutputItem = serde_json::from_str(json_str).unwrap();
        assert_eq!(item.id, "item_abc123");
        assert_eq!(item.run_id, "run_abc123");
        assert_eq!(item.status, "pass");
        assert_eq!(item.results.len(), 1);
        assert!(item.datasource_item_id.is_none());
        assert!(item.sample.is_none());
    }

    #[test]
    fn test_list_params_defaults() {
        let params = ListEvalsParams::default();
        assert!(params.limit.is_none());
        assert!(params.order.is_none());
        assert!(params.after.is_none());
        assert!(params.before.is_none());

        let run_params = ListEvalRunsParams::default();
        assert!(run_params.limit.is_none());

        let output_params = ListEvalRunOutputItemsParams::default();
        assert!(output_params.status.is_none());
    }
}
