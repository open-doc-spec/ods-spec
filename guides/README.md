---
description: "Start here to learn ODS from first principles: a step-by-step path from one Markdown file to an expert workspace."
tags:
  - learn
  - ods
  - onboarding
owner: team:ods
ods:
  profile: note
  status: stable
  related:
    - 00-why-ods.md
    - 01-first-document.md
    - 02-pick-a-shape.md
    - 03-link-documents.md
    - 04-bind-code-and-files.md
    - 05-ai-reading-list.md
    - 06-run-the-workspace.md
    - 07-extend-ods.md
    - decision-cards.md
    - faq.md
    - mistakes.md
    - ../specs/README.md
---

# Learn ODS: Novice to Expert

This folder is the **human front door** to Open Document Spec. Read it in order. Stop when the next page is more than you need.

The normative rules live in [`specs/`](../specs/README.md). Come back to them when you need a field definition, a lint rule ID, or an implementer contract.

---

## The ladder

| Level | Page | You can stop here if… |
| :---: | :--- | :--- |
| 0 | [Why ODS exists](00-why-ods.md) | You only wanted the idea. |
| 1 | [Your first document](01-first-document.md) | One trustworthy guide is enough. |
| 2 | [Pick a shape](02-pick-a-shape.md) | You write several kinds of docs and need the right headings. |
| 3 | [Link documents](03-link-documents.md) | Some docs are prerequisites for others. |
| 4 | [Bind files and code](04-bind-code-and-files.md) | The doc must point at a diagram, schema, or function. |
| 5 | [Give AI a reading list](05-ai-reading-list.md) | Agents should read a few files, not the whole repo. |
| 6 | [Run the workspace](06-run-the-workspace.md) | A team will lint, rename, and discover docs in CI. |
| 7 | [Extend ODS](07-extend-ods.md) | You need custom profiles, packs, or the engine contract. |

Pocket references (any time):

- [Decision cards](decision-cards.md) — which profile, which key, YAML vs headings
- [Common mistakes](mistakes.md) — the six errors that cause most lint failures
- [FAQ](faq.md) — short answers to the questions the spec buries in design notes

---

## How to use this track

1. Start at [00 · Why ODS exists](00-why-ods.md).
2. Do the steps. Each page uses the same **billing / refunds** example, grown one idea at a time.
3. When a page says **You can stop here**, believe it. The rest is optional.
4. Jump to [`specs/`](../specs/README.md) only for lookup, not for learning.

```text
You are here          After you adopt
────────────          ────────────────
guides/  (learn)  →   your repo's docs/
specs/   (look up)    ods.toml + ods lint
```
