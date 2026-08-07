# EquivalenceMatrix V1 Architecture

## Crate Graph

```text
eqm_domain

eqm_manifest -> eqm_domain
eqm_engine   -> eqm_domain
eqm_protocol -> eqm_domain
eqm_runner   -> eqm_domain + eqm_protocol

eqm_mcp      -> eqm_engine + eqm_protocol + eqm_runner
eqm_cli      -> all production crates
```

## Responsibilities

- `eqm_domain`: pure validated semantic types and graph containers; no I/O,
  process, network, Git, terminal, CLI, or MCP dependency.
- `eqm_manifest`: authored TOML DTOs, parsing, source spans, discovery, strict
  validation, formatting, domain conversion, and canonical projection.
- `eqm_engine`: pure resolution, invariants, fragment expansion, policy,
  obligations, evidence, freshness, waivers, conformance, equivalence,
  exposure, release, diff, affected analysis, matrices, and explanations.
- `eqm_protocol`: versioned public JSON, SARIF, adapter, evidence, attestation,
  report, and MCP DTOs.
- `eqm_runner`: the only process-launching crate; approved runner backends,
  adapters, bounds, cancellation, normalization, and immutable results.
- `eqm_mcp`: a thin current-version stdio adapter over prepared session and
  engine behavior.
- `eqm_cli`: arguments, orchestration, rendering, machine-output discipline,
  and exit codes.
- `eqm_test_support`: unpublished shared fixtures and test utilities.

## Evaluation Flow

```text
TOML + lock + inventories + facts + evidence + release records
-> manifest parsing and strict validation
-> validated domain inputs
-> graph resolution
-> fragment expansion and invariant validation
-> finalized graph canonicalization and digest
-> policy-relative engine evaluation
-> protocol reports
-> CLI or MCP rendering
```

The caller supplies an exact baseline, clock, subject, trust configuration,
runtime facts, and evidence. The engine never acquires them.

## Determinism

- Exactly one workspace config exists inside the VCS boundary unless an
  explicit config is supplied.
- Source discovery and public output use specified stable ordering.
- Duplicate authorities are errors.
- Defaults are explicit before canonicalization.
- The normative graph projection uses RFC 8785 JSON Canonicalization Scheme and
  SHA-256.
- Source spans, comments, formatting, notes, and display-only links do not
  affect normative digests.
- Domain objects and public DTOs remain separate.

## Execution And Trust Boundaries

- Bindings reference approved runner IDs; they contain no commands.
- Runners use argv arrays, typed placeholders, confined cwd, allowlisted
  environment, timeouts, output caps, and cancellation.
- Local process execution does not claim generic sandboxing.
- Container guarantees are accepted only when enforced.
- Adapters are out of process and digest-pinned in `eqm.lock`.
- Normal validation/checking is offline; explicit lock update is the remote
  acquisition boundary.
- Candidate policy cannot self-certify weakening against protected authority.

## API Boundary

The coordinated v1 crates accept only current v1 schemas and protocols. There
are no public Cargo features in v1, no release Git dependencies, and no hidden
compatibility surfaces.
