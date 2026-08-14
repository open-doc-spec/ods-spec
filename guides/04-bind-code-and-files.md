---
description: "Attach diagrams, source symbols, and small prompt fixtures to an ODS document without wasting tokens or using line numbers."
tags:
  - learn
  - ods
  - assets
  - code
owner: team:ods
ods:
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
| :--- | :--- | :---: | :--- |
| A diagram, PDF, or other human file | `ods.resources` | Yes | **No** |
| Source the doc describes | `ods.code` + `symbol` | Yes | Only if you ask (`--with-code`) |
| A small JSON/CSV the agent must read | `ods.context.load` | Yes | **Yes** |

Canonical table (same facts, more rows): [`specs/assets.md`](../specs/assets.md) §3.

## Prerequisites

- A document that already has a profile ([Pick a shape](02-pick-a-shape.md)).
- Optional: a `depends` edge ([Link documents](03-link-documents.md)).
- The files you want to point at, sitting on disk in the same repo.

## Steps

### 1. Attach human artifacts as `resources`

The refunds flowchart is for people. It must exist; it must not be dumped into a prompt.

```yaml
ods:
  profile: guide
  status: draft
  depends:
    - ../auth/sessions.md
  resources:
    - path: ../diagrams/refund-flow.png
```

`ods lint` fails if the PNG is missing. An AI context build ignores it. That is intentional: a 4 MB image is not a paragraph of instructions.

Never put `.ts` / `.rs` / `.py` here. Source uses `code`.

### 2. Bind implementation with `code`, not line numbers

```yaml
ods:
  code:
    - path: apps/billing/src/refund.ts
      role: implementation
      symbol: processRefund
    - path: apps/billing/tests/refund.test.ts
      role: test
      symbol: TestProcessRefund
```

**Path + symbol**, never `apps/billing/src/refund.ts:L45`. The next import you add shifts every line and the doc goes stale. A function name survives that edit.

Start with two roles:

| Role | Use when the file is… |
| :--- | :--- |
| `entrypoint` | Where work starts (HTTP route, CLI, UI view). |
| `implementation` | The domain logic the prose describes. |
| `test` | The test that proves the doc's claims. |

The other five (`schema`, `migration`, `config`, `infrastructure`, `pipeline`) are listed in [`specs/assets.md`](../specs/assets.md). You do not need them for the refunds guide.

Source files are **not** ODS documents. Do not put frontmatter in `refund.ts`.

### 3. Put small prompt fixtures in `context.load`

The agent that implements refunds needs the request schema. That file is not a conceptual prerequisite (so not `depends`) and not a human PDF (so not `resources` as the only declaration).

```yaml
ods:
  context:
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
ods:
  profile: guide
  status: draft
  depends:
    - ../auth/sessions.md
  related:
    - ../decisions/004-stripe.md
  resources:
    - path: ../diagrams/refund-flow.png
  code:
    - path: apps/billing/src/refund.ts
      role: implementation
      symbol: processRefund
  context:
    load:
      - ../schemas/refund-request.json
---
```

Every path is relative to `docs/guides/refunds.md`. Every path must exist.

### 5. Recheck the decision in one breath

- Human-only binary → `resources`
- Named symbol in source → `code`
- Small text the model must see → `context.load`
- Another Markdown doc you must understand first → `depends` (previous page)
- Another Markdown doc that is optional → `related`

If you can sort a new file into one row, you are done with attachments.

## Troubleshooting

- **"Lint rejects `:L45`."** Remove the suffix. Add `symbol: processRefund`.
- **"Can I invent `role: helper`?"** No. Unknown code roles are errors. Pick the closest of the eight, or describe the helper in prose.
- **"Should OpenAPI YAML be `resources` or `code`?"** Human contract / large spec → `resources`. If an agent must read a small excerpt, also `context.load`. `code` + `role: schema` is for source-shaped schemas (Prisma, protobuf) tied to implementation.
- **"The PNG is huge; will lint load it?"** No. Lint checks the path exists. It does not open the pixels.

**You can stop here** if docs only need to point at files and functions.

**Next only if** an agent should receive a bounded bundle instead of the whole repo: [05 · Give AI a reading list](05-ai-reading-list.md).
