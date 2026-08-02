//! # Batch Example Utilities
//!
//! This module supports the optional security-oriented Batch API case study. Batch JSONL
//! generation is always available; local YARA-X compilation and validation require the `yara`
//! feature. These helpers are not required to use the SDK's OpenAI API clients.
//!
//! ## Features
//!
//! When the `yara` feature is enabled, the example also provides:
//! - Core YARA rule validation using the yara-x engine
//! - Pre-defined test cases and test suite management
//!
//! Batch API job generation is available without optional features.
//!
//! ## Sub-modules
//!
//! - [`batch_generator`](crate::testing::batch_generator): `OpenAI` Batch API job generation

pub mod batch_generator;
pub mod prompts;

#[cfg(feature = "yara")]
pub mod test_cases;

#[cfg(feature = "yara")]
mod validator_helpers;

#[cfg(feature = "yara")]
pub mod yara_validator;

#[allow(unused_imports)]
pub use batch_generator::BatchJobGenerator;

#[cfg(feature = "yara")]
pub use test_cases::YaraTestCases;

#[cfg(feature = "yara")]
pub use yara_validator::YaraValidator;
