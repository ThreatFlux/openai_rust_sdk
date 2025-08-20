//! # Embeddings API
//!
//! This module provides access to OpenAI's embeddings API for creating
//! vector representations of text.

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use crate::models::embeddings::{EmbeddingRequest, EmbeddingResponse};

/// Embeddings API client
pub struct EmbeddingsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl EmbeddingsApi {
    /// Create a new Embeddings API client
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
        })
    }

    /// Create a new client with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Create embeddings for the given input
    pub async fn create_embeddings(&self, request: &EmbeddingRequest) -> Result<EmbeddingResponse> {
        self.http_client.post("/v1/embeddings", request).await
    }

    /// Create embeddings for a single text input
    pub async fn embed_text(&self, model: &str, text: &str) -> Result<Vec<f32>> {
        let request = EmbeddingRequest::new(model, text);
        let response = self.create_embeddings(&request).await?;

        response
            .get_first_embedding()
            .ok_or_else(|| OpenAIError::ParseError("No embedding returned".to_string()))
    }

    /// Create embeddings for multiple text inputs
    pub async fn embed_texts(&self, model: &str, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let request = EmbeddingRequest::new_batch(model, texts);
        let response = self.create_embeddings(&request).await?;

        Ok(response.get_embeddings())
    }

    /// Create embeddings with custom dimensions
    pub async fn embed_with_dimensions(
        &self,
        model: &str,
        text: &str,
        dimensions: u32,
    ) -> Result<Vec<f32>> {
        let request = EmbeddingRequest::new(model, text).with_dimensions(dimensions);
        let response = self.create_embeddings(&request).await?;

        response
            .get_first_embedding()
            .ok_or_else(|| OpenAIError::ParseError("No embedding returned".to_string()))
    }

    /// Calculate cosine similarity between two vectors
    #[must_use]
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }

    /// Calculate euclidean distance between two vectors
    #[must_use]
    pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return f32::MAX;
        }

        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Find the most similar text from a list based on cosine similarity
    pub async fn find_most_similar(
        &self,
        model: &str,
        query: &str,
        candidates: Vec<String>,
    ) -> Result<(usize, f32)> {
        if candidates.is_empty() {
            return Err(OpenAIError::InvalidRequest(
                "No candidates provided".to_string(),
            ));
        }

        // Get query embedding
        let query_embedding = self.embed_text(model, query).await?;

        // Get candidate embeddings
        let candidate_embeddings = self.embed_texts(model, candidates).await?;

        // Find most similar
        let mut best_index = 0;
        let mut best_similarity = f32::MIN;

        for (i, embedding) in candidate_embeddings.iter().enumerate() {
            let similarity = Self::cosine_similarity(&query_embedding, embedding);
            if similarity > best_similarity {
                best_similarity = similarity;
                best_index = i;
            }
        }

        Ok((best_index, best_similarity))
    }
}

/// Helper functions for working with embeddings
pub struct EmbeddingUtils;

impl EmbeddingUtils {
    /// Normalize a vector to unit length
    pub fn normalize(vector: &mut [f32]) {
        let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in vector.iter_mut() {
                *x /= norm;
            }
        }
    }

    /// Calculate the mean of multiple embeddings
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn mean_embedding(embeddings: &[Vec<f32>]) -> Vec<f32> {
        if embeddings.is_empty() {
            return Vec::new();
        }

        let dim = embeddings[0].len();
        let mut mean = vec![0.0; dim];

        for embedding in embeddings {
            for (i, val) in embedding.iter().enumerate() {
                mean[i] += val;
            }
        }

        let count = embeddings.len() as f32;
        for val in &mut mean {
            *val /= count;
        }

        mean
    }

    /// Calculate weighted mean of embeddings
    #[must_use]
    pub fn weighted_mean_embedding(embeddings: &[Vec<f32>], weights: &[f32]) -> Vec<f32> {
        if embeddings.is_empty() || embeddings.len() != weights.len() {
            return Vec::new();
        }

        let dim = embeddings[0].len();
        let mut weighted_mean = vec![0.0; dim];
        let total_weight: f32 = weights.iter().sum();

        if total_weight == 0.0 {
            return vec![0.0; dim];
        }

        for (embedding, weight) in embeddings.iter().zip(weights.iter()) {
            for (i, val) in embedding.iter().enumerate() {
                weighted_mean[i] += val * weight;
            }
        }

        for val in &mut weighted_mean {
            *val /= total_weight;
        }

        weighted_mean
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(EmbeddingsApi::cosine_similarity(&a, &b), 1.0);

        let c = vec![0.0, 1.0, 0.0];
        assert_eq!(EmbeddingsApi::cosine_similarity(&a, &c), 0.0);

        let d = vec![-1.0, 0.0, 0.0];
        assert_eq!(EmbeddingsApi::cosine_similarity(&a, &d), -1.0);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        assert_eq!(EmbeddingsApi::euclidean_distance(&a, &b), 5.0);
    }

    #[test]
    fn test_normalize() {
        let mut vec = vec![3.0, 4.0];
        EmbeddingUtils::normalize(&mut vec);
        assert!((vec[0] - 0.6).abs() < 0.001);
        assert!((vec[1] - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_mean_embedding() {
        let embeddings = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let mean = EmbeddingUtils::mean_embedding(&embeddings);
        assert_eq!(mean, vec![4.0, 5.0, 6.0]);
    }
}
