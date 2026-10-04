use std::{
    sync::OnceLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use openai_rust_sdk::webhooks::{WebhookError, WebhookHeaders, WebhookVerifier};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SENT: u64 = 1_750_287_078;
const BODY: &str = r#"{
  "object": "event", "id": "evt_synthetic", "type": "response.completed",
  "created_at": 1750287018, "data": {"id":"resp_synthetic","future":false}, "future":{"n":0}
}"#;

const fn headers(signature: &str) -> WebhookHeaders<'_> {
    WebhookHeaders {
        id: "wh_synthetic_01",
        timestamp: "1750287078",
        signature,
    }
}

fn now(seconds: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(seconds)
}

struct Fixture {
    key: String,
    signature: String,
    binary_signature: String,
}

impl Fixture {
    fn verifier(&self) -> WebhookVerifier {
        WebhookVerifier::new(&self.key).unwrap()
    }

    // Independent RFC 2104 reference: two SHA-256 passes, without the HMAC
    // implementation used by the production verifier. The random ASCII key
    // always fits one SHA-256 block, so this fixture needs no key hashing.
    fn sign(&self, body: &[u8]) -> String {
        assert!(self.key.len() <= 64);
        let mut inner_pad = [0x36; 64];
        let mut outer_pad = [0x5c; 64];
        for (index, byte) in self.key.bytes().enumerate() {
            inner_pad[index] ^= byte;
            outer_pad[index] ^= byte;
        }
        let mut inner = Sha256::new();
        inner.update(inner_pad);
        inner.update(b"wh_synthetic_01.1750287078.");
        inner.update(body);
        let mut outer = Sha256::new();
        outer.update(outer_pad);
        outer.update(inner.finalize());
        STANDARD.encode(outer.finalize())
    }
}

fn fixture() -> &'static Fixture {
    static FIXTURE: OnceLock<Fixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        // Fresh test-only key material instead of committed signing keys.
        let mut fixture = Fixture {
            key: STANDARD.encode(rand::random::<[u8; 32]>()),
            signature: String::new(),
            binary_signature: String::new(),
        };
        fixture.signature = fixture.sign(BODY.as_bytes());
        fixture.binary_signature = fixture.sign(b"\xff\xfe\x00");
        fixture
    })
}

#[test]
fn independent_reference_accepts_plain_and_prefixed_keys() {
    let fixture = fixture();
    let encoded = format!("whsec_{}", STANDARD.encode(&fixture.key));
    for secret in [fixture.key.as_str(), encoded.as_str()] {
        let verifier = WebhookVerifier::new(secret).unwrap();
        assert_eq!(
            verifier.verify_at(BODY.as_bytes(), headers(&fixture.signature), now(SENT)),
            Ok(())
        );
        let event = verifier
            .unwrap_at(BODY.as_bytes(), headers(&fixture.signature), now(SENT))
            .unwrap();
        assert_eq!(event.event_type, "response.completed");
        assert_eq!(event.resource_id(), Some("resp_synthetic"));
        assert_eq!(event.data["future"], false);
        assert_eq!(event.extra["future"]["n"], 0);
    }
}

#[test]
fn authenticates_exact_body_bytes_and_header_values() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    let parsed: Value = serde_json::from_str(BODY).unwrap();
    let reserialized = serde_json::to_vec(&parsed).unwrap();
    assert_eq!(
        verifier.verify_at(&reserialized, headers(&fixture.signature), now(SENT)),
        Err(WebhookError::InvalidSignature)
    );
    let changed_id = WebhookHeaders {
        id: "wh_different",
        ..headers(&fixture.signature)
    };
    assert_eq!(
        verifier.verify_at(BODY.as_bytes(), changed_id, now(SENT)),
        Err(WebhookError::InvalidSignature)
    );
    let changed_time = WebhookHeaders {
        timestamp: "1750287079",
        ..headers(&fixture.signature)
    };
    assert_eq!(
        verifier.verify_at(BODY.as_bytes(), changed_time, now(SENT)),
        Err(WebhookError::InvalidSignature)
    );
}

#[test]
fn signature_rotation_accepts_any_valid_supported_signature() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    let signature = &fixture.signature;
    let rotated = format!("v2,unsupported v1,malformed v1,{signature}");
    assert_eq!(
        verifier.verify_at(BODY.as_bytes(), headers(&rotated), now(SENT)),
        Ok(())
    );
    let unsupported = format!("v2,{signature}");
    assert_eq!(
        verifier.verify_at(BODY.as_bytes(), headers(&unsupported), now(SENT)),
        Err(WebhookError::InvalidSignature)
    );
    assert_eq!(
        verifier.verify_at(BODY.as_bytes(), headers("v1,AAAA"), now(SENT)),
        Err(WebhookError::InvalidSignature)
    );
}

#[test]
fn rejects_old_and_future_timestamps_and_accepts_window_boundaries() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    for current in [SENT - 300, SENT + 300] {
        assert_eq!(
            verifier.verify_at(BODY.as_bytes(), headers(&fixture.signature), now(current)),
            Ok(())
        );
    }
    for current in [SENT - 301, SENT + 301] {
        assert_eq!(
            verifier.verify_at(BODY.as_bytes(), headers(&fixture.signature), now(current)),
            Err(WebhookError::TimestampOutsideTolerance)
        );
    }
    assert_eq!(
        verifier.with_tolerance(Duration::from_secs(301)).verify_at(
            BODY.as_bytes(),
            headers(&fixture.signature),
            now(SENT + 301)
        ),
        Ok(())
    );
}

#[test]
fn rejects_malformed_times_and_invalid_clocks() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    for timestamp in ["-1", "+1750287078", "1750287078.0", "18446744073709551616"] {
        let malformed = WebhookHeaders {
            timestamp,
            ..headers(&fixture.signature)
        };
        assert_eq!(
            verifier.verify_at(BODY.as_bytes(), malformed, now(SENT)),
            Err(WebhookError::InvalidTimestamp)
        );
    }
    assert_eq!(
        verifier.verify_at(
            BODY.as_bytes(),
            headers(&fixture.signature),
            UNIX_EPOCH - Duration::from_secs(1)
        ),
        Err(WebhookError::InvalidClock)
    );
}

#[test]
fn raw_binary_signature_verifies_before_json_validation() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    assert_eq!(
        verifier.verify_at(
            b"\xff\xfe\x00",
            headers(&fixture.binary_signature),
            now(SENT)
        ),
        Ok(())
    );
    assert_eq!(
        verifier.unwrap_at(
            b"\xff\xfe\x00",
            headers(&fixture.binary_signature),
            now(SENT)
        ),
        Err(WebhookError::InvalidPayload)
    );
    assert_eq!(
        verifier.unwrap_at(b"{not json", headers("v1,AAAA"), now(SENT)),
        Err(WebhookError::InvalidSignature)
    );
}

#[test]
fn future_event_data_and_extensions_round_trip() {
    let fixture = fixture();
    let payload = json!({
        "id":"evt_future", "object":"event", "type":"future.resource.changed",
        "created_at":SENT, "data":{"nested":[null, false, 0], "future":{"id":"x"}},
        "future_envelope": {"enabled":false}
    });
    let body = serde_json::to_vec(&payload).unwrap();
    let signature = fixture.sign(&body);
    let event = WebhookVerifier::new(&fixture.key)
        .unwrap()
        .unwrap_at(&body, headers(&signature), now(SENT))
        .unwrap();
    assert_eq!(event.resource_id(), None);
    assert_eq!(serde_json::to_value(event).unwrap(), payload);
}

#[test]
fn authenticated_malformed_envelopes_fail_without_echoing_payloads() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    for payload in [
        json!({"id":"evt", "object":"event", "type":"response.completed", "created_at":SENT}),
        json!({"id":"evt", "object":"event", "type":4, "created_at":SENT, "data":{}}),
        json!({"id":"", "object":"event", "type":"future", "created_at":SENT, "data":{}}),
        json!({"id":"evt", "object":"other", "type":"future", "created_at":SENT, "data":{}}),
        json!({"id":"evt", "object":"event", "type":"future", "created_at":-1, "data":{}}),
        json!({"id":"evt", "object":"event", "type":"future", "created_at":SENT, "data":[]}),
    ] {
        let body = serde_json::to_vec(&payload).unwrap();
        let signature = fixture.sign(&body);
        assert_eq!(
            verifier.unwrap_at(&body, headers(&signature), now(SENT)),
            Err(WebhookError::InvalidPayload)
        );
    }
}

#[test]
fn bounds_untrusted_headers_and_payloads() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    let large = "x".repeat(8193);
    assert_eq!(
        verifier.verify_at(BODY.as_bytes(), headers(&large), now(SENT)),
        Err(WebhookError::HeaderTooLarge)
    );
    assert_eq!(
        verifier
            .clone()
            .with_max_body_bytes(BODY.len() - 1)
            .unwrap()
            .verify_at(BODY.as_bytes(), headers(&fixture.signature), now(SENT)),
        Err(WebhookError::BodyTooLarge)
    );
    assert_eq!(
        verifier.with_max_body_bytes(0).unwrap_err(),
        WebhookError::InvalidBodyLimit
    );
}

#[test]
fn empty_headers_and_bad_secrets_have_static_errors() {
    let fixture = fixture();
    for secret in ["", "whsec_", "whsec_%%%"] {
        assert_eq!(
            WebhookVerifier::new(secret).unwrap_err(),
            WebhookError::InvalidSecret
        );
    }
    let verifier = fixture.verifier();
    for (name, missing) in [
        (
            "webhook-id",
            WebhookHeaders {
                id: "",
                ..headers(&fixture.signature)
            },
        ),
        (
            "webhook-timestamp",
            WebhookHeaders {
                timestamp: "",
                ..headers(&fixture.signature)
            },
        ),
        (
            "webhook-signature",
            WebhookHeaders {
                signature: "",
                ..headers(&fixture.signature)
            },
        ),
    ] {
        assert_eq!(
            verifier.verify_at(BODY.as_bytes(), missing, now(SENT)),
            Err(WebhookError::MissingHeader(name))
        );
    }
}

#[test]
fn debug_redacts_keys_and_supplied_signatures() {
    let fixture = fixture();
    let verifier = fixture.verifier();
    let debug = format!("{verifier:?}");
    assert!(!debug.contains(&fixture.key));
    assert!(debug.contains("[REDACTED]"));
    assert!(!format!("{:?}", headers(&fixture.signature)).contains(&fixture.signature));
}
