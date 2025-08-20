//! # `OpenAI` Batch Job Generator
//!
//! This module provides functionality to generate `OpenAI` Batch API jobs
//! for automated YARA rule creation and testing.
//!
//! ## Features
//!
//! - **Batch Job Creation**: Generate properly formatted batch API requests
//! - **Test Suite Templates**: Pre-defined prompt sets for different use cases
//! - **JSONL Output**: Correctly formatted output for `OpenAI` Batch API
//! - **Configurable Models**: Support for different `OpenAI` models
//!
//! ## Example
//!
//! ```
//! use openai_rust_sdk::testing::BatchJobGenerator;
//! use std::path::Path;
//!
//! let generator = BatchJobGenerator::new(Some("gpt-4".to_string()));
//! generator.generate_test_suite(
//!     Path::new("batch_jobs.jsonl"),
//!     "comprehensive"
//! )?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// A single batch job request for the `OpenAI` Batch API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJobRequest {
    /// Custom identifier for tracking this specific request
    pub custom_id: String,
    /// HTTP method (typically "POST")
    pub method: String,
    /// API endpoint URL
    pub url: String,
    /// Request body containing the chat completion parameters
    pub body: BatchJobBody,
}

/// Body of a batch job request containing chat completion parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJobBody {
    /// `OpenAI` model to use (e.g., "gpt-4", "gpt-3.5-turbo")
    pub model: String,
    /// Chat messages including system and user prompts
    pub messages: Vec<ChatMessage>,
    /// Maximum tokens to generate in the response
    pub max_tokens: Option<u32>,
    /// Sampling temperature (0.0-2.0)
    pub temperature: Option<f64>,
}

/// A single chat message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of the message sender ("system", "user", "assistant")
    pub role: String,
    /// Content of the message
    pub content: String,
}

/// Generator for `OpenAI` Batch API jobs focused on YARA rule creation
///
/// The generator creates properly formatted batch job files that can be
/// submitted to `OpenAI`'s Batch API for automated YARA rule generation.
#[allow(dead_code)]
pub struct BatchJobGenerator {
    /// System prompt used for all generated jobs
    system_prompt: String,
    /// `OpenAI` model to use for generation
    model: String,
}

impl BatchJobGenerator {
    /// Creates a new batch job generator
    ///
    /// # Arguments
    ///
    /// * `model` - Optional `OpenAI` model name. Defaults to "gpt-4" if not specified
    ///
    /// # Example
    ///
    /// ```
    /// use openai_rust_sdk::testing::BatchJobGenerator;
    ///
    /// // Use default model (gpt-4)
    /// let generator = BatchJobGenerator::new(None);
    ///
    /// // Use specific model
    /// let generator = BatchJobGenerator::new(Some("gpt-3.5-turbo".to_string()));
    /// ```
    #[must_use]
    #[allow(dead_code)]
    pub fn new(model: Option<String>) -> Self {
        let system_prompt = "You are an expert YARA rule developer. Create syntactically correct YARA rules. Return only the YARA rule code.";

        Self {
            system_prompt: system_prompt.to_string(),
            model: model.unwrap_or_else(|| "gpt-4".to_string()),
        }
    }

    /// Generates a batch job file for a specific test suite
    ///
    /// Creates a JSONL file containing batch job requests for the specified
    /// test suite. Each line in the output file represents a single batch job
    /// request that can be submitted to `OpenAI`'s Batch API.
    ///
    /// # Arguments
    ///
    /// * `output_path` - Path where the batch job file will be written
    /// * `suite_name` - Name of the test suite ("basic", "malware", "comprehensive")
    ///
    /// # Available Test Suites
    ///
    /// - `"basic"`: Simple rules for testing basic functionality
    /// - `"malware"`: Advanced rules for malware detection
    /// - `"comprehensive"`: Complete set including all rule types
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error if file writing fails
    ///
    /// # Example
    ///
    /// ```
    /// use openai_rust_sdk::testing::BatchJobGenerator;
    /// use std::path::Path;
    ///
    /// let generator = BatchJobGenerator::new(None);
    /// generator.generate_test_suite(
    ///     Path::new("comprehensive_batch.jsonl"),
    ///     "comprehensive"
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    #[allow(dead_code)]
    pub fn generate_test_suite(&self, output_path: &Path, suite_name: &str) -> Result<()> {
        let prompts = match suite_name {
            "basic" => vec![
                "Create a YARA rule that detects files containing 'Hello World'.",
                "Generate a YARA rule to detect PE headers (MZ signature).",
                "Write a YARA rule for detecting error/warning/debug strings.",
            ],
            "malware" => vec![
                "Generate a YARA rule to detect UPX packed executables.",
                "Create a YARA rule for ransomware detection based on encryption strings.",
                "Write a YARA rule to detect keylogger APIs.",
            ],
            "comprehensive" => vec![
                "Create a YARA rule that detects files containing 'Hello World'.",
                "Generate a YARA rule to detect PE headers (MZ signature).",
                "Generate a YARA rule to detect UPX packed executables.",
                "Create a YARA rule for ransomware detection.",
                "Write a YARA rule using regex to detect email addresses.",
                "Create a YARA rule to detect cryptocurrency addresses.",
                "Generate a YARA rule with external variables for file size detection.",
                "Write a YARA rule using for loops to detect repeating patterns.",
                "Create a YARA rule that combines multiple modules for comprehensive analysis.",
                "Generate a YARA rule for detecting obfuscated JavaScript code.",
            ],
            _ => return Err(anyhow::anyhow!("Unknown test suite: {}", suite_name)),
        };

        let mut requests = Vec::new();
        for (i, prompt) in prompts.iter().enumerate() {
            let request = BatchJobRequest {
                custom_id: format!("{}_{:03}", suite_name, i + 1),
                method: "POST".to_string(),
                url: "/v1/chat/completions".to_string(),
                body: BatchJobBody {
                    model: self.model.clone(),
                    messages: vec![
                        ChatMessage {
                            role: "system".to_string(),
                            content: self.system_prompt.clone(),
                        },
                        ChatMessage {
                            role: "user".to_string(),
                            content: (*prompt).to_string(),
                        },
                    ],
                    max_tokens: Some(1000),
                    temperature: Some(0.3),
                },
            };
            requests.push(request);
        }

        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        for request in requests {
            let json_line = serde_json::to_string(&request)?;
            writeln!(writer, "{json_line}")?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::NamedTempFile;

    /// Helper function to create a test generator and temp file
    fn setup_test() -> (BatchJobGenerator, NamedTempFile) {
        let generator = BatchJobGenerator::new(None);
        let temp_file = NamedTempFile::new().unwrap();
        (generator, temp_file)
    }

    /// Helper function to generate and read test suite
    fn generate_and_read(suite_type: &str) -> String {
        let (generator, temp_file) = setup_test();
        let path = temp_file.path();
        generator.generate_test_suite(path, suite_type).unwrap();
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_generator_creation_default_model() {
        let generator = BatchJobGenerator::new(None);
        assert_eq!(generator.model, "gpt-4");
        assert!(!generator.system_prompt.is_empty());
    }

    #[test]
    fn test_generator_creation_custom_model() {
        let custom_model = "gpt-3.5-turbo".to_string();
        let generator = BatchJobGenerator::new(Some(custom_model.clone()));
        assert_eq!(generator.model, custom_model);
    }

    #[test]
    fn test_generate_basic_test_suite() {
        let generator = BatchJobGenerator::new(None);
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = generator.generate_test_suite(path, "basic");
        assert!(result.is_ok());

        // Read the generated file
        let content = fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // Should have 3 basic test requests
        assert_eq!(lines.len(), 3);

        // Each line should be valid JSON
        for line in lines {
            let request: BatchJobRequest = serde_json::from_str(line).unwrap();
            assert_eq!(request.method, "POST");
            assert_eq!(request.url, "/v1/chat/completions");
            assert_eq!(request.body.model, "gpt-4");
            assert!(request.custom_id.starts_with("basic_"));
        }
    }

    #[test]
    fn test_generate_malware_test_suite() {
        let generator = BatchJobGenerator::new(Some("gpt-3.5-turbo".to_string()));
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = generator.generate_test_suite(path, "malware");
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // Should have 3 malware test requests
        assert_eq!(lines.len(), 3);

        for line in lines {
            let request: BatchJobRequest = serde_json::from_str(line).unwrap();
            assert_eq!(request.body.model, "gpt-3.5-turbo");
            assert!(request.custom_id.starts_with("malware_"));
        }
    }

    #[test]
    fn test_generate_comprehensive_test_suite() {
        let generator = BatchJobGenerator::new(None);
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = generator.generate_test_suite(path, "comprehensive");
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // Should have 10 comprehensive test requests
        assert_eq!(lines.len(), 10);

        for (i, line) in lines.iter().enumerate() {
            let request: BatchJobRequest = serde_json::from_str(line).unwrap();
            assert_eq!(request.custom_id, format!("comprehensive_{:03}", i + 1));
        }
    }

    #[test]
    fn test_invalid_test_suite() {
        let generator = BatchJobGenerator::new(None);
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = generator.generate_test_suite(path, "invalid_suite");
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_job_request_structure() {
        let content = generate_and_read("basic");
        let first_line = content.lines().next().unwrap();
        let request: BatchJobRequest = serde_json::from_str(first_line).unwrap();

        // Verify structure
        assert_eq!(request.method, "POST");
        assert_eq!(request.url, "/v1/chat/completions");
        assert_eq!(request.body.messages.len(), 2);

        // Check system message
        assert_eq!(request.body.messages[0].role, "system");
        assert!(!request.body.messages[0].content.is_empty());

        // Check user message
        assert_eq!(request.body.messages[1].role, "user");
        assert!(!request.body.messages[1].content.is_empty());

        // Check optional parameters
        assert!(request.body.max_tokens.is_some());
        assert!(request.body.temperature.is_some());
        assert_eq!(request.body.max_tokens.unwrap(), 1000);
        assert_eq!(request.body.temperature.unwrap(), 0.3);
    }

    #[test]
    fn test_custom_id_formatting() {
        let content = generate_and_read("basic");
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let request: BatchJobRequest = serde_json::from_str(line).unwrap();
            let expected_id = format!("basic_{:03}", i + 1);
            assert_eq!(request.custom_id, expected_id);
        }
    }

    #[test]
    fn test_serialization_format() {
        let content = generate_and_read("basic");

        // Each line should be valid JSON
        for line in content.lines() {
            assert!(!line.is_empty());
            let _: serde_json::Value = serde_json::from_str(line).unwrap();
        }

        // File should end with newline
        assert!(content.ends_with('\n'));
    }

    #[test]
    fn test_system_prompt_content() {
        let generator = BatchJobGenerator::new(None);
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        generator.generate_test_suite(path, "basic").unwrap();

        let content = fs::read_to_string(path).unwrap();
        let first_line = content.lines().next().unwrap();
        let request: BatchJobRequest = serde_json::from_str(first_line).unwrap();

        let system_message = &request.body.messages[0];
        assert_eq!(system_message.role, "system");
        assert!(system_message.content.contains("YARA"));
        assert!(system_message.content.contains("expert"));
    }

    #[test]
    fn test_batch_job_structures_serialization() {
        // Test individual structure serialization
        let message = ChatMessage {
            role: "user".to_string(),
            content: "test content".to_string(),
        };
        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("test content"));

        let body = BatchJobBody {
            model: "gpt-4".to_string(),
            messages: vec![message],
            max_tokens: Some(500),
            temperature: Some(0.5),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("gpt-4"));
        assert!(json.contains("500"));

        let request = BatchJobRequest {
            custom_id: "test_001".to_string(),
            method: "POST".to_string(),
            url: "/v1/chat/completions".to_string(),
            body,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test_001"));
        assert!(json.contains("POST"));
    }

    #[test]
    fn test_different_suite_prompts() {
        let generator = BatchJobGenerator::new(None);

        // Test that different suites have different prompts
        let basic_file = NamedTempFile::new().unwrap();
        let malware_file = NamedTempFile::new().unwrap();

        generator
            .generate_test_suite(basic_file.path(), "basic")
            .unwrap();
        generator
            .generate_test_suite(malware_file.path(), "malware")
            .unwrap();

        let basic_content = fs::read_to_string(basic_file.path()).unwrap();
        let malware_content = fs::read_to_string(malware_file.path()).unwrap();

        // Should have different content
        assert_ne!(basic_content, malware_content);

        // Parse first request from each
        let basic_request: BatchJobRequest =
            serde_json::from_str(basic_content.lines().next().unwrap()).unwrap();
        let malware_request: BatchJobRequest =
            serde_json::from_str(malware_content.lines().next().unwrap()).unwrap();

        // User messages should be different
        assert_ne!(
            basic_request.body.messages[1].content,
            malware_request.body.messages[1].content
        );
    }

    #[test]
    fn test_file_creation_error_handling() {
        let generator = BatchJobGenerator::new(None);

        // Try to write to an invalid path (should fail)
        let invalid_path = std::path::Path::new("/invalid/path/that/does/not/exist.jsonl");
        let result = generator.generate_test_suite(invalid_path, "basic");
        assert!(result.is_err());
    }
}
