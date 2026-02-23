use crate::api::base::HttpClient;
use crate::api::common::{
    build_list_query_params, ApiClientConstructors, ListQueryParams, StandardListParams,
};
use crate::api::shared_utilities::EnumConverter;
use crate::error::{ApiErrorResponse, OpenAIError, Result};
use crate::models::responses_v2::{
    ContentPart, CreateResponseRequest, ResponseInput, ResponseItem, ResponseObject,
    ResponseStreamEvent,
};
use crate::{De, Ser};
use eventsource_stream::Eventsource;
use futures::StreamExt as FuturesStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// Streaming response type for the Responses API
pub type ResponsesEventStream =
    Pin<Box<dyn futures::Stream<Item = Result<ResponseStreamEvent>> + Send>>;

/// Client for the modern `/v1/responses` API surface
#[derive(Clone)]
pub struct ResponsesApiV2 {
    /// The HTTP client used for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for ResponsesApiV2 {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ResponsesApiV2 {
    /// Create a new response
    pub async fn create_response(&self, request: &CreateResponseRequest) -> Result<ResponseObject> {
        let payload = request.to_payload()?;
        self.http_client.post("/v1/responses", &payload).await
    }

    /// Create a streaming response using SSE
    pub async fn stream_response(
        &self,
        request: &CreateResponseRequest,
    ) -> Result<ResponsesEventStream> {
        let mut streaming_request = request.clone();
        streaming_request.stream = Some(true);
        let payload = streaming_request.to_payload()?;

        let url = format!("{}{}", self.http_client.base_url(), "/v1/responses");
        let response = self
            .http_client
            .client()
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.http_client.api_key()),
            )
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_response: ApiErrorResponse = response.json().await?;
            return Err(OpenAIError::from_api_response(
                status.as_u16(),
                error_response,
            ));
        }

        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|event| async move { parse_sse_event(event) });

        Ok(Box::pin(stream))
    }

    /// Retrieve a response by ID
    pub async fn retrieve_response(
        &self,
        response_id: impl AsRef<str>,
        include: Option<Vec<String>>,
    ) -> Result<ResponseObject> {
        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(include) = include {
            if !include.is_empty() {
                query.push(("include".into(), include.join(",")));
            }
        }
        let url = self
            .http_client
            .build_url(&format!("/v1/responses/{}", response_id.as_ref()), &query);
        let headers = self.http_client.build_headers()?;
        self.http_client
            .client()
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await
            .map_err(OpenAIError::from)
    }

    /// Delete a stored response
    pub async fn delete_response(&self, response_id: impl AsRef<str>) -> Result<DeleteResponseAck> {
        let url = format!("/v1/responses/{}", response_id.as_ref());
        self.http_client.delete(&url).await
    }

    /// Cancel a background response execution
    pub async fn cancel_response(&self, response_id: impl AsRef<str>) -> Result<ResponseObject> {
        let url = format!("/v1/responses/{}/cancel", response_id.as_ref());
        self.http_client.post(&url, &Value::Null).await
    }

    /// List responses for the project
    pub async fn list_responses(&self, params: &ListResponsesParams) -> Result<ResponseList> {
        let mut query = build_list_query_params(params);
        if let Some(model) = &params.model {
            query.push(("model".into(), model.clone()));
        }
        if let Some(status) = &params.status {
            query.push(("status".into(), status.clone()));
        }
        let url = self.http_client.build_url("/v1/responses", &query);
        let headers = self.http_client.build_headers()?;
        self.http_client
            .client()
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await
            .map_err(OpenAIError::from)
    }

    /// Compact a response (server-side context compaction)
    ///
    /// Reduces the size of a stored response's context while preserving key information.
    pub async fn compact_response(
        &self,
        response_id: impl AsRef<str>,
        background: Option<bool>,
    ) -> Result<ResponseObject> {
        let url = format!("/v1/responses/{}/compact", response_id.as_ref());
        let mut body = serde_json::json!({});
        if let Some(bg) = background {
            body["background"] = serde_json::json!(bg);
        }
        self.http_client.post(&url, &body).await
    }

    /// Count input tokens for a response
    ///
    /// Returns the number of tokens that would be consumed by the given input.
    pub async fn count_input_tokens(&self, response_id: impl AsRef<str>) -> Result<Value> {
        let url = format!("/v1/responses/{}/input_tokens/count", response_id.as_ref());
        self.http_client.post(&url, &Value::Null).await
    }

    /// List input items used to generate a response
    pub async fn list_response_input_items(
        &self,
        response_id: impl AsRef<str>,
        params: &StandardListParams,
    ) -> Result<ResponseInputItemList> {
        let query = build_list_query_params(params);
        let url = self.http_client.build_url(
            &format!("/v1/responses/{}/input_items", response_id.as_ref()),
            &query,
        );
        let headers = self.http_client.build_headers()?;
        self.http_client
            .client()
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await
            .map_err(OpenAIError::from)
    }
}

/// Acknowledgement returned when deleting a response
#[derive(Debug, Clone, Ser, De)]
pub struct DeleteResponseAck {
    /// Response identifier
    pub id: String,
    /// Object type
    pub object: String,
    /// Whether deletion succeeded
    pub deleted: bool,
}

/// Response list wrapper
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseList {
    /// Object type (`list`)
    pub object: String,
    /// Response objects (or summaries) returned
    pub data: Vec<ResponseObject>,
    /// First identifier in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Last identifier in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether additional results are available
    pub has_more: bool,
}

/// Input item list wrapper
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseInputItemList {
    /// Object type (`list`)
    pub object: String,
    /// Items included in the response input
    pub data: Vec<ResponseItem>,
    /// First identifier in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Last identifier in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether additional items are available
    pub has_more: bool,
}

/// Parameters used to list responses
#[derive(Debug, Clone, Default)]
pub struct ListResponsesParams {
    /// Standard list parameters (limit/order/after/before)
    pub list: StandardListParams,
    /// Filter responses by status
    pub status: Option<String>,
    /// Filter responses by model name
    pub model: Option<String>,
}

impl ListQueryParams for ListResponsesParams {
    fn limit(&self) -> Option<u32> {
        self.list.limit
    }

    fn order_str(&self) -> Option<&str> {
        self.list.order.as_deref()
    }

    fn after(&self) -> Option<&String> {
        self.list.after.as_ref()
    }

    fn before(&self) -> Option<&String> {
        self.list.before.as_ref()
    }
}

/// Parse an SSE event into a ResponseStreamEvent
///
/// Returns None for ping events (keep-alive), and Some(Result) for data events
fn parse_sse_event(
    event: std::result::Result<
        eventsource_stream::Event,
        eventsource_stream::EventStreamError<reqwest::Error>,
    >,
) -> Option<Result<ResponseStreamEvent>> {
    match event {
        Ok(event) => {
            if event.event == "ping" {
                return None; // keep-alive
            }

            let data = event.data;
            let value: serde_json::Result<ResponseStreamEvent> = serde_json::from_str(&data);
            match value {
                Ok(parsed) => Some(Ok(parsed)),
                Err(err) => Some(Err(OpenAIError::Json(err))),
            }
        }
        Err(err) => Some(Err(OpenAIError::streaming(err.to_string()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::responses_v2::CreateResponseRequest;
    use eventsource_stream::{Event, EventStreamError};
    use futures::StreamExt;
    use httpmock::prelude::*;
    use serde_json::json;

    #[test]
    fn parse_known_event_returns_variant() {
        let event = Event {
            id: String::new(),
            event: "response.output_text.delta".into(),
            data: json!({
                "type": "response.output_text.delta",
                "response_id": "resp_1",
                "output_index": 0,
                "delta": "Hello"
            })
            .to_string(),
            retry: None,
        };

        let parsed = parse_sse_event(Ok(event)).expect("some").expect("ok");
        match parsed {
            ResponseStreamEvent::OutputTextDelta { delta, .. } => assert_eq!(delta, "Hello"),
            other => panic!("unexpected event: {:?}", other),
        }
    }

    #[test]
    fn parse_error_event_maps_to_stream_error() {
        let event = Event {
            id: String::new(),
            event: "response.failed".into(),
            data: json!({
                "type": "response.failed",
                "event_id": null,
                "response": {
                    "id": "resp_2",
                    "object": "response",
                    "created_at": 0,
                    "status": "failed",
                    "error": {"message": "bad"},
                    "output": [],
                    "model": null,
                    "usage": null
                }
            })
            .to_string(),
            retry: None,
        };

        let parsed = parse_sse_event(Ok(event)).expect("some").expect("ok");
        assert!(matches!(parsed, ResponseStreamEvent::ResponseFailed { .. }));
    }

    #[test]
    fn parse_event_error_surface() {
        let utf8_error = String::from_utf8(vec![0xFF]).unwrap_err();
        let error = EventStreamError::Utf8(utf8_error);
        let result = parse_sse_event(Err(error)).expect("some");
        assert!(result.is_err());
    }

    fn sample_response(text: &str) -> serde_json::Value {
        json!({
            "id": "resp_123",
            "object": "response",
            "created_at": 1,
            "status": "completed",
            "model": "gpt-test",
            "output": [
                {
                    "type": "message",
                    "id": "msg_1",
                    "status": "completed",
                    "role": "assistant",
                    "content": [
                        { "type": "output_text", "text": text }
                    ]
                }
            ],
            "usage": {
                "input_tokens": 1,
                "output_tokens": 2,
                "total_tokens": 3,
                "input_tokens_details": { "cached_tokens": 0 },
                "output_tokens_details": {
                    "reasoning_tokens": 0,
                    "accepted_prediction_tokens": 0,
                    "rejected_prediction_tokens": 0
                }
            }
        })
    }

    #[tokio::test]
    async fn exercise_http_endpoints() {
        let server = MockServer::start_async().await;

        let create_body = sample_response("Created").to_string();
        let retrieve_body = sample_response("Retrieved").to_string();
        let cancel_body = sample_response("Cancelled").to_string();
        let list_body = json!({
            "object": "list",
            "data": [sample_response("Listed")],
            "first_id": "resp_123",
            "last_id": "resp_123",
            "has_more": false
        })
        .to_string();
        let input_items_body = json!({
            "object": "list",
            "data": [{
                "type": "message",
                "id": "msg_123",
                "status": "completed",
                "role": "user",
                "content": [{"type": "input_text", "text": "Hello"}]
            }],
            "first_id": "msg_123",
            "last_id": "msg_123",
            "has_more": false
        })
        .to_string();

        let completed_event = json!({
            "type": "response.completed",
            "event_id": null,
            "response": sample_response("Stream done")
        })
        .to_string();
        let delta_event = json!({
            "type": "response.output_text.delta",
            "response_id": "resp_123",
            "output_index": 0,
            "delta": "Stream"
        })
        .to_string();
        let sse_body = format!(
            "event: response.output_text.delta\ndata: {}\n\n\
event: response.completed\ndata: {}\n\n",
            delta_event, completed_event
        );

        let stream_mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/v1/responses")
                    .header("Accept", "text/event-stream");
                then.status(200)
                    .header("Content-Type", "text/event-stream")
                    .body(&sse_body);
            })
            .await;

        let create_mock = server
            .mock_async(|when, then| {
                when.method(POST).path("/v1/responses");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .body(&create_body);
            })
            .await;

        let retrieve_mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/v1/responses/resp_123");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .body(&retrieve_body);
            })
            .await;

        let cancel_mock = server
            .mock_async(|when, then| {
                when.method(POST).path("/v1/responses/resp_123/cancel");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .body(&cancel_body);
            })
            .await;

        let delete_mock = server
            .mock_async(|when, then| {
                when.method(DELETE).path("/v1/responses/resp_123");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .body("{\"id\":\"resp_123\",\"object\":\"response\",\"deleted\":true}");
            })
            .await;

        let list_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/v1/responses")
                    .query_param("status", "completed")
                    .query_param("model", "gpt-4o-mini");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .body(&list_body);
            })
            .await;

        let input_items_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/v1/responses/resp_123/input_items")
                    .query_param("limit", "5");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .body(&input_items_body);
            })
            .await;

        let api = ResponsesApiV2::new_with_base_url("test-key", &server.base_url()).unwrap();
        let request = CreateResponseRequest::new_text("gpt-4o-mini", "Hello");

        let created = api.create_response(&request).await.unwrap();
        assert_eq!(created.output_text(), "Created");

        let mut stream = api.stream_response(&request).await.unwrap();
        let mut deltas = 0;
        while let Some(event) = stream.next().await {
            match event.unwrap() {
                ResponseStreamEvent::OutputTextDelta { delta, .. } => {
                    assert_eq!(delta, "Stream");
                    deltas += 1;
                }
                ResponseStreamEvent::ResponseCompleted { response, .. } => {
                    assert_eq!(response.output_text(), "Stream done");
                }
                _ => {}
            }
        }
        assert_eq!(deltas, 1);

        let retrieved = api
            .retrieve_response("resp_123", Some(vec!["output".into()]))
            .await
            .unwrap();
        assert_eq!(retrieved.output_text(), "Retrieved");

        let cancelled = api.cancel_response("resp_123").await.unwrap();
        assert_eq!(cancelled.output_text(), "Cancelled");

        let deleted = api.delete_response("resp_123").await.unwrap();
        assert!(deleted.deleted);

        let list_params = super::ListResponsesParams {
            status: Some("completed".into()),
            model: Some("gpt-4o-mini".into()),
            ..Default::default()
        };
        let listed = api.list_responses(&list_params).await.unwrap();
        assert_eq!(listed.data.len(), 1);

        let standard = super::StandardListParams {
            limit: Some(5),
            ..Default::default()
        };
        let items = api
            .list_response_input_items("resp_123", &standard)
            .await
            .unwrap();
        assert_eq!(items.data.len(), 1);

        stream_mock.assert_async().await;
        create_mock.assert_async().await;
        retrieve_mock.assert_async().await;
        cancel_mock.assert_async().await;
        delete_mock.assert_async().await;
        list_mock.assert_async().await;
        input_items_mock.assert_async().await;
    }
}
