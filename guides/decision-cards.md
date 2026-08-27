---
description: "Pocket decision cards for ODS: which profile to pick, where a fact belongs, and which attachment key to use."
tags:
  - learn
  - ods
  - reference
owner: team:ods
ods:
  profile: note
  status: stable
  related:
    - 02-pick-a-shape.md
    - 03-link-documents.md
    - 04-bind-code-and-files.md
    - 05-ai-reading-list.md
    - mistakes.md
    - ../specs/profiles.md
    - ../specs/keys.md
    - ../specs/assets.md
---

# Decision Cards

Short cards. If a card is not enough, follow the link.

---

## 1. Which profile?

| I am writing… | Use |
| :--- | :--- |
| A how-to, setup, or tutorial | `guide` |
| A choice we already made (ADR) | `decision` |
| A product capability / PRD | `feature` |
| An on-call or ops runbook | `sop` |
| An HTTP/RPC contract | `api` |
| A system design | `architecture` |
| A governance rule | `policy` |
| Meeting notes | `meeting` |
| Q&A | `faq` |
| A release or deploy gate | `checklist` |
| An executable agent prompt (`agent.md`) | `agent` |
| A reusable skill (`SKILL.md`) | `skill` |
| Free-form notes, entity definitions, memory logs, computations | `note` (default) |

Teach-through: [Pick a shape](02-pick-a-shape.md). Templates: [`specs/profiles.md`](../specs/profiles.md).

---

## 2. YAML, heading, or `ods.toml`?

| Fact | Lives in |
| :--- | :--- |
| Document title | First `# H1` in the body (or top-level `title:` for OKF compatibility) |
| Summary, tags, owner, dates, OKF keys (`type`, `sources`, `verified`, `runtime`, `parameters`) | Top-level frontmatter |
| Profile, status, share, entity, domain, schema, invariants, depends, related, resources, code, context | Directly under `ods:` |
| Memory: tier, valid_from, valid_to, asserted_at, mutations, pin | Top-level `memory:` block |
| Procedure, decision text, guardrails, workflow, tools, eval | `##` body headings |
| Spec version, dialect, ignore paths, custom profiles, packs, schemas, aliases, ontology, memory, okf, attestation, service | Root `ods.toml` only |

Never: `tags` under `ods:`. Never: `profile` at top-level. Never: a nested `ods.ontology:` wrapper.

Deprecated but still parsing until 2.0: `ods.relations` (use `ods.related`), `ods.memory:` and the flat `ods.tier` / `valid_from` / … keys (use the top-level `memory:` block). Full list: [`specs/scope.md` §7.2](../specs/scope.md#72-deprecated-in-11--scheduled-for-removal-in-20).

---

## 3. `depends` vs `related` vs attachments

| Need | Key |
| :--- | :--- |
| The reader/agent must understand that **Markdown doc** first | `ods.depends` |
| Optional "see also" **Markdown doc** | `ods.related` |
| Human diagram / PDF (do not prompt-dump) | `ods.resources` |
| Named symbol in source | `ods.code` + `symbol` |
| A file more than a folder or two away | Any path key + an `@handle` |
| Docs an agent may only *act* on once reviewed | `ods.context.trust-min` |
| Small JSON/CSV/text the model must read | `ods.context.load` |
| Business class schema validator | `ods.schema` |

---

## 4. `entity` vs `tags` vs `type`

| Need | Key | Example |
| :--- | :--- | :--- |
| **Canonical Business Class** | `ods.entity` | `entity: Customer` |
| **Categorical Search Facets** | `tags` (Top-level) | `tags: [billing, security]` |
| **OKF Concept Type / Profile Alias** | `type` (Top-level) | `type: BigQuery Table` |

---

## 5. The 6 Canonical Document Recipes

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

Dictionary: [`specs/keys.md`](../specs/keys.md).

