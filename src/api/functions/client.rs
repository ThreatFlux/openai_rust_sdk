use crate::api::custom_tools::CustomToolsApi;
use crate::error::{OpenAIError, Result};
use crate::models::functions::Tool;
use reqwest::Client;
use std::collections::HashMap;

/// Core function calling API for `OpenAI`
#[derive(Debug, Clone)]
pub struct FunctionsApi {
    /// HTTP client for API requests
    pub(crate) client: Client,
    /// API key for authentication
    pub(crate) api_key: String,
    /// Base URL for API requests
    pub(crate) base_url: String,
    /// Custom tools registry
    pub(crate) custom_tools: CustomToolsApi,
    /// Active conversation state
    pub(crate) conversation_state: ConversationState,
}

/// State management for function calling conversations
#[derive(Debug, Clone, Default)]
pub struct ConversationState {
    /// Pending function calls awaiting results
    pub(crate) pending_calls: HashMap<String, crate::models::functions::FunctionCall>,
    /// Completed function calls with results
    pub(crate) completed_calls: HashMap<String, crate::models::functions::FunctionCallOutput>,
    /// Function call history for context
    pub(crate) call_history: Vec<FunctionCallEvent>,
}

/// Events in the function calling lifecycle
#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum FunctionCallEvent {
    /// A function call was initiated
    CallInitiated {
        /// When the call was initiated
        timestamp: u64,
        /// The function call details
        call: crate::models::functions::FunctionCall,
    },
    /// A function call was completed
    CallCompleted {
        /// When the call was completed
        timestamp: u64,
        /// The call ID
        call_id: String,
        /// The function result
        output: crate::models::functions::FunctionCallOutput,
    },
    /// A function call failed
    CallFailed {
        /// When the call failed
        timestamp: u64,
        /// The call ID
        call_id: String,
        /// Error message
        error: String,
    },
}

impl FunctionsApi {
    /// Create a new functions API
    pub fn new(api_key: &str) -> Result<Self> {
        Self::with_base_url(api_key, "https://api.openai.com")
    }

    /// Create a new functions API with custom base URL
    pub fn with_base_url(api_key: &str, base_url: &str) -> Result<Self> {
        if api_key.trim().is_empty() {
            return Err(OpenAIError::authentication("API key cannot be empty"));
        }

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(crate::network_err!("Failed to create HTTP client: {}"))?;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            custom_tools: CustomToolsApi::new(),
            conversation_state: ConversationState::default(),
        })
    }

    /// Get API key
    #[must_use]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

/// Configuration for function calling
#[derive(Debug, Clone, Default)]
pub struct FunctionConfig {
    /// Tools available for calling
    pub tools: Vec<Tool>,
    /// Tool choice strategy
    pub tool_choice: Option<crate::models::functions::ToolChoice>,
    /// Whether to allow parallel function calls
    pub parallel_function_calls: Option<bool>,
    /// Whether to use strict mode for schema adherence
    pub strict_mode: Option<bool>,
    /// Maximum number of function calls per response
    pub max_function_calls: Option<u32>,
}

impl FunctionConfig {
    /// Create a new function config
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add tools to the configuration
    #[must_use]
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = tools;
        self
    }

    /// Set tool choice strategy
    #[must_use]
    pub fn with_tool_choice(mut self, choice: crate::models::functions::ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Enable/disable parallel function calls
    #[must_use]
    pub const fn with_parallel_calls(mut self, enabled: bool) -> Self {
        self.parallel_function_calls = Some(enabled);
        self
    }

    /// Enable/disable strict mode
    #[must_use]
    pub const fn with_strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = Some(enabled);
        self
    }

    /// Set maximum function calls per response
    #[must_use]
    pub const fn with_max_calls(mut self, max: u32) -> Self {
        self.max_function_calls = Some(max);
        self
    }
}
