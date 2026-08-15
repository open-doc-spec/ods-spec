---
description: "Exhaustive ODS frontmatter key dictionary: 3-tier layering, 5-subsystem engine mapping, field definitions, valid/invalid examples, and multi-tool preservation rules."
ods:
  profile: "note"
  status: "stable"
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

This document is the normative reference for **every frontmatter key** in the Open Document Spec (ODS), detailing placement rules, 5-subsystem engine mappings, data types, semantic meanings, and commented valid/invalid usage examples.

## At a glance

- **What this chapter defines:** Where each key lives (top-level vs `ods:` vs `ods.toml`), profile-definition metadata, types, and valid/invalid examples.
- **Why it exists:** Authors and parsers need one dictionary, not ten overlapping lists.
- **When you need it:** You are adding a field or implementing a frontmatter parser.
- **When you can skip it:** Day-1 authoring only needs `description`, `tags`, `ods.profile`, `ods.status` — see below.
- **Learn this first:** [Your first document](../guides/01-first-document.md) · [Decision cards](../guides/decision-cards.md)
- **Prerequisite chapters:** [core.md](core.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt), [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt)) when, and only when, they appear in all capitals.

---

## 1.1 Minimum authoring set

Do not treat the cheat sheet below as mandatory on day one. Adopt keys in this order:

| When | Keys |
| :--- | :--- |
| **Day 1** | `description`, `tags`, `ods.profile`, `ods.status` |
| **When linking documents** | `ods.depends`, `ods.related` |
| **When attaching the world** | `ods.resources`, `ods.code`, `ods.context` |
| **Rare** | `ods.id` (rename stability), `ods.share` (privacy), `owner`, `created`, `updated` |

Teaching path: [Your first document](../guides/01-first-document.md). Pocket form: [Decision cards](../guides/decision-cards.md).

---

## 2. Author Cheat Sheet (Copy-Paste Reference)

```yaml
---
# ═════════════════════════════════════════════════════════════════
# TIER 1: UNIVERSAL TOP-LEVEL KEYS (Visible to all YAML consumers)
# ═════════════════════════════════════════════════════════════════
description: One-line summary for search previews, AI tool calls, and index listings.
tags:
  - billing
  - customer-care
owner: team:support
created: 2026-01-15
updated: 2026-08-14

# ═════════════════════════════════════════════════════════════════
# TIER 2: ODS ENGINE KEYS (Scoped under ods: to prevent collisions)
# ═════════════════════════════════════════════════════════════════
ods:
  profile: guide                              # Document shape / expected H2 or H3 sections
  status: stable                              # Lifecycle maturity: draft | stable | deprecated | archived
  share: public                               # Privacy boundary: public | org | private

  # ─────────────────────────────────────────────────────────────
  # Subsystem 1: Knowledge Graph (Structural Prerequisites)
  # • Auto-traversed by 'ods context' up to max-depth (default: 2)
  # • Strict DAG: Cycles are forbidden (checked by 'ods lint')
  # ─────────────────────────────────────────────────────────────
  depends:
    - ../auth/sessions.md
    - ../crypto/tokens.md

  # ─────────────────────────────────────────────────────────────
  # Subsystem 2: Discovery Graph (Human Associative Links)
  # • Skipped by default in 'ods context' (opt-in via --include-related)
  # • Cycles allowed (e.g. Doc A <-> Doc B)
  # ─────────────────────────────────────────────────────────────
  related:
    - ../policy/refund-sla.md

  # ─────────────────────────────────────────────────────────────
  # Subsystem 3: Asset Catalog (Disk-level Non-Markdown Files)
  # • Verified for disk existence by 'ods lint'
  # • NOT loaded into LLM prompts by default (protects token limits)
  # ─────────────────────────────────────────────────────────────
  resources:
    - path: ../diagrams/refund-flow.pdf        # Binary PDF diagram (verified, not in prompt)

  # ─────────────────────────────────────────────────────────────
  # Subsystem 4: Code Bindings (Implementation & Tests)
  # • Binds document to source code without line numbers
  # ─────────────────────────────────────────────────────────────
  code:
    - path: apps/billing/src/refund.ts
      role: entrypoint
      symbol: processRefund
    - path: apps/billing/tests/refund.test.ts
      role: test
      symbol: TestProcessRefund

  # ─────────────────────────────────────────────────────────────
  # Subsystem 5: AI Prompt Bounds & Inclusions (Surgical Scoping)
  # • Explicitly injected into the LLM context bundle
  # ─────────────────────────────────────────────────────────────
  context:
    max-depth: 2                              # Recursion depth along 'depends'
    load:
      - ../schemas/refund-request.json        # Injects data schema into AI prompt
    ignore:
      - archive/                              # Prunes noisy path prefixes
---

# Document Title Lives Only in the H1 Body Heading
```

---

## 3. The 3-Tier Layering Architecture

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ TIER 1: Universal Top-Level Keys (Common Metadata)                      │
│ description, tags, owner, created, updated                             │
│ -> Visible to all YAML consumers (Hugo, Astro, Docusaurus, Obsidian)    │
├─────────────────────────────────────────────────────────────────────────┤
│ TIER 2: ODS Engine Keys (Scoped under ods:)                            │
│ profile, status, id, share, depends, related, resources, code, context │
│ -> Scoped to prevent collision with SSG reserved template variables     │
├─────────────────────────────────────────────────────────────────────────┤
│ TIER 3: Workspace Manifest Keys (In root ods.toml only)                │
│ spec, ignore, custom_profiles, packs, aliases, specs, service           │
│ -> Repository-wide boundary and discovery configuration                 │
└─────────────────────────────────────────────────────────────────────────┘
```

| Layer | Placement | Target Consumers | Purpose |
| :--- | :--- | :--- | :--- |
| **1. Universal** | **Top-level only** | Any YAML consumer (SSGs, CMSs, search indexers, Obsidian) | Common metadata that should be universally readable. |
| **2. ODS Engine** | **Under `ods:` map only** | ODS CLI, linters, AI agents, context builders | Engine-specific metadata for DAG edges, assets, and AI context. |
| **3. Workspace** | **Root `ods.toml` only** | ODS runtime, CI runners, build systems | Global repository settings, ignore patterns, and pack imports. |

---

## 4. Subsystem Matrix of Engine Keys

| Key | Engine Subsystem | Auto-loaded in `ods context`? | Verified by `ods lint`? | Key Purpose |
| :--- | :--- | :---: | :---: | :--- |
| **`ods.depends`** | Knowledge Graph | **Yes** (up to `max-depth`) | **Yes** (strict DAG, no cycles) | Hard structural prerequisites. |
| **`ods.related`** | Discovery Graph | **No** (opt-in via flag) | **Yes** (path must exist) | Soft associative reading. |
| **`ods.resources`** | Asset Inventory | **No** (static metadata) | **Yes** (file must exist on disk) | Physical non-Markdown attachments (PDF, CSV). |
| **`ods.code`** | Code Bindings | **Optional** (`--with-code`) | **Yes** (path exists, valid role, no `:L45`) | Links to implementation, tests, and infra. |
| **`ods.context.load`**| AI Prompt Scoping | **Yes** (injected directly) | **Yes** (file must exist on disk) | Auxiliary JSON schemas, CSVs, and fixtures. |
| **`ods.context.max-depth`**| Traversal Bound | Governs recursion limit | **Yes** (integer $\ge 0$) | Max graph distance to follow `depends`. |
| **`ods.context.ignore`**| Scoping Boundary | Filters expansion queue | **Yes** (list of prefixes) | Path prefixes pruned during traversal. |

---

## 5. Canonical Emit Ordering

When scaffolding (`ods new`), adopting (`ods adopt`), or formatting (`ods fmt`), tools MUST emit keys inside the `ods:` map in this exact sequence:

$$\text{profile} \longrightarrow \text{status} \longrightarrow \text{id} \longrightarrow \text{share} \longrightarrow \text{depends} \longrightarrow \text{related} \longrightarrow \text{resources} \longrightarrow \text{code} \longrightarrow \text{context}$$

> **Note**: Parsers MUST accept engine keys in any order; only emit/formatting tooling enforces the canonical sequence.

---

## 6. Tier 1: Universal Top-Level Keys

Universal keys MUST appear at the top level of frontmatter. They MUST NOT be placed under `ods:`.

### 6.1 `description`
- **Type**: `string`
- **Purpose**: A concise, one-sentence summary of the document used in index listings, search previews, and AI tool calling descriptions.
- **Normative Rules**: SHOULD be between 10 and 200 characters. MUST NOT contain multi-paragraph markdown prose.

```yaml
# VALID: Clear, concise one-line summary
description: Step-by-step instructions for issuing customer credit card refunds.

# INVALID: Markdown prose in description field
description: |
  # Refund Guide
  This is a full paragraph explaining the historical background of refunds...
```

### 6.2 `tags`
- **Type**: `list of strings`
- **Purpose**: Free-form categorical keywords for multi-dimensional filtering, search facets, and team taxonomy.
- **Normative Rules**:
  - MUST be declared at the top level. MUST NOT be nested under `ods:`.
  - Tools MUST normalize tags to lowercase `a-z`, `0-9`, `-`.
  - Tags SHOULD NOT collide with status names (`draft`, `stable`) or profile names (`guide`, `decision`).

```yaml
# VALID: Top-level tags array
tags:
  - billing
  - customer-care
  - refunds

# INVALID: tags placed under ods:
ods:
  tags: [billing, refunds]  # INVALID: tags MUST NOT be nested under ods:
```

### 6.3 `owner`
- **Type**: `string` or `list of strings`
- **Purpose**: Identifies the individual or team accountable for maintaining the accuracy of the document.
- **Normative Rules**: RECOMMENDED format is a GitHub team slug (e.g. `team:billing`), individual handle (`alice`), or email.

```yaml
# VALID (Single team ownership)
owner: "team:payments"

# VALID (Multiple individual maintainers)
owner:
  - alice
  - bob
```

### 6.4 `created` and `updated`
- **Type**: `string` (`YYYY-MM-DD` or ISO-8601 format)
- **Purpose**: Optional human-readable timestamps for workflows where Git history is unavailable or flattened (e.g. exported static sites).
- **Normative Rules**: In Git-native environments, authors SHOULD rely on Git commit timestamps rather than manually maintaining these fields. `last_updated` is recognized as an alias of `updated`.

---

## 7. Tier 2: ODS Engine Keys (Nested under `ods:`)

All engine keys MUST be nested inside the `ods:` mapping.

### 7.1 `ods.profile`
- **Type**: `string` (default: `"note"`)
- **Purpose**: Declares the structural shape and expected H2 or H3 sections (`##` or `###`) of the document.
- **Values**: Standard profiles (`note`, `guide`, `feature`, `decision`, `sop`, `api`, `architecture`, `policy`, `meeting`, `faq`, `checklist`, `agent`, `skill`) or custom profiles registered in `ods.toml`. See [profiles.md](profiles.md).

```yaml
# VALID: Scoped under ods:
ods:
  profile: agent

# INVALID: Placed at top level
profile: agent  # INVALID: engine key must be under ods:
```

### 7.2 `ods.status`
- **Type**: `enum` (default: `"draft"`)
- **Purpose**: Declares the document's lifecycle maturity.
- **Allowed Values**:
  - `draft`: Work-in-progress; content may be incomplete or actively changing.
  - `stable`: Verified, authoritative documentation ready for general consumption.
  - `deprecated`: Outdated knowledge; superseded by newer documentation.
  - `archived`: Historical record preserved for auditing; no longer actively maintained.

```yaml
# VALID: Standard lifecycle state
ods:
  profile: guide
  status: stable

# INVALID: Non-standard status value
ods:
  status: in-review  # INVALID: must be one of [draft, stable, deprecated, archived]
```

### 7.3 `ods.id`
- **Type**: `string` (default: workspace-relative path without `.md`)
- **Purpose**: Explicit document identifier override used for **rename stability**.
- **Normative Rules**: Authors SHOULD omit this field and rely on automatic path-derived IDs unless preserving external links during a major directory restructuring.

```yaml
# VALID: Explicit ID to preserve legacy link identity across file moves
ods:
  id: docs/v1/auth-setup
  profile: guide
```

### 7.4 `ods.share`
- **Type**: `enum` (default: `"public"`)
- **Purpose**: Visibility control for context export filtering and prompt boundary protection.
- **Allowed Values**:
  - `public`: Safe for public distribution, external search, and unprivileged AI prompts.
  - `org`: Internal to the organization / repository team.
  - `private`: Sensitive or confidential; MUST be excluded from context exports unless explicitly requested.

```yaml
# VALID: Marked as private to prevent AI context leakage
ods:
  profile: guide
  share: private
```

### 7.5 `ods.depends`
- **Type**: `list of strings` (relative file paths)
- **Subsystem**: **Knowledge Graph (Structural Prerequisites)**
- **Purpose**: Hard directional prerequisites. The reader or AI agent MUST understand the target document(s) before acting on this document.
- **Normative Rules**:
  - MUST point to resolvable `.md` file paths relative to the current document.
  - The dependency graph MUST NOT contain cycles.
  - **Duplication Rule**: Targets listed in `depends` are automatically traversed during `ods context`; they MUST NOT be duplicated in `context.load`.

```yaml
# VALID: Clear directional prerequisites
ods:
  depends:
    - ../auth/sessions.md
    - ../crypto/key-rotation.md
```

### 7.6 `ods.related`
- **Type**: `list of strings` (relative file paths)
- **Subsystem**: **Discovery Graph (Human Associative Links)**
- **Purpose**: Soft associative references and suggested further reading.
- **Normative Rules**:
  - AI agents MAY optionally expand related links if token budget allows (`--include-related`).
  - Cycles in `related` are permitted (e.g. Doc A points to Doc B, and Doc B points to Doc A).

```yaml
# VALID: Soft related reading
ods:
  related:
    - ../policy/refund-sla.md
```

### 7.7 `ods.resources`
- **Type**: `list of maps` containing a required `path` string.
- **Subsystem**: **Asset Catalog (Disk Inventory)**
- **Purpose**: Non-Markdown attachments associated with the document (diagrams, PDFs, CSVs, OpenAPI specs).
- **Normative Rules**:
  - Paths MUST resolve to real files on disk.
  - **Token Protection Rule**: Resources are NOT automatically loaded into LLM prompts by default. To inject a specific text/JSON resource into the prompt, declare it in `context.load`.
  - Source code MUST NOT be listed here (use `ods.code`).

```yaml
# VALID: Disk attachments (verified by lint, not auto-dumped into prompt)
ods:
  resources:
    - path: ../diagrams/auth-flow.png
    - path: ../contracts/payments-v2.openapi.yaml
```

### 7.8 `ods.code`
- **Type**: `list of maps` with `path` (`string`, required), `role` (`enum`, required), and `symbol` (`string` or `list of strings`, optional).
- **Subsystem**: **Code Bindings**
- **Purpose**: Binds the document to implementation source code, test suites, infrastructure definitions, and CI pipelines.
- **Allowed Roles**: `entrypoint`, `implementation`, `test`, `schema`, `migration`, `config`, `infrastructure`, `pipeline`.
- **Normative Rules**:
  - `path` MUST NOT contain line numbers (e.g. `:L42` is prohibited).
  - Unknown roles are invalid and MUST trigger a lint error.

```yaml
# VALID: Refactor-resilient symbol binding
ods:
  code:
    - path: apps/web/src/routes/refund.tsx
      role: entrypoint
      symbol: RefundRoute
    - path: apps/web/src/features/refunds/process.ts
      role: implementation
      symbol:
        - processRefund
        - validateRefundAmount
```

### 7.9 `ods.context`
- **Type**: `map` containing optional `max-depth` (`integer`), `load` (`list of strings`), and `ignore` (`list of strings`).
- **Subsystem**: **AI Prompt Bounds & Inclusions**
- **Purpose**: Declares a deterministic bounded reading list for AI agent prompt assembly.
- **Normative Rules**:
  - `load`: Injects auxiliary JSON schemas, CSVs, or specific documents directly into the prompt.
  - `max-depth`: Governs graph recursion depth (default: 2).
  - `ignore`: Prunes path prefixes during traversal.

```yaml
# VALID: Bounded AI context scope
ods:
  context:
    max-depth: 2
    load:
      - ../schemas/sample-payload.json
    ignore:
      - legacy/
```

---

## 8. Custom Profile Definition Keys

The following keys are allowed under `ods.custom_profile` in a registered custom profile-definition Markdown file. They describe the profile schema; they are not ordinary document engine keys.

| Key | Placement | Type | Purpose |
| :--- | :--- | :--- | :--- |
| `name` | `ods.custom_profile.name` | string, optional | Profile identifier. If omitted, the profile file stem is used. |
| `required_keys` | `ods.custom_profile.required_keys` | list of strings, optional | Names of top-level document keys required when the profile is selected. |
| `optional_keys` | `ods.custom_profile.optional_keys` | list of strings, optional | Names of useful top-level document keys that are not required. |
| `forbidden_keys` | `ods.custom_profile.forbidden_keys` | list of strings, optional | Names of top-level document keys that should not appear with the profile. |

`ods.custom_profile` is valid only in a registered profile-definition file selected by `custom_profiles` (or a registered pack). It is not copied into documents using the profile and does not make third-party metadata globally required. Tools MUST reject the block in any other document. See [profiles.md](profiles.md#711-profile-definition-metadata) for the complete contract.

Every `custom_profiles` path in `ods.toml` MUST exist at the exact configured location. A missing path, a non-Markdown file, or invalid profile-definition frontmatter is a `PROF-005` error. An `ods.profile` value that does not resolve to a standard or loaded custom profile is a `PROF-001` error; the diagnostic MUST identify the configured profile paths.

```yaml
ods:
  custom_profile:
    name: incident
    required_keys:
      - github-issue
      - service
```

`required_keys`, `optional_keys`, and `forbidden_keys` are optional lists of top-level key names. Add one `-` entry for each key. If a list has no entries, omit that profile-definition key; `[]` is valid YAML for an explicitly empty list but is not required.

```yaml
# INVALID: profile-definition keys must be grouped under custom_profile
ods:
  profile: custom-profile
  required_keys:
    - github-issue
```

In a document using `incident`, the required metadata remains top-level:

```yaml
github-issue: 123
service: checkout
ods:
  profile: incident
  status: draft
```

---

## Navigation & Reading Order

| [← Previous Chapter](core.md) | [📑 Specification Index](README.md) | [Next Chapter →](profiles.md) |
| :--- | :---: | ---: |
| **02. Core Format Model & Conformance** | **Open Document Spec (ODS)** | **04. Structural Profiles & Shapes** |
