//! # Embeddings Models
//!
//! Data structures for the OpenAI Embeddings API

use serde::{Deserialize, Serialize};

/// Request for creating embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Input text to embed (string, array of strings, or array of token arrays)
    pub input: EmbeddingInput,

    /// ID of the model to use (e.g., "text-embedding-ada-002", "text-embedding-3-small")
    pub model: String,

    /// Number of dimensions for the output embeddings (text-embedding-3 and later)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,

    /// Format to return embeddings in ("float" or "base64")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,

    /// Unique identifier for end-user tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Input for embedding requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text string
    String(String),
    /// Array of text strings
    StringArray(Vec<String>),
    /// Array of token arrays
    TokenArray(Vec<Vec<u32>>),
}

/// Encoding format for embeddings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EncodingFormat {
    /// Return embeddings as float arrays
    Float,
    /// Return embeddings as base64-encoded strings
    Base64,
}

/// Response from embeddings API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Object type (always "list")
    pub object: String,

    /// List of embedding objects
    pub data: Vec<Embedding>,

    /// Model used for embeddings
    pub model: String,

    /// Token usage statistics
    pub usage: EmbeddingUsage,
}

/// Individual embedding object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// Object type (always "embedding")
    pub object: String,

    /// The embedding vector
    pub embedding: EmbeddingVector,

    /// Index of the embedding in the list
    pub index: usize,
}

/// Embedding vector that can be either floats or base64
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingVector {
    /// Float array representation
    Float(Vec<f32>),
    /// Base64-encoded string representation
    Base64(String),
}

/// Usage statistics for embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,

    /// Total number of tokens used
    pub total_tokens: u32,
}

impl EmbeddingRequest {
    /// Create a new embedding request with a single string
    pub fn new(model: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            input: EmbeddingInput::String(input.into()),
            model: model.into(),
            dimensions: None,
            encoding_format: None,
            user: None,
        }
    }

    /// Create a new embedding request with multiple strings
    pub fn new_batch(model: impl Into<String>, inputs: Vec<String>) -> Self {
        Self {
            input: EmbeddingInput::StringArray(inputs),
            model: model.into(),
            dimensions: None,
            encoding_format: None,
            user: None,
        }
    }

    /// Create a new embedding request with token arrays
    pub fn new_tokens(model: impl Into<String>, tokens: Vec<Vec<u32>>) -> Self {
        Self {
            input: EmbeddingInput::TokenArray(tokens),
            model: model.into(),
            dimensions: None,
            encoding_format: None,
            user: None,
        }
    }

    /// Set the number of dimensions for the output
    #[must_use]
    pub fn with_dimensions(mut self, dimensions: u32) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Set the encoding format
    #[must_use]
    pub fn with_encoding_format(mut self, format: EncodingFormat) -> Self {
        self.encoding_format = Some(format);
        self
    }

    /// Set the user identifier
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl EmbeddingResponse {
    /// Get all embeddings as float vectors (if format is float)
    #[must_use]
    pub fn get_embeddings(&self) -> Vec<Vec<f32>> {
        self.data
            .iter()
            .filter_map(|e| match &e.embedding {
                EmbeddingVector::Float(vec) => Some(vec.clone()),
                _ => None,
            })
            .collect()
    }

    /// Get the first embedding as a float vector
    #[must_use]
    pub fn get_first_embedding(&self) -> Option<Vec<f32>> {
        self.data.first().and_then(|e| match &e.embedding {
            EmbeddingVector::Float(vec) => Some(vec.clone()),
            _ => None,
        })
    }

    /// Get all embeddings as base64 strings (if format is base64)
    #[must_use]
    pub fn get_base64_embeddings(&self) -> Vec<String> {
        self.data
            .iter()
            .filter_map(|e| match &e.embedding {
                EmbeddingVector::Base64(s) => Some(s.clone()),
                _ => None,
            })
            .collect()
    }

    /// Get the dimension of the embeddings
    #[must_use]
    pub fn dimension(&self) -> Option<usize> {
        self.data.first().and_then(|e| match &e.embedding {
            EmbeddingVector::Float(vec) => Some(vec.len()),
            _ => None,
        })
    }
}

impl Embedding {
    /// Get the embedding as a float vector
    #[must_use]
    pub fn as_float_vector(&self) -> Option<&Vec<f32>> {
        match &self.embedding {
            EmbeddingVector::Float(vec) => Some(vec),
            _ => None,
        }
    }

    /// Get the embedding as a base64 string
    #[must_use]
    pub fn as_base64(&self) -> Option<&String> {
        match &self.embedding {
            EmbeddingVector::Base64(s) => Some(s),
            _ => None,
        }
    }
}

/// Common embedding models
pub struct EmbeddingModels;

impl EmbeddingModels {
    /// Legacy model (1536 dimensions)
    pub const ADA_002: &'static str = "text-embedding-ada-002";

    /// Small model (1536 dimensions, configurable)
    pub const EMBEDDING_3_SMALL: &'static str = "text-embedding-3-small";

    /// Large model (3072 dimensions, configurable)
    pub const EMBEDDING_3_LARGE: &'static str = "text-embedding-3-large";
}

/// Builder for creating embedding requests
pub struct EmbeddingBuilder {
    /// The embedding request being built
    request: EmbeddingRequest,
}

impl EmbeddingBuilder {
    /// Create a new builder with a single input
    pub fn new(model: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            request: EmbeddingRequest::new(model, input),
        }
    }

    /// Create a new builder with multiple inputs
    pub fn new_batch(model: impl Into<String>, inputs: Vec<String>) -> Self {
        Self {
            request: EmbeddingRequest::new_batch(model, inputs),
        }
    }

    /// Set the number of dimensions (only for text-embedding-3 models)
    #[must_use]
    pub fn dimensions(mut self, dims: u32) -> Self {
        self.request.dimensions = Some(dims);
        self
    }

    /// Use float encoding format (default)
    #[must_use]
    pub fn float_format(mut self) -> Self {
        self.request.encoding_format = Some(EncodingFormat::Float);
        self
    }

    /// Use base64 encoding format
    #[must_use]
    pub fn base64_format(mut self) -> Self {
        self.request.encoding_format = Some(EncodingFormat::Base64);
        self
    }

    /// Set user identifier
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request.user = Some(user.into());
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> EmbeddingRequest {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_request_creation() {
        let req = EmbeddingRequest::new("text-embedding-ada-002", "Hello world");
        assert_eq!(req.model, "text-embedding-ada-002");
        matches!(req.input, EmbeddingInput::String(s) if s == "Hello world");
    }

    #[test]
    fn test_embedding_builder() {
        let req = EmbeddingBuilder::new(EmbeddingModels::EMBEDDING_3_SMALL, "Test")
            .dimensions(512)
            .float_format()
            .user("user123")
            .build();

        assert_eq!(req.dimensions, Some(512));
        assert!(matches!(req.encoding_format, Some(EncodingFormat::Float)));
        assert_eq!(req.user, Some("user123".to_string()));
    }

    #[test]
    fn test_batch_embeddings() {
        let inputs = vec![
            "First".to_string(),
            "Second".to_string(),
            "Third".to_string(),
        ];
        let req = EmbeddingRequest::new_batch("text-embedding-ada-002", inputs.clone());

        match req.input {
            EmbeddingInput::StringArray(arr) => assert_eq!(arr, inputs),
            _ => panic!("Expected StringArray"),
        }
    }
}
