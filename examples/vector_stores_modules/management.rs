//! Vector Store Management Operations
//!
//! This module contains demonstrations of vector store management, listing, statistics, and error handling.

use crate::vector_stores_modules::utilities::format_bytes;
use openai_rust_sdk::api::vector_stores::VectorStoresApi;
use openai_rust_sdk::error::{OpenAIError, Result};
use openai_rust_sdk::models::vector_stores::{ListVectorStoresParams, VectorStoreRequest};

/// Demo 8: Vector Store Management
pub async fn demo_vector_store_management(
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

/// Demo 9: Listing All Vector Stores
pub async fn demo_list_vector_stores(vector_stores_api: &VectorStoresApi) -> Result<()> {
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

/// Demo 10: Usage Statistics
pub async fn demo_usage_statistics(vector_stores_api: &VectorStoresApi) -> Result<()> {
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

/// Demo 11: Error Handling Examples
pub async fn demo_error_handling(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
) -> Result<()> {
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

/// Combined store management demo
pub async fn demo_store_management(
    vector_stores_api: &VectorStoresApi,
    store_id: &str,
) -> Result<openai_rust_sdk::models::vector_stores::VectorStore> {
    let updated_store = demo_vector_store_management(vector_stores_api, store_id).await?;
    demo_list_vector_stores(vector_stores_api).await?;
    demo_usage_statistics(vector_stores_api).await?;
    demo_error_handling(vector_stores_api, &updated_store.id).await?;
    Ok(updated_store)
}
