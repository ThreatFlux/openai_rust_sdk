/// Macros for reducing builder pattern duplication
#[macro_use]
pub mod macros;

/// Function builder for creating function definitions
pub mod function_builder;

pub use function_builder::*;
