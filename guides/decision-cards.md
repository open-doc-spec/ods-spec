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
| Profile, status, share, entity, domain, schema, relations, invariants, tier, valid_from, valid_to, mutations, pin, depends, related, resources, code, context | Directly under `ods:` |
| Procedure, decision text, guardrails, workflow, tools, eval | `##` body headings |
| Spec version, ignore paths, custom profiles, packs, aliases, ontology, memory | Root `ods.toml` only |

Never: `tags` under `ods:`. Never: `profile` at top-level. Never: nested namespace wrappers (`ods.ontology:` or `ods.memory:`).

---

## 3. `depends` vs `related` vs attachments

| Need | Key |
| :--- | :--- |
| The reader/agent must understand that **Markdown doc** first | `ods.depends` |
| Optional "see also" **Markdown doc** | `ods.related` |
| Human diagram / PDF (do not prompt-dump) | `ods.resources` |
| Named symbol in source | `ods.code` + `symbol` |
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

## 5. Minimum keys vs later keys

**Write these on day 1**

`description`, `tags`, `ods.profile`, `ods.status`

**Add when building domain ontologies**

`ods.entity`, `ods.domain`, `ods.schema`, `ods.relations`, `ods.invariants`

**Add when recording agent memory**

`ods.tier`, `ods.valid_from`, `ods.valid_to`, `ods.mutations`, `ods.pin`

**Add when two docs relate**

`ods.depends`, `ods.related`

**Add when the doc points at the world**

`ods.resources`, `ods.code`, `ods.context`

Dictionary: [`specs/keys.md`](../specs/keys.md).

