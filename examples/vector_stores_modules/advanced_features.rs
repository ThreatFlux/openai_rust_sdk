//! Advanced Vector Store Features
//!
//! This module demonstrates advanced vector store features including expiration policies,
//! chunking strategies, metadata management, and monitoring.

use openai_rust_sdk::api::vector_stores::VectorStoresApi;
use openai_rust_sdk::error::Result;
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ExpirationPolicy, VectorStoreRequest,
};
use std::collections::HashMap;

/// Demo 12: Advanced Features
pub async fn demo_advanced_features(
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
