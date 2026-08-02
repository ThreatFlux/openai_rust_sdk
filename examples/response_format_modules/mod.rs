//! Response Format Demo Modules
//!
//! Modular structure for the response format demonstration

const DEFAULT_MODEL: &str = "gpt-5.6-luna";

pub mod basic_demos;
pub mod builders;
pub mod error_handling;
pub mod schema_examples;
pub mod type_definitions;
pub mod validation;

/// Return the model used by the response-format examples.
pub(crate) fn example_model() -> String {
    std::env::var("OPENAI_MODEL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_MODEL.to_owned())
}
