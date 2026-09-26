# Protected authority, verification and release decisions

## Threat and authority model (R23–R26, R64)

Candidate repository files, test code, runner/adapter output, imported evidence, logs, prose and MCP resources are untrusted claims. An approved typed graph validates local structure; it does not independently establish trust. A stored verified flag or deserialized receipt is not proof.

Trusted release authority is obtained independently of candidate-controlled changes. Protect policy, relevant contracts, requirement/target participation, evidence mappings and selections, waivers, trust configuration and the EQM evaluator used to grade the candidate. Deleting or reclassifying critical requirements/tests must be checked as possible weakening, not fewer obligations. Approved authority changes are versioned and explicit; protect the whole expectation chain rather than one file path.

The trust boundary must check signature validity, authorized identity/issuer, signer permissions, expected builder/producer, exact repository/source/build/artifact/configuration subject, approved execution context, and required time/revocation/verification material. Each mandatory input—evidence, release/candidate records, runtime facts—needs an explicit verification conclusion. Missing conclusions cannot disappear through filter_map/min aggregation.

A trust enum can remain display shorthand but does not replace independent acceptance dimensions. A claim cannot raise authority. Trusted test-only stand-ins stay inaccessible to production builds and configuration. Private constructors mitigate misuse by cooperating code, not malicious same-privilege execution.

## Cryptographic integration (R25)

Use a reviewed implementation and a provider-neutral internal verifier boundary. Sigstore bundle verification is the approved default CI integration direction, subject to inspecting exact library/tool support and pinning a currently audited implementation. Configure authorized identity/issuer, expected builder/subject, roots/material and algorithm policy explicitly. Do not invent real signer IDs, use arbitrary public signed artifacts as trusted EQM evidence, or disable claim checks to make an import pass.

Local deterministic cryptographic fixtures may be generated with test-only keys/material and clearly labeled. Production activation requires operator configuration and independent verification evidence. Default developer operation remains explicitly untrusted where appropriate. No implicit network acquisition; any needed retrieval is an authorized acquisition step, not a hidden evaluator action.

A persistent verification receipt binds exact bytes/digests, verifier identity, trust configuration, checked dimensions, time and supporting material. Reimport/replay reestablishes the required verification boundary. Preserve receipts for historical replay without treating them as permanently valid under future trust policy.

## Waiver contract (R29)

Waivers require explicit scope/approvers/reason/controls/start/expiry and protected provenance. Apply only to the precise obligations allowed by the normative rules. Report only actually contributing waivers, not every authored waiver in the graph. Agents must not create, widen, renew, or reinterpret waivers without human authority. Unknown trust/context and unstable evidence cannot be silently waived into satisfied evidence.

## Readiness, statements and authorization (R27–R28, R63)

Pre-release readiness binds a candidate subject before deployment. Post-deployment conformance evaluates actual deployed observations later. Runtime facts must be compared to expected exposure and exact subjects; mere document existence is not match. Separate lifecycle record types where required to avoid evidence/decision digest cycles.

Evaluation, optional signing, release authorization and deployment are distinct operations. A signed failure/unknown decision remains a signed failure/unknown decision. Unsigned reports say unsigned. Unknown never authorizes. Conditional release requires explicit protected acceptance of exact contributing deviations, not automatic interpretation of a waiver label as approval.

Authorization binds the decision, exact artifacts/participant subjects, channel/destination, policy/authority, allowed deviations and validity interval. The deployment system verifies this against what it actually promotes. EQM supplies validation/records, not a new deployment orchestrator. Artifact transformations after evaluation need an explicit trusted provenance relationship or new evaluation; the same commit is not enough.

Independent target release cadence requires a declared supported compatibility matrix. Do not assume all devices/apps/providers upgrade together. Scope a release to the combinations and participant roles the owner actually supports.

## Operational defaults

Keep production release enforcement disabled/shadowed until the supported-host, crypto, scope, exact-subject and deployment-consumption gates pass and an authorized operator approves enforcement. Do not leave an unconditional synthetic trust adapter behind a production feature flag. A rollback restores a known-good evaluator/authority or pauses promotion; it never trusts claims to preserve availability.

Missing production identities, product repositories, deployment routes or live verification material are explicit readiness blockers, not reasons to invent values. Implement generic contracts and local negative tests while preserving that distinction.
