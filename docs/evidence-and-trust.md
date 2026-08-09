# Evidence and trust

## Evidence flow

Policy derives exact obligations. Bindings select evidence producers. Runners
produce normalized results. EQM then checks coordinate identity, result shape,
counts, freshness, retries, trust, and conflicts before evidence can satisfy an
obligation.

```text
runner output -> normalized evidence -> identity/freshness/trust checks
                                      -> obligation status
```

Pass, failure, skip, filter, quarantine, retry, timeout, and malformed output
remain distinct. A pass count below policy minimum is missing. Conflicting
records, unstable retry history, expired evidence, or unverifiable identity do
not become success.

## Runner safety

Runner programs and adapters are exact local pins. EQM launches direct argv,
not a shell, and validates typed substitutions as single arguments. Execution
uses a confined working directory, an allowlisted environment, timeout,
maximum output bytes, and bounded concurrency. Symlink or path escape,
inherited environment, output flood, timeout, and cancellation fail terminally.
Declared secret values are redacted from retained output.

Always inspect `eqm verify --dry-run` before authorizing execution. A plan
should name every selector, target, executable, argv element, environment input,
limit, and output destination.

## Effective trust

Trust is computed from independent authority. A producer claim is only a claim;
it cannot raise effective trust. Imported CI evidence must bind the exact
repository, target, source commit, build, artifact, profile, schema, and digest,
and its configured signature profile must verify. Replay, subject substitution,
signature tampering, and immutable-result collision are rejected.

Inventory completeness is equally important. Only complete, current, trusted
inventory can prove that a product unit is absent. Partial or missing inventory
produces unknown.

## Generated state

Generated consumer state lives under ignored `.eqm/`. Evidence results are
immutable and collision-safe: writing the same identity and bytes is
idempotent; writing different bytes to an existing identity fails. Authored
contracts and generated evidence never share a directory.

## Waivers

A waiver is protected authored authority, not evidence. It requires explicit
scope, approvers, controls, reason, start, and expiry. It is visible in results
and may make a policy outcome conditional, but it never turns missing or failed
evidence into satisfied evidence. Automation and agents must not create,
broaden, renew, or reinterpret waivers without explicit human authority.

## Release records and attestations

A release record binds application version, build number, channel, producer,
target, repository identity, source commit, artifact digest, release time, and
claimed trust. Runtime facts and all evidence must bind the same exact subject.
The release gate reports pass, fail, conditional, or unknown based on explicit
preconditions; unknown is never pass.

`eqm attest` emits an in-toto statement binding the workspace, policy,
profiles, evidence, runtime facts, release record, and evaluation subject.
Unsigned output says it is unsigned. Signing requires an explicitly selected
signer and external key governance; EQM does not invent identity, approval, or
publication authority.
