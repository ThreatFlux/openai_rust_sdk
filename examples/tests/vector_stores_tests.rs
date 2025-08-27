//! Vector Stores API test module

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, vector_stores::VectorStoresApi},
    error::Result,
    models::vector_stores::VectorStoreRequest,
};
use std::collections::HashMap;

pub async fn run_vector_stores_api_test(api_key: &str) -> Result<()> {
    println!("\n🗄️ Test 4: Vector Stores API");
    println!("{}", "-".repeat(70));
    test_vector_stores_api(api_key).await
}

async fn test_vector_stores_api(api_key: &str) -> Result<()> {
    let api = VectorStoresApi::new(api_key)?;

    println!("   🆕 Creating vector store...");
    let request = create_vector_store_request();

    match api.create_vector_store(request).await {
        Ok(store) => {
            println!("   ✅ Vector store created: {}", store.id);
            let store_id = store.id.clone();

            test_list_vector_stores(&api).await;
            test_retrieve_vector_store(&api, &store_id).await;
            test_delete_vector_store(&api, &store_id).await;
        }
        Err(e) => {
            println!("   ❌ Create vector store failed: {e}");
        }
    }

    Ok(())
}

fn create_vector_store_request() -> VectorStoreRequest {
    VectorStoreRequest {
        name: Some("Test Vector Store".to_string()),
        file_ids: Some(vec![]),
        expires_after: None,
        chunking_strategy: None,
        metadata: Some(HashMap::new()),
    }
}

async fn test_list_vector_stores(api: &VectorStoresApi) {
    println!("   📋 Listing vector stores...");
    match api.list_vector_stores(None).await {
        Ok(list) => {
            println!("   ✅ Found {} vector stores", list.data.len());
        }
        Err(e) => println!("   ❌ List vector stores failed: {e}"),
    }
}

async fn test_retrieve_vector_store(api: &VectorStoresApi, store_id: &str) {
    println!("   🔍 Retrieving vector store...");
    match api.retrieve_vector_store(store_id).await {
        Ok(retrieved) => {
            println!("   ✅ Vector store retrieved: {:?}", retrieved.name);
        }
        Err(e) => println!("   ❌ Retrieve vector store failed: {e}"),
    }
}

async fn test_delete_vector_store(api: &VectorStoresApi, store_id: &str) {
    println!("   🗑️ Deleting vector store...");
    match api.delete_vector_store(store_id).await {
        Ok(_) => {
            println!("   ✅ Vector store deleted successfully");
        }
        Err(e) => println!("   ❌ Delete vector store failed: {e}"),
    }
}
