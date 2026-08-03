use openai_rust_sdk::{OpenAIError, from_env, models::CreateResponseRequest};

#[tokio::main]
async fn main() -> openai_rust_sdk::Result<()> {
    let client = from_env()?;
    let model = std::env::var("OPENAI_MODEL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            OpenAIError::InvalidRequest(
                "OPENAI_MODEL must be set to a non-empty model ID".to_owned(),
            )
        })?;

    let request = CreateResponseRequest::new_text(
        model,
        "Explain Rust's ownership model in one concise sentence.",
    );
    let response = client.create_response_v2(&request).await?;

    println!("{}", response.output_text());
    Ok(())
}
