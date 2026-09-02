---
description: 'Start here to learn ODS from first principles: a step-by-step path from
  one Markdown file to an expert workspace.'
tags:
- learn
- ods
- onboarding
owner: team:ods
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
- 08-extend-ods.md
- 09-domain-ontology.md
- decision-cards.md
- faq.md
- mistakes.md
- ../specs/README.md
---

# Learn ODS: Novice to Expert

This folder is the **human front door** to Open Document Spec. Read it in order. Stop when the next page is more than you need.

The normative rules live in [`specs/`](../specs/README.md). Come back to them when you need a field definition, a lint rule ID, or an implementer contract.

---

## The 4-stage adoption path

| Stage | Page | You can stop here if… |
| :---: | :--- | :--- |
| 0 | [Why ODS exists](00-why-ods.md) | You only wanted the idea. |
| 1 | [Your first document](01-first-document.md) · [Pick a shape](02-pick-a-shape.md) | One trustworthy guide (or several shaped docs) is enough. |
| 2 | [Run the workspace](06-run-the-workspace.md) | A team will lint, rename, and discover docs in CI. |
| 3 | [Link documents](03-link-documents.md) | Some docs are prerequisites for others. |
| 4 | [Bind files and code](04-bind-code-and-files.md) · [Give AI a reading list](05-ai-reading-list.md) | Agents should read a few files, not the whole repo. |

Beyond the four stages:

| Page | When you need it |
| :--- | :--- |
| [Extend ODS](08-extend-ods.md) | Custom profiles, packs, or the engine contract. |
| [Domain ontology](09-domain-ontology.md) | ODS 2.1 entity definitions and typed `related` predicates. |

Pocket references (any time):

- [Decision cards](decision-cards.md) — which profile, which key, YAML vs headings
- [Common mistakes](mistakes.md) — the seven errors that cause most lint failures
- [FAQ](faq.md) — short answers to the questions the spec buries in design notes

Already know ODS and want to change it? [Contributing](../CONTRIBUTING.md) · [Changelog](../CHANGELOG.md)

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
