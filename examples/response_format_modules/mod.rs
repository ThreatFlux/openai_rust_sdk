//! Response Format Demo Modules
//!
//! Modular structure for the response format demonstration

pub mod basic_demos;
pub mod builders;
pub mod error_handling;
pub mod schema_examples;
pub mod type_definitions;
pub mod validation;

/// Return the explicitly configured model used by the response-format examples.
pub(crate) fn example_model() -> std::io::Result<String> {
    let model = std::env::var("OPENAI_MODEL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "OPENAI_MODEL must be set to a non-empty model ID",
            )
        })?;

    Ok(model)
}
