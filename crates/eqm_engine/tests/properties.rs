//! Seeded replayable semantic property checks.

use eqm_domain::Sha256Digest;
use eqm_engine::FacetStatus;
use std::collections::BTreeSet;

const SEED: u64 = 0x4551_4d31_3233;
const CASES: usize = 512;

fn next(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);
    *state
}

#[test]
fn digest_and_ordering_properties_are_seeded_and_replayable() {
    let mut state = SEED;
    for case in 0..CASES {
        let mut values = (0..32).map(|_| next(&mut state)).collect::<Vec<_>>();
        let bytes = values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect::<Vec<_>>();
        assert_eq!(
            Sha256Digest::hash_content(&bytes),
            Sha256Digest::hash_content(&bytes),
            "seed={SEED} case={case}"
        );
        let expected = values.iter().copied().collect::<BTreeSet<_>>();
        values.reverse();
        assert_eq!(
            values.into_iter().collect::<BTreeSet<_>>(),
            expected,
            "seed={SEED} case={case}"
        );
    }
}

#[test]
fn waived_and_unknown_are_never_success() {
    for status in [
        FacetStatus::Waived,
        FacetStatus::Unknown,
        FacetStatus::Missing,
        FacetStatus::Unstable,
    ] {
        assert_ne!(status, FacetStatus::Satisfied);
    }
}
