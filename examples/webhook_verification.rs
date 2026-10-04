//! Verify an incoming webhook body from stdin without an `OpenAI` API key.
//!
//! Set `OPENAI_WEBHOOK_SECRET`, `OPENAI_WEBHOOK_ID`, `OPENAI_WEBHOOK_TIMESTAMP`, and
//! `OPENAI_WEBHOOK_SIGNATURE` to the signing secret and original delivery headers.
//! Feed the exact raw HTTP body to stdin. No API requests or acknowledgments occur.

use std::io::{self, Read};

use openai_rust_sdk::webhooks::{WebhookHeaders, WebhookVerifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = required_env("OPENAI_WEBHOOK_SECRET")?;
    let id = required_env("OPENAI_WEBHOOK_ID")?;
    let timestamp = required_env("OPENAI_WEBHOOK_TIMESTAMP")?;
    let signature = required_env("OPENAI_WEBHOOK_SIGNATURE")?;
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

/// Strip decoding errors that can retain secret bytes in `VarError::NotUnicode`.
fn required_env(name: &str) -> io::Result<String> {
    std::env::var(name).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Set {name} to a valid Unicode value"),
        )
    })
}
