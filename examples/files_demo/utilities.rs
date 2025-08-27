//! Utility constants, data structures and helper functions for the files demo

use openai_rust_sdk::api::files::FilesApi;
use openai_rust_sdk::models::files::{FilePurpose, FileUploadRequest};
use std::path::Path;
use tokio::fs;

/// Sample training data for fine-tuning demo
pub const SAMPLE_FINE_TUNE_DATA: &str = r#"{"messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is machine learning?"}, {"role": "assistant", "content": "Machine learning is a subset of artificial intelligence that enables computers to learn and make decisions from data without being explicitly programmed."}], "tools": []}
{"messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "Explain neural networks."}, {"role": "assistant", "content": "Neural networks are computational models inspired by biological neural networks. They consist of interconnected nodes (neurons) that process information through weighted connections."}], "tools": []}
{"messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "What is deep learning?"}, {"role": "assistant", "content": "Deep learning is a subset of machine learning that uses artificial neural networks with multiple layers (deep networks) to model and understand complex patterns in data."}], "tools": []}"#;

/// Sample batch processing data
pub const SAMPLE_BATCH_DATA: &str = r#"{"custom_id": "request-1", "method": "POST", "url": "/v1/chat/completions", "body": {"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello, world!"}], "max_tokens": 50}}
{"custom_id": "request-2", "method": "POST", "url": "/v1/chat/completions", "body": {"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "What is AI?"}], "max_tokens": 100}}
{"custom_id": "request-3", "method": "POST", "url": "/v1/chat/completions", "body": {"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Explain quantum computing."}], "max_tokens": 150}}"#;

/// Sample document for assistants
pub const SAMPLE_DOCUMENT: &str = r"# OpenAI API Documentation

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
pub struct DemoFiles {
    pub uploaded_file_ids: Vec<String>,
    pub local_file_paths: Vec<String>,
}

impl DemoFiles {
    pub fn new() -> Self {
        Self {
            uploaded_file_ids: Vec::new(),
            local_file_paths: Vec::new(),
        }
    }

    pub fn add_uploaded(&mut self, file_id: String) {
        self.uploaded_file_ids.push(file_id);
    }

    pub fn add_local(&mut self, path: String) {
        self.local_file_paths.push(path);
    }
}

/// Helper function to format file sizes in human-readable format
pub fn human_readable_size(bytes: u64) -> String {
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

/// Helper function to upload a single file with error handling
pub async fn upload_file_with_logging(
    files_api: &FilesApi,
    file_path: &str,
    purpose: FilePurpose,
    description: &str,
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   📤 Uploading {}...", description);
    let request = FileUploadRequest::from_file_path(Path::new(file_path), purpose).await?;

    match files_api.upload_file(request).await {
        Ok(file) => {
            println!(
                "      ✅ {} uploaded: {} ({})",
                description,
                file.id,
                file.size_human_readable()
            );
            demo_files.add_uploaded(file.id);
        }
        Err(e) => println!("      ❌ Failed to upload {}: {}", description, e),
    }
    Ok(())
}

/// Clean up demo files
pub async fn cleanup_demo_files(
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
