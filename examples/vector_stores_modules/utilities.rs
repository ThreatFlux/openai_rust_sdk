//! Utility Functions and Helpers
//!
//! This module contains utility functions, initialization, cleanup, and workflow orchestration.

use openai_rust_sdk::api::{
    common::ApiClientConstructors, files::FilesApi, vector_stores::VectorStoresApi,
};
use openai_rust_sdk::error::Result;

/// Helper function to format bytes in human-readable format
pub fn format_bytes(bytes: u64) -> String {
    let bytes = bytes as f64;
    if bytes < 1024.0 {
        format!("{bytes} B")
    } else if bytes < 1024.0 * 1024.0 {
        format!("{:.1} KB", bytes / 1024.0)
    } else if bytes < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} MB", bytes / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Initialize API clients and print startup message
pub async fn initialize_apis(api_key: &str) -> Result<(VectorStoresApi, FilesApi)> {
    println!("🚀 OpenAI Vector Stores API Demo");
    println!("=================================\n");

    let vector_stores_api = VectorStoresApi::new(api_key)?;
    let files_api = FilesApi::new(api_key)?;

    println!("✅ Initialized Vector Stores and Files APIs\n");
    Ok((vector_stores_api, files_api))
}

/// Create basic and expiring vector stores
pub async fn create_demo_stores(
    vector_stores_api: &VectorStoresApi,
) -> Result<(
    openai_rust_sdk::models::vector_stores::VectorStore,
    openai_rust_sdk::models::vector_stores::VectorStore,
)> {
    use crate::vector_stores_modules::core_demos::{
        demo_basic_vector_store, demo_expiring_vector_store,
    };

    let basic_store = demo_basic_vector_store(vector_stores_api).await?;
    let expiring_store = demo_expiring_vector_store(vector_stores_api).await?;
    Ok((basic_store, expiring_store))
}

/// Run the complete demo workflow
pub async fn run_demo_workflow(
    vector_stores_api: &VectorStoresApi,
    _files_api: &FilesApi,
    basic_store: &openai_rust_sdk::models::vector_stores::VectorStore,
    uploaded_file_ids: &[String],
) -> Result<openai_rust_sdk::models::vector_stores::VectorStore> {
    use crate::vector_stores_modules::advanced_features::demo_advanced_features;
    use crate::vector_stores_modules::file_operations::demo_file_operations;
    use crate::vector_stores_modules::management::demo_store_management;

    demo_file_operations(vector_stores_api, &basic_store.id, uploaded_file_ids).await?;
    let _updated_store = demo_store_management(vector_stores_api, &basic_store.id).await?;
    let advanced_store = demo_advanced_features(vector_stores_api, uploaded_file_ids).await?;
    Ok(advanced_store)
}

/// Demo 13: Cleanup
pub async fn demo_cleanup(
    files_api: &FilesApi,
    vector_stores_api: &VectorStoresApi,
    file_ids: &[String],
    store_ids: &[&str],
) -> Result<()> {
    println!("🧹 Demo 13: Cleanup");
    println!("-------------------");

    for file_id in file_ids {
        match files_api.delete_file(file_id).await {
            Ok(delete_response) if delete_response.deleted => {
                println!("✅ Deleted file: {file_id}");
            }
            Ok(_) => println!("⚠️ File deletion not confirmed: {file_id}"),
            Err(e) => println!("❌ Failed to delete file {file_id}: {e}"),
        }
    }

    for store_id in store_ids {
        match vector_stores_api.delete_vector_store(*store_id).await {
            Ok(delete_response) if delete_response.deleted => {
                println!("✅ Deleted vector store: {store_id}");
            }
            Ok(_) => println!("⚠️ Vector store deletion not confirmed: {store_id}"),
            Err(e) => println!("❌ Failed to delete vector store {store_id}: {e}"),
        }
    }

    println!("\n🎉 Vector Stores API Demo Complete!");
    println!("===================================");
    println!("Summary of demonstrated features:");
    println!("• Vector store creation with various configurations");
    println!("• File upload and attachment (individual and batch)");
    println!("• Processing status monitoring with timeouts");
    println!("• File listing and filtering by status");
    println!("• Vector store management and updates");
    println!("• Usage statistics and monitoring");
    println!("• Comprehensive error handling");
    println!("• Advanced features (expiration, chunking, metadata)");
    println!("• Proper cleanup and resource management");
    println!("\nFor production use, consider:");
    println!("• Implementing retry logic for transient failures");
    println!("• Using structured logging for monitoring");
    println!("• Setting up alerts for processing failures");
    println!("• Implementing proper pagination for large datasets");
    println!("• Adding input validation and sanitization");

    Ok(())
}

/// Clean up all created resources
pub async fn cleanup_resources(
    files_api: &FilesApi,
    vector_stores_api: &VectorStoresApi,
    uploaded_file_ids: &[String],
    basic_store: &openai_rust_sdk::models::vector_stores::VectorStore,
    expiring_store: &openai_rust_sdk::models::vector_stores::VectorStore,
    advanced_store: &openai_rust_sdk::models::vector_stores::VectorStore,
) -> Result<()> {
    let store_ids = vec![
        basic_store.id.as_str(),
        expiring_store.id.as_str(),
        advanced_store.id.as_str(),
    ];
    demo_cleanup(files_api, vector_stores_api, uploaded_file_ids, &store_ids).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use openai_rust_sdk::models::vector_stores::{
        ChunkingStrategy, ExpirationPolicy, VectorStoreRequest,
    };
    use std::collections::HashMap;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1_048_576), "1.0 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.0 GB");
    }

    #[test]
    fn test_demo_configuration() {
        let basic_request = VectorStoreRequest::builder()
            .name("Demo Knowledge Base")
            .add_metadata("purpose", "demo")
            .build();

        assert_eq!(basic_request.name, Some("Demo Knowledge Base".to_string()));
        assert!(basic_request.metadata.is_some());

        let expiring_request = VectorStoreRequest::builder()
            .name("Temporary Knowledge Base")
            .expires_after(ExpirationPolicy::new_days(7))
            .chunking_strategy(ChunkingStrategy::static_chunking(512, 50))
            .build();

        assert!(expiring_request.expires_after.is_some());
        assert!(expiring_request.chunking_strategy.is_some());
    }

    #[test]
    fn test_sample_file_content() {
        let sample_files = vec![
            (
                "knowledge_base_1.txt",
                "This is the first knowledge base document.",
            ),
            (
                "knowledge_base_2.txt",
                "This is the second knowledge base document.",
            ),
        ];

        for (filename, content) in sample_files {
            assert!(!filename.is_empty());
            assert!(!content.is_empty());
            assert!(filename.ends_with(".txt"));
        }
    }

    #[test]
    fn test_demo_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("project".to_string(), "demo".to_string());
        metadata.insert("version".to_string(), "1.0".to_string());
        metadata.insert("environment".to_string(), "development".to_string());

        assert_eq!(metadata.len(), 3);
        assert_eq!(metadata.get("project"), Some(&"demo".to_string()));
        assert_eq!(metadata.get("version"), Some(&"1.0".to_string()));
        assert_eq!(
            metadata.get("environment"),
            Some(&"development".to_string())
        );
    }
}
