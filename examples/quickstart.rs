use openai_rust_sdk::{from_env, models::CreateResponseRequest};

const DEFAULT_MODEL: &str = "gpt-5.6-luna";

#[tokio::main]
async fn main() -> openai_rust_sdk::Result<()> {
    let client = from_env()?;
    let model = std::env::var("OPENAI_MODEL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_MODEL.to_owned());

    let request = CreateResponseRequest::new_text(
        model,
        "Explain Rust's ownership model in one concise sentence.",
    );
    let response = client.create_response_v2(&request).await?;

    println!("{}", response.output_text());
    Ok(())
}
