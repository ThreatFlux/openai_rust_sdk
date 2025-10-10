# OpenAI Rust SDK

A comprehensive Rust SDK for the OpenAI API with integrated YARA-X rule validation testing. This library provides complete access to all OpenAI APIs including Chat, Assistants, Batch processing, and more, with special capabilities for testing AI models' ability to generate valid YARA rules.

Developed by Wyatt Roersma and Claude Code.

## Features

✅ **Complete OpenAI API Support**
- Create, retrieve, cancel, and list batch jobs
- JSONL format support for batch requests
- Automatic retry and error handling
- Type-safe API interactions

✅ **YARA-X Integration**
- Real-time YARA rule validation using yara-x
- Feature detection (hex patterns, strings, regex, metadata)
- Performance metrics and complexity scoring
- Pattern testing against sample data

✅ **Testing Framework**
- Generate batch jobs with YARA-specific questions
- Validate AI-generated rules for correctness
- Built-in test suite with various rule types
- Comprehensive error reporting

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
openai_rust_sdk = "0.1.0"
```

## Quick Start

### Environment Setup

```bash
export OPENAI_API_KEY=your_api_key_here

# Optional: target a non-default OpenAI-compatible endpoint (e.g., proxy or hosted variant)
export OPENAI_BASE_URL=https://my-openai-proxy.example.com/v1
```

With both variables set, `OpenAIClient::from_env()` will authenticate with your key and route
requests through the alternate base URL automatically.

### Generate a Batch Job

```bash
cargo run -- generate-batch basic output.jsonl
```

### Validate a YARA Rule

```bash
cargo run -- validate-rule rule.yar
```

### Run Test Suite

```bash
cargo run -- run-tests
```

## Usage Examples

### Full Integration Example

```rust
use openai_rust_sdk::testing::{
    batch_generator::BatchJobGenerator,
    yara_validator::YaraValidator,
};

fn main() {
    // Generate batch job
    let generator = BatchJobGenerator::new(Some("gpt-5-nano".to_string()));
    let batch_file = std::path::Path::new("test_batch.jsonl");
    generator.generate_test_suite(batch_file, "basic").unwrap();
    
    // Validate a YARA rule
    let rule = r#"
    rule DetectMalware {
        strings:
            $a = "malware"
        condition:
            $a
    }
    "#;
    
    let validator = YaraValidator::new();
    let result = validator.validate_rule(rule).unwrap();
    
    if result.is_valid {
        println!("✓ Rule is valid!");
    }
}
```

### Batch Processing Workflow

1. **Prepare Questions**: Generate JSONL file with YARA-related questions
2. **Upload File**: Use OpenAI Files API to upload the JSONL
3. **Create Batch**: Submit batch job with file ID
4. **Monitor Progress**: Poll for completion (up to 24 hours)
5. **Download Results**: Retrieve generated YARA rules
6. **Validate Rules**: Use yara-x validator to test correctness

### Modern Responses API

The SDK now ships with a native client for the `/v1/responses` endpoints. You can access
the full feature set--including conversations, background execution, and structured
outputs--through the new builder and client helpers:

```rust
use futures::StreamExt;
use openai_rust_sdk::{
    CreateResponseRequest, OpenAIClient,
    ResponsesApiServiceTier as ServiceTier,
};

# tokio_test::block_on(async {
let client = OpenAIClient::new(std::env::var("OPENAI_API_KEY")?)?;

let request = CreateResponseRequest::new_text("gpt-4o-mini", "Summarize Rust ownership")
    .with_service_tier(ServiceTier::Auto)
    .with_store(true);

let response = client.create_response_v2(&request).await?;
println!("Summary: {}", response.output_text());

// Stream events with strong typing
let mut stream = client.stream_response_v2(&request).await?;
while let Some(event) = stream.next().await {
    match event? {
        openai_rust_sdk::ResponseStreamEvent::OutputTextDelta { delta, .. } => {
            print!("{}", delta);
        }
        openai_rust_sdk::ResponseStreamEvent::ResponseCompleted { .. } => println!("\nDone!"),
        _ => {}
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
# })?;
```

Compatibility helpers such as `generate_text`, `create_chat_completion`, and
`create_custom_response` automatically route through the Responses API to maintain
the existing interface while unlocking new functionality.

### Using a Custom Base URL

```rust
# tokio_test::block_on(async {
use openai_rust_sdk::{from_env, CreateResponseRequest};

// Set OPENAI_API_KEY and optionally OPENAI_BASE_URL before running.
let client = from_env()?;

let response = client
    .create_response_v2(&CreateResponseRequest::new_text(
        "gpt-4o-mini",
        "Send this request through my proxy",
    ))
    .await?;

println!("{}", response.output_text());
# Ok::<(), Box<dyn std::error::Error>>(())
# })?;
```

When `OPENAI_BASE_URL` is supplied, the client automatically routes requests through that
endpoint instead of the default `https://api.openai.com`.

## Test Suites

The SDK includes three test suites for different complexity levels:

### Basic Suite
- Simple string detection rules
- Basic PE file detection
- Error/warning pattern matching

### Malware Suite
- Ransomware detection patterns
- Trojan indicators
- Cryptominer signatures
- Advanced malware techniques

### Comprehensive Suite
- Complex multi-condition rules
- External variable usage
- Iterator patterns
- Module imports
- Performance-optimized rules

## Project Structure

```
openai_rust_sdk/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # CLI application
│   └── testing/
│       ├── mod.rs          # Testing module exports
│       ├── yara_validator.rs    # YARA-X validation
│       ├── test_cases.rs       # Built-in test cases
│       └── batch_generator.rs  # Batch job generation
├── examples/
│   └── full_integration.rs # Complete usage example
├── test_data/
│   ├── yara_x_questions.jsonl # Sample questions
│   └── simple_batch.jsonl     # Basic test batch
└── tests/
    └── integration_test.rs # Integration tests
```

## CLI Commands

```bash
# Validate a single YARA rule
cargo run -- validate-rule path/to/rule.yar

# Run the built-in test suite
cargo run -- run-tests

# Generate batch job for basic testing
cargo run -- generate-batch basic output.jsonl

# Generate batch job for malware detection
cargo run -- generate-batch malware output.jsonl

# Generate comprehensive test batch
cargo run -- generate-batch comprehensive output.jsonl
```

## Testing with GPT-5-Nano

The SDK is configured to use `gpt-5-nano` for testing, which provides fast and cost-effective rule generation. Example batch request:

```json
{
  "custom_id": "yara_001",
  "method": "POST",
  "url": "/v1/chat/completions",
  "body": {
    "model": "gpt-5-nano",
    "messages": [
      {
        "role": "system",
        "content": "You are an expert YARA rule developer."
      },
      {
        "role": "user",
        "content": "Create a YARA rule to detect UPX-packed PE files."
      }
    ],
    "max_tokens": 1000,
    "temperature": 0.3
  }
}
```

## Validation Metrics

The validator provides comprehensive metrics:

- **Compilation Status**: Whether the rule compiles successfully
- **Feature Detection**: Identifies rule components (strings, hex, regex, etc.)
- **Performance Metrics**: Compilation time and complexity scoring
- **Pattern Testing**: Tests rules against sample data
- **Error Reporting**: Detailed error messages for invalid rules

## Development

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Formatting & Linting

```bash
cargo fmt
cargo clippy -- -D warnings
```

## License

MIT

## Contributing

Contributions are welcome! Please ensure all tests pass and code is properly formatted before submitting PRs.

## Requirements

- Rust 1.89.0 or later
- OpenAI API key for batch processing
- yara-x crate for rule validation
