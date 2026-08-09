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

The source tree contains the authoritative JSON Schemas under `schemas/v1/`
and a complete copyable workspace under `examples/android-ios/`.

Documents describe current v1 only. There are no legacy readers, migration
formats, deprecated aliases, or implicit fallback semantics.
