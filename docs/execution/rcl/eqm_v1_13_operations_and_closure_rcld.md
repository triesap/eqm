# EquivalenceMatrix V1 RCLD 13: Operations And Closure

Status: in progress; Step 134 corrective completion audit active
Created: 2026-08-07
Updated: 2026-08-08
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 12
Current checkpoint: step_134

## Purpose

Complete standalone operator, agent, security, contribution, and release
guidance; enforce final current-only naming and schema policy; and record a
fully reproducible acceptance report without overstating organizational or
production release readiness.

## Scope Boundary

This RCLD documents and verifies the implemented product. It does not add new
product behavior, publish artifacts, configure real trust roots, choose owners,
integrate real pilot applications, or manufacture legal/name clearance.
Unexpected behavioral work discovered here returns to the owning earlier RCLD
as a corrective checkpoint before final closure.

## Definition Of Green

- Operators can configure profiles, baselines, runners, adapters, trust,
  waivers, evidence, and release gates from standalone documentation.
- Agents receive bounded-context, authority, untrusted-data, and mutation
  guidance consistent with the implemented CLI/MCP behavior.
- Security policy and threat model match actual controls and limitations.
- Contribution and release guides match repository commands and package state.
- No forbidden compatibility surface exists outside controlled rejection data.
- Every required verification lane passes without suppressed failure.
- The final report distinguishes implemented/local evidence from unresolved
  organizational, pilot, legal, publication, and production trust inputs.

## Verification Lane

Run every repository-required lane: locked Rust gates, authority validation,
docs/link checks, schema parity, deterministic output, no-legacy scan,
fixtures/goldens, properties, fuzz smoke, adversarial security, cross-platform,
coverage/mutation, performance/resource, dependency/license, SBOM/provenance,
clean package, CLI/MCP smoke, and release-gate fixtures. Run final commands from
a clean checkout/package context and require a clean worktree afterward.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_129` | Add operator guide for workspace setup, profiles, exact baselines, runners, adapters, evidence, trust, waivers, facts, releases, offline operation, and recovery. | Commands/examples are current, tested where possible, standalone, and do not imply unavailable production authority. | `docs(eqm): add operator guide` |
| `step_130` | Add agent guide for context, obligations, affected, check, explicit verify authority, untrusted data, bounded output, and authored mutation boundaries. | Guide matches CLI/MCP schemas and root instructions; no prose can authorize waiver/trust/policy mutation. | `docs(eqm): add agent guide` |
| `step_131` | Add `SECURITY.md`, threat model, disclosure process, supported-major statement, control/limitation mapping, and security-owner placeholder gate. | Threats map to implemented tests; unsupported guarantees are explicit; no fake contact or owner is invented. | `docs(eqm): add security policy` |
| `step_132` | Replace bootstrap contribution guidance and add versioning, maintenance, generated-file, test, dependency, and release guide. | All documented commands exist; Cargo/package/license metadata agree; release publication prerequisites are explicit. | `docs(eqm): add contributor and release guides` |
| `step_133` | Run and harden final no-compat enforcement across source, schemas, normal docs, examples, fixtures, CI, packaging, and produced artifacts. | Scanner and negative self-tests pass; only controlled rejection data contains forbidden tokens; binary/package inspection is clean. | `chore(eqm): enforce no compatibility names` |
| `step_134` | Run complete acceptance, remove dead code, reconcile requirements/tests/checkpoints, and record final verification plus remaining organizational inputs. | Every mandatory command passes, including unsuppressed schema verification; worktree is clean; report has exact commands/results and honest non-claims. | `chore(eqm): complete v1 verification` |

## Final Acceptance Record

The final report must include:

- repository commit and toolchain identity;
- crate/package versions and dependency-lock digest;
- requirements implemented and direct evidence;
- commands run and exact results;
- schema, example, fixture, and golden digests;
- property/fuzz/security/coverage/mutation/performance summaries;
- clean-package contents, checksums, SBOM, and provenance inputs;
- CLI/MCP/release fixture results;
- deviations and their ADRs;
- unresolved organizational inputs and resulting non-claims;
- confirmation that no push, publication, signing, or deployment occurred
  unless separately authorized and evidenced.

## Organizational Completion Gate

Local implementation acceptance may complete while these remain explicitly
open: real CODEOWNERS, protected baseline, trusted CI/signing identity, pilot
roots/build commands, retention policy, namespace control, and legal/name
clearance. Package publication and production release readiness cannot complete
until those values are supplied and verified.

## Reconciliation Rules

- Final cleanup cannot conceal a failed gate or delete a test to obtain green.
- Any behavior change returns to the owning RCLD and reruns downstream gates.
- The multi-RCLD umbrella is marked complete only after this ledger and every
  preceding child ledger are complete.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_129` | complete | The standalone operator guide covers initialization, authored/generated boundaries, validation/checking, explicit profiles, exact baselines, conservative affected analysis, pinned runners/adapters, discovery/reconciliation, dry-run and authorized verification, immutable evidence, independent trust, conditional-only waivers, exact runtime/release subjects, offline operation, doctor, and fail-closed recovery without implying production authority or recommending policy weakening |
| `step_130` | complete | The agent guide defines authority and untrusted-data boundaries, prescribes bounded context/obligations/matrix/affected/check reads, preserves every non-success state, explains conservative inventory semantics, documents the default MCP read surface and paired invocation-only audited verify gate, confirms no waiver tool or prose authority exists, requires dry-run inspection and channel purity, and forbids unauthorized waiver/policy/trust/signing/evidence/publication mutation |
| `step_131` | complete | `SECURITY.md` states current-major support, a safe disclosure fallback without inventing a contact, and the deliberately unassigned security-owner production gate; the threat model distinguishes hostile and trusted inputs, maps traversal/symlink, argv/env, flood/timeout/redaction, signature/replay/substitution, immutable evidence, and MCP escalation to mandatory tests, and states limits around OS behavior, untrusted programs, declared-secret redaction, external trust/retention, completeness, application security, legal approval, and organizational ownership |
| `step_132` | complete | Bootstrap contribution prose is replaced with exact toolchain/router setup, repository boundaries, current-only/unsafe/generated-state rules, the real aggregate command and its gates, schema-generation ownership, exact dependency/lock expectations, commit/PR evidence, dual licensing, and release linkage; maintenance guidance defines shared semantic versioning, no-deprecation compatibility policy, synchronized generated artifacts, dependency/advisory/license/MSRV/SBOM review, candidate fuzz/scale/package gates, and explicit publication prerequisites, while README routes operators, agents, and reporters to the implemented guides |
| `step_133` | complete | No-compat enforcement scans text and binary content across production source, schemas, ordinary docs, examples, fixtures, workflows, packaging, fuzz/benchmark packages, and arbitrary produced-artifact roots while narrowly excluding only the authority contract, encoded scanner tokens, and controlled negative corpus from the ordinary repository pass; negative self-tests prove both repository-inclusive and explicit artifact-root scans reject the forbidden corpus, and dry-run packaging scans its exact staged contents before archive creation |
| `step_134` | in progress | A corrective completion audit withdrew the prior local acceptance claim after finding that several reported lanes used synthetic declarations instead of production entry points, the aggregate gate omitted mandatory lanes, and production limits did not fully match the normative security contract. The checkpoint remains active until every finding below is corrected and every mandatory lane is rerun. |

## Corrective Completion Audit

The following findings are part of the active Step 134 scope. Earlier owning
RCLDs remain historically complete, but their affected behavior must be
corrected and their downstream evidence regenerated before this ledger can
close:

- enforce the normative MCP frame limit and secure invocation correlation IDs;
- enforce the exact signed-CI trust profile at the verification boundary;
- replace declared release outcomes with exercised pass, fail, and unknown
  release-gate fixtures carrying runtime facts and immutable evidence;
- replace hand-authored command samples with byte-compared outputs from real
  CLI and MCP entry points;
- fuzz production TOML, protocol, graph, adapter, inventory, evidence, and
  canonicalization boundaries rather than generic parsers;
- add direct adversarial assertions for every security-matrix case, including
  stable diagnostics and secret non-disclosure;
- measure cold validation, warm context, affected analysis, and peak memory
  through production code at the specified 10,000-unit/100,000-requirement
  scale;
- make the final aggregate gate invoke every mandatory fixture, golden,
  property, fuzz, security, cross-platform, coverage/mutation, performance,
  dependency/license, package, CLI/MCP, and release lane;
- pin workflow dependencies immutably and regenerate package/SBOM/provenance
  evidence; and
- rerun requirement-by-requirement traceability and publish a replacement
  acceptance report containing only observed results.
