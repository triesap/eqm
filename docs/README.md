# Documentation

This directory is the canonical public usage context for EquivalenceMatrix.
It is intentionally organized for selective loading by people and agents.

| Need | Read |
| --- | --- |
| Integrate or modify EQM with an agent | [agent-context.md](agent-context.md) |
| Create the first workspace | [getting-started.md](getting-started.md) |
| Understand the graph and evaluation model | [concepts.md](concepts.md) |
| Author `eqm.toml`, `eqm.lock`, and `eqm/` | [manifests.md](manifests.md) |
| Invoke commands and interpret exit codes | [cli.md](cli.md) |
| Produce and trust evidence | [evidence-and-trust.md](evidence-and-trust.md) |
| Connect an LLM or agent over MCP | [mcp.md](mcp.md) |
| Model Android and iOS together | [integrations/android-ios.md](integrations/android-ios.md) |
| Review the approved product specification package | [eqm_product_v1/README.md](eqm_product_v1/README.md) |

The [eqm_product_v1/](eqm_product_v1/) subtree is the owner-approved
revision-2 product specification and ordered implementation plan. Its
`implementation/COMMIT_SEQUENCE.md` is the governing execution sequence and
`implementation/progress.json` is the live completion ledger. It is a
specification, not implemented software; code and tests are marked unimplemented
until their owning steps commit.

The source tree contains the authoritative JSON Schemas under `schemas/v1/`
and a complete copyable workspace under `examples/android-ios/`.

Documents describe current v1 only. There are no legacy readers, migration
formats, deprecated aliases, or implicit fallback semantics.
