//! Core file operations: upload, list, retrieve, download

use crate::files_demo::utilities::{upload_file_with_logging, DemoFiles};
use openai_rust_sdk::api::files::FilesApi;
use openai_rust_sdk::models::files::{File, FilePurpose, ListFilesParams, SortOrder};
use std::path::Path;
use tokio::fs;

/// Run core file management demos (1-4)
pub async fn run_core_demos(
    files_api: &FilesApi,
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    // Demo 1: Upload files with different purposes
    println!("\n🔄 Demo 1: Uploading files with different purposes");
    upload_demo_files(files_api, demo_files).await?;

    // Demo 2: List and filter files
    println!("\n📋 Demo 2: Listing and filtering files");
    list_files_demo(files_api).await?;

    // Demo 3: Retrieve file information and content
    println!("\n📄 Demo 3: Retrieving file information and content");
    if !demo_files.uploaded_file_ids.is_empty() {
        retrieve_file_demo(files_api, &demo_files.uploaded_file_ids[0]).await?;
    }

    // Demo 4: Download files
    println!("\n💾 Demo 4: Downloading files");
    if !demo_files.uploaded_file_ids.is_empty() {
        let file_id = demo_files.uploaded_file_ids[0].clone();
        download_file_demo(files_api, &file_id, demo_files).await?;
    }

    Ok(())
}

/// Demonstrate uploading files with different purposes
async fn upload_demo_files(
    files_api: &FilesApi,
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    // Upload fine-tuning file
    upload_file_with_logging(
        files_api,
        "demo_fine_tune_data.jsonl",
        FilePurpose::FineTune,
        "fine-tuning data",
        demo_files,
    )
    .await?;

    // Upload batch file
    upload_file_with_logging(
        files_api,
        "demo_batch_data.jsonl",
        FilePurpose::Batch,
        "batch processing data",
        demo_files,
    )
    .await?;

    // Upload assistants document
    upload_file_with_logging(
        files_api,
        "demo_document.md",
        FilePurpose::Assistants,
        "assistants document",
        demo_files,
    )
    .await?;

    // Upload user data file
    upload_file_with_logging(
        files_api,
        "demo_text.txt",
        FilePurpose::UserData,
        "user data file",
        demo_files,
    )
    .await?;

    Ok(())
}

/// Demonstrate listing and filtering files
async fn list_files_demo(files_api: &FilesApi) -> Result<(), Box<dyn std::error::Error>> {
    list_all_files(files_api).await;
    list_files_by_purposes(files_api).await;
    list_files_with_pagination(files_api).await;
    Ok(())
}

async fn list_all_files(files_api: &FilesApi) {
    println!("   📋 Listing all files...");
    match files_api.list_files(None).await {
        Ok(response) => {
            print_all_files_summary(&response);
            print_file_samples(&response.data);
        }
        Err(e) => println!("      ❌ Failed to list files: {}", e),
    }
}

fn print_all_files_summary(response: &openai_rust_sdk::models::files::ListFilesResponse) {
    println!(
        "      ✅ Found {} total files ({})",
        response.data.len(),
        response.total_size_human_readable()
    );
}

fn print_file_samples(files: &[openai_rust_sdk::models::files::File]) {
    for file in files.iter().take(5) {
        println!(
            "         • {} - {} ({}, {})",
            file.filename,
            file.id,
            file.purpose,
            file.size_human_readable()
        );
    }

    if files.len() > 5 {
        println!("         ... and {} more files", files.len() - 5);
    }
}

async fn list_files_by_purposes(files_api: &FilesApi) {
    let purposes = [
        FilePurpose::FineTune,
        FilePurpose::Assistants,
        FilePurpose::Batch,
    ];

    for purpose in &purposes {
        list_files_by_purpose(files_api, purpose).await;
    }
}

async fn list_files_by_purpose(files_api: &FilesApi, purpose: &FilePurpose) {
    println!("   📋 Listing {} files...", purpose);
    match files_api
        .list_files_by_purpose(purpose.clone(), Some(10))
        .await
    {
        Ok(response) => {
            print_purpose_files_summary(&response, purpose);
            print_purpose_files_list(&response.data);
        }
        Err(e) => println!("      ❌ Failed to list {} files: {}", purpose, e),
    }
}

fn print_purpose_files_summary(
    response: &openai_rust_sdk::models::files::ListFilesResponse,
    purpose: &FilePurpose,
) {
    println!("      ✅ Found {} {} files", response.data.len(), purpose);
}

fn print_purpose_files_list(files: &[openai_rust_sdk::models::files::File]) {
    for file in files {
        println!(
            "         • {} - {} ({})",
            file.filename,
            file.id,
            file.size_human_readable()
        );
    }
}

async fn list_files_with_pagination(files_api: &FilesApi) {
    println!("   📋 Listing files with pagination (newest first)...");
    let params = ListFilesParams::new()
        .with_limit(3)
        .with_order(SortOrder::Desc);

    match files_api.list_files(Some(params)).await {
        Ok(response) => {
            print_paginated_files_summary(&response);
            print_paginated_files_list(&response.data);
        }
        Err(e) => println!("      ❌ Failed to list files with pagination: {}", e),
    }
}

fn print_paginated_files_summary(response: &openai_rust_sdk::models::files::ListFilesResponse) {
    println!(
        "      ✅ Found {} files (limited to 3, newest first)",
        response.data.len()
    );
}

fn print_paginated_files_list(files: &[openai_rust_sdk::models::files::File]) {
    for file in files {
        println!(
            "         • {} - {} (created: {})",
            file.filename,
            file.id,
            file.created_at_formatted()
        );
    }
}

/// Demonstrate retrieving file information and content
async fn retrieve_file_demo(
    files_api: &FilesApi,
    file_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve file metadata
    println!("   📄 Retrieving file metadata for {}...", file_id);
    match files_api.retrieve_file(file_id).await {
        Ok(file) => display_file_metadata(&file),
        Err(e) => println!("      ❌ Failed to retrieve file metadata: {}", e),
    }

    // Retrieve file content
    println!("   📄 Retrieving file content for {}...", file_id);
    match files_api.retrieve_file_content(file_id).await {
        Ok(content) => display_file_content_preview(&content),
        Err(e) => println!("      ❌ Failed to retrieve file content: {}", e),
    }

    // Check if file exists
    check_and_report_file_existence(files_api, file_id).await?;

    Ok(())
}

/// Display file metadata in a formatted way
fn display_file_metadata(file: &File) {
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

/// Display a preview of file content
fn display_file_content_preview(content: &str) {
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

/// Check and report file existence status
async fn check_and_report_file_existence(
    files_api: &FilesApi,
    file_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
