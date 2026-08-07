# Naming And No-Compatibility Contract

Canonical identities are:

- `EquivalenceMatrix`
- `eqm`
- `eqm.toml`
- `eqm/`
- `.eqm/`
- `eqm_*`
- `EQM-`
- `EQM_`
- `eqm://`
- `https://schemas.equivalencematrix.dev/v1/`

The prior identifiers `FeatureMatrix`, `fmtx`, `.fmtx`, `fmtx.toml`, and `FMTX`
are rejection data only. They must not appear as accepted input, aliases,
commands, package names, environment variables, paths, schemas, modules,
fixtures outside the controlled negative corpus, or generated artifacts.

V1 has no aliases, shims, deprecated keys, serde aliases, dual readers,
migrations, fallback protocols, old schemas, redirects, tombstone resolution,
or compatibility modules. Renaming a canonical ID is remove-plus-add and is
intentionally breaking.

The repository scanner covers production source, schemas, normal docs,
examples, fixtures, CI, packaging, and generated artifacts. This contract, the
scanner's encoded rejection data, and explicit negative fixtures are the only
narrow exceptions and cannot introduce executable compatibility behavior.
