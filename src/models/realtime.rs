//! Current REST models for ephemeral Realtime and transcription sessions.

use crate::{De, Ser};
use serde_json::Value;
use std::collections::HashMap;

/// Expiration configuration for an ephemeral Realtime client secret.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct ClientSecretExpiration {
    /// The expiration anchor. Currently this must be `created_at`.
    #[serde(default = "default_created_at")]
    pub anchor: String,
    /// Lifetime in seconds, between 10 and 7200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u64>,
}

/// Return the only currently supported expiration anchor.
fn default_created_at() -> String {
    "created_at".to_owned()
}

/// Request to create an ephemeral Realtime client secret.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct RealtimeClientSecretRequest {
    /// Optional expiration configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ClientSecretExpiration>,
    /// Realtime or transcription session configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Value>,
}

impl RealtimeClientSecretRequest {
    /// Create a request for a session configuration.
    #[must_use]
    pub fn new(session: Value) -> Self {
        Self {
            expires_after: None,
            session: Some(session),
        }
    }

    /// Set the ephemeral key lifetime in seconds.
    #[must_use]
    pub fn with_expiration_seconds(mut self, seconds: u64) -> Self {
        self.expires_after = Some(ClientSecretExpiration {
            anchor: default_created_at(),
            seconds: Some(seconds),
        });
        self
    }
}

/// Session data returned alongside an ephemeral client secret.
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeClientSecretSession {
    /// Session discriminator and configuration fields.
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// Response from the Realtime client-secret endpoint.
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeClientSecretResponse {
    /// Ephemeral client secret value.
    pub value: String,
    /// Unix expiration timestamp.
    pub expires_at: u64,
    /// Session configuration returned by the service.
    pub session: Value,
}

/// Current REST request for a transcription session.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct RealtimeTranscriptionSessionRequest {
    /// Server VAD configuration, or null to disable it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<Value>,
    /// Input audio noise reduction configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_noise_reduction: Option<Value>,
    /// Audio encoding (`pcm16`, `g711_ulaw`, or `g711_alaw`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<String>,
    /// Optional transcription model/language/prompt configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<Value>,
    /// Additional fields to include, such as transcription logprobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

/// Ephemeral secret returned for a transcription session.
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeSessionClientSecret {
    /// Ephemeral token.
    pub value: String,
    /// Unix expiration timestamp.
    pub expires_at: u64,
}

/// Response from the transcription-session endpoint.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct RealtimeTranscriptionSessionResponse {
    /// Ephemeral client secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<RealtimeSessionClientSecret>,
    /// Session expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    /// Any server fields added over time.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn client_secret_request_serializes_current_expiration_shape() {
        let request = RealtimeClientSecretRequest::new(json!({
            "type": "realtime",
            "model": "gpt-realtime"
        }))
        .with_expiration_seconds(600);

        let value = serde_json::to_value(request).expect("request JSON");
        assert_eq!(value["expires_after"]["anchor"], "created_at");
        assert_eq!(value["expires_after"]["seconds"], 600);
        assert_eq!(value["session"]["model"], "gpt-realtime");
    }
}
