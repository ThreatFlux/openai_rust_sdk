//! # Vector Stores API Demo
//!
//! This example demonstrates the complete OpenAI Vector Stores API functionality,
//! including creating vector stores, managing files, batch operations, and
//! advanced features like expiration policies and chunking strategies.
//!
//! ## Features Demonstrated
//!
//! - Creating vector stores with various configurations
//! - Managing individual files and batch operations
//! - Monitoring processing status and file counts
//! - Using different chunking strategies for optimal embeddings
//! - Setting up expiration policies for automatic cleanup
//! - Error handling and best practices
//!
//! ## Prerequisites
//!
//! Set your OpenAI API key:
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! ```
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example vector_stores_demo
//! ```

use openai_rust_sdk::api::files::FilesApi;
use openai_rust_sdk::api::vector_stores::VectorStoresApi;
use openai_rust_sdk::error::{OpenAIError, Result};
use openai_rust_sdk::models::files::{FilePurpose, FileUploadRequest};
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ExpirationPolicy, ListVectorStoreFilesParams, ListVectorStoresParams,
    VectorStoreFileBatchRequest, VectorStoreFileRequest, VectorStoreFileStatus, VectorStoreRequest,
};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 OpenAI Vector Stores API Demo");
    println!("=================================\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        OpenAIError::Authentication("Please set OPENAI_API_KEY environment variable".to_string())
    })?;

    // Initialize APIs
    let vector_stores_api = VectorStoresApi::new(&api_key)?;
    let files_api = FilesApi::new(&api_key)?;

    println!("✅ Initialized Vector Stores and Files APIs\n");

    // Demo 1: Basic Vector Store Creation
    println!("📦 Demo 1: Creating a Basic Vector Store");
    println!("----------------------------------------");

    let basic_request = VectorStoreRequest::builder()
        .name("Demo Knowledge Base")
        .add_metadata("purpose", "demo")
        .add_metadata("created_by", "vector_stores_demo")
        .build();

    let basic_store = vector_stores_api.create_vector_store(basic_request).await?;
    println!("✅ Created vector store: {}", basic_store.id);
    println!("   Name: {}", basic_store.name.as_ref().unwrap());
    println!("   Status: {:?}", basic_store.status);
    println!("   Usage: {}\n", basic_store.usage_human_readable());

    // Demo 2: Vector Store with Expiration Policy
    println!("⏰ Demo 2: Vector Store with Expiration Policy");
    println!("----------------------------------------------");

    let expiring_request = VectorStoreRequest::builder()
        .name("Temporary Knowledge Base")
        .expires_after(ExpirationPolicy::new_days(7))
        .chunking_strategy(ChunkingStrategy::static_chunking(512, 50))
        .add_metadata("type", "temporary")
        .build();

    let expiring_store = vector_stores_api
        .create_vector_store(expiring_request)
        .await?;
    println!("✅ Created expiring vector store: {}", expiring_store.id);
    println!("   Name: {}", expiring_store.name.as_ref().unwrap());
    println!(
        "   Expires after: {} days",
        expiring_store.expires_after.as_ref().unwrap().days
    );
    println!("   Chunking strategy: Static (512 tokens, 50 overlap)\n");

    // Demo 3: Upload Files for Vector Store
    println!("📁 Demo 3: Uploading Files for Vector Store");
    println!("-------------------------------------------");

    // Create sample files (in real usage, these would be your actual documents)
    let sample_files = vec![
        (
            "knowledge_base_1.txt",
            "This is the first knowledge base document containing important information about AI and machine learning.",
        ),
        (
            "knowledge_base_2.txt",
            "This is the second knowledge base document with details about natural language processing and embeddings.",
        ),
        (
            "knowledge_base_3.txt",
            "This is the third knowledge base document covering vector databases and retrieval systems.",
        ),
    ];

    let mut uploaded_file_ids = Vec::new();

    for (filename, content) in sample_files {
        let upload_request = FileUploadRequest::new(
            content.as_bytes().to_vec(),
            filename.to_string(),
            FilePurpose::Assistants,
        );

        match files_api.upload_file(upload_request).await {
            Ok(file) => {
                println!("✅ Uploaded file: {} ({})", file.filename, file.id);
                uploaded_file_ids.push(file.id);
            }
            Err(e) => {
                println!("❌ Failed to upload {}: {}", filename, e);
            }
        }
    }

    println!("   Total files uploaded: {}\n", uploaded_file_ids.len());

    // Demo 4: Add Files to Vector Store (Individual)
    println!("🔗 Demo 4: Adding Files to Vector Store (Individual)");
    println!("----------------------------------------------------");

    if let Some(first_file_id) = uploaded_file_ids.first() {
        let file_request = VectorStoreFileRequest::new(first_file_id)
            .with_chunking_strategy(ChunkingStrategy::static_chunking(256, 25));

        let vector_store_file = vector_stores_api
            .create_vector_store_file(&basic_store.id, file_request)
            .await?;

        println!("✅ Added file to vector store: {}", vector_store_file.id);
        println!("   File ID: {}", first_file_id);
        println!("   Status: {:?}", vector_store_file.status);
        println!("   Usage: {} bytes\n", vector_store_file.usage_bytes);
    }

    // Demo 5: Batch File Upload
    println!("📦 Demo 5: Batch File Upload");
    println!("----------------------------");

    if uploaded_file_ids.len() > 1 {
        let batch_file_ids = uploaded_file_ids[1..].to_vec(); // Use remaining files
        let batch_request = VectorStoreFileBatchRequest::new(batch_file_ids.clone())
            .with_chunking_strategy(ChunkingStrategy::auto());

        let file_batch = vector_stores_api
            .create_vector_store_file_batch_with_request(&basic_store.id, batch_request)
            .await?;

        println!("✅ Created file batch: {}", file_batch.id);
        println!("   Files in batch: {}", file_batch.file_counts.total);
        println!("   Batch status: {:?}\n", file_batch.status);

        // Demo 6: Monitor Batch Processing
        println!("⏳ Demo 6: Monitoring Batch Processing");
        println!("-------------------------------------");

        // Poll batch status until completion (with timeout)
        let mut attempts = 0;
        const MAX_ATTEMPTS: u8 = 12; // 1 minute with 5-second intervals

        loop {
            let current_batch = vector_stores_api
                .retrieve_vector_store_file_batch(&basic_store.id, &file_batch.id)
                .await?;

            println!("   Batch status: {:?}", current_batch.status);
            println!(
                "   Progress: {}/{} completed, {}/{} failed",
                current_batch.file_counts.completed,
                current_batch.file_counts.total,
                current_batch.file_counts.failed,
                current_batch.file_counts.total
            );

            if current_batch.file_counts.is_processing_complete() {
                println!("✅ Batch processing completed!");
                println!(
                    "   Success rate: {:.1}%\n",
                    current_batch.file_counts.completion_percentage()
                );
                break;
            }

            attempts += 1;
            if attempts >= MAX_ATTEMPTS {
                println!("⏰ Timeout waiting for batch to complete\n");
                break;
            }

            time::sleep(Duration::from_secs(5)).await;
        }
    }

    // Demo 7: List and Filter Files
    println!("📋 Demo 7: Listing and Filtering Files");
    println!("--------------------------------------");

    // List all files in the vector store
    let all_files = vector_stores_api
        .list_vector_store_files(&basic_store.id, None)
        .await?;
    println!("✅ Total files in vector store: {}", all_files.data.len());

    // List only completed files
    let completed_params = ListVectorStoreFilesParams::new()
        .with_filter(VectorStoreFileStatus::Completed)
        .with_limit(10);

    let completed_files = vector_stores_api
        .list_vector_store_files(&basic_store.id, Some(completed_params))
        .await?;

    println!("   Completed files: {}", completed_files.data.len());
    println!(
        "   Total usage: {} bytes",
        completed_files.total_usage_bytes()
    );

    for file in &completed_files.data {
        println!("   - {} ({})", file.id, file.usage_bytes);
    }
    println!();

    // Demo 8: Vector Store Management
    println!("⚙️ Demo 8: Vector Store Management");
    println!("----------------------------------");

    // Update vector store metadata
    let update_request = VectorStoreRequest::builder()
        .name("Updated Demo Knowledge Base")
        .add_metadata("updated_at", chrono::Utc::now().to_rfc3339())
        .add_metadata("status", "production")
        .build();

    let updated_store = vector_stores_api
        .modify_vector_store(&basic_store.id, update_request)
        .await?;

    println!("✅ Updated vector store: {}", updated_store.id);
    println!("   New name: {}", updated_store.name.as_ref().unwrap());
    println!("   Metadata entries: {}\n", updated_store.metadata.len());

    // Demo 9: List All Vector Stores
    println!("🗂️ Demo 9: Listing All Vector Stores");
    println!("------------------------------------");

    let list_params = ListVectorStoresParams::new()
        .with_limit(10)
        .with_order("desc");

    let stores_list = vector_stores_api
        .list_vector_stores(Some(list_params))
        .await?;
    println!("✅ Found {} vector stores", stores_list.data.len());
    println!(
        "   Total usage: {}",
        format_bytes(stores_list.total_usage_bytes())
    );

    for store in &stores_list.data {
        println!(
            "   - {} ({}) - {} files, {}",
            store.name.as_ref().unwrap_or(&"Unnamed".to_string()),
            store.id,
            store.file_counts.total,
            store.usage_human_readable()
        );
    }
    println!();

    // Demo 10: Usage Statistics
    println!("📊 Demo 10: Usage Statistics");
    println!("----------------------------");

    let stats = vector_stores_api.get_usage_statistics().await?;
    println!("✅ Vector Store Statistics:");
    for (key, value) in &stats {
        println!("   {}: {}", key.replace('_', " ").to_uppercase(), value);
    }
    println!();

    // Demo 11: Error Handling Examples
    println!("🚨 Demo 11: Error Handling Examples");
    println!("-----------------------------------");

    // Try to retrieve a non-existent vector store
    match vector_stores_api
        .retrieve_vector_store("vs-nonexistent")
        .await
    {
        Ok(_) => println!("   Unexpected: Found non-existent vector store"),
        Err(OpenAIError::ApiError { status: 404, .. }) => {
            println!("✅ Correctly handled 404 error for non-existent vector store");
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    // Check if vector store exists (convenience method)
    let exists = vector_stores_api
        .vector_store_exists(&basic_store.id)
        .await?;
    println!("✅ Vector store exists check: {}", exists);

    let not_exists = vector_stores_api.vector_store_exists("vs-fake123").await?;
    println!("✅ Non-existent vector store check: {}\n", not_exists);

    // Demo 12: Advanced Features
    println!("🔬 Demo 12: Advanced Features");
    println!("-----------------------------");

    // Create a vector store with all features
    let advanced_request = VectorStoreRequest::builder()
        .name("Advanced Feature Demo")
        .file_ids(uploaded_file_ids.clone())
        .expires_after(ExpirationPolicy::new_with_anchor("created_at", 30))
        .chunking_strategy(ChunkingStrategy::static_chunking(1024, 128))
        .metadata({
            let mut meta = HashMap::new();
            meta.insert("project".to_string(), "demo".to_string());
            meta.insert("version".to_string(), "1.0".to_string());
            meta.insert("environment".to_string(), "development".to_string());
            meta
        })
        .build();

    let advanced_store = vector_stores_api
        .create_vector_store(advanced_request)
        .await?;
    println!("✅ Created advanced vector store: {}", advanced_store.id);
    println!("   Features enabled:");
    println!("   - Custom expiration policy (30 days from creation)");
    println!("   - Large chunk size (1024 tokens, 128 overlap)");
    println!("   - Rich metadata (3 entries)");
    println!("   - Pre-loaded with {} files\n", uploaded_file_ids.len());

    // Wait for processing to complete
    println!("⏳ Waiting for advanced vector store to be ready...");
    match vector_stores_api
        .wait_for_vector_store_ready(&advanced_store.id, Some(60), Some(5))
        .await
    {
        Ok(ready_store) => {
            println!("✅ Vector store is ready!");
            println!("   Status: {:?}", ready_store.status);
            println!(
                "   Files processed: {}/{}",
                ready_store.file_counts.completed, ready_store.file_counts.total
            );
        }
        Err(e) => {
            println!("⚠️ Timeout or error waiting for vector store: {}", e);
        }
    }
    println!();

    // Demo 13: Cleanup
    println!("🧹 Demo 13: Cleanup");
    println!("-------------------");

    // Delete uploaded files
    for file_id in &uploaded_file_ids {
        match files_api.delete_file(file_id).await {
            Ok(delete_response) if delete_response.deleted => {
                println!("✅ Deleted file: {}", file_id);
            }
            Ok(_) => println!("⚠️ File deletion not confirmed: {}", file_id),
            Err(e) => println!("❌ Failed to delete file {}: {}", file_id, e),
        }
    }

    // Delete vector stores
    let stores_to_delete = vec![&basic_store.id, &expiring_store.id, &advanced_store.id];

    for store_id in stores_to_delete {
        match vector_stores_api.delete_vector_store(store_id).await {
            Ok(delete_response) if delete_response.deleted => {
                println!("✅ Deleted vector store: {}", store_id);
            }
            Ok(_) => println!("⚠️ Vector store deletion not confirmed: {}", store_id),
            Err(e) => println!("❌ Failed to delete vector store {}: {}", store_id, e),
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

/// Helper function to format bytes in human-readable format
fn format_bytes(bytes: u64) -> String {
    let bytes = bytes as f64;
    if bytes < 1024.0 {
        format!("{} B", bytes)
    } else if bytes < 1024.0 * 1024.0 {
        format!("{:.1} KB", bytes / 1024.0)
    } else if bytes < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} MB", bytes / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes / (1024.0 * 1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1_048_576), "1.0 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.0 GB");
    }

    #[test]
    fn test_demo_configuration() {
        // Test that the demo configurations are valid
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
