//! Fixture and HTTP regressions for the current Responses wire format.

use futures::StreamExt;
use httpmock::prelude::*;
use openai_rust_sdk::OpenAIError;
use openai_rust_sdk::api::common::{ApiClientConstructors, StandardListParams};
use openai_rust_sdk::api::responses_v2::{ListResponsesParams, ResponsesApiV2};
use openai_rust_sdk::models::responses_v2::{
    CompactResponseRequest, CompactedResponse, ContextManagement, CreateResponseRequest,
    Instructions, PromptCacheRetention, ResponseCreateOptions, ResponseStreamEnvelope,
    ResponseStreamEvent,
};
use serde_json::{Value, json};

fn current_delta() -> Value {
    json!({"type":"response.output_text.delta","item_id":"msg_1",
        "content_index":0,"output_index":0,"sequence_number":4,"delta":"🦀",
        "logprobs":[],"future_metadata":{"enabled":true}})
}

#[test]
fn current_text_events_do_not_require_legacy_response_id() {
    let envelope = ResponseStreamEnvelope::from_payload(current_delta()).unwrap();
    assert!(
        matches!(envelope.event, ResponseStreamEvent::OutputTextDelta { ref delta, .. } if delta == "🦀")
    );
    assert_eq!(envelope.sequence_number, Some(4));
    assert_eq!(envelope.payload, current_delta());
    let mut done = current_delta();
    done["type"] = json!("response.output_text.done");
    done["text"] = json!("complete");
    assert!(
        matches!(ResponseStreamEnvelope::from_payload(done).unwrap().event,
        ResponseStreamEvent::OutputTextDone { text, .. } if text == "complete")
    );
}

#[test]
fn current_item_event_without_response_id_is_typed() {
    let value = json!({"type":"response.output_item.added","output_index":0,
        "sequence_number":2,"item":{"type":"reasoning","id":"rs_1",
        "encrypted_content":"opaque_replay_value"}});
    let envelope = ResponseStreamEnvelope::from_payload(value.clone()).unwrap();
    assert!(matches!(
        envelope.event,
        ResponseStreamEvent::OutputItemAdded { .. }
    ));
    assert_eq!(envelope.payload, value);
}

#[test]
fn flat_error_and_future_events_preserve_their_payloads() {
    let error = json!({"type":"error","message":"request failed","code":"invalid_request",
        "param":null,"sequence_number":0,"request_id":"req_fixture"});
    let envelope = ResponseStreamEnvelope::from_payload(error.clone()).unwrap();
    assert!(
        matches!(envelope.event, ResponseStreamEvent::StreamError { error, .. }
        if error.message.as_deref() == Some("request failed"))
    );
    assert_eq!(envelope.payload, error);
    let future = json!({"type":"response.future_event","sequence_number":7,"data":[1,{"x":true}]});
    let envelope = ResponseStreamEnvelope::from_payload(future.clone()).unwrap();
    assert!(matches!(envelope.event, ResponseStreamEvent::Unknown));
    assert_eq!(envelope.payload, future);
}

#[test]
fn malformed_known_events_are_errors() {
    let mut value = current_delta();
    value.as_object_mut().unwrap().remove("delta");
    assert!(ResponseStreamEnvelope::from_payload(value).is_err());
    assert!(ResponseStreamEnvelope::from_payload(json!({"type":"error"})).is_err());
    assert!(
        ResponseStreamEnvelope::from_payload(
            json!({"type":"response.future","sequence_number":-1})
        )
        .is_err()
    );
    assert!(ResponseStreamEnvelope::from_payload(json!({"data":true})).is_err());
}

#[test]
fn count_projection_keeps_prompt_fields_and_excludes_generation_controls() {
    let request: CreateResponseRequest = serde_json::from_value(json!({
        "model":"model_fixture","input":"hello","instructions":"be concise",
        "previous_response_id":"resp_1","parallel_tool_calls":false,
        "temperature":0.1,"max_output_tokens":10,"stream":true,"store":false,
        "background":true,"metadata":{"fixture":"yes"},"safety_identifier":"safe_fixture",
        "prompt_cache_key":"cache_fixture","truncation":"auto","personality":"friendly",
        "reasoning":{"effort":"low"},"text":{"verbosity":"low"}
    }))
    .unwrap();
    let payload = request.to_input_token_payload().unwrap();
    assert_eq!(
        payload,
        json!({"model":"model_fixture","input":"hello",
        "instructions":"be concise","previous_response_id":"resp_1",
        "parallel_tool_calls":false,"truncation":"auto","personality":"friendly",
        "reasoning":{"effort":"low"},"text":{"verbosity":"low"}})
    );
}

#[test]
fn count_and_compaction_reject_unsupported_instruction_or_prompt_inputs() {
    let mut request = CreateResponseRequest::new_text("model_fixture", "input");
    request.instructions = Some(Instructions::Raw(
        json!([{"role":"system","content":"instructions"}]),
    ));
    assert!(request.to_input_token_payload().is_err());
    assert!(CompactResponseRequest::try_from(&request).is_err());
    let request: CreateResponseRequest = serde_json::from_value(json!({
        "model":"model_fixture","input":"hello","prompt":{"id":"pmpt_fixture"}
    }))
    .unwrap();
    assert!(request.to_input_token_payload().is_err());
    assert!(CompactResponseRequest::try_from(&request).is_err());
}

#[test]
fn current_options_extend_existing_request_without_changing_its_layout() {
    let request = CreateResponseRequest::new_text("model_fixture", "input");
    let options = ResponseCreateOptions {
        context_management: Some(vec![ContextManagement::Compaction {
            compact_threshold: Some(12000),
        }]),
        prompt_cache_retention: Some(PromptCacheRetention::TwentyFourHours),
    };
    assert_eq!(
        options.to_payload(&request).unwrap(),
        json!({
            "model":"model_fixture","input":"input","prompt_cache_retention":"24h",
            "context_management":[{"type":"compaction","compact_threshold":12000}]
        })
    );
    assert_eq!(
        ResponseCreateOptions::default()
            .to_payload(&request)
            .unwrap(),
        request.to_payload().unwrap()
    );
}

fn compaction_response() -> Value {
    json!({"id":"cmp_fixture","object":"response.compaction","created_at":1,
        "output":[{"type":"compaction","encrypted_content":"opaque_replay_value","future":true}],
        "usage":{"input_tokens":12,"output_tokens":3,"total_tokens":15,
            "input_tokens_details":{"cached_tokens":2,"cache_write_tokens":4,"future":9},
            "output_tokens_details":{"reasoning_tokens":1}},"future_metadata":false})
}

#[test]
fn compaction_preserves_opaque_output_and_cache_write_accounting() {
    let original = compaction_response();
    let response: CompactedResponse = serde_json::from_value(original.clone()).unwrap();
    assert_eq!(
        response.usage.input_tokens_details.cache_write_tokens,
        Some(4)
    );
    assert_eq!(
        response.output[0].extra["encrypted_content"],
        "opaque_replay_value"
    );
    assert_eq!(serde_json::to_value(response).unwrap(), original);
}

#[tokio::test]
async fn typed_compaction_and_count_send_only_supported_fields() {
    let server = MockServer::start_async().await;
    let compact = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/responses/compact")
                .json_body(json!({
            "model":"model_fixture","input":"hello","prompt_cache_retention":"in_memory"}));
            then.status(200).json_body(compaction_response());
        })
        .await;
    let count = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/responses/input_tokens")
                .json_body(json!({"model":"model_fixture","input":"hello"}));
            then.status(200)
                .json_body(json!({"object":"response.input_tokens","input_tokens":7}));
        })
        .await;
    let api =
        ResponsesApiV2::new_with_base_url("fixture-key".to_owned(), server.base_url()).unwrap();
    let mut request = CompactResponseRequest::new("model_fixture", "hello");
    request.prompt_cache_retention = Some(PromptCacheRetention::InMemory);
    assert_eq!(
        api.compact_context(&request).await.unwrap().id,
        "cmp_fixture"
    );
    let request = CreateResponseRequest::new_text("model_fixture", "hello")
        .with_streaming(true)
        .with_max_tokens(10)
        .with_store(false);
    assert_eq!(
        api.count_input_tokens(&request).await.unwrap().input_tokens,
        7
    );
    compact.assert_async().await;
    count.assert_async().await;
}

#[tokio::test]
async fn lossless_sse_preserves_wire_and_transport_metadata() {
    let server = MockServer::start_async().await;
    let payload = current_delta();
    let body = format!(
        "id: frame_1\r\nretry: 3000\r\nevent: response.output_text.delta\r\ndata: {payload}\r\n\r\nevent: ping\r\ndata: keepalive\r\n\r\ndata: [DONE]\r\n\r\ndata: {{\"type\":\"response.after_terminal\"}}\r\n\r\n",
    );
    let mock = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/responses")
                .header("authorization", "Bearer fixture-key")
                .json_body(json!({"model":"model_fixture","input":"hello","stream":true}));
            then.status(200)
                .header("content-type", "text/event-stream")
                .body(body);
        })
        .await;
    let api =
        ResponsesApiV2::new_with_base_url("fixture-key".to_owned(), server.base_url()).unwrap();
    let events: Vec<_> = api
        .stream_response_envelopes(&CreateResponseRequest::new_text("model_fixture", "hello"))
        .await
        .unwrap()
        .collect()
        .await;
    assert_eq!(events.len(), 1);
    let event = events.into_iter().next().unwrap().unwrap();
    assert_eq!(event.payload, payload);
    assert_eq!(event.sse_id, "frame_1");
    assert_eq!(event.sse_event, "response.output_text.delta");
    assert_eq!(event.sse_retry, Some(std::time::Duration::from_secs(3)));
    mock.assert_async().await;
}

fn mock_responses_http_errors(server: &MockServer) {
    let paths = [
        "/v1/responses/resp_fixture",
        "/v1/responses",
        "/v1/responses/resp_fixture/input_items",
    ];
    for path in paths {
        server.mock(|when, then| {
            when.method(GET).path(path);
            then.status(403).json_body(
                json!({"error":{"message":"fixture forbidden","type":"permission_error"}}),
            );
        });
    }
    server.mock(|when, then| {
        when.method(POST).path("/v1/responses");
        then.status(502).body("fixture upstream unavailable");
    });
}

#[tokio::test]
async fn response_reads_and_streaming_preserve_http_status_errors() {
    let server = MockServer::start_async().await;
    mock_responses_http_errors(&server);
    let api =
        ResponsesApiV2::new_with_base_url("fixture-key".to_owned(), server.base_url()).unwrap();
    assert!(matches!(
        api.retrieve_response("resp_fixture", None).await,
        Err(OpenAIError::Api {
            status_code: 403,
            ..
        })
    ));
    assert!(matches!(
        api.list_responses(&ListResponsesParams::default()).await,
        Err(OpenAIError::Api {
            status_code: 403,
            ..
        })
    ));
    assert!(matches!(
        api.list_response_input_items("resp_fixture", &StandardListParams::default())
            .await,
        Err(OpenAIError::Api {
            status_code: 403,
            ..
        })
    ));
    assert!(matches!(
        api.stream_response(&CreateResponseRequest::new_text("model_fixture", "hello"))
            .await,
        Err(OpenAIError::ApiError { status: 502, .. })
    ));
}

#[test]
fn shared_query_builder_percent_encodes_values_and_keys() {
    let client = openai_rust_sdk::api::base::HttpClient::new("fixture-key").unwrap();
    let url = client.build_url(
        "/v1/responses",
        &[
            ("after".into(), "cursor&model=injected ?#".into()),
            ("custom key".into(), "a+b".into()),
        ],
    );
    let parsed = url::Url::parse(&url).unwrap();
    let pairs: Vec<_> = parsed.query_pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].1, "cursor&model=injected ?#");
    assert_eq!(pairs[1].0, "custom key");
    assert_eq!(pairs[1].1, "a+b");
    assert!(parsed.fragment().is_none());
}
