//! # Testing Module
//!
//! This module provides comprehensive testing and validation functionality for YARA rules
//! integrated with `OpenAI`'s Batch API.
//!
//! ## Sub-modules
//!
//! - [`yara_validator`](crate::testing::yara_validator): Core YARA rule validation using the yara-x engine
//! - [`test_cases`](crate::testing::test_cases): Pre-defined test cases and test suite management
//! - [`batch_generator`](crate::testing::batch_generator): `OpenAI` Batch API job generation and management
//!
//! ## Usage
//!
//! ```rust
//! use openai_rust_sdk::testing::{YaraValidator, YaraTestCases, BatchJobGenerator};
//!
//! // Validate a single rule
//! let validator = YaraValidator::new();
//! let result = validator.validate_rule("rule test { condition: true }")?;
//!
//! // Run a test suite
//! let test_cases = YaraTestCases::new();
//! let results = test_cases.run_all_tests()?;
//!
//! // Generate batch jobs
//! let generator = BatchJobGenerator::new(None);
//! generator.generate_test_suite(std::path::Path::new("batch.jsonl"), "basic")?;
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod batch_generator;
pub mod test_cases;
mod validator_helpers;
pub mod yara_validator;

pub use batch_generator::BatchJobGenerator;
pub use test_cases::YaraTestCases;
pub use yara_validator::YaraValidator;
