# Independent supervisor review — 00bec96

Reviewed clean candidate `00bec96b0e97c09898975700bbd1790c0c27de79` on 2026-10-04.
The returned batch comprises four retained commits: P03-R8 `4b6e86e`, P03-R9
`7889bc6`, P03-R10 `f1419e4`, S043-R4 `00bec96`. Two actual implementation
attempts were matched externally; the latter continued the former and reached
the required independent review boundary. No production source/tests edited by
Codex. Preserve all 42 previously accepted numbered steps.

RF65 is accepted for its allocated repair: nested target/provider scenario
references use their owning vocabulary and source roots enforce cardinality.
RF45 whole graph parity remains partial. S043/P03 remain changes-required;
whole RA02/RA13, original 64 R/AC,21 RA,Q01–Q15,S211 remain unchanged.

## RF66 — Wrong owning type for fragment prefix (P2)

`crates/eqm_protocol/src/schema_graph_v2.rs::FragmentUseV2Dto.prefix` uses
selector text. `crates/eqm_domain/src/entity.rs::FragmentUse.prefix` is
`Option<LocalRequirementId>`. The audit/corpus inherit the wrong type allocation.
An isolated real Draft2020-12 validator accepts literal `Bad Prefix` in
`/surfaces/0/fragment_origins/0/prefix`; `LocalRequirementId::new` rejects it.
Preserve the existing owning single-segment lowercase ASCII ID contract; repair
schema/type mapping and analogous misallocations. Do not loosen the domain.

## RF67 — Locally expressible collection invariants omitted (P2)

The graph schema accepts a surface with both requirements and fragment origins
empty; `Surface::new` rejects it with SurfaceCompositionRequired. It also accepts
empty journey surfaces or duplicate identical surface IDs; `Journey::new` rejects
both. A one-surface owning positive succeeds. All four mismatch controls are
executed in isolated probes, with valid schema baseline and owning controls.
The surface condition is locally expressible with anyOf; journey cardinality and
uniqueItems preserve order. These are existing invariants, not new policy.

## Evidence and limits

Independent standard gate: exit0, 766 successful test executions. Owning domain,
manifest, protocol, engine and application doctests: exit0,9. Package structure
validator and diff checks passed. Isolated probes: exit0,2 tests reproducing four
mismatches. Raw evidence retained externally under the supervisor receipt.
Generated v1 schemas and Cargo.lock unchanged. The corpus's schema-only helper
checks do not establish owning parity; use same-input owning APIs/checked loaders
and a genuine finalized positive graph in the next batch. Preserve intentional
schema/token/Unicode/relationship layers explicitly.

No release readiness or human release-testing gate reached. Full verify/Linux/
coverage/mutation/fuzz/benchmark/package/live product/crypto/host/signing and
operational acceptance remain unverified. Local automation is not inherently
hardware-blocked. DEV-025 in the sole COMMIT_SEQUENCE owns the next batch.
