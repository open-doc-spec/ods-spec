---
description: 'ODS 2.0 frontmatter key dictionary: two-layer placement (frontmatter +
  ods.toml), flat engine keys, field definitions, valid/invalid examples, and multi-tool
  preservation rules.'
profile: note
status: stable
depends:
- README.md
- core.md
related:
- profiles.md
- graph.md
- assets.md
- context.md
- indexes.md
- validation.md
- ../guides/01-first-document.md
- ../guides/decision-cards.md
---

# ODS · Frontmatter Key Dictionary

This document is the normative reference for **every frontmatter key** in Open Document Spec (ODS) **v2.0**, detailing flat placement rules, data types, semantic meanings, and commented valid/invalid usage examples.

## At a glance

- **What this chapter defines:** Where each key lives (flat frontmatter vs `ods.toml`), profile-definition metadata, types, and valid/invalid examples.
- **Why it exists:** Authors and parsers need one dictionary, not ten overlapping lists.
- **When you need it:** You are adding a field or implementing a frontmatter parser.
- **When you can skip it:** Day-1 authoring only needs `description`, `profile`, `status` — see below.
- **Learn this first:** [Your first document](../guides/01-first-document.md) · [Decision cards](../guides/decision-cards.md)
- **Prerequisite chapters:** [core.md](core.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14, exactly as stated in [README.md §1](README.md#1-conformance-language). That is the canonical statement; do not maintain a second copy here.

---

## 1.1 Novice Quick Start: The 4 Canonical Document Recipes

Novice authors do NOT need to memorize the full key dictionary. A plain Markdown file is already conformant ([core.md §3.0](core.md#30-minimal-conformant-document)). Beyond that floor, ODS 2.0 standardizes **4 Plug-and-Play Recipes**:

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                    THE 4 CANONICAL ODS 2.0 RECIPES                    │
├───────────────────┬─────────────────────────────────────────────────────┤
│ 1. DAILY DOC      │ description + tags + profile + status               │
│ 2. LINKED DOC     │ Recipe 1 + depends / related                        │
│ 3. CODE BINDING   │ Recipe 2 + code (string paths)                      │
│ 4. PROMPT FIXTURE │ Recipe 2 + load (non-Markdown files for AI context)   │
└───────────────────┴─────────────────────────────────────────────────────┘
```

### The Novice "I Want To..." Decision Matrix

| I Want To… | Use This Profile | Required Keys | Optional Keys |
| :--- | :--- | :--- | :--- |
| **Write a setup guide or tutorial** | `profile: guide` | `description`, `tags`, `status` | `depends`, `related` |
| **Record an architectural decision** | `profile: decision` | `description`, `tags`, `status` | `depends` |
| **Link docs to real source code** | Any profile | `code: ["src/file.ts"]` | — |
| **Give an agent extra JSON/CSV context** | Any profile | `load: ["schemas/payload.json"]` | `depends` |

Teaching path: [Your first document](../guides/01-first-document.md). Pocket form: [Decision cards](../guides/decision-cards.md).

---

## 2. Author Cheat Sheet (Copy-Paste Reference)

```yaml
---
# ═════════════════════════════════════════════════════════════════
# LAYER 1: DOCUMENT FRONTMATTER (Flat top-level keys only)
# ═════════════════════════════════════════════════════════════════
description: One-line summary for search previews, AI tool calls, and index listings.
tags:
  - billing
  - customer-care
owner: team:support
author: Alice Smith
created_at: 2026-01-15
updated_at: 2026-08-14

# OKF v0.2 superset keys (optional; preserved for bundle interoperability)
type: BigQuery Table
title: Customer Table
sources:
  - id: bq-schema
    resource: datasets/billing/customers.sql
verified:
  - { by: "human:ahormati", at: "2026-08-22T00:00:00Z" }
stale_after: 2026-12-31T00:00:00Z

# ODS 2.0 engine keys (all flat — no ods: wrapper)
profile: guide
status: stable
share: public
id: docs/v1/refunds

depends:
  - ../auth/sessions.md
  - ../crypto/tokens.md

related:
  - ../policy/refund-sla.md
  - ../faq/billing-faq.md

resources:
  - ../diagrams/refund-flow.pdf
  - https://figma.com/file/refund-ui-mockup
  - path: ../contracts/refund.openapi.yaml
    title: Refund OpenAPI Spec

code:
  - apps/billing/src/refund.ts
  - apps/billing/tests/refund.test.ts

load:
  - ../schemas/refund-request.json
---
# Document Title Lives in the First # H1 Heading
```

Workspace defaults (`context.default_max_depth`, `context.ignore`) live in root `ods.toml` — see [indexes.md §3](indexes.md#3-workspace-configuration-key-reference).

---

## 3. The Two-Layer Key Placement Architecture

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ LAYER 1: Document Frontmatter (Flat YAML in each .md file)              │
│ Universal: description, tags, owner, author, created*, updated*         │
│ OKF superset: type, title, name, resource, sources, verified, …         │
│ ODS engine: profile, status, id, share, depends, related, resources,    │
│             code, load                                                  │
│ -> All keys at the top level; NO ods: wrapper                           │
├─────────────────────────────────────────────────────────────────────────┤
│ LAYER 2: Workspace Manifest (Root ods.toml only)                      │
│ spec, dialect, ignore, custom_profiles, packs, [context], [aliases],    │
│ [okf]                                                                   │
│ -> Repository-wide boundary and discovery configuration                 │
│ -> Full key reference: indexes.md §3                                  │
└─────────────────────────────────────────────────────────────────────────┘
```

The `ods:` namespace from ODS 1.x is **removed**. Tools MUST reject frontmatter containing an `ods` key.

---

## 4. Engine Key Summary Matrix

| Key | Auto-loaded in `ods context`? | Verified by `ods lint`? | Key Purpose |
| :--- | :---: | :---: | :--- |
| **`depends`** | **Yes** (up to workspace `default_max_depth`) | **Yes** (DAG, no cycles) | Hard Markdown prerequisites. |
| **`related`** | **Titles only** (one-line index) | **Yes** (`GRAPH-003`) | Soft lateral document links. |
| **`resources`** | **No** (catalog metadata only) | **Yes** (local files must exist) | Non-Markdown attachments (PDF, CSV, URLs). |
| **`code`** | **Opt-in** (caller requests code) | **Yes** (path exists, no `:L45`) | Source file paths bound to the document. |
| **`load`** | **Yes** (injected directly) | **Yes** (`ASSET-004`) | Auxiliary JSON, CSV, and prompt fixtures. |

Traversal depth is governed by `ods.toml` → `[context].default_max_depth` (default `2`), not by per-document frontmatter. See [context.md](context.md).

---

## 5. Canonical Emit Ordering

When scaffolding (`ods new`), adopting (`ods adopt`), or formatting (`ods fmt`), tools MUST emit top-level keys in this sequence:

`$schema` → `description` → `tags` → `owner` → `author` → `created_at` → `updated_at` → OKF keys (`type`, `title`, `name`, `resource`, `sources`, `usage_window`, `generated`, `verified`, `status`, `stale_after`, `runtime`, `parameters`, `computation`, `executor`, `attester`, `okf_version`) → `profile` → `status` → `id` → `share` → `depends` → `related` → `resources` → `code` → `load`

> **Note**: Parsers MUST accept keys in any order; only emit/formatting tooling enforces the canonical sequence.

---

## 6. Universal & Metadata Keys

These keys MAY appear at the top level of frontmatter alongside ODS engine keys.

### 6.1 `description`

- **Type**: `string`
- **Purpose**: A concise, one-sentence summary used in index listings, search previews, and AI tool calling descriptions.
- **Normative Rules**: SHOULD be between 10 and 200 characters. MUST NOT contain multi-paragraph markdown prose.

```yaml
# VALID
description: Step-by-step instructions for issuing customer credit card refunds.

# INVALID
description: |
  # Refund Guide
  This is a full paragraph explaining the historical background of refunds...
```

### 6.2 `tags`

- **Type**: `list of strings`
- **Purpose**: Free-form categorical keywords for filtering, search facets, and team taxonomy.
- **Normative Rules**:
  - MUST be declared at the top level.
  - Tools SHOULD normalize tags to lowercase `a-z`, `0-9`, `-`.
  - Tags SHOULD NOT collide with status names (`draft`, `stable`) or profile names (`guide`, `decision`).

```yaml
# VALID
tags:
  - billing
  - customer-care

# INVALID (ODS 1.x legacy — rejected in 2.0)
ods:
  tags: [billing]
```

### 6.3 `owner` and `author`

- **`owner`** (`string`): Accountable team, role, or on-call group (e.g. `owner: team:payments`).
- **`author`** (`string`): Individual author, creator, or agent (e.g. `author: Alice Smith`).
- Documents MAY specify either or both.

### 6.4 `created` / `created_at` and `updated` / `updated_at`

- **Type**: `string` (ISO-8601 UTC timestamp or `YYYY-MM-DD` date)
- **Bidirectional Aliases**: `created` ↔ `created_at`, `updated` ↔ `updated_at`.
- Parsers MUST accept both forms transparently.

### 6.5 `title` and `name`

- **Type**: `string`
- **Purpose**: Document title for OKF bundles and tooling that expects a frontmatter title.
- **Normative Rules**:
  - In pure ODS authoring, the canonical title is the first `# H1` in the body.
  - When `title` or `name` is present without OKF signal, it MUST match the H1 (`TITLE-001`) or the body MUST contain an H1 (`TITLE-002`).
  - A `title:` key without OKF signal emits `SYNTAX-002` as a **warning** advising authors to prefer the H1.

```yaml
# VALID (OKF document — title in frontmatter is expected)
type: Dataset
title: Customer Orders
description: Canonical orders table.

# VALID (pure ODS — title in H1 only)
description: How refunds are processed.
profile: guide
status: stable
---
# Refund Processing Guide
```

### 6.6 `$schema` (Optional)

- **Type**: `string` (URI)
- **Purpose**: Enables editor autocomplete and instant validation.
- MAY reference [`schemas/2.0.0/document.schema.json`](../schemas/2.0.0/document.schema.json) or a workspace-relative path.
- `$schema` is OPTIONAL. Documents without it remain fully compliant.

---

## 7. ODS Engine Keys (Flat Top-Level)

All ODS engine keys MUST appear at the top level of frontmatter. There is no `ods:` wrapper.

### 7.1 `profile`

- **Type**: `string` (default: `"note"`)
- **Purpose**: Declares the structural shape and expected section headings of the document.
- **Values**: Standard profiles (`note`, `guide`, `feature`, `decision`, `sop`, `api`, `architecture`, `policy`, `meeting`, `faq`, `checklist`, `agent`, `skill`) or custom profiles registered in `ods.toml`. See [profiles.md](profiles.md).
- Missing sections are **advisory only** in ODS 2.0 (no `PROF-002` error).

```yaml
# VALID
profile: agent
status: stable
```

### 7.2 `status`

- **Type**: `enum` (default: `"draft"`)
- **Allowed Values**: `draft`, `stable`, `deprecated`, `archived`.

```yaml
# VALID
profile: guide
status: stable

# INVALID
status: in-review
```

### 7.3 `id`

- **Type**: `string` (default: workspace-relative path without `.md`)
- **Purpose**: Explicit document identifier override for **rename stability**.
- Authors SHOULD omit this field unless preserving inbound links during a directory restructure.

```yaml
# VALID: preserve legacy link identity across file moves
id: docs/v1/auth-setup
profile: guide
status: stable
```

### 7.4 `share`

- **Type**: `enum` (default: `"public"`)
- **Allowed Values**:
  - `public`: Safe for public distribution and unprivileged AI prompts.
  - `org`: Internal to the organization / repository team.
  - `private`: Sensitive; MUST be excluded from context exports unless explicitly requested.

```yaml
profile: guide
share: private
```

### 7.5 `depends`

- **Type**: `list of strings`
- **Purpose**: Hard directional prerequisites. The reader or AI agent MUST understand the target document(s) before acting on this document.
- **Normative Rules**:
  - Each entry MUST be a workspace-relative path to a `.md` file.
  - The dependency graph MUST be a strict **DAG** (MUST NOT contain cycles).
  - Targets listed in `depends` are automatically traversed during `ods context`; they MUST NOT be duplicated in `load`.

```yaml
depends:
  - ../auth/sessions.md
  - ../crypto/tokens.md
```

### 7.6 `related`

- **Type**: `list of strings` (ODS 2.0) or **union** of strings and predicate objects (ODS 2.1)
- **Purpose**: Soft associative references, suggested reading, and (2.1) domain semantics.
- **Normative Rules**:
  - Plain entries MUST be workspace-relative paths to `.md` files.
  - ODS 2.1 additionally accepts Pareto predicate shorthand (`is_a`, `part_of`, `owns`, `governed_by`, `maps_to`) and `{ predicate: custom, verb, target }`.
  - Cycles in `related` are permitted.
  - During context resolution, `related` contributes **titles and descriptions only** — bodies are never auto-loaded.

```yaml
# VALID (ODS 2.0 and 2.1)
related:
  - ../policies/refund-sla.md
  - ../faq/billing-faq.md

# VALID (ODS 2.1 only)
related:
  - governed_by: ../policies/refund-policy.md
  - maps_to: ../api/refunds-api.md

# INVALID (ENUM-006 in 2.1)
related:
  - replaces: ../decisions/old-adr.md
```

### 7.7 `resources`

- **Type**: `list of strings or resource maps`
- **Purpose**: Attachments associated with the document (diagrams, PDFs, CSVs, OpenAPI specs, Figma/Miro URLs).
- **Entry Formats**:
  - **Simple String**: Relative file path or external URL.
  - **Detailed Map**: `{ path?: string, url?: string, title?: string, description?: string }` — exactly one of `path` or `url` (`ASSET-005`).
- **Normative Rules**:
  - Local paths MUST resolve to real files on disk (`ASSET-001`). URLs are syntax-checked only.
  - Resources are NEVER automatically loaded into LLM prompts. To inject text/JSON into a prompt, declare it in `load`.
  - Full entry-shape contract: [assets.md §5.1](assets.md#51-entry-shapes-normative).

```yaml
resources:
  - ../diagrams/auth-flow.png
  - https://figma.com/file/auth-flow-v2
  - path: ../contracts/payments-v2.openapi.yaml
    title: Payments OpenAPI Spec
```

### 7.8 `code`

- **Type**: `list of strings`
- **Purpose**: Binds the document to source code files (implementations, tests, infrastructure).
- **Normative Rules**:
  - Each entry MUST be a plain string path. Mapping objects with `path`, `role`, or `symbol` are rejected (`CODE-002`).
  - Paths MUST NOT contain line numbers (e.g. `:L42` is prohibited, `CODE-001`).
  - Code bindings are included in context payloads only when the caller explicitly requests code.

```yaml
# VALID: string paths only
code:
  - src/auth.service.ts
  - tests/auth.service.test.ts

# INVALID
code:
  - path: src/auth.service.ts
    role: implementation
    symbol: verifySession
```

### 7.9 `load`

- **Type**: `list of strings`
- **Purpose**: Auxiliary non-Markdown files (JSON schemas, CSV fixtures, environment templates) injected directly into AI prompt context.
- **Normative Rules**:
  - Each path MUST resolve to an existing file on disk (`ASSET-004`).
  - `load` is for **non-Markdown** fixtures. Markdown prerequisites belong in `depends`.
  - Traversal depth for `depends` is controlled by workspace `[context].default_max_depth`, not by frontmatter.

```yaml
load:
  - ../schemas/refund-request.json
  - ../fixtures/sample-order.csv
```

### 7.10 `entity` (ODS 2.1)

- **Type**: `string`
- **Purpose**: Canonical concept or class name for Pareto ontology documents (e.g. `Customer`, `RefundRequest`).
- **Normative Rules**:
  - When present, the value MUST resolve to exactly one **entity definition document** in the workspace (`ENT-001`).
  - Entity names MUST be unique across the workspace (`ENT-002`).
  - Use on concept-definition docs and on feature/API docs that model a specific entity.

```yaml
entity: Customer
domain: Billing
schema: schemas/customer.schema.json
```

### 7.11 `domain` (ODS 2.1)

- **Type**: `string`
- **Purpose**: Business domain partition for an entity (e.g. `Billing`, `Identity`).
- **Normative Rules**:
  - Optional on entity documents. When omitted, tools use `ods.toml` → `[ontology].default_domain`.
  - Does not affect context traversal.

### 7.12 `schema` (ODS 2.1)

- **Type**: `string` (workspace-relative path)
- **Purpose**: JSON Schema file on disk describing the entity's data shape.
- **Normative Rules**:
  - When present, the path MUST resolve to an existing file (`ONT-001`).
  - Authors SHOULD mirror the path in `resources` (human catalog) and `load` (agent injection) when both audiences need it.

---

## 8. Custom Profile Definition Keys

Custom profile metadata lives in Markdown files registered via `custom_profiles` in `ods.toml`. Profile-definition frontmatter uses the keys defined in [`profile.schema.json`](../schemas/2.0.0/profile.schema.json):

| Key | Type | Purpose |
| :--- | :--- | :--- |
| `name` | string, optional | Profile identifier. If omitted, the profile file stem is used. |
| `description` | string, optional | One-line summary shown in tooling. |
| `required_sections` | list of strings, optional | Section headings a document using the profile is expected to carry (advisory). |
| `optional_sections` | list of strings, optional | Recognized but never required sections. |
| `required_keys` | list of strings, optional | Top-level document keys required when the profile is selected. |
| `optional_keys` | list of strings, optional | Useful top-level keys that are not required. |
| `forbidden_keys` | list of strings, optional | Top-level keys that should not appear with the profile. |

Profile-definition metadata MUST appear only in a registered profile-definition file (`PROF-006`). Ordinary documents select the profile via flat `profile:`.

```yaml
# Profile definition file (docs/profiles/incident.md)
name: incident
description: Post-incident review shape.
required_sections:
  - Timeline
  - Root Cause
required_keys:
  - service
```

```yaml
# Document using the custom profile
service: checkout
profile: incident
status: draft
description: Outage on 2026-08-14 affecting checkout.
```

Every `custom_profiles` path in `ods.toml` MUST exist at the exact configured location (`PROF-005`). An unregistered `profile` value is a `PROF-001` error.

---

## 9. Keys Removed in ODS 2.0 (and not restored in 2.1)

The following ODS 1.x keys are **not valid** in ODS 2.0+ frontmatter:

| Removed Key / Block | Replacement |
| :--- | :--- |
| `ods:` wrapper and all nested engine keys | Flat top-level keys |
| `ods.context` (`max-depth`, `load`, `ignore`, `trust-min`) | Top-level `load`; workspace `[context]` in `ods.toml` |
| `@` symbolic handles in graph edges | Workspace-relative Markdown paths |
| `code[].role`, `code[].symbol`, mapping-object `code` | Plain string paths in `code` |
| `invariants`, `relations`, `ods.relations` | Prose, `profile: policy`, or typed `related` (2.1) |
| `memory:` block and cognitive memory keys | Removed (no memory subsystem) |
| Edge metadata (`confidence`, `cardinality`, `role`, `since`/`until`) | Removed |
| 10+ predicates and predicate aliases from 1.1 | Five Pareto predicates + `custom` verb (2.1) |

ODS **2.1** restores a minimal ontology surface: `entity`, `domain`, `schema`, and typed `related` predicates. See §7.6 and §7.10–7.12.

---

## Navigation & Reading Order

| [← Previous Chapter](core.md)           | [📑 Specification Index](README.md) |        [Next Chapter →](profiles.md) |
| :-------------------------------------- | :---------------------------------: | -----------------------------------: |
| **02. Core Format Model & Conformance** |    **Open Document Spec (ODS)**     | **04. Structural Profiles & Shapes** |
