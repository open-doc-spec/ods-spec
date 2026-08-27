# Open Document Spec (ODS) JSON Schemas

This directory contains the official, versioned machine-readable **JSON Schemas (Draft 2020-12)** for Open Document Spec (ODS).

---

## Published Schemas

### Version 1.1.0 (Latest — Flat Pareto, Ontologies, Memory & OKF Superset)

| Schema | File | Canonical `$id` / `$schema` URL | Description |
| :--- | :--- | :--- | :--- |
| **Document Frontmatter** | [`1.1.0/document.schema.json`](1.1.0/document.schema.json) | `https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json` | Validates Markdown YAML frontmatter, flat Pareto ontology & memory keys, and Google OKF v0.2 top-level keys. Includes `x-ods-spec` traceability annotations. |
| **Workspace Configuration** | [`1.1.0/config.schema.json`](1.1.0/config.schema.json) | `https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/config.schema.json` | Validates repository root `ods.toml` workspace manifest with ontology & memory backend options. |
| **Custom Profile Definition** | [`1.1.0/profile.schema.json`](1.1.0/profile.schema.json) | `https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/profile.schema.json` | Validates custom profile declarations (`ods.custom_profile`). |

---

## Spec-to-Schema Traceability Matrix (`x-ods-lifecycle`)

To ensure seamless maintenance, every key in `document.schema.json` carries an `x-ods-lifecycle` annotation pointing directly to its normative governing chapter in `specs/`:

| Frontmatter Key | Placement Layer | Governing Specification Chapter | Normative Section Anchor |
| :--- | :--- | :--- | :--- |
| `description` | Layer 1 / OKF | [`specs/keys.md`](../specs/keys.md) | `#description` |
| `tags` | Layer 1 / OKF | [`specs/keys.md`](../specs/keys.md) | `#tags` |
| `owner`, `author` | Layer 1 | [`specs/keys.md`](../specs/keys.md) | `#owner-and-author` |
| `created` / `created_at`, `updated` / `updated_at` | Layer 1 | [`specs/keys.md`](../specs/keys.md) | `#created-and-created_at`, `#updated-and-updated_at` |
| `type` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#okf-type-interoperability` |
| `title` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#okf-title-handling` |
| `resource` | Layer 1 (OKF Native) | [`specs/assets.md`](../specs/assets.md) | `#okf-resource-uri` |
| `sources` / `usage_window`| Layer 1 (OKF Native) | [`specs/graph.md`](../specs/graph.md) | `#provenance-sources` |
| `generated` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#actor-generated-metadata` |
| `verified` | Layer 1 (OKF Native) | [`specs/context.md`](../specs/context.md) | `#trust-tier-verification` |
| `status` / `stale_after` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md), [`specs/context.md`](../specs/context.md) | `#status-lifecycle`, `#staleness-gating` |
| `runtime`, `parameters`, `executor`, `attester` | Layer 1 (OKF Attested Computation) | [`specs/assets.md`](../specs/assets.md) | `#attested-computation-contracts` |
| `memory` (`tier`, `valid_from`, `valid_to`, `asserted_at`, `mutations`, `pin`) | Layer 1 / Layer 2 | [`specs/context.md`](../specs/context.md), [`specs/graph.md`](../specs/graph.md) | `#cognitive-memory-block` |
| `ods.profile` | Layer 2 (`ods:`) | [`specs/profiles.md`](../specs/profiles.md) | `#profile-contracts` |
| `ods.status`, `ods.id`, `ods.share` | Layer 2 (`ods:`) | [`specs/core.md`](../specs/core.md), [`specs/context.md`](../specs/context.md) | `#status-lifecycle`, `#visibility-boundaries` |
| `ods.entity`, `ods.domain` | Layer 2 (Domain Modeling) | [`specs/graph.md`](../specs/graph.md) | `#domain-entity-classes` |
| `ods.schema` | Layer 2 (Domain Modeling) | [`specs/validation.md`](../specs/validation.md) | `#schema-disk-contracts` |
| `ods.invariants` | Layer 2 (Domain Modeling) | [`specs/validation.md`](../specs/validation.md) | `#neuro-symbolic-invariants` |
| `ods.depends` | Layer 2 (`ods:`) | [`specs/graph.md`](../specs/graph.md) | `#knowledge-graph-depends` |
| `ods.related` | Layer 2 (`ods:`) | [`specs/graph.md`](../specs/graph.md) | `#discovery-graph-and-semantic-relations` |
| `ods.resources` | Layer 2 (`ods:`) | [`specs/assets.md`](../specs/assets.md) | `#asset-catalog` |
| `ods.code` | Layer 2 (`ods:`) | [`specs/assets.md`](../specs/assets.md) | `#source-code-bindings` |
| `ods.context` | Layer 2 (`ods:`) | [`specs/context.md`](../specs/context.md) | `#prompt-budget-configuration` |

---

## Editor Autocompletion & Validation

### 1. VS Code & Cursor (via YAML Extension)

To get instant autocomplete, hover tooltips, and real-time error squiggles in Markdown frontmatter, add this line at the very top of your frontmatter:

```markdown
---
# yaml-language-server: $schema=https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json
description: Authentication guide for API tokens.
tags: [auth, security]
owner: team:security
ods:
  profile: guide
  status: stable
---
```

Alternatively, configure VS Code's `.vscode/settings.json` workspace settings to apply the schema automatically to all Markdown files:

```json
{
  "yaml.schemas": {
    "https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json": [
      "*.md",
      "docs/**/*.md",
      "specs/**/*.md",
      "guides/**/*.md"
    ]
  }
}
```

### 2. `ods.toml` Validation in Editors (Taplo TOML)

Add the schema association in `.vscode/settings.json` or root `Taplo.toml`:

```json
{
  "evenBetterToml.schema.associations": {
    "ods.toml": "https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/config.schema.json"
  }
}
```
