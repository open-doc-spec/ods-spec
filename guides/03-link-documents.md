---
description: "Link ODS documents with depends and related, use path-derived IDs, and keep the knowledge graph acyclic and document-only."
tags:
  - learn
  - ods
  - graph
  - authoring
owner: team:ods
ods:
  profile: guide
  status: stable
  depends:
    - 02-pick-a-shape.md
  related:
    - 04-bind-code-and-files.md
    - decision-cards.md
    - mistakes.md
    - ../specs/graph.md
---

# Link Documents

## Overview

Until now each file stood alone. Real docs have prerequisites: you cannot follow the refunds guide until you understand sessions.

ODS records that as an explicit edge in frontmatter, not as a sentence a tool cannot check.

Two edges. That is the whole graph:

| Edge | Means | Cycles allowed? | Loaded for AI by default? |
| :--- | :--- | :---: | :---: |
| `ods.depends` | Read this first. Hard prerequisite. | No | Yes |
| `ods.related` | See also. Soft pointer. | Yes | No |

## Prerequisites

- Two or more Markdown files in the same workspace.
- The placement rules from [Your first document](01-first-document.md).
- A profile chosen from [Pick a shape](02-pick-a-shape.md).

## Steps

### 1. Add the prerequisite document

Create `docs/auth/sessions.md`:

```markdown
---
description: How dashboard sessions are created, validated, and revoked.
tags:
  - auth
ods:
  profile: guide
  status: stable
---

# Dashboard Sessions

## Overview
Every billing action, including refunds, requires a valid session token.

## Prerequisites
- Access to the auth service.

## Steps
1. Sign in through the dashboard.
2. Confirm the session cookie is present.
3. Refresh if the token is older than 12 hours.

## Troubleshooting
A 401 on refunds almost always means the session expired.
```

### 2. Point the refunds guide at it

In `docs/guides/refunds.md`, add `depends`:

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
---
```

Read this as: **you must understand sessions before you act on refunds.**

Paths are relative to the current file, and they must exist. A typo is a lint error, not a silent dead link.

### 3. Use `related` for "see also"

The refunds SLA is useful background. It is not a prerequisite:

```yaml
ods:
  profile: guide
  status: draft
  depends:
    - ../auth/sessions.md
  related:
    - ../policy/refund-sla.md
    - ../decisions/004-stripe.md
```

`related` may point both ways. `depends` may not.

**Test:** if an agent cannot do the job without that file, it is `depends`. If a human might want it later, it is `related`.

### 4. Let IDs come from paths

You do not invent slugs.

```text
docs/guides/refunds.md     →  docs/guides/refunds
docs/auth/sessions.md      →  docs/auth/sessions
```

Write `ods.id` only when you rename a heavily linked file and must keep the old identity for a while. Almost nobody needs this in week one.

### 5. Never close a loop in `depends`

This fails lint:

```text
refunds.md  --depends-->  sessions.md
sessions.md --depends-->  refunds.md    ← cycle
```

Fix: one of those edges is not actually a prerequisite. Demote it to `related`, or extract the shared fact into a third document that both depend on.

```text
refunds.md   --depends-->  sessions.md
sessions.md  --related-->  refunds.md    ← allowed
```

### 6. Keep the graph as documents only

`depends` is for **Markdown documents**. JSON schemas, CSVs, and diagrams are not graph nodes.

```yaml
# Right
ods:
  depends:
    - ../auth/sessions.md

# Wrong — that JSON is not a document
ods:
  depends:
    - ../schemas/refund-request.json
```

The JSON goes in `ods.context.load` when an agent must read it. That is the next two pages.

Do not hand-write backlinks. If refunds depends on sessions, you do not also list refunds on sessions. Tools compute inbound links.

## Troubleshooting

- **"Lint reports a dangling reference."** The relative path is wrong, or the file was moved by hand. Use `ods mv` (see [Run the workspace](06-run-the-workspace.md)) so inbound edges update together.
- **"Do Markdown `[links](../auth/sessions.md)` still count?"** They are for readers. The machine graph is `depends` / `related`. Prefer both: a prose link *and* a frontmatter edge when it is a real prerequisite.
- **"How deep can depends go?"** As deep as the subject needs. AI expansion stops at `max-depth` (default 2) so a long chain does not explode a prompt. Humans can still walk the whole graph.

**You can stop here** if your docs only need "read this first" and "see also."

**Next only if** a document should point at a diagram, a schema, or a function in source: [04 · Bind files and code](04-bind-code-and-files.md).
