# ADR 0006: Repository Bootstrap And Verification Adaptation

Status: approved

## Context

The imported implementation sequence assumed a valid Rust workspace and could
therefore begin with product code. The repository instead began with an
unusable scaffold and unrelated package metadata. Building on that state would
have hidden the intended package graph and made later verification unreliable.

## Decision

Bootstrap the explicit eight-crate Rust workspace, pinned toolchain, lockfile,
lint policy, metadata layout, and repository verification commands before
implementing domain behavior. Treat the existing public repository state as
the implementation baseline; source-package checksums establish provenance but
do not make external filesystem locations part of the repository contract.

## Consequences

Bootstrap work is independently reviewable and later checkpoints inherit one
stable verification lane. Public documentation remains standalone and cannot
refer to a containing workspace or operator-local build tooling.

## Affected Original Steps

Steps 001-008 are the prerequisite authority-and-bootstrap wave for all later
implementation steps.

## Acceptance Evidence

- Explicit Cargo metadata resolves under the committed lockfile.
- All workspace targets pass format, check, test, lint, and documentation gates.
- Repository instructions and specifications contain no containing-workspace
  dependency.
