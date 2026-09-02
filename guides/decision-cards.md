---
description: 'Pocket decision cards for ODS: which profile to pick, where a fact belongs,
  and which attachment key to use.'
tags:
- learn
- ods
- reference
owner: team:ods
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
| Free-form notes | `note` (default) |

Teach-through: [Pick a shape](02-pick-a-shape.md). Templates: [`specs/profiles.md`](../specs/profiles.md).

---

## 2. YAML, heading, or `ods.toml`? (2-layer model)

| Fact | Lives in |
| :--- | :--- |
| Document title | First `# H1` in the body (optional `title:` / `name:` must match H1) |
| Summary, tags, owner, dates, OKF keys (`type`, `sources`, `verified`, `runtime`, `parameters`) | Top-level frontmatter |
| Profile, status, share, depends, related, resources, code, load | Top-level frontmatter (flat — no `ods:` wrapper) |
| Procedure, decision text, guardrails, workflow, tools, eval | `##` body headings |
| Spec version, ignore paths, custom profiles, packs, `[context]`, `[aliases]` | Root `ods.toml` only |

Never: an `ods:` wrapper in 2.0. Never: `spec` or `ignore` in document frontmatter.

---

## 3. `depends` vs `related` vs attachments

| Need | Key |
| :--- | :--- |
| The reader/agent must understand that **Markdown doc** first | `depends` |
| Optional "see also" **Markdown doc** | `related` |
| Domain semantics on `related` (ODS 2.1) | Typed predicates — [guide 09](09-domain-ontology.md) |
| Human diagram / PDF (do not prompt-dump) | `resources` |
| Source file the doc describes | `code` |
| Small JSON/CSV/text the model must read | `load` |

---

## 4. The 6 Primitives

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                       THE 6 ODS 2.0 PRIMITIVES                          │
├───────────────────┬─────────────────────────────────────────────────────┤
│ 1. DOCS           │ description + profile + status                      │
│ 2. LINKS          │ depends + related                                   │
│ 3. RESOURCES      │ resources (diagrams, PDFs, URLs)                    │
│ 4. CODE           │ code (string file paths)                            │
│ 5. LOAD           │ load (prompt fixtures)                              │
│ 6. LINT           │ ods lint (pass/fail CI gate)                        │
└───────────────────┴─────────────────────────────────────────────────────┘
```

Dictionary: [`specs/keys.md`](../specs/keys.md).
