# EquivalenceMatrix V1 Acceptance Criteria

## Product

- Model capabilities, journeys, surfaces, fragments, and requirements.
- Bind units to independently implemented targets and typed artifacts.
- Model intended exposure separately from discovered, enabled, released, and
  conformant facts.
- Derive policy/profile/target/facet/trust/release obligations.
- Evaluate evidence coverage, outcomes, freshness, trust, and waivers.
- Evaluate target conformance before target-set equivalence.
- Provide bounded agent context and exact release checks.

## Engineering

- Implement the approved crate graph without cycles or boundary leaks.
- Keep `eqm_domain` and `eqm_engine` pure and deterministic.
- Accept current manifests, schemas, and protocols only.
- Reject unknown fields and malformed references with source diagnostics.
- Produce deterministic canonical digests and machine output.
- Enforce runner, adapter, path, resource, trust, and protected-baseline
  boundaries.
- Keep authored metadata and generated local state distinct.
- Contain no compatibility aliases, readers, migrations, or fallbacks.

## Verification

- Formatting, locked check/test/Clippy/rustdoc, and diff gates pass.
- Unit, fixture, golden, property, fuzz, adversarial, cross-platform,
  integration, and release-gate tests exist and pass.
- Generated schema parity and example validation pass without suppression.
- Deterministic repeated outputs are byte-identical.
- Compatibility, dependency, license, security, package-content, SBOM, and
  provenance checks pass.
- Required performance and memory targets are measured and satisfied.
- Final evidence records exact commands, results, deviations, unresolved
  organizational inputs, and honest release non-claims.
