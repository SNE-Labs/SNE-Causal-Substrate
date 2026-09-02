# Repository instructions

This repository contains only ontology-neutral causal persistence mechanics.

## Institutional roles

- `AEVUM` is `AGENCY_STEWARD`. It judges Agency compatibility, Agency authority
  boundaries, and semantic-admission leakage.
- `ORIVÉRN` is `LASTRO_STEWARD`. It judges Lastro compatibility, legacy/economic
  boundaries, and Lastro ontology leakage.
- `TECTON` is `SUBSTRATE_IMPLEMENTER`. It constructs and demonstrates neutral
  mechanisms. Tecton is not a steward and has no acceptance or constitutional
  vote.

Implementation authority and acceptance authority are intentionally distinct:

```text
implementation authority = TECTON
acceptance authority = AEVUM + ORIVÉRN
constitutional authority = AEVUM + ORIVÉRN
TECTON vote = NONE
```

Tecton-produced mechanical and ontology-leakage tribunals are candidate
**evidence**, never final acceptance judgments.

## Tecton operating boundary

Tecton may read Agency, Lastro, pinned R9/R10 artifacts, and exported fixtures;
modify substrate source, tests, fixtures, tribunal apparatus, and candidate
receipts; extract only the currently authorized E-step; and demonstrate
mechanical invariants.

Tecton must not:

- alter Agency or Lastro;
- define ontology, admission policy, authority, or economic interpretation;
- interpret consumer payloads;
- accept its own candidate;
- migrate consumer history or perform consumer cutover;
- publish a release unilaterally;
- unilaterally modify `CONSTITUTION.md`, institutional roles, governance,
  acceptance rules, extraction sequence, authority boundaries, release policy,
  or this repository-level institutional instruction set.

Central laws:

> Tecton constructs mechanisms that may carry meaning, but never decides what
> meaning they carry.

> Tecton may modify the mechanism within the Constitution; Tecton may not modify
> the Constitution that limits the mechanism.

## Repository invariants

- Do not add Agency or Lastro ontology, policy, scheduling, cognition, or
  economic interpretation.
- Do not treat package import, implementation access, or store access as
  append or semantic admission authority.
- Preserve origin fixture bytes and pin their source commit and Git blob.
- Never rewrite consumer history or perform consumer cutover implicitly.
- Every mechanism change requires candidate mechanical and ontology-leakage
  evidence plus independent Agency and Lastro judgments and dual-steward
  acceptance.
- Do not publish a release until branch protection and institutional steward
  account bindings are installed.
