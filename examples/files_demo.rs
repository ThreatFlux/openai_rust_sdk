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

mod files_demo {
    pub mod advanced_operations;
    pub mod core_operations;
    pub mod demo_setup;
    pub mod utilities;

    pub use advanced_operations::*;
    pub use core_operations::*;
    pub use demo_setup::*;
    pub use utilities::*;
}

use files_demo::{
    cleanup_demo_files, create_sample_files, initialize_demo, print_demo_summary,
    run_advanced_demos, run_core_demos,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (files_api, mut demo_files) = initialize_demo()?;

    // Create sample files for demonstration
    println!("📝 Creating sample files for demonstration...");
    create_sample_files(&mut demo_files).await?;

    run_core_demos(&files_api, &mut demo_files).await?;
    run_advanced_demos(&files_api, &mut demo_files).await?;

    // Cleanup
    println!("\n🧹 Cleaning up demo files...");
    cleanup_demo_files(&files_api, &demo_files).await?;

    print_demo_summary();
    Ok(())
}
