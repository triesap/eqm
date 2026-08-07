# Security, Trust, Diagnostics, And Resource Limits

Status: normative

EQM treats repository content, imported authority, adapter/runner output,
evidence, protocol input, and human-authored prose as untrusted until the
specific validation and trust checks for that input succeed. Validation does
not itself grant authority.

## Threat Model

V1 defends against malicious contributors, candidate self-certification,
compromised adapters or runners, forged and replayed evidence, stale facts,
cross-repository substitution, path traversal, symlink escape, command and
prompt injection, control-sequence output, dependency substitution, malformed
or oversized inputs, resource exhaustion, partial writes, and secret leakage.

V1 does not claim to defend a process from a fully compromised host, kernel,
administrator, configured signing identity, or CI control plane. Organizational
identity proofing, production trust roots, secret-provider configuration, CI
authentication, and release approval policy are required deployment inputs and
are not invented by this repository.

## Authority Classes

| Class | Controls | Candidate-local authority in protected evaluation |
| --- | --- | --- |
| product | capabilities, journeys, surfaces, fragments, requirements | may strengthen; weakening requires external approval |
| architecture policy | policy, profiles, required targets, trust and freshness thresholds | may strengthen only |
| runner | executable definitions and enforceable guarantees | exact protected or externally approved digest |
| target binding | implementation mapping and intended exposure | candidate supplies facts but cannot weaken protected obligations |
| exception | waiver scope, approval, duration, controls | protected/external approval only |
| verification | producer identities, evidence, inventories, runtime facts | independently verified, exact-subject data only |
| release | release records, signing identities, final gate approval | protected/external authority only |

Authority classes do not imply one another. An owner of a target binding is not
automatically a waiver approver, runner authority, trust-root administrator, or
release signer.

## Protected Baseline

Pull-request and release evaluation requires either an exact protected
baseline bundle or a signed policy bundle supplied outside candidate-controlled
content. The bundle binds:

- repository identity and exact baseline subject;
- semantic graph and policy digests;
- runner and adapter digests;
- waiver-approver and authority assignments;
- trust-root set, allowed algorithms, and revocation state;
- required evaluation mode and profile rules.

Candidate additions compose through the monotonicity table. Missing baseline,
floating identity, signature failure, repository mismatch, or incomparable
authority is a trust failure, not a development fallback.

## Cryptographic Profile

| Use | V1 algorithm/encoding |
| --- | --- |
| content and semantic digests | SHA-256, lowercase `sha256:` wire form |
| signatures | Ed25519 only |
| signed envelope | DSSE over exact UTF-8 payload bytes |
| key ID | `sha256:` digest of the 32-byte raw Ed25519 public key |
| signature encoding | unpadded base64 in the DSSE `sig` field |

Unsupported algorithm identifiers fail closed. There is no algorithm fallback,
key download, ambient platform key lookup, or automatic signer selection.

A trust-root input is a protected object with exact `key_id`, `algorithm`,
`public_key`, `authority_classes`, `valid_from`, `valid_until`, and `revoked_at`
fields. `revoked_at` is null when not revoked. Public key bytes are base64.
Evaluation uses the injected clock; validity includes `valid_from` and excludes
`valid_until`. A signature made before later revocation remains invalid in v1
unless an external protected policy explicitly supplies a historical trust
decision.

`trusted_ci` may also be established by an externally authenticated CI result
transport that binds repository, immutable run ID, source commit, workflow
identity, producer, and payload digest. It cannot establish `signed_ci` without
an allowed Ed25519 signature over the exact result envelope.

## Exact Subjects And Replay Binding

| Payload | Required signed/bound subject fields |
| --- | --- |
| evidence | repository, target/provider/target-set, source commit, build/artifact identity when applicable, contract, binding, evidence spec, runner, adapter, policy, profiles, runtime facts, release record |
| inventory | repository, target, source commit or build identity, adapter digest, target configuration |
| runtime facts | repository, target, release/build identity, profile values, provider, observation and expiry times |
| release record | repository, target, app version, build number, source commit, artifact digest, channel |
| attestation | all predicate inputs plus statement subjects and evaluation time |
| protected bundle | repository, baseline identity, authority digests, trust configuration, validity interval |

Repository identity is a configured stable URI plus a protected repository-ID
digest; a matching relative path in another checkout is insufficient. Every CI
result includes an immutable run ID and producer identity. Every adapter and
runner invocation includes a random 128-bit request ID from an injected secure
random source; the response/result must echo it. Request IDs are uniqueness
and correlation inputs, not semantic graph inputs.

A valid signature over a different subject, repository, request/run ID,
profile, policy, time window, or digest is a replay/substitution failure. Exact
reuse of one immutable evidence result for the same bound subject and freshness
context is permitted and content-addressed.

## Repository Paths And Symlinks

Lexical `RepoPath` validation occurs before filesystem access:

- relative UTF-8 path only, `/` separators, no empty/`.`/`..` segment;
- no leading slash, drive prefix, UNC prefix, NUL, backslash, or control code;
- NFC normalization and a maximum 1,024 UTF-8 bytes / 128 segments;
- portable collision key is NFC plus ASCII case folding for ASCII letters;
- duplicate portable keys are rejected even on case-sensitive filesystems.

Filesystem access then resolves one component at a time from an already-opened
VCS root. Authored source discovery does not follow directory symlinks. A file
symlink is accepted only for read-only target artifacts when its fully resolved
path remains inside the target root and policy explicitly allows artifact
symlinks; authored metadata, config, lock, generated results, atomic-write
destinations, programs, and working directories may not be symlinks.
Resolution loops, missing components where existence is required, root escape,
mount/substitution races detected by identity recheck, and path-type mismatch
fail closed.

## Runner Boundary

- Only `eqm_runner` may launch a process.
- `program` and every argument are separate OS strings; no shell is invoked.
- Typed placeholders are exactly `{target_root}`, `{selector_json}`, and
  `{result_path}`. A placeholder occupies one complete argument or one complete
  cwd value; interpolation into surrounding text is forbidden.
- `target_root` and `result_path` are resolved/constrained paths.
  `selector_json` is one bounded compact JSON argument.
- The child environment begins empty. Allowed fixed variables are `PATH` from
  trusted runner configuration and `LANG=C.UTF-8`, `LC_ALL=C.UTF-8`,
  `TZ=UTC`. Additional variables require explicit runner bindings.
- Secret values come only from a configured provider at invocation time, are
  never persisted in manifests/evidence, and are registered for exact and
  encoded redaction before process launch.
- cwd is confined to the target root. Programs must be repository-confined or
  exact digest-pinned executables from the lock.
- Timeout, output cap, concurrency, cancellation, and process-tree cleanup are
  mandatory. Partial output never becomes a valid result.

The local backend can enforce process separation, environment construction,
cwd, timeout, output cap, concurrency, and process-tree cancellation. It does
not claim network denial, read-only source, filesystem sandboxing, memory
isolation, or protection from same-user host processes. The container backend
may claim `network_denied`, `read_only_source`, `isolated_process`, and
`resource_limited` only when an exact digest-pinned runtime configuration
enforces each selected guarantee and the corresponding backend test is green.
Unsupported container execution reports unavailable and never falls back to
local execution.

## Adapter Boundary

Adapters are exact digest-pinned out-of-process executables from `eqm.lock`.
They receive one bounded JSON request on stdin and return one bounded JSON
response on stdout. They use the runner process controls, empty-plus-allowlist
environment, confined target cwd, and no secrets unless a future approved ADR
defines a specific adapter credential. V1 permits none.

Dynamic libraries, in-process plugins, embedded scripts, source-code eval,
WASM modules, implicit framework tools, and automatic downloads are forbidden.
Normal commands never acquire adapters. A malformed, mismatched, partial,
timed-out, capped, nonzero, or untrusted adapter response yields explicit
partial/unknown/error facts and never proves absence.

## Untrusted Text And Prompt Injection

Product prose, comments, source code, file names, extension values, adapter and
runner output, test names/logs, evidence messages, diagnostics received from
other tools, and MCP caller text are untrusted data. Renderers:

- label untrusted sections with source and trust;
- escape terminal control characters and unsafe Markdown/URI constructs;
- never concatenate data into shell commands, prompts, tool schemas, policy,
  or procedural instructions;
- keep trusted repository authority separate from collected implementation
  data in context output;
- enforce bounds before decoding/rendering nested content.

MCP and context consumers must treat data fields as quotations. No text field
can grant execution, expand scope, enable verify, select credentials, create a
waiver, or change authority.

## Privacy And Redaction

Telemetry and network reporting are absent by default. Profiles and runtime
facts describe finite symbolic cohorts; email addresses, phone numbers, names,
account IDs, device IDs, IP addresses, precise locations, free-form user
attributes, and other individual identifiers are rejected.

Logs and diagnostics omit environment values, secret-provider responses,
private key material, access tokens, authorization headers, and raw payloads
that may contain secrets. Redaction replaces matches with `[REDACTED]` before
truncation and applies to plain, JSON-escaped, URL-encoded, and base64 forms of
configured secrets. Secret values shorter than 8 bytes are never placed in a
child environment because reliable redaction cannot be guaranteed. Paths in
public machine output are repository-relative; host absolute paths are never
emitted.

## Resource Limits

All limits are inclusive unless stated otherwise. Exceeding a limit produces a
typed diagnostic before further allocation or execution.

| Resource | V1 limit |
| --- | --- |
| one authored TOML file | 4 MiB |
| total authored TOML bytes | 64 MiB |
| authored documents | 10,000 |
| TOML/JSON nesting depth | 64 (extensions/applicability retain their lower limits) |
| string value | 1 MiB; prose-specific limits remain lower |
| array/table entries | 100,000 per container |
| targets | 1,000 |
| capabilities/journeys/surfaces/fragments combined | 100,000 |
| finalized requirements | 100,000 |
| bindings, policies, profiles, runners, waivers each | 100,000 |
| graph reference edges | 1,000,000 |
| fragment expansion depth | 32; fragment cycles are invalid |
| diagnostics retained | 10,000; one truncation diagnostic records omissions |
| CLI JSON input or output document | 64 MiB |
| adapter request / response | 4 MiB / 16 MiB |
| inventory entries | 250,000 |
| normalized test result / evidence result | 16 MiB each |
| evidence attachments | 1,000 records and 1 GiB total referenced bytes |
| MCP frame | 4 MiB; at most 32 in-flight requests |
| context output | caller bound, maximum 1 MiB |
| runner stdout and stderr | configured cap, maximum 16 MiB each |
| adapter stderr | 1 MiB |
| one runner/adapter timeout | 1 second to 1 hour |
| runner concurrency | 1-64 and no more than configured host policy |
| evaluation wall budget | caller-supplied; CLI default 10 minutes, maximum 1 hour |
| canonical projection bytes | 256 MiB |

The 100,000-requirement reference fixture must validate within the performance
budget; limits are rejection boundaries, not promises that all maximums can be
combined under 1 GiB simultaneously. Implementations use checked arithmetic
and reject allocation-size overflow.

## Stable Diagnostic Allocation

Codes have exact form `EQM-E####` and are never reused for a different meaning.

| Range | Owner |
| --- | --- |
| `EQM-E0001`-`EQM-E0099` | CLI usage, configuration selection, discovery setup |
| `EQM-E0100`-`EQM-E0199` | TOML syntax, schema dispatch, field/value validation |
| `EQM-E0200`-`EQM-E0299` | identifiers, paths, collisions, external references, limits |
| `EQM-E0300`-`EQM-E0399` | graph references, fragments, lifecycle, invariants, canonicalization |
| `EQM-E0400`-`EQM-E0499` | applicability, policy, obligations, waivers, conformance, equivalence |
| `EQM-E0500`-`EQM-E0599` | evidence coverage, outcomes, freshness, exposure, release analysis |
| `EQM-E0600`-`EQM-E0699` | runner definitions, execution, results, generated-state writes |
| `EQM-E0700`-`EQM-E0799` | adapters, inventories, discovery, reconciliation |
| `EQM-E0800`-`EQM-E0899` | protected authority, trust, signatures, attestations, replay |
| `EQM-E0900`-`EQM-E0999` | JSON/SARIF/MCP protocol, rendering, framing, output |
| `EQM-E1000`-`EQM-E1099` | environment doctor, dependency, packaging, release preparation |
| `EQM-E9000`-`EQM-E9099` | internal invariant failures; never expected for invalid user input |

Unallocated ranges are reserved. Each allocated code appears exactly once in a
machine-readable registry with title, severity, authority link, explanation,
remediation, emitting component, and tests. User input must never trigger an
internal-range code in an accepted test fixture.

## Security Failure Rules

- Unsupported guarantees and absent organizational configuration are reported
  explicitly; they are never simulated or silently weakened.
- Invalid user-controlled input returns a diagnostic without panic, unsafe
  code, secret disclosure, host absolute paths, or partial authoritative data.
- Temporary files are confined, permission-restricted, atomically renamed,
  and removed on success, failure, or cancellation.
- Dependency advisories, licenses, checksums, SBOM, and provenance are release
  gates. Packaging may prepare signing inputs but cannot claim or select a
  production identity without explicit external configuration.
