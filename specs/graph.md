---
description: "Document identity, path-derived IDs, single source of truth, depends/related graph edges, DAG cycle prevention, and knowledge graph purity."
ods:
  profile: "note"
  status: "stable"
  depends:
    - README.md
    - keys.md
  related:
    - context.md
    - validation.md
    - core.md
    - ../guides/03-link-documents.md
---

# ODS · Document Graph & Identity

This document specifies the **ODS Document Graph**: document identity, path-derived IDs, single source of truth rules, graph edge types (`depends` / `related`), DAG cycle prevention, and the principle of **Knowledge Graph Purity**.

## At a glance

- **What this chapter defines:** Path-derived IDs, `depends` vs `related`, acyclicity, and document-only purity.
- **Why it exists:** Tools can only lint and walk edges that are explicit and well-typed.
- **When you need it:** You are linking documents, debugging a cycle, or implementing graph validation.
- **When you can skip it:** Isolated documents with no prerequisites — you do not need a graph yet.
- **Learn this first:** [Link documents](../guides/03-link-documents.md)
- **Prerequisite chapters:** [keys.md](keys.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt), [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt)) when, and only when, they appear in all capitals.

---

## 2. Document Identity: IDs are Paths

Every ODS document has a unique identifier within its workspace.

### 2.1 Default: Path-Derived ID
By default, a document's ID **is its workspace-relative file path without the `.md` extension**, normalized using forward slash (`/`) separators:

```text
File Path on Disk                  Workspace Document ID
docs/guides/setup.md           →   docs/guides/setup
features/billing/checkout.md   →   features/billing/checkout
README.md                      →   README
```

- **Deterministic & Zero-Config**: Authors do not need to invent arbitrary UUIDs, database slugs, or hash strings.
- **Normalization**: IDs are case-insensitive. Tools MUST normalize paths to lowercase `a-z`, `0-9`, `-`, `_`, and `/`.
- **Cross-Platform Separators**: Path separators MUST always be normalized to `/` across macOS, Linux, and Windows.

### 2.2 Explicit Override: `ods.id`
An explicit `ods.id` field MAY be set in frontmatter to override the path-derived ID:

```yaml
---
ods:
  # Overrides default path-derived ID for rename stability
  id: architecture/auth-v2
  profile: architecture
  status: stable
---
```

- **When to Use**: Use `ods.id` primarily for **rename stability** when reorganizing heavily referenced legacy documents without immediately cascading link rewrites.
- **Uniqueness**: All document IDs (path-derived or explicit) MUST be unique across the workspace. Duplicate IDs MUST trigger a validation error.

---

## 3. The Dual-Graph Architecture

ODS 1.1 explicitly formalizes and connects two complementary graph topologies across the workspace:

```text
                     ┌────────────────────────────────┐
                     │          DOMAIN GRAPH          │
                     │  (Entities & Conceptual Links) │
                     │   [Customer] --buys--> [Plan]  │
                     └───────────────┬────────────────┘
                                     │
                             grounded_in / cites
                                     │
                     ┌───────────────▼────────────────┐
                     │         LEXICAL GRAPH          │
                     │  (Documents, AST & Embeddings) │
                     │   [Doc.md] -> [H2 Chunk: AST]  │
                     └────────────────────────────────┘
```

1. **Domain Graph**: Captures the real-world business and system semantics:
   - Entities (`ods.entity`, `ods.domain`).
   - Typed semantic relations (`ods.relations`: `is_a`, `part_of`, `owns`, `governed_by`, `maps_to`, `derives_from`).
   - Provenance sources (`sources` / `usage_window`).
2. **Lexical Graph**: Captures the physical documentation AST hierarchy:
   - Hard structural prerequisites (`ods.depends` — strict DAG).
   - Soft lateral discovery references (`ods.related` — cyclic allowed).
   - Non-markdown asset catalogs (`ods.resources`) and source code bindings (`ods.code`).

---

## 4. Directed Semantic Relations (Pareto 80/20: `ods.related`)

To enable rich neuro-symbolic reasoning without syntax overhead, documents declare directed relations using the **Pareto Rule** (`- <predicate>: <target>`) directly under `ods.related` (or `ods.relations`):

```yaml
ods:
  entity: Customer
  domain: Billing
  related:
    # Bare string (Implicit see_also lateral reference)
    - ../guides/billing-overview.md

    # Pareto Core Single-Key Shorthand
    - is_a: Account
    - owns: [Subscription, Invoice] # Multi-target array & Symbolic Entity resolution
    - part_of: PaymentGateway
    - governed_by: RefundPolicy
    - maps_to: datasets/bq-customers.sql
    - see_also: @faq.md

    # Attributed Relation Object (Only when edge metadata is needed)
    - predicate: manages
      target: @SupportTeam
      role: "Tier 1 Support"
      confidence: 0.98
      since: 2026-01-01
```

### 4.1 The 5 Pareto Core Verbs

| Predicate | Semantic Meaning | Graph Direction | Auto-Inferred Inverse | Example |
| :--- | :--- | :--- | :--- | :--- |
| `is_a` | Classification / Inheritance / Typing. | Subclass $\rightarrow$ Superclass | `superclass_of` | `- is_a: Account` |
| `part_of` | Structural composition / Modular component. | Child $\rightarrow$ Parent Container | `has_part` | `- part_of: PaymentGateway` |
| `owns` | Domain ownership / Lifecycle containment. | Owner $\rightarrow$ Owned Resource | `owned_by` | `- owns: [Subscription, Invoice]` |
| `governed_by` | Policy, SLA, or compliance rule enforcement. | Entity $\rightarrow$ Governing Policy | `governs` | `- governed_by: RefundPolicy` |
| `see_also` | Lateral discovery / associative reference. | Source $\rightarrow$ Target Reference | `see_also` (Symmetric) | `- see_also: @faq.md` |

*(Technical bindings like `maps_to`, `derives_from`, and `implements`, as well as custom domain `snake_case` verbs, are also supported).*

### 4.2 Auto-Inverse Edge Materialization
Authors ONLY write the natural forward relation. The ODS compiler automatically synthesizes reverse inbound edges in memory during workspace indexing:
- When Document A declares `owns: @DocB`, the graph index automatically records `DocB owned_by DocA`.
- When Document A declares `part_of: @DocB`, the graph index automatically records `DocB has_part DocA`.
- When Document A declares `governed_by: @DocB`, the graph index automatically records `DocB governs DocA`.

### 4.3 Attributed Edge Metadata
When relationships require machine-extracted scores or temporal bounds, use the expanded object shape:
- **`role`**: Semantic role or association title (e.g. `role: "Lead Maintainer"`).
- **`confidence`**: Machine extraction certainty score between `0.0` and `1.0`.
- **`since` / `until`**: ISO 8601 temporal activation window.
- **`cardinality`**: Edge constraint (`1`, `N`, `*`, `0..1`, `1..1`, `0..N`, `1..N`, `0..*`, `1..*`, `*..*`).

### 4.4 Symbolic Entity & Handle Resolution (`@handle`)
Authors can reference target entities and files directly using **`@` handles** instead of brittle relative paths (`../../billing/entities/subscription.md`):
- **Entity Handles**: `@Subscription` or `Subscription` auto-resolves to the document declaring `ods.entity: Subscription`.
- **File Handles**: `@tokens.md`, `@server.ts`, `@customer.schema.json` auto-resolve to unique files in the workspace.
- **Disambiguated Handles**: `@billing/index.md` disambiguates duplicate basenames using folder prefixes.

### 4.5 Workspace Symbol & Basename Indexing Algorithm
The ODS compiler indexes workspace symbols in two passes during discovery:
1. **Pass 1 (Entity & Basename Discovery)**: Scans all documents and builds memory-mapped lookup tables:
   - `entities: Map<Symbol, FilePath>` mapping PascalCase identifiers (`Subscription` $\rightarrow$ `entities/subscription.md`).
   - `basenames: Map<Basename, Set<FilePath>>` mapping unique file names (`server.ts` $\rightarrow$ `apps/api/src/server.ts`).
2. **Pass 2 (O(1) Resolution & Collision Detection)**:
   - If a handle has exactly one match, it resolves in $\mathcal{O}(1)$ time.
   - If zero matches exist, the linter emits `SYM-001: Unresolved @ handle`.
   - If multiple files share the basename without disambiguation, the linter emits `SYM-002: Ambiguous @ handle`.

---

## 5. Bi-Temporal Memory Traversal

When traversing agent memory nodes (`ods.tier: episodic`), the graph engine applies bi-temporal filtering:
- **`valid_from` / `valid_to`**: The real-world validity window. Facts where `now >= valid_to` are excluded from active context queries unless an explicit historical timestamp is requested (`ods memory query --at <iso-time>`).
- **`asserted_at`**: The instant the recording agent observed the fact.
- **`pin`**: Nodes marked `pin: true` are immune to automated decay and pruning.

---

## 6. Structural Edge Types (`depends` & `related`)

The Lexical Graph standardizes two directional edge types:
- **`ods.depends`**: Strict structural and conceptual prerequisites forming a **strict DAG** (participates in auto-expanded AI context).
- **`ods.related`**: Associative domain and discovery links (discovery graph, cycle-tolerant).

### 6.1 Pareto Prerequisite Verbs for `ods.depends`

```yaml
ods:
  depends:
    # Bare string (Default: Hard prerequisite)
    - @jwt-auth.md

    # Typed Prerequisite Child Verbs
    - requires: @database-setup.md       # Must be read/executed first
    - extends: @base-service-spec.md     # Base architectural specification
    - imports: @common-types.schema.json # Shared contract/type dependency

    # Attributed Dependency Object
    - predicate: requires
      target: @redis-cluster.md
      optional: false
      scope: runtime                     # compile | runtime | test
```

---

## 7. Knowledge Graph Purity (Normative)

A critical principle of ODS is **Knowledge Graph Purity**:

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ PRINCIPLE: KNOWLEDGE GRAPH PURITY                                       │
│ • 'ods.depends' expresses conceptual prerequisites between DOCUMENTS.   │
│ • Non-document fixtures (JSON schemas, sample CSVs, mock payloads) MUST │
│   NOT be placed in 'depends'.                                           │
│ • Why? Non-document files cannot participate in topological sort DAG    │
│   validation. Auxiliary test/prompt data belongs in 'context.load'.     │
└─────────────────────────────────────────────────────────────────────────┘
```

### Commented Comparison:
```yaml
# VALID: Pure Knowledge Graph dependencies + surgical prompt scoping
ods:
  # 1. Conceptual document dependencies (Participate in DAG topological sort)
  depends:
    - ../auth/sessions.md
    - ../crypto/jwt-spec.md

  # 2. Auxiliary prompt payload (Non-document fixtures for AI agent)
  context:
    load:
      - ../schemas/auth-payload.json

# INVALID: Corrupting the Knowledge Graph with non-document fixtures
ods:
  depends:
    - ../auth/sessions.md
    - ../schemas/auth-payload.json    # INVALID: JSON schema is not an ODS document!
```

---

## 8. DAG Validation & Cycle Prevention

The dependency graph formed by `ods.depends` edges MUST be a **Directed Acyclic Graph (DAG)**.

```mermaid
graph TD
    subgraph ValidDAG ["VALID: Acyclic Dependency Graph"]
        A["Checkout Guide (checkout.md)"] -->|"depends"| B["Auth Sessions (sessions.md)"]
        B -->|"depends"| C["Crypto Keys (keys.md)"]
    end

    subgraph InvalidCycle ["INVALID: Cyclic Dependency Loop"]
        X["Order Service (orders.md)"] -->|"depends"| Y["Payment Service (payments.md)"]
        Y -->|"depends"| Z["Ledger Service (ledger.md)"]
        Z -.->|"INVALID depends cycle"| X
    end
```

### 8.1 Cycle Detection Algorithm
1. Tooling performs a topological sort or Depth-First Search (DFS) traversal of all `ods.depends` edges across the workspace.
2. If any node path encounters a back-edge to an ancestor node in the active traversal stack, a cycle error is reported (`GRAPH-004`).
3. If two documents are mutually interdependent, one relationship MUST be changed to `ods.related` or refactored into a shared prerequisite document.

---

## 9. Single Source of Truth & Dynamic Backlinks

1. **Title Single Source of Truth**: In pure ODS, the document title exists as the first `# H1` in the prose body. Top-level `title:` is supported for Google OKF v0.2 interoperability.
2. **Relationships Live in Frontmatter**: Machine-readable dependencies live exclusively in `ods.depends` and `ods.related`.
3. **No Hand-Written Backlinks**: Authors MUST declare graph edges only on the dependent document. Inbound backlinks MUST be computed dynamically by tooling (`ods graph --backlinks`) and NEVER hand-maintained in frontmatter.

---

## 10. Design Decisions

### Why separate the Domain Graph from the Lexical Graph?
Document dependencies (`depends` / `related`) model how humans and agents read documents sequentially. Business relationships (`is_a`, `owns`, `maps_to`) model how data and domain entities interact. Separating the two into a **Dual-Graph** provides full neuro-symbolic expressiveness without adding friction to basic document authoring.

### Why forbid hand-written backlinks?
Maintaining bidirectional links manually (e.g. Doc A listing Doc B as child, and Doc B listing Doc A as parent) results in link rot whenever a file is renamed or moved. Computing backlinks on demand in tooling ensures 100% synchronization accuracy.

---

## Navigation & Reading Order

| [← Previous Chapter](profiles.md) | [📑 Specification Index](README.md) | [Next Chapter →](context.md) |
| :--- | :---: | ---: |
| **04. Structural Profiles & Shapes** | **Open Document Spec (ODS)** | **06. Bounded AI Context Scope** |
