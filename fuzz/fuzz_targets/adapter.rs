#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let midpoint = data.len() / 2;
    if let (Ok(request), Ok(response)) = (
        eqm_protocol::AdapterRequestDto::from_json(&data[..midpoint]),
        eqm_protocol::AdapterResponseDto::from_json(&data[midpoint..]),
    ) {
        let _ = response.matches_request(&request);
    }
});
