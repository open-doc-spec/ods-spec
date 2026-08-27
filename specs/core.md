---
description: "ODS format model, single source of truth, binary compliance, atomic lifecycle operations, and backward compatibility guarantees."
ods:
  profile: "note"
  status: "stable"
  depends:
    - README.md
  related:
    - keys.md
    - graph.md
    - indexes.md
    - profiles.md
    - validation.md
    - ../guides/01-first-document.md
    - ../guides/06-run-the-workspace.md
---

# ODS · Core Format Model & Conformance

This document defines the normative format model, compliance requirements, lifecycle operations, and backward-compatibility architecture for Open Document Spec (**ODS**).

## At a glance

- **What this chapter defines:** The document file model (optional YAML + Markdown body), binary compliance, lifecycle operations, and backward-compatible reads.
- **Why it exists:** Every other chapter assumes one format, one pass/fail gate, and one home for the title.
- **When you need it:** You are implementing a parser, writing CI, or deciding where the title lives.
- **When you can skip it:** You only want to write a first document — use [Your first document](../guides/01-first-document.md).
- **Learn this first:** [Why ODS exists](../guides/00-why-ods.md) → [Your first document](../guides/01-first-document.md)
- **Prerequisite chapters:** [README.md](README.md) (map).

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14, exactly as stated in [README.md §1](README.md#1-conformance-language). That is the canonical statement; do not maintain a second copy here.

---

## 2. Design Principles (Priority Order)

1. **Human First**: Documents MUST remain plain UTF-8 text, readable and editable in any standard text editor across all operating systems.
2. **Zero-Friction Adoption**: Standard Markdown without frontmatter is valid. Adopting ODS means enriching documents with metadata; it MUST NOT require rewriting or migrating existing documentation into a proprietary schema.
3. **Token Efficient (DRY / SSOT)**: Every metadata fact MUST have exactly one canonical home. Metadata MUST NOT duplicate prose, and body text MUST NOT re-declare machine attributes.
4. **Graph Native**: Relationships between documents are explicit frontmatter edges forming a verifiable Directed Acyclic Graph (DAG), rather than inferred through ambiguous prose links.
5. **Trust from Validation**: The specification MUST NOT require rules that cannot be automatically verified by tooling and CI linters.

---

## 3. Format Model

An ODS Document is a Markdown file (`.md`) containing optional YAML frontmatter.

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ YAML Frontmatter (Optional)                                             │
│ ---                                                                     │
│ # Layer 1: Universal & OKF Native Keys (Visible to all YAML/OKF tools)  │
│ description: Universal summary for previews and search                  │
│ tags: [auth, security]                                                  │
│ owner: team:security                                                    │
│ author: Alice Smith                                                     │
│ created_at: 2026-08-26                                                  │
│ type: BigQuery Table                                                    │
│ sources: [{ id: bq-src, resource: datasets/auth.sql }]                  │
│ verified: [{ by: "human:ahormati", at: "2026-08-20T00:00:00Z" }]        │
│                                                                         │
│ # Layer 2: Scoped ODS Engine Keys (Direct under ods:)                   │
│ ods:                                                                    │
│   profile: guide                                                        │
│   status: stable                                                        │
│   entity: UserSession                                                   │
│   domain: Identity                                                      │
│   schema: schemas/session.schema.json                                   │
│   invariants: ["mrr >= 0", "email is required"]                         │
│   depends: [../crypto/tokens.md]                                        │
│   related:                                                              │
│     - is_a: SessionModel                                                │
│     - owns: [Token, RefreshSession]                                     │
│     - ../policy/data-retention.md                                       │
│ ---                                                                     │
├─────────────────────────────────────────────────────────────────────────┤
│ Body Prose (Markdown)                                                   │
│ # Document Title (Sole Title Definition in pure ODS)                    │
│                                                                         │
│ ## Overview                                                             │
│ Human-readable explanation, decisions, and usage.                       │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.0 Minimal Conformant Document

ODS adoption is **additive**. Conformance is defined by what a document does not get wrong, not by what it declares.

| Level | Requirement | Status |
| :--- | :--- | :--- |
| **Absolute minimum** | A UTF-8 Markdown file inside the workspace. Frontmatter MAY be absent entirely. | **Conformant** |
| **Recommended floor** | `description` (top level) + `ods.profile` + `ods.status`. | **Conformant + useful** |
| **Everything else** | `depends`, `related`, `code`, `resources`, `context`, `entity`, `memory`, attestations. | **Progressive enhancement** |

- A document with no frontmatter MUST NOT be reported as an error. Tools MAY report an informational hint suggesting `description` and `ods.profile`.
- A document that omits `ods.profile` is treated as `profile: note`, whose section contract is empty; it therefore cannot fail `PROF-002`.
- A document that omits `ods.status` is treated as `status: draft`.
- No key is required for conformance. Errors arise only from keys that are **present and wrong** (bad placement, invalid enum, dangling path, cyclic `depends`).

```markdown
---
description: "How JWT session tokens are signed, verified, and revoked."
ods:
  profile: guide
  status: stable
---

# User Authentication Guide

## Overview
...
```

This document is complete. Authors SHOULD NOT add keys speculatively; each additional key exists to solve a problem stated in its own chapter.

Adoption stages, and the guide that teaches each, are laid out in [Learn ODS](../guides/README.md).

---

### 3.1 Frontmatter
- Frontmatter MUST be a single YAML document delimited by `---` on the first line of the file and closed by `---` on its own line.
- Frontmatter is **optional**. All fields within frontmatter are **optional**.
- Frontmatter contains machine-readable metadata intended for developer tooling, search indexers, and AI agent runtimes.
- In pure ODS authoring, frontmatter SHOULD NOT contain a `title:` key (the document title is defined by the first `# H1` heading). Parsers MUST accept top-level `title:` and `type:` without error — the OKF superset depends on them.
- A `title:` key in a document carrying **no** OKF signal (no `type`, `okf_version`, or `sources`) is reported as a `SYNTAX-002` **warning** advising the author to move the title to the `# H1`. It is never an error, and tools MUST NOT strip or rewrite it. See [validation.md](validation.md#4-normative-lint-rules-matrix).
- Parsers and tools MUST preserve unknown frontmatter keys to guarantee zero-friction interoperability with Static Site Generators (SSGs) and external tools.

### 3.2 Native Google OKF v0.2 Superset Interoperability
ODS 1.1 operates as a strict superset of Google's Open Knowledge Format (OKF v0.2):
- Any valid OKF bundle (`index.md`, `log.md`, `references/`, `computations/`) is automatically a 100% compliant ODS workspace without configuration or file conversions.
- Top-level OKF keys (`type`, `title`, `description`, `resource`, `tags`, `sources`, `usage_window`, `generated`, `verified`, `status`, `stale_after`, `runtime`, `parameters`, `computation`, `executor`, `attester`, `okf_version`) are first-class native primitives.

### 3.3 Body Prose
- The body contains human-readable Markdown prose (purpose, architectural rationale, workflows, diagrams, and code snippets).
- The body MUST NOT re-declare metadata already declared in frontmatter (such as `owner`, `status`, or edge lists).
- In standard ODS documents, the document's primary title MUST be defined as the first `# H1` heading in the body.

### 3.4 Machine-Readable JSON Schemas
The normative data structures of ODS are formally defined using **JSON Schema Draft 2020-12**. Three schemas are normative:
- **Frontmatter Schema (v1.1.0)**: [`schemas/1.1.0/document.schema.json`](../schemas/1.1.0/document.schema.json) (`https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json`)
- **Workspace Config Schema (v1.1.0)**: [`schemas/1.1.0/config.schema.json`](../schemas/1.1.0/config.schema.json) (`https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/config.schema.json`)
- **Custom Profile Schema (v1.1.0)**: [`schemas/1.1.0/profile.schema.json`](../schemas/1.1.0/profile.schema.json) (`https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/profile.schema.json`)

Three further schemas — [`ontology.schema.json`](../schemas/1.1.0/ontology.schema.json), [`memory.schema.json`](../schemas/1.1.0/memory.schema.json), and [`attestation.schema.json`](../schemas/1.1.0/attestation.schema.json) — are published as **experimental**. They describe richer modelling shapes under review for a future revision and are NOT part of the ODS 1.1 conformance contract. Tools MUST NOT reject a document for failing them. See [`schemas/README.md`](../schemas/README.md).

Tooling, linters, and language servers SHOULD use the normative schemas for Stage 1 structural validation, key lifecycle verification (`x-ods-lifecycle`), and editor autocompletion.

---

## 4. Compliance Model (Binary)

ODS evaluates workspace compliance as a **binary state**. There is no Level 0–3 compliance ladder.

```mermaid
stateDiagram-v2
    [*] --> PlainMarkdown: Files on disk
    PlainMarkdown --> ODSWorkspace: Add root ods.toml
    ODSWorkspace --> Compliant: ods lint passes (0 errors)
    ODSWorkspace --> NonCompliant: ods lint finds errors
    NonCompliant --> Compliant: Fix errors & re-lint
```

| State | Definition | Validation Criteria |
| :--- | :--- | :--- |
| **Plain Markdown** | Markdown files without a workspace root marker. | Valid Markdown; not managed by ODS. |
| **ODS Workspace** | Directory tree containing a root `ods.toml` marker. | Tooling discovers documents and enforces ODS rules. |
| **Compliant** | An ODS workspace where `ods lint` passes with **zero errors** (exit code `0`). | Graph edges resolve, IDs are unique, no cycles exist, paths exist, schemas conform. |
| **Non-Compliant** | An ODS workspace containing one or more lint **errors** (exit code `1`). | Tooling reports directive diagnostics and remediation steps. |

---

## 5. Backward Compatibility & Migration Architecture

To ensure zero disruption for repositories adopting newer ODS engines or migrating from legacy tools, ODS mandates strict backward compatibility contracts:

### 5.1 CLI Argument Compatibility
Older CI scripts, GitHub Actions (`action.yml`), and developer aliases may pass legacy flags such as `--level 1`, `--level 3`, `--mode standard`, or `--mode strict`.
- The `ods` CLI and engine MUST accept these flags gracefully without crashing.
- Tools MUST silently map legacy level flags to the unified **Full Compliance Mode**.

### 5.2 Legacy Frontmatter Migration (`ods fmt --migrate`)
During early adoption, documents may contain un-nested engine keys at the top level:

```yaml
# LEGACY FORMAT (Accepted on read during migration)
---
description: User setup guide.
profile: guide                  # Legacy flat engine key
status: draft                   # Legacy flat engine key
tags: [setup]
---

# User Setup
```

- Parsers MUST accept legacy flat engine keys on read.
- Migration and formatting tools (`ods fmt --migrate`) MUST hoist universal keys (`description`, `tags`) to the top level, nest engine keys (`profile`, `status`) under `ods:`, and preserve all unknown third-party keys.

---

## 6. Atomic Lifecycle Operations

Conformant ODS tools MUST implement or support four atomic lifecycle operations to maintain graph integrity during repository evolution:

```mermaid
graph LR
    Scaffold["1. Scaffold<br><code>ods new</code>"] --> Relocate["2. Relocate<br><code>ods mv</code>"]
    Relocate --> Archive["3. Archive<br><code>ods archive</code>"]
    Relocate --> Delete["4. Delete<br><code>ods rm</code>"]
```

### 1. Scaffold (`ods new <path>`)
- Creates a new Markdown document at the specified path with valid frontmatter (`ods.profile`, `ods.status: draft`, optional `description`).
- Derives the document ID automatically from `<path>`.
- Injects standard section heading placeholders corresponding to the chosen profile.

### 2. Relocate (`ods mv <from> <to>`)
- Moves or renames the file from `<from>` to `<to>`.
- Automatically rewrites all inbound references across the workspace, including:
  - `ods.depends` and `ods.related` in other documents.
  - `ods.context.load` references.
  - Inline Markdown links written in standard `[text](target)` form, where `target` is a workspace-relative path to the moved document.
  - Code bindings and relative resource paths.

### 3. Archive (`ods archive <path>`)
- Updates `ods.status` to `archived`.
- Preserves all inbound and outbound graph edges so historical context remains intact.
- Optionally moves the document to an `archive/` folder if configured by the workspace.

### 4. Delete (`ods rm <path>`)
- Removes the document file from the filesystem.
- Scans the entire workspace and automatically scrubs the deleted document's path/ID from all inbound `ods.depends`, `ods.related`, and `ods.context.load` arrays to prevent dangling references.

---

## 7. Smart Profile Inference Heuristics

When adopting untyped Markdown documents into an ODS workspace, tools SHOULD scan existing `##` and `###` headings to infer the most appropriate `ods.profile`. The heading sets below are *inference hints*; the normative section contract for each profile lives in [profiles.md §3](profiles.md#3-standard-profiles-catalog).

| Heading Keywords Found in Document | Inferred Profile | Rationale |
| :--- | :--- | :--- |
| Goal, Scope, Requirements, Acceptance Criteria, Risks | `feature` | Product specification / PRD structure |
| Overview, Prerequisites, Steps, Troubleshooting | `guide` | Step-by-step procedural tutorial |
| Context, Decision, Alternatives, Consequences | `decision` | Architecture Decision Record (ADR) |
| Purpose, Prerequisites, Steps, Validation, Rollback | `sop` | Operations runbook / standard procedure |
| Overview, Request, Response, Errors, Examples, Endpoint | `api` | API endpoint / interface reference |
| Overview, Components, Data Flow, Trade-offs | `architecture` | System architecture overview |
| Purpose, Scope, Rules, Exceptions | `policy` | Organizational policy / governance |
| Attendees, Agenda, Decisions, Action Items | `meeting` | Meeting notes and outcomes |
| Items, Verification, Checklist, Gates | `checklist` | Verifiable deployment/release checklist |
| Goal, Task, Constraints, Success Criteria, Failure Modes, Output | `agent` | Autonomous agent instruction / prompt contract |
| Purpose, Capability, Activation, Workflow, Tools, Eval, Validation | `skill` | Reusable skill package / capability definition |
| *(None of the above / mixed headings)* | `note` | Default free-form document shape |

---

## 8. Design Decisions

### Why separate Frontmatter and Body Prose?
Frontmatter is optimized for deterministic machine indexing, CI validation, and graph traversal. Body prose is optimized for human reading and rich explanations. Mixing machine metadata (such as graph edges and code bindings) inside prose leads to fragile regular expressions and parse errors.

### Why discourage `title:` in Frontmatter — but not forbid it?
When title exists in both frontmatter (`title: Foo`) and body prose (`# Bar`), they inevitably drift out of sync during edits. Defining title solely as the first `# H1` adheres strictly to the Single Source of Truth (SSOT) principle, so pure ODS authoring omits `title:`.

Rejecting it outright, however, would break the OKF v0.2 superset guarantee — OKF concepts carry `title:` natively, and a spec cannot claim to accept "any valid OKF bundle" while erroring on one of its core keys. ODS therefore accepts `title:` universally and reports it as a `SYNTAX-002` warning only where no OKF signal is present.

### Why binary compliance instead of compliance levels?
Compliance levels (e.g. Level 0 through 3) created confusion for developers regarding whether a doc was "good enough" for CI. Binary compliance provides an unambiguous contract: `ods lint` either passes (exit 0) or fails (exit 1).

---

## Navigation & Reading Order

| [← Previous Chapter](README.md) | [📑 Specification Index](README.md) | [Next Chapter →](keys.md) |
| :--- | :---: | ---: |
| **01. Introduction & Overview** | **Open Document Spec (ODS)** | **03. Frontmatter Key Dictionary** |
