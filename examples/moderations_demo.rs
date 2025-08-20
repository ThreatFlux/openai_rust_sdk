#![allow(clippy::pedantic, clippy::nursery)]
//! # Moderations API Demo
//!
//! This example demonstrates how to use the `OpenAI` Moderations API to classify
//! content according to `OpenAI`'s usage policies.

use openai_rust_sdk::{
    api::moderations::ModerationsApi,
    error::OpenAIError,
    models::moderations::{ModerationBuilder, SafetyThresholds},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), OpenAIError> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");

    // Create the moderations API client
    let moderations = ModerationsApi::new(api_key)?;

    println!("🛡️  OpenAI Moderations API Demo");
    println!("================================\n");

    // Example 1: Single text moderation
    println!("📝 Example 1: Single Text Moderation");
    println!("------------------------------------");

    let safe_text = "I love programming and creating helpful applications.";
    let result = moderations.moderate_text(safe_text).await?;

    println!("Text: \"{safe_text}\"");
    println!("Flagged: {}", result.flagged);

    if result.flagged {
        println!("Violations detected:");
        let violations = result.get_violations();
        for violation in violations {
            println!("  - {violation}");
        }
    } else {
        println!("✅ Content is safe");
    }
    println!();

    // Example 2: Batch text moderation
    println!("📚 Example 2: Batch Text Moderation");
    println!("-----------------------------------");

    let texts_to_moderate = vec![
        "I enjoy helping people learn new things.".to_string(),
        "The weather is beautiful today!".to_string(),
        "Let's work together on this project.".to_string(),
    ];

    let results = moderations
        .moderate_texts(texts_to_moderate.clone())
        .await?;

    for (i, (text, result)) in texts_to_moderate.iter().zip(results.iter()).enumerate() {
        println!("{}. \"{}\"", i + 1, text);
        println!("   Flagged: {}", result.flagged);
        if result.flagged {
            let violations = result.get_violations();
            println!("   Violations: {violations:?}");
        }
    }
    println!();

    // Example 3: Using different models
    println!("🔧 Example 3: Different Moderation Models");
    println!("----------------------------------------");

    let test_text = "This is a test of the moderation system.";

    // Using default model
    let default_result = moderations.moderate_text(test_text).await?;
    println!("Default model - Flagged: {}", default_result.flagged);

    // Using stable model
    let stable_result = moderations
        .moderate_text_with_model(test_text, "text-moderation-stable")
        .await?;
    println!("Stable model - Flagged: {}", stable_result.flagged);

    // Using latest model
    let latest_result = moderations
        .moderate_text_with_model(test_text, "text-moderation-latest")
        .await?;
    println!("Latest model - Flagged: {}", latest_result.flagged);
    println!();

    // Example 4: Custom thresholds
    println!("⚖️  Example 4: Custom Safety Thresholds");
    println!("-------------------------------------");

    let content = "Let's have a friendly conversation about technology.";

    // Check with conservative threshold
    let is_safe_conservative = moderations
        .is_safe_with_threshold(content, SafetyThresholds::CONSERVATIVE)
        .await?;
    println!(
        "Conservative (0.1): {}",
        if is_safe_conservative {
            "Safe"
        } else {
            "Flagged"
        }
    );

    // Check with moderate threshold
    let is_safe_moderate = moderations
        .is_safe_with_threshold(content, SafetyThresholds::MODERATE)
        .await?;
    println!(
        "Moderate (0.3): {}",
        if is_safe_moderate { "Safe" } else { "Flagged" }
    );

    // Check with permissive threshold
    let is_safe_permissive = moderations
        .is_safe_with_threshold(content, SafetyThresholds::PERMISSIVE)
        .await?;
    println!(
        "Permissive (0.7): {}",
        if is_safe_permissive {
            "Safe"
        } else {
            "Flagged"
        }
    );
    println!();

    // Example 5: Get violation details
    println!("📋 Example 5: Check Violations");
    println!("------------------------------------");

    let report_text = "Let's have a productive discussion about technology.";
    let violations = moderations.check_violations(report_text).await?;

    println!("Text: \"{report_text}\"");
    if violations.is_empty() {
        println!("✅ No violations detected");
    } else {
        println!("⚠️ Violations detected:");
        for violation in violations {
            println!("  - {violation}");
        }
    }
    println!();

    // Example 6: Get moderation scores
    println!("📊 Example 6: Get Moderation Scores");
    println!("---------------------------------");

    let test_text = "I love learning new things.";
    let scores = moderations.get_scores(test_text).await?;

    println!("Text: \"{test_text}\"");
    println!("Sexual: {:.4}", scores.sexual);
    println!("Hate: {:.4}", scores.hate);
    println!("Harassment: {:.4}", scores.harassment);
    println!("Self-harm: {:.4}", scores.self_harm);
    println!("Violence: {:.4}", scores.violence);
    println!("Max score: {:.4}", scores.max_score());
    println!();

    // Example 7: Moderate with details
    println!("🔍 Example 7: Moderate with Details");
    println!("-----------------------------------");

    let mixed_content = vec![
        "This is completely normal text.",
        "I need help with my project.",
        "Can we discuss this topic?",
        "Thank you for your assistance.",
    ];

    for text in mixed_content {
        let (is_flagged, violations) = moderations.moderate_with_details(text).await?;
        if is_flagged {
            println!("❌ {text}: Flagged - {violations:?}");
        } else {
            println!("✅ {text}: Safe");
        }
    }
    println!();

    // Example 8: Using ModerationBuilder for complex requests
    println!("🏗️  Example 8: Using ModerationBuilder");
    println!("------------------------------------");

    let builder_request = ModerationBuilder::new("I'm excited to learn about AI safety!")
        .model("text-moderation-latest")
        .build();

    let builder_result = moderations.create_moderation(&builder_request).await?;

    println!("Built request for: {:?}", builder_request.input);
    println!("Results count: {}", builder_result.results.len());

    for result in builder_result.results {
        println!("  Flagged: {}", result.flagged);
        if result.flagged {
            println!("  Violations: {:?}", result.get_violations());
        }
    }
    println!();

    // Example 9: Quick safety check
    println!("✅ Example 9: Quick Safety Check");
    println!("-------------------------------");

    let texts_to_check = vec![
        "This is a helpful message",
        "I appreciate your assistance",
        "Let's solve this problem together",
    ];

    for text in texts_to_check {
        let is_safe = moderations.is_safe(text).await?;
        println!(
            "\"{}\" - {}",
            text,
            if is_safe {
                "✅ Safe"
            } else {
                "⚠️ Flagged"
            }
        );
    }

    println!("\n🎉 Demo completed successfully!");

    Ok(())
}
