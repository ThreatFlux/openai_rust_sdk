//! # Evals API
//!
//! Client for the OpenAI Evals API, which allows you to create, manage,
//! and run evaluations for measuring model performance.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use crate::models::evals::{
    CreateEvalRequest, CreateEvalRunRequest, Eval, EvalDeleteResponse, EvalList, EvalRun,
    EvalRunList, EvalRunOutputItem, EvalRunOutputItemList, ListEvalRunOutputItemsParams,
    ListEvalRunsParams, ListEvalsParams, UpdateEvalRequest,
};

/// Evals API client for managing evaluations and eval runs
pub struct EvalsApi {
    /// Shared HTTP client for making requests
    client: HttpClient,
}

impl ApiClientConstructors for EvalsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self {
            client: http_client,
        }
    }
}

/// Build query parameters from optional fields, filtering out None values
fn build_list_params(
    limit: Option<u32>,
    order: Option<&str>,
    after: Option<&str>,
    before: Option<&str>,
) -> Vec<(String, String)> {
    let mut params = Vec::new();
    if let Some(l) = limit {
        params.push(("limit".to_string(), l.to_string()));
    }
    if let Some(o) = order {
        params.push(("order".to_string(), o.to_string()));
    }
    if let Some(a) = after {
        params.push(("after".to_string(), a.to_string()));
    }
    if let Some(b) = before {
        params.push(("before".to_string(), b.to_string()));
    }
    params
}

impl EvalsApi {
    /// Create a new eval
    pub async fn create_eval(&self, request: &CreateEvalRequest) -> Result<Eval> {
        self.client.post("/v1/evals", request).await
    }

    /// Retrieve an eval by ID
    pub async fn retrieve_eval(&self, eval_id: impl AsRef<str>) -> Result<Eval> {
        let path = format!("/v1/evals/{}", eval_id.as_ref());
        self.client.get(&path).await
    }

    /// Update an eval by ID
    pub async fn update_eval(
        &self,
        eval_id: impl AsRef<str>,
        request: &UpdateEvalRequest,
    ) -> Result<Eval> {
        let path = format!("/v1/evals/{}", eval_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Delete an eval by ID
    pub async fn delete_eval(&self, eval_id: impl AsRef<str>) -> Result<EvalDeleteResponse> {
        let path = format!("/v1/evals/{}", eval_id.as_ref());
        self.client.delete(&path).await
    }

    /// List evals with optional pagination parameters
    pub async fn list_evals(&self, params: Option<&ListEvalsParams>) -> Result<EvalList> {
        match params {
            Some(p) => {
                let query = build_list_params(
                    p.limit,
                    p.order.as_deref(),
                    p.after.as_deref(),
                    p.before.as_deref(),
                );
                self.client.get_with_query("/v1/evals", &query).await
            }
            None => self.client.get("/v1/evals").await,
        }
    }

    /// Create a run for an eval
    pub async fn create_eval_run(
        &self,
        eval_id: impl AsRef<str>,
        request: &CreateEvalRunRequest,
    ) -> Result<EvalRun> {
        let path = format!("/v1/evals/{}/runs", eval_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Retrieve a run for an eval
    pub async fn retrieve_eval_run(
        &self,
        eval_id: impl AsRef<str>,
        run_id: impl AsRef<str>,
    ) -> Result<EvalRun> {
        let path = format!("/v1/evals/{}/runs/{}", eval_id.as_ref(), run_id.as_ref());
        self.client.get(&path).await
    }

    /// Delete a run for an eval
    pub async fn delete_eval_run(
        &self,
        eval_id: impl AsRef<str>,
        run_id: impl AsRef<str>,
    ) -> Result<EvalRun> {
        let path = format!("/v1/evals/{}/runs/{}", eval_id.as_ref(), run_id.as_ref());
        self.client.delete(&path).await
    }

    /// List runs for an eval with optional pagination parameters
    pub async fn list_eval_runs(
        &self,
        eval_id: impl AsRef<str>,
        params: Option<&ListEvalRunsParams>,
    ) -> Result<EvalRunList> {
        let path = format!("/v1/evals/{}/runs", eval_id.as_ref());
        match params {
            Some(p) => {
                let query = build_list_params(
                    p.limit,
                    p.order.as_deref(),
                    p.after.as_deref(),
                    p.before.as_deref(),
                );
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }

    /// Cancel a running eval run
    pub async fn cancel_eval_run(
        &self,
        eval_id: impl AsRef<str>,
        run_id: impl AsRef<str>,
    ) -> Result<EvalRun> {
        let path = format!(
            "/v1/evals/{}/runs/{}/cancel",
            eval_id.as_ref(),
            run_id.as_ref()
        );
        self.client.post(&path, &()).await
    }

    /// Retrieve a specific output item from an eval run
    pub async fn retrieve_eval_run_output_item(
        &self,
        eval_id: impl AsRef<str>,
        run_id: impl AsRef<str>,
        output_item_id: impl AsRef<str>,
    ) -> Result<EvalRunOutputItem> {
        let path = format!(
            "/v1/evals/{}/runs/{}/output_items/{}",
            eval_id.as_ref(),
            run_id.as_ref(),
            output_item_id.as_ref()
        );
        self.client.get(&path).await
    }

    /// List output items from an eval run with optional pagination parameters
    pub async fn list_eval_run_output_items(
        &self,
        eval_id: impl AsRef<str>,
        run_id: impl AsRef<str>,
        params: Option<&ListEvalRunOutputItemsParams>,
    ) -> Result<EvalRunOutputItemList> {
        let path = format!(
            "/v1/evals/{}/runs/{}/output_items",
            eval_id.as_ref(),
            run_id.as_ref()
        );
        match params {
            Some(p) => {
                let mut query = build_list_params(
                    p.limit,
                    p.order.as_deref(),
                    p.after.as_deref(),
                    p.before.as_deref(),
                );
                if let Some(ref status) = p.status {
                    query.push(("status".to_string(), status.clone()));
                }
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_evals_api_creation() {
        let api = EvalsApi::new("test-key").unwrap();
        assert_eq!(api.client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_evals_api_creation_with_base_url() {
        let api = EvalsApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_evals_api_empty_key_fails() {
        let result = EvalsApi::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_list_params_all_set() {
        let params = build_list_params(Some(10), Some("desc"), Some("after-id"), Some("before-id"));
        assert_eq!(params.len(), 4);
        assert_eq!(params[0], ("limit".to_string(), "10".to_string()));
        assert_eq!(params[1], ("order".to_string(), "desc".to_string()));
        assert_eq!(params[2], ("after".to_string(), "after-id".to_string()));
        assert_eq!(params[3], ("before".to_string(), "before-id".to_string()));
    }

    #[test]
    fn test_build_list_params_none() {
        let params = build_list_params(None, None, None, None);
        assert!(params.is_empty());
    }

    #[test]
    fn test_build_list_params_partial() {
        let params = build_list_params(Some(5), None, Some("cursor"), None);
        assert_eq!(params.len(), 2);
        assert_eq!(params[0], ("limit".to_string(), "5".to_string()));
        assert_eq!(params[1], ("after".to_string(), "cursor".to_string()));
    }

    #[test]
    fn test_create_eval_request_serialization() {
        let req = CreateEvalRequest {
            name: "my-eval".to_string(),
            description: Some("A test eval".to_string()),
            metadata: None,
            data_source_config: json!({"type": "custom"}),
            testing_criteria: vec![json!({"type": "string_check"})],
        };
        let json_val = serde_json::to_value(&req).unwrap();
        assert_eq!(json_val["name"], "my-eval");
        assert_eq!(json_val["description"], "A test eval");
        assert!(json_val.get("metadata").is_none());
    }

    #[test]
    fn test_update_eval_request_skip_none() {
        let req = UpdateEvalRequest {
            name: Some("new-name".to_string()),
            description: None,
            metadata: None,
        };
        let json_val = serde_json::to_value(&req).unwrap();
        assert_eq!(json_val["name"], "new-name");
        assert!(json_val.get("description").is_none());
        assert!(json_val.get("metadata").is_none());
    }

    #[test]
    fn test_eval_deserialization() {
        let json = r#"{
            "id": "eval-123",
            "object": "eval",
            "created_at": 1700000000,
            "name": "my-eval",
            "data_source_config": {"type": "custom"},
            "testing_criteria": []
        }"#;
        let eval: Eval = serde_json::from_str(json).unwrap();
        assert_eq!(eval.id, "eval-123");
        assert_eq!(eval.name, "my-eval");
        assert!(eval.description.is_none());
    }

    #[test]
    fn test_eval_run_deserialization() {
        let json = r#"{
            "id": "run-abc",
            "object": "eval.run",
            "eval_id": "eval-123",
            "created_at": 1700000000,
            "status": "completed",
            "model": "gpt-4o"
        }"#;
        let run: EvalRun = serde_json::from_str(json).unwrap();
        assert_eq!(run.id, "run-abc");
        assert_eq!(run.status, "completed");
        assert_eq!(run.model, "gpt-4o");
    }

    #[test]
    fn test_eval_delete_response_deserialization() {
        let json = r#"{"id": "eval-123", "object": "eval", "deleted": true}"#;
        let resp: EvalDeleteResponse = serde_json::from_str(json).unwrap();
        assert!(resp.deleted);
        assert_eq!(resp.id, "eval-123");
    }

    #[test]
    fn test_list_eval_run_output_items_params_with_status() {
        let params = ListEvalRunOutputItemsParams {
            limit: Some(20),
            order: None,
            after: None,
            before: None,
            status: Some("pass".to_string()),
        };
        assert_eq!(params.limit, Some(20));
        assert_eq!(params.status, Some("pass".to_string()));
    }

    #[test]
    fn test_create_eval_run_request_serialization() {
        let req = CreateEvalRunRequest {
            model: "gpt-4o".to_string(),
            name: Some("run-1".to_string()),
            metadata: None,
            data_source: json!({"type": "completions"}),
        };
        let json_val = serde_json::to_value(&req).unwrap();
        assert_eq!(json_val["model"], "gpt-4o");
        assert_eq!(json_val["name"], "run-1");
    }

    #[test]
    fn test_eval_list_deserialization() {
        let json = r#"{"object": "list", "data": [], "has_more": false}"#;
        let list: EvalList = serde_json::from_str(json).unwrap();
        assert!(list.data.is_empty());
        assert!(!list.has_more);
    }

    #[test]
    fn test_eval_run_output_item_deserialization() {
        let json = r#"{
            "id": "item-1",
            "object": "eval.run.output_item",
            "eval_id": "eval-1",
            "run_id": "run-1",
            "created_at": 1700000000,
            "status": "pass",
            "results": [{"score": 1.0}]
        }"#;
        let item: EvalRunOutputItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item-1");
        assert_eq!(item.results.len(), 1);
    }
}
