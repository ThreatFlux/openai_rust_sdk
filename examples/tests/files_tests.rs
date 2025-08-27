//! Files API test module

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, files::FilesApi},
    error::Result,
    models::files::{FilePurpose, FileUploadRequest},
};
use std::path::PathBuf;
use tokio::fs;

pub async fn run_files_api_test(api_key: &str) -> Result<()> {
    println!("\n📁 Test 1: Files API");
    println!("{}", "-".repeat(70));
    test_files_api(api_key).await
}

async fn test_files_api(api_key: &str) -> Result<()> {
    let api = FilesApi::new(api_key)?;

    // Create a test file
    let (temp_file, file_bytes) = create_test_file().await?;

    // Test file operations
    let file_id = test_file_upload(&api, file_bytes).await?;
    if let Some(id) = file_id {
        test_file_listing(&api).await?;
        test_file_retrieval(&api, &id).await?;
        test_file_deletion(&api, &id).await?;
    }

    // Clean up temp file
    cleanup_temp_file(&temp_file).await;

    Ok(())
}

async fn create_test_file() -> Result<(PathBuf, Vec<u8>)> {
    let test_content = "This is a test file for OpenAI API testing.";
    let temp_dir = tempfile::tempdir().map_err(openai_rust_sdk::invalid_request_err!(to_string))?;
    let temp_file = temp_dir.path().join("test_file.txt");
    fs::write(&temp_file, test_content)
        .await
        .map_err(openai_rust_sdk::invalid_request_err!(to_string))?;

    let file_bytes = fs::read(&temp_file)
        .await
        .map_err(openai_rust_sdk::invalid_request_err!(to_string))?;

    Ok((temp_file, file_bytes))
}

async fn test_file_upload(api: &FilesApi, file_bytes: Vec<u8>) -> Result<Option<String>> {
    println!("   📤 Uploading file...");
    let upload_request = FileUploadRequest::new(
        file_bytes,
        "test_file.txt".to_string(),
        FilePurpose::Assistants,
    );

    match api.upload_file(upload_request).await {
        Ok(file) => {
            println!("   ✅ File uploaded: {}", file.id);
            Ok(Some(file.id))
        }
        Err(e) => {
            println!("   ❌ File upload failed: {e}");
            Ok(None)
        }
    }
}

async fn test_file_listing(api: &FilesApi) -> Result<()> {
    println!("   📋 Listing files...");
    match api.list_files(None).await {
        Ok(files) => {
            println!("   ✅ Found {} files", files.data.len());
        }
        Err(e) => println!("   ❌ List files failed: {e}"),
    }
    Ok(())
}

async fn test_file_retrieval(api: &FilesApi, file_id: &str) -> Result<()> {
    println!("   🔍 Retrieving file info...");
    match api.retrieve_file(file_id).await {
        Ok(retrieved) => {
            println!("   ✅ File retrieved: {} bytes", retrieved.bytes);
        }
        Err(e) => println!("   ❌ Retrieve file failed: {e}"),
    }
    Ok(())
}

async fn test_file_deletion(api: &FilesApi, file_id: &str) -> Result<()> {
    println!("   🗑️ Deleting file...");
    match api.delete_file(file_id).await {
        Ok(_) => {
            println!("   ✅ File deleted successfully");
        }
        Err(e) => println!("   ❌ Delete file failed: {e}"),
    }
    Ok(())
}

async fn cleanup_temp_file(temp_file: &PathBuf) {
    let _ = fs::remove_file(temp_file).await;
}
