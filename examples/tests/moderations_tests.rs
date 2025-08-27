//! Moderations API test module

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, moderations::ModerationsApi},
    error::Result,
    models::moderations::ModerationRequest,
};

pub async fn run_moderations_api_test(api_key: &str) -> Result<()> {
    println!("\n🛡️ Test 7: Moderations API");
    println!("{}", "-".repeat(70));
    test_moderations_api(api_key).await
}

async fn test_moderations_api(api_key: &str) -> Result<()> {
    let api = ModerationsApi::new(api_key)?;

    test_safe_content_moderation(&api).await?;
    test_edge_case_moderation(&api).await?;

    Ok(())
}

async fn test_safe_content_moderation(api: &ModerationsApi) -> Result<()> {
    println!("   ✅ Testing with safe content...");
    let safe_request =
        ModerationRequest::new("This is a friendly message about science and technology.")
            .with_model("omni-moderation-latest");

    match api.create_moderation(&safe_request).await {
        Ok(response) => {
            if let Some(result) = response.results.first() {
                println!("   ✅ Safe content moderation complete");
                println!("      Flagged: {}", result.flagged);
                if !result.flagged {
                    println!("      Content is safe ✓");
                }
            }
        }
        Err(e) => {
            println!("   ❌ Safe content moderation failed: {e}");
        }
    }

    Ok(())
}

async fn test_edge_case_moderation(api: &ModerationsApi) -> Result<()> {
    println!("   ⚠️ Testing with edge case content...");
    let edge_request =
        ModerationRequest::new("I hate when my computer crashes and I lose all my work!")
            .with_model("omni-moderation-latest");

    match api.create_moderation(&edge_request).await {
        Ok(response) => {
            if let Some(result) = response.results.first() {
                println!("   ✅ Edge case moderation complete");
                println!("      Flagged: {}", result.flagged);
                if result.flagged {
                    println!("      Categories: {:?}", result.categories);
                } else {
                    println!("      Content passed moderation");
                }
            }
        }
        Err(e) => {
            println!("   ❌ Edge case moderation failed: {e}");
        }
    }

    Ok(())
}
