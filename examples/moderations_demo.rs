#![allow(clippy::pedantic, clippy::nursery)]
//! # Moderations API Demo
//!
//! This example demonstrates how to use the `OpenAI` Moderations API to classify
//! content according to `OpenAI`'s usage policies.

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, moderations::ModerationsApi},
    error::OpenAIError,
    models::moderations::{ModerationBuilder, SafetyThresholds},
};
use std::env;

/// Helper function to display safety status
fn display_safety_status(is_safe: bool) -> &'static str {
    if is_safe {
        "Safe"
    } else {
        "Flagged"
    }
}

/// Example 1: Single text moderation
async fn demo_single_text_moderation(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
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
    Ok(())
}

/// Example 2: Batch text moderation
async fn demo_batch_moderation(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
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
    Ok(())
}

/// Example 3: Using different models
async fn demo_different_models(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
    println!("🔧 Example 3: Different Moderation Models");
    println!("----------------------------------------");

    let test_text = "This is a test of the moderation system.";

    let default_result = moderations.moderate_text(test_text).await?;
    println!("Default model - Flagged: {}", default_result.flagged);

    let stable_result = moderations
        .moderate_text_with_model(test_text, "text-moderation-stable")
        .await?;
    println!("Stable model - Flagged: {}", stable_result.flagged);

    let latest_result = moderations
        .moderate_text_with_model(test_text, "text-moderation-latest")
        .await?;
    println!("Latest model - Flagged: {}", latest_result.flagged);
    println!();
    Ok(())
}

/// Example 4: Custom thresholds
async fn demo_custom_thresholds(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
    println!("⚖️  Example 4: Custom Safety Thresholds");
    println!("-------------------------------------");

    let content = "Let's have a friendly conversation about technology.";

    let is_safe_conservative = moderations
        .is_safe_with_threshold(content, SafetyThresholds::CONSERVATIVE)
        .await?;
    println!(
        "Conservative (0.1): {}",
        display_safety_status(is_safe_conservative)
    );

    let is_safe_moderate = moderations
        .is_safe_with_threshold(content, SafetyThresholds::MODERATE)
        .await?;
    println!(
        "Moderate (0.3): {}",
        display_safety_status(is_safe_moderate)
    );

    let is_safe_permissive = moderations
        .is_safe_with_threshold(content, SafetyThresholds::PERMISSIVE)
        .await?;
    println!(
        "Permissive (0.7): {}",
        display_safety_status(is_safe_permissive)
    );
    println!();
    Ok(())
}

/// Example 5: Get violation details
async fn demo_violation_details(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
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
    Ok(())
}

/// Example 6: Get moderation scores
async fn demo_moderation_scores(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
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
    Ok(())
}

/// Example 7: Moderate with details
async fn demo_moderate_with_details(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
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
    Ok(())
}

/// Example 8: Using ModerationBuilder for complex requests
async fn demo_moderation_builder(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
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
    Ok(())
}

/// Example 9: Quick safety check
async fn demo_quick_safety_check(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
    println!("✅ Example 9: Quick Safety Check");
    println!("-------------------------------");

    let texts_to_check = vec![
        "This is a helpful message",
        "I appreciate your assistance",
        "Let's solve this problem together",
    ];

    for text in texts_to_check {
        let is_safe = moderations.is_safe(text).await?;
        let status = if is_safe {
            "✅ Safe"
        } else {
            "⚠️ Flagged"
        };
        println!("\"{text}\" - {status}");
    }
    Ok(())
}

/// Run all demonstration examples in sequence using a macro to reduce cyclomatic complexity
async fn run_all_demos(moderations: &ModerationsApi) -> Result<(), OpenAIError> {
    // Use macro to eliminate branching and reduce cyclomatic complexity
    macro_rules! run_demos {
        ($($demo_fn:ident),*) => {
            $(
                $demo_fn(moderations).await?;
            )*
        };
    }

    // Execute all demos using the macro pattern
    run_demos!(
        demo_single_text_moderation,
        demo_batch_moderation,
        demo_different_models,
        demo_custom_thresholds,
        demo_violation_details,
        demo_moderation_scores,
        demo_moderate_with_details,
        demo_moderation_builder,
        demo_quick_safety_check
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), OpenAIError> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let moderations = ModerationsApi::new(api_key)?;

    println!("🛡️  OpenAI Moderations API Demo");
    println!("================================\n");

    run_all_demos(&moderations).await?;

    println!("\n🎉 Demo completed successfully!");
    Ok(())
}
