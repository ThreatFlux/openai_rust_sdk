//! Lossless events preserve fields absent from the compatibility event enum.

use super::ResponseStreamEvent;
use crate::error::{OpenAIError, Result};
use serde_json::{Map, Value};

/// A typed Responses event together with its original wire payload.
///
/// The compatibility enum's `Unknown` remains a unit variant. Inspect `payload`
/// for future event types and additional fields on known events. Known malformed
/// events are errors rather than being converted to `Unknown`.
#[derive(Debug, Clone)]
pub struct ResponseStreamEnvelope {
    /// Parsed compatibility event.
    pub event: ResponseStreamEvent,
    /// Complete original JSON event, including unknown properties.
    pub payload: Value,
    /// JSON `type` discriminator.
    pub event_type: String,
    /// Sequence number when supplied by the service.
    pub sequence_number: Option<u64>,
    /// SSE event name, which can differ from the JSON discriminator.
    pub sse_event: String,
    /// Last event ID delivered by the SSE framing layer.
    pub sse_id: String,
    /// Suggested SSE reconnection interval. No automatic reconnect is performed.
    pub sse_retry: Option<std::time::Duration>,
}

impl ResponseStreamEnvelope {
    /// Parse a JSON event without an HTTP transport.
    pub fn from_payload(payload: Value) -> Result<Self> {
        let event_type = payload
            .get("type")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| OpenAIError::streaming("Responses event has no type"))?
            .to_owned();
        let sequence_number = parse_sequence_number(&payload)?;
        let normalized = normalize_error(&event_type, &payload)?;
        let event = serde_json::from_value(normalized)?;
        Ok(Self {
            event,
            payload,
            event_type,
            sequence_number,
            sse_event: String::new(),
            sse_id: String::new(),
            sse_retry: None,
        })
    }

    /// Attach transport metadata to a decoded JSON event.
    pub(crate) fn from_sse(event: eventsource_stream::Event) -> Result<Self> {
        let mut envelope = Self::from_payload(serde_json::from_str(&event.data)?)?;
        envelope.sse_event = event.event;
        envelope.sse_id = event.id;
        envelope.sse_retry = event.retry;
        Ok(envelope)
    }
}

/// Accept absent sequence metadata, but reject an invalid supplied value.
fn parse_sequence_number(payload: &Value) -> Result<Option<u64>> {
    match payload.get("sequence_number") {
        None => Ok(None),
        Some(value) => value
            .as_u64()
            .map(Some)
            .ok_or_else(|| OpenAIError::streaming("Invalid Responses event sequence number")),
    }
}

/// Translate current flat errors while preserving the untouched original payload.
fn normalize_error(event_type: &str, payload: &Value) -> Result<Value> {
    if event_type != "error" {
        return Ok(payload.clone());
    }
    if let Some(error) = payload.get("error") {
        if error.get("message").and_then(Value::as_str).is_none() {
            return Err(OpenAIError::streaming(
                "Responses error event has no message",
            ));
        }
        return Ok(payload.clone());
    }
    if payload.get("message").and_then(Value::as_str).is_none() {
        return Err(OpenAIError::streaming(
            "Responses error event has no message",
        ));
    }
    let mut normalized = payload.clone();
    let mut error = Map::new();
    for field in ["code", "message", "param"] {
        if let Some(value) = payload.get(field) {
            error.insert(field.into(), value.clone());
        }
    }
    normalized["error"] = Value::Object(error);
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eventsource_stream::Eventsource;
    use futures::StreamExt;
    use serde_json::json;

    #[tokio::test]
    async fn split_utf8_crlf_and_multiline_data_preserve_the_complete_event() {
        let payload = json!({"type":"response.future_fixture","sequence_number":1,"text":"🦀"});
        let raw = "event: response.future_fixture\r\nid: split_fixture\r\ndata: {\r\ndata: \"type\":\"response.future_fixture\",\r\ndata: \"sequence_number\":1,\"text\":\"🦀\"}\r\n\r\n";
        // One byte per chunk splits every multi-byte character and CRLF boundary.
        let chunks = raw
            .bytes()
            .map(|byte| Ok::<_, std::io::Error>(bytes::Bytes::from(vec![byte])));
        let mut stream = futures::stream::iter(chunks).eventsource();
        let event = stream.next().await.unwrap().unwrap();
        let envelope = ResponseStreamEnvelope::from_sse(event).unwrap();
        assert_eq!(envelope.payload, payload);
        assert_eq!(envelope.sse_id, "split_fixture");
        assert!(stream.next().await.is_none());
    }
}
