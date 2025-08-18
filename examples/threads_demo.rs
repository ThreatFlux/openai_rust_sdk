//! # OpenAI Threads & Messages API Demo
//!
//! This example demonstrates the complete usage of the OpenAI Threads API,
//! including creating threads, managing messages, handling file attachments,
//! and demonstrating various content types and annotations.
//!
//! ## Features Demonstrated
//!
//! - Creating and managing conversation threads
//! - Adding user and assistant messages
//! - Working with file attachments
//! - Handling different content types (text, images)
//! - Managing message metadata
//! - Pagination through message lists
//! - Error handling and best practices
//!
//! ## Running the Example
//!
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! cargo run --example threads_demo
//! ```

use openai_rust_sdk::api::threads::ThreadsApi;
use openai_rust_sdk::error::{OpenAIError, Result};
use openai_rust_sdk::models::threads::{
    Annotation, FileCitation, FilePathInfo, ListMessagesParams, MessageContent, MessageRequest,
    MessageRole, SortOrder, TextContent, ThreadRequest,
};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the API client
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        OpenAIError::Authentication("OPENAI_API_KEY environment variable not set".to_string())
    })?;

    let api = ThreadsApi::new(api_key)?;

    println!("💬 OpenAI Threads & Messages API Demo");
    println!("=====================================\n");

    // Demo 1: Create a basic thread and add messages
    println!("🧵 Demo 1: Creating a Customer Support Thread");
    let support_thread = demo_customer_support_thread(&api).await?;
    println!("✅ Created thread: {}\n", support_thread.id);

    // Demo 2: Create a thread with file attachments
    println!("📎 Demo 2: Creating a Document Analysis Thread");
    let analysis_thread = demo_document_analysis_thread(&api).await?;
    println!("✅ Created thread: {}\n", analysis_thread.id);

    // Demo 3: Demonstrate message listing and pagination
    println!("📋 Demo 3: Listing Messages with Pagination");
    demo_message_pagination(&api, &support_thread.id).await?;

    // Demo 4: Demonstrate metadata management
    println!("🏷️  Demo 4: Managing Thread and Message Metadata");
    demo_metadata_management(&api, &analysis_thread.id).await?;

    // Demo 5: Demonstrate different content types
    println!("🎨 Demo 5: Working with Different Content Types");
    demo_content_types(&api).await?;

    // Demo 6: Demonstrate message file management
    println!("📁 Demo 6: Managing Message Files");
    demo_message_files(&api, &analysis_thread.id).await?;

    // Demo 7: Demonstrate error handling
    println!("⚠️  Demo 7: Error Handling Examples");
    demo_error_handling(&api).await?;

    // Cleanup: Delete the created threads
    println!("🧹 Cleanup: Deleting Created Threads");
    cleanup_threads(&api, vec![support_thread.id, analysis_thread.id]).await?;

    println!("🎉 Demo completed successfully!");
    Ok(())
}

/// Demo 1: Create a customer support conversation thread
async fn demo_customer_support_thread(
    api: &ThreadsApi,
) -> Result<openai_rust_sdk::models::threads::Thread> {
    // Create a thread for customer support
    let thread_request = ThreadRequest::builder()
        .metadata_pair("purpose", "customer_support")
        .metadata_pair("priority", "high")
        .metadata_pair("department", "billing")
        .build();

    let thread = api.create_thread(thread_request).await?;
    println!("   📱 Created customer support thread: {}", thread.id);

    // Add initial customer message
    let customer_message = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Hello, I'm having trouble with my billing. I was charged twice for the same service last month.")
        .metadata_pair("message_type", "complaint")
        .metadata_pair("category", "billing_issue")
        .build()?;

    let message1 = api.create_message(&thread.id, customer_message).await?;
    println!("   💬 Customer message: {}", message1.id);

    // Add assistant response
    let assistant_message = MessageRequest::builder()
        .role(MessageRole::Assistant)
        .content("I'm sorry to hear about the billing issue. Let me look into this for you. Can you please provide your account number and the approximate dates of the charges?")
        .metadata_pair("message_type", "response")
        .metadata_pair("agent_id", "agent_001")
        .build()?;

    let message2 = api.create_message(&thread.id, assistant_message).await?;
    println!("   🤖 Assistant response: {}", message2.id);

    // Add follow-up customer message
    let followup_message = MessageRequest::builder()
        .role(MessageRole::User)
        .content("My account number is AC12345. The charges were on March 15th and March 16th, both for $29.99.")
        .metadata_pair("message_type", "information")
        .metadata_pair("contains_account_info", "true")
        .build()?;

    let message3 = api.create_message(&thread.id, followup_message).await?;
    println!("   📋 Customer follow-up: {}", message3.id);

    Ok(thread)
}

/// Demo 2: Create a document analysis thread with file attachments
async fn demo_document_analysis_thread(
    api: &ThreadsApi,
) -> Result<openai_rust_sdk::models::threads::Thread> {
    // Create a thread for document analysis
    let thread_request = ThreadRequest::builder()
        .metadata_pair("purpose", "document_analysis")
        .metadata_pair("client", "legal_firm_abc")
        .metadata_pair("case_id", "CASE-2024-001")
        .build();

    let thread = api.create_thread(thread_request).await?;
    println!("   📄 Created document analysis thread: {}", thread.id);

    // Add message with file attachments
    let analysis_request = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Please analyze the attached contracts and identify any potential legal risks or unusual clauses.")
        .file_id("file-contract1_abc123")
        .file_id("file-contract2_def456")
        .file_id("file-addendum_ghi789")
        .metadata_pair("analysis_type", "legal_review")
        .metadata_pair("urgency", "high")
        .build()?;

    let message1 = api.create_message(&thread.id, analysis_request).await?;
    println!("   📎 Analysis request with attachments: {}", message1.id);

    // Add assistant response with analysis
    let analysis_response = MessageRequest::builder()
        .role(MessageRole::Assistant)
        .content("I've analyzed the three contract documents. Here's my initial assessment:\n\n1. Contract 1: Standard terms, no major concerns\n2. Contract 2: Contains an unusual termination clause in section 4.2\n3. Addendum: Modifies liability terms - requires careful review\n\nI'll provide a detailed report with specific recommendations.")
        .metadata_pair("analysis_complete", "true")
        .metadata_pair("risk_level", "medium")
        .build()?;

    let message2 = api.create_message(&thread.id, analysis_response).await?;
    println!("   📊 Analysis response: {}", message2.id);

    Ok(thread)
}

/// Demo 3: Demonstrate message listing and pagination
async fn demo_message_pagination(api: &ThreadsApi, thread_id: &str) -> Result<()> {
    // List messages with default parameters
    println!("   📋 Listing all messages in thread...");
    let all_messages = api.list_messages(thread_id, None).await?;
    println!("   ✅ Found {} messages total", all_messages.data.len());

    // List messages with pagination (limit 2)
    let params = ListMessagesParams::new().limit(2).order(SortOrder::Desc);

    let limited_messages = api.list_messages(thread_id, Some(params)).await?;
    println!(
        "   📄 Limited to 2 messages: found {}",
        limited_messages.data.len()
    );
    println!("   🔄 Has more: {}", limited_messages.has_more);

    // If there are more messages, get the next page
    if limited_messages.has_more && !limited_messages.data.is_empty() {
        let last_message_id = &limited_messages.data.last().unwrap().id;
        let next_params = ListMessagesParams::new()
            .limit(2)
            .order(SortOrder::Desc)
            .after(last_message_id);

        let next_page = api.list_messages(thread_id, Some(next_params)).await?;
        println!("   📄 Next page: found {} messages", next_page.data.len());
    }

    // List messages in ascending order (oldest first)
    let asc_params = ListMessagesParams::new().order(SortOrder::Asc);

    let asc_messages = api.list_messages(thread_id, Some(asc_params)).await?;
    println!(
        "   🔼 Messages in ascending order: {}",
        asc_messages.data.len()
    );

    Ok(())
}

/// Demo 4: Demonstrate metadata management
async fn demo_metadata_management(api: &ThreadsApi, thread_id: &str) -> Result<()> {
    // Retrieve the thread and show its metadata
    let thread = api.retrieve_thread(thread_id).await?;
    println!("   🏷️  Current thread metadata:");
    for (key, value) in &thread.metadata {
        println!("      {} = {}", key, value);
    }

    // Modify thread metadata
    let updated_metadata = ThreadRequest::builder()
        .metadata_pair("status", "in_progress")
        .metadata_pair("last_updated", "2024-03-15T10:30:00Z")
        .metadata_pair("assigned_to", "analyst_jane")
        .build();

    let updated_thread = api.modify_thread(thread_id, updated_metadata).await?;
    println!(
        "   ✅ Updated thread metadata (now has {} pairs)",
        updated_thread.metadata.len()
    );

    // Get a message and show its metadata
    let messages = api
        .list_messages(thread_id, Some(ListMessagesParams::new().limit(1)))
        .await?;
    if let Some(message) = messages.data.first() {
        println!("   💬 Message metadata for {}:", message.id);
        for (key, value) in &message.metadata {
            println!("      {} = {}", key, value);
        }

        // Modify message metadata (create a new message request with updated metadata)
        let updated_message_request = MessageRequest::builder()
            .role(message.role.clone())
            .content("Updated message content with new metadata")
            .metadata_pair("reviewed", "true")
            .metadata_pair("reviewer", "supervisor_bob")
            .metadata_pair("review_date", "2024-03-15")
            .build()?;

        let updated_message = api
            .modify_message(thread_id, &message.id, updated_message_request)
            .await?;
        println!(
            "   ✅ Updated message metadata (now has {} pairs)",
            updated_message.metadata.len()
        );
    }

    Ok(())
}

/// Demo 5: Demonstrate different content types
async fn demo_content_types(api: &ThreadsApi) -> Result<()> {
    // Create a thread for content type demonstrations
    let thread_request = ThreadRequest::builder()
        .metadata_pair("purpose", "content_type_demo")
        .build();

    let thread = api.create_thread(thread_request).await?;
    println!("   🎨 Created content types demo thread: {}", thread.id);

    // Text content with annotations
    let text_with_citations = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Based on the research in document A and the data from spreadsheet B, we can conclude that the market trends are positive.")
        .file_id("file-research_doc_a")
        .file_id("file-spreadsheet_b")
        .metadata_pair("content_type", "text_with_citations")
        .build()?;

    let message1 = api.create_message(&thread.id, text_with_citations).await?;
    println!("   📝 Text message with file references: {}", message1.id);

    // Image analysis request
    let image_analysis = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Please analyze this chart and provide insights on the trends shown.")
        .file_id("file-chart_image_123")
        .metadata_pair("content_type", "image_analysis")
        .metadata_pair("image_type", "chart")
        .build()?;

    let message2 = api.create_message(&thread.id, image_analysis).await?;
    println!("   🖼️  Image analysis request: {}", message2.id);

    // Assistant response with structured content
    let structured_response = MessageRequest::builder()
        .role(MessageRole::Assistant)
        .content(r#"## Analysis Results

### Key Findings:
1. **Revenue Growth**: 15% increase over last quarter
2. **Market Share**: Gained 2.3% in target demographic
3. **Customer Satisfaction**: 94% approval rating

### Recommendations:
- Continue current marketing strategy
- Expand into adjacent markets
- Increase customer support staff

### Supporting Data:
The analysis is based on data from files referenced in your request. See citations [1] and [2] for detailed metrics."#)
        .metadata_pair("response_type", "structured_analysis")
        .metadata_pair("format", "markdown")
        .build()?;

    let message3 = api.create_message(&thread.id, structured_response).await?;
    println!("   📊 Structured analysis response: {}", message3.id);

    // Clean up the demo thread
    let _ = api.delete_thread(&thread.id).await?;
    println!("   🧹 Cleaned up content types demo thread");

    Ok(())
}

/// Demo 6: Demonstrate message file management
async fn demo_message_files(api: &ThreadsApi, thread_id: &str) -> Result<()> {
    // Get messages that have file attachments
    let messages = api.list_messages(thread_id, None).await?;

    for message in &messages.data {
        if !message.file_ids.is_empty() {
            println!(
                "   📁 Message {} has {} file(s)",
                message.id,
                message.file_ids.len()
            );

            // List files attached to this message
            let message_files = api.list_message_files(thread_id, &message.id).await?;
            println!(
                "   📋 Retrieved {} message file objects",
                message_files.data.len()
            );

            // Get details for each file
            for message_file in &message_files.data {
                let file_details = api
                    .retrieve_message_file(thread_id, &message.id, &message_file.id)
                    .await?;
                println!(
                    "   📄 File {}: created at {}",
                    file_details.id, file_details.created_at
                );
            }

            break; // Just demonstrate with the first message that has files
        }
    }

    Ok(())
}

/// Demo 7: Demonstrate error handling
async fn demo_error_handling(api: &ThreadsApi) -> Result<()> {
    println!("   ⚠️  Testing error scenarios...");

    // Test 1: Try to retrieve a non-existent thread
    match api.retrieve_thread("thread_nonexistent").await {
        Ok(_) => println!("   ❌ Expected error but got success"),
        Err(e) => println!("   ✅ Correctly handled non-existent thread error: {}", e),
    }

    // Test 2: Try to create a message with invalid content
    let invalid_content = "a".repeat(50000); // Exceeds the 32,768 character limit
    match MessageRequest::builder()
        .role(MessageRole::User)
        .content(invalid_content)
        .build()
    {
        Ok(_) => println!("   ❌ Expected validation error but got success"),
        Err(e) => println!("   ✅ Correctly caught content length validation: {}", e),
    }

    // Test 3: Try to create a message with too many file IDs
    let mut builder = MessageRequest::builder()
        .role(MessageRole::User)
        .content("Test");

    for i in 0..15 {
        // More than the 10 file limit
        builder = builder.file_id(format!("file-{}", i));
    }

    match builder.build() {
        Ok(_) => println!("   ❌ Expected validation error but got success"),
        Err(e) => println!("   ✅ Correctly caught file ID limit validation: {}", e),
    }

    // Test 4: Try to create a thread with too much metadata
    let mut thread_request = ThreadRequest::new();
    for i in 0..20 {
        // More than the 16 metadata limit
        thread_request
            .metadata
            .insert(format!("key{}", i), "value".to_string());
    }

    match thread_request.validate() {
        Ok(_) => println!("   ❌ Expected validation error but got success"),
        Err(e) => println!("   ✅ Correctly caught metadata limit validation: {}", e),
    }

    Ok(())
}

/// Cleanup function to delete created threads
async fn cleanup_threads(api: &ThreadsApi, thread_ids: Vec<String>) -> Result<()> {
    for thread_id in thread_ids {
        match api.delete_thread(&thread_id).await {
            Ok(deletion_status) => {
                if deletion_status.deleted {
                    println!("   ✅ Successfully deleted thread: {}", thread_id);
                } else {
                    println!("   ❌ Failed to delete thread: {}", thread_id);
                }
            }
            Err(e) => {
                println!("   ⚠️  Error deleting thread {}: {}", thread_id, e);
            }
        }
    }
    Ok(())
}

/// Helper function to create sample annotations for demonstration
#[allow(dead_code)]
fn create_sample_annotations() -> Vec<Annotation> {
    vec![
        Annotation::FileCitation {
            text: "market research data".to_string(),
            start_index: 25,
            end_index: 44,
            file_citation: FileCitation {
                file_id: "file-research_2024_q1".to_string(),
                quote: Some(
                    "Q1 market share increased by 12.5% compared to previous quarter".to_string(),
                ),
            },
        },
        Annotation::FilePath {
            text: "financial report".to_string(),
            start_index: 85,
            end_index: 100,
            file_path: FilePathInfo {
                file_id: "file-financial_report_march".to_string(),
            },
        },
    ]
}

/// Helper function to create text content with annotations
#[allow(dead_code)]
fn create_annotated_text_content() -> MessageContent {
    let annotations = create_sample_annotations();
    let text_content = TextContent {
        value: "According to the latest market research data from our Q1 analysis and the financial report, we're seeing positive trends across all key metrics.".to_string(),
        annotations,
    };

    MessageContent::Text { text: text_content }
}
