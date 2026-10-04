//! Extensible delivery envelopes.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{WebhookError, WebhookResult};

/// An event envelope, with extensible event type and data fields.
///
/// Resource-specific payloads remain JSON objects. New event types and nested
/// fields survive parsing and serialization without a new SDK release.
/// Direct serde deserialization performs no signature verification; use
/// `WebhookVerifier::unwrap` for authenticated incoming deliveries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookEvent {
    /// The provider event identifier.
    pub id: String,
    /// The object discriminator, `event`.
    pub object: String,
    #[serde(rename = "type")]
    /// The event type, including future provider types.
    pub event_type: String,
    /// Event creation time in Unix seconds.
    pub created_at: u64,
    /// Resource-specific data and nested extension fields.
    pub data: Map<String, Value>,
    #[serde(flatten)]
    /// Additional event envelope fields.
    pub extra: BTreeMap<String, Value>,
}

impl WebhookEvent {
    /// Parse a common envelope after authentication.
    pub(super) fn from_payload(body: &[u8]) -> WebhookResult<Self> {
        let event: Self = serde_json::from_slice(body).map_err(|_| WebhookError::InvalidPayload)?;
        if event.id.is_empty() || event.event_type.is_empty() || event.object != "event" {
            return Err(WebhookError::InvalidPayload);
        }
        Ok(event)
    }

    /// The referenced resource ID, when supplied by this event type.
    pub fn resource_id(&self) -> Option<&str> {
        self.data.get("id").and_then(Value::as_str)
    }
}
