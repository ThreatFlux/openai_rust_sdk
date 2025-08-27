#![allow(
    clippy::too_many_lines,
    clippy::uninlined_format_args,
    clippy::cast_precision_loss,
    clippy::ignored_unit_patterns
)]
#[cfg(not(feature = "yara"))]
#[tokio::main]
async fn main() {
    println!("This example requires the 'yara' feature to be enabled.");
    println!("Run with: cargo run --example sdk_demo --features yara");
}

#[cfg(feature = "yara")]
use openai_rust_sdk::{
    api::{
        functions::{FunctionConfig, FunctionsApi},
        gpt5::{GPT5Api, GPT5RequestBuilder},
        responses::ResponsesApi,
        streaming::StreamingApi,
    },
    models::{
        functions::{FunctionTool, Tool, ToolChoice},
        gpt5::models,
        responses::{Message, ResponseRequest},
    },
    schema::builder::SchemaBuilder,
    testing::yara_validator::{ValidationResult, YaraValidator},
    OpenAIClient,
};

#[cfg(feature = "yara")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_header();

    demo_client_creation()?;
    demo_request_building();
    demo_gpt5_builder()?;
    demo_function_building();
    demo_yara_integration()?;
    demo_model_constants();
    demo_schema_building();

    print_completion_summary();
    Ok(())
}

#[cfg(feature = "yara")]
fn print_header() {
    println!("🚀 OpenAI SDK Demo - Structure and Configuration");
    println!("═══════════════════════════════════════════════");
}

#[cfg(feature = "yara")]
fn demo_client_creation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1️⃣ SDK Components Demonstration");
    println!("──────────────────────────────");

    let dummy_key = "dummy-key-for-demo";

    println!("✅ Creating OpenAI Client...");
    let _client = OpenAIClient::new(dummy_key)?;
    println!("   Client created successfully");

    println!("✅ Creating Responses API...");
    let _responses_api =
        ResponsesApi::with_base_url(dummy_key.to_string(), "https://api.openai.com".to_string())?;
    println!("   Responses API created successfully");

    println!("✅ Creating GPT-5 API...");
    let _gpt5_api = GPT5Api::new(dummy_key.to_string())?;
    println!("   GPT-5 API created successfully");

    println!("✅ Creating Functions API...");
    let _functions_api = FunctionsApi::new(dummy_key)?;
    println!("   Functions API created successfully");

    println!("✅ Creating Streaming API...");
    let _streaming_api = StreamingApi::new(dummy_key.to_string())?;
    println!("   Streaming API created successfully");

    Ok(())
}

#[cfg(feature = "yara")]
fn demo_request_building() {
    println!("\n2️⃣ Request Building Demonstration");
    println!("─────────────────────────────────");

    let basic_request = ResponseRequest::new_text("gpt-4o-mini", "Hello, world!")
        .with_temperature(0.7)
        .with_max_tokens(100);

    println!("✅ Basic Request:");
    println!("   Model: {}", basic_request.model);
    println!("   Temperature: {:?}", basic_request.temperature);
    println!("   Max tokens: {:?}", basic_request.max_tokens);

    let messages = vec![
        Message::user("You are a helpful assistant."),
        Message::assistant("I'm here to help!"),
        Message::user("What can you do?"),
    ];

    let conversation_request =
        ResponseRequest::new_messages("gpt-4o-mini", messages).with_temperature(0.5);

    println!("✅ Conversation Request:");
    println!("   Model: {}", conversation_request.model);
    if let openai_rust_sdk::models::responses::ResponseInput::Messages(msgs) =
        &conversation_request.input
    {
        println!("   Messages: {} total", msgs.len());
        for (i, msg) in msgs.iter().enumerate() {
            println!("     {}: {:?} - {:?}", i + 1, msg.role, msg.content);
        }
    }
}

#[cfg(feature = "yara")]
fn demo_gpt5_builder() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3️⃣ GPT-5 Request Builder Demonstration");
    println!("──────────────────────────────────────");

    let gpt5_request = GPT5RequestBuilder::new()
        .gpt5_mini()
        .input("Explain quantum computing briefly")
        .high_reasoning()
        .medium_verbosity()
        .temperature(0.8)
        .max_tokens(200)
        .build()?;

    println!("✅ GPT-5 Request:");
    println!("   Model: {}", gpt5_request.model);
    println!("   Reasoning: {:?}", gpt5_request.reasoning);
    println!("   Text config: {:?}", gpt5_request.text);
    println!("   Temperature: {:?}", gpt5_request.temperature);

    Ok(())
}

#[cfg(feature = "yara")]
fn demo_function_building() {
    println!("\n4️⃣ Function Building Demonstration");
    println!("──────────────────────────────");

    let parameters = SchemaBuilder::new_object()
        .required_property(
            "operation",
            SchemaBuilder::string().pattern(r"^(add|subtract|multiply|divide)$"),
        )
        .required_property("a", SchemaBuilder::number())
        .required_property("b", SchemaBuilder::number())
        .build();

    let calculator_function = Tool::Function {
        function: FunctionTool {
            name: "calculate".to_string(),
            description: "Perform mathematical calculations".to_string(),
            parameters: parameters.unwrap(),
            strict: None,
        },
    };

    println!("✅ Calculator Function created successfully");

    let function_config = FunctionConfig::new()
        .with_tools(vec![calculator_function])
        .with_tool_choice(ToolChoice::Auto)
        .with_parallel_calls(true);

    println!("✅ Function Configuration:");
    println!("   Tools count: {}", function_config.tools.len());
    println!("   Tool choice: {:?}", function_config.tool_choice);
    println!(
        "   Parallel calls: {:?}",
        function_config.parallel_function_calls
    );
}

#[cfg(feature = "yara")]
fn demo_yara_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5️⃣ YARA Integration Demonstration");
    println!("─────────────────────────────────");

    let validator = YaraValidator::new();
    let test_rule = create_test_yara_rule();
    let validation_result = validator.validate_rule(test_rule)?;

    print_yara_results(&validation_result);

    Ok(())
}

#[cfg(feature = "yara")]
const fn create_test_yara_rule() -> &'static str {
    r#"
rule demo_rule {
    meta:
        description = "Demo rule for SDK testing"
        author = "SDK Demo"
        date = "2024-01-01"
    strings:
        $text = "malware"
        $hex = { 4D 5A }
        $regex = /user[0-9]+@example\.com/
    condition:
        $text or $hex or $regex
}
"#
}

#[cfg(feature = "yara")]
fn print_yara_results(validation_result: &ValidationResult) {
    println!("✅ YARA Validation:");
    println!("   Valid: {}", validation_result.is_valid);
    println!(
        "   Rule name: {}",
        validation_result.rule_name.as_deref().unwrap_or("Unknown")
    );
    println!(
        "   Compilation time: {}ms",
        validation_result.metrics.compilation_time_ms
    );
    println!(
        "   Rule size: {} bytes",
        validation_result.metrics.rule_size_bytes
    );
    println!(
        "   Pattern count: {}",
        validation_result.metrics.pattern_count
    );

    println!("✅ Features detected:");
    println!("   Has strings: {}", validation_result.features.has_strings);
    println!(
        "   Has hex patterns: {}",
        validation_result.features.has_hex_patterns
    );
    println!(
        "   Has regex patterns: {}",
        validation_result.features.has_regex_patterns
    );
    println!(
        "   String count: {}",
        validation_result.features.string_count
    );
}

#[cfg(feature = "yara")]
fn demo_model_constants() {
    println!("\n6️⃣ Available Models");
    println!("──────────────────");

    println!("✅ GPT-5 Family:");
    println!("   GPT-5: {}", models::GPT_5);
    println!("   GPT-5 Mini: {}", models::GPT_5_MINI);
    println!("   GPT-5 Nano: {}", models::GPT_5_NANO);
    println!("   GPT-5 Chat Latest: {}", models::GPT_5_CHAT_LATEST);

    println!("✅ Legacy Models:");
    println!("   GPT-4 Turbo: {}", models::GPT_4_TURBO);
    println!("   O4 Mini: {}", models::O4_MINI);
}

#[cfg(feature = "yara")]
fn demo_schema_building() {
    println!("\n7️⃣ JSON Schema Building");
    println!("──────────────────────");

    let user_schema = SchemaBuilder::new_object()
        .required_property(
            "name",
            SchemaBuilder::string().min_length(1).max_length(100),
        )
        .required_property("age", SchemaBuilder::integer().minimum(0.0).maximum(150.0))
        .optional_property("email", SchemaBuilder::string().format("email"))
        .build();

    println!("✅ User Schema created successfully");
    if let Ok(schema_value) = user_schema {
        if let Some(schema_type) = schema_value.get("type") {
            println!("   Schema type: {}", schema_type);
        }
        if let Some(properties) = schema_value.get("properties") {
            if let Some(obj) = properties.as_object() {
                let keys: Vec<String> = obj.keys().cloned().collect();
                println!("   Properties: {}", keys.join(", "));
            }
        }
    }
}

#[cfg(feature = "yara")]
fn print_completion_summary() {
    println!("\n🎉 SDK Demo Completed Successfully!");
    println!("═══════════════════════════════");
    println!("✅ All components initialized correctly");
    println!("✅ Request building works properly");
    println!("✅ Function definitions created successfully");
    println!("✅ YARA validation operational");
    println!("✅ Schema building functional");
    println!("\n📋 Next Steps:");
    println!("   1. Set OPENAI_API_KEY environment variable");
    println!("   2. Run: cargo run --example api_integration_test");
    println!("   3. Enjoy using the OpenAI SDK!");
}
