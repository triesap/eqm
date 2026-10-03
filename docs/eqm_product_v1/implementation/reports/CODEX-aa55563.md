# Independent review — aa55563

Codex reviewed clean `aa5556336075aae2065862ed65166176717888f6` against the
original P03 obligations, retained findings and DEV-017. Five returned commits
are retained. The implementation agent reached its required review endpoint;
that endpoint does not establish P03 acceptance.

## Accepted progress

S039 and RF55/RF60 are accepted for allocated scope. The real comparison consumer
retains exact cohort value, checks consuming graph membership and selected
scenario identity/revision, then independently resolves expected axes before
matching candidate bytes. Equivalent graph copies pass; foreign/missing cohort,
wrong scenario, altered cohort and configuration/content substitutions reject.
Existing coverage, repeated actor and distinct build controls remain. RF57's
allocated graph-authority repair is accepted; S041 still requires owning schema
parity. Preserve all 40 prior numbered acceptances: 41 now accepted including
S039. RF56/RF58 remain accepted. Whole RA02/RA13 remain open.

## RF62 — P2: schema identifier bounds remain incomplete

Both new pattern families accept unbounded dotted segments and total length.
`eqm_domain/src/id.rs` requires each ASCII segment <=63 bytes and an ID <=255
bytes, with per-type segment counts and requirement limits. An isolated real
Draft 2020-12 probe accepts a comparison profile ID with a 64-character first
segment while `ComparisonContextV2Dto::from_json` rejects it. These ASCII bounds
are schema-expressible; they are not the Unicode byte-count exception.
The report claims vocabulary and bounds but the owning sources omit these bounds.

## RF63 — P2: recursive extension namespace/key rules omitted

`schema_graph_v2.rs` closes tagged value branches but extension maps still allow
arbitrary property names. Isolated schema/checked-reader probes establish schema
acceptance and reader rejection for an invalid namespace and separately an invalid
nested object key under a valid namespace. `ExtensionNamespace` requires >=3
lowercase segments; `ExtensionKey` requires one lowercase segment, <=63 bytes.
Audit authored extension maps as well as graph/comparison recursive forms. Domain
NFC/depth/node/raw-duplicate checks remain separate; lexical key constraints are
schema-expressible and were already required by DEV-017.

## RF64 — P2: repository schema narrows valid owning authority

Both generators use `[a-z0-9.-]+` for host. The domain's existing
`RepositoryIdentity::from_str` accepts an explicit port; generated cohort schema
rejects `https://example.com:443/team/repo`. The isolated probe confirms both sides.
Align schema to the existing owning parser, including expressible length/character
limits; do not tighten the domain or invent a new URI policy to make parity pass.

## Evidence and limitations

Independent standard gate passed 720 successful test executions and changed
owning doctests passed 9. Structure validation and diff checks passed. v1 schemas
and Cargo.lock are unchanged. One isolated probe with four assertions confirms
RF62/RF63/RF64; its passing result proves mismatches, not feature correctness.
The initial probe location was outside the configured source root: doctor blocked,
so no build ran there; the isolated worktree was moved into an approved source
root, doctor passed, and routed verification completed. Production code/tests
were not modified by Codex.

The returned P03-R5 report still names obsolete baseline `03b2a3e` after an
unreviewed S039-R3 amend to `078d8b4`; preserve original failures and reconcile
actual tested revisions instead of claiming every intermediate tree was green.
STATUS and the plan also retain obsolete missing-run assertions. DEV-020 updates
current authority while preserving historical receipts. Final run evidence supports
local tests; piped targeted commands alone do not prove their upstream exits.

S041/S043, RF59/RF61 and P03 gate remain changes-required. RF45/RF53 remain partial;
no whole requirement/RA closes from this allocation. P04–P15 are pending. No human
release-testing milestone is reached. Extended verify, Linux, coverage/mutation/
fuzz/benchmark/package and real product/crypto/host/signing/operational evidence
remain unverified; these are not all established external blockers.
