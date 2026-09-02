# Open Document Spec (ODS) JSON Schemas

This directory contains the official, versioned **JSON Schemas (Draft 2020-12)** for Open Document Spec (ODS).

---

## Schema Status

ODS **2.0** is a clean break from 1.x. Legacy schemas (`1.0.0/`, `1.1.0/`) are removed. Workspaces MUST declare `spec = "2.0"` (or `2.1` for ontology) in `ods.toml`.

### Normative — Version 2.0.0

| Schema | File | Validates |
| :--- | :--- | :--- |
| **Document Frontmatter** | [`2.0.0/document.schema.json`](2.0.0/document.schema.json) | Flat YAML frontmatter: engine keys at top level; no `ods:` wrapper. |
| **Workspace Configuration** | [`2.0.0/config.schema.json`](2.0.0/config.schema.json) | Repository root `ods.toml`. |
| **Custom Profile Definition** | [`2.0.0/profile.schema.json`](2.0.0/profile.schema.json) | Profile definition files registered via `custom_profiles`. |

### Normative — Version 2.1.0 (optional ontology extension)

| Schema | File | Validates |
| :--- | :--- | :--- |
| **Document Frontmatter** | [`2.1.0/document.schema.json`](2.1.0/document.schema.json) | Extends 2.0 with `entity`, `domain`, `schema`, and typed `related` predicates. |
| **Workspace Configuration** | [`2.1.0/config.schema.json`](2.1.0/config.schema.json) | Adds optional `[ontology]` table. |
| **Custom Profile Definition** | [`2.1.0/profile.schema.json`](2.1.0/profile.schema.json) | Same as 2.0 profile schema. |

Documents without `entity` or typed `related` are valid under both 2.0 and 2.1 schemas.

---

## Editor Autocompletion

Add to Markdown frontmatter:

```markdown
---
# yaml-language-server: $schema=https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/2.0.0/document.schema.json
title: Refund Processing
description: How refunds work.
profile: guide
status: stable
---
```

For `ods.toml` (Taplo):

```json
{
  "evenBetterToml.schema.associations": {
    "ods.toml": "https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/2.0.0/config.schema.json"
  }
}
```

---

## 2.0 Document Keys (flat)

| Key | Purpose |
| :--- | :--- |
| `title`, `name` | Optional; must match `# H1` if present (`TITLE-001`) |
| `description`, `tags`, `owner`, `author` | Universal metadata |
| `profile`, `status`, `id`, `share` | Engine keys |
| `depends` | Markdown prerequisites (traversed) |
| `related` | Soft doc links (not traversed); typed predicates in 2.1 |
| `entity`, `domain`, `schema` | Pareto ontology (2.1 only) |
| `resources` | Human assets (PNG, PDF, URLs) |
| `code` | Source file path strings |
| `load` | AI prompt fixtures (JSON, CSV, schemas) |

Traversal depth is configured in `ods.toml` → `[context].default_max_depth` (default `2`).

Normative reference: [`specs/keys.md`](../specs/keys.md).
