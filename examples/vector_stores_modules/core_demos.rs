//! Core Vector Store Demonstrations
//!
//! This module contains basic vector store creation and configuration demos.

use openai_rust_sdk::api::vector_stores::VectorStoresApi;
use openai_rust_sdk::error::Result;
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ExpirationPolicy, VectorStoreRequest,
};

/// Demo 1: Creating a Basic Vector Store
pub async fn demo_basic_vector_store(
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

/// Demo 2: Vector Store with Expiration Policy
pub async fn demo_expiring_vector_store(
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
