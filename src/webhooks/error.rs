//! Static, redacted verification diagnostics.

use thiserror::Error;

/// Errors contain no signing secret, signature, or customer payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum WebhookError {
    /// The configured secret cannot be decoded or is empty.
    #[error("Webhook signing secret is empty or malformed")]
    InvalidSecret,
    /// One of the required header values is empty.
    #[error("Missing webhook header: {0}")]
    MissingHeader(&'static str),
    /// A supplied header exceeds the verifier limit.
    #[error("Webhook header exceeds the configured size limit")]
    HeaderTooLarge,
    /// The timestamp is not an unsigned Unix second.
    #[error("Webhook timestamp must be an unsigned integer")]
    InvalidTimestamp,
    /// The delivery is too old or too far in the future.
    #[error("Webhook timestamp is outside the configured tolerance")]
    TimestampOutsideTolerance,
    /// None of the supplied signatures authenticated the body.
    #[error("Webhook signature did not match")]
    InvalidSignature,
    /// The raw body exceeds the configured limit.
    #[error("Webhook body exceeds the configured size limit")]
    BodyTooLarge,
    /// A body limit of zero was requested.
    #[error("Webhook body limit must be positive")]
    InvalidBodyLimit,
    /// The authenticated body is not a valid JSON event envelope.
    #[error("Webhook payload is not a valid event envelope")]
    InvalidPayload,
    /// The trusted clock predates Unix time.
    #[error("Webhook verification clock is before the Unix epoch")]
    InvalidClock,
}
