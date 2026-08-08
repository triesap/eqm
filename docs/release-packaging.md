# Release packaging

Run `bash scripts/package_release.sh OUTPUT_DIRECTORY` through the repository's
approved build router. The command creates a deterministic archive, SHA-256
checksum, SPDX 2.3 dependency inventory, and provenance inputs. It never
publishes and records `production_signature: false`.

Signing is separate. `scripts/sign_release.sh` requires an explicit absolute
signer executable and exact artifact path; it has no default credential,
identity, key, or environment-driven fallback. Publication remains an external
authorized operation and is not part of the dry-run workflow.
