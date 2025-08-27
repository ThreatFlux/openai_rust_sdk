#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive test of all OpenAI APIs
//!
//! Tests every API that requires authentication

use openai_rust_sdk::error::Result;

mod tests;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = tests::core_tests::get_api_key();
    tests::core_tests::print_test_header();

    tests::core_tests::run_core_api_tests(&api_key).await?;
    tests::core_tests::run_assistant_related_tests(&api_key).await?;
    tests::core_tests::run_specialized_api_tests(&api_key).await?;

    tests::core_tests::print_test_footer();
    Ok(())
}
