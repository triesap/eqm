# Independent review of b442b80

RF79 and P04-R4A are accepted for allocated matcher safety. All 47 prior numbered
acceptances remain intact; no whole requirement, aggregate repair or release is
accepted. S044/S047/S048/S049 remain partial; S052–S211 unstarted.

Production release matching rejects non-Target subjects before binding lookup.
Full-scope, single-member target-set and foreign-provider real-ingress controls
retain their obligations and reject ordinary target evidence. A separate authored
EachTarget-only synthetic control passes; it establishes no live signing trust.

Independent owning release lane: 16 passed. In an isolated review tree restoring
the previous matcher, all three new negative controls fail by returning Pass;
13 other release tests pass. This establishes causal RF79 regression coverage.
Pi's full standard/check/Clippy/docs/schema/structure evidence matches the committed
candidate; independent targeted checks do not stand in for extended acceptance.

## RF84 — dirty-source test is masked (P2, P04-R4C)

`dirty_source_inspection_cannot_produce_pass_or_conditional` starts from the same
full-scope fixture that the new negative control proves already non-authorizing.
Its six mutations therefore no longer demonstrate source-clean rejection. An
isolated probe removing `provenance.clean()` still passes this dirty-source test;
the missing guard is not detected. Restore
a supported synthetic clean Pass baseline and mutation-sensitive source controls,
keeping the full-scope fixture negative. Nearby typed-error controls still assert
errors and are not declared defective merely because they share this fixture.

RF75/RF78/RF81/RF82/RF83 remain incomplete; graph/status provenance, protected real
consumer and durable version/participation controls precede S052. Linux, extended
verify, live product/crypto/host and operational gates remain unverified. Human
release testing is not the next gate: substantial software work remains.
