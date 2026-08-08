#![no_main]
use eqm_domain::SourceName;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(source) = SourceName::new("fuzz/input.eqm.toml") else {
        return;
    };
    let _ = eqm_manifest::parse_toml(source, data);
});
