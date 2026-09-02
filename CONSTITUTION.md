# Constitutional boundary

The canonical constitution is the byte-exact R9 artifact pinned in
`constitutional/bootstrap.json`. This file is an operational summary, not a
replacement for those source receipts.

## Governance

The repository is governed jointly by `AGENCY_STEWARD` and `LASTRO_STEWARD`.
Neither steward may unilaterally change the protocol, publish a release,
approve a breaking change, migrate history, or alter this constitution.

Every release requires:

- immutable version and commit identity;
- Agency compatibility CI;
- Lastro compatibility CI;
- ontology-leakage checks;
- explicit acceptance by both stewards.

GitHub account or team bindings for the two institutional roles must be
recorded before enabling CODEOWNERS-based enforcement and before the first
release. A repository administrator is not automatically either steward.

## Mechanical scope

Only these mechanisms may enter incrementally:

1. E1 — record framing;
2. E2 — digest and predecessor chaining;
3. E3 — strict replay;
4. E4 — expected-head append;
5. E5 — exclusive-writer locking;
6. E6 — snapshot-to-head binding.

Each step requires the substrate tribunal, both consumer adapter tribunals,
ontology-leakage checks, and dual acceptance. Existing consumer implementations
remain canonical until acceptance of that step.

## Exclusions

Agency and Lastro ontologies, world-object meaning, economic meaning, truth,
authority semantics, Projection semantics, scheduling, commitments, cognition,
residency, Quests, Probes, Opportunities, Expeditions, and economic policy do
not belong here.

Importing the package grants nothing. The substrate cannot self-admit or
interpret payloads. It mechanically effects and verifies operations only when
given an explicit, scoped, host-provisioned capability.

## Still forbidden

R10 does not authorize bulk code movement, historical rewriting, journal
migration, automatic consumer cutover, ontology extraction, unilateral
governance, authority by import, or a package release before both tribunals.

