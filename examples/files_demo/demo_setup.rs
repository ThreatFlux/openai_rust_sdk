//! Demo setup and initialization functions

use crate::files_demo::utilities::{
    DemoFiles, SAMPLE_BATCH_DATA, SAMPLE_DOCUMENT, SAMPLE_FINE_TUNE_DATA,
};
use openai_rust_sdk::api::{common::ApiClientConstructors, files::FilesApi};
use std::env;
use tokio::fs;

/// Initialize the Files API client and demo files tracker
pub fn initialize_demo() -> Result<(FilesApi, DemoFiles), Box<dyn std::error::Error>> {
    println!("🚀 OpenAI Files API Demo");
    println!("========================\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "Please set the OPENAI_API_KEY environment variable")?;

    // Initialize the Files API client
    let files_api = FilesApi::new(api_key)?;
    let demo_files = DemoFiles::new();

    println!("✅ Files API client initialized\n");
    Ok((files_api, demo_files))
}

/// Create sample files for demonstration
pub async fn create_sample_files(
    demo_files: &mut DemoFiles,
) -> Result<(), Box<dyn std::error::Error>> {
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

/// Print the summary of what was learned in the demo
pub fn print_demo_summary() {
    println!("\n✅ Files API demo completed successfully!");
    println!("\n📚 What you learned:");
    println!("   • How to upload files with different purposes");
    println!("   • How to list and filter files by purpose");
    println!("   • How to retrieve file metadata and content");
    println!("   • How to download and delete files");
    println!("   • How to perform bulk operations");
    println!("   • How to handle errors and validate files");
    println!("\n🚀 You're ready to integrate the Files API into your applications!");
}
