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

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt), [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt)) when, and only when, they appear in all capitals.

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
│ # Tier 1: Universal Keys (Visible to all YAML/SSG tools)                │
│ description: Universal summary for previews and search                  │
│ tags: [auth, security]                                                  │
│                                                                         │
│ # Tier 2: ODS Engine Keys (Scoped to prevent keyword collisions)         │
│ ods:                                                                    │
│   profile: guide                                                        │
│   status: stable                                                        │
│   depends: [../crypto/tokens.md]                                        │
│ ---                                                                     │
├─────────────────────────────────────────────────────────────────────────┤
│ Body Prose (Markdown)                                                   │
│ # Document Title (Sole Title Definition)                                │
│                                                                         │
│ ## Overview                                                             │
│ Human-readable explanation, decisions, and usage.                       │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.1 Frontmatter
- Frontmatter MUST be a single YAML document delimited by `---` on the first line of the file and closed by `---` on its own line.
- Frontmatter is **optional**. All fields within frontmatter are **optional**.
- Frontmatter contains machine-readable metadata intended for developer tooling, search indexers, and AI agent runtimes.
- Frontmatter MUST NOT contain a `title:` key. The document title is declared exclusively by the first `# H1` heading in the body.
- Parsers and tools MUST preserve unknown frontmatter keys to guarantee zero-friction interoperability with Static Site Generators (SSGs) like Hugo, Astro, Jekyll, Docusaurus, Next.js, and Obsidian.

### 3.2 Body Prose
- The body contains human-readable Markdown prose (purpose, architectural rationale, workflows, diagrams, and code snippets).
- The body MUST NOT re-declare metadata already declared in frontmatter (such as `owner`, `status`, or edge lists).
- The document's primary title MUST be defined as the first `# H1` heading in the body.

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
| **ODS Workspace** | Directory tree whose root **`ods.toml`** has a non-empty `spec`. A root `index.md` (even with `ods: 0.1`) is **not** sufficient. | Tooling discovers documents and enforces ODS rules. |
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

### 5.3 Legacy root index → `ods.toml`
Early workspaces used a root `index.md` / `index.ods.md` with scalar `ods: 0.1` and policy keys (`packs`, `ignore`, `custom-profiles`) in frontmatter. That form is **not** a workspace marker.

- `ods init` MUST write root `ods.toml` and MAY copy those policy keys into it.
- After `ods.toml` exists, tools MUST load policy from `ods.toml` only. They MUST NOT write `packs:` / `custom-profiles:` back into document frontmatter.
- A leftover root `index.md` MAY remain as optional navigation (`ods.profile: index`). It MUST NOT keep workspace policy keys (`PLACE-003`).

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
  - Inline Markdown links (`[text](relative/path.md)`).
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

When adopting untyped Markdown documents into an ODS workspace (`ods adopt`), tools SHOULD scan existing `##` and `###` headings to infer the most appropriate `ods.profile`:

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

### Why prohibit `title:` in Frontmatter?
When title exists in both frontmatter (`title: Foo`) and body prose (`# Bar`), they inevitably drift out of sync during edits. Defining title solely as the first `# H1` adheres strictly to the Single Source of Truth (SSOT) principle.

### Why binary compliance instead of compliance levels?
Compliance levels (e.g. Level 0 through 3) created confusion for developers regarding whether a doc was "good enough" for CI. Binary compliance provides an unambiguous contract: `ods lint` either passes (exit 0) or fails (exit 1).

---

## Navigation & Reading Order

| [← Previous Chapter](README.md) | [📑 Specification Index](README.md) | [Next Chapter →](keys.md) |
| :--- | :---: | ---: |
| **01. Introduction & Overview** | **Open Document Spec (ODS)** | **03. Frontmatter Key Dictionary** |
