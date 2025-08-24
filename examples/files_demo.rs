#![allow(
    clippy::too_many_lines,
    clippy::uninlined_format_args,
    clippy::cast_precision_loss,
    clippy::ignored_unit_patterns,
    clippy::missing_const_for_fn,
    clippy::inefficient_to_string
)]
//! # Files API Demo
//!
//! This example demonstrates how to use the `OpenAI` Files API for managing files
//! that can be used with various `OpenAI` services including fine-tuning, assistants,
//! batch processing, and more.
//!
//! ## Features Demonstrated
//!
//! - **File Upload**: Upload files with different purposes
//! - **File Listing**: List and filter files by purpose
//! - **File Management**: Retrieve file metadata and content
//! - **File Operations**: Download and delete files
//! - **Bulk Operations**: Upload multiple files and bulk delete
//! - **Error Handling**: Proper error handling for file operations
//!
//! ## Usage
//!
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! cargo run --example files_demo
//! ```
//!
//! The demo will:
//! 1. Create sample files for demonstration
//! 2. Upload files with different purposes
//! 3. List and filter files
//! 4. Download file content
//! 5. Demonstrate file management operations
//! 6. Clean up by deleting demo files

use openai_rust_sdk::api::{common::ApiClientConstructors, files::FilesApi};
use openai_rust_sdk::models::files::{FilePurpose, FileUploadRequest, ListFilesParams, SortOrder};
use std::env;
use std::path::Path;
use tokio::fs;

/// Sample training data for fine-tuning demo
const SAMPLE_FINE_TUNE_DATA: &str = r#"{"messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is machine learning?"}, {"role": "assistant", "content": "Machine learning is a subset of artificial intelligence that enables computers to learn and make decisions from data without being explicitly programmed."}], "tools": []}
{"messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "Explain neural networks."}, {"role": "assistant", "content": "Neural networks are computational models inspired by biological neural networks. They consist of interconnected nodes (neurons) that process information through weighted connections."}], "tools": []}
{"messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is deep learning?"}, {"role": "assistant", "content": "Deep learning is a subset of machine learning that uses artificial neural networks with multiple layers (deep networks) to model and understand complex patterns in data."}], "tools": []}"#;

/// Sample batch processing data
const SAMPLE_BATCH_DATA: &str = r#"{"custom_id": "request-1", "method": "POST", "url": "/v1/chat/completions", "body": {"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello, world!"}], "max_tokens": 50}}
{"custom_id": "request-2", "method": "POST", "url": "/v1/chat/completions", "body": {"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "What is AI?"}], "max_tokens": 100}}
{"custom_id": "request-3", "method": "POST", "url": "/v1/chat/completions", "body": {"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Explain quantum computing."}], "max_tokens": 150}}"#;

/// Sample document for assistants
const SAMPLE_DOCUMENT: &str = r"# OpenAI API Documentation

## Overview
The OpenAI API provides access to powerful AI models for various tasks including:
- Text generation and completion
- Code generation and analysis  
- Image generation and editing
- Audio transcription and generation
- Embeddings for semantic search

## Authentication
All API requests require an API key in the Authorization header:
```
Authorization: Bearer YOUR_API_KEY
```

## Rate Limits
- Free tier: 3 requests per minute
- Paid tier: Varies by model and subscription

## Models Available
- GPT-4: Most capable model for complex tasks
- GPT-3.5 Turbo: Fast and efficient for most use cases
- DALL-E 3: Advanced image generation
- Whisper: Speech-to-text conversion

## Best Practices
1. Use appropriate models for your use case
2. Implement proper error handling
3. Respect rate limits
4. Use caching when possible
5. Monitor usage and costs";

/// Helper struct to track demo files for cleanup
#[derive(Debug)]
struct DemoFiles {
    uploaded_file_ids: Vec<String>,
    local_file_paths: Vec<String>,
}

impl DemoFiles {
    fn new() -> Self {
        Self {
            uploaded_file_ids: Vec::new(),
            local_file_paths: Vec::new(),
        }
    }

    fn add_uploaded(&mut self, file_id: String) {
        self.uploaded_file_ids.push(file_id);
    }

    fn add_local(&mut self, path: String) {
        self.local_file_paths.push(path);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OpenAI Files API Demo");
    println!("========================\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "Please set the OPENAI_API_KEY environment variable")?;

    // Initialize the Files API client
    let files_api = FilesApi::new(api_key)?;
    let mut demo_files = DemoFiles::new();

    println!("✅ Files API client initialized\n");

    // Create sample files for demonstration
    println!("📝 Creating sample files for demonstration...");
    create_sample_files(&mut demo_files).await?;

    // Demo 1: Upload files with different purposes
    println!("\n🔄 Demo 1: Uploading files with different purposes");
    upload_demo_files(&files_api, &mut demo_files).await?;

    // Demo 2: List and filter files
    println!("\n📋 Demo 2: Listing and filtering files");
    list_files_demo(&files_api).await?;

    // Demo 3: Retrieve file information and content
    println!("\n📄 Demo 3: Retrieving file information and content");
    if !demo_files.uploaded_file_ids.is_empty() {
        retrieve_file_demo(&files_api, &demo_files.uploaded_file_ids[0]).await?;
    }

    // Demo 4: Download files
    println!("\n💾 Demo 4: Downloading files");
    if !demo_files.uploaded_file_ids.is_empty() {
        let file_id = demo_files.uploaded_file_ids[0].clone();
        download_file_demo(&files_api, &file_id, &mut demo_files).await?;
    }

    // Demo 5: File usage statistics
    println!("\n📊 Demo 5: File usage statistics");
    file_statistics_demo(&files_api).await?;

    // Demo 6: Bulk operations
    println!("\n🔄 Demo 6: Bulk file operations");
    bulk_operations_demo(&files_api, &mut demo_files).await?;

    // Demo 7: File validation and error handling
    println!("\n⚠️  Demo 7: File validation and error handling");
    error_handling_demo(&files_api).await?;

    // Cleanup
    println!("\n🧹 Cleaning up demo files...");
    cleanup_demo_files(&files_api, &demo_files).await?;

    println!("\n✅ Files API demo completed successfully!");
    println!("\n📚 What you learned:");
    println!("   • How to upload files with different purposes");
    println!("   • How to list and filter files by purpose");
    println!("   • How to retrieve file metadata and content");
    println!("   • How to download and delete files");
    println!("   • How to perform bulk operations");
    println!("   • How to handle errors and validate files");
    println!("\n🚀 You're ready to integrate the Files API into your applications!");

    Ok(())
}

/// Create sample files for demonstration
async fn create_sample_files(demo_files: &mut DemoFiles) -> Result<(), Box<dyn std::error::Error>> {
    // Create fine-tuning data file
    let fine_tune_path = "demo_fine_tune_data.jsonl";
    fs::write(fine_tune_path, SAMPLE_FINE_TUNE_DATA).await?;
    demo_files.add_local(fine_tune_path.to_string());
    println!("   ✅ Created fine-tuning data file: {}", fine_tune_path);

    // Create batch processing file
    let batch_path = "demo_batch_data.jsonl";
    fs::write(batch_path, SAMPLE_BATCH_DATA).await?;
    demo_files.add_local(batch_path.to_string());
    println!("   ✅ Created batch processing file: {}", batch_path);

    // Create assistants document
    let doc_path = "demo_document.md";
    fs::write(doc_path, SAMPLE_DOCUMENT).await?;
    demo_files.add_local(doc_path.to_string());
    println!("   ✅ Created assistants document: {}", doc_path);

    // Create a simple text file
    let text_path = "demo_text.txt";
    fs::write(text_path, "This is a sample text file for demonstration purposes.\nIt contains multiple lines of text.\nUseful for testing file upload and retrieval.").await?;
    demo_files.add_local(text_path.to_string());
    println!("   ✅ Created text file: {}", text_path);

    Ok(())
}

/// Demonstrate uploading files with different purposes
async fn upload_demo_files(
    files_api: &FilesApi,
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    // Upload fine-tuning file
    println!("   📤 Uploading fine-tuning data...");
    let fine_tune_request = FileUploadRequest::from_file_path(
        Path::new("demo_fine_tune_data.jsonl"),
        FilePurpose::FineTune,
    )
    .await?;

    match files_api.upload_file(fine_tune_request).await {
        Ok(file) => {
            println!(
                "      ✅ Fine-tuning file uploaded: {} ({})",
                file.id,
                file.size_human_readable()
            );
            demo_files.add_uploaded(file.id);
        }
        Err(e) => println!("      ❌ Failed to upload fine-tuning file: {}", e),
    }

    // Upload batch file
    println!("   📤 Uploading batch processing data...");
    let batch_request =
        FileUploadRequest::from_file_path(Path::new("demo_batch_data.jsonl"), FilePurpose::Batch)
            .await?;

    match files_api.upload_file(batch_request).await {
        Ok(file) => {
            println!(
                "      ✅ Batch file uploaded: {} ({})",
                file.id,
                file.size_human_readable()
            );
            demo_files.add_uploaded(file.id);
        }
        Err(e) => println!("      ❌ Failed to upload batch file: {}", e),
    }

    // Upload assistants document
    println!("   📤 Uploading assistants document...");
    let doc_request =
        FileUploadRequest::from_file_path(Path::new("demo_document.md"), FilePurpose::Assistants)
            .await?;

    match files_api.upload_file(doc_request).await {
        Ok(file) => {
            println!(
                "      ✅ Assistants document uploaded: {} ({})",
                file.id,
                file.size_human_readable()
            );
            demo_files.add_uploaded(file.id);
        }
        Err(e) => println!("      ❌ Failed to upload assistants document: {}", e),
    }

    // Upload user data file
    println!("   📤 Uploading user data file...");
    let text_request =
        FileUploadRequest::from_file_path(Path::new("demo_text.txt"), FilePurpose::UserData)
            .await?;

    match files_api.upload_file(text_request).await {
        Ok(file) => {
            println!(
                "      ✅ User data file uploaded: {} ({})",
                file.id,
                file.size_human_readable()
            );
            demo_files.add_uploaded(file.id);
        }
        Err(e) => println!("      ❌ Failed to upload user data file: {}", e),
    }

    Ok(())
}

/// Demonstrate listing and filtering files
async fn list_files_demo(files_api: &FilesApi) -> Result<(), Box<dyn std::error::Error>> {
    // List all files
    println!("   📋 Listing all files...");
    match files_api.list_files(None).await {
        Ok(response) => {
            println!(
                "      ✅ Found {} total files ({})",
                response.data.len(),
                response.total_size_human_readable()
            );

            for file in response.data.iter().take(5) {
                println!(
                    "         • {} - {} ({}, {})",
                    file.filename,
                    file.id,
                    file.purpose,
                    file.size_human_readable()
                );
            }

            if response.data.len() > 5 {
                println!("         ... and {} more files", response.data.len() - 5);
            }
        }
        Err(e) => println!("      ❌ Failed to list files: {}", e),
    }

    // List files by purpose
    for purpose in &[
        FilePurpose::FineTune,
        FilePurpose::Assistants,
        FilePurpose::Batch,
    ] {
        println!("   📋 Listing {} files...", purpose);
        match files_api
            .list_files_by_purpose(purpose.clone(), Some(10))
            .await
        {
            Ok(response) => {
                println!("      ✅ Found {} {} files", response.data.len(), purpose);
                for file in &response.data {
                    println!(
                        "         • {} - {} ({})",
                        file.filename,
                        file.id,
                        file.size_human_readable()
                    );
                }
            }
            Err(e) => println!("      ❌ Failed to list {} files: {}", purpose, e),
        }
    }

    // List files with pagination and sorting
    println!("   📋 Listing files with pagination (newest first)...");
    let params = ListFilesParams::new()
        .with_limit(3)
        .with_order(SortOrder::Desc);

    match files_api.list_files(Some(params)).await {
        Ok(response) => {
            println!(
                "      ✅ Found {} files (limited to 3, newest first)",
                response.data.len()
            );
            for file in &response.data {
                println!(
                    "         • {} - {} (created: {})",
                    file.filename,
                    file.id,
                    file.created_at_formatted()
                );
            }
        }
        Err(e) => println!("      ❌ Failed to list files with pagination: {}", e),
    }

    Ok(())
}

/// Demonstrate retrieving file information and content
async fn retrieve_file_demo(
    files_api: &FilesApi,
    file_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve file metadata
    println!("   📄 Retrieving file metadata for {}...", file_id);
    match files_api.retrieve_file(file_id).await {
        Ok(file) => {
            println!("      ✅ File metadata:");
            println!("         • ID: {}", file.id);
            println!("         • Filename: {}", file.filename);
            println!("         • Purpose: {}", file.purpose);
            println!("         • Size: {}", file.size_human_readable());
            println!("         • Status: {}", file.status);
            println!("         • Created: {}", file.created_at_formatted());

            if let Some(details) = &file.status_details {
                println!("         • Status Details: {}", details);
            }
        }
        Err(e) => println!("      ❌ Failed to retrieve file metadata: {}", e),
    }

    // Retrieve file content
    println!("   📄 Retrieving file content for {}...", file_id);
    match files_api.retrieve_file_content(file_id).await {
        Ok(content) => {
            println!(
                "      ✅ File content retrieved ({} characters)",
                content.len()
            );

            // Show first few lines of content
            let lines: Vec<&str> = content.lines().take(3).collect();
            for (i, line) in lines.iter().enumerate() {
                let display_line = if line.len() > 80 {
                    format!("{}...", &line[..77])
                } else {
                    line.to_string()
                };
                println!("         Line {}: {}", i + 1, display_line);
            }

            if content.lines().count() > 3 {
                println!(
                    "         ... and {} more lines",
                    content.lines().count() - 3
                );
            }
        }
        Err(e) => println!("      ❌ Failed to retrieve file content: {}", e),
    }

    // Check if file exists
    println!("   🔍 Checking if file exists...");
    match files_api.file_exists(file_id).await {
        Ok(exists) => {
            if exists {
                println!("      ✅ File exists and is accessible");
            } else {
                println!("      ❌ File does not exist or is not accessible");
            }
        }
        Err(e) => println!("      ❌ Failed to check file existence: {}", e),
    }

    Ok(())
}

/// Demonstrate downloading files
async fn download_file_demo(
    files_api: &FilesApi,
    file_id: &str,
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    let download_path = format!("downloaded_{}.txt", file_id);

    println!("   💾 Downloading file {} to {}...", file_id, download_path);
    match files_api
        .download_file(file_id, Path::new(&download_path))
        .await
    {
        Ok(bytes_written) => {
            println!(
                "      ✅ Downloaded {} bytes to {}",
                bytes_written, download_path
            );
            demo_files.add_local(download_path.clone());

            // Verify the download by reading the file
            if let Ok(content) = fs::read_to_string(&download_path).await {
                println!(
                    "      ✅ Downloaded file contains {} characters",
                    content.len()
                );
            }
        }
        Err(e) => println!("      ❌ Failed to download file: {}", e),
    }

    Ok(())
}

/// Demonstrate file usage statistics
async fn file_statistics_demo(files_api: &FilesApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("   📊 Getting file usage statistics...");
    match files_api.get_file_usage_stats().await {
        Ok(stats) => {
            println!("      ✅ File usage statistics:");

            let mut total_files = 0;
            let mut total_size = 0u64;

            for (purpose, (count, size)) in &stats {
                println!(
                    "         • {}: {} files, {} bytes",
                    purpose,
                    count,
                    human_readable_size(*size)
                );
                total_files += count;
                total_size += size;
            }

            println!(
                "         • Total: {} files, {}",
                total_files,
                human_readable_size(total_size)
            );
        }
        Err(e) => println!("      ❌ Failed to get file usage statistics: {}", e),
    }

    Ok(())
}

/// Demonstrate bulk file operations
async fn bulk_operations_demo(
    files_api: &FilesApi,
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create multiple files for bulk upload
    println!("   🔄 Creating multiple files for bulk upload...");
    let bulk_requests = vec![
        FileUploadRequest::new(
            b"Bulk upload test file 1".to_vec(),
            "bulk_test_1.txt".to_string(),
            FilePurpose::UserData,
        ),
        FileUploadRequest::new(
            b"Bulk upload test file 2".to_vec(),
            "bulk_test_2.txt".to_string(),
            FilePurpose::UserData,
        ),
        FileUploadRequest::new(
            b"Bulk upload test file 3".to_vec(),
            "bulk_test_3.txt".to_string(),
            FilePurpose::UserData,
        ),
    ];

    // Bulk upload files
    println!(
        "   📤 Uploading {} files in parallel...",
        bulk_requests.len()
    );
    let results = files_api
        .upload_files_parallel(bulk_requests, Some(2))
        .await;

    let mut successful_uploads = 0;
    for result in results {
        match result {
            Ok(file) => {
                println!("      ✅ Uploaded: {} ({})", file.filename, file.id);
                demo_files.add_uploaded(file.id);
                successful_uploads += 1;
            }
            Err(e) => println!("      ❌ Upload failed: {}", e),
        }
    }

    println!(
        "      📊 Bulk upload completed: {}/3 files uploaded successfully",
        successful_uploads
    );

    Ok(())
}

/// Demonstrate error handling and validation
async fn error_handling_demo(files_api: &FilesApi) -> Result<(), Box<dyn std::error::Error>> {
    // Test file validation errors
    println!("   ⚠️  Testing file validation errors...");

    // Empty file
    let empty_request =
        FileUploadRequest::new(Vec::new(), "empty.txt".to_string(), FilePurpose::UserData);

    if let Err(e) = empty_request.validate() {
        println!("      ✅ Caught expected validation error: {}", e);
    }

    // Wrong file extension for fine-tuning
    let wrong_ext_request = FileUploadRequest::new(
        b"some content".to_vec(),
        "wrong.txt".to_string(),
        FilePurpose::FineTune,
    );

    if let Err(e) = wrong_ext_request.validate() {
        println!("      ✅ Caught expected validation error: {}", e);
    }

    // Test API errors
    println!("   ⚠️  Testing API error handling...");

    // Try to retrieve a non-existent file
    match files_api.retrieve_file("file-nonexistent").await {
        Ok(_) => println!("      ❌ Expected error for non-existent file"),
        Err(e) => println!("      ✅ Caught expected API error: {}", e),
    }

    // Test file existence check
    match files_api.file_exists("file-nonexistent").await {
        Ok(false) => println!("      ✅ Correctly detected non-existent file"),
        Ok(true) => println!("      ❌ Incorrectly reported non-existent file as existing"),
        Err(e) => println!("      ❌ Unexpected error checking file existence: {}", e),
    }

    Ok(())
}

/// Clean up demo files
async fn cleanup_demo_files(
    files_api: &FilesApi,
    demo_files: &DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    // Delete uploaded files
    println!("   🗑️  Deleting uploaded files...");
    for file_id in &demo_files.uploaded_file_ids {
        match files_api.delete_file(file_id).await {
            Ok(response) if response.deleted => {
                println!("      ✅ Deleted file: {}", file_id);
            }
            Ok(_) => {
                println!("      ⚠️  File deletion not confirmed: {}", file_id);
            }
            Err(e) => {
                println!("      ❌ Failed to delete file {}: {}", file_id, e);
            }
        }
    }

    // Delete local files
    println!("   🗑️  Deleting local files...");
    for path in &demo_files.local_file_paths {
        match fs::remove_file(path).await {
            Ok(_) => println!("      ✅ Deleted local file: {}", path),
            Err(e) => println!("      ❌ Failed to delete local file {}: {}", path, e),
        }
    }

    Ok(())
}

/// Helper function to format file sizes in human-readable format
fn human_readable_size(bytes: u64) -> String {
    let bytes = bytes as f64;
    if bytes < 1024.0 {
        format!("{} B", bytes)
    } else if bytes < 1024.0 * 1024.0 {
        format!("{:.1} KB", bytes / 1024.0)
    } else if bytes < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} MB", bytes / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes / (1024.0 * 1024.0 * 1024.0))
    }
}
