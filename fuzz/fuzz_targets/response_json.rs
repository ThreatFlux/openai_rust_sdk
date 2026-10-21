#![no_main]

use libfuzzer_sys::fuzz_target;
use openai_rust_sdk::models::responses::ResponseResult;

fuzz_target!(|data: &[u8]| {
    if let Ok(response) = serde_json::from_slice::<ResponseResult>(data) {
        let _ = response.output_text();
        let _ = response.structured_data();
        let _ = response.parse_structured_data::<serde_json::Value>();
        let _ = response.has_valid_structured_data();
        let encoded = serde_json::to_vec(&response).expect("parsed response serializes");
        let _: ResponseResult = serde_json::from_slice(&encoded).expect("response roundtrip");
    }
});
