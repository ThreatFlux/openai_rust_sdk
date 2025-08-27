#![allow(clippy::pedantic, clippy::nursery)]
//! # Batch File Processing Example
//!
//! This example demonstrates how to download and process completed batch files:
//! 1. Download batch results and error files
//! 2. Extract YARA rules from AI responses  
//! 3. Generate comprehensive reports
//! 4. Analyze batch processing statistics
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example batch_file_processing <batch_id>
//! ```

use openai_rust_sdk::api::{batch::BatchApi, batch::BatchReport, common::ApiClientConstructors};
use std::{env, path::Path};

struct BatchProcessor {
    batch_api: BatchApi,
    batch_id: String,
    output_dir: std::path::PathBuf,
}

impl BatchProcessor {
    fn new(batch_api: BatchApi, batch_id: String) -> Result<Self, std::io::Error> {
        let output_dir = std::env::current_dir()?.join("batch_processing_output");
        Ok(Self {
            batch_api,
            batch_id,
            output_dir,
        })
    }

    async fn check_batch_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n📊 Step 1: Checking batch status...");
        let batch_status = self.batch_api.get_batch_status(&self.batch_id).await?;
        println!("   Status: {}", batch_status.status);
        println!("   Total requests: {}", batch_status.request_counts.total);
        println!("   Completed: {}", batch_status.request_counts.completed);
        println!("   Failed: {}", batch_status.request_counts.failed);

        if batch_status.status.to_string() != "completed" {
            println!(
                "⚠️ Batch is not completed yet. Current status: {}",
                batch_status.status
            );
            println!("   Please wait for the batch to complete before processing files.");
            return Err("Batch not completed".into());
        }
        Ok(())
    }

    async fn download_files(&self) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        println!("\n📥 Step 2: Downloading batch files...");
        let (result_count, error_count) = self
            .batch_api
            .download_all_batch_files(&self.batch_id, &self.output_dir)
            .await?;

        println!("✅ Files downloaded successfully:");
        println!("   📁 Output directory: {}", self.output_dir.display());
        println!("   📄 Result lines: {result_count}");
        println!("   ❌ Error lines: {error_count}");

        Ok((result_count, error_count))
    }

    async fn extract_yara_rules(&self) -> Result<usize, Box<dyn std::error::Error>> {
        println!("\n🔍 Step 3: Extracting YARA rules...");
        let results_file = self
            .output_dir
            .join(format!("{}_results.jsonl", self.batch_id));
        let yara_dir = self.output_dir.join("extracted_yara_rules");

        let yara_count = self
            .batch_api
            .process_yara_results(&results_file, &yara_dir)
            .await?;

        println!(
            "✅ Extracted {} YARA rules to: {}",
            yara_count,
            yara_dir.display()
        );

        if yara_count > 0 {
            self.display_yara_files(&yara_dir, yara_count)?;
        }

        Ok(yara_count)
    }

    fn display_yara_files(
        &self,
        yara_dir: &Path,
        yara_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("   📝 YARA rule files created:");

        let entries = match std::fs::read_dir(yara_dir) {
            Ok(entries) => entries,
            Err(_) => return Ok(()),
        };

        self.display_yara_file_list(entries)?;
        self.display_remaining_file_count(yara_count);
        Ok(())
    }

    fn display_yara_file_list(
        &self,
        entries: std::fs::ReadDir,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (i, entry) in entries.enumerate().take(10) {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_path = entry.path();
            let size = self.get_file_size_string(&file_path);

            println!(
                "      {}. {} ({})",
                i + 1,
                file_name.to_string_lossy(),
                size
            );
        }
        Ok(())
    }

    fn get_file_size_string(&self, file_path: &Path) -> String {
        match std::fs::metadata(file_path) {
            Ok(metadata) => format!("{} bytes", metadata.len()),
            Err(_) => "unknown size".to_string(),
        }
    }

    fn display_remaining_file_count(&self, yara_count: usize) {
        if yara_count > 10 {
            println!("      ... and {} more rule files", yara_count - 10);
        }
    }

    async fn generate_report(
        &self,
        error_count: usize,
    ) -> Result<BatchReport, Box<dyn std::error::Error>> {
        println!("\n📊 Step 4: Generating comprehensive report...");
        let results_file = self
            .output_dir
            .join(format!("{}_results.jsonl", self.batch_id));
        let errors_file = self
            .output_dir
            .join(format!("{}_errors.jsonl", self.batch_id));
        let report_file = self.output_dir.join("processing_report.md");

        let error_file_path = if error_count > 0 {
            Some(errors_file.as_path())
        } else {
            None
        };

        let report = self
            .batch_api
            .generate_batch_report(&results_file, error_file_path, &report_file)
            .await?;

        println!("✅ Report generated: {}", report_file.display());
        Ok(report)
    }

    fn display_statistics(&self, report: &BatchReport) {
        println!("\n📈 Step 5: Summary Statistics");
        println!("─────────────────────────────");

        self.display_batch_processing_stats(report);
        self.display_yara_analysis_stats(report);
        self.display_content_analysis_stats(report);
        self.display_error_analysis_stats(report);
    }

    fn display_batch_processing_stats(&self, report: &BatchReport) {
        println!("📊 Batch Processing Results:");
        println!("   • Total responses: {}", report.total_responses);
        println!("   • Successful responses: {}", report.successful_responses);
        println!("   • Error responses: {}", report.error_responses);
        println!("   • Success rate: {:.1}%", report.success_rate());
    }

    fn display_yara_analysis_stats(&self, report: &BatchReport) {
        println!("\n🔍 YARA Rule Analysis:");
        println!("   • YARA rules found: {}", report.yara_rules_found);
        println!(
            "   • YARA extraction rate: {:.1}%",
            report.yara_extraction_rate()
        );
    }

    fn display_content_analysis_stats(&self, report: &BatchReport) {
        let avg_length = self.calculate_average_response_length(report);

        println!("\n📏 Content Analysis:");
        println!(
            "   • Total content length: {} characters",
            report.total_tokens
        );
        println!("   • Average response length: {avg_length:.0} characters");
    }

    fn calculate_average_response_length(&self, report: &BatchReport) -> f64 {
        if report.successful_responses > 0 {
            report.total_tokens as f64 / report.successful_responses as f64
        } else {
            0.0
        }
    }

    fn display_error_analysis_stats(&self, report: &BatchReport) {
        if report.error_types.is_empty() {
            return;
        }

        println!("\n⚠️ Error Analysis:");
        for (error_type, count) in &report.error_types {
            println!("   • {error_type}: {count} occurrences");
        }
    }

    fn display_sample_yara_content(
        &self,
        yara_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if yara_count == 0 {
            return Ok(());
        }

        println!("\n📝 Step 6: Sample YARA Rule Content");
        println!("──────────────────────────────────");

        let yara_dir = self.output_dir.join("extracted_yara_rules");
        self.display_first_yara_rule(&yara_dir)?;
        Ok(())
    }

    fn display_first_yara_rule(&self, yara_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let entries = std::fs::read_dir(yara_dir)?;
        let first_entry = entries.take(1).next();

        if let Some(Ok(entry)) = first_entry {
            self.display_yara_rule_content(&entry)?;
        }

        Ok(())
    }

    fn display_yara_rule_content(
        &self,
        entry: &std::fs::DirEntry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = entry.path();
        let content = std::fs::read_to_string(&file_path)?;

        println!(
            "🔍 Sample rule from {}:",
            entry.file_name().to_string_lossy()
        );
        println!("```yara");
        println!("{content}");
        println!("```");

        Ok(())
    }

    fn display_recommendations(&self, report: &BatchReport, yara_count: usize) {
        println!("\n💡 Step 7: Recommendations");
        println!("──────────────────────────");

        self.display_success_rate_recommendations(report);

        if yara_count > 0 {
            self.display_yara_recommendations(report);
            self.display_next_steps();
        }
    }

    fn display_success_rate_recommendations(&self, report: &BatchReport) {
        let success_rate = report.success_rate();

        if success_rate >= 95.0 {
            println!("✅ Excellent success rate! Your batch configuration is working well.");
        } else if success_rate < 90.0 {
            println!(
                "⚠️ Success rate is below 90%. Consider reviewing your prompts or model parameters."
            );
        }
    }

    fn display_yara_recommendations(&self, report: &BatchReport) {
        let extraction_rate = report.yara_extraction_rate();

        if extraction_rate >= 90.0 {
            println!("✅ High YARA rule extraction rate indicates effective prompts.");
        } else if extraction_rate < 80.0 {
            println!("⚠️ YARA rule extraction rate is low. Consider improving prompt specificity.");
        }
    }

    fn display_next_steps(&self) {
        println!("🔧 Next steps:");
        println!("   1. Validate extracted YARA rules with yara-x");
        println!("   2. Test rules against sample malware datasets");
        println!("   3. Integrate successful rules into your detection pipeline");
        println!("   4. Consider batch processing for rule optimization");
    }
}

fn parse_command_line_args() -> Result<String, Box<dyn std::error::Error>> {
    // Safely collect args into a vector to avoid security concerns  
    // This is just for parsing a batch ID, not for security operations
    // nosemgrep: rust.lang.security.args.args - Safe usage: only reading batch ID from CLI
    let args: Vec<String> = env::args().collect();
    
    // Check argument count
    if args.len() < 2 {
        eprintln!("Usage: batch_file_processing <batch_id>");
        eprintln!("Example: batch_file_processing batch_68a039c97af48190b645b9ece8266a52");
        return Err("Missing batch ID".into());
    }
    
    // Ensure no extra arguments
    if args.len() > 2 {
        eprintln!("Error: Too many arguments provided");
        eprintln!("Usage: batch_file_processing <batch_id>");
        return Err("Too many arguments".into());
    }
    
    // Get batch ID from command line arguments (skip program name at index 0)
    let batch_id = args[1].clone();

    Ok(batch_id)
}

fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here".into()
    })
}

/// Initialize and validate the batch processor
async fn initialize_processor() -> Result<BatchProcessor, Box<dyn std::error::Error>> {
    let api_key = get_api_key()?;
    let batch_id = parse_command_line_args()?;

    println!("🔄 Batch File Processing Demo");
    println!("============================");
    println!("📋 Batch ID: {batch_id}");

    let batch_api = BatchApi::new(api_key)?;
    Ok(BatchProcessor::new(batch_api, batch_id)?)
}

/// Execute the main batch processing pipeline
async fn execute_batch_processing(
    processor: &BatchProcessor,
) -> Result<ProcessingResults, Box<dyn std::error::Error>> {
    processor.check_batch_status().await?;
    let (_, error_count) = processor.download_files().await?;
    let yara_count = processor.extract_yara_rules().await?;
    let report = processor.generate_report(error_count).await?;

    Ok(ProcessingResults { yara_count, report })
}

/// Display final results and recommendations
fn display_final_results(
    processor: &BatchProcessor,
    results: &ProcessingResults,
) -> Result<(), Box<dyn std::error::Error>> {
    processor.display_statistics(&results.report);
    processor.display_sample_yara_content(results.yara_count)?;
    processor.display_recommendations(&results.report, results.yara_count);

    println!("\n✨ Batch file processing completed!");
    println!(
        "📁 All output files saved to: {}",
        processor.output_dir.display()
    );

    Ok(())
}

/// Structure to hold processing results
struct ProcessingResults {
    yara_count: usize,
    report: BatchReport,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processor = initialize_processor().await?;
    let results = execute_batch_processing(&processor).await?;
    display_final_results(&processor, &results)?;
    Ok(())
}
