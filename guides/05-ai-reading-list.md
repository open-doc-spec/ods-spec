---
description: "Assemble a bounded AI reading list from depends, context.load, ignore, and share — without dumping the repository into the prompt."
tags:
  - learn
  - ods
  - context
  - ai
owner: team:ods
ods:
  profile: guide
  status: stable
  depends:
    - 04-bind-code-and-files.md
  related:
    - 06-run-the-workspace.md
    - decision-cards.md
    - ../specs/context.md
---

# Give AI a Reading List

## Overview

Most AI tooling fails docs the same way: it embeds the whole tree, or it greps until the window is full. ODS does the opposite. You name an entrypoint. The engine walks **only** the hard prerequisites, injects the fixtures you listed, and stops.

That walk is called **bounded context**. You already declared most of it:

- `depends` — documents the agent must have read
- `context.load` — extra text files to inject
- `resources` — not injected
- `related` — not injected unless you opt in

This page shows what comes out the other side.

## Prerequisites

- A document with at least one `depends` edge ([Link documents](03-link-documents.md)).
- Optional attachments from [Bind files and code](04-bind-code-and-files.md).
- The `ods` CLI if you want to print the bundle. You can follow the walk-through without it.

## Steps

### 1. Start from the job, not from the repo root

The question is never "what is in `docs/`?" It is "what does an agent need to issue a refund?"

That entrypoint is `docs/guides/refunds.md`.

### 2. Read the refunds frontmatter as a reading list

```yaml
ods:
  profile: guide
  status: draft
  share: public
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
    max-depth: 2
    load:
      - ../schemas/refund-request.json
    ignore:
      - archive/
```

| Field | Effect on the bundle |
| :--- | :--- |
| `depends` | Walk these documents, then *their* `depends`, up to `max-depth`. |
| `related` | Skip (unless you pass `--include-related`). |
| `resources` | Skip. The PNG stays on disk. |
| `context.load` | Inject these files at the entrypoint. |
| `context.max-depth` | Default `2`. Raise only if a third hop is truly required. |
| `context.ignore` | Drop any path with this prefix, even if `depends` pointed at it. |
| `share: private` | Exclude from unprivileged / public exports. |
| `code` | Include only when the caller asks (`--with-code`). |

### 3. Walk the default bundle

Assume `sessions.md` depends on `docs/crypto/tokens.md`. Then:

```bash
ods context docs/guides/refunds.md
```

emits, deepest first:

1. `docs/crypto/tokens.md` — transitive prerequisite, depth 2
2. `docs/auth/sessions.md` — direct prerequisite, depth 1
3. `schemas/refund-request.json` — `context.load`
4. `docs/guides/refunds.md` — the entrypoint

Not in the bundle: the PNG, the ADR, `archive/**`, and `refund.ts` (until `--with-code`).

That is the point. A few thousand tokens, reproducible, no binary surprise.

### 4. Follow three rules that keep the list small

1. **Do not repeat `depends` targets in `load`.** The walk already includes them.
2. **Do not put fixtures in `depends`.** JSON is not a document. Use `load`.
3. **Do not raise `max-depth` to "get everything."** If hop 3 matters, it should probably be a direct `depends` on the entrypoint.

### 5. Hide what must not leave the building

```yaml
ods:
  share: private
```

Private documents are skipped when assembling public or unprivileged context. Use this for credentials runbooks, customer data, and anything you would not paste into an external model.

`org` means internal-ok. `public` (the default) means safe to export.

## Troubleshooting

- **"The bundle missed a file I care about."** It is probably `related`, behind `max-depth`, in `ignore`, or `share: private`. Promote it to `depends` or `load` if it is truly required.
- **"The bundle is huge."** A `depends` chain is wider than you think, or someone listed large files in `load`. Check `ignore` and stop loading PDFs.
- **"Why not embeddings / RAG?"** Similarity search is useful for exploration. It is not a substitute for "these three docs are required." ODS makes the required set explicit. Details: [`specs/context.md`](../specs/context.md).

**You can stop here** if agents can already start from one doc and receive a tight bundle.

**Next only if** a team will lint, rename, and discover these files every day: [06 · Run the workspace](06-run-the-workspace.md).
