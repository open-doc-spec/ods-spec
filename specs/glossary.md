---
description: "Normative glossary and terminology dictionary for Open Document Spec (ODS): 28 formal definitions across 7 domains, cross-chapter references, and concept disambiguation."
tags:
  - glossary
  - terminology
  - reference
  - specification
ods:
  profile: note
  status: stable
  depends:
    - README.md
  related:
    - core.md
    - keys.md
    - profiles.md
    - graph.md
    - context.md
    - assets.md
    - indexes.md
    - validation.md
    - scope.md
    - ../guides/README.md
---

# ODS · Terminology & Comprehensive Glossary

This document is the authoritative **Normative Terminology Reference** for Open Document Spec (ODS). It provides rigorous, unambiguous definitions for all 28 core concepts across the 7 specification domains, details frequently confused distinctions, and cross-references each term to its governing specification chapter.

## At a glance

- **What this chapter defines:** Formal definitions for every core term, grouped by domain.
- **Why it exists:** Chapters should not each reinvent "workspace" or "profile."
- **When you need it:** You hit an unfamiliar term or two chapters seem to disagree.
- **When you can skip it:** Learning the product — terms are introduced in [Learn ODS](../guides/README.md) when first needed.
- **Learn this first:** [Why ODS exists](../guides/00-why-ods.md)
- **Prerequisite chapters:** [README.md](README.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt), [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt)) when, and only when, they appear in all capitals.

---

## 2. Terminology Map by Domain

ODS terminology is organized into 7 functional domains:

```mermaid
graph TD
    Glossary["ODS Normative Glossary"]
    
    Glossary --> D1["1. Core & Format Model"]
    Glossary --> D2["2. Document Graph & Identity"]
    Glossary --> D3["3. AI Bounded Context"]
    Glossary --> D4["4. Assets & Code Bindings"]
    Glossary --> D5["5. Profiles & Structural Shapes"]
    Glossary --> D6["6. Conformance & Lifecycle"]
    Glossary --> D7["7. Discovery & Ecosystem"]
```

---

## 3. Core Architecture & Format Model

| Term | Normative Definition | Chapter Reference |
| :--- | :--- | :--- |
| **Workspace** | A directory tree declared by the presence of a root `ods.toml` manifest containing a `spec` version. The workspace defines the boundary for document discovery, identity resolution, graph traversal, and CI validation. Root `index.md` is **not** this marker. | [Chapter 08 · indexes.md](indexes.md#2-workspace-marker-odstoml) |
| **Navigation index** | An optional Markdown document with `ods.profile: index`. It may list children for humans. It is **not** the workspace marker and MUST NOT hold policy keys (`spec`, `ignore`, `packs`, `custom_profiles`, `specs`). | [Chapter 08 · indexes.md](indexes.md#21-what-is-not-the-workspace-marker) |
| **Document** | Any standard Markdown (`.md`) file located within an ODS workspace. ODS documents maintain 100% Markdown compatibility without requiring proprietary file extensions. Frontmatter is optional. | [Chapter 02 · core.md](core.md#3-format-model) |
| **Frontmatter** | A YAML metadata block delimited by opening and closing `---` markers at the top of a document. Contains machine-readable metadata partitioned strictly into 3 tiers. | [Chapter 02 · core.md](core.md#31-frontmatter) |
| **Body Prose** | The human-readable Markdown text following the frontmatter. Contains narrative explanations, workflows, headings, and code snippets. The document's title is declared exclusively by the first `# H1` heading in the body. | [Chapter 02 · core.md](core.md#32-body-prose) |
| **3-Tier Key Placement** | The strict architectural partitioning of metadata: **Tier 1 (Universal)** top-level keys (`description`, `tags`, `owner`, `created`, `updated`); **Tier 2 (Engine)** keys scoped under `ods:` (`profile`, `status`, `depends`, `related`, `resources`, `code`, `context`); and **Tier 3 (Operational)** prose contracts placed in `##` H2 body sections. | [Chapter 03 · keys.md](keys.md#3-the-3-tier-metadata-architecture) |
| **Single Source of Truth (SSOT)** | The core design invariant dictating that every piece of information has exactly one canonical location. Specifically, the document title lives solely in the first `# H1` body header (forbidding `title:` in frontmatter), and prose body text MUST NOT re-declare frontmatter metadata. | [Chapter 02 · core.md](core.md#2-design-principles-priority-order) |
| **Unknown Key Preservation** | The mandatory parser and tooling invariant requiring all third-party and unrecognized YAML frontmatter keys (e.g. Astro `hero_image`, Hugo `layout`, Jekyll `permalink`) to be preserved verbatim during formatting, migration, or refactoring. | [Chapter 09 · validation.md](validation.md#5-unknown-content-behavior) |

---

## 4. Document Graph & Identity

| Term | Normative Definition | Chapter Reference |
| :--- | :--- | :--- |
| **Path-Derived Document ID** | The default, deterministic identifier of a document, defined as its workspace-relative file path without the `.md` extension, normalized to lowercase with forward slashes `/` (e.g. `guides/auth.md` → `guides/auth`). | [Chapter 05 · graph.md](graph.md#21-default-path-derived-id) |
| **Explicit Document ID (`ods.id`)** | An optional frontmatter override used primarily during document renaming to preserve existing inbound references without requiring immediate cascading link rewrites. | [Chapter 05 · graph.md](graph.md#22-explicit-override-odsid) |
| **Knowledge Graph (`ods.depends`)** | Explicit, directional frontmatter edges declaring **hard conceptual prerequisites**. Auto-traversed during AI context resolution up to `max-depth`. MUST form a strict Directed Acyclic Graph (DAG). | [Chapter 05 · graph.md](graph.md#3-graph-edge-types-depends-vs-related) |
| **Discovery Graph (`ods.related`)** | Explicit frontmatter edges declaring **soft associative relationships** for human cross-referencing. Bidirectional and cyclic edges are permitted. Skipped by default during AI context resolution. | [Chapter 05 · graph.md](graph.md#32-discovery-graph-odsrelated) |
| **DAG Acyclicity** | The mathematical constraint that `ods.depends` edges MUST NOT contain circular reference loops (e.g. `A → B → C → A`). Enforced deterministically by `ods lint` via topological sorting. | [Chapter 05 · graph.md](graph.md#4-dag-cycle-prevention) |
| **Knowledge Graph Purity** | The rule restricting `ods.depends` strictly to other **Markdown documents**. Non-Markdown fixtures, schemas, and binary assets MUST NOT be placed in `depends`. | [Chapter 05 · graph.md](graph.md#5-knowledge-graph-purity) |

---

## 5. AI Bounded Context Scope

| Term | Normative Definition | Chapter Reference |
| :--- | :--- | :--- |
| **Bounded AI Context** | A deterministic, token-optimized context bundle assembled on demand by the `ods context` engine for LLM prompt windows, eliminating prompt bloat, token exhaustion, and context hallucination. | [Chapter 06 · context.md](context.md#2-the-4-distinct-subsystems-under-ods) |
| **Context Expansion Algorithm** | The recursive resolution routine that traverses `ods.depends` edges up to `max-depth`, injects `context.load` text fixtures, and prunes paths matched by `context.ignore`. | [Chapter 06 · context.md](context.md#3-context-expansion-algorithm) |
| **Context Max Depth (`context.max-depth`)** | An integer (default: `2`, maximum: `5`) specifying the maximum recursion depth for transitive dependency resolution along `ods.depends` edges. | [Chapter 06 · context.md](context.md#4-traversal-scoping-max-depth-and-ignore) |
| **Prompt Fixtures (`ods.context.load`)** | An array of relative paths to auxiliary non-Markdown text files (JSON schemas, sample CSVs, mock payloads) that are explicitly injected into the AI prompt window. | [Chapter 06 · context.md](context.md#5-auxiliary-prompt-fixtures-contextload) |
| **Context Pruning (`ods.context.ignore`)** | Path prefix patterns excluded from context expansion, used to filter out legacy directories or irrelevant subtrees during prompt assembly. | [Chapter 06 · context.md](context.md#42-pruning-subtrees-contextignore) |

---

## 6. Assets & Source Code Bindings

| Term | Normative Definition | Chapter Reference |
| :--- | :--- | :--- |
| **Asset Catalog (`ods.resources`)** | An array of attached non-Markdown files (PDF architecture documents, PNG diagrams, OpenAPI specifications) verified by `ods lint` for disk existence but **never** dumped into AI prompt windows. | [Chapter 07 · assets.md](assets.md#4-the-asset-catalog-odsresources) |
| **Code Bindings (`ods.code`)** | Structured metadata declarations connecting Markdown documentation to concrete source code files and AST symbols across the repository. | [Chapter 07 · assets.md](assets.md#5-code-bindings-odscode) |
| **Code Role (`role`)** | One of 8 standardized architectural classifications assigned to a code binding: `entrypoint`, `implementation`, `test`, `schema`, `migration`, `config`, `infrastructure`, or `pipeline`. | [Chapter 07 · assets.md](assets.md#52-the-8-standard-code-roles) |
| **Symbol-Based Binding (`symbol`)** | Linking documentation to specific programming language constructs (functions, structs, classes, interfaces, constants) rather than fragile, commit-volatile line numbers (`:L45`). | [Chapter 07 · assets.md](assets.md#53-symbol-based-bindings-why-line-numbers-are-forbidden) |

---

## 7. Structural Profiles & Taxonomy

| Term | Normative Definition | Chapter Reference |
| :--- | :--- | :--- |
| **Profile (`ods.profile`)** | A structural validation contract that defines the semantic intent of a document and specifies its expected H2 or H3 section headings (`##` or `###`). Profiles are not file extensions or presentation layouts. | [Chapter 04 · profiles.md](profiles.md#2-what-is-a-profile) |
| **13 Standard Profiles** | Built-in profile contracts provided by ODS: `note` (default), `guide`, `feature`, `decision`, `sop`, `api`, `architecture`, `policy`, `meeting`, `faq`, `checklist`, `agent`, and `skill`. | [Chapter 04 · profiles.md](profiles.md#3-standard-profiles-catalog) |
| **Custom Profiles** | Organization-specific or domain-specific structural contracts declared in workspace documents and registered via `ods.toml`. | [Chapter 04 · profiles.md](profiles.md#6-custom-profiles--profile-definition-files) |
| **Packs (`packs`)** | Reusable, versioned bundles of custom profiles, templates, and skills shared across repositories and configured in `ods.toml`. | [Chapter 04 · profiles.md](profiles.md#7-ods-packs-reusable-profile-catalogs) |

---

## 8. Conformance, Validation & Lifecycle

| Term | Normative Definition | Chapter Reference |
| :--- | :--- | :--- |
| **Binary Compliance** | The foundational validation contract: an ODS workspace is evaluated as either **Compliant** (`ods lint` exits with code `0`) or **Non-Compliant** (`ods lint` exits with code `1`). There is no ambiguous compliance level ladder. | [Chapter 09 · validation.md](validation.md#2-binary-compliance-contract) |
| **Lint Diagnostics** | Structured compiler messages emitted by `ods lint`, classified into `ERROR` (blocks compliance), `WARN` (advisory non-blocking), and `INFO`. | [Chapter 09 · validation.md](validation.md#4-diagnostic-message-format) |
| **Atomic Lifecycle Operations** | The 4 safe workspace mutation operations guaranteed by ODS tooling: `new` (scaffold), `mv` (relocate & cascade links), `archive` (retire), and `rm` (delete & scrub inbound edges). | [Chapter 02 · core.md](core.md#6-atomic-lifecycle-operations) |
| **Document Status (`ods.status`)** | The maturity stage of a document: `draft` (work in progress), `stable` (production-ready), `deprecated` (scheduled for retirement), or `archived` (historical reference). | [Chapter 03 · keys.md](keys.md#42-odsstatus-required-for-formal-docs) |
| **Share Boundaries (`ods.share`)** | Visibility classification: `public` (open export), `org` (internal organization), or `private` (confidential; automatically pruned from unprivileged AI contexts). | [Chapter 03 · keys.md](keys.md#43-odsshare-optional) |

---

## 9. Discovery, Workspace & Ecosystem

| Term | Normative Definition | Chapter Reference |
| :--- | :--- | :--- |
| **Progressive Discovery** | The on-demand CLI query workflow (`ods overview` → `ods find` / `ods tag` / `ods ls` → `ods context`) that eliminates the need for hand-maintained, conflict-prone folder index files (`index.ods.md`). | [Chapter 08 · indexes.md](indexes.md#3-progressive-discovery-model) |
| **Multi-Dialect Support** | The ODS engine's capability to process native ODS, Google Open Knowledge Format (`--okf`), and Agent Skills (`--skills`) schemas through unified graph traversal and validation. | [Chapter 01 · README.md](README.md#8-multi-dialect-context) |

---

## 10. Frequently Confused Concepts (Disambiguation Guide)

### 10.1 `depends` vs `related` vs `context.load` vs `resources`

```mermaid
graph TD
    Root["Document Asset & Link Types"]
    
    Root --> MD["Markdown Files"]
    Root --> NonMD["Non-Markdown Files"]
    
    MD --> Dep["ods.depends<br>• Hard Prerequisite<br>• Strict DAG (No Cycles)<br>• Auto-Traversed in Context"]
    MD --> Rel["ods.related<br>• Soft Associative Link<br>• Cycles Allowed<br>• Skipped in Context"]
    
    NonMD --> Res["ods.resources<br>• Binary / Large Files<br>• Disk Verified by Lint<br>• NEVER in Prompt"]
    NonMD --> Load["ods.context.load<br>• Text Schemas / Mocks<br>• Disk Verified by Lint<br>• INJECTED into Prompt"]
```

| Field | Target Type | Cycle Allowed? | Auto-Loaded in AI Prompt? | Purpose |
| :--- | :--- | :---: | :---: | :--- |
| **`ods.depends`** | Markdown (`.md`) | ❌ NO (DAG) | ✅ YES (up to `max-depth`) | Hard conceptual prerequisite needed to understand current document. |
| **`ods.related`** | Markdown (`.md`) | ✅ YES | ❌ NO (opt-in only) | Soft reference for human discovery and cross-reading. |
| **`ods.context.load`**| Non-Markdown (Text) | N/A | ✅ YES | Auxiliary prompt payload (JSON schema, sample CSV, mock payload). |
| **`ods.resources`** | Non-Markdown (Any) | N/A | ❌ NO | Attached binary asset (diagram, PDF report) verified for disk existence. |

---

### 10.2 `profile` vs `tags` vs `type`

| Concept | Canonical Field | Purpose | Format | Example |
| :--- | :--- | :--- | :--- | :--- |
| **Structural Shape** | `ods.profile` | Defines expected H2 or H3 section headings (`##` or `###`). | Single enum string | `profile: decision` |
| **Search Taxonomy** | `tags` (Top-level) | Categorization for search, filtering, and discovery. | List of lowercase strings | `tags: [auth, security]` |
| **Legacy / Anti-Pattern** | `type:` / `kind:` | **FORBIDDEN / NON-GOAL**. Redundant taxonomy. | N/A | *Do not use in ODS* |

---

### 10.3 `symbol` vs Line Numbers

| Approach | Example | Stability Across Git Commits | ODS Status |
| :--- | :--- | :---: | :--- |
| **Symbol-Based Binding** | `path: src/auth.ts`<br>`symbol: verifySession` | 🟢 High (persists through surrounding line edits) | ✅ **MANDATORY** |
| **Line Number Reference** | `path: src/auth.ts:L45-L60` | 🔴 Very Low (breaks on next commit or whitespace edit) | ❌ **FORBIDDEN** (`CODE-002`) |

---

## 11. Navigation & Quick Links

| Chapter | Specification Module | Focus Area |
| :---: | :--- | :--- |
| **01** | [**`README.md`**](README.md) | Introduction, 5W1H, Specification Map |
| **02** | [**`core.md`**](core.md) | Format Model, Binary Compliance, Atomic Lifecycle |
| **03** | [**`keys.md`**](keys.md) | Frontmatter Key Dictionary & 3-Tier Placement Rules |
| **04** | [**`profiles.md`**](profiles.md) | Document Profiles, Expected Headings, Custom Profiles |
| **05** | [**`graph.md`**](graph.md) | Document Graph, Path-Derived IDs, DAG Acyclicity |
| **06** | [**`context.md`**](context.md) | AI Bounded Context, Expansion Algorithm, Token Budgets |
| **07** | [**`assets.md`**](assets.md) | Resources, Code Bindings, 8 Standard Code Roles |
| **08** | [**`indexes.md`**](indexes.md) | Workspace Config (`ods.toml`), Progressive Discovery |
| **09** | [**`validation.md`**](validation.md) | Validation Contract, Diagnostic Formats, Lint Rules |
| **10** | [**`scope.md`**](scope.md) | Scope Boundaries, Non-Goals, Architectural Trade-offs |
| **REF**| [**`glossary.md`**](glossary.md) *(Current)* | **Normative Terminology Dictionary & Concept Disambiguation** |
