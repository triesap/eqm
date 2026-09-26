# Trust activation, rotation and current acceptance

## Initial activation

Record an authorized operator, exact verifier/evaluator artifacts, protected trust configuration, permitted identities/issuers/signers/builders, expected subject constraints, required material and temporal policy. Values must come from actual approved infrastructure, not fixtures or a guessed CI organization. Inspect current verifier documentation and exact pinned implementation before activation.

Demonstrate real cryptographic positive and negative fixtures, then verify authorized environment evidence against exact expected subjects. Confirm no production synthetic trust path or imported verified flag bypass exists. Workers must not access signer credentials or mutate authority/store. Read-only MCP has no implied signing permission.

## Rotation or trust change

Version new trust material/configuration and preserve historical referenced material subject to retention. Test signature/identity/issuer/subject/build acceptance and relevant temporal rules before replacing active configuration. Invalidate current acceptance/cache entries that depend on the changed trust context. Do not mutate archived decisions to make their historical context appear current.

## Revocation/compromise

Suspend affected current authorization while impact is uncertain. Identify decisions/evidence bound to the affected signer/root/builder/evaluator via reverse indexes. Apply the configured current verification policy, retain historical records and record reasons. Lack of required verification material is unknown/non-authorizing, not a best-effort pass. Restore authority only through the approved reviewed path.

## Signing versus authorization

Sign only an exact selected decision under explicit signer authority. A signed failed/unknown record is a faithful audit artifact, not a release pass. Release authorization is a separate protected action with current-acceptance and subject/destination checks. Do not use signing as a side effect of ordinary report reads.

## Required evidence

Keep actual command names/config paths in the environment-specific operator record after implementation; this package does not invent administration flags. Record raw payload and bundle references, checked dimensions, verifier/authority IDs, relevant time/material, test results, independent review and unresolved limitations. No production key material belongs in Git or this handoff.
