# Acquisition, filesystem, process and transport security

## Explicit authorization (R30–R34)

A dry-run resolves every selected executable and digest, typed argv, selector/case selection, cwd, explicit environment/secret references, backend capabilities, time/output/resource/concurrency limits and output destination. It performs bounded approved reads but no execution, including no executable discovery. A plan identity binds subsequent approval. Any meaningful plan change invalidates approval; secret values never appear in the plan.

Execute direct argv, not shell command strings or ambiguous command interpolation. A repository-owned script can still execute arbitrary code; direct argv is not a sandbox. Execution authority is provided by the operator/host, not synthesized from candidate declarations of enforceable guarantees. Validate actual backend capabilities and reject unsupported guarantees.

Worker mounts/permissions prevent access to evaluator/authority mutation, durable evidence database writes, signing credentials and unrelated product/user data. Provide a dedicated output/staging area. Discovery that executes Gradle or another program is subject to the same rules. Hostile contribution builds may need disposable isolated workers; process groups and ordinary containers do not automatically establish every threat-model guarantee. Document supported isolation rather than pretending all hosts are equivalent.

## Complete process supervision (R32)

The deadline covers spawn, execution, descendants, stdout/stderr drainage, result-file acquisition and cleanup. Immediate-child exit is not the end of supervision. Readers signal overflow promptly; stop workers on overflow/cancellation/deadline and drain only within bounded shutdown. Reap owned children and clean staged outputs on every error path. Signal child trees using platform-appropriate ownership (including Windows Job Objects when Windows is supported), not only the direct child. Detached/escaped processes require explicit host containment rather than optimistic cleanup claims.

Define terminal precedence for simultaneous overflow/timeout/cancellation and retain all reasons. Success requires valid terminal execution and normalized output, not just exit zero. Persist failure/timeout/cancel receipts where safely possible so required work does not vanish. A result already atomically committed before cancellation remains discoverable; cancellation must not rewrite committed success or create a phantom completed record. Exact API cancellation semantics are frozen in tests before exposure.

## Filesystem and byte boundaries (R33)

Perform confinement through safe opened handles/capability boundaries where available; lexical checks and final-component symlink checks alone are insufficient. Validate ancestors, final type, containment and limits. Address TOCTOU by retaining validated handles rather than validate-then-reopen where possible. Trusted execution uses immutable materialized inputs. Tests must cover parent symlinks, replacement races, path aliases/collisions and output files changed after child exit.

Enforce size before unbounded allocation. A result-file cap applies at read time, not only in a parser after fs::read. Bound existing collision files, imported evidence, attachments and logs. Validate digest and regular-file requirements after execution, do not assume a pre-created temp filename remained safe. Independently observe structural roles; never populate observed roles from authored declarations merely to make comparison pass.

## Environment and secrets

Clear ambient environment; provide an operator-authorized host PATH and explicitly declared inputs, not a hardcoded Unix PATH on all systems. Resolve secret references through an explicit provider. Redact retained outputs and structured errors with bounded behavior, but do not claim redaction stops a malicious authorized process exfiltrating secrets. Avoid giving untrusted workers secrets at all. Logging, plans, reports and test fixtures must not include production credentials.

## Resource limits (R42)

Bound parser bytes/depth, graph/fragment expansion, profile combinations, candidate evidence, attachment sizes, queue lengths, worker concurrency and requests. Exact budgets are configured and benchmarked; this conversation did not approve arbitrary numeric values. Limits must fail explicitly and non-authorizing, never silently truncate the required universe. Preserve deterministic ordering under bounded concurrency.

## MCP and consumer transport (R47)

MCP is read-only by default and reads prepared snapshots. Text/descriptions/logs/attachments remain untrusted data and cannot become instructions, runner approval, signer selection or waiver authority. Use private stdio or restricted authenticated IPC; do not add an unauthenticated localhost HTTP daemon by default. Bound requests/responses and show pagination/truncation truthfully. Mutating/executing/signing capabilities need separate explicit authorization and must not be inferred from read permissions.

## Security tests

Use adversarial fake workers and disposable fixture roots: inherited environment, shell metacharacters as one arg, parent symlink, bounded large result, flooding streams, immediate parent exit with open descendant pipes, cancellation races, malformed payload, missing expected selection, replay/conflict, unsupported backend guarantee and attempted authority-store writes. Run native host tests rather than treating cross-compilation as evidence of process isolation or durability.

## Approved lifecycle and host closure

Initial EQM host targets are macOS arm64 and Linux x86_64 (ADR-0012). Product mobile/desktop targets are separately discovered. Unsupported hosts reject guarantees they cannot provide; Windows support is not implied. Native tests on each declared host remain required before claiming support.

RA11 extends whole-invocation supervision explicitly to stdin: establish deadline/cancellation and cleanup ownership before request delivery, handle backpressure concurrently with output, and retain child ownership on broken pipes. Child exit does not end input/output/descendant cleanup supervision. Test all lifecycle stages, not only retained output size.

RA12 applies bounded regular-file handle and confinement rules to every ingress, including config/lock files, executable reads and existing-object comparisons. Test FIFO/device inputs, ancestor symlinks, replacement and growth races; do not allocate the full file before checking its budget or create work directories before validating confinement. Actual isolation/network/secrets and resource ceilings remain independent operator configuration, not worker declarations.
