//! Main CLI application

mod testing;

use anyhow::Result;
use clap::{Parser, Subcommand};
#[cfg(feature = "yara")]
use std::fs;
use std::path::PathBuf;
#[cfg(feature = "yara")]
use testing::BatchJobGenerator;

#[cfg(feature = "yara")]
use testing::{YaraTestCases, YaraValidator};

#[derive(Parser)]
#[command(name = "openai_rust_sdk")]
#[command(about = "YARA rule validation testing")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a YARA rule
    ValidateRule {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run test suite
    RunTests,
    /// Generate batch jobs
    GenerateBatch {
        #[arg(short, long)]
        output_dir: PathBuf,
        #[arg(short, long, default_value = "comprehensive")]
        suite: String,
    },
}

#[cfg(feature = "yara")]
fn handle_validate_rule(file: &PathBuf, verbose: bool) -> Result<()> {
    let rule_content = openai_rust_sdk::helpers::read_string_sync(file)
        .map_err(|e| anyhow::anyhow!("Failed to read file: {e}"))?;
    let validator = YaraValidator::new();

    match validator.validate_rule(&rule_content) {
        Ok(result) => {
            println!("Valid: {}", result.is_valid);
            if let Some(name) = &result.rule_name {
                println!("Rule Name: {name}");
            }
            println!("Compilation Time: {}ms", result.metrics.compilation_time_ms);

            if verbose {
                println!("Features: {:?}", result.features);
            }

            if !result.errors.is_empty() {
                println!("Errors: {:?}", result.errors);
            }
        }
        Err(e) => {
            eprintln!("Failed to validate: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[cfg(feature = "yara")]
fn handle_run_tests() -> Result<()> {
    let test_cases = YaraTestCases::new();
    let results = test_cases.run_all_tests()?;

    println!("Test Results:");
    println!("Total Tests: {}", results.total_tests);
    println!("Passed: {}", results.passed_tests);
    println!("Failed: {}", results.failed_tests);
    println!("Success Rate: {:.1}%", results.success_rate);
    Ok(())
}

#[cfg(feature = "yara")]
fn handle_generate_batch(output_dir: &PathBuf, suite: &str) -> Result<()> {
    fs::create_dir_all(output_dir)?;

    let generator = BatchJobGenerator::new(None);
    let output_file = output_dir.join(format!("{suite}_batch_jobs.jsonl"));

    generator.generate_test_suite(&output_file, suite)?;

    println!("Generated batch jobs: {}", output_file.display());
    Ok(())
}

#[allow(clippy::missing_const_for_fn)]
fn check_yara_feature() {
    #[cfg(not(feature = "yara"))]
    {
        eprintln!("This CLI requires the 'yara' feature to be enabled.");
        eprintln!("Please rebuild with: cargo build --features yara");
        std::process::exit(1);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    check_yara_feature();

    #[cfg(feature = "yara")]
    {
        let cli = Cli::parse();

        match cli.command {
            Commands::ValidateRule { file, verbose } => {
                handle_validate_rule(&file, verbose)?;
            }
            Commands::RunTests => {
                handle_run_tests()?;
            }
            Commands::GenerateBatch { output_dir, suite } => {
                handle_generate_batch(&output_dir, &suite)?;
            }
        }
    }

    Ok(())
}
