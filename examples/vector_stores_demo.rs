#![allow(clippy::pedantic, clippy::nursery)]
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

use openai_rust_sdk::api::{
    common::ApiClientConstructors, files::FilesApi, vector_stores::VectorStoresApi,
};
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

async fn demo_basic_vector_store(
    vector_stores_api: &VectorStoresApi,
) -> Result<openai_rust_sdk::models::vector_stores::VectorStore> {
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

    Ok(basic_store)
}

async fn demo_expiring_vector_store(
    vector_stores_api: &VectorStoresApi,
) -> Result<openai_rust_sdk::models::vector_stores::VectorStore> {
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

    Ok(expiring_store)
}

async fn demo_file_upload(files_api: &FilesApi) -> Result<Vec<String>> {
    println!("📁 Demo 3: Uploading Files for Vector Store");
    println!("-------------------------------------------");

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
                println!("❌ Failed to upload {filename}: {e}");
            }
        }
    }

    println!("   Total files uploaded: {}\n", uploaded_file_ids.len());
    Ok(uploaded_file_ids)
}

async fn demo_individual_file_add(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
    file_ids: &[String],
) -> Result<()> {
    println!("🔗 Demo 4: Adding Files to Vector Store (Individual)");
    println!("----------------------------------------------------");

    if let Some(first_file_id) = file_ids.first() {
        let file_request = VectorStoreFileRequest::new(first_file_id)
            .with_chunking_strategy(ChunkingStrategy::static_chunking(256, 25));

        let vector_store_file = vector_stores_api
            .create_vector_store_file(store_id, file_request)
            .await?;

        println!("✅ Added file to vector store: {}", vector_store_file.id);
        println!("   File ID: {first_file_id}");
        println!("   Status: {:?}", vector_store_file.status);
        println!("   Usage: {} bytes\n", vector_store_file.usage_bytes);
    }

    Ok(())
}

async fn demo_batch_upload(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
    file_ids: &[String],
) -> Result<()> {
    println!("📦 Demo 5: Batch File Upload");
    println!("----------------------------");

    if file_ids.len() > 1 {
        let batch_file_ids = file_ids[1..].to_vec();
        let batch_request = VectorStoreFileBatchRequest::new(batch_file_ids.clone())
            .with_chunking_strategy(ChunkingStrategy::auto());

        let file_batch = vector_stores_api
            .create_vector_store_file_batch_with_request(store_id, batch_request)
            .await?;

        println!("✅ Created file batch: {}", file_batch.id);
        println!("   Files in batch: {}", file_batch.file_counts.total);
        println!("   Batch status: {:?}\n", file_batch.status);

        demo_monitor_batch_processing(vector_stores_api, store_id, &file_batch.id).await?;
    }

    Ok(())
}

async fn demo_monitor_batch_processing(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
    batch_id: &str,
) -> Result<()> {
    println!("⏳ Demo 6: Monitoring Batch Processing");
    println!("-------------------------------------");

    let mut attempts = 0;
    const MAX_ATTEMPTS: u8 = 12;

    loop {
        let current_batch = vector_stores_api
            .retrieve_vector_store_file_batch(store_id, batch_id)
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

    Ok(())
}

async fn demo_list_filter_files(vector_stores_api: &VectorStoresApi, store_id: &str) -> Result<()> {
    println!("📋 Demo 7: Listing and Filtering Files");
    println!("--------------------------------------");

    let all_files = vector_stores_api
        .list_vector_store_files(store_id, None)
        .await?;
    println!("✅ Total files in vector store: {}", all_files.data.len());

    let completed_params = ListVectorStoreFilesParams::new()
        .with_filter(VectorStoreFileStatus::Completed)
        .with_limit(10);

    let completed_files = vector_stores_api
        .list_vector_store_files(store_id, Some(completed_params))
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

    Ok(())
}

async fn demo_vector_store_management(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
) -> Result<openai_rust_sdk::models::vector_stores::VectorStore> {
    println!("⚙️ Demo 8: Vector Store Management");
    println!("----------------------------------");

    let update_request = VectorStoreRequest::builder()
        .name("Updated Demo Knowledge Base")
        .add_metadata("updated_at", chrono::Utc::now().to_rfc3339())
        .add_metadata("status", "production")
        .build();

    let updated_store = vector_stores_api
        .modify_vector_store(store_id, update_request)
        .await?;

    println!("✅ Updated vector store: {}", updated_store.id);
    println!("   New name: {}", updated_store.name.as_ref().unwrap());
    println!("   Metadata entries: {}\n", updated_store.metadata.len());

    Ok(updated_store)
}

async fn demo_list_vector_stores(vector_stores_api: &VectorStoresApi) -> Result<()> {
    println!("🗺️ Demo 9: Listing All Vector Stores");
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

    Ok(())
}

async fn demo_usage_statistics(vector_stores_api: &VectorStoresApi) -> Result<()> {
    println!("📊 Demo 10: Usage Statistics");
    println!("----------------------------");

    let stats = vector_stores_api.get_usage_statistics().await?;
    println!("✅ Vector Store Statistics:");
    for (key, value) in &stats {
        println!("   {}: {}", key.replace('_', " ").to_uppercase(), value);
    }
    println!();

    Ok(())
}

async fn demo_error_handling(vector_stores_api: &VectorStoresApi, store_id: &str) -> Result<()> {
    println!("🚨 Demo 11: Error Handling Examples");
    println!("-----------------------------------");

    match vector_stores_api
        .retrieve_vector_store("vs-nonexistent")
        .await
    {
        Ok(_) => println!("   Unexpected: Found non-existent vector store"),
        Err(OpenAIError::ApiError { status: 404, .. }) => {
            println!("✅ Correctly handled 404 error for non-existent vector store");
        }
        Err(e) => println!("   Unexpected error: {e}"),
    }

    let exists = vector_stores_api.vector_store_exists(store_id).await?;
    println!("✅ Vector store exists check: {exists}");

    let not_exists = vector_stores_api.vector_store_exists("vs-fake123").await?;
    println!("✅ Non-existent vector store check: {not_exists}\n");

    Ok(())
}

async fn demo_advanced_features(
    vector_stores_api: &VectorStoresApi,
    file_ids: &[String],
) -> Result<openai_rust_sdk::models::vector_stores::VectorStore> {
    println!("🔬 Demo 12: Advanced Features");
    println!("-----------------------------");

    let advanced_request = VectorStoreRequest::builder()
        .name("Advanced Feature Demo")
        .file_ids(file_ids.to_vec())
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
    println!("   - Pre-loaded with {} files\n", file_ids.len());

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
            println!("⚠️ Timeout or error waiting for vector store: {e}");
        }
    }
    println!();

    Ok(advanced_store)
}

async fn demo_cleanup(
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

async fn demo_file_operations(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
    file_ids: &[String],
) -> Result<()> {
    demo_individual_file_add(vector_stores_api, store_id, file_ids).await?;
    demo_batch_upload(vector_stores_api, store_id, file_ids).await?;
    demo_list_filter_files(vector_stores_api, store_id).await?;
    Ok(())
}

async fn demo_store_management(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
) -> Result<openai_rust_sdk::models::vector_stores::VectorStore> {
    let updated_store = demo_vector_store_management(vector_stores_api, store_id).await?;
    demo_list_vector_stores(vector_stores_api).await?;
    demo_usage_statistics(vector_stores_api).await?;
    demo_error_handling(vector_stores_api, &updated_store.id).await?;
    Ok(updated_store)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 OpenAI Vector Stores API Demo");
    println!("=================================\n");

    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        OpenAIError::Authentication("Please set OPENAI_API_KEY environment variable".to_string())
    })?;

    let vector_stores_api = VectorStoresApi::new(&api_key)?;
    let files_api = FilesApi::new(&api_key)?;

    println!("✅ Initialized Vector Stores and Files APIs\n");

    let basic_store = demo_basic_vector_store(&vector_stores_api).await?;
    let expiring_store = demo_expiring_vector_store(&vector_stores_api).await?;
    let uploaded_file_ids = demo_file_upload(&files_api).await?;

    demo_file_operations(&vector_stores_api, &basic_store.id, &uploaded_file_ids).await?;
    let _updated_store = demo_store_management(&vector_stores_api, &basic_store.id).await?;
    let advanced_store = demo_advanced_features(&vector_stores_api, &uploaded_file_ids).await?;

    let store_ids = vec![
        basic_store.id.as_str(),
        expiring_store.id.as_str(),
        advanced_store.id.as_str(),
    ];
    demo_cleanup(
        &files_api,
        &vector_stores_api,
        &uploaded_file_ids,
        &store_ids,
    )
    .await?;

    Ok(())
}

/// Helper function to format bytes in human-readable format
fn format_bytes(bytes: u64) -> String {
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
