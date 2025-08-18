# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is the OpenAI Rust SDK (`openai_rust_sdk`) - a comprehensive Rust library providing complete OpenAI API integration with specialized YARA rule validation capabilities. It's designed for cost-effective, large-scale testing using OpenAI's Batch API. 

Developed by Wyatt Roersma and Claude Code.
Repository: https://github.com/threatflux/openai_rust_sdk

## Essential Commands

### Build & Development
```bash
# Quick development cycle
make dev                    # Format + lint + test
make all                    # Full CI-like checks

# Standard Rust commands
cargo build --release --all-features
cargo run --example chat_completion
```

### Testing
```bash
# Run all tests (528+ tests)
cargo test --verbose

# Test with OpenAI API (requires OPENAI_API_KEY env var)
make test-openai

# Coverage reports
make coverage              # Generate HTML + LCOV reports
make coverage-open        # Open HTML report in browser
```

### Code Quality
```bash
# Before committing
cargo fmt                  # Format code
cargo clippy --all-features -- -W warnings  # Lint
cargo audit               # Security audit
```

### Running Examples
```bash
# Set API key first
export OPENAI_API_KEY=your_api_key_here

# Run specific examples
cargo run --example chat_completion
cargo run --example function_calling
cargo run --example streaming_chat
```

## High-Level Architecture

### Core Structure
The SDK follows a modular design with clear separation:
- `src/api/` - API client implementations (19 modules)
- `src/models/` - Request/response models (18 modules)  
- `src/builders/` - Fluent builder patterns for complex requests
- `src/testing/` - YARA validation framework
- `src/client.rs` - Main OpenAI client orchestrating all APIs

### Key Architectural Patterns

1. **HttpClient Pattern**: Shared HTTP client across 13/19 APIs reduces code duplication by 77%. All API modules follow consistent request/response patterns.

2. **Builder Pattern**: Complex requests use builders for type-safe construction:
   - Function builders for tool definitions
   - Schema builders for JSON validation
   - Request builders for API calls

3. **Async Everything**: Full async/await using Tokio runtime. All API calls are async, with streaming support via Server-Sent Events.

4. **Type Safety**: Extensive use of Rust's type system with generic response parsing, compile-time validation, and safe error handling.

### API Module Organization
Each API module typically contains:
- Request/response models in `src/models/`
- API client implementation in `src/api/`
- Builder patterns where applicable
- Comprehensive examples in `examples/`
- Integration tests in `tests/`

### GPT-5 Support
Full GPT-5 family support with advanced features:
- Reasoning effort control via `reasoning_effort` parameter
- Verbosity settings for output control
- Chain-of-thought reasoning support
- Model-specific optimizations

### YARA Integration
The SDK includes a unique YARA rule validation system:
- Real-time validation using yara-x engine
- Batch job generation for testing AI model outputs
- Performance metrics and feature detection
- Test suites in `test_data/yara_rules/`

## Important Configuration

### Environment Variables
```bash
OPENAI_API_KEY=your_api_key_here  # Required for all API calls
```

### Key Dependencies
- `tokio`: Async runtime (full features)
- `reqwest`: HTTP client with streaming support
- `serde/serde_json`: JSON serialization
- `yara-x`: YARA rule validation
- `eventsource-stream`: SSE streaming

## Development Guidelines

### When Adding New Features
1. Follow the existing HttpClient pattern for new API modules
2. Add models in `src/models/`, API client in `src/api/`
3. Include comprehensive examples in `examples/`
4. Write integration tests in `tests/`
5. Update FEATURES.md with implementation status

### Testing Requirements
- Unit tests for all new functionality
- Integration tests for API interactions (use mock responses when possible)
- Examples demonstrating real usage
- Current coverage: 65%, target: 80%

### Performance Considerations
- Use streaming for large responses (reduces memory by 90%)
- Implement connection pooling for multiple requests
- Add benchmarks for performance-critical paths in `benches/`
- Current benchmarks: <1ms JSON parsing, <100ms YARA validation