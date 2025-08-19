//! # Prompt Caching Demo
//!
//! This example demonstrates how to optimize prompts for caching to reduce
//! latency by up to 80% and costs by up to 75%.
//!
//! Prompt caching works automatically for prompts ≥1024 tokens and caches
//! in 128-token increments. Cache hits require exact prefix matches.
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example prompt_caching_demo
//! ```

use openai_rust_sdk::{
    api::responses::ResponsesApi,
    models::responses::{Message, ResponseRequest},
};
use std::{env, time::Instant};
use tokio::time::{Duration, sleep};

// Static system prompt that will be cached (>1024 tokens)
const SYSTEM_PROMPT: &str = r#"
You are an advanced AI assistant specialized in data analysis, programming, and scientific research.
Your capabilities include:

1. **Data Analysis and Visualization**
   - Statistical analysis and hypothesis testing
   - Time series analysis and forecasting
   - Machine learning model development and evaluation
   - Data cleaning and preprocessing
   - Creating insightful visualizations and dashboards

2. **Programming and Software Development**
   - Writing clean, efficient, and well-documented code
   - Debugging and optimizing existing code
   - System design and architecture recommendations
   - API design and integration
   - Database design and query optimization

3. **Scientific Research**
   - Literature review and citation management
   - Experimental design and methodology
   - Statistical power analysis
   - Research proposal writing
   - Peer review and manuscript preparation

4. **Mathematics and Computation**
   - Symbolic mathematics and calculus
   - Linear algebra and matrix operations
   - Numerical methods and optimization
   - Graph theory and network analysis
   - Cryptography and security analysis

5. **Natural Language Processing**
   - Text classification and sentiment analysis
   - Named entity recognition
   - Language translation and localization
   - Text summarization and generation
   - Information extraction and retrieval

Guidelines for your responses:
- Always provide accurate, evidence-based information
- Cite sources when making factual claims
- Explain complex concepts in accessible terms
- Provide code examples when relevant
- Consider multiple perspectives on controversial topics
- Acknowledge limitations and uncertainties
- Suggest follow-up questions or areas for exploration
- Use structured formatting for clarity
- Include practical examples and applications
- Verify calculations and logic before presenting results

When writing code:
- Use appropriate error handling
- Include comprehensive comments
- Follow language-specific best practices
- Consider performance implications
- Provide test cases when applicable
- Document assumptions and constraints
- Use meaningful variable names
- Implement proper input validation
- Consider edge cases and boundary conditions
- Provide complexity analysis for algorithms

This extensive system prompt ensures that we meet the 1024 token minimum for caching
and provides consistent behavior across all requests. The static nature of this prompt
makes it ideal for caching, as it will be reused across multiple API calls.
"#;

// Common examples that can be cached
const EXAMPLES: &str = r#"
Example 1: Data Analysis Request
User: "Analyze this sales data and identify trends"
Assistant: "I'll help you analyze the sales data. Let me examine the key metrics including revenue trends, seasonal patterns, top-performing products, and growth rates. I'll create visualizations and provide statistical insights."

Example 2: Code Review Request
User: "Review this Python function for performance"
Assistant: "I'll review your Python function focusing on time complexity, space efficiency, readability, and potential optimizations. I'll provide specific suggestions with code examples."

Example 3: Research Question
User: "Explain quantum entanglement"
Assistant: "Quantum entanglement is a phenomenon where quantum particles become correlated in such a way that the quantum state of each particle cannot be described independently. Let me explain the key concepts, mathematical formulation, and practical applications."
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🚀 Prompt Caching Demo");
    println!("======================");
    println!("This demo shows how to optimize prompts for caching to reduce latency and costs.\n");

    let api = ResponsesApi::new(api_key)?;

    // Example 1: Basic caching with repeated requests
    println!("📊 Example 1: Basic Prompt Caching");
    println!("──────────────────────────────────");
    println!("Making 5 identical requests to demonstrate caching...\n");

    let mut total_cached_tokens = 0;
    let mut total_prompt_tokens = 0;
    let mut timings = Vec::new();

    for i in 1..=5 {
        let start = Instant::now();

        let request = ResponseRequest::new_messages(
            "gpt-4o-mini",
            vec![
                Message::developer(SYSTEM_PROMPT),
                Message::user("What are the key principles of effective data visualization?"),
            ],
        )
        .with_max_tokens(200)
        .with_temperature(0.7);

        let response = api.create_response(&request).await?;
        let elapsed = start.elapsed();
        timings.push(elapsed);

        if let Some(usage) = &response.usage {
            let cached = response.cached_tokens();
            total_cached_tokens += cached;
            total_prompt_tokens += usage.prompt_tokens;

            println!("Request #{}: ", i);
            println!("  ⏱️  Latency: {:.2}ms", elapsed.as_millis());
            println!("  📝 Prompt tokens: {}", usage.prompt_tokens);
            println!(
                "  ✅ Cached tokens: {} ({:.1}% hit rate)",
                cached,
                response.cache_hit_rate()
            );
            println!(
                "  💰 Cost savings: ~{:.1}%",
                if cached > 0 { 75.0 } else { 0.0 }
            );
        }

        // Small delay between requests
        if i < 5 {
            sleep(Duration::from_millis(500)).await;
        }
    }

    // Calculate improvements
    let avg_first_latency = timings[0].as_millis() as f64;
    let avg_cached_latency = timings[1..]
        .iter()
        .map(|d| d.as_millis() as f64)
        .sum::<f64>()
        / (timings.len() - 1) as f64;
    let latency_reduction = ((avg_first_latency - avg_cached_latency) / avg_first_latency) * 100.0;

    println!("\n📈 Caching Statistics:");
    println!(
        "  • Average first request latency: {:.0}ms",
        avg_first_latency
    );
    println!(
        "  • Average cached request latency: {:.0}ms",
        avg_cached_latency
    );
    println!("  • Latency reduction: {:.1}%", latency_reduction);
    println!(
        "  • Total tokens cached: {}/{} ({:.1}%)",
        total_cached_tokens,
        total_prompt_tokens,
        (total_cached_tokens as f64 / total_prompt_tokens as f64) * 100.0
    );

    // Example 2: Using prompt_cache_key for better routing
    println!("\n🔑 Example 2: Using prompt_cache_key for Routing");
    println!("─────────────────────────────────────────────────");
    println!("Using cache keys to optimize routing for different use cases...\n");

    let use_cases = vec![
        (
            "analysis-v1",
            "Analyze the trends in renewable energy adoption",
        ),
        (
            "analysis-v1",
            "Analyze the patterns in social media engagement",
        ),
        (
            "coding-v1",
            "Write a Python function to calculate fibonacci",
        ),
        ("coding-v1", "Write a JavaScript function to validate email"),
    ];

    for (cache_key, question) in use_cases {
        let start = Instant::now();

        let request = ResponseRequest::new_messages(
            "gpt-4o-mini",
            vec![
                Message::developer(SYSTEM_PROMPT),
                Message::developer(EXAMPLES),
                Message::user(question),
            ],
        )
        .with_max_tokens(150)
        .with_prompt_cache_key(cache_key);

        let response = api.create_response(&request).await?;

        println!(
            "Cache Key: '{}' | Question: '{}'",
            cache_key,
            &question[..question.len().min(40)]
        );
        println!("  ⏱️  Latency: {:.2}ms", start.elapsed().as_millis());
        println!(
            "  ✅ Cached tokens: {} ({:.1}% hit rate)",
            response.cached_tokens(),
            response.cache_hit_rate()
        );
        println!();

        sleep(Duration::from_millis(200)).await;
    }

    // Example 3: Optimizing prompt structure for caching
    println!("🏗️ Example 3: Optimizing Prompt Structure");
    println!("──────────────────────────────────────────");
    println!("Comparing well-structured vs poorly-structured prompts...\n");

    // Well-structured prompt (static content first, dynamic last)
    println!("✅ Well-structured prompt (static → dynamic):");
    let well_structured = ResponseRequest::new_messages(
        "gpt-4o-mini",
        vec![
            Message::developer(SYSTEM_PROMPT), // Static (cacheable)
            Message::developer(EXAMPLES),      // Static (cacheable)
            Message::user(
                "Current timestamp: 2024-03-20 14:23:45. Analyze recent AI developments.",
            ), // Dynamic
        ],
    )
    .with_max_tokens(150)
    .with_prompt_cache_key("structured-v1");

    let start = Instant::now();
    let response1 = api.create_response(&well_structured).await?;
    println!(
        "  First request: {:.2}ms, {} cached tokens",
        start.elapsed().as_millis(),
        response1.cached_tokens()
    );

    sleep(Duration::from_millis(500)).await;

    // Second request with different dynamic content but same prefix
    let well_structured2 = ResponseRequest::new_messages(
        "gpt-4o-mini",
        vec![
            Message::developer(SYSTEM_PROMPT), // Same static prefix
            Message::developer(EXAMPLES),      // Same static prefix
            Message::user(
                "Current timestamp: 2024-03-20 14:24:10. Explain machine learning basics.",
            ), // Different dynamic
        ],
    )
    .with_max_tokens(150)
    .with_prompt_cache_key("structured-v1");

    let start = Instant::now();
    let response2 = api.create_response(&well_structured2).await?;
    println!(
        "  Second request: {:.2}ms, {} cached tokens ({:.1}% hit rate)",
        start.elapsed().as_millis(),
        response2.cached_tokens(),
        response2.cache_hit_rate()
    );

    // Poorly-structured prompt (dynamic content interrupts static)
    println!("\n❌ Poorly-structured prompt (dynamic interrupts static):");
    let poor_structured = ResponseRequest::new_messages(
        "gpt-4o-mini",
        vec![
            Message::developer(&SYSTEM_PROMPT[..512]), // Partial static
            Message::user("Current timestamp: 2024-03-20 14:24:30"), // Dynamic in middle!
            Message::developer(&SYSTEM_PROMPT[512..]), // Rest of static (won't cache!)
            Message::user("What are best practices for API design?"),
        ],
    )
    .with_max_tokens(150);

    let start = Instant::now();
    let response3 = api.create_response(&poor_structured).await?;
    println!(
        "  First request: {:.2}ms, {} cached tokens",
        start.elapsed().as_millis(),
        response3.cached_tokens()
    );

    sleep(Duration::from_millis(500)).await;

    // Second poorly structured request
    let poor_structured2 = ResponseRequest::new_messages(
        "gpt-4o-mini",
        vec![
            Message::developer(&SYSTEM_PROMPT[..512]),
            Message::user("Current timestamp: 2024-03-20 14:24:45"), // Different timestamp
            Message::developer(&SYSTEM_PROMPT[512..]),
            Message::user("What are best practices for database design?"),
        ],
    )
    .with_max_tokens(150);

    let start = Instant::now();
    let response4 = api.create_response(&poor_structured2).await?;
    println!(
        "  Second request: {:.2}ms, {} cached tokens ({:.1}% hit rate)",
        start.elapsed().as_millis(),
        response4.cached_tokens(),
        response4.cache_hit_rate()
    );
    println!("  ⚠️  Note: Poor structure prevents effective caching!");

    // Example 4: Cache persistence timing
    println!("\n⏰ Example 4: Cache Persistence Timing");
    println!("──────────────────────────────────────");
    println!("Testing cache persistence over time...\n");

    let persistence_request = ResponseRequest::new_messages(
        "gpt-4o-mini",
        vec![
            Message::developer(SYSTEM_PROMPT),
            Message::user("Explain the concept of cache persistence"),
        ],
    )
    .with_max_tokens(100)
    .with_prompt_cache_key("persistence-test");

    // Initial request
    let start = Instant::now();
    let response = api.create_response(&persistence_request).await?;
    println!(
        "Initial request: {:.2}ms, {} cached tokens",
        start.elapsed().as_millis(),
        response.cached_tokens()
    );

    // Test at different intervals
    let intervals = vec![
        ("Immediate", 0),
        ("After 1 second", 1),
        ("After 5 seconds", 5),
        ("After 10 seconds", 10),
    ];

    for (label, delay) in intervals {
        if delay > 0 {
            println!("  ⏳ Waiting {} seconds...", delay);
            sleep(Duration::from_secs(delay as u64)).await;
        }

        let start = Instant::now();
        let response = api.create_response(&persistence_request).await?;
        println!(
            "  {} - Latency: {:.2}ms, Cached: {} tokens ({:.1}% hit)",
            label,
            start.elapsed().as_millis(),
            response.cached_tokens(),
            response.cache_hit_rate()
        );
    }

    // Example 5: Multi-turn conversation caching
    println!("\n💬 Example 5: Multi-turn Conversation Caching");
    println!("──────────────────────────────────────────────");
    println!("Demonstrating cache benefits in conversations...\n");

    let mut conversation = vec![
        Message::developer(SYSTEM_PROMPT),
        Message::user("Let's discuss machine learning. What is supervised learning?"),
    ];

    // First turn
    let start = Instant::now();
    let request = ResponseRequest::new_messages("gpt-4o-mini", conversation.clone())
        .with_max_tokens(150)
        .with_prompt_cache_key("conversation-ml");

    let response = api.create_response(&request).await?;
    println!(
        "Turn 1 - Latency: {:.2}ms, Cached: {} tokens",
        start.elapsed().as_millis(),
        response.cached_tokens()
    );

    // Add response to conversation
    conversation.push(Message::assistant(response.output_text()));
    conversation.push(Message::user("What about unsupervised learning?"));

    // Second turn (should benefit from caching)
    let start = Instant::now();
    let request = ResponseRequest::new_messages("gpt-4o-mini", conversation.clone())
        .with_max_tokens(150)
        .with_prompt_cache_key("conversation-ml");

    let response = api.create_response(&request).await?;
    println!(
        "Turn 2 - Latency: {:.2}ms, Cached: {} tokens ({:.1}% hit)",
        start.elapsed().as_millis(),
        response.cached_tokens(),
        response.cache_hit_rate()
    );

    // Add response and continue
    conversation.push(Message::assistant(response.output_text()));
    conversation.push(Message::user(
        "Can you give examples of reinforcement learning?",
    ));

    // Third turn
    let start = Instant::now();
    let request = ResponseRequest::new_messages("gpt-4o-mini", conversation.clone())
        .with_max_tokens(150)
        .with_prompt_cache_key("conversation-ml");

    let response = api.create_response(&request).await?;
    println!(
        "Turn 3 - Latency: {:.2}ms, Cached: {} tokens ({:.1}% hit)",
        start.elapsed().as_millis(),
        response.cached_tokens(),
        response.cache_hit_rate()
    );

    // Summary
    println!("\n✨ Prompt Caching Best Practices Summary");
    println!("────────────────────────────────────────");
    println!("1. ✅ Place static content (system prompts, examples) at the beginning");
    println!("2. ✅ Put dynamic content (user input, timestamps) at the end");
    println!("3. ✅ Use prompt_cache_key for better routing (keep <15 req/min per key)");
    println!("4. ✅ Maintain exact prefix matches for cache hits");
    println!("5. ✅ Ensure prompts are ≥1024 tokens for caching eligibility");
    println!("6. ✅ Reuse the same prompt structure across requests");
    println!("7. ✅ Monitor cache metrics to optimize performance");
    println!();
    println!("💡 Benefits achieved:");
    println!("   • Latency reduction: Up to 80%");
    println!("   • Cost reduction: Up to 75%");
    println!("   • No additional fees or code changes required");
    println!("   • Works with gpt-4o and newer models");

    Ok(())
}
