//! Audio processing types and WebRTC connection management
//!
//! Contains data structures for WebRTC connections, voice activity detection,
//! audio buffer management, and real-time audio processing utilities.

use chrono::{DateTime, Utc};

/// WebRTC connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebRtcConnectionState {
    /// Connection is new
    New,
    /// Connection is connecting
    Connecting,
    /// Connection is connected
    Connected,
    /// Connection is disconnected
    Disconnected,
    /// Connection failed
    Failed,
    /// Connection is closed
    Closed,
}

/// WebRTC peer connection statistics
#[derive(Debug, Clone)]
pub struct WebRtcStats {
    /// Connection state
    pub connection_state: WebRtcConnectionState,

    /// Audio bytes sent
    pub audio_bytes_sent: u64,

    /// Audio bytes received
    pub audio_bytes_received: u64,

    /// Audio packets sent
    pub audio_packets_sent: u64,

    /// Audio packets received
    pub audio_packets_received: u64,

    /// Audio packets lost
    pub audio_packets_lost: u64,

    /// Round trip time in milliseconds
    pub round_trip_time_ms: Option<f64>,

    /// Jitter in milliseconds
    pub jitter_ms: Option<f64>,

    /// Connected timestamp
    pub connected_at: Option<DateTime<Utc>>,
}

/// Voice activity detection result
#[derive(Debug, Clone)]
pub struct VoiceActivityResult {
    /// Whether voice activity is detected
    pub is_speech: bool,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,

    /// Audio energy level
    pub energy: f32,

    /// Timestamp of the detection
    pub timestamp: DateTime<Utc>,
}

/// Audio buffer for real-time processing
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    /// Audio samples (PCM 16-bit)
    pub samples: Vec<i16>,

    /// Sample rate in Hz
    pub sample_rate: u32,

    /// Number of channels
    pub channels: u16,

    /// Timestamp when buffer was created
    pub timestamp: DateTime<Utc>,
}

impl WebRtcStats {
    /// Create new WebRTC statistics with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate packet loss percentage
    #[must_use]
    pub fn packet_loss_percentage(&self) -> f64 {
        let total_sent = self.audio_packets_sent;
        if total_sent == 0 {
            return 0.0;
        }
        (self.audio_packets_lost as f64 / total_sent as f64) * 100.0
    }

    /// Calculate total audio data sent in MB
    #[must_use]
    pub fn audio_mb_sent(&self) -> f64 {
        self.audio_bytes_sent as f64 / (1024.0 * 1024.0)
    }

    /// Calculate total audio data received in MB
    #[must_use]
    pub fn audio_mb_received(&self) -> f64 {
        self.audio_bytes_received as f64 / (1024.0 * 1024.0)
    }

    /// Check if connection is active
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.connection_state == WebRtcConnectionState::Connected
    }

    /// Get connection duration in seconds if connected
    #[must_use]
    pub fn connection_duration_seconds(&self) -> Option<i64> {
        self.connected_at
            .map(|connected| Utc::now().signed_duration_since(connected).num_seconds())
    }
}

impl Default for WebRtcStats {
    fn default() -> Self {
        Self {
            connection_state: WebRtcConnectionState::New,
            audio_bytes_sent: 0,
            audio_bytes_received: 0,
            audio_packets_sent: 0,
            audio_packets_received: 0,
            audio_packets_lost: 0,
            round_trip_time_ms: None,
            jitter_ms: None,
            connected_at: None,
        }
    }
}

impl AudioBuffer {
    /// Create a new audio buffer
    #[must_use]
    pub fn new(samples: Vec<i16>, sample_rate: u32, channels: u16) -> Self {
        Self {
            samples,
            sample_rate,
            channels,
            timestamp: Utc::now(),
        }
    }

    /// Create an empty audio buffer with specified capacity
    #[must_use]
    pub fn with_capacity(capacity: usize, sample_rate: u32, channels: u16) -> Self {
        Self {
            samples: Vec::with_capacity(capacity),
            sample_rate,
            channels,
            timestamp: Utc::now(),
        }
    }

    /// Get the duration of the audio buffer in seconds
    #[must_use]
    pub fn duration_seconds(&self) -> f64 {
        self.samples.len() as f64 / (f64::from(self.sample_rate) * f64::from(self.channels))
    }

    /// Get the duration of the audio buffer in milliseconds
    #[must_use]
    pub fn duration_ms(&self) -> u32 {
        (self.duration_seconds() * 1000.0) as u32
    }

    /// Get the number of frames in the buffer
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.samples.len() / self.channels as usize
    }

    /// Convert to mono if stereo
    #[must_use]
    pub fn to_mono(&self) -> Vec<i16> {
        if self.channels == 1 {
            self.samples.clone()
        } else {
            self.samples
                .chunks(self.channels as usize)
                .map(|frame| {
                    let sum: i32 = frame.iter().map(|&s| i32::from(s)).sum();
                    (sum / frame.len() as i32) as i16
                })
                .collect()
        }
    }

    /// Get RMS (Root Mean Square) energy level
    #[must_use]
    pub fn rms_energy(&self) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let sum_squares: f64 = self.samples.iter().map(|&s| f64::from(s).powi(2)).sum();
        (sum_squares / self.samples.len() as f64).sqrt() as f32
    }

    /// Get peak amplitude
    #[must_use]
    pub fn peak_amplitude(&self) -> i16 {
        self.samples.iter().map(|&s| s.abs()).max().unwrap_or(0)
    }

    /// Check if buffer is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Get the size of the buffer in bytes
    #[must_use]
    pub fn size_bytes(&self) -> usize {
        self.samples.len() * std::mem::size_of::<i16>()
    }

    /// Append samples to the buffer
    pub fn append(&mut self, samples: &[i16]) {
        self.samples.extend_from_slice(samples);
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.samples.clear();
        self.timestamp = Utc::now();
    }

    /// Truncate the buffer to specified length
    pub fn truncate(&mut self, len: usize) {
        self.samples.truncate(len);
    }

    /// Split the buffer at the specified index
    pub fn split_off(&mut self, at: usize) -> AudioBuffer {
        let remaining_samples = self.samples.split_off(at);
        AudioBuffer {
            samples: remaining_samples,
            sample_rate: self.sample_rate,
            channels: self.channels,
            timestamp: Utc::now(),
        }
    }
}

impl VoiceActivityResult {
    /// Create a new voice activity result
    #[must_use]
    pub fn new(is_speech: bool, confidence: f32, energy: f32) -> Self {
        Self {
            is_speech,
            confidence,
            energy,
            timestamp: Utc::now(),
        }
    }

    /// Check if this result indicates speech with high confidence
    #[must_use]
    pub fn is_confident_speech(&self, confidence_threshold: f32) -> bool {
        self.is_speech && self.confidence >= confidence_threshold
    }

    /// Check if this result indicates silence with high confidence
    #[must_use]
    pub fn is_confident_silence(&self, confidence_threshold: f32) -> bool {
        !self.is_speech && self.confidence >= confidence_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_buffer_operations() {
        let samples = vec![100, -200, 300, -400];
        let buffer = AudioBuffer::new(samples, 44100, 2);

        assert_eq!(buffer.frame_count(), 2);
        assert!(buffer.duration_seconds() > 0.0);

        let mono = buffer.to_mono();
        assert_eq!(mono.len(), 2);
        assert_eq!(mono[0], -50); // (100 + (-200)) / 2
        assert_eq!(mono[1], -50); // (300 + (-400)) / 2

        let energy = buffer.rms_energy();
        assert!(energy > 0.0);
    }

    #[test]
    fn test_webrtc_stats_default() {
        let stats = WebRtcStats::default();
        assert_eq!(stats.connection_state, WebRtcConnectionState::New);
        assert_eq!(stats.audio_bytes_sent, 0);
        assert_eq!(stats.audio_packets_lost, 0);
        assert!(stats.connected_at.is_none());
        assert!(!stats.is_connected());
    }

    #[test]
    fn test_webrtc_stats_calculations() {
        let stats = WebRtcStats {
            audio_packets_sent: 100,
            audio_packets_lost: 10,
            audio_bytes_sent: 1024 * 1024, // 1 MB
            ..Default::default()
        };

        assert_eq!(stats.packet_loss_percentage(), 10.0);
        assert_eq!(stats.audio_mb_sent(), 1.0);
    }

    #[test]
    fn test_voice_activity_result() {
        let result = VoiceActivityResult::new(true, 0.8, 100.0);

        assert!(result.is_speech);
        assert_eq!(result.confidence, 0.8);
        assert_eq!(result.energy, 100.0);
        assert!(result.is_confident_speech(0.7));
        assert!(!result.is_confident_silence(0.7));
    }

    #[test]
    fn test_audio_buffer_with_capacity() {
        let buffer = AudioBuffer::with_capacity(1000, 48000, 1);

        assert!(buffer.is_empty());
        assert_eq!(buffer.sample_rate, 48000);
        assert_eq!(buffer.channels, 1);
        assert_eq!(buffer.samples.capacity(), 1000);
    }

    #[test]
    fn test_audio_buffer_append_and_clear() {
        let mut buffer = AudioBuffer::new(vec![1, 2, 3], 44100, 1);

        buffer.append(&[4, 5, 6]);
        assert_eq!(buffer.samples, vec![1, 2, 3, 4, 5, 6]);

        buffer.clear();
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_audio_buffer_split_off() {
        let mut buffer = AudioBuffer::new(vec![1, 2, 3, 4, 5, 6], 44100, 1);

        let second_half = buffer.split_off(3);

        assert_eq!(buffer.samples, vec![1, 2, 3]);
        assert_eq!(second_half.samples, vec![4, 5, 6]);
        assert_eq!(buffer.sample_rate, second_half.sample_rate);
    }

    #[test]
    fn test_audio_buffer_peak_amplitude() {
        let buffer = AudioBuffer::new(vec![-300, 100, -500, 200], 44100, 1);

        assert_eq!(buffer.peak_amplitude(), 500);
    }

    #[test]
    fn test_audio_buffer_duration_ms() {
        let buffer = AudioBuffer::new(vec![1, 2, 3, 4], 1000, 1); // 1000 Hz, 4 samples

        assert_eq!(buffer.duration_ms(), 4); // 4 samples / 1000 Hz = 4ms
    }

    #[test]
    fn test_webrtc_connection_duration() {
        let mut stats = WebRtcStats::default();

        // Test without connection time
        assert!(stats.connection_duration_seconds().is_none());

        // Test with connection time
        stats.connected_at = Some(Utc::now() - chrono::Duration::seconds(30));
        let duration = stats.connection_duration_seconds();
        assert!(duration.is_some());
        assert!(duration.unwrap() >= 29 && duration.unwrap() <= 31); // Allow some timing variance
    }
}
