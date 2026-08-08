#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(response) = eqm_protocol::AdapterResponseDto::from_json(data) {
        if let Some(inventory) = response.inventory {
            let _ = serde_json::to_vec(&inventory);
        }
    }
});
