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

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14, exactly as stated in [README.md §1](README.md#1-conformance-language). That is the canonical statement; do not maintain a second copy here.

---

## 1.1 Novice Quick Start: The 6 Canonical Document Recipes

Novice authors do NOT need to memorize the full key dictionary — or any of it. A plain Markdown file is already conformant ([core.md §3.0](core.md#30-minimal-conformant-document)). Beyond that floor, ODS standardizes **6 Plug-and-Play Recipes** covering day-to-day use:

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                    THE 6 CANONICAL ODS 1.1 RECIPES                      │
├───────────────────┬─────────────────────────────────────────────────────┤
│ 1. DAILY DOC      │ description + tags + ods.profile + ods.status       │
│ 2. LINKED DOC     │ Recipe 1 + ods.depends / ods.related                │
│ 3. CODE BINDING   │ Recipe 2 + ods.code (shorthand or symbol list)      │
│ 4. DOMAIN ENTITY  │ Recipe 1 + ods.entity + ods.domain + ods.related     │
│ 5. AGENT MEMORY   │ Recipe 1 + memory: { tier, mutations }              │
│ 6. ATTESTED COMP  │ type: Attested Computation + runtime + executor     │
└───────────────────┴─────────────────────────────────────────────────────┘
```

### The Novice "I Want To..." Decision Matrix

| I Want To… | Use This Profile | Required Keys | Optional Keys |
| :--- | :--- | :--- | :--- |
| **Write a setup guide or tutorial** | `profile: guide` | `description`, `tags`, `ods.status` | `ods.depends`, `ods.related` |
| **Record an architectural decision** | `profile: decision` | `description`, `tags`, `ods.status` | `ods.depends` |
| **Link docs to real source code** | Any profile | `ods.code: ["src/file.ts"]` | `symbol`, `role`, `description` |
| **Define a business domain entity** | `profile: note` (or `feature`) | `ods.entity`, `ods.domain` | `ods.schema`, `ods.related` (typed), `ods.invariants` |
| **Save an agent interaction trace** | `profile: note` | `memory: { tier: episodic, mutations }` | `memory.valid_from`, `memory.pin` |
| **Run a verifiable SQL computation** | `profile: note` | `type: Attested Computation`, `runtime` | `parameters`, `executor`, `attester` |

Teaching path: [Your first document](../guides/01-first-document.md). Pocket form: [Decision cards](../guides/decision-cards.md).

---

## 1.2 Universal `@` Symbolic Handle Resolution

Every path-bearing key (`depends`, `related`, `schema`, `code[].path`, `resources`) accepts **`@` handles** in place of brittle relative paths: `@Subscription` resolves to the document declaring that entity, `@tokens.md` to the uniquely-named file, `@billing/index.md` to a disambiguated one.

```yaml
ods:
  schema: "@customer.schema.json"
  depends:
    - "@tokens.md"
  related:
    - owns: "@Subscription"
  code:
    - path: "@refund.ts"
      symbol: processRefund
```

Canonical resolution rules, the two-pass indexing algorithm, and the `SYM-001`/`SYM-002` collision cases: [graph.md §4.4–4.5](graph.md#44-symbolic-entity--handle-resolution-handle). Do not maintain a second copy here.

---

## 2. Author Cheat Sheet (Copy-Paste Reference)

```yaml
---
# ═════════════════════════════════════════════════════════════════
# LAYER 1: UNIVERSAL & OKF NATIVE KEYS (Visible to all YAML/OKF tools)
# ═════════════════════════════════════════════════════════════════
description: One-line summary for search previews, AI tool calls, and index listings.
tags:
  - billing
  - customer-care
owner: team:support # Responsible team, role, or on-call group
author: Alice Smith # Individual author, creator, or agent
created_at: 2026-01-15 # created or created_at (bidirectional alias)
updated_at: 2026-08-14 # updated or updated_at (bidirectional alias)
type: BigQuery Table # OKF v0.2 concept type / profile alias
sources:
  - id: bq-schema
    resource: datasets/billing/customers.sql
    author: team:data-platform
    usage_count: 5000
    last_modified: 2026-08-20T00:00:00Z
verified:
  - { by: "human:ahormati", at: "2026-08-22T00:00:00Z" }
stale_after: 2026-12-31T00:00:00Z

# Encapsulated Cognitive Memory Block (Optional)
memory:
  tier: episodic # episodic | semantic | procedural | state
  valid_from: 2026-08-26T00:00:00Z
  valid_to: null # null = currently active reality
  asserted_at: 2026-08-26T00:05:00Z
  pin: true # Protects from decay pruning
  mutations:
    - entity: Customer
      id: cust-4048
      property: billing_plan
      old_value: "starter"
      new_value: "enterprise"

# ═════════════════════════════════════════════════════════════════
# LAYER 2: FLAT ODS ENGINE KEYS (Direct under ods:, zero extra nesting!)
# ═════════════════════════════════════════════════════════════════
ods:
  profile: guide # Document shape / expected H2 or H3 sections
  status: stable # Lifecycle maturity: draft | stable | deprecated | archived
  share: public # Privacy boundary: public | org | private

  # ─────────────────────────────────────────────────────────────
  # Subsystem 1: Domain Modeling & Entities (Direct Pareto Keys)
  # ─────────────────────────────────────────────────────────────
  entity: Customer # Canonical entity class name
  domain: Billing # Business domain partition
  schema: schemas/customer.schema.json # "Paid at the door" disk schema validator
  invariants:
    - "mrr >= 0"
    - "email is required"

  # ─────────────────────────────────────────────────────────────
  # Subsystem 2: Knowledge Graph (Structural Prerequisites)
  # • Auto-traversed by 'ods context' up to max-depth (default: 2)
  # • Strict DAG: Cycles are forbidden (checked by 'ods lint')
  # ─────────────────────────────────────────────────────────────
  depends:
    - ../auth/sessions.md
    - ../crypto/tokens.md

  # ─────────────────────────────────────────────────────────────
  # Subsystem 3: Discovery Graph & Directed Relations (Pareto 80/20)
  # • Simple string paths AND Pareto '- predicate: target' pairs!
  # ─────────────────────────────────────────────────────────────
  related:
    - is_a: Account
    - owns: [Subscription, Invoice] # Multi-target array & Symbolic Entity resolution
    - governed_by: RefundPolicy
    - maps_to: datasets/bq-customers.sql
    - ../policy/refund-sla.md # Simple lateral reading doc

  # ─────────────────────────────────────────────────────────────
  # Subsystem 4: Asset Catalog (Disk-level Files & URLs)
  # • Verified for disk existence by 'ods lint'
  # ─────────────────────────────────────────────────────────────
  resources:
    - ../diagrams/refund-flow.pdf # Local file shorthand
    - https://figma.com/file/refund-ui-mockup # External design URL

  # ─────────────────────────────────────────────────────────────
  # Subsystem 5: Code Bindings (Implementation & Tests)
  # • Strings, symbol arrays, and descriptions supported!
  # ─────────────────────────────────────────────────────────────
  code:
    - apps/billing/src/refund.ts # Simple string shorthand (role: implementation)
    - path: apps/billing/tests/refund.test.ts
      role: test
      symbol:
        - TestProcessRefund
        - TestRefundTaxCalculation
      description: "Verifies refund calculations and tax adjustments."

  # ─────────────────────────────────────────────────────────────
  # Subsystem 6: Context Bounds & Inclusions (Surgical Prompt Scoping)
  # ─────────────────────────────────────────────────────────────
  context:
    max-depth: 2
    load:
      - ../schemas/refund-request.json
    ignore:
      - archive/
---
# Document Title Lives Only in the H1 Body Heading
```

---

## 3. The 3-Layer Key Placement Architecture

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ LAYER 1: Universal & OKF Native Keys (Common Metadata)                  │
│ description, tags, owner, created, updated, type, title, resource,      │
│ sources, usage_window, generated, verified, status, stale_after,        │
│ runtime, parameters, computation, executor, attester, memory            │
│ -> Visible to all YAML consumers (Hugo, Astro, Docusaurus, OKF tools)   │
├─────────────────────────────────────────────────────────────────────────┤
│ LAYER 2: Flat ODS Engine Keys (Scoped under ods:)                       │
│ profile, status, id, share, entity, domain, schema, invariants,         │
│ depends, related, resources, code, context, custom_profile              │
│ deprecated: relations, memory, tier, valid_from, valid_to,              │
│             asserted_at, mutations, pin                                 │
│ -> Flat engine metadata with zero unnecessary indentation wrappers      │
├─────────────────────────────────────────────────────────────────────────┤
│ LAYER 3: Workspace Manifest Keys (In root ods.toml only)                │
│ spec, dialect, ignore, custom_profiles, packs, schemas, [aliases],      │
│ [ontology], [memory], [okf], [attestation], [service]                   │
│ -> Repository-wide boundary and discovery configuration                 │
│ -> Full key reference: indexes.md §3                                    │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Subsystem Matrix of Engine Keys

| Key                         | Engine Subsystem   | Auto-loaded in `ods context`? |           Verified by `ods lint`?            | Key Purpose                                      |
| :-------------------------- | :----------------- | :---------------------------: | :------------------------------------------: | :----------------------------------------------- |
| **`ods.entity`**            | Ontology Subsystem |   **Yes** (Identity header)   |          **Yes** (valid identifier)          | Class/concept name in Domain Graph.              |
| **`ods.domain`**            | Ontology Subsystem |    **Yes** (Domain header)    |          **Yes** (valid identifier)          | Business domain boundary partition.              |
| **`ods.schema`**            | Ontology Subsystem |    **Yes** (Schema shapes)    |        **Yes** (path exists on disk)         | "Paid at the door" disk schema contract.         |
| **`ods.relations`** *(deprecated)* | Ontology Subsystem |  Merged into `related`  |     **Yes** (targets exist, valid enum) + `DEPR-001`     | Superseded by `ods.related`. See [scope.md §7.2](scope.md#72-deprecated-in-11--scheduled-for-removal-in-20). |
| **`ods.invariants`**        | Ontology Subsystem |   **Yes** (Guardrail list)    |        **Yes** (parseable expression)        | Deterministic boolean refusal rules.             |
| **`memory.tier`**           | Memory Subsystem   |    **Yes** (Memory filter)    |          **Yes** (valid tier enum)           | Cognitive memory classification (5 tiers).       |
| **`memory.valid_from`**     | Memory Subsystem   |   **Yes** (Temporal filter)   |              **Yes** (ISO-8601)              | Real-world validity start instant.               |
| **`memory.valid_to`**       | Memory Subsystem   |   **Yes** (Staleness gate)    |     **Yes** (valid_to $\ge$ valid_from)      | Real-world expiration instant (`null` = active). |
| **`memory.asserted_at`**    | Memory Subsystem   |   **Yes** (Assertion time)    |              **Yes** (ISO-8601)              | Timestamp when the agent recorded the fact.      |
| **`memory.mutations`**      | Memory Subsystem   |     **Yes** (State delta)     |     **Yes** (entity/id/property present)     | Graphiti-style structured attribute changes.     |
| **`memory.pin`**            | Memory Subsystem   |     **No** (Pruning flag)     |              **Yes** (boolean)               | Protects from automated decay pruning.           |
| **`ods.depends`**           | Knowledge Graph    |  **Yes** (up to `max-depth`)  |       **Yes** (strict DAG, no cycles)        | Hard structural prerequisites.                   |
| **`ods.related`**           | Discovery Graph    | **Titles only** (1-line index; bodies are opt-in) |    **Yes** (see `GRAPH-003` / `ENT-001`)     | Soft associative reading and typed domain edges. |
| **`ods.resources`**         | Asset Inventory    |  **No** (catalog metadata only) |     **Yes** (local files must exist; URLs syntax-only)      | Physical non-Markdown attachments (PDF, CSV).    |
| **`ods.code`**              | Code Bindings      | **Opt-in** (caller requests code) | **Yes** (path exists, valid role, no `:L45`) | Links to implementation, tests, and infra.       |
| **`ods.context.load`**      | AI Prompt Scoping  |  **Yes** (injected directly)  |      **Yes** (file must exist on disk)       | Auxiliary JSON schemas, CSVs, and fixtures.      |
| **`ods.context.max-depth`** | Traversal Bound    |    Governs recursion limit    |        **Yes** (integer `0`–`10`)            | Max graph distance to follow `depends`.          |
| **`ods.context.trust-min`** | Trust Gate         |    Filters by trust tier      |          **Yes** (valid tier enum)           | Minimum verification level for inclusion.        |
| **`ods.context.ignore`**    | Scoping Boundary   |    Filters expansion queue    |          **Yes** (list of prefixes)          | Path prefixes pruned during traversal.           |

> This matrix is the canonical statement of auto-load and lint behavior per key. [context.md §3](context.md#3-subsystem-summary-matrix) defers to it; do not maintain a second copy there.

---

## 5. Canonical Emit Ordering

When scaffolding (`ods new`), adopting (`ods adopt`), or formatting (`ods fmt`), tools MUST emit keys inside the `ods:` map in this exact sequence:

**Top-level keys**, in order:

`$schema` → `description` → `tags` → `owner` → `author` → `created_at` → `updated_at` → OKF keys (`type`, `title`, `resource`, `sources`, `usage_window`, `generated`, `verified`, `status`, `stale_after`, `runtime`, `parameters`, `computation`, `executor`, `attester`, `okf_version`) → `memory` → `ods`

**Keys inside the `ods:` map**, in order:

`profile` → `status` → `id` → `share` → `entity` → `domain` → `schema` → `invariants` → `depends` → `related` → `resources` → `code` → `context` → `custom_profile`

> **Note**: Parsers MUST accept engine keys in any order; only emit/formatting tooling enforces the canonical sequence.
>
> Deprecated keys (`ods.relations`, `ods.memory`, and the flat `ods.tier` / `valid_from` / `valid_to` / `asserted_at` / `mutations` / `pin`) are **not** part of the canonical order. A formatter MUST preserve them where it finds them rather than reordering them into the sequence above, and SHOULD attach the matching `DEPR-*` diagnostic.

---

## 6. Layer 1: Universal Top-Level Keys

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
  tags: [billing, refunds] # INVALID: tags MUST NOT be nested under ods:
```

### 6.3 `owner` and `author`

- **`owner`** (`string`): Identifies the accountable team, role, or on-call group maintaining the accuracy of the document (e.g. `owner: "team:payments"`).
- **`author`** (`string`): Identifies the individual author, creator, or agent who wrote this document or revision (e.g. `author: "Alice Smith"`).
- **Normative Rules**:
  - Both `owner` and `author` are first-class universal keys. Documents MAY specify either or both.

```yaml
# VALID: Both owner (team) and author (creator) specified
owner: "team:payments"
author: "Alice Smith"
```

### 6.4 `created` / `created_at` and `updated` / `updated_at`

- **Type**: `string` (ISO-8601 UTC timestamp or `YYYY-MM-DD` date)
- **Purpose**: Document lifecycle timestamps for static site generators, search indexes, and export manifests.
- **Bidirectional Aliases**:
  - `created` $\longleftrightarrow$ `created_at`: Interchangeable document creation timestamps.
  - `updated` $\longleftrightarrow$ `updated_at`: Interchangeable document last modified timestamps.
- **Normative Rules**:
  - Values SHOULD follow `YYYY-MM-DD` or full ISO-8601 UTC timestamp (`YYYY-MM-DDTHH:MM:SSZ`).
  - Parsers MUST accept both forms transparently (`doc.created = doc.created_at || doc.created`).

```yaml
# VALID: Standard date strings (both forms supported)
created_at: 2026-01-15
updated_at: 2026-08-14
```

### 6.5 `$schema` (Optional)

- **Type**: `string` (URI)
- **Purpose**: Enables real-time editor autocomplete, hover tooltips, and instant validation in VS Code, Cursor, JetBrains, and Zed.
- **Normative Rules**:
  - MUST be placed at the top level of frontmatter.
  - MAY reference the canonical URI (`https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json`) or a workspace-relative path.
  - `$schema` is OPTIONAL. Markdown documents without `$schema` remain 100% compliant ODS documents.

```yaml
# VALID: Editor schema binding
$schema: https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json
description: Guide for setting up user authentication.
tags: [auth, security]
ods:
  profile: guide
  status: stable
```

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
profile: agent # INVALID: engine key must be under ods:
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
  status: in-review # INVALID: must be one of [draft, stable, deprecated, archived]
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

- **Type**: `list of (strings OR dependency predicate maps OR attributed dependency objects)`
- **Subsystem**: **Knowledge Graph (Structural DAG Prerequisites)**
- **Purpose**: Hard directional prerequisites. The reader or AI agent MUST understand the target document(s) before acting on this document.
- **Entry Formats**:
  - **Bare String (Default)**: Path (`- ../auth/sessions.md`) or `@` handle (`- @sessions.md`).
  - **Typed Prerequisite Shorthand**: `- requires: @database-setup.md`, `- extends: @base-spec.md`, `- imports: @types.schema.json`.
  - **Attributed Map Object**: `{ predicate: requires, target: string, optional?: boolean, scope?: "compile" | "runtime" | "test" }`.
- **Normative Rules**:
  - MUST point to resolvable `.md` file paths or symbolic handles.
  - The dependency graph MUST be a strict **DAG** (MUST NOT contain cycles).
  - **Duplication Rule**: Targets listed in `depends` are automatically traversed during `ods context`; they MUST NOT be duplicated in `context.load`.

```yaml
# VALID: Clear directional prerequisites with Pareto child keys
ods:
  depends:
    - @jwt-auth.md                       # Bare string (implicit requires)
    - requires: @database-setup.md       # Typed prerequisite
    - extends: @base-service-spec.md     # Base architectural specification
    - imports: @common-types.schema.json # Shared contract/type dependency
```

### 7.6 `ods.related`

- **Type**: `list of (strings OR predicate maps OR attributed relation objects)`
- **Subsystem**: **Discovery Graph & Domain Relations (Pareto 80/20)**
- **Purpose**: Soft associative references, suggested reading, and directed semantic ontology relations.
- **The Pareto Rule for Relations**: `- <predicate>: <target>`
- **Entry Formats**:
  - **The 5 Pareto Core Verbs**: `- is_a: @Account`, `- part_of: @BillingEngine`, `- owns: [@Subscription, @Invoice]`, `- governed_by: @RefundPolicy`, `- see_also: @faq.md`.
  - **Bare String (Implicit `see_also`)**: Path to related document (`- ../policies/refund-sla.md`) or Symbolic Handle (`- @billing-faq.md`).
  - **Attributed Relation Object**: `{ predicate: string, target: string, role?: string, confidence?: float, since?: string, until?: string, cardinality?: string }`.
- **Allowed Predicates**: a **closed set** — `is_a`, `part_of`, `owns`, `governed_by`, `maps_to`, `depends_on`, `derives_from`, `implements`, `exercises`, `see_also`, plus the aliases `extends`, `contains`, `policy`, `rule`, `table`, `see`. A bare verb outside this set is rejected (`ENUM-006`); domain-specific verbs use `{ predicate: custom, custom_predicate: <verb> }`. Canonical vocabulary with inverses: [graph.md §4.1](graph.md#41-the-complete-predicate-vocabulary).
- **Normative Rules**:
  - Inbound inverse relations (`owned_by`, `has_part`, `governs`) are materialized automatically by compiler tooling.
  - Cycles in `related` are permitted.

```yaml
# VALID: Pareto 80/20 Unified related list (Strings + Predicate Maps + Attributed Objects)
ods:
  related:
    - @pricing-faq.md                   # Simple lateral reading doc (implicit see_also)
    - is_a: Account                     # Classification
    - part_of: BillingEngine            # Structural containment
    - owns: [Subscription, Invoice]     # Multi-target lifecycle ownership
    - governed_by: RefundPolicy         # Governance constraint
    - predicate: manages
      target: @DevTeam
      role: "Team Lead"
      confidence: 1.0
```

### 7.7 `ods.resources`

- **Type**: `list of (strings OR resource maps)`
- **Subsystem**: **Asset Catalog (Disk Files & URLs)**
- **Purpose**: Attachments associated with the document (diagrams, PDFs, CSVs, OpenAPI specs, Figma/Miro URLs).
- **Entry Formats**:
  - **Simple String**: Relative file path (`- ../diagrams/flow.png`) or external URL (`- https://figma.com/...`).
  - **Detailed Map**: `{ path?: string, url?: string, title?: string, description?: string }` — exactly one of `path` or `url` (`ASSET-005`).
- **Normative Rules**:
  - Local paths MUST resolve to real files on disk (`ASSET-001`). URL entries are syntax-checked only and MUST NOT be network-fetched — a lint run MUST succeed offline.
  - **Token Protection Rule**: Resources are NEVER automatically loaded into LLM prompts. To inject a specific text/JSON resource into the prompt, declare it in `context.load`.
  - Full entry-shape contract: [assets.md §5.1](assets.md#51-entry-shapes-normative).

```yaml
# VALID: Resource shorthand and detailed objects
ods:
  resources:
    - ../diagrams/auth-flow.png # Local file shorthand
    - https://figma.com/file/auth-flow-v2 # External design URL
    - path: ../contracts/payments-v2.openapi.yaml
      title: "Payments OpenAPI Spec"
      description: "API contract verified by CI."
```

### 7.8 `ods.code`

- **Type**: `list of (strings OR code binding maps)`
- **Subsystem**: **Code Bindings (Implementation & Tests)**
- **Purpose**: Binds the document to source code implementations, test suites, infrastructure definitions, and CI pipelines.
- **Allowed Roles** (10, closed set): `entrypoint`, `implementation` (default), `interface`, `test`, `fixture`, `schema`, `migration`, `config`, `infrastructure`, `pipeline`. Semantics for each: [assets.md §7](assets.md#7-the-10-standard-code-roles-reference).
- **Symbol Types**: `symbol` MAY be a single string (`symbol: startServer`) or a list of strings (`symbol: [TestA, TestB]`).
- **Normative Rules**:
  - `path` MUST NOT contain line numbers (e.g. `:L42` is prohibited).
  - When `code` entries are simple strings, `role` defaults to `implementation` (or is inferred as `test` for test fixtures).

```yaml
# Style 1: Pure String Shorthand (Zero boilerplate)
ods:
  code:
    - src/auth.service.ts
    - tests/auth.service.test.ts

# Style 2: Hybrid Mix (Targeted test symbols)
ods:
  code:
    - src/refunds/calculator.ts
    - path: tests/refunds/calculator.test.ts
      role: test
      symbol: TestRefundTaxCalculation

# Style 3: Explicit Full Objects with Symbol Arrays & Description
ods:
  code:
    - path: src/crypto/jwt.ts
      role: implementation
      symbol: signJwtToken
      description: "Primary JWT signing routine."
    - path: tests/crypto/jwt.test.ts
      role: test
      symbol:
        - TestJwtExpiration
        - TestInvalidSignatureRejection
      description: "Verifies token rejection when secret is expired or forged."
```

### 7.9 `ods.context`

- **Type**: `map`
- **Subsystem**: **AI Prompt Bounds & Inclusions**
- **Purpose**: Declares a deterministic bounded reading list for AI agent prompt assembly.

| Field | Type | Default | Meaning |
| :--- | :--- | :--- | :--- |
| `max-depth` | integer `0`–`10` | `2` | Hops to follow along `ods.depends`. `0` loads this document alone. Values above `10` are rejected (`ENUM-005`). |
| `trust-min` | enum | `unverified` | Minimum trust tier a document must reach to enter the payload. |
| `load` | list of strings | — | Auxiliary non-Markdown files (JSON schemas, CSVs, fixtures) injected directly into the prompt. |
| `ignore` | list of strings | — | Path prefixes pruned during traversal. |

#### Trust tiers (`trust-min`)

A document's trust tier is **derived from its `verified` entries**, not declared directly:

| Tier | Derived when | Meaning |
| :--- | :--- | :--- |
| `unverified` | No `verified` entries. | Nobody and nothing has confirmed this. |
| `machine-confirmed` | Every `verified[].by` names a process or agent (e.g. `agent:ci-linter`, `process:attester`). | A deterministic check passed. |
| `human-reviewed` | At least one `verified[].by` begins with `human:`. | A person signed off. |

Tiers are ordered `unverified` < `machine-confirmed` < `human-reviewed`. Setting `trust-min` filters out every document below that bar during context resolution — used when an agent is about to act on documentation rather than merely summarize it. Filtering is reported, not silent: a conformant engine MUST tell the caller which documents were dropped and why.

```yaml
# VALID: Bounded AI context scope
ods:
  context:
    max-depth: 2
    trust-min: human-reviewed   # do not act on unreviewed docs
    load:
      - ../schemas/sample-payload.json
    ignore:
      - legacy/
```

### 7.10 `ods.entity`

- **Type**: `string` (PascalCase identifier, minLength: 1)
- **Subsystem**: **Domain Ontology (Concepts & Classes)**
- **Purpose**: Declares the primary canonical business entity or system concept documented by this file.
- **Normative Rules**:
  - Used by the neuro-symbolic engine to construct the Domain Graph.
  - Entities participate in typed semantic relations (`ods.relations`).

```yaml
# VALID: Canonical entity declaration
ods:
  entity: Customer
  domain: Billing
```

### 7.11 `ods.domain`

- **Type**: `string` (PascalCase or lowercase identifier, minLength: 1)
- **Subsystem**: **Domain Ontology (Bounded Contexts)**
- **Purpose**: Declares the high-level business domain boundary or partition.
- **Normative Rules**:
  - Groups entities into bounded contexts (e.g. `Billing`, `Identity`, `Inventory`, `Support`).

```yaml
# VALID: Bounded context domain declaration
ods:
  entity: Subscription
  domain: Billing
```

### 7.12 `ods.schema`

- **Type**: `string` (relative file path on disk)
- **Subsystem**: **Domain Ontology (Validation Contracts)**
- **Purpose**: "Paid at the door" disk schema validator file (`.schema.json`, `.py`, `.ts`, `.zod.ts`).
- **Normative Rules**:
  - MUST resolve to an existing file on disk.
  - Validated by `ods lint` (`SCHEMA-001`).

```yaml
# VALID: Paid-at-the-door schema validator
ods:
  entity: Customer
  domain: Billing
  schema: ../schemas/customer.schema.json
```

### 7.13 `ods.relations` *(Deprecated)*

- **Status**: **Deprecated in 1.1** (`DEPR-001`). Removal targeted at 2.0.
- **Replacement**: [`ods.related`](#76-odsrelated), which accepts the same attributed-object entries plus the shorthand and bare-string forms.
- **Parser behavior**: Tools MUST still read `ods.relations`. Its entries are appended to `ods.related` and de-duplicated by `(predicate, target)`. Tools emit a `DEPR-001` warning and preserve the key on write unless the author runs a migration.

```yaml
# DEPRECATED: works, warns
ods:
  relations:
    - predicate: owns
      target: entities/subscription.md

# CANONICAL
ods:
  related:
    - owns: "@Subscription"
```

Rationale and the full deprecation schedule: [scope.md §7](scope.md#7-deprecations--versioning-policy).

### 7.14 `ods.invariants`

- **Type**: `list of strings` (boolean domain guardrail expressions)
- **Subsystem**: **Domain Ontology (Refusal Guardrails)**
- **Purpose**: Non-negotiable domain rules enforced during autonomous agent reasoning.
- **Normative Rules**:
  - Expressions evaluate to boolean truths (e.g. `"mrr >= 0"`, `"email is required"`).
  - Agents MUST NOT execute actions that violate declared invariants.

```yaml
# VALID: Domain invariants and refusal rules
ods:
  entity: Customer
  invariants:
    - "mrr >= 0"
    - "email is required and valid"
```

### 7.15 The `memory:` Block

Cognitive memory fields live in the **top-level `memory:` block**. `ods.memory:` and the flat `ods.tier` / `ods.valid_from` / `ods.valid_to` / `ods.asserted_at` / `ods.mutations` / `ods.pin` keys are deprecated (`DEPR-002`); parsers MUST still read them, with precedence `memory:` > `ods.memory:` > flat `ods.*`.

| Field | Type | Purpose |
| :--- | :--- | :--- |
| `tier` | enum: `episodic`, `semantic`, `procedural`, `state`, `profile` | Cognitive classification of the memory node. |
| `valid_from` | RFC 3339 timestamp | Instant the fact became true in the world. |
| `valid_to` | RFC 3339 timestamp or `null` | Instant it stopped being true. `null` (or absent) means still true. MUST NOT precede `valid_from` (`MEM-001`). |
| `asserted_at` | RFC 3339 timestamp | Instant the agent observed and recorded the fact. |
| `pin` | boolean (default `false`) | Protects the node from automated decay pruning. |
| `mutations` | list of maps | Structured attribute transitions: `entity`, `id`, `property` (all required), plus optional `old_value`, `new_value`, `confidence`. |

```yaml
# VALID: canonical memory block
memory:
  tier: episodic
  valid_from: "2026-08-26T10:00:00Z"
  valid_to: null
  asserted_at: "2026-08-26T10:05:00Z"
  pin: true
  mutations:
    - entity: Customer
      id: cust-4048
      property: plan
      old_value: "starter"
      new_value: "enterprise"
      confidence: 1.0
```

What each tier means, how bi-temporal filtering works during traversal, and why `state` and `profile` are distinct: [graph.md §5](graph.md#5-cognitive-memory--bi-temporal-traversal). That is the canonical definition; this table is the type reference only.

---

## 8. Custom Profile Definition Keys

The following keys are allowed under `ods.custom_profile` in a registered custom profile-definition Markdown file. They describe the profile schema; they are not ordinary document engine keys.

| Key                 | Placement                              | Type                      | Purpose                                                                   |
| :------------------ | :------------------------------------- | :------------------------ | :------------------------------------------------------------------------ |
| `name`              | `ods.custom_profile.name`              | string, optional          | Profile identifier. If omitted, the profile file stem is used.            |
| `description`       | `ods.custom_profile.description`       | string, optional          | One-line summary of what the profile is for, shown in tooling.            |
| `required_sections` | `ods.custom_profile.required_sections` | list of strings, optional | Canonical H2/H3 section names a document using the profile is expected to carry. Missing sections are a `PROF-002` warning. |
| `optional_sections` | `ods.custom_profile.optional_sections` | list of strings, optional | Sections recognized for the profile but never required.                   |
| `required_keys`     | `ods.custom_profile.required_keys`     | list of strings, optional | Names of top-level document keys required when the profile is selected.   |
| `optional_keys`     | `ods.custom_profile.optional_keys`     | list of strings, optional | Names of useful top-level document keys that are not required.            |
| `forbidden_keys`    | `ods.custom_profile.forbidden_keys`    | list of strings, optional | Names of top-level document keys that should not appear with the profile. |

Section names may also be declared as `##` headings in the profile-definition file's body, with `|` separating acceptable alternatives (see [profiles.md §7.1](profiles.md#71-custom-profile-definition-file-docsprofilesrfcmd)). The two forms are equivalent; a profile SHOULD use one or the other, not both. Where both are present, `required_sections` wins and tools SHOULD warn.

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

| [← Previous Chapter](core.md)           | [📑 Specification Index](README.md) |        [Next Chapter →](profiles.md) |
| :-------------------------------------- | :---------------------------------: | -----------------------------------: |
| **02. Core Format Model & Conformance** |    **Open Document Spec (ODS)**     | **04. Structural Profiles & Shapes** |
