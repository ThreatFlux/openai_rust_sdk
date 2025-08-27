//! Runs API test module

use openai_rust_sdk::{
    api::{
        assistants::AssistantsApi, common::ApiClientConstructors, runs::RunsApi,
        threads::ThreadsApi,
    },
    error::Result,
    models::{
        assistants::AssistantRequest,
        runs::RunRequest,
        threads::{MessageRequest, MessageRole, ThreadRequest},
    },
};
use std::collections::HashMap;

pub async fn run_runs_api_test(api_key: &str) -> Result<()> {
    println!("\n⚡ Test 8: Runs API");
    println!("{}", "-".repeat(70));
    test_runs_api(api_key).await
}

pub struct RunTestApis {
    pub assistants_api: AssistantsApi,
    pub threads_api: ThreadsApi,
    pub runs_api: RunsApi,
}

impl RunTestApis {
    pub fn new(api_key: &str) -> Result<Self> {
        Ok(Self {
            assistants_api: AssistantsApi::new(api_key)?,
            threads_api: ThreadsApi::new(api_key)?,
            runs_api: RunsApi::new(api_key)?,
        })
    }
}

async fn test_runs_api(api_key: &str) -> Result<()> {
    let apis = RunTestApis::new(api_key)?;
    println!("   🔧 Setting up assistant and thread for run...");

    let assistant_request = create_test_assistant_request();
    match apis
        .assistants_api
        .create_assistant(assistant_request)
        .await
    {
        Ok(assistant) => {
            let assistant_id = assistant.id.clone();
            println!("   ✅ Assistant created: {assistant_id}");

            execute_run_test_workflow(&apis, assistant_id).await;
        }
        Err(e) => println!("   ❌ Create assistant failed: {e}"),
    }

    Ok(())
}

fn create_test_assistant_request() -> AssistantRequest {
    AssistantRequest {
        model: "gpt-4-turbo-preview".to_string(),
        name: Some("Run Test Assistant".to_string()),
        description: None,
        instructions: Some("You are a helpful assistant. Answer questions concisely.".to_string()),
        tools: vec![],
        file_ids: vec![],
        metadata: HashMap::new(),
    }
}

async fn execute_run_test_workflow(apis: &RunTestApis, assistant_id: String) {
    match create_test_thread(&apis.threads_api).await {
        Ok(thread_id) => {
            execute_thread_workflow(apis, &assistant_id, &thread_id).await;
            cleanup_thread(&apis.threads_api, &thread_id).await;
        }
        Err(e) => println!("   ❌ Create thread failed: {e}"),
    }
    cleanup_assistant(&apis.assistants_api, &assistant_id).await;
}

async fn create_test_thread(
    threads_api: &ThreadsApi,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let thread_request = ThreadRequest::default();
    let thread = threads_api.create_thread(thread_request).await?;
    let thread_id = thread.id.clone();
    println!("   ✅ Thread created: {thread_id}");
    Ok(thread_id)
}

async fn execute_thread_workflow(apis: &RunTestApis, assistant_id: &str, thread_id: &str) {
    if add_test_message(&apis.threads_api, thread_id).await {
        execute_run_workflow(&apis.runs_api, assistant_id, thread_id).await;
    }
}

async fn add_test_message(threads_api: &ThreadsApi, thread_id: &str) -> bool {
    let message = MessageRequest {
        role: MessageRole::User,
        content: "What is 2+2?".to_string(),
        file_ids: vec![],
        metadata: HashMap::new(),
    };

    match threads_api.create_message(thread_id, message).await {
        Ok(_) => {
            println!("   ✅ Message added to thread");
            true
        }
        Err(e) => {
            println!("   ❌ Add message failed: {e}");
            false
        }
    }
}

async fn execute_run_workflow(runs_api: &RunsApi, assistant_id: &str, thread_id: &str) {
    println!("   ⚡ Creating run...");
    let run_request = create_test_run_request(assistant_id);

    match runs_api.create_run(thread_id, run_request).await {
        Ok(run) => {
            print_run_created(&run);
            list_runs(runs_api, thread_id).await;
            cancel_run(runs_api, thread_id, &run.id).await;
        }
        Err(e) => println!("   ❌ Create run failed: {e}"),
    }
}

fn create_test_run_request(assistant_id: &str) -> RunRequest {
    RunRequest {
        assistant_id: assistant_id.to_string(),
        model: None,
        instructions: None,
        tools: None,
        file_ids: None,
        metadata: Some(HashMap::new()),
    }
}

fn print_run_created(run: &openai_rust_sdk::models::runs::Run) {
    println!("   ✅ Run created: {}", run.id);
    println!("      Status: {:?}", run.status);
}

async fn list_runs(runs_api: &RunsApi, thread_id: &str) {
    println!("   📋 Listing runs...");
    match runs_api.list_runs(thread_id, None).await {
        Ok(runs) => {
            println!("   ✅ Found {} runs", runs.data.len());
        }
        Err(e) => println!("   ❌ List runs failed: {e}"),
    }
}

async fn cancel_run(runs_api: &RunsApi, thread_id: &str, run_id: &str) {
    println!("   🛑 Cancelling run...");
    match runs_api.cancel_run(thread_id, run_id).await {
        Ok(_) => {
            println!("   ✅ Run cancelled");
        }
        Err(e) => println!("   ❌ Cancel run failed: {e}"),
    }
}

async fn cleanup_thread(threads_api: &ThreadsApi, thread_id: &str) {
    let _ = threads_api.delete_thread(thread_id).await;
}

async fn cleanup_assistant(assistants_api: &AssistantsApi, assistant_id: &str) {
    let _ = assistants_api.delete_assistant(assistant_id).await;
}
