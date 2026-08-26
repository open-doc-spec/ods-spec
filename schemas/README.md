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

## Spec-to-Schema Traceability Matrix (`x-ods-spec`)

To ensure seamless maintenance, every key in `document.schema.json` carries an `x-ods-spec` annotation pointing directly to its normative governing chapter in `specs/`:

| Frontmatter Key | Tier / Namespace | Governing Specification Chapter | Normative Section Anchor |
| :--- | :--- | :--- | :--- |
| `description` | Tier 1 / OKF | [`specs/keys.md`](../specs/keys.md) | `#description` |
| `tags` | Tier 1 / OKF | [`specs/keys.md`](../specs/keys.md) | `#tags` |
| `owner` | Tier 1 | [`specs/keys.md`](../specs/keys.md) | `#owner` |
| `created` / `updated` | Tier 1 | [`specs/keys.md`](../specs/keys.md) | `#created`, `#updated` |
| `type` | Tier 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#okf-type-interoperability` |
| `title` | Tier 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#okf-title-handling` |
| `resource` | Tier 1 (OKF Native) | [`specs/assets.md`](../specs/assets.md) | `#okf-resource-uri` |
| `sources` / `usage_window`| Tier 1 (OKF Native) | [`specs/graph.md`](../specs/graph.md) | `#provenance-sources` |
| `generated` | Tier 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#actor-generated-metadata` |
| `verified` | Tier 1 (OKF Native) | [`specs/context.md`](../specs/context.md) | `#trust-tier-verification` |
| `status` / `stale_after` | Tier 1 (OKF Native) | [`specs/core.md`](../specs/core.md), [`specs/context.md`](../specs/context.md) | `#status-lifecycle`, `#staleness-gating` |
| `runtime`, `parameters`, `executor`, `attester` | Tier 1 (OKF Attested Computation) | [`specs/assets.md`](../specs/assets.md) | `#attested-computation-contracts` |
| `ods.profile` | Tier 2 (`ods:`) | [`specs/profiles.md`](../specs/profiles.md) | `#profile-contracts` |
| `ods.status`, `ods.id`, `ods.share` | Tier 2 (`ods:`) | [`specs/core.md`](../specs/core.md), [`specs/context.md`](../specs/context.md) | `#status-lifecycle`, `#visibility-boundaries` |
| `ods.entity`, `ods.domain` | Tier 2 (Pareto Ontology) | [`specs/graph.md`](../specs/graph.md) | `#ontology-entity-classes` |
| `ods.schema` | Tier 2 (Pareto Ontology) | [`specs/validation.md`](../specs/validation.md) | `#schema-disk-contracts` |
| `ods.relations` | Tier 2 (Pareto Ontology) | [`specs/graph.md`](../specs/graph.md) | `#typed-semantic-relations` |
| `ods.invariants` | Tier 2 (Pareto Ontology) | [`specs/validation.md`](../specs/validation.md) | `#neuro-symbolic-invariants` |
| `ods.tier` | Tier 2 (Pareto Memory) | [`specs/context.md`](../specs/context.md) | `#cognitive-memory-tiers` |
| `ods.valid_from`, `ods.valid_to` | Tier 2 (Pareto Memory) | [`specs/graph.md`](../specs/graph.md) | `#bi-temporal-valid-windows` |
| `ods.mutations`, `ods.pin` | Tier 2 (Pareto Memory) | [`specs/context.md`](../specs/context.md) | `#entity-state-mutations`, `#memory-decay-pinning` |
| `ods.depends`, `ods.related` | Tier 2 (`ods:`) | [`specs/graph.md`](../specs/graph.md) | `#knowledge-graph-depends`, `#discovery-graph-related` |
| `ods.resources`, `ods.code` | Tier 2 (`ods:`) | [`specs/assets.md`](../specs/assets.md) | `#asset-catalog`, `#source-code-bindings` |
| `ods.context` | Tier 2 (`ods:`) | [`specs/context.md`](../specs/context.md) | `#prompt-budget-configuration` |

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
