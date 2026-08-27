---
description: "Open Document Spec (ODS) — The universal, vendor-neutral Markdown specification for humans, AI coding agents, and neuro-symbolic knowledge graphs."
tags:
  - ods
  - spec
  - documentation
  - ai-agents
  - knowledge-graph
owner: team:ods
author: "Open Document Spec Contributors"
created_at: 2026-08-14
updated_at: 2026-08-26
ods:
  profile: note
  status: stable
  share: public
  related:
    - specs/README.md
    - AGENTS.md
    - guides/README.md
---

# Open Document Spec (ODS)

### 📖 [Read the Open Document Spec v1.1 Specification → specs/README.md](./specs/README.md) · [Learn ODS (Tutorials) → guides/README.md](./guides/README.md)

> **Open Document Spec (ODS)** is a **universal, vendor-neutral format** for representing technical documentation, domain entity models, and agent memory as plain Markdown files with YAML frontmatter.
>
> It is **not tied to any specific tool, agent framework, model provider, or serving system**.
>
> - **Anyone can author** — engineers writing by hand, autonomous AI coding agents (Claude, Gemini, OpenAI, Antigravity), or export pipelines.
> - **Anyone can serve and consume** — static site generators (Hugo, Astro, Docusaurus, VitePress), IDEs (VS Code, Cursor, JetBrains), LLMs assembling surgical prompt context, or CI linters.

---

## Why ODS?

Plain markdown files with structured 3-layer frontmatter unlock capabilities that proprietary databases and metadata silos cannot provide:

- **Human- and Agent-Centric**: Plain `.md` text files. An engineer can `cat` a document; an AI agent can ingest it directly into prompt context.
- **Dual-Graph Architecture**: Seamlessly integrates a **Domain Graph** (`ods.entity`, `ods.domain`, `invariants`) with a **Lexical Graph** (`ods.depends`, `ods.related`).
- **Cognitive Agent Memory**: Built-in support for agent session traces, bi-temporal validity windows (`valid_from`/`valid_to`), and state mutations (`memory:`).
- **Bounded AI Context (Zero Token Waste)**: Surgical prompt scoping via `ods.context` loads only exact prerequisite documents instead of flooding LLM windows.
- **Refactor-Resilient Code Bindings**: Binds documentation directly to AST symbols (`ods.code`) across TypeScript, Python, Rust, and Go without fragile line numbers.
- **Google OKF v0.2 Superset**: 100% transparently interoperable with Google Open Knowledge Format concepts, datasets, and attested computations.
- **Works Out-of-the-Box with Existing Tooling**: Fully compatible with Hugo, Astro, Docusaurus, MkDocs, and GitHub Markdown.

---

## Quick Start (The Canonical Document)

Every ODS document is plain Markdown with clean, 3-layer frontmatter:

```markdown
---
description: "User authentication service specification and session flow."
tags: [auth, security]
owner: team:security
author: Alice Smith
created_at: 2026-08-26
updated_at: 2026-08-26

ods:
  profile: guide
  status: stable
  share: public

  # 1. Knowledge Graph (Hard prerequisites traversed during context loading)
  depends:
    - ../crypto/jwt-spec.md

  # 2. Discovery Graph (Lateral reading & typed domain relations)
  related:
    - ../policy/data-retention.md

  # 3. Code Bindings (Implementation & Tests)
  code:
    - src/auth.service.ts
    - path: tests/auth.service.test.ts
      role: test
      symbol: TestJwtValidation

  # 4. Resources (Local files & external URLs)
  resources:
    - ../diagrams/session-flow.png
    - https://figma.com/file/auth-v2-mockup
---

# User Authentication Guide

## Overview
This guide explains how JWT session tokens are signed, verified, and revoked.

## Prerequisites
- Node.js 20+ runtime.
- Redis cluster running on port 6379.

## Steps
1. Configure JWT signing keys in the environment.
2. Initialize the authentication middleware.
3. Validate incoming bearer tokens on secure routes.

## Troubleshooting
- **Token Expired**: Verify client system clock synchronization with NTP.
```

---

## The 5 Canonical Document Recipes

Novice authors do not need to memorize 42 keys. ODS standardizes 5 plug-and-play recipes:

| Recipe | Common Use Case | Required Keys |
| :--- | :--- | :--- |
| **1. Daily Doc** | How-to guide, SOP, or tech note | `description` + `tags` + `ods.profile` + `ods.status` |
| **2. Linked Doc** | Multi-page architecture or design | Recipe 1 + `ods.depends` / `ods.related` |
| **3. Code-Linked Doc** | Implementation & unit test bindings | Recipe 2 + `ods.code: ["src/main.rs"]` |
| **4. Domain Entity** | Business class & refusal guardrails | Recipe 1 + `ods.entity` + `ods.domain` + `invariants` |
| **5. Agent Memory** | Session traces & state mutations | Recipe 1 + `memory: { tier: episodic, mutations }` |
| **6. Attested Computation** | Verifiable SQL/Python execution | `type: Attested Computation` + `runtime` + `executor` |

---

## Standard Profile Shapes (13 Universal Profiles)

| Profile | Primary Intent | Expected Prose Sections |
| :--- | :--- | :--- |
| `note` | Free-form knowledge, entities, memory, and scratchpads | *(none required)* |
| `guide` | Step-by-step how-to tutorial | Overview, Prerequisites, Steps, Troubleshooting |
| `feature` | Product capability / PRD specification | Goal, Scope, Requirements, Acceptance Criteria, Risks |
| `decision` | Architecture Decision Record (ADR) | Context, Decision, Alternatives, Consequences |
| `sop` | Standard operating procedure / runbook | Purpose, Prerequisites, Steps, Validation, Rollback |
| `api` | Endpoint / RPC contract | Overview, Request, Response, Errors, Examples |
| `architecture` | System design and data flow | Overview, Components, Data Flow, Trade-offs |
| `policy` | Governance / team rules | Purpose, Scope, Rules, Exceptions |
| `meeting` | Meeting minutes and team sync notes | Attendees, Agenda, Decisions, Action Items |
| `faq` | Frequently Asked Questions | *(Question/Answer pairs)* |
| `checklist` | Verifiable deployment or release gates | Overview, Items, Verification, Notes |
| `agent` | Autonomous agent execution contracts (`agent.md`) | Goal, Task, Scope, Steps, Success Criteria |
| `skill` | Reusable skill packages and tool contracts (`SKILL.md`) | Purpose, Capability, Activation, Workflow, Rules |

---

## Specification Chapters (`specs/`)

| Chapter | Module | What It Defines |
| :---: | :--- | :--- |
| **01** | [`specs/README.md`](./specs/README.md) | Specification Index & Reader Pathways |
| **02** | [`specs/core.md`](./specs/core.md) | Format Model, # H1 Titles & OKF Superset |
| **03** | [`specs/keys.md`](./specs/keys.md) | Complete 3-Layer Key Dictionary & Recipes |
| **04** | [`specs/profiles.md`](./specs/profiles.md) | 13 Universal Profiles & Custom Profile Contracts |
| **05** | [`specs/graph.md`](./specs/graph.md) | Dual-Graph Architecture & Bi-Temporal Memory |
| **06** | [`specs/context.md`](./specs/context.md) | Bounded AI Context & Surgical Prompt Scoping |
| **07** | [`specs/assets.md`](./specs/assets.md) | Code Bindings, Resources & Attested Computations |
| **08** | [`specs/indexes.md`](./specs/indexes.md) | Root `ods.toml`, Discovery & OKF Bundle Roots |
| **09** | [`specs/validation.md`](./specs/validation.md) | Validation Rules, Error Codes & CI Linters |
| **10** | [`specs/scope.md`](./specs/scope.md) | Scope Boundaries & Intentional Non-Goals |
| **REF** | [`specs/glossary.md`](./specs/glossary.md) | Normative Terminology & Vocabulary Definitions |

---

## Machine-Readable JSON Schemas (Draft 2020-12)

Published in [`schemas/1.1.0/`](./schemas/README.md) for IDE validation (VS Code, Cursor, JetBrains, Zed):
- **Document Frontmatter**: [`schemas/1.1.0/document.schema.json`](./schemas/1.1.0/document.schema.json)
- **Workspace Config**: [`schemas/1.1.0/config.schema.json`](./schemas/1.1.0/config.schema.json)
- **Custom Profile**: [`schemas/1.1.0/profile.schema.json`](./schemas/1.1.0/profile.schema.json)

---

## Specification Conformance Suite (Native Rust)

The ODS specification repository includes an automated native Rust test runner:

```bash
# Run all integration tests
cargo test

# Run rich colored CLI conformance report
cargo run -p ods-spec-conformance
```

---

## License

This specification is licensed under the Apache License, Version 2.0. See [LICENSE](./LICENSE) for details.
