//! Advanced file operations: bulk operations, statistics, error handling

use crate::files_demo::utilities::{human_readable_size, DemoFiles};
use openai_rust_sdk::api::files::FilesApi;
use openai_rust_sdk::models::files::{FilePurpose, FileUploadRequest};

/// Run advanced file operation demos (5-7)
pub async fn run_advanced_demos(
    files_api: &FilesApi,
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    // Demo 5: File usage statistics
    println!("\n📊 Demo 5: File usage statistics");
    file_statistics_demo(files_api).await?;

    // Demo 6: Bulk operations
    println!("\n🔄 Demo 6: Bulk file operations");
    bulk_operations_demo(files_api, demo_files).await?;

    // Demo 7: File validation and error handling
    println!("\n⚠️  Demo 7: File validation and error handling");
    error_handling_demo(files_api).await?;

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
