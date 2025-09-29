use futures::StreamExt;
use openai_rust_sdk::{
    client::OpenAIClient,
    models::{
        responses_v2::{CreateResponseRequest, ResponseStreamEvent},
        ResponsesApiServiceTier as ServiceTier,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("Set OPENAI_API_KEY environment variable");
    let client = OpenAIClient::new(api_key)?;

    // Basic response
    let request = CreateResponseRequest::new_text("gpt-4o-mini", "List three Rust advantages")
        .with_service_tier(ServiceTier::Auto);
    let response = client.create_response_v2(&request).await?;
    println!("Response: {}", response.output_text());

    // Streaming response
    let mut stream = client.stream_response_v2(&request).await?;
    println!("\nStreaming response:");
    while let Some(event) = stream.next().await {
        match event? {
            ResponseStreamEvent::OutputTextDelta { delta, .. } => print!("{delta}"),
            ResponseStreamEvent::ResponseCompleted { .. } => println!("\n[done]"),
            ResponseStreamEvent::StreamError { error, .. } => {
                eprintln!("stream error: {:?}", error.message);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
