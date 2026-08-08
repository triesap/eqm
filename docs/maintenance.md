# Versioning and maintenance

The Rust crates and `eqm` package share one semantic version. V1 schema and MCP
identities are exact and current-only: incompatible semantic changes require a
new major; additive compatible behavior requires a minor; fixes without public
contract change require a patch. V1 does not add deprecated aliases, fallback
readers, redirects, or migration commands.

Maintainers update source, tests, specifications, schemas, examples, fixtures,
goldens, CLI/MCP descriptions, and package contents in the same reviewed
sequence. Generated schemas come only from `scripts/generate_schemas.sh` and
must pass clean regeneration/parity. Dependencies remain exactly locked;
security advisories, licenses, MSRV, isolated tool locks, SBOM output, and
reproducibility are reviewed before merge.

The aggregate verifier is the minimum maintenance gate. Release candidates
also run all fuzz targets, the scale probe, two-package byte comparison,
archive inspection, checksum verification, no-compat scan, and CLI/MCP/release
fixtures. Publication requires separately verified namespace control, legal
clearance, security ownership, protected CI, signing identity, retention
policy, pilot inputs, and release authorization. None is inferred from a green
local package.
