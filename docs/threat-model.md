# Threat model

EQM assumes repository content, product source, inventories, evidence, runtime
facts, release records, MCP clients, runner output, adapters, and logs may be
hostile. Trusted inputs are explicit invocation authority, current schemas,
finalized semantic digests, configured local pins, independently verified CI
signatures, and externally governed trust/signing identities.

| Threat | Control | Verification | Limitation |
| --- | --- | --- | --- |
| traversal and symlink escape | normalized repository paths and component checks | `path_traversal`, `symlink_escape` | filesystem/OS correctness is assumed |
| argv/env injection | typed argument templates and empty inherited environment | `argv_injection`, `environment_inheritance` | invoked programs remain untrusted |
| flood, timeout, secret retention | byte/time/cancellation limits and redaction | `output_flood`, `timeout`, `secret_redaction` | redaction covers declared secrets only |
| signature, replay, substitution | exact digest, subject, producer, and replay authority | `signature_tamper`, `replay`, `subject_substitution` | real trust roots are external |
| immutable evidence replacement | content addressing, collision and symlink rejection | `immutable_collision` | retention policy is external |
| MCP privilege escalation | verify absent by default; paired flag/audit gate | `execution_authority` | authorized runners can execute declared programs |

The mandatory mapping is `tests/security/adversarial-cases.tsv`, checked by
`scripts/check_security_matrix.sh`. EQM evaluates declared conformance; it does
not establish application security, complete discovery, legal approval,
production signing, or organizational ownership.
