# Evidence store recovery

## Preparation

Use an authorized operator with the declared store/host permissions. Identify store version, actual linked SQLite, checkpoint owner, relevant running acquisitions/readers and archive references. Preserve backups/manifests/verification material according to configured retention and sensitivity. Do not run cleanup against a guessed path or a live product media store.

## Interrupted publication

Stop new writes or coordinate through the implemented store operation. Inspect transaction/publication state: staging only, durably installed but unindexed object, committed record with verified objects, or missing/corrupt referenced object. An unindexed object is an orphan candidate, not automatically a completed result. A committed reference with missing bytes is a reported integrity failure, not permission to synthesize evidence.

Recover or retain active staged objects using leases; reclaim only proven inactive orphans. Revalidate raw digest/semantic/attempt relationships before indexing a recovered object. Preserve immutable conflicts rather than choosing the last file seen.

## Index rebuild

Use the implemented bounded rebuild operation over retained immutable manifests/receipts in a disposable or protected destination. Validate versions and signed originals. Compare identities, campaign history, conflict records and decision references with the preserved source. Rebuilding discoverability must not mint trust or accept serialized verified flags. Promote a rebuilt index only after the recorded integrity checks pass.

## Migration/restore

Acquire the documented exclusive migration coordination and preserve a tested backup/restore point. Reject incompatible writers. Exercise interruption/reopen and rollback/restore before production use. Keep wire/evaluator/DB versions distinct; never rewrite original signed evidence as an undocumented migration. If an older evaluator is required for historical replay, report that explicitly.

## Evidence of recovery

Record exact store/evaluator versions, affected object IDs, actions, commands actually used, checksum/integrity comparisons, tests, outstanding missing objects and current-acceptance impact. Resume write/GC/promotion only after appropriate owner approval. The supported native-host durability and crash-injection suite must cover the mechanism actually used.
