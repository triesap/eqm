# Supervisor review at 3da0b50

Reviewed range: `e61a28e4047b6ba9be87f0afc5646c055cf22317` through
`3da0b50aeb7cf7f8b23f8f5ebd47f043892ed00b`. Six commits changed 31 files.
The review inspected all changed source, tests, contracts, ledger changes and
the corresponding execution logs. Findings below are source-review findings;
no new Rust reproduction program was written by the supervisor.

**Disposition: S015 requires corrections; S016 must not begin yet.** Fourteen
numbered steps remain recorded complete. The S015 implementation commit exists,
but its full acceptance has not been achieved. See DEV-002 and the corrective
checkpoint schedule in [COMMIT_SEQUENCE](../COMMIT_SEQUENCE.md).

## Independent verification

- Standard `cargo xtask check`: passed at the reviewed HEAD.
- `cargo test --locked -p eqm_application --doc`: passed, including the new
  negative compile test. The standard all-targets test lane alone does not run
  that doctest.
- The submitted correction/S015 logs record successful standard checks. Log
  review found no failed test result concealed as a pass, but did not establish
  the claim that every correction had an observed red-before-green test run.
- No extended security/coverage/mutation/fuzz/benchmark/package, Linux-native,
  live product or operational acceptance is established by these checks.
- The initial worktree was clean. Subsequent supervisor edits are planning and
  review documents only; implementation remains with the implementation agent.

## Findings and required disposition

| ID | Priority | Evidence and consequence | Owning correction |
| --- | --- | --- | --- |
| RF01 | P1 | S015 adds `exact_unsigned` to v1 counts (`eqm_protocol/src/evidence.rs`, `validate_counts`) and `exact_signed` to v1 runtime/inventory facts (`records.rs`, `eqm_runner/src/inventory.rs`). Previously accepted integers now reject while v1 schemas are unchanged. Preserving schema bytes alone does not preserve the contract. | S015-R1 restores v1 acceptance; exact v2 rules apply only at explicit v2 boundaries. |
| RF02 | P1 | `eqm_protocol/src/wire.rs` adds standalone types, but `SchemaVersion` and generators still know only v1. There are no v2 record DTOs or generated schemas. S015 explicitly owns numeric/time/null/absence DTO validation and schema updates; null/absence was incorrectly deferred to S016. | S015-R2/R3 establish the concrete v2 layer and complete S015. |
| RF03 | P1 | `wire.rs::parse_signed_decimal` parses the unsigned magnitude into `i64` before negation. Therefore `"-9223372036854775808"` rejects even though `WireInt::new(i64::MIN)` serializes to that string. Round-trip closure fails for a valid signed endpoint. | S015-R1, full endpoint and one-past tests. |
| RF04 | P1 | Both decimal-string visitors accept small values such as `"5"`, `"0"`, or `"-5"`; serialization changes them to JSON numbers. This violates the frozen single wire form for each integer value. | S015-R1 rejects strings within the safe numeric interval. |
| RF05 | P2 | `signed_schema` permits `"-0"`; both string schemas admit arbitrarily large magnitudes and small-string alternatives rejected by the decided canonical contract. A syntactically accepted schema does not establish runtime conformance. | S015-R1/R3 synchronize representational schema constraints and parser behavior; test both on the same boundary corpus. |
| RF06 | P1 | S015 claims canonical time was already fully enforced. `UtcInstant::from_str` only parses with Chrono then checks round-trip spelling. Chrono supports leap-second notation and its formatter preserves it; there is no explicit leap-second rejection. The timestamp corpus omits that case. The v2 ADR rejects leap seconds. | S015-R3 adds a v2-specific validated instant and leap-second/year/precision tests without rewriting v1 behavior. |
| RF07 | P1 | `release_check::confined_bytes` admits 16 MiB, but the new `read_release_record` calls default `strict_from_slice` and `parse_strict`, each capped at 8 MiB. A valid release record padded with JSON whitespace between 8 and 16 MiB now fails. CR01 was repaired for adapters but repeated at this new ingress. | S015-R4 uses the established explicit limit at both parses (or one strict parse reused), with valid exact-size controls. |
| RF08 | P2 | New `eqm_application::evaluation::read_evidence` forwards a provider's vector without checking `request.limit()`. An over-returning provider can violate the advertised bounded application result; `LimitExceeded` is never emitted there. | S015-R4 rejects an over-limit result without truncation. Provider-side allocation bounds remain a separate concrete-store obligation. |
| RF09 | P2 | Numeric vectors use a hardcoded ID allow-list and `checked >= 9`, so newly added numeric vectors can be silently skipped. Domain digest vectors parse a precomputed string rather than test domain hashing. Reconcile tests stop at DTO parsing; the two old invalid EvidenceResult tests remain. Clock call-site tests invoke only context helpers. Read-view tests establish a narrow interface, not audited production absence of network effects. | S015-R4/R5 closes applicable ingestion/application tests and exact vector dispatch; S016/S027 and later integration steps own remaining semantic coverage. |
| RF10 | P2 | STATUS.md still says S002 is next and S002–S211 pending; progress claims S015 complete; CR08 and S015 SHAs remain empty; the S015 report says safe to advance despite missing explicit scope. | Supervisor reconciles planning now; S015-R5 records actual implementation closure and truthful remaining acceptance. |

## Review of prior corrections

- CR01: adapter/request limits are repaired; release-record coverage is still
  incomplete (RF07). Mark partial, not universally closed.
- CR02: actual inventory and release command ingress now call strict parsing.
  The valid DTO duplicate-map control is useful. A real locked-adapter reconcile
  test remains required; no claim of complete RA13 relationship closure.
- CR03: the AutoSi decision, signed/unsigned distinction and authored/opaque
  distinction are clearer. Concrete timestamp examples execute. Missing
  endpoint/negative/canonical-form vectors and incomplete v2 integration remain.
- CR04/CR05/CR07: reviewed fixes are accepted for subsecond preservation,
  private validated request limits and retained standard error chaining.
  This does not accept later freshness arithmetic or all CLI integration.
- CR06: the restricted view is an improvement; its compile-fail doctest passes
  independently. The new bounded helper needs RF08 and production integration
  remains pending. A Rust trait's implementation can still perform I/O; lacking
  an executor getter alone is not a proof of absence of network activity.
- CR08: the JSON resume cursor was improved, but document reconciliation was
  incomplete and the S015 completion claim is premature.

The duplicate-error marker is confined to the strict visitor and is not used
to grant authority. No exploitable marker bypass was established. Preserve
escaped-name/deep-input tests; do not rewrite the parser solely on speculation.
The scoped internal dependency change is one test-only `serde_json` edge in
this reviewed range; no new external package version was added.

## Supervisor decisions

The owner delegates coding-period scope and representation decisions to the
supervisor. DEV-002 and COMMIT_SEQUENCE now decide the compatibility stance,
v2 schema allocation, repair order and admission criteria. The implementation
agent implements these decisions and reports new decision requirements; it
does not defer owned work, weaken acceptance or change scope on its own.

Remaining numbered work is S015 correction plus S016–S211. Continue all
dependency-unblocked approved work; S143 is a checkpoint, not the next run's
mandatory stopping point. Concrete external prerequisites still block their
first dependent step. Do not manufacture identities, product requirements,
policy values, host evidence or operational authorization to advance.

## Release testing disposition

This is not a release candidate or a point requiring human release testing.
The evaluator, store, acquisition, protected trust, frozen decisions and public
adapter integration are mostly still pending. Human smoke testing cannot
substitute for the missing implementation.

Once S191–S209 prerequisites and relevant implementation exist:

1. Identify the exact candidate commit, package checksums and supported host/
   product cohorts. Run standard, extended, schema, compatibility, migration,
   cryptographic fixture and native-host gates; record every unavailable lane.
2. Install the candidate in a disposable consumer workspace using synthetic
   data. Run doctor/validate and inspect scope, obligations, matrix, explanations
   and MCP views; verify reads cause no acquisition or mutation.
3. Review an exact dry-run execution plan, then run only authorized fixture
   acquisition. Verify immutable evidence, receipt, evaluation, query/replay,
   CLI/MCP agreement and restart behavior.
4. Mutate missing/stale/wrong-subject/tampered/duplicate/conflicting evidence,
   permissions and resource limits. Verify explicit non-success with useful
   diagnostics; test interruption/reopen and error output paths.
5. Run the owner-defined Rust/KMP/provider lifecycle and mixed-version scenarios
   on actual supported hosts/devices, including offline/restart/retry, account
   boundaries and permission changes. Capture exact expected/actual results.
6. Independently review shadow release decisions, signatures and exact
   authorization consumption in fixtures. Keep production enforcement inactive
   until separately authorized; test installation is not publication approval.

Any newly required product or operational fact returns to the supervisor for
discovery/decision. The supervisor cannot invent an external fact or grant
authority the owner has not supplied.
