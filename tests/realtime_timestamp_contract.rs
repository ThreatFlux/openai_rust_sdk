//! Public Realtime response fixtures for the maintained timestamp migration.

use openai_rust_sdk::Timestamp;
use openai_rust_sdk::models::realtime_audio::{
    RealtimeSessionConfig, RealtimeSessionResponse, SessionStatus,
};

fn response(at: Timestamp) -> RealtimeSessionResponse {
    RealtimeSessionResponse {
        id: "maintained-time-fixture".to_owned(),
        object: "realtime.session".to_owned(),
        status: SessionStatus::Active,
        ephemeral_key: "synthetic-test-key".to_owned(),
        webrtc_url: "https://example.invalid/realtime".to_owned(),
        config: RealtimeSessionConfig::default(),
        expires_at: at,
        created_at: at,
    }
}

#[test]
fn realtime_json_keeps_offset_instants_and_nanoseconds() {
    let at: Timestamp = "2026-11-18T09:00:00.123456789+05:30".parse().unwrap();
    let json = serde_json::to_value(response(at)).unwrap();
    assert_eq!(json["created_at"], "2026-11-18T03:30:00.123456789Z");
    assert_eq!(json["expires_at"], json["created_at"]);
    let decoded: RealtimeSessionResponse = serde_json::from_value(json).unwrap();
    assert_eq!(decoded.created_at, at);
    assert_eq!(decoded.expires_at, at);
}

#[test]
fn realtime_json_preserves_negative_unix_fractions() {
    let at: Timestamp = "1969-12-31T23:59:59.999999999Z".parse().unwrap();
    let json = serde_json::to_value(response(at)).unwrap();
    assert_eq!(json["created_at"], "1969-12-31T23:59:59.999999999Z");
    assert_eq!(
        serde_json::from_value::<RealtimeSessionResponse>(json)
            .unwrap()
            .created_at,
        at
    );
}

#[test]
fn realtime_json_explicitly_normalizes_leap_seconds() {
    let ordinary: Timestamp = "2016-12-31T23:59:59.123456789Z".parse().unwrap();
    let mut json = serde_json::to_value(response(ordinary)).unwrap();
    json["created_at"] = serde_json::json!("2016-12-31T23:59:60.123456789Z");
    let decoded: RealtimeSessionResponse = serde_json::from_value(json).unwrap();
    assert_eq!(decoded.created_at, ordinary);
    assert_eq!(
        serde_json::to_value(decoded).unwrap()["created_at"],
        "2016-12-31T23:59:59.123456789Z"
    );
}

#[test]
fn realtime_json_rejects_invalid_or_ambiguous_instants_and_keeps_optional_null() {
    let ordinary = Timestamp::from_second(0).unwrap();
    for invalid in ["2026-02-30T00:00:00Z", "2026-11-18T09:00:00"] {
        let mut json = serde_json::to_value(response(ordinary)).unwrap();
        json["created_at"] = serde_json::json!(invalid);
        assert!(serde_json::from_value::<RealtimeSessionResponse>(json).is_err());
    }
    assert_eq!(
        serde_json::to_value(Option::<Timestamp>::None).unwrap(),
        serde_json::Value::Null
    );
    assert!(
        serde_json::from_value::<Option<Timestamp>>(serde_json::Value::Null)
            .unwrap()
            .is_none()
    );
}
