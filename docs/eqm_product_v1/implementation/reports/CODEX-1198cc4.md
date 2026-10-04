# Independent supervisor review — 1198cc4

Reviewed clean `1198cc4c3dfe6fea6443e0f4b1998a539d564d20` on 2026-10-04.
Exact issued instruction matched one native implementation branch, including
actual repository use and runtime selection. All four requested checkpoints
P03-R11 `c09fe770`, P03-R12 `3aa5ab23`, P03-R13 `a135e637`, S043-R5 `1198cc4`
were committed and retained; Pi correctly stopped at mandatory independent review.

RF66 and RF67 are accepted for their allocated repairs. Fragment prefix now uses
LocalRequirementId; ordered journey cardinality/uniqueness and surface alternatives
match owning constructors. Additional local combinations and the existing release
selector variant are faithful to existing domain behavior. An isolated complete
196-combination evidence kind/runner/selector/count matrix matches the owning
EvidenceSpecification constructor. Real loader, preparation, CLI controls and
finalized positive graph regressions remain green. Preserve all42 numbered prior
acceptances; S043/P03 still changes-required, RF45 partial, whole RA02/RA13 open.

## RF68 — PositiveDays wire bound exceeds owning representation (P2)

`WaiverPolicyV2Dto.maximum_days` in
`crates/eqm_protocol/src/schema_graph_v2.rs` uses generic positive u64 schema.
`eqm_domain::PositiveDays` in `crates/eqm_domain/src/policy.rs` is NonZeroU32.
Isolated same-input controls at1 and4294967295 pass both layers; literal4294967296
passes real graph schema but cannot convert to the owning PositiveDays and rejects
owning WaiverPolicy construction. This is a schema-expressible numeric bound,
not Unicode, a relationship exception or an external product threshold.
Audit every numeric member against actual owning type width/range/normal form.
Preserve v2 safe-number/decimal-string forms and original v1 bytes.

## RF69 — Owning forbidden control characters absent from text schemas (P2)

A literal title containing U+0000 passes real graph schema while Title::new rejects
it. Genuine normal title, NFC non-ASCII, LF and TAB positives remain accepted.
`entity.rs::normalized_text` rejects control characters except LF/TAB; the title,
selector/route/description/statement/trigger/reason helpers currently enforce
length only. The forbidden-control subset is faithfully schema-expressible.
Audit actual owning text types and enforce their existing control policy without
applying it to opaque observed text, raw bytes, diagnostics or extension strings
that have a different contract. NFC and UTF-8-byte bounds remain explicit layers.
No domain loosening, normalization of input or new product text policy.

## Verification and historical evidence

Independent `cargo xtask check`: exit0,771 successful test executions. Owning
five-package doctests: exit0,9. Structure validator and diff checks pass.
Three isolated tests pass: reproduce RF68/RF69, compare196 evidence combinations,
and retain valid identifier/end-control and LF/TAB/non-ASCII controls. Production
checkout source/tests remain unchanged by Codex; isolated probes retained externally.
v1 generated schemas and Cargo.lock unchanged across this batch.

Native results reveal intermediate corpus failures, seven expect-used Clippy
errors and a formatting-check failure, repaired before the corresponding commits.
The reports omitted some failed attempts; supervisor historical annotations now
preserve that fact. Some temporary logs were overwritten on retries, so native
JSONL remains the source for those failed results. Surviving logs are retained in
an external review archive. Final-source verification is independent evidence,
not proof that every historical command passed. Next batch must retain distinct
attempt files and report actual exits without replacing failures with green runs.

Full verify, Linux, coverage/mutation/fuzz/benchmark/package and live product/
crypto/host/signing/operational acceptance remain unverified. Their absence is not
inherently hardware-blocked. No release or human release-testing gate reached;
no publication/deployment/signing/activation authority. Sole DEV-026 defines next
substantial scalar audit, differential corpus and consumer exit before S044.
