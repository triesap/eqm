# Security policy

## Supported versions

Only the current `1.x` schema/protocol major and the latest released `eqm`
patch are eligible for security fixes. There is no legacy reader or migration
surface. This repository is currently locally accepted software, not a
production-trust or published-release claim.

## Reporting

Do not include credentials, private application data, exploit payloads, or
unredacted logs in a public issue. Use the repository host's private
vulnerability-reporting channel when one is visibly enabled. If no private
channel is available, do not guess a contact: open a minimal public issue that
requests a private reporting route without vulnerability details.

Receipt, response, embargo, CVE, and disclosure timelines require an assigned
security owner. That owner is intentionally unassigned; production release is
blocked until repository governance supplies and verifies one.

## Security scope

Relevant reports include authority confusion, digest/canonicalization bypass,
path or symlink escape, command/environment injection, secret leakage,
signature/replay/subject substitution, resource-limit bypass, immutable result
replacement, MCP execution without explicit audit authority, and schema or
compatibility acceptance outside v1. Claims that EQM proves application
security, signing identity, organizational approval, or complete inventory are
out of scope because the product makes no such guarantee.
