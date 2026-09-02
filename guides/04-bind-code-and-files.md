---
description: Attach diagrams, source files, and small prompt fixtures to an ODS
  document without wasting tokens or using line numbers.
tags:
- learn
- ods
- assets
- code
owner: team:ods
profile: guide
status: stable
depends:
- 03-link-documents.md
related:
- 05-ai-reading-list.md
- decision-cards.md
- mistakes.md
- ../specs/assets.md
---

# Bind Files and Code

## Overview

A document is more useful when it points at the artifact it describes: a flowchart, a handler function, a sample payload.

ODS has three attachment slots. They look similar and do different jobs. Picking the wrong one is the most common intermediate mistake.

| I have… | I put it in… | Does `ods lint` check the path? | Does an AI prompt receive it? |
| :--- | :--- | :---: | :---: |
| A diagram, PDF, or other human file | `resources` | Yes | **No** |
| Source the doc describes | `code` | Yes | Only if you ask (`--with-code`) |
| A small JSON/CSV the agent must read | `load` | Yes | **Yes** |

Canonical table (same facts, more rows): [`specs/assets.md`](../specs/assets.md) §3.

## Prerequisites

- A document that already has a profile ([Pick a shape](02-pick-a-shape.md)).
- Optional: a `depends` edge ([Link documents](03-link-documents.md)).
- The files you want to point at, sitting on disk in the same repo.

## Steps

### 1. Attach human artifacts as `resources`

The refunds flowchart is for people. It must exist; it must not be dumped into a prompt.

```yaml
profile: guide
status: draft
depends:
  - ../auth/sessions.md
resources:
  - ../diagrams/refund-flow.png
  - https://figma.com/file/refund-mockup
  - path: ../contracts/refund.yaml
    title: "Refund OpenAPI Contract"
```

`ods lint` verifies local files exist on disk. An AI context build ignores them. That is intentional: a 4 MB image is not a paragraph of instructions.

Never put `.ts` / `.rs` / `.py` here. Source uses `code`.

### 2. Bind implementation with `code` (string paths)

In ODS 2.0, `code` is a list of source file path strings:

```yaml
code:
  - apps/billing/src/refund.ts
  - apps/billing/tests/refund.test.ts
```

**No line numbers** — never `apps/billing/src/refund.ts:L45`. The next import you add shifts every line and the doc goes stale. A file path survives that edit.

**Keep bindings small.** If the source file is longer than ~300 lines of code, bind the directory or a smaller module file instead of a monolith. Large files make agent context expensive and obscure the relevant logic.

Source files are **not** ODS documents. Do not put frontmatter in `refund.ts`.

### 3. Put small prompt fixtures in `load`

The agent that implements refunds needs the request schema. That file is not a conceptual prerequisite (so not `depends`) and not a human PDF (so not `resources` as the only declaration).

```yaml
load:
  - ../schemas/refund-request.json
```

`load` is the surgical "put this text in the prompt" list. How the rest of the reading list is built is [Give AI a reading list](05-ai-reading-list.md).

You may also list the same JSON under `resources` if humans should see it in the asset catalog. Existence is then checked twice; only `load` injects it.

### 4. See the refunds file with attachments

```yaml
---
description: How to issue a customer credit-card refund from the billing dashboard.
tags:
  - billing
  - customer-care
profile: guide
status: draft
depends:
  - ../auth/sessions.md
related:
  - ../decisions/004-stripe.md
resources:
  - ../diagrams/refund-flow.png
code:
  - apps/billing/src/refund.ts
load:
  - ../schemas/refund-request.json
---
```

Every path is relative to `docs/guides/refunds.md`. Every path must exist.

### 5. Recheck the decision in one breath

- Human-only binary → `resources`
- Source file the prose describes → `code`
- Small text the model must see → `load`
- Another Markdown doc you must understand first → `depends` (previous page)
- Another Markdown doc that is optional → `related`

If you can sort a new file into one row, you are done with attachments.

## Troubleshooting

- **"Lint rejects `:L45`."** Remove the suffix. Point at the file, not a line.
- **"Should OpenAPI YAML be `resources` or `code`?"** Human contract / large spec → `resources`. If an agent must read a small excerpt, also `load`. `code` is for source files (`.ts`, `.py`, `.rs`, …).
- **"The PNG is huge; will lint load it?"** No. Lint checks the path exists. It does not open the pixels.
- **"My handler file is 800 lines."** Split the binding: point `code` at a smaller module, or describe the entry function in prose and bind a focused file under 300 LOC.

**You can stop here** if docs only need to point at files and functions.

**Next only if** an agent should receive a bounded bundle instead of the whole repo: [05 · Give AI a reading list](05-ai-reading-list.md).
