//! Verify an incoming webhook body from stdin without an `OpenAI` API key.
//!
//! Set `OPENAI_WEBHOOK_SECRET`, `OPENAI_WEBHOOK_ID`, `OPENAI_WEBHOOK_TIMESTAMP`, and
//! `OPENAI_WEBHOOK_SIGNATURE` to the signing secret and original delivery headers.
//! Feed the exact raw HTTP body to stdin. No API requests or acknowledgments occur.

use std::io::{self, Read};

use openai_rust_sdk::webhooks::{WebhookHeaders, WebhookVerifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = std::env::var("OPENAI_WEBHOOK_SECRET")?;
    let id = std::env::var("OPENAI_WEBHOOK_ID")?;
    let timestamp = std::env::var("OPENAI_WEBHOOK_TIMESTAMP")?;
    let signature = std::env::var("OPENAI_WEBHOOK_SIGNATURE")?;
    let mut body = Vec::new();
    io::stdin()
        .take(8 * 1024 * 1024 + 1)
        .read_to_end(&mut body)?;
    let verifier = WebhookVerifier::new(&secret)?;
    verifier.unwrap(
        &body,
        WebhookHeaders {
            id: &id,
            timestamp: &timestamp,
            signature: &signature,
        },
    )?;
    println!("Webhook authenticated");
    Ok(())
}
