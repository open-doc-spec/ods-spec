---
description: The seven ODS 2.0 authoring mistakes that cause most lint failures,
  with a wrong example and the fix for each.
tags:
- learn
- ods
- lint
- troubleshooting
owner: team:ods
profile: guide
status: stable
related:
- 01-first-document.md
- 03-link-documents.md
- 04-bind-code-and-files.md
- 02-pick-a-shape.md
- decision-cards.md
- ../specs/validation.md
---

# Common Mistakes

## Overview

Most first-month lint failures are the same seven mistakes. Each block below is the broken form, then the fix.

## Prerequisites

- The two placement rules in [Your first document](01-first-document.md).
- Optional: [`specs/validation.md`](../specs/validation.md) if you want the rule IDs.

## Steps

### 1. The `ods:` wrapper (`PLACE-001`)

ODS 2.0 forbids the `ods:` namespace. Engine keys are flat at the top level.

```yaml
# Wrong — 1.x shape
ods:
  profile: guide
  status: draft
  depends:
    - ../auth/sessions.md

# Right — 2.0 flat keys
profile: guide
status: draft
depends:
  - ../auth/sessions.md
```

### 2. `title:` / `name:` out of sync with H1 (`TITLE-001`)

```yaml
# Wrong — frontmatter disagrees with the heading
title: Refund Processing Guide
profile: guide
---

# Something Else Entirely

# Right — either omit title: or match the H1 exactly
profile: guide
---

# Refund Processing Guide
```

### 3. Keys on the wrong layer (`PLACE-002`)

```yaml
# Wrong — workspace keys in frontmatter
spec: "2.0"
ignore: [node_modules]

# Right — workspace keys in ods.toml only
# (frontmatter carries document keys; ods.toml carries workspace keys)
profile: guide
```

### 4. JSON (or any non-doc) in `depends` (graph purity)

```yaml
# Wrong
depends:
  - ../auth/sessions.md
  - ../schemas/refund-request.json

# Right
depends:
  - ../auth/sessions.md
load:
  - ../schemas/refund-request.json
```

`depends` is Markdown documents only.

### 5. A cycle in `depends` (`GRAPH-004`)

```yaml
# refunds.md
depends:
  - ../auth/sessions.md

# sessions.md
depends:
  - ../guides/refunds.md   # loop
```

Demote the weaker edge:

```yaml
# sessions.md
related:
  - ../guides/refunds.md
```

Or extract the shared fact into a third document both depend on.

### 6. Line numbers on code paths (`ASSET-003`)

```yaml
# Wrong
code:
  - src/refund.ts:L45

# Right
code:
  - src/refund.ts
```

### 7. Wrong attachment slot

```yaml
# Wrong — source in resources, fixture in depends
resources:
  - apps/billing/src/refund.ts
depends:
  - ../schemas/refund-request.json

# Right
code:
  - apps/billing/src/refund.ts
resources:
  - ../diagrams/refund-flow.png
load:
  - ../schemas/refund-request.json
depends:
  - ../auth/sessions.md
```

| Slot | Use for |
| :--- | :--- |
| `resources` | Diagrams, PDFs, URLs (human artifacts) |
| `code` | Source file paths |
| `load` | Small text/JSON the agent must read |
| `depends` | Markdown prerequisites |

## Troubleshooting

- **Error vs warning.** Missing `## Troubleshooting` on a guide is a warning. Anything in the list above except a missing heading is an error.
- **Still failing?** Read the rule ID in the diagnostic, then the matching row in [`specs/validation.md`](../specs/validation.md).
- **Migrating from 1.x?** Flatten every `ods:` block, rename `context.load` to `load`, simplify `code` to string paths, and bump `spec = "2.0"`. ODS 2.0 does not read 1.x documents.
- **More "why did we design it this way?"** [FAQ](faq.md).
