#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive test of all OpenAI APIs
//!
//! Tests every API that requires authentication

use openai_rust_sdk::{
    api::{
        assistants::AssistantsApi, audio::AudioApi, common::ApiClientConstructors, files::FilesApi,
        fine_tuning::FineTuningApi, moderations::ModerationsApi, runs::RunsApi,
        threads::ThreadsApi, vector_stores::VectorStoresApi,
    },
    error::Result,
    models::{
        assistants::{AssistantRequest, ListAssistantsParams},
        audio::{AudioSpeechRequest, AudioTranscriptionRequest, Voice},
        files::{FilePurpose, FileUploadRequest},
        moderations::ModerationRequest,
        runs::RunRequest,
        threads::{MessageRequest, MessageRole, ThreadRequest},
        vector_stores::VectorStoreRequest,
    },
};
use std::collections::HashMap;
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = get_api_key();
    print_test_header();

    run_core_api_tests(&api_key).await?;
    run_assistant_related_tests(&api_key).await?;
    run_specialized_api_tests(&api_key).await?;

    print_test_footer();
    Ok(())
}

fn get_api_key() -> String {
    env::var("OPENAI_API_KEY").expect("Please set OPENAI_API_KEY environment variable")
}

fn print_test_header() {
    println!("\n🧪 Comprehensive OpenAI API Testing\n");
    println!("{}", "=".repeat(70));
}

async fn run_core_api_tests(api_key: &str) -> Result<()> {
    run_files_api_test(api_key).await?;
    run_moderations_api_test(api_key).await?;
    Ok(())
}

async fn run_assistant_related_tests(api_key: &str) -> Result<()> {
    run_assistants_api_test(api_key).await?;
    run_threads_api_test(api_key).await?;
    run_runs_api_test(api_key).await?;
    run_vector_stores_api_test(api_key).await?;
    Ok(())
}

async fn run_specialized_api_tests(api_key: &str) -> Result<()> {
    run_fine_tuning_api_test(api_key).await?;
    run_audio_api_test(api_key).await?;
    Ok(())
}

async fn run_files_api_test(api_key: &str) -> Result<()> {
    println!("\n📁 Test 1: Files API");
    println!("{}", "-".repeat(70));
    test_files_api(api_key).await
}

async fn run_assistants_api_test(api_key: &str) -> Result<()> {
    println!("\n🤖 Test 2: Assistants API");
    println!("{}", "-".repeat(70));
    test_assistants_api(api_key).await
}

async fn run_threads_api_test(api_key: &str) -> Result<()> {
    println!("\n🧵 Test 3: Threads API");
    println!("{}", "-".repeat(70));
    test_threads_api(api_key).await
}

async fn run_vector_stores_api_test(api_key: &str) -> Result<()> {
    println!("\n🗄️ Test 4: Vector Stores API");
    println!("{}", "-".repeat(70));
    test_vector_stores_api(api_key).await
}

async fn run_fine_tuning_api_test(api_key: &str) -> Result<()> {
    println!("\n🎯 Test 5: Fine-tuning API");
    println!("{}", "-".repeat(70));
    test_fine_tuning_api(api_key).await
}

async fn run_audio_api_test(api_key: &str) -> Result<()> {
    println!("\n🔊 Test 6: Audio API");
    println!("{}", "-".repeat(70));
    test_audio_api(api_key).await
}

async fn run_moderations_api_test(api_key: &str) -> Result<()> {
    println!("\n🛡️ Test 7: Moderations API");
    println!("{}", "-".repeat(70));
    test_moderations_api(api_key).await
}

async fn run_runs_api_test(api_key: &str) -> Result<()> {
    println!("\n⚡ Test 8: Runs API");
    println!("{}", "-".repeat(70));
    test_runs_api(api_key).await
}

fn print_test_footer() {
    println!("\n");
    println!("{}", "=".repeat(70));
    println!("✅ All API Tests Complete!");
    println!("{}", "=".repeat(70));
}

async fn test_files_api(api_key: &str) -> Result<()> {
    let api = FilesApi::new(api_key)?;

    // Create a test file
    let test_content = "This is a test file for OpenAI API testing.";
    let temp_dir = tempfile::tempdir().map_err(openai_rust_sdk::invalid_request_err!(to_string))?;
    let temp_file = temp_dir.path().join("test_file.txt");
    fs::write(&temp_file, test_content)
        .await
        .map_err(openai_rust_sdk::invalid_request_err!(to_string))?;

    // Test upload
    println!("   📤 Uploading file...");
    let file_bytes = fs::read(&temp_file)
        .await
        .map_err(openai_rust_sdk::invalid_request_err!(to_string))?;
    let upload_request = FileUploadRequest::new(
        file_bytes,
        "test_file.txt".to_string(),
        FilePurpose::Assistants,
    );
    match api.upload_file(upload_request).await {
        Ok(file) => {
            println!("   ✅ File uploaded: {}", file.id);

            // Test list files
            println!("   📋 Listing files...");
            match api.list_files(None).await {
                Ok(files) => {
                    println!("   ✅ Found {} files", files.data.len());
                }
                Err(e) => println!("   ❌ List files failed: {e}"),
            }

            // Test retrieve file
            println!("   🔍 Retrieving file info...");
            match api.retrieve_file(&file.id).await {
                Ok(retrieved) => {
                    println!("   ✅ File retrieved: {} bytes", retrieved.bytes);
                }
                Err(e) => println!("   ❌ Retrieve file failed: {e}"),
            }

            // Test delete file
            println!("   🗑️ Deleting file...");
            match api.delete_file(&file.id).await {
                Ok(_) => {
                    println!("   ✅ File deleted successfully");
                }
                Err(e) => println!("   ❌ Delete file failed: {e}"),
            }
        }
        Err(e) => {
            println!("   ❌ File upload failed: {e}");
        }
    }

    // Clean up temp file
    let _ = fs::remove_file(&temp_file).await;

    Ok(())
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

async fn test_threads_api(api_key: &str) -> Result<()> {
    let api = ThreadsApi::new(api_key)?;

    // Test create thread
    println!("   🆕 Creating thread...");
    let request = ThreadRequest::default();

    match api.create_thread(request).await {
        Ok(thread) => {
            println!("   ✅ Thread created: {}", thread.id);
            let thread_id = thread.id.clone();

            // Test add message to thread
            println!("   💬 Adding message to thread...");
            let message = MessageRequest {
                role: MessageRole::User,
                content: "Hello, this is a test message!".to_string(),
                file_ids: vec![],
                metadata: HashMap::new(),
            };

            match api.create_message(&thread_id, message).await {
                Ok(msg) => {
                    println!("   ✅ Message added: {}", msg.id);

                    // Test list messages
                    println!("   📋 Listing messages...");
                    match api.list_messages(&thread_id, None).await {
                        Ok(messages) => {
                            println!("   ✅ Found {} messages", messages.data.len());
                        }
                        Err(e) => println!("   ❌ List messages failed: {e}"),
                    }
                }
                Err(e) => println!("   ❌ Add message failed: {e}"),
            }

            // Test delete thread
            println!("   🗑️ Deleting thread...");
            match api.delete_thread(&thread_id).await {
                Ok(_) => {
                    println!("   ✅ Thread deleted successfully");
                }
                Err(e) => println!("   ❌ Delete thread failed: {e}"),
            }
        }
        Err(e) => {
            println!("   ❌ Create thread failed: {e}");
        }
    }

    Ok(())
}

async fn test_vector_stores_api(api_key: &str) -> Result<()> {
    let api = VectorStoresApi::new(api_key)?;

    // Test create vector store
    println!("   🆕 Creating vector store...");
    let request = VectorStoreRequest {
        name: Some("Test Vector Store".to_string()),
        file_ids: Some(vec![]),
        expires_after: None,
        chunking_strategy: None,
        metadata: Some(HashMap::new()),
    };

    match api.create_vector_store(request).await {
        Ok(store) => {
            println!("   ✅ Vector store created: {}", store.id);
            let store_id = store.id.clone();

            // Test list vector stores
            println!("   📋 Listing vector stores...");
            match api.list_vector_stores(None).await {
                Ok(list) => {
                    println!("   ✅ Found {} vector stores", list.data.len());
                }
                Err(e) => println!("   ❌ List vector stores failed: {e}"),
            }

            // Test retrieve vector store
            println!("   🔍 Retrieving vector store...");
            match api.retrieve_vector_store(&store_id).await {
                Ok(retrieved) => {
                    println!("   ✅ Vector store retrieved: {:?}", retrieved.name);
                }
                Err(e) => println!("   ❌ Retrieve vector store failed: {e}"),
            }

            // Test delete vector store
            println!("   🗑️ Deleting vector store...");
            match api.delete_vector_store(&store_id).await {
                Ok(_) => {
                    println!("   ✅ Vector store deleted successfully");
                }
                Err(e) => println!("   ❌ Delete vector store failed: {e}"),
            }
        }
        Err(e) => {
            println!("   ❌ Create vector store failed: {e}");
        }
    }

    Ok(())
}

async fn test_fine_tuning_api(api_key: &str) -> Result<()> {
    let api = FineTuningApi::new(api_key)?;

    // Test list fine-tuning jobs
    println!("   📋 Listing fine-tuning jobs...");
    match api.list_fine_tuning_jobs(None).await {
        Ok(jobs) => {
            println!("   ✅ Found {} fine-tuning jobs", jobs.data.len());

            if let Some(job) = jobs.data.first() {
                println!("      First job: {}", job.id);
                println!("      Status: {:?}", job.status);

                // Test retrieve job
                println!("   🔍 Retrieving fine-tuning job...");
                match api.retrieve_fine_tuning_job(&job.id).await {
                    Ok(retrieved) => {
                        println!("   ✅ Job retrieved: {}", retrieved.model);
                    }
                    Err(e) => println!("   ❌ Retrieve job failed: {e}"),
                }
            }
        }
        Err(e) => {
            println!("   ❌ List fine-tuning jobs failed: {e}");
        }
    }

    println!("   ℹ️ Note: Creating fine-tuning jobs requires prepared training data");

    Ok(())
}

async fn test_audio_api(api_key: &str) -> Result<()> {
    let api = AudioApi::new(api_key)?;

    // Test Text-to-Speech
    println!("   🔊 Testing Text-to-Speech...");
    let speech_request = AudioSpeechRequest::new(
        "tts-1",
        "Hello, this is a test of the OpenAI text-to-speech API.",
        Voice::Alloy,
    );

    match api.create_speech(&speech_request).await {
        Ok(response) => {
            println!(
                "   ✅ Speech generated: {} bytes",
                response.audio_data.len()
            );
            println!("      Content type: {}", response.content_type);

            // Save audio file
            let audio_dir =
                tempfile::tempdir().map_err(openai_rust_sdk::invalid_request_err!(to_string))?;
            let audio_path = audio_dir.path().join("test_speech.mp3");
            match fs::write(&audio_path, &response.audio_data).await {
                Ok(_) => {
                    println!("      Audio saved to: {audio_path:?}");

                    // Test Transcription (using the generated audio)
                    println!("   🎤 Testing Transcription...");
                    let transcription_request =
                        AudioTranscriptionRequest::new(audio_path.to_str().unwrap(), "whisper-1");

                    match api
                        .create_transcription(&transcription_request, response.audio_data.clone())
                        .await
                    {
                        Ok(transcription) => {
                            println!("   ✅ Transcription successful: {}", transcription.text);
                        }
                        Err(e) => {
                            println!("   ❌ Transcription failed: {e}");
                            if e.to_string().contains("400") {
                                println!("      Note: May need actual audio file");
                            }
                        }
                    }

                    // Clean up
                    let _ = fs::remove_file(&audio_path).await;
                }
                Err(e) => println!("   ❌ Failed to save audio: {e}"),
            }
        }
        Err(e) => {
            println!("   ❌ Speech generation failed: {e}");
        }
    }

    Ok(())
}

async fn test_moderations_api(api_key: &str) -> Result<()> {
    let api = ModerationsApi::new(api_key)?;

    // Test moderation with safe content
    println!("   ✅ Testing with safe content...");
    let safe_request =
        ModerationRequest::new("This is a friendly message about science and technology.")
            .with_model("omni-moderation-latest");

    match api.create_moderation(&safe_request).await {
        Ok(response) => {
            if let Some(result) = response.results.first() {
                println!("   ✅ Safe content moderation complete");
                println!("      Flagged: {}", result.flagged);
                if !result.flagged {
                    println!("      Content is safe ✓");
                }
            }
        }
        Err(e) => {
            println!("   ❌ Safe content moderation failed: {e}");
        }
    }

    // Test moderation with potentially problematic content
    println!("   ⚠️ Testing with edge case content...");
    let edge_request =
        ModerationRequest::new("I hate when my computer crashes and I lose all my work!")
            .with_model("omni-moderation-latest");

    match api.create_moderation(&edge_request).await {
        Ok(response) => {
            if let Some(result) = response.results.first() {
                println!("   ✅ Edge case moderation complete");
                println!("      Flagged: {}", result.flagged);
                if result.flagged {
                    println!("      Categories: {:?}", result.categories);
                } else {
                    println!("      Content passed moderation");
                }
            }
        }
        Err(e) => {
            println!("   ❌ Edge case moderation failed: {e}");
        }
    }

    Ok(())
}

async fn test_runs_api(api_key: &str) -> Result<()> {
    let apis = create_run_test_apis(api_key)?;
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

struct RunTestApis {
    assistants_api: AssistantsApi,
    threads_api: ThreadsApi,
    runs_api: RunsApi,
}

fn create_run_test_apis(api_key: &str) -> Result<RunTestApis> {
    Ok(RunTestApis {
        assistants_api: AssistantsApi::new(api_key)?,
        threads_api: ThreadsApi::new(api_key)?,
        runs_api: RunsApi::new(api_key)?,
    })
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
