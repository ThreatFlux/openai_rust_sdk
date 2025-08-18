//! # Testing Module
//!
//! This module provides comprehensive testing and validation functionality.
//!
//! ## Features
//!
//! When the `yara` feature is enabled, this module provides YARA rule validation:
//! - Core YARA rule validation using the yara-x engine
//! - Pre-defined test cases and test suite management
//! - OpenAI Batch API job generation for YARA testing
//!
//! ## Sub-modules
//!
//! - [`batch_generator`](crate::testing::batch_generator): OpenAI Batch API job generation

pub mod batch_generator;

#[cfg(feature = "yara")]
pub mod test_cases;

#[cfg(feature = "yara")]
mod validator_helpers;

#[cfg(feature = "yara")]
pub mod yara_validator;

pub use batch_generator::BatchJobGenerator;

#[cfg(feature = "yara")]
pub use test_cases::YaraTestCases;

#[cfg(feature = "yara")]
pub use yara_validator::YaraValidator;
