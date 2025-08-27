//! # Real-time Audio Builders
//!
//! Builder patterns for creating real-time audio sessions.

use crate::models::realtime_audio::{
    RealtimeAudioModels, RealtimeSessionConfig, RealtimeSessionRequest, RealtimeVoice,
};

/// Builder for creating real-time audio sessions
pub struct RealtimeSessionBuilder {
    /// The realtime session request
    request: RealtimeSessionRequest,
}

impl RealtimeSessionBuilder {
    /// Create a new session builder
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            request: RealtimeSessionRequest {
                model: model.into(),
                config: None,
                instructions: None,
                voice: None,
                temperature: None,
                max_response_output_tokens: None,
            },
        }
    }

    /// Use GPT-4o real-time model
    #[must_use]
    pub fn gpt_4o_realtime() -> Self {
        Self::new(RealtimeAudioModels::GPT_4O_REALTIME_PREVIEW)
    }

    /// Use GPT-4o mini real-time model
    #[must_use]
    pub fn gpt_4o_mini_realtime() -> Self {
        Self::new(RealtimeAudioModels::GPT_4O_MINI_REALTIME_PREVIEW)
    }

    /// Set session configuration
    #[must_use]
    pub fn config(mut self, config: RealtimeSessionConfig) -> Self {
        self.request.config = Some(config);
        self
    }

    /// Set instructions
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request.instructions = Some(instructions.into());
        self
    }

    /// Set voice
    #[must_use]
    pub fn voice(mut self, voice: RealtimeVoice) -> Self {
        self.request.voice = Some(voice);
        self
    }

    /// Set temperature
    #[must_use]
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature.clamp(0.0, 2.0));
        self
    }

    /// Set max response tokens
    #[must_use]
    pub fn max_response_tokens(mut self, tokens: u32) -> Self {
        self.request.max_response_output_tokens = Some(tokens);
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> RealtimeSessionRequest {
        self.request
    }
}
