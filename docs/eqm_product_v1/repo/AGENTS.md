# EQM implementation guidance — merge template

This file is a proposed supplement for the repository's actual AGENTS.md, not permission to overwrite it or existing nested instructions. Resolve the approved governance/version/unsafe-boundary changes explicitly in the plan.

Read `docs/eqm_product_v1/README.md`, then the step-referenced durable specs. Treat `docs/eqm_product_v1/implementation/COMMIT_SEQUENCE.md` as an ordered executable plan, not a replacement for product intent. Use the pending-step ledger and Git history to resume. Keep this instruction file a short map, not the full specification.

Preserve EQM's role as assurance infrastructure for a Rust CLI, KMP desktop control tower, KMP mobile edge app, and relevant providers. Do not implement product accounts/tasks/media/sync or embed EQM on mobile by default. No execution, signing, waiver mutation, release authorization, publication or deployment from a read-only request.

Keep domain/engine pure and unsafe-free; centralize semantics behind the shared application layer. All report adapters consume one immutable evaluated snapshot. Complete scope includes unknown/provider/end-to-end cases. Claims, JSON deserialization, database lookups and imported verified flags do not grant trust. Unknown/missing/stale/unstable/failed/waived/untrusted is not satisfied evidence.

One plan step per independently tested/reviewed commit. Do not skip/merge/reorder/broaden; document repository-proven obsolete/unsafe deviations before proceeding. Preserve unrelated work; never reset/stash/force-push it. Use `type: imperative subject` messages and report exact commands, results, unverified lanes and next-step safety after each commit.

Run `cargo xtask check` as the standard gate and `cargo xtask verify` for release candidates, plus step-specific and declared native/feature lanes. The CLI package is `eqm`, not `eqm_cli`. Generate schemas from DTOs with `cargo xtask schemas generate`; never hand-edit generated JSON. Exact dependency changes must update Cargo.lock intentionally and pass audits. Do not remove --locked or downgrade tools to conceal failures.

Keep authored consumer metadata in eqm.toml/eqm.lock/eqm and generated local state in ignored .eqm. Store trusted release inputs outside candidate write access. Never commit credentials, live media, private operational values, generated build output, or fabricated passing evidence.

Treat new symbol/module names in the handoff as proposed allocations until verified. Discover actual Cargo/Gradle/Xcode tasks before using them. Unsupported platforms, missing real product repos, unavailable credentials and unresolved policy values remain explicit gates, not guessed defaults. Synthetic fixtures cannot prove live product readiness.
