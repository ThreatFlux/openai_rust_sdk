#![allow(clippy::pedantic, clippy::nursery)]
//! # Code Interpreter Demo
//!
//! This example demonstrates how to use the Code Interpreter tool with container management
//! to execute Python code in sandboxed environments.
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example code_interpreter_demo
//! ```

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, containers::ContainersApi, responses::ResponsesApi},
    models::{
        containers::{ContainerBuilder, ContainerMode},
        responses::ResponseRequest,
        tools::ToolBuilder,
    },
};
use std::{env, path::Path};
use tokio::fs;

async fn demo_auto_mode(responses_api: &ResponsesApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🤖 Example 1: Auto Mode (Model manages containers)");
    println!("──────────────────────────────────────────────────");

    let auto_code_tool = ToolBuilder::code_interpreter()
        .container_mode(ContainerMode::Auto)
        .language("python")
        .libraries(vec![
            "numpy".to_string(),
            "pandas".to_string(),
            "matplotlib".to_string(),
            "scipy".to_string(),
        ])
        .max_execution_time_ms(10000)
        .include_citations(true)
        .build();

    let auto_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Generate a dataset of 100 random points, fit a polynomial curve, and create a visualization. \
         Save the plot as 'curve_fit.png' and provide statistics about the fit quality."
    )
    .with_enhanced_tool(auto_code_tool)
    .with_max_tokens(500);

    println!("📊 Running data analysis with auto-managed container...");
    match responses_api.create_response(&auto_request).await {
        Ok(response) => {
            println!("✅ Auto Mode Response:");
            println!("{}", response.output_text());

            if let Some(first_choice) = response.choices.first() {
                if let Some(tool_calls) = &first_choice.message.tool_calls {
                    println!("\n📁 Files created during execution:");
                    for call in tool_calls {
                        println!("  - {}: {}", call.name, call.id);
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠️ Auto mode execution failed: {e}");
        }
    }

    Ok(())
}

async fn demo_explicit_mode(
    containers_api: &ContainersApi,
    responses_api: &ResponsesApi,
) -> Result<openai_rust_sdk::models::containers::Container, Box<dyn std::error::Error>> {
    println!("\n🎯 Example 2: Explicit Mode (Manual container management)");
    println!("────────────────────────────────────────────────────────");

    let container_config = ContainerBuilder::new()
        .name("data-science-container")
        .python_version("3.11")
        .add_library("numpy")
        .add_library("pandas")
        .add_library("scikit-learn")
        .add_library("plotly")
        .memory_limit_mb(2048)
        .cpu_limit(2.0)
        .expiration_minutes(30)
        .build();

    println!("🔨 Creating container with custom configuration...");
    let container = containers_api.create_container(container_config).await?;
    println!("✅ Container created: {}", container.id);
    println!("   Status: {:?}", container.status);
    println!("   Python: {}", container.python_version);
    println!("   Libraries: {:?}", container.libraries);

    upload_sample_data(containers_api, &container.id).await?;
    analyze_with_container(responses_api, &container.id).await?;

    Ok(container)
}

async fn upload_sample_data(
    containers_api: &ContainersApi,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📤 Uploading sample data to container...");
    let sample_data = r"date,value,category
2024-01-01,100,A
2024-01-02,150,B
2024-01-03,130,A
2024-01-04,180,B
2024-01-05,165,A
2024-01-06,195,B
2024-01-07,145,A
2024-01-08,210,B
2024-01-09,175,A
2024-01-10,225,B";

    let file = containers_api
        .upload_file_content(
            container_id,
            "sample_data.csv",
            sample_data.as_bytes().to_vec(),
        )
        .await?;
    println!("✅ File uploaded: {} ({})", file.filename, file.id);

    Ok(())
}

async fn analyze_with_container(
    responses_api: &ResponsesApi,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let explicit_code_tool = ToolBuilder::code_interpreter()
        .container_id(container_id)
        .include_citations(true)
        .persist_container(true)
        .build();

    let explicit_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Load the sample_data.csv file, analyze the trends by category, \
         create a time series visualization, and calculate growth rates. \
         Save your analysis as 'analysis_report.txt' and the visualization as 'trends.html'.",
    )
    .with_enhanced_tool(explicit_code_tool)
    .with_max_tokens(500);

    println!("\n📈 Analyzing data in explicit container...");
    match responses_api.create_response(&explicit_request).await {
        Ok(response) => {
            println!("✅ Explicit Mode Response:");
            println!("{}", response.output_text());
        }
        Err(e) => {
            println!("⚠️ Explicit mode execution failed: {e}");
        }
    }

    Ok(())
}

async fn demo_direct_code_execution(
    containers_api: &ContainersApi,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚡ Example 3: Direct Code Execution in Container");
    println!("────────────────────────────────────────────────");

    let fibonacci_code = r#"
def fibonacci(n):
    """Generate first n Fibonacci numbers"""
    fib = [0, 1]
    for i in range(2, n):
        fib.append(fib[-1] + fib[-2])
    return fib[:n]

# Generate first 20 Fibonacci numbers
fib_sequence = fibonacci(20)
print(f"First 20 Fibonacci numbers: {fib_sequence}")

# Calculate golden ratio approximations
ratios = [fib_sequence[i+1]/fib_sequence[i] for i in range(1, len(fib_sequence)-1)]
print(f"\nGolden ratio approximations: {ratios[-5:]}")
print(f"Final approximation: {ratios[-1]:.10f}")
print(f"Actual golden ratio: {(1 + 5**0.5) / 2:.10f}")

# Save results to file
with open('fibonacci_results.txt', 'w') as f:
    f.write(f"Fibonacci Sequence (n=20):\n")
    f.write(f"{fib_sequence}\n\n")
    f.write(f"Golden Ratio Convergence:\n")
    for i, ratio in enumerate(ratios[-10:], start=11):
        f.write(f"F({i+1})/F({i}) = {ratio:.10f}\n")

print("\nResults saved to fibonacci_results.txt")
"#;

    println!("🧮 Executing Fibonacci analysis...");
    let execution_result = containers_api
        .execute_code_with_timeout(container_id, fibonacci_code, 5000)
        .await?;

    println!("✅ Execution completed!");
    println!("   Status: {:?}", execution_result.status);
    if let Some(stdout) = &execution_result.stdout {
        println!("\n📝 Output:\n{stdout}");
    }
    if let Some(stderr) = &execution_result.stderr {
        if !stderr.is_empty() {
            println!("\n⚠️ Errors:\n{stderr}");
        }
    }
    if !execution_result.created_files.is_empty() {
        println!("\n📁 Files created:");
        for file in &execution_result.created_files {
            println!("  - {} ({} bytes)", file.filename, file.size);
        }
    }

    Ok(())
}

async fn demo_file_management(
    containers_api: &ContainersApi,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📥 Example 4: File Management");
    println!("────────────────────────────");

    let files = containers_api.list_files(container_id).await?;
    println!("📂 Files in container:");
    for file in &files.data {
        println!("  - {} ({} bytes) [{}]", file.filename, file.size, file.id);
    }

    if let Some(result_file) = files
        .data
        .iter()
        .find(|f| f.filename == "fibonacci_results.txt")
    {
        println!("\n⬇️ Downloading fibonacci_results.txt...");
        let content = containers_api
            .download_file(container_id, &result_file.id)
            .await?;
        let content_str = String::from_utf8(content)?;
        println!("📄 File contents:\n{content_str}");

        let output_path = Path::new("downloaded_fibonacci_results.txt");
        fs::write(output_path, content_str).await?;
        println!("💾 Saved to: {}", output_path.display());
    }

    Ok(())
}

async fn demo_container_lifecycle(
    containers_api: &ContainersApi,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n♻️ Example 5: Container Lifecycle");
    println!("──────────────────────────────────");

    println!("🔄 Keeping container alive...");
    containers_api.keep_alive(container_id).await?;
    println!("✅ Container lifetime extended");

    let updated_container = containers_api.get_container(container_id).await?;
    println!("📊 Container status:");
    println!("   Status: {:?}", updated_container.status);
    println!("   Last activity: {}", updated_container.last_activity_at);
    println!("   Expires at: {}", updated_container.expires_at);
    if let Some(memory) = updated_container.memory_usage_mb {
        println!("   Memory usage: {memory} MB");
    }
    if let Some(cpu) = updated_container.cpu_usage_percent {
        println!("   CPU usage: {cpu:.1}%");
    }

    Ok(())
}

async fn demo_multi_step_analysis(
    responses_api: &ResponsesApi,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔬 Example 6: Multi-Step Analysis with Citations");
    println!("────────────────────────────────────────────────");

    let analysis_code_tool = ToolBuilder::code_interpreter()
        .container_id(container_id)
        .include_citations(true)
        .persist_container(true)
        .build();

    let complex_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Perform a comprehensive analysis:\n\
         1. Generate synthetic sales data for 12 months\n\
         2. Apply seasonal decomposition\n\
         3. Forecast next 3 months using ARIMA\n\
         4. Create an interactive dashboard\n\
         5. Document all findings with proper citations",
    )
    .with_enhanced_tool(analysis_code_tool)
    .with_max_tokens(800);

    println!("🔍 Running comprehensive analysis...");
    match responses_api.create_response(&complex_request).await {
        Ok(response) => {
            println!("✅ Analysis Response:");
            println!("{}", response.output_text());

            if let Some(first_choice) = response.choices.first() {
                if let Some(tool_calls) = &first_choice.message.tool_calls {
                    if !tool_calls.is_empty() {
                        println!("\n📚 Citations:");
                        for (i, call) in tool_calls.iter().enumerate() {
                            println!("  [{}] {} - {}", i + 1, call.name, call.id);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠️ Complex analysis failed: {e}");
        }
    }

    Ok(())
}

async fn demo_cleanup(
    containers_api: &ContainersApi,
    container_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧹 Cleanup");
    println!("──────────");

    let container_list = containers_api.list_containers(None).await?;
    println!("📦 Active containers: {}", container_list.data.len());
    for c in &container_list.data {
        println!("  - {} ({:?})", c.id, c.status);
    }

    println!("\n🗑️ Deleting container {container_id}...");
    containers_api.delete_container(container_id).await?;
    println!("✅ Container deleted");

    Ok(())
}

fn print_demo_summary() {
    println!("\n✨ Code Interpreter Demo Complete!");
    println!("💡 Key Takeaways:");
    println!("   • Auto mode creates and manages containers automatically");
    println!("   • Explicit mode gives you full control over container lifecycle");
    println!("   • Containers can persist files across multiple executions");
    println!("   • Code execution includes citations for transparency");
    println!("   • Containers expire after 20 minutes of inactivity");
    println!("   • Files can be uploaded/downloaded for data exchange");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🐍 Code Interpreter Demo");
    println!("=======================");

    let containers_api = ContainersApi::new(api_key.clone())?;
    let responses_api = ResponsesApi::new(api_key)?;

    demo_auto_mode(&responses_api).await?;
    let container = demo_explicit_mode(&containers_api, &responses_api).await?;
    demo_direct_code_execution(&containers_api, &container.id).await?;
    demo_file_management(&containers_api, &container.id).await?;
    demo_container_lifecycle(&containers_api, &container.id).await?;
    demo_multi_step_analysis(&responses_api, &container.id).await?;
    demo_cleanup(&containers_api, &container.id).await?;
    print_demo_summary();

    Ok(())
}
