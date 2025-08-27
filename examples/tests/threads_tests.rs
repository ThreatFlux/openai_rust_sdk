//! Threads API test module

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, threads::ThreadsApi},
    error::Result,
    models::threads::{MessageRequest, MessageRole, ThreadRequest},
};
use std::collections::HashMap;

pub async fn run_threads_api_test(api_key: &str) -> Result<()> {
    println!("\n🧵 Test 3: Threads API");
    println!("{}", "-".repeat(70));
    test_threads_api(api_key).await
}

async fn test_threads_api(api_key: &str) -> Result<()> {
    let api = ThreadsApi::new(api_key)?;

    println!("   🆕 Creating thread...");
    let request = ThreadRequest::default();

    match api.create_thread(request).await {
        Ok(thread) => {
            println!("   ✅ Thread created: {}", thread.id);
            let thread_id = thread.id.clone();

            test_add_message(&api, &thread_id).await;
            test_delete_thread(&api, &thread_id).await;
        }
        Err(e) => {
            println!("   ❌ Create thread failed: {e}");
        }
    }

    Ok(())
}

async fn test_add_message(api: &ThreadsApi, thread_id: &str) {
    println!("   💬 Adding message to thread...");
    let message = MessageRequest {
        role: MessageRole::User,
        content: "Hello, this is a test message!".to_string(),
        file_ids: vec![],
        metadata: HashMap::new(),
    };

    match api.create_message(thread_id, message).await {
        Ok(msg) => {
            println!("   ✅ Message added: {}", msg.id);
            test_list_messages(api, thread_id).await;
        }
        Err(e) => println!("   ❌ Add message failed: {e}"),
    }
}

async fn test_list_messages(api: &ThreadsApi, thread_id: &str) {
    println!("   📋 Listing messages...");
    match api.list_messages(thread_id, None).await {
        Ok(messages) => {
            println!("   ✅ Found {} messages", messages.data.len());
        }
        Err(e) => println!("   ❌ List messages failed: {e}"),
    }
}

async fn test_delete_thread(api: &ThreadsApi, thread_id: &str) {
    println!("   🗑️ Deleting thread...");
    match api.delete_thread(thread_id).await {
        Ok(_) => {
            println!("   ✅ Thread deleted successfully");
        }
        Err(e) => println!("   ❌ Delete thread failed: {e}"),
    }
}
