# Compatibility, schemas and migration contract

## Four version axes (R21, R52)

Version the public wire schemas, evaluation semantics, optional native ABI, and database schema independently. `eqm_product_v1` is this specification's name; it is not permission to repurpose existing `schemas/v1` URLs with incompatible semantics. Tool/package version is another implementation identity used for reproducibility.

The baseline repository explicitly supports current-only v1, prohibits legacy readers/aliases, and generates schemas from DTOs. Preserve that public contract until the compatibility decision step records an explicit policy. Do not introduce invisible fallback readers merely to keep a fixture passing. Repository proof that no public legacy compatibility is required may permit a coordinated version transition, but it must be documented.

## Version decision gate

Before new incompatible wire shapes are exposed, inspect actual published releases, consumers, schema generators/URI checks and declared support obligations. Record the chosen new namespace/version or coordinated pre-release transition and corresponding exact tests. Do not select a magic version number from this package. Human approval is required when a choice changes externally committed compatibility or product scope.

Exact JSON field names, optionality, integer ranges, timestamp precision, extension semantics and digest domains are closed in this decision and accompanying protocol spec. Unknown security-critical semantics fail closed. Reader compatibility, where explicitly supported, is tested; unsupported versions produce clear errors rather than implicit reinterpretation.

## Signed records

Retain original exact bytes and signature material. Migration creates new records with explicit source/provenance, never rewrites an original while claiming its signature covers changed data. Historical replay may require the original evaluator/runtime in a controlled environment; an unsupported version is a stated limitation, not a reason to promote results under new semantics automatically.

## Database transitions

Migrations execute under exclusive coordination with safe interruption/recovery and backups appropriate to the selected storage guarantees. Version checks prevent incompatible concurrent writers. Test upgrade, interruption, reopen, rollback or restore, and unavailable older/newer readers. Schema migration does not silently destroy immutable objects/receipts or release archives. Retention cleanup is a separate authorized lifecycle.

## Distribution

Exact dependencies and toolchain pins remain justified and locked. Audit the actual linked SQLite and cryptographic implementation and the included native libraries. Keep generated artifacts out of source except intentional generated schemas/approved bindings. Preserve reproducible distribution verification, checksums, SPDX/SBOM inputs and provenance. Local unsigned distribution is not publication/signing authorization.

Native/sidecar consumers negotiate compatibility and fail explicitly when unsupported. UI/report consumers show unsupported statuses instead of assuming success. Support matrices are owner-approved and backed by host execution, not merely cross-compilation.

## Revision 2 owner-approved direction

ADR-0012 resolves the direction in Q03: incompatible changes use an explicit new namespace; preserve signed originals, reject unsupported versions and avoid implicit fallback readers. S006 still determines exact namespace/URI spelling, numeric/time/null/set representations and the consumer transition from inspected evidence. No known external consumer obligation may be silently discarded; document a newly discovered conflict before exposure. Wire/evaluator/database/optional-ABI versions remain independent.

The approved initial EQM execution/storage host targets are macOS arm64 and Linux x86_64. They are verification obligations, not completed host certifications. Product participant OS/SDK/cohort promises remain separate unresolved facts. No N/N-1 cohort or Windows support is inferred.

## DEV-002 implementation ownership

The Codex supervisor has resolved the transition engineering allocation:
S015-R1 preserves the pre-S015 v1 accepted range and corrects v2 primitives;
S015-R2 introduces version-qualified identities and dual-inventory generator
plumbing; S015-R3 adds five concrete v2 protocol families and their nested DTOs
with numeric/time/absence validation. The exact schedule is in the existing
COMMIT_SEQUENCE.md. v1 signed originals, DTO behavior and schemas stay stable;
strict duplicate rejection remains the approved security correction. Subsequent
owning steps add actual v2 shapes and explicit consumers together. No placeholder
manifest schemas or implicit version conversion is authorized. The staged v2
contract is not a published release or a promise of indefinite v1/v2 support.
