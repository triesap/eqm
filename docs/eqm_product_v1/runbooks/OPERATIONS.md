# Operational ownership and observability

Status: approved procedure requirements; implementation/real owners and environment values must be recorded before activation. This runbook does not grant access to production systems.

## Ownership

Assign actual accountable people/teams for semantic contracts and policy changes; trust identities/roots/signing; worker isolation and host capabilities; object/index storage and recovery; Rust/KMP platform support; release enforcement and rollback. Record reviewer/approver separation appropriate to the operator's governance. The conversation supplies roles, not names or a new mandatory organizational hierarchy.

## Deployment checklist

Record the exact evaluator artifact, configuration/authority digests, audited dependencies and linked SQLite/verifier versions, supported hosts/filesystems, native/product test evidence, resource budgets, retention classes and protected store location. Ensure workers cannot mutate authoritative state and do not receive unneeded secrets. Declare unsupported capabilities rather than defaulting to weaker behavior.

## Operational signals

Expose safe structured diagnostics for preparation/evaluation failures, missing/unknown coverage, rejected verification dimensions, acquisition terminal outcomes, queue saturation, object/index publication failure, corruption/recovery, GC leases and current-acceptance invalidation. Bind observations to evaluation/snapshot/plan identities. Logs must not carry secret values or raw sensitive business media. Metrics are proposed observability categories, not an invented hosted monitoring dependency or numeric SLO.

## Incident handling

Freeze affected promotion or execution authority before investigation when trust/correctness is uncertain. Preserve exact decision/input/signature/evaluator/authority references and relevant bounded logs. Determine whether failure affects historical evidence integrity, current eligibility or host acquisition safety. Restore only through the recorded trusted process. Do not turn unknown into pass to clear an outage.

Use STORAGE_RECOVERY.md for publication/index/corruption incidents, TRUST_OPERATIONS.md for trust rotation/revocation, and RELEASE_OPERATIONS.md for promotion/rollback. Reproduce the issue in disposable fixtures, add a regression and use the same one-step review/verification rules for fixes.

## Support claims

Cross-compilation, a benchmark constant, test-only signer, or a copyable fixture does not establish native host behavior or real product assurance. Maintain separate code/local/platform/operational readiness states and exact unrun lanes. Distribute unsigned local packages honestly unless signing/publication is explicitly authorized.
