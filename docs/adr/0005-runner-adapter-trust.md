# ADR 0005: Runner, Adapter, And Trust Boundaries

Status: approved

## Decision

Bindings reference protected runner IDs and typed selectors. Commands are argv
arrays, adapters run out of process and are digest-pinned, and protected
baselines prevent candidate self-certification.

## Consequence

The runner crate is the only process boundary. Isolation and trust claims are
accepted only when backed by explicit enforceable or verifiable facts.
