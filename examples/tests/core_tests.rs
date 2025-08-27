//! Core API test utilities

use openai_rust_sdk::error::Result;
use std::env;

pub fn get_api_key() -> String {
    env::var("OPENAI_API_KEY").expect("Please set OPENAI_API_KEY environment variable")
}

pub fn print_test_header() {
    println!("\n🧪 Comprehensive OpenAI API Testing\n");
    println!("{}", "=".repeat(70));
}

pub fn print_test_footer() {
    println!("\n");
    println!("{}", "=".repeat(70));
    println!("✅ All API Tests Complete!");
    println!("{}", "=".repeat(70));
}

pub async fn run_core_api_tests(api_key: &str) -> Result<()> {
    crate::tests::files_tests::run_files_api_test(api_key).await?;
    crate::tests::moderations_tests::run_moderations_api_test(api_key).await?;
    Ok(())
}

pub async fn run_assistant_related_tests(api_key: &str) -> Result<()> {
    crate::tests::assistants_tests::run_assistants_api_test(api_key).await?;
    crate::tests::threads_tests::run_threads_api_test(api_key).await?;
    crate::tests::runs_tests::run_runs_api_test(api_key).await?;
    crate::tests::vector_stores_tests::run_vector_stores_api_test(api_key).await?;
    Ok(())
}

pub async fn run_specialized_api_tests(api_key: &str) -> Result<()> {
    crate::tests::fine_tuning_tests::run_fine_tuning_api_test(api_key).await?;
    crate::tests::audio_tests::run_audio_api_test(api_key).await?;
    Ok(())
}
