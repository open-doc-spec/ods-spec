# Open Document Spec (ODS) JSON Schemas

This directory contains the official, versioned machine-readable **JSON Schemas (Draft 2020-12)** for Open Document Spec (ODS).

---

## Published Schemas (Version 1.0.0)

| Schema | File | Canonical `$id` / `$schema` URL | Description |
| :--- | :--- | :--- | :--- |
| **Document Frontmatter** | [`1.0.0/document.schema.json`](1.0.0/document.schema.json) | `https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.0.0/document.schema.json` | Validates Markdown YAML frontmatter, 3-tier key placement, and engine contracts. |
| **Workspace Configuration** | [`1.0.0/config.schema.json`](1.0.0/config.schema.json) | `https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.0.0/config.schema.json` | Validates repository root `ods.toml` workspace manifest. |
| **Custom Profile Definition** | [`1.0.0/profile.schema.json`](1.0.0/profile.schema.json) | `https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.0.0/profile.schema.json` | Validates custom profile definitions (`ods.custom_profile`) for packs and extensions. |

---

## Editor Autocompletion & Validation

### 1. VS Code & Cursor (via YAML Extension)

To get instant autocomplete, hover tooltips, and real-time error squiggles in Markdown frontmatter, add this line at the very top of your frontmatter:

```markdown
---
# yaml-language-server: $schema=https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.0.0/document.schema.json
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
    "https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.0.0/document.schema.json": [
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
    "ods.toml": "https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.0.0/config.schema.json"
  }
}
```

---

## Two-Tier Validation Architecture

ODS uses a strict **Two-Tier Validation Model**:

1. **Tier 1 (Schema Layer)**: Structural syntax, 3-tier key placement, forbidden keys (`title:`), enum verification (`ods.status`, `ods.share`, `ods.code[].role`), and pattern constraints. Executes in `<1ms` per document with zero disk I/O.
2. **Tier 2 (Semantic & Graph Layer)**: Directed Acyclic Graph (DAG) acyclicity, disk path existence checks (`resources`, `code`, `load`), code symbol extraction, and profile section heading verification.
