#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = serde_json::from_slice::<serde_json::Value>(data) {
        if let Ok(canonical) = serde_json_canonicalizer::to_vec(&value) {
            let _ = eqm_domain::Sha256Digest::hash_content(&canonical);
        }
    }
});
