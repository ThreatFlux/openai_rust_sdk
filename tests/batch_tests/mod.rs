#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive tests for the Batch API
//!
//! This test suite covers all functionality of the Batch API including:
//! - Batch creation and management
//! - File upload for batch processing  
//! - Status monitoring and polling
//! - Result retrieval and processing
//! - YARA rule extraction from responses
//! - Error handling and edge cases
//! - Batch cancellation and cleanup
//! - Report generation and analysis

pub mod api_creation_tests;
pub mod data_structure_tests;
pub mod edge_cases_tests;
pub mod error_handling_tests;
pub mod metadata_tests;
pub mod report_tests;
pub mod serialization_tests;
pub mod status_tests;
pub mod test_data_helpers;
pub mod timestamps_tests;
pub mod validation_tests;
pub mod yara_extraction_tests;

// Re-export test helper functions
pub use test_data_helpers::*;
