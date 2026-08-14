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
| Anything else, or not sure | `note` |

Teach-through: [Pick a shape](02-pick-a-shape.md). Templates: [`specs/profiles.md`](../specs/profiles.md).

---

## 2. YAML, heading, or `ods.toml`?

| Fact | Lives in |
| :--- | :--- |
| Document title | First `# H1` in the body |
| One-line summary, tags, owner, optional dates | Top-level frontmatter |
| Profile, status, share, id, depends, related, resources, code, context | Under `ods:` |
| Procedure, decision text, guardrails, workflow, tools, eval | `##` body headings |
| Spec version, ignore paths, custom profiles, packs, aliases | Root `ods.toml` only |

Never: `title:` in YAML. Never: `tags` under `ods:`. Never: `profile` at the top level. Never: `role:` / `workflow:` / `refusal_guardrails:` in YAML.

---

## 3. `depends` vs `related` vs attachments

| Need | Key |
| :--- | :--- |
| The reader/agent must understand that **Markdown doc** first | `ods.depends` |
| Optional "see also" **Markdown doc** | `ods.related` |
| Human diagram / PDF (do not prompt-dump) | `ods.resources` |
| Named symbol in source | `ods.code` + `symbol` |
| Small JSON/CSV/text the model must read | `ods.context.load` |

One file, one primary home. A schema may be both `resources` (catalog) and `load` (prompt). It must not be in `depends`.

---

## 4. Minimum keys vs later keys

**Write these on day 1**

`description`, `tags`, `ods.profile`, `ods.status`

**Add when two docs relate**

`ods.depends`, `ods.related`

**Add when the doc points at the world**

`ods.resources`, `ods.code`, `ods.context`

**Rare**

`ods.id` (rename stability), `ods.share` (privacy), `owner`, `created`, `updated`

Dictionary: [`specs/keys.md`](../specs/keys.md).
