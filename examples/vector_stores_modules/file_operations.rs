//! File Operations for Vector Stores
//!
//! This module contains demonstrations of file upload, batch operations, and monitoring.

use openai_rust_sdk::api::{files::FilesApi, vector_stores::VectorStoresApi};
use openai_rust_sdk::error::Result;
use openai_rust_sdk::models::files::{FilePurpose, FileUploadRequest};
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ListVectorStoreFilesParams, VectorStoreFileBatchRequest,
    VectorStoreFileRequest, VectorStoreFileStatus,
};
use std::time::Duration;
use tokio::time;

/// Demo 3: Uploading Files for Vector Store
pub async fn demo_file_upload(files_api: &FilesApi) -> Result<Vec<String>> {
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

/// Demo 4: Adding Files to Vector Store (Individual)
pub async fn demo_individual_file_add(
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

/// Demo 5: Batch File Upload
pub async fn demo_batch_upload(
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

/// Demo 6: Monitoring Batch Processing
pub async fn demo_monitor_batch_processing(
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

/// Demo 7: Listing and Filtering Files
pub async fn demo_list_filter_files(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
) -> Result<()> {
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

/// Combined file operations demo
pub async fn demo_file_operations(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
    file_ids: &[String],
) -> Result<()> {
    demo_individual_file_add(vector_stores_api, store_id, file_ids).await?;
    demo_batch_upload(vector_stores_api, store_id, file_ids).await?;
    demo_list_filter_files(vector_stores_api, store_id).await?;
    Ok(())
}
