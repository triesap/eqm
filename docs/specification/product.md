# Product Specification: EquivalenceMatrix V1

EquivalenceMatrix (`eqm`) is a deterministic, local-first product-conformance
system for teams that ship the same user capabilities across independently
implemented native, web, mobile, desktop, and service targets.

## Primary Outcomes

- Make product intent explicit and versioned.
- Make target implementation locations discoverable.
- Make cross-target semantic equivalence measurable.
- Make evidence freshness and trust visible.
- Make release readiness enforceable against exact subjects.
- Give developers and coding agents stable bounded context.

## Core Question

For a product unit and evaluation context, EQM determines:

- what user-observable behavior is required;
- which targets must expose it;
- where each target implementation lives;
- how users reach it and under which symbolic profile it is enabled;
- which evidence proves each obligation;
- whether that evidence is current and sufficiently trusted;
- whether a valid waiver makes a deviation conditional rather than hidden;
- whether every required target is conformant against one normative contract;
- whether the required target set is semantically equivalent.

## Core Model

- Capability: a stable user outcome.
- Journey: an ordered or branching interaction.
- Surface: a bounded user-visible interaction.
- Fragment: an immutable reusable contract unit.
- Requirement: one atomic user-observable assertion.
- Target: an independently implemented client or service lineage.
- Binding: a target-specific mapping from product unit to artifacts, exposure,
  and evidence specifications.
- Artifact: a typed implementation location.
- Exposure: intended availability conditions.
- Policy: profile-relative obligations, assurance, trust, and waiver rules.
- Waiver: a visible, approved, scoped, expiring exception.
- Evidence specification: the source-controlled declaration of how an
  obligation is demonstrated.
- Evidence result: the immutable record of what ran or was observed.
- Inventory: discovered target facts with an explicit completeness claim.
- Runtime facts: provider-neutral observed exposure facts.
- Release record: an immutable exact build/distribution record.

## Canonical Signup Use Case

`auth.signup` is a journey under `account.create` with identifier, verification
code, and profile surfaces. Web, iOS, and Android implement idiomatic target UI
but share user-observable requirements such as the default identifier channel,
visible phone selection, six-digit verification entry, required profile data,
and visible workspace-name conflict handling.

EQM does not require source layout or UI implementation equality. It evaluates
semantic conformance under the same contract, policy, profile, release, trust,
freshness, and runtime context.

## Non-Goals

EQM is not:

- an application-code generator;
- a shared UI framework or cross-platform abstraction;
- a feature-flag service or vendor rule evaluator;
- a generic policy engine;
- a general software catalog;
- a CI or release platform;
- a test framework replacement;
- an AI system.

## Success Boundary

V1 succeeds when a repository can model complete product units, bind required
targets, derive obligations, ingest trustworthy evidence and facts, evaluate
target conformance, derive target-set equivalence, produce deterministic public
reports, and enforce an exact release gate without compatibility behavior or
application-code generation.
