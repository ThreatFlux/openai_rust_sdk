//! # Voice Activity Detection
//!
//! Voice activity detection for real-time audio processing.

use crate::models::realtime_audio::{
    AudioBuffer, VoiceActivityDetectionConfig, VoiceActivityResult,
};
use chrono::Utc;
use std::time::Instant;

/// Voice Activity Detector
pub struct VoiceActivityDetector {
    /// Voice activity detection configuration
    config: VoiceActivityDetectionConfig,
    /// History of audio energy levels for smoothing
    energy_history: Vec<f32>,
    /// Timestamp of last detected speech
    last_speech_time: Option<Instant>,
    /// Whether speech is currently being detected
    is_speaking: bool,
    /// Audio sample rate (for future use)
    _sample_rate: u32,
}

impl VoiceActivityDetector {
    /// Create a new voice activity detector
    #[must_use]
    pub fn new(config: VoiceActivityDetectionConfig, sample_rate: u32) -> Self {
        Self {
            config,
            energy_history: Vec::new(),
            last_speech_time: None,
            is_speaking: false,
            _sample_rate: sample_rate,
        }
    }

    /// Process audio buffer and detect voice activity
    pub fn process(&mut self, audio_buffer: &AudioBuffer) -> VoiceActivityResult {
        let energy = audio_buffer.rms_energy();
        self.energy_history.push(energy);

        // Keep only recent history (last 100 frames)
        if self.energy_history.len() > 100 {
            self.energy_history.remove(0);
        }

        // Calculate average energy
        let avg_energy = if self.energy_history.is_empty() {
            0.0
        } else {
            self.energy_history.iter().sum::<f32>() / self.energy_history.len() as f32
        };

        // Determine if speech is detected
        let is_speech = energy > self.config.threshold && energy > avg_energy * 2.0;
        let confidence = if is_speech {
            (energy / self.config.threshold).min(1.0)
        } else {
            0.0
        };

        if is_speech {
            self.last_speech_time = Some(Instant::now());
            self.is_speaking = true;
        } else if let Some(last_speech) = self.last_speech_time {
            if last_speech.elapsed().as_millis() > u128::from(self.config.silence_duration_ms) {
                self.is_speaking = false;
            }
        }

        VoiceActivityResult {
            is_speech: self.is_speaking,
            confidence,
            energy,
            timestamp: Utc::now(),
        }
    }

    /// Check if currently speaking
    #[must_use]
    pub fn is_speaking(&self) -> bool {
        self.is_speaking
    }
}
