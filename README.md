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

## Start here (2 minutes)

**A plain Markdown file with no frontmatter at all is already a conformant ODS document.** Adoption is additive: you never rewrite what you have.

The recommended floor is three keys:

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

That is a complete, lintable ODS document. Add a `ods.toml` at the repo root (`spec = "1.1"`) and you have a workspace.

Everything below is **progressive enhancement** — reach for it when you have the problem it solves, not before. The gentle path is [Learn ODS](./guides/README.md); the full normative rules are in [`specs/`](./specs/README.md).

| Stage | You have | You add | Learn it in |
| :---: | :--- | :--- | :--- |
| 0 | Plain Markdown | *(nothing — already valid)* | [00 · Why ODS](./guides/00-why-ods.md) |
| 1 | One trustworthy doc | `description`, `ods.profile`, `ods.status` | [01 · Your first document](./guides/01-first-document.md) |
| 2 | A workspace | root `ods.toml` | [06 · Run the workspace](./guides/06-run-the-workspace.md) |
| 3 | A linked set | `ods.depends`, `ods.related` | [03 · Link documents](./guides/03-link-documents.md) |
| 4 | Agent-ready docs | `ods.code`, `ods.context` | [04](./guides/04-bind-code-and-files.md) · [05](./guides/05-ai-reading-list.md) |
| 5 | Domain & memory | `ods.entity`, `memory:`, attestations | [07 · Ontologies & memory](./guides/07-ontologies-and-memory.md) |

Normative definition of the minimum: [`specs/core.md` §3.0](./specs/core.md#30-minimal-conformant-document).

---

## What you can grow into

Plain Markdown files with structured 3-layer frontmatter unlock capabilities that proprietary databases and metadata silos cannot provide:

- **Human- and Agent-Centric**: Plain `.md` text files. An engineer can `cat` a document; an AI agent can ingest it directly into prompt context.
- **Dual-Graph Architecture**: Seamlessly integrates a **Domain Graph** (`ods.entity`, `ods.domain`, `invariants`) with a **Lexical Graph** (`ods.depends`, `ods.related`).
- **Cognitive Agent Memory**: Built-in support for agent session traces, bi-temporal validity windows (`valid_from`/`valid_to`), and state mutations (`memory:`).
- **Bounded AI Context (Zero Token Waste)**: Surgical prompt scoping via `ods.context` loads only exact prerequisite documents instead of flooding LLM windows.
- **Refactor-Resilient Code Bindings**: Binds documentation directly to AST symbols (`ods.code`) across TypeScript, Python, Rust, and Go without fragile line numbers.
- **Google OKF v0.2 Superset**: 100% transparently interoperable with Google Open Knowledge Format concepts, datasets, and attested computations.
- **Works Out-of-the-Box with Existing Tooling**: Fully compatible with Hugo, Astro, Docusaurus, MkDocs, and GitHub Markdown.

---

## A fully-loaded document

Every key below is optional. This is what a document looks like once it has grown into all five subsystems — not what you must write on day one:

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

## The 6 Canonical Document Recipes

Novice authors do not need to memorize the full key dictionary. ODS standardizes six plug-and-play recipes — *Daily Doc*, *Linked Doc*, *Code-Linked Doc*, *Domain Entity*, *Agent Memory*, and *Attested Computation* — each layering a handful of keys on the one before it.

Canonical, normative list: [`specs/keys.md` §1.1](./specs/keys.md#11-novice-quick-start-the-6-canonical-document-recipes). Pocket form: [`guides/decision-cards.md`](./guides/decision-cards.md). Do not maintain a second copy here.

---

## Standard Profile Shapes (13 Universal Profiles)

A **profile** declares a document's *shape* — the H2/H3 sections a conformant document of that kind is expected to carry. ODS ships 13: `note`, `guide`, `feature`, `decision`, `sop`, `api`, `architecture`, `policy`, `meeting`, `faq`, `checklist`, `agent`, and `skill`.

Canonical catalog with the expected section list for each: [`specs/profiles.md` §3](./specs/profiles.md#3-standard-profiles-catalog). Copy-paste templates: [`specs/profiles.md` §4](./specs/profiles.md#4-complete-profile-templates-copy-paste-ready). Which one to pick: [`guides/02-pick-a-shape.md`](./guides/02-pick-a-shape.md).

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

Published in [`schemas/1.1.0/`](./schemas/README.md) for IDE validation (VS Code, Cursor, JetBrains, Zed).

**Normative (part of the ODS 1.1 contract):**
- **Document Frontmatter**: [`schemas/1.1.0/document.schema.json`](./schemas/1.1.0/document.schema.json)
- **Workspace Config**: [`schemas/1.1.0/config.schema.json`](./schemas/1.1.0/config.schema.json)
- **Custom Profile**: [`schemas/1.1.0/profile.schema.json`](./schemas/1.1.0/profile.schema.json)

**Experimental (published for review; not required for conformance):**
- **Domain Ontology**: [`schemas/1.1.0/ontology.schema.json`](./schemas/1.1.0/ontology.schema.json)
- **Agent Memory**: [`schemas/1.1.0/memory.schema.json`](./schemas/1.1.0/memory.schema.json)
- **Attestation**: [`schemas/1.1.0/attestation.schema.json`](./schemas/1.1.0/attestation.schema.json)

---

## Specification Conformance Suite (Native Rust)

The ODS specification repository includes an automated native Rust test runner:

```bash
cargo test
```

Fourteen tests verify the specification against itself: the schemas compile, the fixtures behave as documented, every internal link and anchor resolves, and the prose and the schemas agree on every closed vocabulary (code roles, memory tiers, predicates, enums, traversal bounds). Change "10 roles" to "8" in one chapter and two tests fail.

```bash
cargo run -p ods-spec-conformance
```

---

## Contributing

This repository holds the **specification only**. Tooling — the `ods` CLI, language server, and editor extensions — lives in [`open-doc-spec/ods`](https://github.com/open-doc-spec/ods).

- [CONTRIBUTING.md](./CONTRIBUTING.md) — how to propose a key, a lint rule, or a deprecation
- [CHANGELOG.md](./CHANGELOG.md) — what each release added, deprecated, and clarified
- [`specs/scope.md` §7](./specs/scope.md#7-deprecations--versioning-policy) — versioning policy and the 2.0 deprecation schedule

---

## License

This specification is licensed under the Apache License, Version 2.0. See [LICENSE](./LICENSE) for details.
