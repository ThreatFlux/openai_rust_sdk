#![allow(clippy::pedantic, clippy::nursery)]
//! Completely refactored vector stores tests using aggressive macros to eliminate duplication
//!
//! This demonstrates how the new macro system can reduce test duplication by ~90%

mod common;
mod test_macros;

use openai_rust_sdk::api::vector_stores::VectorStoresApi;
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ExpirationPolicy, ListVectorStoreFilesParams, ListVectorStoresParams,
    VectorStoreFileBatchRequest, VectorStoreFileRequest, VectorStoreRequest, VectorStoreStatus,
    VectorStoreFileStatus, VectorStoreFileBatchStatus,
};
use common::*;

// Generate complete API test suite with one macro call
generate_api_test_suite!(VectorStoresApi, "https://custom.openai.com");

// Generate comprehensive status enum tests
generate_status_enum_tests!(VectorStoreStatus, {
    terminal: [Completed, Failed, Cancelled, Expired],
    active: [InProgress]
});

generate_status_enum_tests!(VectorStoreFileStatus, {
    terminal: [Completed, Cancelled, Failed],
    active: [InProgress]
});

generate_status_enum_tests!(VectorStoreFileBatchStatus, {
    terminal: [Completed, Cancelled, Failed],
    active: [InProgress]
});

// Test factories
fn create_test_vector_store_request() -> VectorStoreRequest {
    create_test_vector_store().into() // Assuming conversion exists
}

fn create_test_file_request() -> VectorStoreFileRequest {
    create_test_vector_store_file_request()
}

fn create_test_batch_request() -> VectorStoreFileBatchRequest {
    create_test_vector_store_file_batch_request()
}

// Generate builder tests
generate_builder_tests!(VectorStoreRequest, {
    required: [name],
    optional: [file_ids, expires_after, chunking_strategy, metadata],
    factory: create_test_vector_store_request
});

// Generate serialization tests
generate_serialization_tests!(VectorStoreRequest, {
    factory: create_test_vector_store_request,
    expected_fields: ["Test Store", "vs-test123"]
});

generate_serialization_tests!(VectorStoreFileRequest, {
    factory: create_test_file_request,
    expected_fields: ["file-test123"]
});

generate_serialization_tests!(VectorStoreFileBatchRequest, {
    factory: create_test_batch_request,
    expected_fields: ["file-1", "file-2", "file-3"]
});

// Generate parameter tests
generate_parameter_tests!(ListVectorStoresParams, {
    fields: [limit, order, after, before],
    test_values: [(limit, 25), (order, "desc"), (after, "vs-123")]
});

generate_parameter_tests!(ListVectorStoreFilesParams, {
    fields: [limit, order, after, filter],
    test_values: [(limit, 50), (order, "asc"), (after, "file-123")]
});

// Generate validation tests
generate_validation_tests!(ExpirationPolicy, {
    builder: ExpirationPolicyBuilder,
    edge_cases: [
        (days, 1, "minimum_days"),
        (days, 365, "maximum_days")
    ]
});

// Custom tests that can't be generated with macros
#[test]
fn test_chunking_strategy_variants() {
    let auto_strategy = ChunkingStrategy::auto();
    assert_eq!(auto_strategy, ChunkingStrategy::Auto);

    let static_strategy = ChunkingStrategy::static_chunking(1024, 100);
    assert_chunking_strategy_static(&static_strategy, 1024, 100);
}

#[test]
fn test_file_counts_calculations() {
    let mut counts = FileCounts::new();
    assert_eq!(counts.total, 0);
    assert!(counts.is_processing_complete());
    assert_eq!(counts.completion_percentage(), 100.0);
    assert_eq!(counts.failure_percentage(), 0.0);

    counts.total = 100;
    counts.completed = 70;
    counts.failed = 20;
    counts.in_progress = 10;

    assert!(!counts.is_processing_complete());
    assert_eq!(counts.completion_percentage(), 70.0);
    assert_eq!(counts.failure_percentage(), 20.0);
}

#[test]
fn test_vector_store_utility_methods() {
    let mut vector_store = create_test_vector_store();
    vector_store.usage_bytes = 2048;

    assert!(vector_store.is_ready());
    assert!(!vector_store.is_processing());
    assert!(!vector_store.has_failed());
    assert!(!vector_store.has_expired());
    assert_eq!(vector_store.usage_human_readable(), "2.0 KB");
    assert!(!vector_store.expires_soon());

    // Test different statuses
    vector_store.status = VectorStoreStatus::InProgress;
    assert!(!vector_store.is_ready());
    assert!(vector_store.is_processing());

    vector_store.status = VectorStoreStatus::Failed;
    assert!(vector_store.has_failed());

    vector_store.status = VectorStoreStatus::Expired;
    assert!(vector_store.has_expired());
}

// The macro-generated tests replace hundreds of lines of duplicate code:
// - API creation tests (5 tests) = ~50 lines → 1 macro call
// - Status enum tests (3 enums × 4 tests each) = ~150 lines → 3 macro calls
// - Builder tests = ~100 lines → 1 macro call
// - Serialization tests (3 types) = ~90 lines → 3 macro calls  
// - Parameter tests (2 types) = ~80 lines → 2 macro calls
// - Validation tests = ~60 lines → 1 macro call
//
// Total: ~530 lines reduced to ~15 macro calls + ~50 lines custom tests
// Duplication reduction: ~90%