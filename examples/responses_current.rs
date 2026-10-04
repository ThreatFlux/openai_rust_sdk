//! Current Responses options, input counting, lossless streaming, and compaction.
//!
//! Running this example makes API calls. Set `OPENAI_API_KEY` and an explicit
//! `OPENAI_MODEL` supporting the selected compaction/cache options first.

use futures::StreamExt;
use openai_rust_sdk::models::responses_v2::{
    CompactResponseRequest, ContextManagement, CreateResponseRequest, PromptCacheRetention,
    ResponseCreateOptions, ResponseInput,
};
use openai_rust_sdk::{OpenAIError, Result, from_env};

#[tokio::main]
async fn main() -> Result<()> {
    let model = std::env::var("OPENAI_MODEL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| OpenAIError::invalid_request("Set OPENAI_MODEL explicitly"))?;
    let client = from_env()?;
    let api = client.responses_v2();
    let request = CreateResponseRequest::new_text(&model, "Explain Rust ownership briefly.");
    let _count = api.count_input_tokens(&request).await?;
    let options = ResponseCreateOptions {
        context_management: Some(vec![ContextManagement::Compaction {
            compact_threshold: Some(12000),
        }]),
        prompt_cache_retention: Some(PromptCacheRetention::InMemory),
    };
    let mut stream = api
        .stream_response_envelopes_with_options(&request, &options)
        .await?;
    while let Some(event) = stream.next().await {
        let event = event?;
        // Application handlers can inspect event.event and persist event.payload.
        drop(event);
    }
    let compacted = api
        .compact_context(&CompactResponseRequest::new(
            &model,
            ResponseInput::Text("Context to preserve for a later turn.".into()),
        ))
        .await?;
    // Replay the entire output, including the opaque compaction item.
    let _next_request = CreateResponseRequest {
        model,
        input: ResponseInput::Raw(serde_json::to_value(compacted.output)?),
        ..Default::default()
    };
    println!("Responses operations completed.");
    Ok(())
}
