#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let (selector, payload) = data.split_first().unwrap_or((&0, &[]));
    match selector % 7 {
        0 => {
            let _ = eqm_protocol::TestResultDto::from_json(payload);
        }
        1 => {
            let _ = eqm_protocol::EvidenceResultDto::from_json(payload);
        }
        2 => {
            let _ = eqm_protocol::RuntimeFactsDto::from_json(payload);
        }
        3 => {
            let _ = eqm_protocol::InTotoStatementDto::from_json(payload);
        }
        4 => {
            let _ = eqm_protocol::AdapterRequestDto::from_json(payload);
        }
        5 => {
            let _ = eqm_protocol::AdapterResponseDto::from_json(payload);
        }
        _ => {
            let _ = serde_json::from_slice::<eqm_protocol::ReleaseRecordDto>(payload);
        }
    }
});
