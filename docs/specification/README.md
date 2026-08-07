# EquivalenceMatrix V1 Specification

This directory is the repository-owned source of product and protocol intent
for EquivalenceMatrix v1.

Authority is organized as follows:

- `product.md`: product outcomes, use case, and exclusions;
- `architecture.md`: crate boundaries and evaluation flow;
- `decision-register.md`: the approved v1 decision set;
- `manifest-contracts.md`: exhaustive authored field and default contracts;
- `vocabularies.md`: closed values, selectors, IDs, and extension behavior;
- `schema-inventory.md`: exact schema identities and deterministic dispatch;
- `canonicalization.md`: finalized graph projection, ordering, JCS, and digest;
- `evaluation.md`: total policy, evidence, conformance, equivalence, and analysis tables;
- `protocol.md`: exact JSON, SARIF, evidence, adapter, attestation, and MCP DTOs;
- `cli.md`: command grammar, defaults, mutability, output, and exit behavior;
- `security-and-limits.md`: trust, subjects, execution boundaries, diagnostics, and limits;
- `requirements.tsv`: machine-readable decision-to-authority/test traceability;
- `../execution/commit-sequence.md`: corrected verified implementation order;
- `acceptance.md`: implementation and verification completion criteria;
- `naming-and-no-compat.md`: canonical identity and clean-break contract;
- later executable contracts for manifests, vocabularies, canonicalization,
  evaluation, protocols, CLI behavior, security, trust, and limits;
- `../adr/`: approved architectural decisions and later corrections;
- `provenance.md`: source-package integrity and import record.

The executable contracts added during RCLD 00 refine these approved decisions
without weakening them. If authority conflicts, record a new approved ADR
before implementing dependent behavior.
