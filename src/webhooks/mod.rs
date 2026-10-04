//! Verify webhook deliveries before parsing their exact raw request bodies.
//!
//! This module makes no API requests and does not need an OpenAI API key.
//! Pass the original body bytes and the three webhook header values from your
//! HTTP framework. Authenticate with `unwrap` before processing an event.
//! Timestamp validation limits replay age; applications still deduplicate
//! deliveries using `webhook-id` and persist work before acknowledging.
//!
//! See [OpenAI's webhook guide](https://developers.openai.com/api/docs/guides/webhooks).

mod error;
mod event;

use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

pub use error::WebhookError;
pub use event::WebhookEvent;

/// The result of offline webhook verification or parsing.
pub type WebhookResult<T> = std::result::Result<T, WebhookError>;

/// Bound each untrusted header before decoding.
const MAX_HEADER_BYTES: usize = 8 * 1024;
/// Default bound on body hashing and decoding.
const DEFAULT_MAX_BODY_BYTES: usize = 8 * 1024 * 1024;

/// Raw values of `webhook-id`, `webhook-timestamp` and `webhook-signature`.
///
/// Header names are case insensitive in HTTP; resolve them with your framework.
/// Join repeated signature header values with a space before constructing this
/// view. The signature may contain multiple space-separated `v1,<base64>` values.
#[derive(Clone, Copy)]
pub struct WebhookHeaders<'a> {
    /// Exact `webhook-id` value.
    pub id: &'a str,
    /// Exact `webhook-timestamp` value.
    pub timestamp: &'a str,
    /// Exact signature value or space-joined repeated values.
    pub signature: &'a str,
}

impl fmt::Debug for WebhookHeaders<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebhookHeaders")
            .field("id", &self.id)
            .field("timestamp", &self.timestamp)
            .field("signature", &"[REDACTED]")
            .finish()
    }
}

impl WebhookHeaders<'_> {
    /// Check required header values and size bounds.
    fn validate(&self) -> WebhookResult<()> {
        for (name, value) in [
            ("webhook-id", self.id),
            ("webhook-timestamp", self.timestamp),
            ("webhook-signature", self.signature),
        ] {
            if value.is_empty() {
                return Err(WebhookError::MissingHeader(name));
            }
            if value.len() > MAX_HEADER_BYTES {
                return Err(WebhookError::HeaderTooLarge);
            }
        }
        Ok(())
    }
}

/// An HMAC-SHA256 verifier with a five-minute timestamp tolerance by default.
///
/// Accepts an OpenAI `whsec_` secret containing Base64-encoded key bytes or a
/// plain-text key, matching the official SDK. Debug formatting redacts the key.
/// The default body limit is 8 MiB and each header is limited to 8 KiB.
#[derive(Clone)]
pub struct WebhookVerifier {
    /// Decoded signing key, redacted in Debug.
    key: Vec<u8>,
    /// Permitted absolute timestamp difference.
    tolerance: Duration,
    /// Bound on raw request bytes before hashing.
    max_body_bytes: usize,
}

impl fmt::Debug for WebhookVerifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebhookVerifier")
            .field("key", &"[REDACTED]")
            .field("tolerance", &self.tolerance)
            .field("max_body_bytes", &self.max_body_bytes)
            .finish()
    }
}

impl WebhookVerifier {
    /// Decode a configured signing key without reading environment variables.
    pub fn new(secret: &str) -> WebhookResult<Self> {
        let key = if let Some(encoded) = secret.strip_prefix("whsec_") {
            STANDARD
                .decode(encoded)
                .map_err(|_| WebhookError::InvalidSecret)?
        } else {
            secret.as_bytes().to_vec()
        };
        if key.is_empty() {
            return Err(WebhookError::InvalidSecret);
        }
        Ok(Self {
            key,
            tolerance: Duration::from_secs(300),
            max_body_bytes: DEFAULT_MAX_BODY_BYTES,
        })
    }

    /// Set the permitted timestamp difference, measured in whole seconds.
    pub fn with_tolerance(mut self, tolerance: Duration) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Set a positive maximum number of raw payload bytes.
    pub fn with_max_body_bytes(mut self, maximum: usize) -> WebhookResult<Self> {
        if maximum == 0 {
            return Err(WebhookError::InvalidBodyLimit);
        }
        self.max_body_bytes = maximum;
        Ok(self)
    }

    /// Authenticate the original raw body against the current system clock.
    pub fn verify(&self, body: &[u8], headers: WebhookHeaders<'_>) -> WebhookResult<()> {
        self.verify_at(body, headers, SystemTime::now())
    }

    /// Authenticate using an explicitly supplied trusted clock.
    ///
    /// This is useful for deterministic tests and server clock integrations.
    pub fn verify_at(
        &self,
        body: &[u8],
        headers: WebhookHeaders<'_>,
        now: SystemTime,
    ) -> WebhookResult<()> {
        if body.len() > self.max_body_bytes {
            return Err(WebhookError::BodyTooLarge);
        }
        headers.validate()?;
        self.validate_timestamp(headers.timestamp, now)?;
        let mac = self.signed_mac(body, headers)?;
        for candidate in headers.signature.split_whitespace() {
            let encoded = candidate.strip_prefix("v1,").unwrap_or(candidate);
            if let Ok(tag) = STANDARD.decode(encoded)
                && mac.clone().verify_slice(&tag).is_ok()
            {
                return Ok(());
            }
        }
        Err(WebhookError::InvalidSignature)
    }

    /// Reject malformed or stale delivery times.
    fn validate_timestamp(&self, timestamp: &str, now: SystemTime) -> WebhookResult<()> {
        if !timestamp.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(WebhookError::InvalidTimestamp);
        }
        let sent: u64 = timestamp
            .parse()
            .map_err(|_| WebhookError::InvalidTimestamp)?;
        let current = now
            .duration_since(UNIX_EPOCH)
            .map_err(|_| WebhookError::InvalidClock)?
            .as_secs();
        if current.abs_diff(sent) > self.tolerance.as_secs() {
            return Err(WebhookError::TimestampOutsideTolerance);
        }
        Ok(())
    }

    /// Hash provider-defined id.timestamp.body framing.
    fn signed_mac(&self, body: &[u8], headers: WebhookHeaders<'_>) -> WebhookResult<Hmac<Sha256>> {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&self.key).map_err(|_| WebhookError::InvalidSecret)?;
        mac.update(headers.id.as_bytes());
        mac.update(b".");
        mac.update(headers.timestamp.as_bytes());
        mac.update(b".");
        mac.update(body);
        Ok(mac)
    }

    /// Verify and parse an event using the current system clock.
    ///
    /// Pass raw bytes directly; parsing/reserializing before verification changes
    /// the signed message. This function performs no retrieval or acknowledgment.
    pub fn unwrap(&self, body: &[u8], headers: WebhookHeaders<'_>) -> WebhookResult<WebhookEvent> {
        self.unwrap_at(body, headers, SystemTime::now())
    }

    /// Verify and parse an event using an explicitly supplied trusted clock.
    pub fn unwrap_at(
        &self,
        body: &[u8],
        headers: WebhookHeaders<'_>,
        now: SystemTime,
    ) -> WebhookResult<WebhookEvent> {
        self.verify_at(body, headers, now)?;
        WebhookEvent::from_payload(body)
    }
}
