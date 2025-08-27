#![allow(clippy::pedantic, clippy::nursery)]
//! # Real-time Audio API Demo
//!
//! This example demonstrates how to use the `OpenAI` Real-time Audio API with WebRTC for:
//! - Creating real-time audio sessions
//! - Bidirectional audio streaming
//! - Voice activity detection
//! - Event handling and session management
//! - Real-time speech-to-speech interaction
//!
//! ## Usage
//!
//! Make sure you have set the `OPENAI_API_KEY` environment variable:
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! cargo run --example realtime_audio_demo
//! ```

use openai_rust_sdk::api::realtime_audio::{
    RealtimeAudioApi, RealtimeAudioConfig, RealtimeSessionBuilder,
};
use openai_rust_sdk::models::realtime_audio::{
    AudioBuffer, ContentPart, ConversationItem, ConversationItemStatus, ConversationItemType,
    ConversationRole, RealtimeAudioFormat, RealtimeEvent, RealtimeModality, RealtimeSessionConfig,
    RealtimeVoice, ResponseConfig, TurnDetectionConfig, TurnDetectionType,
    VoiceActivityDetectionConfig,
};
use std::env;
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;

/// Initialize the API client with configuration
fn create_api_client() -> Result<RealtimeAudioApi, Box<dyn std::error::Error>> {
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable must be set");

    let config = RealtimeAudioConfig {
        sample_rate: 24000,
        channels: 1,
        buffer_size_ms: 20,
        vad_config: VoiceActivityDetectionConfig {
            threshold: 0.6,
            prefix_padding_ms: 300,
            silence_duration_ms: 500,
        },
        connection_timeout: Duration::from_secs(30),
        max_reconnect_attempts: 3,
        enable_aec: true,
        enable_noise_suppression: true,
        enable_agc: true,
        ..Default::default()
    };

    Ok(RealtimeAudioApi::new_with_config(api_key, config)?)
}

/// Demo 1: Create a real-time audio session
async fn demo_session_creation(
    realtime_api: &RealtimeAudioApi,
) -> Result<
    std::sync::Arc<openai_rust_sdk::api::realtime_audio::RealtimeSession>,
    Box<dyn std::error::Error>,
> {
    println!("🎯 Demo 1: Creating Real-time Audio Session");
    println!("============================================");

    let session_request = RealtimeSessionBuilder::gpt_4o_realtime()
        .instructions("You are a helpful voice assistant. Respond naturally and conversationally.")
        .voice(RealtimeVoice::Alloy)
        .temperature(0.8)
        .max_response_tokens(1000)
        .config(RealtimeSessionConfig {
            input_audio_format: Some(RealtimeAudioFormat::Pcm16),
            output_audio_format: Some(RealtimeAudioFormat::Pcm16),
            voice_activity_detection: Some(VoiceActivityDetectionConfig::default()),
            turn_detection: Some(TurnDetectionConfig {
                detection_type: TurnDetectionType::ServerVad,
                threshold: Some(0.5),
                prefix_padding_ms: Some(300),
                silence_duration_ms: Some(500),
            }),
            modalities: Some(vec![RealtimeModality::Audio, RealtimeModality::Text]),
            tools: None,
            tool_choice: Some("auto".to_string()),
        })
        .build();

    match realtime_api.create_session(&session_request).await {
        Ok(session) => {
            println!("✅ Session created successfully: {}", session.id);
            Ok(session)
        }
        Err(e) => {
            println!("❌ Failed to create session: {e}");
            Err(e.into())
        }
    }
}

/// Demo 2: Session Management
async fn demo_session_management(
    realtime_api: &RealtimeAudioApi,
    session: &openai_rust_sdk::api::realtime_audio::RealtimeSession,
) -> Vec<String> {
    println!("\n📋 Demo 2: Session Management");
    println!("=============================");

    println!("🔍 Active sessions:");
    let sessions = realtime_api.list_sessions().await;
    for session_id in &sessions {
        println!("  - {session_id}");
    }

    println!("📊 Session statistics:");
    let stats = session.get_stats().await;
    println!("  - Connection state: {:?}", stats.connection_state);
    println!("  - Started at: {}", session.started_at());
    println!("  - Is active: {}", session.is_active());

    sessions
}

/// Demo 3: Event Handling
async fn demo_event_handling(
    session: &openai_rust_sdk::api::realtime_audio::RealtimeSession,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎭 Demo 3: Event Handling");
    println!("=========================");

    // Get event stream
    if let Some(mut event_receiver) = session.event_stream().await {
        println!("📡 Listening for events...");

        // Send a session update event
        let update_event = RealtimeEvent::SessionUpdate {
            event_id: Uuid::new_v4().to_string(),
            session: RealtimeSessionConfig::default(),
        };

        if let Err(e) = session.send_event(update_event).await {
            println!("⚠️  Failed to send event: {e}");
        } else {
            println!("📤 Sent session update event");
        }

        // Listen for events with timeout
        match timeout(Duration::from_secs(5), event_receiver.recv()).await {
            Ok(Some(event)) => {
                println!("📥 Received event: {event:?}");
            }
            Ok(None) => {
                println!("🔚 Event stream ended");
            }
            Err(_) => {
                println!("⏱️  Event listening timed out");
            }
        }
    }

    Ok(())
}

/// Demo 4: Audio Processing and Voice Activity Detection
fn demo_audio_processing() {
    println!("\n🎵 Demo 4: Audio Processing & Voice Activity Detection");
    println!("======================================================");

    // Create sample audio buffers
    let silent_audio = AudioBuffer::new(vec![0; 480], 24000, 1); // 20ms of silence
    let speech_audio = AudioBuffer::new(vec![1000; 480], 24000, 1); // 20ms of loud audio

    println!("🔇 Processing silent audio:");
    println!("  - Duration: {:.3}s", silent_audio.duration_seconds());
    println!("  - Frame count: {}", silent_audio.frame_count());
    println!("  - RMS energy: {:.2}", silent_audio.rms_energy());

    println!("🔊 Processing speech audio:");
    println!("  - Duration: {:.3}s", speech_audio.duration_seconds());
    println!("  - Frame count: {}", speech_audio.frame_count());
    println!("  - RMS energy: {:.2}", speech_audio.rms_energy());
}

/// Demo 5: Audio Streaming
async fn demo_audio_streaming(
    session: &openai_rust_sdk::api::realtime_audio::RealtimeSession,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎶 Demo 5: Audio Streaming");
    println!("==========================");

    // Get audio stream
    if let Some(mut audio_receiver) = session.audio_stream().await {
        println!("🎧 Starting audio streaming...");

        // Send some test audio
        let test_audio = AudioBuffer::new(
            (0..480)
                .map(|i| (1000.0 * (i as f32 * 0.01).sin()) as i16)
                .collect(),
            24000,
            1,
        );

        if let Err(e) = session.send_audio(test_audio).await {
            println!("⚠️  Failed to send audio: {e}");
        } else {
            println!("📤 Sent audio buffer");
        }

        // Listen for incoming audio with timeout
        match timeout(Duration::from_secs(5), audio_receiver.recv()).await {
            Ok(Some(audio_buffer)) => {
                println!("📥 Received audio buffer:");
                println!("  - Samples: {}", audio_buffer.samples.len());
                println!("  - Duration: {:.3}s", audio_buffer.duration_seconds());
                println!("  - Energy: {:.2}", audio_buffer.rms_energy());
            }
            Ok(None) => {
                println!("🔚 Audio stream ended");
            }
            Err(_) => {
                println!("⏱️  Audio listening timed out");
            }
        }
    }

    Ok(())
}

/// Demo 6: Real-time Conversation Simulation
async fn demo_conversation_simulation(
    session: &openai_rust_sdk::api::realtime_audio::RealtimeSession,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💬 Demo 6: Real-time Conversation Simulation");
    println!("============================================");

    // Create a conversation item
    let conversation_item = ConversationItem {
        id: Some(Uuid::new_v4().to_string()),
        object: "realtime.item".to_string(),
        item_type: ConversationItemType::Message,
        status: ConversationItemStatus::Completed,
        role: ConversationRole::User,
        content: vec![ContentPart::Text {
            text: "Hello! Can you hear me?".to_string(),
        }],
    };

    // Send conversation item create event
    let create_event = RealtimeEvent::ConversationItemCreate {
        event_id: Uuid::new_v4().to_string(),
        previous_item_id: None,
        item: conversation_item,
    };

    if let Err(e) = session.send_event(create_event).await {
        println!("⚠️  Failed to send conversation item: {e}");
    } else {
        println!("📝 Sent conversation item");
    }

    // Request a response
    let response_event = RealtimeEvent::ResponseCreate {
        event_id: Uuid::new_v4().to_string(),
        response: ResponseConfig {
            modalities: Some(vec![RealtimeModality::Audio, RealtimeModality::Text]),
            instructions: Some("Respond in a friendly, conversational tone.".to_string()),
            voice: Some(RealtimeVoice::Alloy),
            output_audio_format: Some(RealtimeAudioFormat::Pcm16),
            tools: None,
            tool_choice: Some("auto".to_string()),
            temperature: Some(0.8),
            max_response_output_tokens: Some(500),
        },
    };

    if let Err(e) = session.send_event(response_event).await {
        println!("⚠️  Failed to request response: {e}");
    } else {
        println!("🤖 Requested AI response");
    }

    Ok(())
}

/// Demo 7: Advanced Audio Features
fn demo_advanced_audio_features() {
    println!("\n🎛️  Demo 7: Advanced Audio Features");
    println!("===================================");

    // Test audio format conversion
    let stereo_audio = AudioBuffer::new(vec![100, -100, 200, -200], 24000, 2);
    let mono_converted = stereo_audio.to_mono();
    println!("🔄 Audio format conversion:");
    println!(
        "  - Original: {} samples, {} channels",
        stereo_audio.samples.len(),
        stereo_audio.channels
    );
    println!("  - Converted: {} samples (mono)", mono_converted.len());

    // Test different audio formats
    println!("🎵 Supported audio formats:");
    for format in [
        RealtimeAudioFormat::Pcm16,
        RealtimeAudioFormat::G711Ulaw,
        RealtimeAudioFormat::G711Alaw,
    ] {
        println!("  - {format:?}");
    }

    // Test different voices
    println!("🎤 Available voices:");
    for voice in [
        RealtimeVoice::Alloy,
        RealtimeVoice::Echo,
        RealtimeVoice::Fable,
        RealtimeVoice::Onyx,
        RealtimeVoice::Nova,
        RealtimeVoice::Shimmer,
    ] {
        println!("  - {voice:?}");
    }
}

/// Demo 8: Error Handling and Cleanup
async fn demo_cleanup(
    realtime_api: &RealtimeAudioApi,
    session: &openai_rust_sdk::api::realtime_audio::RealtimeSession,
    sessions: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧹 Demo 8: Error Handling and Cleanup");
    println!("======================================");

    // Test graceful session closure
    println!("🔒 Closing session gracefully...");
    if let Err(e) = session.close().await {
        println!("⚠️  Warning during session close: {e}");
    } else {
        println!("✅ Session closed successfully");
    }

    // Close all sessions
    for session_id in sessions {
        if let Err(e) = realtime_api.close_session(&session_id).await {
            println!("⚠️  Warning closing session {session_id}: {e}");
        }
    }

    Ok(())
}

/// Print completion summary
fn print_completion_summary() {
    println!("\n🎊 Demo completed successfully!");
    println!("===============================");
    println!("✨ All real-time audio features demonstrated:");
    println!("   - Session creation and management");
    println!("   - WebRTC connection setup");
    println!("   - Event-driven communication");
    println!("   - Bidirectional audio streaming");
    println!("   - Voice activity detection");
    println!("   - Audio processing and format conversion");
    println!("   - Real-time conversation handling");
    println!("   - Graceful error handling and cleanup");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎙️  OpenAI Real-time Audio API Demo");
    println!("===================================\n");

    // Initialize the API client
    let realtime_api = create_api_client()?;

    // Demo 1: Create a real-time audio session
    let session = demo_session_creation(&realtime_api).await?;

    // Demo 2: Session Management
    let sessions = demo_session_management(&realtime_api, &session).await;

    // Demo 3: Event Handling
    demo_event_handling(&session).await?;

    // Demo 4: Audio Processing and Voice Activity Detection
    demo_audio_processing();

    // Demo 5: Audio Streaming
    demo_audio_streaming(&session).await?;

    // Demo 6: Real-time Conversation Simulation
    demo_conversation_simulation(&session).await?;

    // Demo 7: Advanced Audio Features
    demo_advanced_audio_features();

    // Demo 8: Error Handling and Cleanup
    demo_cleanup(&realtime_api, &session, sessions).await?;

    // Print completion summary
    print_completion_summary();

    Ok(())
}
