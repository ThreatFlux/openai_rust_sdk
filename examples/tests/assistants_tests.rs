//! Assistants API test module

use openai_rust_sdk::{
    api::{assistants::AssistantsApi, common::ApiClientConstructors},
    error::Result,
    models::assistants::{AssistantRequest, ListAssistantsParams},
};
use std::collections::HashMap;

pub async fn run_assistants_api_test(api_key: &str) -> Result<()> {
    println!("\n🤖 Test 2: Assistants API");
    println!("{}", "-".repeat(70));
    test_assistants_api(api_key).await
}

async fn test_assistants_api(api_key: &str) -> Result<()> {
    let api = AssistantsApi::new(api_key)?;
    let request = create_assistant_request();

    println!("   🆕 Creating assistant...");
    match api.create_assistant(request.clone()).await {
        Ok(assistant) => {
            println!("   ✅ Assistant created: {}", assistant.id);
            let assistant_id = assistant.id.clone();

            test_list_assistants(&api).await;
            test_retrieve_assistant(&api, &assistant_id).await;
            test_update_assistant(&api, &assistant_id, request).await;
            test_delete_assistant(&api, &assistant_id).await;
        }
        Err(e) => println!("   ❌ Create assistant failed: {e}"),
    }
    Ok(())
}

fn create_assistant_request() -> AssistantRequest {
    AssistantRequest {
        model: "gpt-4-turbo-preview".to_string(),
        name: Some("Test Assistant".to_string()),
        description: Some("A test assistant for API testing".to_string()),
        instructions: Some("You are a helpful assistant.".to_string()),
        tools: vec![],
        file_ids: vec![],
        metadata: HashMap::new(),
    }
}

async fn test_list_assistants(api: &AssistantsApi) {
    println!("   📋 Listing assistants...");
    match api
        .list_assistants(Some(ListAssistantsParams::default()))
        .await
    {
        Ok(list) => println!("   ✅ Found {} assistants", list.data.len()),
        Err(e) => println!("   ❌ List assistants failed: {e}"),
    }
}

async fn test_retrieve_assistant(api: &AssistantsApi, assistant_id: &str) {
    println!("   🔍 Retrieving assistant...");
    match api.retrieve_assistant(assistant_id).await {
        Ok(retrieved) => println!("   ✅ Assistant retrieved: {:?}", retrieved.name),
        Err(e) => println!("   ❌ Retrieve assistant failed: {e}"),
    }
}

async fn test_update_assistant(
    api: &AssistantsApi,
    assistant_id: &str,
    mut request: AssistantRequest,
) {
    println!("   ✏️ Updating assistant...");
    request.name = Some("Updated Test Assistant".to_string());
    match api.modify_assistant(assistant_id, request).await {
        Ok(_) => println!("   ✅ Assistant updated successfully"),
        Err(e) => println!("   ❌ Update assistant failed: {e}"),
    }
}

async fn test_delete_assistant(api: &AssistantsApi, assistant_id: &str) {
    println!("   🗑️ Deleting assistant...");
    match api.delete_assistant(assistant_id).await {
        Ok(_) => println!("   ✅ Assistant deleted successfully"),
        Err(e) => println!("   ❌ Delete assistant failed: {e}"),
    }
}
