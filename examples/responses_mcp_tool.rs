//! Example: Responses API with MCP tool integration.
//!
//! This sample demonstrates how to call the Responses API with an enhanced MCP tool.
//! It streams events so you can observe when the model triggers the MCP server and
//! then prints the final assistant reply.
//!
//! Required environment variables:
//! - `OPENAI_API_KEY`
//!
//! Optional environment variables:
//! - `MCP_SERVER_URL` (defaults to `http://localhost:8989`) pointing at a running MCP server
//! - `MCP_SERVER_LABEL` (defaults to `docs_mcp`)
//! - `MCP_BEARER_TOKEN` for authenticated MCP servers.

use std::env;
use std::io::{self, Write};

use anyhow::{bail, Context};
use futures::StreamExt;
use openai_rust_sdk::{
    client::OpenAIClient,
    models::{
        responses::message_types::{Message, MessageContentInput, MessageRole},
        responses_v2::{CreateResponseRequest, ResponseStreamEvent},
        tools::{EnhancedToolChoice, McpApproval, ToolBuilder},
    },
};
use serde_json::to_string_pretty;

const DEFAULT_MCP_URL: &str = "http://localhost:8989";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key =
        env::var("OPENAI_API_KEY").context("Set the OPENAI_API_KEY environment variable")?;
    let (mcp_url, used_default_server) = match env::var("MCP_SERVER_URL") {
        Ok(url) if !url.trim().is_empty() => (url, false),
        Ok(_) | Err(env::VarError::NotPresent) => (DEFAULT_MCP_URL.to_string(), true),
        Err(env::VarError::NotUnicode(_)) => {
            bail!("MCP_SERVER_URL contains non-Unicode data; please set a valid URL")
        }
    };
    let mcp_label = env::var("MCP_SERVER_LABEL").unwrap_or_else(|_| "docs_mcp".to_string());
    let mcp_token = env::var("MCP_BEARER_TOKEN").ok();

    let mut plan_steps = vec![if used_default_server {
        format!("Use default MCP server at {DEFAULT_MCP_URL} because MCP_SERVER_URL is not set.")
    } else {
        format!("Use MCP server configured via MCP_SERVER_URL: {mcp_url}")
    }];
    plan_steps.push(format!(
        "Confirm the MCP server is running and reachable at {mcp_url}."
    ));
    plan_steps.push(
        "Run `cargo run --example responses_mcp_tool` to stream tool call events.".to_string(),
    );
    plan_steps.push(
        "Watch for `tool_call` items to verify the MCP server handled the documentation lookup."
            .to_string(),
    );
    plan_steps.push("Review the assistant summary for the final response.".to_string());

    println!("Test plan for MCP server integration:");
    for (idx, step) in plan_steps.iter().enumerate() {
        println!("  {}. {}", idx + 1, step);
    }
    println!();

    let mut builder = ToolBuilder::mcp(&mcp_label, &mcp_url)
        .require_approval(McpApproval::Sensitive)
        .timeout_ms(30_000);

    if let Some(token) = mcp_token {
        builder = builder.header("Authorization", format!("Bearer {token}"));
    }

    let mcp_tool = builder.build();

    let developer = Message {
        role: MessageRole::Developer,
        content: MessageContentInput::Text(format!(
            "You can query the {mcp_label} MCP server for project documentation or task helpers. \
             Prefer MCP tool calls when the user asks for specific procedures or references."
        )),
    };

    let user = Message::user(
        "Using the docs MCP server, find the `make watch-run` workflow and summarize the steps.",
    );

    let request = CreateResponseRequest::new_messages("gpt-5-chat-latest", vec![developer, user])
        .with_enhanced_tools(vec![mcp_tool])
        .with_enhanced_tool_choice(EnhancedToolChoice::Auto)
        .with_parallel_tool_calls(false)
        .with_streaming(true);

    let client = OpenAIClient::new(api_key)?;
    let mut stream = client.stream_response_v2(&request).await?;

    println!("Streaming response with MCP tool events:\n");

    let mut assistant_text = String::new();

    while let Some(event) = stream.next().await {
        match event? {
            ResponseStreamEvent::OutputItemAdded { item, .. } => {
                println!("→ Output item: {}", item.item_type);
                if item.item_type == "tool_call" {
                    let payload = to_string_pretty(&item)?;
                    println!("  MCP call payload:\n{payload}");
                }
            }
            ResponseStreamEvent::OutputTextDelta { delta, .. } => {
                print!("{delta}");
                assistant_text.push_str(&delta);
                io::stdout().flush().ok();
            }
            ResponseStreamEvent::OutputTextDone { text, .. } => {
                assistant_text = text;
                println!("\n");
            }
            ResponseStreamEvent::ResponseCompleted { response, .. } => {
                println!("\nFinal status: {:?}", response.status);
                if let Some(usage) = response.usage {
                    println!(
                        "Token usage => input: {}, output: {}, total: {}",
                        usage.input_tokens, usage.output_tokens, usage.total_tokens
                    );
                }
                break;
            }
            ResponseStreamEvent::ResponseFailed { response, .. } => {
                eprintln!("\nResponse failed: {:?}", response.error);
                break;
            }
            ResponseStreamEvent::StreamError { error, .. } => {
                eprintln!("\nStream error: {:?}", error.message);
                break;
            }
            _ => {}
        }
    }

    if !assistant_text.is_empty() {
        println!("\nAssistant summary:\n{assistant_text}");
    }

    Ok(())
}
