#![allow(clippy::pedantic, clippy::nursery)]
//! # Embeddings Demo
//!
//! This example demonstrates how to use `OpenAI`'s embeddings API to:
//! - Create text embeddings
//! - Compare text similarity
//! - Build a simple semantic search
//! - Cluster similar texts
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example embeddings_demo
//! ```

use openai_rust_sdk::{
    api::{
        common::ApiClientConstructors,
        embeddings::{EmbeddingUtils, EmbeddingsApi},
    },
    models::embeddings::{EmbeddingBuilder, EmbeddingModels},
};
use std::env;

/// Demonstrates basic embedding creation
async fn demo_basic_embeddings(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Example 1: Basic Embedding Creation");
    println!("──────────────────────────────────────");

    let text = "OpenAI embeddings transform text into high-dimensional vectors.";
    println!("Input text: \"{text}\"");

    let embedding = api.embed_text(EmbeddingModels::ADA_002, text).await?;
    println!("✅ Created embedding with {} dimensions", embedding.len());
    println!("First 10 values: {:?}", &embedding[..10]);

    Ok(())
}

/// Demonstrates batch embeddings processing
async fn demo_batch_embeddings(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📚 Example 2: Batch Embeddings");
    println!("───────────────────────────────");

    let texts = vec![
        "Machine learning is a subset of artificial intelligence.".to_string(),
        "Deep learning uses neural networks with multiple layers.".to_string(),
        "Natural language processing helps computers understand human language.".to_string(),
        "Computer vision enables machines to interpret visual information.".to_string(),
    ];

    println!("Creating embeddings for {} texts...", texts.len());
    let embeddings = api
        .embed_texts(EmbeddingModels::ADA_002, texts.clone())
        .await?;

    for (i, text) in texts.iter().enumerate() {
        println!(
            "  [{}] \"{}...\" → {} dimensions",
            i + 1,
            &text[..text.len().min(40)],
            embeddings[i].len()
        );
    }

    Ok(())
}

/// Demonstrates text similarity comparison
async fn demo_similarity_comparison(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Example 3: Text Similarity Comparison");
    println!("────────────────────────────────────────");

    let base_text = "Artificial intelligence is transforming technology";
    let similar_text = "AI is revolutionizing the tech industry";
    let different_text = "I enjoy cooking Italian pasta dishes";

    println!("Base text: \"{base_text}\"");

    let base_embedding = api.embed_text(EmbeddingModels::ADA_002, base_text).await?;
    let similar_embedding = api
        .embed_text(EmbeddingModels::ADA_002, similar_text)
        .await?;
    let different_embedding = api
        .embed_text(EmbeddingModels::ADA_002, different_text)
        .await?;

    let similarity_1 = EmbeddingsApi::cosine_similarity(&base_embedding, &similar_embedding);
    let similarity_2 = EmbeddingsApi::cosine_similarity(&base_embedding, &different_embedding);

    println!("\nCosine Similarities:");
    println!("  Similar text: \"{similar_text}\"");
    println!("  → Similarity: {similarity_1:.4} (higher = more similar)");
    println!("\n  Different text: \"{different_text}\"");
    println!("  → Similarity: {similarity_2:.4} (lower = less similar)");

    Ok(())
}

/// Demonstrates semantic search functionality
async fn demo_semantic_search(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔎 Example 4: Semantic Search");
    println!("─────────────────────────────");

    let documents = vec![
        "The quick brown fox jumps over the lazy dog.".to_string(),
        "Python is a popular programming language for data science.".to_string(),
        "Climate change is affecting global weather patterns.".to_string(),
        "The stock market showed strong gains this quarter.".to_string(),
        "Quantum computing could revolutionize cryptography.".to_string(),
        "Mediterranean diet is known for its health benefits.".to_string(),
        "Electric vehicles are becoming more affordable.".to_string(),
        "Machine learning models require large datasets.".to_string(),
    ];

    let query = "artificial intelligence and data";

    println!("Query: \"{query}\"");
    println!("\nSearching through {} documents...", documents.len());

    let (best_index, best_score) = api
        .find_most_similar(EmbeddingModels::ADA_002, query, documents.clone())
        .await?;

    println!("\n✅ Most relevant document:");
    println!("  [{}] \"{}\"", best_index + 1, documents[best_index]);
    println!("  Similarity score: {best_score:.4}");

    // Show top 3 results
    display_top_results(api, query, &documents).await?;

    Ok(())
}

/// Helper function to display top search results
async fn display_top_results(
    api: &EmbeddingsApi,
    query: &str,
    documents: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let query_embedding = api.embed_text(EmbeddingModels::ADA_002, query).await?;
    let doc_embeddings = api
        .embed_texts(EmbeddingModels::ADA_002, documents.to_vec())
        .await?;

    let mut similarities: Vec<(usize, f32)> = doc_embeddings
        .iter()
        .enumerate()
        .map(|(i, emb)| (i, EmbeddingsApi::cosine_similarity(&query_embedding, emb)))
        .collect();

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n📊 Top 3 Results:");
    for (rank, (idx, score)) in similarities.iter().take(3).enumerate() {
        println!(
            "  {}. [score: {:.4}] \"{}\"",
            rank + 1,
            score,
            &documents[*idx][..documents[*idx].len().min(60)]
        );
    }

    Ok(())
}

/// Demonstrates custom dimensions with text-embedding-3
async fn demo_custom_dimensions(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚙️ Example 5: Custom Dimensions (text-embedding-3)");
    println!("───────────────────────────────────────────────────");

    let text = "OpenAI's text-embedding-3 models support configurable dimensions";

    // Standard dimensions
    let standard_embedding = api
        .embed_text(EmbeddingModels::EMBEDDING_3_SMALL, text)
        .await?;

    // Reduced dimensions for efficiency
    let reduced_embedding = api
        .embed_with_dimensions(
            EmbeddingModels::EMBEDDING_3_SMALL,
            text,
            256, // Reduce to 256 dimensions
        )
        .await?;

    println!("Text: \"{text}\"");
    println!(
        "Standard embedding: {} dimensions",
        standard_embedding.len()
    );
    println!("Reduced embedding: {} dimensions", reduced_embedding.len());
    println!(
        "Memory saved: {:.1}%",
        (1.0 - reduced_embedding.len() as f32 / standard_embedding.len() as f32) * 100.0
    );

    Ok(())
}

/// Demonstrates embedding utilities (mean and weighted mean)
async fn demo_embedding_utilities(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🛠️ Example 6: Embedding Utilities");
    println!("──────────────────────────────────");

    // Mean of multiple embeddings (useful for document representation)
    let doc_parts = vec![
        "Introduction to machine learning.".to_string(),
        "Supervised learning uses labeled data.".to_string(),
        "Unsupervised learning finds patterns.".to_string(),
    ];

    let part_embeddings = api
        .embed_texts(EmbeddingModels::ADA_002, doc_parts.clone())
        .await?;
    let mean_embedding = EmbeddingUtils::mean_embedding(&part_embeddings);

    println!("Document parts:");
    for (i, part) in doc_parts.iter().enumerate() {
        println!("  {}. {}", i + 1, part);
    }
    println!(
        "\n✅ Created mean embedding of {} dimensions",
        mean_embedding.len()
    );

    // Weighted mean (useful when parts have different importance)
    let weights = vec![0.5, 0.3, 0.2]; // First part is most important
    let _weighted_mean = EmbeddingUtils::weighted_mean_embedding(&part_embeddings, &weights);

    println!("Weights: {weights:?}");
    println!("✅ Created weighted mean embedding");

    Ok(())
}

/// Demonstrates base64 encoding format
async fn demo_base64_encoding(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔐 Example 7: Base64 Encoding Format");
    println!("─────────────────────────────────────");

    let request = EmbeddingBuilder::new(EmbeddingModels::ADA_002, "Test text")
        .base64_format()
        .build();

    let response = api.create_embeddings(&request).await?;

    if let Some(base64_embeddings) = response.data.first().and_then(|e| e.as_base64()) {
        println!("Base64 encoded embedding (first 100 chars):");
        println!("{}", &base64_embeddings[..base64_embeddings.len().min(100)]);
        println!("Full length: {} characters", base64_embeddings.len());
    }

    Ok(())
}

/// Demonstrates different distance metrics
fn demo_distance_metrics() {
    println!("\n📏 Example 8: Different Distance Metrics");
    println!("────────────────────────────────────────");

    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![2.0, 3.0, 4.0];

    let cosine_sim = EmbeddingsApi::cosine_similarity(&vec1, &vec2);
    let euclidean_dist = EmbeddingsApi::euclidean_distance(&vec1, &vec2);

    println!("Vector 1: {vec1:?}");
    println!("Vector 2: {vec2:?}");
    println!("\nDistance Metrics:");
    println!("  Cosine similarity: {cosine_sim:.4} (1.0 = identical direction)");
    println!("  Euclidean distance: {euclidean_dist:.4} (0.0 = identical position)");
}

/// Demonstrates text clustering functionality
async fn demo_text_clustering(api: &EmbeddingsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Example 9: Clustering Similar Texts");
    println!("──────────────────────────────────────");

    let items = vec![
        "Python programming tutorial".to_string(),
        "JavaScript web development".to_string(),
        "Healthy breakfast recipes".to_string(),
        "Machine learning with Python".to_string(),
        "Quick dinner ideas".to_string(),
        "React.js frontend framework".to_string(),
        "Vegetarian meal planning".to_string(),
        "Deep learning neural networks".to_string(),
    ];

    println!("Items to cluster:");
    for (i, item) in items.iter().enumerate() {
        println!("  {}. {}", i + 1, item);
    }

    let item_embeddings = api
        .embed_texts(EmbeddingModels::ADA_002, items.clone())
        .await?;

    let clusters = perform_clustering(&item_embeddings, 0.8);

    println!("\n📊 Clusters (similarity threshold: 0.8):");
    for (i, cluster) in clusters.iter().enumerate() {
        println!("\nCluster {}:", i + 1);
        for idx in cluster {
            println!("  - {}", items[*idx]);
        }
    }

    Ok(())
}

/// Helper function to perform simple clustering
fn perform_clustering(embeddings: &[Vec<f32>], threshold: f32) -> Vec<Vec<usize>> {
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    let mut clustered = vec![false; embeddings.len()];

    for i in 0..embeddings.len() {
        if clustered[i] {
            continue;
        }

        let mut cluster = vec![i];
        clustered[i] = true;

        for j in (i + 1)..embeddings.len() {
            if !clustered[j] {
                let similarity = EmbeddingsApi::cosine_similarity(&embeddings[i], &embeddings[j]);

                if similarity > threshold {
                    cluster.push(j);
                    clustered[j] = true;
                }
            }
        }

        clusters.push(cluster);
    }

    clusters
}

/// Displays the summary and use cases
fn display_summary() {
    println!("\n✨ Embeddings API Summary");
    println!("─────────────────────────");
    println!("• Create vector representations of text");
    println!("• Compare semantic similarity between texts");
    println!("• Build semantic search systems");
    println!("• Cluster similar documents");
    println!("• Support for multiple models and dimensions");
    println!("• Batch processing for efficiency");
    println!("• Multiple distance metrics available");

    println!("\n💡 Use Cases:");
    println!("• Semantic search engines");
    println!("• Document clustering");
    println!("• Recommendation systems");
    println!("• Duplicate detection");
    println!("• Content moderation");
    println!("• Question answering systems");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🔢 OpenAI Embeddings Demo");
    println!("=========================");

    let api = EmbeddingsApi::new(api_key)?;

    // Run all demonstrations
    demo_basic_embeddings(&api).await?;
    demo_batch_embeddings(&api).await?;
    demo_similarity_comparison(&api).await?;
    demo_semantic_search(&api).await?;
    demo_custom_dimensions(&api).await?;
    demo_embedding_utilities(&api).await?;
    demo_base64_encoding(&api).await?;
    demo_distance_metrics();
    demo_text_clustering(&api).await?;
    display_summary();

    Ok(())
}
