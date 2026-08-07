# EquivalenceMatrix V1 Specification

This directory is the repository-owned source of product and protocol intent
for EquivalenceMatrix v1.

Authority is organized as follows:

- `product.md`: product outcomes, use case, and exclusions;
- `architecture.md`: crate boundaries and evaluation flow;
- `decision-register.md`: the approved v1 decision set;
- `acceptance.md`: implementation and verification completion criteria;
- `naming-and-no-compat.md`: canonical identity and clean-break contract;
- later executable contracts for manifests, vocabularies, canonicalization,
  evaluation, protocols, CLI behavior, security, trust, and limits;
- `../adr/`: approved architectural decisions and later corrections;
- `provenance.md`: source-package integrity and import record.

The executable contracts added during RCLD 00 refine these approved decisions
without weakening them. If authority conflicts, record a new approved ADR
before implementing dependent behavior.
