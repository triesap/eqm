# EQM V1 Decision Register

Status: normative

This register preserves the approved product and architecture decisions that
govern EQM v1. Later specifications may make a decision more precise, but may
not silently contradict it. A changed decision requires an ADR and an update
to this register.

## Identity And Repository Contract

| ID | Decision |
| --- | --- |
| EQM-001 | The product and executable identity is EquivalenceMatrix (`eqm`). |
| EQM-002 | V1 is a clean break and exposes no predecessor compatibility surface. |
| EQM-003 | Public schemas use the EQM namespace and current v1 identities. |
| EQM-004 | Cargo package, crate, and binary names follow the approved workspace mapping. |
| EQM-005 | Authored metadata and generated state occupy distinct trees. |
| EQM-006 | Workspace configuration is explicit and repository owned. |
| EQM-007 | The workspace lockfile is committed and verification uses it. |
| EQM-008 | Every schema has a stable EQM identity. |
| EQM-009 | Related schema versions advance as a coordinated contract set. |
| EQM-010 | Mixed incompatible schema versions fail closed. |

## Product And Domain Model

| ID | Decision |
| --- | --- |
| EQM-011 | EQM is a conformance and equivalence decision engine, not an application framework. |
| EQM-012 | Human operators, CI, release automation, and agents are supported consumers. |
| EQM-013 | Code generation, deployment, package installation, and self-update are non-goals. |
| EQM-014 | Equivalence is a declared, evidence-backed relation rather than textual similarity. |
| EQM-015 | Conformance is evaluated before equivalence. |
| EQM-016 | Capabilities, journeys, and surfaces are distinct domain entities. |
| EQM-017 | Contracts may be assembled from explicitly pinned fragments. |
| EQM-018 | A target identifies the subject being evaluated. |
| EQM-019 | A binding connects requirements to target-specific evidence mechanisms. |
| EQM-020 | Requirements have explicit applicability scopes. |
| EQM-021 | Variants are modeled explicitly rather than inferred from names. |
| EQM-022 | Entity lifecycle states and invariants are closed contracts. |
| EQM-023 | Public identifiers use one documented ASCII grammar. |
| EQM-024 | Relative identifiers and implicit aliases are not accepted. |
| EQM-025 | Prose is UTF-8 and normalized; machine identifiers remain ASCII. |
| EQM-026 | Repository paths use normalized repository-relative syntax. |
| EQM-027 | Symlinks, case collisions, and path collisions fail according to explicit policy. |
| EQM-028 | Authored TOML follows TOML 1.1 semantics. |
| EQM-029 | JSON schemas use draft 2020-12. |
| EQM-030 | Schemas reject unknown fields unless an extension point explicitly permits them. |

## Discovery And Canonical Data

| ID | Decision |
| --- | --- |
| EQM-031 | Source discovery order and duplicate-authority handling are deterministic. |
| EQM-032 | Imports are pinned to immutable identity and digest. |
| EQM-033 | Authored authority cannot interpolate environment variables or secrets. |
| EQM-034 | Time uses UTC, dates are explicit, and normative numeric data excludes floating point. |
| EQM-035 | Diagnostics preserve deterministic source spans without making formatting normative. |
| EQM-036 | Semantic identity is SHA-256 over an RFC 8785 canonical JSON projection. |

## Policy, Evidence, And Equivalence

| ID | Decision |
| --- | --- |
| EQM-037 | Conformance levels form a closed ordered set. |
| EQM-038 | Risk is represented explicitly and influences required evidence. |
| EQM-039 | Applicability has a finite truth model with fail-closed unknown handling. |
| EQM-040 | Contract revision and semantic digest are separate identities. |
| EQM-041 | Fragment references include immutable pins. |
| EQM-042 | Profiles select policy without rewriting contract meaning. |
| EQM-043 | Authorities are explicit and cannot be inferred from file precedence. |
| EQM-044 | Policy composition is monotonic: added policy cannot weaken an obligation. |
| EQM-045 | No unrestricted policy language is embedded in v1. |
| EQM-046 | Requirements use closed, typed facets. |
| EQM-047 | Evaluation statuses form a closed set. |
| EQM-048 | Waiver and manual-review outcomes are distinct from satisfaction. |
| EQM-049 | Evidence observations are stored separately from evaluation conclusions. |
| EQM-050 | Evidence kinds and their validation rules are explicit. |
| EQM-051 | Evidence maps explicitly to requirements and subjects. |
| EQM-052 | Count and quorum rules are deterministic. |
| EQM-053 | Conflicting retry outcomes produce an unstable result, never silent success. |
| EQM-054 | Evidence subject identity must exactly match the evaluated subject. |
| EQM-055 | Freshness is evaluated from explicit timestamps and policy keys. |
| EQM-056 | Trust decisions and caches are scoped by semantic and trust inputs. |
| EQM-057 | Expired evidence cannot satisfy an obligation. |
| EQM-058 | Raw evidence, normalized results, and conclusions have separate storage contracts. |
| EQM-059 | Attestations bind subject, policy, evidence set, result, and time. |
| EQM-060 | External results are untrusted until their envelope and provenance validate. |

## Runners, Adapters, And Exposure

| ID | Decision |
| --- | --- |
| EQM-061 | Bindings do not contain executable shell command strings. |
| EQM-062 | Runner definitions are protected authority. |
| EQM-063 | Process invocation is represented as typed executable and argument vectors. |
| EQM-064 | Runner backends expose declared capabilities. |
| EQM-065 | The local backend does not claim sandbox guarantees it cannot enforce. |
| EQM-066 | Security guarantees are reported only when technically enforceable. |
| EQM-067 | Environment, secrets, and working directory are explicit runner inputs. |
| EQM-068 | Time, output, memory, and concurrency bounds are explicit. |
| EQM-069 | Every backend returns one normalized result envelope. |
| EQM-070 | Framework adapters execute out of process. |
| EQM-071 | Adapter identities and implementations are pinned. |
| EQM-072 | Discovery modes are explicit and deterministic. |
| EQM-073 | Framework defaults are data-backed rather than hidden heuristics. |
| EQM-074 | Adapter failure is unknown/error and never triggers implicit download. |
| EQM-075 | Exposure is evaluated across independent declared dimensions. |
| EQM-076 | Discovery and evidence must not collect unnecessary personal data. |
| EQM-077 | Normalized facts are provider-neutral. |
| EQM-078 | Fact freshness is explicit and independently evaluated. |
| EQM-079 | Target, environment, build, and release identities remain distinct. |
| EQM-080 | Release identity is immutable and content bound. |
| EQM-081 | Independent exposure facts remain independently auditable. |
| EQM-082 | Release gates consume finalized conformance and equivalence results. |
| EQM-083 | Diff uses an exact identified baseline. |
| EQM-084 | Affected-set computation operates on finalized graph dependencies. |
| EQM-085 | Content-addressed caches include every semantic input. |
| EQM-086 | Federation is local or exact-pinned; mutable remote authority is rejected. |

## CLI, Protocol, And Agent Access

| ID | Decision |
| --- | --- |
| EQM-087 | The v1 command surface is closed and documented. |
| EQM-088 | Mutating commands are explicitly identified and require deliberate invocation. |
| EQM-089 | Global options have consistent precedence and scope. |
| EQM-090 | Human, JSON, and SARIF output modes have distinct contracts. |
| EQM-091 | Machine-readable stdout contains only the selected protocol payload. |
| EQM-092 | JSON results use one versioned envelope. |
| EQM-093 | Exit codes are closed and semantically allocated. |
| EQM-094 | Ordering and color behavior are deterministic. |
| EQM-095 | SARIF represents actionable findings and preserves stable identities. |
| EQM-096 | Invocation context is explicit in protocol output. |
| EQM-097 | CLI and protocol inputs enforce documented resource bounds. |
| EQM-098 | Untrusted content is data and cannot inject commands or prompts. |
| EQM-099 | Agent access cannot exceed the authority of the underlying EQM operation. |
| EQM-100 | MCP is added only after the JSON protocol contract is stable. |
| EQM-101 | MCP transport and tool names are versioned and closed. |
| EQM-102 | MCP diagnostics, logging, and audit output do not corrupt protocol streams. |

## Implementation And Delivery

| ID | Decision |
| --- | --- |
| EQM-103 | EQM v1 is implemented in Rust. |
| EQM-104 | The stable Rust toolchain is pinned. |
| EQM-105 | The workspace uses explicit members and resolver 3. |
| EQM-106 | Crate dependency direction follows the approved acyclic graph. |
| EQM-107 | Only intended public crates are publishable. |
| EQM-108 | First-party code forbids unsafe code and production panic shortcuts. |
| EQM-109 | Public identifiers use newtypes and errors preserve typed diagnostic context. |
| EQM-110 | Async execution is confined to I/O boundaries that require it. |
| EQM-111 | Domain types remain separate from protocol DTOs. |
| EQM-112 | Deterministic collections and explicit types are preferred in normative paths. |
| EQM-113 | Formatting, lint, documentation, and warning gates are mandatory. |
| EQM-114 | Dependencies are minimized, pinned through the lockfile, and policy checked. |
| EQM-115 | Time, randomness, filesystem, and process effects are injectable at test seams. |
| EQM-116 | Supported platforms and release artifacts are explicit. |

## Security, Quality, And Governance

| ID | Decision |
| --- | --- |
| EQM-117 | The repository maintains an explicit threat model. |
| EQM-118 | Protected baselines and authorities fail closed when trust is absent. |
| EQM-119 | All external inputs have size, depth, count, and time limits. |
| EQM-120 | Command and prompt injection are treated as hostile-input risks. |
| EQM-121 | Remote acquisition is disabled unless an explicit pinned workflow permits it. |
| EQM-122 | Privacy and redaction are protocol requirements. |
| EQM-123 | Dependency security and license policy are automated gates. |
| EQM-124 | Release artifacts carry checksums, provenance, and supply-chain attestations. |
| EQM-125 | Vulnerability reporting follows a published disclosure policy. |
| EQM-126 | Licensing and third-party notices are release requirements. |
| EQM-127 | Tests are layered by crate, contract, integration, and end-to-end behavior. |
| EQM-128 | Parsers, schemas, and canonicalization use positive and negative fixtures. |
| EQM-129 | Stable public protocol output uses reviewed golden fixtures. |
| EQM-130 | Algebraic and ordering invariants use property tests. |
| EQM-131 | Adversarial fixtures cover malformed, oversized, ambiguous, and hostile input. |
| EQM-132 | Supported framework adapters have representative fixtures. |
| EQM-133 | Core logic targets 90% line, 85% branch, and 80% mutation coverage. |
| EQM-134 | The accepted test suite has zero known flaky tests. |
| EQM-135 | Performance budgets and benchmark datasets are versioned. |
| EQM-136 | CI and release gates are repository owned and reproducible. |
| EQM-137 | Releases follow semantic versioning and never self-update. |
| EQM-138 | Documentation, decisions, and governance evolve with the implementation. |
| EQM-139 | Adoption is validated through named pilot integrations before broad claims. |
| EQM-140 | Outcome metrics and the integration registry are reviewed after three pilots. |

## Change Control

The machine-readable traceability index introduced by the authority-validation
checkpoint must map every decision above to its governing specification and
planned verification. Missing, duplicate, or unreferenced decision IDs are a
validation failure.
