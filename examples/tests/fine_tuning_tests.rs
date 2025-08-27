//! Fine-tuning API test module

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, fine_tuning::FineTuningApi},
    error::Result,
};

pub async fn run_fine_tuning_api_test(api_key: &str) -> Result<()> {
    println!("\n🎯 Test 5: Fine-tuning API");
    println!("{}", "-".repeat(70));
    test_fine_tuning_api(api_key).await
}

async fn test_fine_tuning_api(api_key: &str) -> Result<()> {
    let api = FineTuningApi::new(api_key)?;

    println!("   📋 Listing fine-tuning jobs...");
    match api.list_fine_tuning_jobs(None).await {
        Ok(jobs) => {
            println!("   ✅ Found {} fine-tuning jobs", jobs.data.len());

            if let Some(job) = jobs.data.first() {
                test_first_job_details(&api, job).await;
            }
        }
        Err(e) => {
            println!("   ❌ List fine-tuning jobs failed: {e}");
        }
    }

    println!("   ℹ️ Note: Creating fine-tuning jobs requires prepared training data");
    Ok(())
}

async fn test_first_job_details(
    api: &FineTuningApi,
    job: &openai_rust_sdk::models::fine_tuning::FineTuningJob,
) {
    println!("      First job: {}", job.id);
    println!("      Status: {:?}", job.status);

    println!("   🔍 Retrieving fine-tuning job...");
    match api.retrieve_fine_tuning_job(&job.id).await {
        Ok(retrieved) => {
            println!("   ✅ Job retrieved: {}", retrieved.model);
        }
        Err(e) => println!("   ❌ Retrieve job failed: {e}"),
    }
}
