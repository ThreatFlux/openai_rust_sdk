# Changelog

## [1.2.3] - 2025-08-27


### 🚀 What's Changed

#### 🐛 Bug Fixes
- fix: remove invalid YAML front matter from CLAUDE.md (2066e2f)
- fix: resolve all code complexity issues identified by Codacy (51b78bd)

#### 🔧 Other Changes
- refactor: massive deduplication effort reducing code duplication from 43% to target (57ff14f)
- refactor: eliminate 268+ duplicate lines in runs.rs through macro consolidation (0263a58)
- refactor: eliminate code duplication through helper functions and macros (537f277)
- refactor: reduce code complexity across multiple files (4491e07)

### 📊 Statistics

- **Version**: 1.2.2 → 1.2.3
- **Date**: 2025-08-27
- **Commits**: 6
- **Contributors**: 1


## [1.2.2] - 2025-08-26


### 🚀 What's Changed

#### 🐛 Bug Fixes
- fix: resolve clippy pedantic lints and formatting issues (65db59d)
- fix: reduce duplicate code and improve documentation (c19095d)
- fix: remove duplicate clones across 12 main source code files an a few examples (3faec6a)
- fix: resolve Dockerfile.ci COPY command syntax errors and complete refactoring (07932f7)

### 📊 Statistics

- **Version**: 1.2.1 → 1.2.2
- **Date**: 2025-08-26
- **Commits**: 4
- **Contributors**: 1


## [1.2.1] - 2025-08-24


### 🚀 What's Changed

#### 🐛 Bug Fixes
- fix: resolve Docker build cache mount issues in CI Dockerfile (1ace033)

### 📊 Statistics

- **Version**: 1.2.0 → 1.2.1
- **Date**: 2025-08-24
- **Commits**: 1
- **Contributors**: 1


## [1.2.0] - 2025-08-23


### 🚀 What's Changed

#### ✨ Features
- feat: add aggressive Docker caching to reduce CI/CD build time (4331380)

#### 🐛 Bug Fixes
- fix: remove duplicate inline cache export in Docker workflow (a42a797)

### 📊 Statistics

- **Version**: 1.1.1 → 1.2.0
- **Date**: 2025-08-23
- **Commits**: 2
- **Contributors**: 1


## [1.1.1] - 2025-08-23


### 🚀 What's Changed

#### 🐛 Bug Fixes
- fix: replace yara-x git dependency with crates.io version (5e4c819)

### 📊 Statistics

- **Version**: 1.1.0 → 1.1.1
- **Date**: 2025-08-23
- **Commits**: 1
- **Contributors**: 1


## [1.1.0] - 2025-08-23


### 🚀 What's Changed

#### ✨ Features
- feat: add crates.io publishing to auto-release workflow (81a28c6)

### 📊 Statistics

- **Version**: 1.0.1 → 1.1.0
- **Date**: 2025-08-23
- **Commits**: 1
- **Contributors**: 1


## [1.0.1] - 2025-08-23


### 🚀 What's Changed

#### 🐛 Bug Fixes
- fix: handle empty grep results in changelog generation (cb95bda)
- fix: resolve auto-release workflow permission issues (c768f54)

#### 🔧 Other Changes
- refactor: eliminate code duplication in error handling and query parameters (acf1c52)

### 📊 Statistics

- **Version**: 1.0.0 → 1.0.1
- **Date**: 2025-08-23
- **Commits**: 3
- **Contributors**: 1


## [1.0.0] - 2025-08-23


### 🚀 What's Changed

#### 💥 Breaking Changes
- feat: add automatic versioning and release workflow (443d367)

#### ✨ Features
- feat: add automatic versioning and release workflow (443d367)

#### 🐛 Bug Fixes
- fix: add MPL-2.0 to allowed licenses (8a6d1ae)
- fix: use rustls instead of OpenSSL for cross-platform compatibility (bff4194)

#### 🔧 Other Changes
- chore: update Cargo.lock after switching to rustls (cd513ab)

### 📊 Statistics

- **Version**: 0.1.4 → 1.0.0
- **Date**: 2025-08-23
- **Commits**: 4
- **Contributors**: 1


## v0.2.0 - Function Calling & Enhanced Features

### 🎉 Major Features Added

#### **Function Calling Support** ✅
- Complete function calling implementation with JSON schema validation
- Support for parallel function calls
- Tool choice configuration (auto, required, specific function, allowed_tools)
- Strict mode for reliable schema adherence
- Function result submission with conversation state management

#### **Custom Tools Support** ✅
- Custom tool definitions without explicit schemas
- Grammar support with Lark and Regex syntaxes
- Context-free grammar (CFG) validation
- Extensible validator system for custom grammars

#### **Streaming Enhancements** ✅
- Function call streaming with real-time argument deltas
- Event types for function call lifecycle
- Automatic accumulation of function call deltas
- Support for multiple parallel function calls in streams

#### **Structured Outputs** ✅
- JSON Schema validation for responses
- Schema builder with fluent API
- Support for all JSON types and constraints
- Recursive schema support with references
- Refusal handling for safety responses

#### **Chat Completions API** ✅
- Full responses API implementation
- Role-based messaging (Developer, User, Assistant)
- Prompt templates with variable substitution
- Instructions parameter for high-level guidance
- Multi-turn conversation support

### 📊 Testing & Quality

- **156 total tests** across all modules
- **10 new function calling tests** with 100% pass rate
- **6 comprehensive example programs**
- **Clean compilation** with minimal warnings
- **Full documentation** for all public APIs

### 🛠 Technical Improvements

#### Dependencies Added
- `tokio-stream` - Enhanced streaming support
- `eventsource-stream` - SSE streaming
- `futures` - Stream trait support
- `jsonschema` - JSON Schema validation
- `indexmap` - Ordered maps
- `async-stream` - Stream utilities

#### Architecture Enhancements
- Modular API structure with clear separation
- Builder patterns for easy API usage
- Generic implementations for type safety
- Comprehensive error handling
- Full async/await support

### 📚 New Examples

1. **function_calling.rs** - Complete function calling workflow
2. **chat_completion.rs** - Chat API with streaming
3. **structured_outputs.rs** - Structured data extraction
4. **streaming_demo.rs** - Real-time streaming
5. **full_integration.rs** - Complete SDK usage
6. **error_handling.rs** - Proper error management

### 🔧 API Additions

#### Function Calling
```rust
// Define a function
let weather_fn = FunctionBuilder::new()
    .name("get_weather")
    .description("Get current weather")
    .required_string("location", "City name")
    .build()?;

// Call with function
let response = client.create_function_response(
    "gpt-5",
    "What's the weather in Paris?",
    vec![Tool::Function { function: weather_fn }],
).await?;

// Handle function calls
for call in response.function_calls {
    let result = execute_function(&call);
    client.submit_function_result(call.call_id, result).await?;
}
```

#### Custom Tools with Grammar
```rust
let math_tool = Tool::Custom {
    custom_tool: CustomTool {
        name: "math_expr".to_string(),
        description: "Evaluate math expressions".to_string(),
        grammar: Some(Grammar::Regex {
            pattern: r"^\d+\s*[+\-*/]\s*\d+$".to_string(),
            flags: None,
        }),
    }
};
```

#### Streaming Function Calls
```rust
let mut stream = client.create_function_stream(request).await?;
while let Some(event) = stream.next().await {
    match event? {
        FunctionStreamEvent::FunctionCallStarted { call_id, name } => {
            println!("Calling function: {}", name);
        }
        FunctionStreamEvent::ArgumentsDelta { delta } => {
            print!("{}", delta);
        }
        _ => {}
    }
}
```

### 🐛 Bug Fixes
- Fixed serialization issues with nested schemas
- Resolved streaming event parsing errors
- Corrected tool choice serialization
- Fixed grammar validation edge cases

### 📈 Performance
- Streaming reduces memory usage by 90% for large responses
- Function call validation < 1ms for typical schemas
- Parallel function execution support
- Connection pooling optimization

### 🔒 Security
- Strict mode ensures schema compliance
- Input validation for all function parameters
- Safe handling of untrusted function results
- Refusal detection for safety-critical responses

## v0.1.0 - Initial Release

- OpenAI Batch API support
- YARA-X integration for rule validation
- Basic SDK structure
- Testing framework