---
description: Extend ODS with custom profiles, heading aliases, and packs, and know
  which spec chapter to open for expert or implementer work.
tags:
- learn
- ods
- advanced
- profiles
owner: team:ods
profile: guide
status: stable
depends:
  - 06-run-the-workspace.md
related:
- ../specs/profiles.md
- ../specs/keys.md
- ../specs/validation.md
- ../specs/scope.md
- ../specs/indexes.md
- 09-domain-ontology.md
- ../AGENTS.md
---

# Extend ODS

## Overview

Everything before this page is standard ODS. You extend the standard when the thirteen built-in profiles are not the shape you actually write, when your team says "Objective" instead of "Goal", or when you are implementing a parser.

This is the off-ramp into the spec, not a second spec.

## Prerequisites

- A workspace you already lint ([Run the workspace](06-run-the-workspace.md)).
- A clear gap: a recurring document kind, a heading synonym, or an engine you are building.
- Willingness to stay inside [what ODS refuses to add](../specs/scope.md) (no new file extension, no profile inheritance trees).

## Steps

### 1. Add a custom profile only when a kind repeats

If you write RFCs every month, define the shape once.

`docs/profiles/rfc.profile.md`:

```markdown
---
name: rfc
required_keys:
  - github-issue
forbidden_keys: []
---

# Profile: RFC

## Summary | Executive Summary

## Motivation | Problem Statement

## Proposed Design | Technical Specification

## Drawbacks | Risks

## Unresolved Questions | Open Issues
```

Pipes are accepted heading aliases. Register the file:

```toml
# ods.toml
spec = "2.0"
custom_profiles = ["docs/profiles/rfc.profile.md"]
```

Then documents may say `profile: rfc`. Resolution order is: built-in profiles, then `custom_profiles`, then `packs`. First match wins.

The `required_keys` list makes profile-specific metadata explicit. Each listed key must appear at the top level of documents using the profile:

```yaml
github-issue: 123
profile: rfc
status: draft
```

If the configured profile path is missing or `profile` cannot be resolved, ODS returns an error and does not fall back to the `note` profile.

Do not build inheritance (`rfc` extends `feature` extends `base`). Flat shapes stay debuggable. Full rules: [`specs/profiles.md`](../specs/profiles.md).

### 2. Alias headings the team already uses

```toml
# ods.toml
[aliases.sections]
Goal = ["Target", "Business Objective"]
Validation = ["Sanity Checks", "Smoke Tests"]
```

Use this when the built-in synonym table almost fits. Do not alias everything on day one.

### 3. Import a pack when several workspaces share shapes

```toml
packs = [
  "vendor/engineering-pack"
]
```

A pack is a versioned folder of profiles (and, optionally, templates or skills). Treat it as shared configuration, not as a plugin marketplace inside the spec.

### 4. Open the spec by job, not by chapter number

| You need… | Open |
| :--- | :--- |
| Every frontmatter key, types, placement | [`specs/keys.md`](../specs/keys.md) |
| All 13 profiles and templates | [`specs/profiles.md`](../specs/profiles.md) |
| ID rules, DAG, purity | [`specs/graph.md`](../specs/graph.md) |
| Context algorithm | [`specs/context.md`](../specs/context.md) |
| Domain ontology (2.1) | [`guides/09-domain-ontology.md`](../guides/09-domain-ontology.md), [`specs/graph.md`](../specs/graph.md) §4.4 |
| Code bindings, no line numbers | [`specs/assets.md`](../specs/assets.md) |
| Full `ods.toml` | [`specs/indexes.md`](../specs/indexes.md) |
| Lint rule IDs, exit codes | [`specs/validation.md`](../specs/validation.md) |
| What ODS will not add | [`specs/scope.md`](../specs/scope.md) |
| Formal definitions | [`specs/glossary.md`](../specs/glossary.md) |
| Rules for coding agents editing this repo | [`AGENTS.md`](../AGENTS.md) |

Authors who finished [Your first document](01-first-document.md) should not read those linearly. Implementers should start at [`specs/core.md`](../specs/core.md) then `keys.md` then `validation.md`.

### 5. Dialects and OKF

A workspace declares a **dialect** in `ods.toml`. Only two values exist in ODS 2.0+:

- **`standard`** (default) — normal lint enforcement; warnings stay warnings.
- **`strict`** — warnings promoted to errors.

Google OKF handling is controlled by `[okf].enabled` in `ods.toml`, not by a separate dialect. Agent and skill profiles are first-class under any dialect. See [`specs/indexes.md` §3.2](../specs/indexes.md#32-dialects).

## Troubleshooting

- **"Profile not found."** ODS checks the exact paths in `custom_profiles`. Create the definition at the configured path, correct the path in `ods.toml`, or make `profile:` match the loaded definition name.
- **"Invalid custom profile placement."** Profile-definition metadata (`name`, `required_sections`, etc.) is only valid in a file selected by `custom_profiles` or a registered pack. Move it to that file and use flat `profile:` in ordinary documents.
- **"Two packs define `rfc`."** First match wins; you should get a warning. Rename one.
- **"I want richer graph edges."** ODS 2.0 uses plain string paths in `related`. ODS 2.1 adds optional typed predicates — see [Domain ontology](09-domain-ontology.md) and [`specs/graph.md`](../specs/graph.md).
- **"I want to put `role:` in frontmatter for agents."** Do not. Use `##` headings. Re-read [Pick a shape](02-pick-a-shape.md) §5.

You are at the end of the learning track. Carry [Decision cards](decision-cards.md) and [Common mistakes](mistakes.md). Use the spec as a dictionary.
