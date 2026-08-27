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
    - ../policy/refund-sla.md         # Simple lateral link
    - ../decisions/004-stripe.md
    - governed_by: @refund-sla.md     # Pareto single-key relation
```

`related` may point both ways and supports simple document paths or Pareto single-key relations (e.g. `governed_by`, `owns`, `is_a`, `part_of`, `see_also`). The predicate list is closed — see [`specs/graph.md` §4.1](../specs/graph.md#41-the-complete-predicate-vocabulary). `depends` is strictly for hard prerequisites.

**Test:** if an agent cannot do the job without that file, it is `depends`. If a human or agent might want it for context or domain lookup, it is `related`.

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

### 7. Stop counting `../` — use `@` handles

Relative paths break the moment somebody reorganizes a folder. Once a workspace exists, you can name a target instead of pathing to it:

```yaml
# Brittle: breaks if either file moves
ods:
  depends:
    - ../../../auth/sessions.md

# Durable: resolves by name, wherever the file lives
ods:
  depends:
    - "@sessions.md"
```

Three kinds of handle, all written with a leading `@`:

| You write | It resolves to |
| :--- | :--- |
| `@sessions.md` | The file named `sessions.md`, anywhere in the workspace. |
| `@Subscription` | The document that declares `ods.entity: Subscription`. |
| `@billing/index.md` | A specific `index.md`, when several share the basename. |

Handles work anywhere a path does — `depends`, `related`, `resources`, `schema`, and `code[].path`.

Two things to know:

- **A handle must be unique.** If two files are called `config.md`, lint reports an ambiguous handle and you add a folder prefix (`@billing/config.md`). This is a feature: it surfaces the ambiguity instead of silently picking one.
- **A typo is an error, not a guess.** `@sesions.md` fails lint rather than resolving to something nearby.

Quote handles in YAML (`"@sessions.md"`). A bare `@` at the start of a scalar is reserved in YAML and some parsers will reject it.

Full resolution rules: [`specs/graph.md` §4.4](../specs/graph.md#44-symbolic-entity--handle-resolution-handle).

## Troubleshooting

- **"Lint reports a dangling reference."** The relative path is wrong, or the file was moved by hand. Use `ods mv` (see [Run the workspace](06-run-the-workspace.md)) so inbound edges update together.
- **"Do Markdown `[links](../auth/sessions.md)` still count?"** They are for readers. The machine graph is `depends` / `related`. Prefer both: a prose link *and* a frontmatter edge when it is a real prerequisite.
- **"How deep can depends go?"** As deep as the subject needs. AI expansion stops at `max-depth` (default 2, ceiling 10) so a long chain does not explode a prompt. Humans can still walk the whole graph.
- **"Path or handle?"** Either is valid. Use a relative path for a close neighbour (`./auth.md`), a handle once you are reaching across more than a folder or two.

**You can stop here** if your docs only need "read this first" and "see also."

**Next only if** a document should point at a diagram, a schema, or a function in source: [04 · Bind files and code](04-bind-code-and-files.md).
