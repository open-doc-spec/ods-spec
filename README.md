---
description: Normative specification, formal dialect definitions, and architecture for Open Document Spec (ODS).
tags:
  - ods
  - spec
  - documentation
  - standard
owner: team:ods
created: 2026-08-14
updated: 2026-08-14
ods:
  profile: note
  status: stable
  share: public
  related:
    - specs/README.md
    - AGENTS.md
    - docs/plan/code-vs-spec-alignment.md
    - docs/report/README.md
---

# Open Document Spec (ODS)

**Open Document Spec (ODS)** is a Markdown-first convention for documentation in Git. Files stay ordinary `.md`. Optional YAML frontmatter and a root `ods.toml` make identity, ownership, document shape, prerequisites, and AI reading lists **explicit and lintable** — for humans and agents — without a new file extension or a vendor database.

This repository is the normative specification. **New readers should not start in `specs/`.**

| You are… | Start here |
| :--- | :--- |
| **Learning ODS** (author, tech writer, team lead) | **[Learn ODS: novice → expert](./guides/README.md)** |
| **Evaluating ecosystem & spec strategy** | **[Comparative Spec Analysis & Roadmap](./docs/report/README.md)** |
| **Looking up a rule** (key, profile, lint ID) | [Specification map](./specs/README.md) |
| **Using JSON Schemas in IDEs** (VS Code, JetBrains, Zed) | [JSON Schemas (Draft 2020-12)](./schemas/README.md) |
| **Implementing a parser or linter** | [Format model](./specs/core.md) → [keys](./specs/keys.md) → [validation](./specs/validation.md) |

---

## Quick Start

An ODS document is plain Markdown (`.md`) with optional YAML frontmatter. Day 1 only needs `description`, `tags`, `ods.profile`, and `ods.status` — [write that first](./guides/01-first-document.md). The snippet below is a mid-track file that also links a prerequisite and a source symbol.

```markdown
---
description: How to configure and process customer refunds in the web dashboard.
tags:
  - billing
  - customer-care
owner: team:billing
ods:
  profile: guide
  status: stable
  depends:
    - ../auth/sessions.md
  code:
    - path: apps/web/src/features/refunds/process.ts
      role: implementation
      symbol: processRefund
---

# Refund Processing Guide

## Overview
Follow this procedure when issuing customer refunds.

## Prerequisites
- Administrative access to the billing console.
- Active authentication session.

## Steps
1. Navigate to **Billing** → **Transactions**.
2. Search for the transaction ID.
3. Click **Issue Refund** and select the amount.

## Troubleshooting
If the refund fails with an API timeout, verify session tokens in the auth console.
```

An ODS workspace is declared by placing an `ods.toml` marker in the repository root:

```toml
# ods.toml — repository root only
spec = "0.1"

ignore = ["src", "target", "node_modules"]
custom_profiles = ["docs/profiles/rfc.md"]
```

---

## Learn ODS (novice → expert)

Do not read the ten spec chapters linearly to learn the product. Follow the ladder and stop when the next page is more than you need.

| Level | Guide | Stop here if… |
| :---: | :--- | :--- |
| 0 | [Why ODS exists](./guides/00-why-ods.md) | You only wanted the idea. |
| 1 | [Your first document](./guides/01-first-document.md) | One trustworthy guide is enough. |
| 2 | [Pick a shape](./guides/02-pick-a-shape.md) | You can choose `guide` / `decision` / `feature` / `sop`. |
| 3 | [Link documents](./guides/03-link-documents.md) | You can tell `depends` from `related`. |
| 4 | [Bind files and code](./guides/04-bind-code-and-files.md) | You can place a PNG, a function, and a JSON schema. |
| 5 | [Give AI a reading list](./guides/05-ai-reading-list.md) | Agents get a bundle, not the repo. |
| 6 | [Run the workspace](./guides/06-run-the-workspace.md) | CI lints; renames do not break edges. |
| 7 | [Extend ODS](./guides/07-extend-ods.md) | You need custom profiles or the implementer map. |

Pocket refs: [decision cards](./guides/decision-cards.md) · [common mistakes](./guides/mistakes.md) · [FAQ](./guides/faq.md)

---

## Specification reference

Normative modules for lookup and implementers. Each chapter opens with when to read it and when to skip it. Map and pathways: [`specs/README.md`](./specs/README.md).

| Chapter | Module | Lookup for |
| :---: | :--- | :--- |
| **01** | [`specs/README.md`](./specs/README.md) | Spec map (not the human intro) |
| **02** | [`core.md`](./specs/core.md) | Format model, compliance, lifecycle operations |
| **03** | [`keys.md`](./specs/keys.md) | Every frontmatter key |
| **04** | [`profiles.md`](./specs/profiles.md) | 13 profiles, templates, packs |
| **05** | [`graph.md`](./specs/graph.md) | IDs, `depends` / `related`, DAG |
| **06** | [`context.md`](./specs/context.md) | Bounded AI context algorithm |
| **07** | [`assets.md`](./specs/assets.md) | `resources`, `code` roles, no line numbers |
| **08** | [`indexes.md`](./specs/indexes.md) | `ods.toml`, discovery CLI |
| **09** | [`validation.md`](./specs/validation.md) | Lint rules, exit 0/1 |
| **10** | [`scope.md`](./specs/scope.md) | Intentional non-goals |
| **REF** | [`glossary.md`](./specs/glossary.md) | Formal terminology |

---

## JSON Schemas (Draft 2020-12)

Machine-readable JSON Schemas are published in [`schemas/1.0.0/`](./schemas/README.md):
- **Document Frontmatter**: [`document.schema.json`](./schemas/1.0.0/document.schema.json)
- **Workspace Config**: [`config.schema.json`](./schemas/1.0.0/config.schema.json)
- **Custom Profile**: [`profile.schema.json`](./schemas/1.0.0/profile.schema.json)

---

## Design Principles

The normative list lives in [`specs/core.md`](./specs/core.md). In short: documents stay plain Markdown; adoption is additive; each fact has one home; relationships are explicit edges; AI context is bounded; only lintable rules are required.

---

## Source of Truth & Implementations

| Concern | Repository |
| :--- | :--- |
| **Normative Specification** (This Repo) | [open-doc-spec/ods-spec](https://github.com/open-doc-spec/ods-spec) |
| **CLI & Core Engine Implementation** (Rust) | [open-doc-spec/ods](https://github.com/open-doc-spec/ods) |
| **Runtime Rust Schema** | [`src/ods-core/src/spec/schema.rs`](https://github.com/open-doc-spec/ods/blob/main/src/ods-core/src/spec/schema.rs) |

---

## License

This specification is licensed under the Apache License, Version 2.0. See [LICENSE](./LICENSE) for details.
