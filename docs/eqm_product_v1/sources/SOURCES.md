# Sources and evidence scope

Prepared 2026-09-26. Durable product intent comes from the user's request, approval of the original architecture direction, approval of the pressure-tested refinements, and this explicit final-package request. This document preserves those decisions through requirement/acceptance/ADR records; original chat access is not needed.

## Repository evidence

Pinned baseline `7c8293837c09c627489cdf244a2fbd97b60aa8ed`, branch master, commit dated 2026-08-09T17:30:53Z. Rechecked head/recent conventions while preparing the handoff. Repository references in repo/BASELINE.md and repo/REVIEW_FINDINGS.md are pinned to this commit. They are primary code evidence for the static observations, not proof of executed tests. Current repository code wins on factual layout/symbol discovery; approved specs govern desired intent. Record meaningful drift before implementation.

- [Repository head/recent commits](https://api.github.com/repos/triesap/eqm/commits?sha=master&per_page=6)
- [Pinned commit](https://github.com/triesap/eqm/commit/7c8293837c09c627489cdf244a2fbd97b60aa8ed)
- [Contributor contract](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/CONTRIBUTING.md)
- [Repository instructions](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/AGENTS.md)
- [Workspace](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/Cargo.toml)
- [Toolchain](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/rust-toolchain.toml)
- [CLI package](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/Cargo.toml)
- [Xtask manifest](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/tools/xtask/Cargo.toml)
- [Xtask gates](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/tools/xtask/src/main.rs)
- [Agent context](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/docs/agent-context.md)

## External primary references

These were checked during handoff preparation. Recheck current dependencies/tool APIs before implementation. The package includes the needed product constraints; external links provide attribution and technical detail, not missing product intent. No third-party full articles, source checkout, credentials or font files are embedded.

### E01 — OpenAI: Harness engineering

Source: https://openai.com/index/harness-engineering/

Layered repository knowledge with concise routing instructions rather than one enormous agent instruction file. This package follows that organizational approach; it does not require a specific Codex model/version.

### E02 — OpenAI: Unrolling the Codex agent loop

Source: https://openai.com/index/unrolling-the-codex-agent-loop/

Official context on repository AGENTS instruction loading and tool permission boundaries. The handoff is a user-supplied implementation contract, not a change to a coding agent sandbox.

### E03 — RFC 8785: JSON Canonicalization Scheme

Source: https://www.rfc-editor.org/rfc/rfc8785.html

Primary canonical JSON reference: duplicate member rejection, numeric representation constraints, deterministic serialization and preservation of Unicode string data. Exact EQM projections remain separately specified.

### E04 — SQLite: Write-Ahead Logging

Source: https://www.sqlite.org/wal.html

Primary local-host WAL/one-writer/checkpoint guidance and WAL-reset fix disclosure. Use a currently audited runtime containing the fix, not an arbitrary system library or a permanently pinned historical fix version.

### E05 — UniFFI: Integrating with Gradle

Source: https://mozilla.github.io/uniffi-rs/latest/kotlin/gradle.html

Documents the Kotlin backend JNA dependency. This is not evidence that its generated Kotlin is a universal Kotlin/Native bridge.

### E06 — Kotlin: Interoperability with C

Source: https://kotlinlang.org/docs/native-c-interop.html

Primary reference for Kotlin/Native C interoperability. A selected optional bridge still requires explicit ownership, compatibility and platform testing.

### E07 — Rust: catch_unwind

Source: https://doc.rust-lang.org/std/panic/fn.catch_unwind.html

Primary boundary on unwinding versus aborting panic behavior. Catching a panic is not process-level crash isolation.

### E08 — Microsoft: Job Objects

Source: https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects

Primary Windows process group/resource management reference. Actual Job Object configuration and hostile-worker containment remain implementation and host-validation responsibilities.

### E09 — SLSA v1.2: Verifying artifacts

Source: https://slsa.dev/spec/v1.2/verifying-artifacts

Verification checks signatures, builder identity and expected build inputs/subjects rather than trusting a signature alone. No SLSA level certification is claimed for EQM.

### E10 — Sigstore: Verifying signatures

Source: https://docs.sigstore.dev/cosign/verifying/verify/

Primary identity/issuer/subject verification guidance and bundle usage. The approved direction requires exact expectations, not disabling claim checks or granting unknown signers trust.

### E11 — MCP: Security best practices

Source: https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices

Primary security guidance for MCP/local-server boundaries. EQM remains read-only by default and does not add an unauthenticated localhost service.
