---
description: "How to propose a change to the Open Document Spec: the key-proposal path, the fixture requirement, and what the conformance suite enforces."
tags:
  - contributing
  - governance
  - ods
owner: team:ods
ods:
  profile: guide
  status: stable
  share: public
  related:
    - specs/scope.md
    - specs/validation.md
    - schemas/README.md
---

# Contributing to the Open Document Spec

## Overview

This repository holds the **specification only**: the normative chapters in `specs/`, the learning track in `guides/`, the JSON Schemas in `schemas/`, and a conformance suite that proves the three agree.

Tooling — the `ods` CLI, the language server, editor extensions — lives in the separate [`open-doc-spec/ods`](https://github.com/open-doc-spec/ods) repository. A pull request here changes what the format *means*; a pull request there changes what a program *does*.

## Prerequisites

- A recent stable Rust toolchain (for `cargo test`).
- Read [`specs/scope.md`](specs/scope.md) first, especially the non-goals. Most rejected proposals are rejected because they were already considered and declined there, not because they were bad ideas.

## Steps

### Proposing a new key

A key proposal is four things, in this order. A proposal missing any of them cannot be evaluated.

1. **The problem.** What can an author not express today, and what do they do instead? "Teams work around this by X" is the strongest possible opening. A key that solves a hypothetical is a key everyone must learn and nobody needs.
2. **Why an existing key will not do.** ODS already has 3 layers and 5 subsystems. Show that the fact does not belong in `related`, `context.load`, a body heading, or an unknown top-level key that tooling already preserves untouched.
3. **The schema delta.** The exact addition to `schemas/1.1.0/document.schema.json` (or `config.schema.json`), including an `x-ods-lifecycle` block naming the chapter that will define it. Additive-only: see the version semantics in [`specs/scope.md` §7.1](specs/scope.md#71-version-semantics).
4. **A fixture.** At least one under `tests/fixtures/` — positive if the key is valid, under `tests/fixtures/invalid/` if you are adding a Tier 1 constraint, under `tests/fixtures/tier2/` with an `x-ods-expect` marker if you are adding a workspace-semantic rule.

Then the prose: the key's normative definition in its governing chapter, its row in the [`keys.md`](specs/keys.md) subsystem matrix, and, if it changes what an author writes on a normal day, a mention in the relevant guide.

### Proposing a new lint rule

Add the row to the matrix in [`specs/validation.md` §4](specs/validation.md#4-normative-lint-rules-matrix) with a rule id, a validation tier, a severity, and a remediation. If the tier is 2, add a fixture under `tests/fixtures/tier2/` declaring `x-ods-expect: <RULE-ID>` — `test_every_tier2_rule_has_a_fixture` fails otherwise, by design.

Severity is not a style choice. **Error** means a conformant workspace cannot contain this. **Warning** means the author should know but CI should not block. If you cannot state the failure a user suffers when the condition holds, it is a warning at most.

### Removing or changing a key

You cannot, inside a MINOR release. Deprecate it: set `x-ods-lifecycle.status` to `deprecated` with `deprecated_in` and `removed_in`, add a `DEPR-*` warning rule, record it in [`specs/scope.md` §7.2](specs/scope.md#72-deprecated-in-11--scheduled-for-removal-in-20), and state the precedence rule for documents that use both spellings. Removal happens in the next MAJOR.

### Editing prose

One rule above all others: **do not copy a normative statement into a second file.** The specification previously stated the code-role count in five places and they drifted to two different numbers. Every duplicated table is a future contradiction.

If a chapter needs a fact that lives elsewhere, link to it and say where the canonical version is — the pattern used in [`specs/README.md` §7](specs/README.md#7-design-principles). The canonical homes are listed in the audit trail of `CHANGELOG.md`.

## Validation

```bash
cargo test
```

Fourteen tests run. Four of them exist specifically to catch the failure modes above:

| Test | Catches |
| :--- | :--- |
| `test_no_broken_internal_links` | Any cross-reference or `#anchor` that no longer resolves. |
| `test_prose_matches_schema_enums` | A closed vocabulary stated differently in prose and schema. |
| `test_config_enums_match_prose` | An `ods.toml` value documented but rejected by its own schema. |
| `test_every_tier2_rule_has_a_fixture` | A rule added to the matrix with no executable example. |

If a change to a heading breaks `test_no_broken_internal_links`, fix the inbound links — do not rename the heading back. The test is telling you something real.

## Troubleshooting

- **"My valid fixture fails schema validation."** Check the layer: universal keys (`tags`, `description`, `owner`, `author`, dates) at the top level, engine keys under `ods:`, workspace keys in `ods.toml`. The document schema rejects universal key names under `ods:` by name.
- **"`test_every_tier2_rule_has_a_fixture` fails after I added a rule."** That is the intended behavior. Add the fixture.
- **"`test_prose_matches_schema_enums` fails and I only edited prose."** You changed a count or dropped an enum member from its canonical chapter. The schema and the prose must state the same thing.
