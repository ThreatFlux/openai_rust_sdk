//! GPT-5 specific API features and functionality

#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    api::responses::ResponsesApi,
    error::{OpenAIError, Result},
    models::{
        functions::{Tool, ToolChoice},
        gpt5::{models, ReasoningConfig, ReasoningEffort, TextConfig, Verbosity},
        responses::{ResponseInput, ResponseRequest, ResponseResult},
    },
};

/// GPT-5 specific API client with advanced features
pub struct GPT5Api {
    /// Underlying responses API
    responses_api: ResponsesApi,
}

impl GPT5Api {
    /// Create a new GPT-5 API client
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            responses_api: ResponsesApi::new(api_key)?,
        })
    }

    /// Create a response with minimal reasoning for fastest time-to-first-token
    pub async fn create_minimal_response(
        &self,
        model: &str,
        input: impl Into<ResponseInput>,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: model.to_string(),
            input: input.into(),
            reasoning: Some(ReasoningConfig::minimal()),
            text: Some(TextConfig::low()),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a response with low latency settings
    pub async fn create_fast_response(
        &self,
        model: &str,
        input: impl Into<ResponseInput>,
        verbosity: Verbosity,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: model.to_string(),
            input: input.into(),
            reasoning: Some(ReasoningConfig::low()),
            text: Some(TextConfig::new(verbosity)),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a response with configurable reasoning effort
    pub async fn create_reasoned_response(
        &self,
        model: &str,
        input: impl Into<ResponseInput>,
        effort: ReasoningEffort,
        verbosity: Verbosity,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: model.to_string(),
            input: input.into(),
            reasoning: Some(ReasoningConfig::new(effort)),
            text: Some(TextConfig::new(verbosity)),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a response for complex reasoning tasks
    pub async fn create_complex_response(
        &self,
        input: impl Into<ResponseInput>,
        instructions: Option<String>,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: models::GPT_5.to_string(),
            input: input.into(),
            instructions,
            reasoning: Some(ReasoningConfig::high()),
            text: Some(TextConfig::high()),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a response for coding tasks with optimal settings
    pub async fn create_coding_response(
        &self,
        input: impl Into<ResponseInput>,
        verbosity: Verbosity,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: models::GPT_5.to_string(),
            input: input.into(),
            reasoning: Some(ReasoningConfig::medium()),
            text: Some(TextConfig::new(verbosity)),
            instructions: Some("You are an expert programmer. Generate clean, efficient, and well-documented code.".to_string()),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a response for frontend development
    pub async fn create_frontend_response(
        &self,
        input: impl Into<ResponseInput>,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: models::GPT_5.to_string(),
            input: input.into(),
            reasoning: Some(ReasoningConfig::medium()),
            text: Some(TextConfig::medium()),
            instructions: Some("You are an expert frontend developer. Generate modern, responsive, and accessible UI code.".to_string()),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Continue a conversation using previous response ID for `CoT`
    pub async fn continue_conversation(
        &self,
        model: &str,
        input: impl Into<ResponseInput>,
        previous_response_id: String,
        effort: ReasoningEffort,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: model.to_string(),
            input: input.into(),
            previous_response_id: Some(previous_response_id),
            reasoning: Some(ReasoningConfig::new(effort)),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a response with tools and minimal reasoning
    pub async fn create_tool_response(
        &self,
        model: &str,
        input: impl Into<ResponseInput>,
        tools: Vec<Tool>,
        tool_choice: Option<ToolChoice>,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: model.to_string(),
            input: input.into(),
            tools: Some(tools),
            tool_choice,
            reasoning: Some(ReasoningConfig::minimal()),
            text: Some(TextConfig::low()),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a cost-optimized response using GPT-5-mini
    pub async fn create_cost_optimized_response(
        &self,
        input: impl Into<ResponseInput>,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: models::GPT_5_MINI.to_string(),
            input: input.into(),
            reasoning: Some(ReasoningConfig::low()),
            text: Some(TextConfig::medium()),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    /// Create a high-throughput response using GPT-5-nano
    pub async fn create_high_throughput_response(
        &self,
        input: impl Into<ResponseInput>,
    ) -> Result<ResponseResult> {
        let request = ResponseRequest {
            model: models::GPT_5_NANO.to_string(),
            input: input.into(),
            reasoning: Some(ReasoningConfig::minimal()),
            text: Some(TextConfig::low()),
            ..Default::default()
        };

        self.responses_api.create_response(&request).await
    }

    // Note: Streaming support will be added in a future update
    // For now, use the standard create_* methods with async/await
}

/// Builder for GPT-5 requests with fluent API
#[allow(clippy::missing_docs_in_private_items)]
pub struct GPT5RequestBuilder {
    model: String,
    input: Option<ResponseInput>,
    instructions: Option<String>,
    previous_response_id: Option<String>,
    reasoning_effort: Option<ReasoningEffort>,
    verbosity: Option<Verbosity>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

impl Default for GPT5RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GPT5RequestBuilder {
    /// Create a new GPT-5 request builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            model: models::GPT_5.to_string(),
            input: None,
            instructions: None,
            previous_response_id: None,
            reasoning_effort: None,
            verbosity: None,
            tools: None,
            tool_choice: None,
            temperature: None,
            max_tokens: None,
        }
    }

    /// Set the model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Use GPT-5 (default)
    #[must_use]
    pub fn gpt5(mut self) -> Self {
        self.model = models::GPT_5.to_string();
        self
    }

    /// Use GPT-5-mini
    #[must_use]
    pub fn gpt5_mini(mut self) -> Self {
        self.model = models::GPT_5_MINI.to_string();
        self
    }

    /// Use GPT-5-nano
    #[must_use]
    pub fn gpt5_nano(mut self) -> Self {
        self.model = models::GPT_5_NANO.to_string();
        self
    }

    /// Set the input
    pub fn input(mut self, input: impl Into<ResponseInput>) -> Self {
        self.input = Some(input.into());
        self
    }

    /// Set instructions
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Set previous response ID for conversation continuation
    pub fn previous_response(mut self, id: impl Into<String>) -> Self {
        self.previous_response_id = Some(id.into());
        self
    }

    /// Set reasoning effort
    #[must_use]
    pub fn reasoning(mut self, effort: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(effort);
        self
    }

    /// Use minimal reasoning
    #[must_use]
    pub fn minimal_reasoning(mut self) -> Self {
        self.reasoning_effort = Some(ReasoningEffort::Minimal);
        self
    }

    /// Use low reasoning
    #[must_use]
    pub fn low_reasoning(mut self) -> Self {
        self.reasoning_effort = Some(ReasoningEffort::Low);
        self
    }

    /// Use medium reasoning (default)
    #[must_use]
    pub fn medium_reasoning(mut self) -> Self {
        self.reasoning_effort = Some(ReasoningEffort::Medium);
        self
    }

    /// Use high reasoning
    #[must_use]
    pub fn high_reasoning(mut self) -> Self {
        self.reasoning_effort = Some(ReasoningEffort::High);
        self
    }

    /// Set verbosity
    #[must_use]
    pub fn verbosity(mut self, verbosity: Verbosity) -> Self {
        self.verbosity = Some(verbosity);
        self
    }

    /// Use low verbosity
    #[must_use]
    pub fn low_verbosity(mut self) -> Self {
        self.verbosity = Some(Verbosity::Low);
        self
    }

    /// Use medium verbosity (default)
    #[must_use]
    pub fn medium_verbosity(mut self) -> Self {
        self.verbosity = Some(Verbosity::Medium);
        self
    }

    /// Use high verbosity
    #[must_use]
    pub fn high_verbosity(mut self) -> Self {
        self.verbosity = Some(Verbosity::High);
        self
    }

    /// Add tools
    #[must_use]
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set tool choice
    #[must_use]
    pub fn tool_choice(mut self, choice: ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Set temperature
    #[must_use]
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Set max tokens
    #[must_use]
    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    /// Build the request
    pub fn build(self) -> Result<ResponseRequest> {
        let input = self
            .input
            .ok_or_else(|| OpenAIError::InvalidRequest("Input is required".to_string()))?;

        Ok(ResponseRequest {
            model: self.model,
            input,
            instructions: self.instructions,
            previous_response_id: self.previous_response_id,
            reasoning: self.reasoning_effort.map(ReasoningConfig::new),
            text: self.verbosity.map(TextConfig::new),
            tools: self.tools,
            tool_choice: self.tool_choice,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            ..Default::default()
        })
    }
}
