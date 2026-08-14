---
description: "The six ODS authoring mistakes that cause most lint failures, with a wrong example and the fix for each."
tags:
  - learn
  - ods
  - lint
  - troubleshooting
owner: team:ods
ods:
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

Most first-month lint failures are the same six mistakes. Each block below is the broken form, then the fix.

## Prerequisites

- The three placement rules in [Your first document](01-first-document.md).
- Optional: [`specs/validation.md`](../specs/validation.md) if you want the rule IDs.

## Steps

### 1. `title:` in frontmatter (`SYNTAX-002`)

```yaml
# Wrong
title: Refund Processing Guide
ods:
  profile: guide

# Right — title is the H1
ods:
  profile: guide
---

# Refund Processing Guide
```

### 2. Keys on the wrong tier (`PLACE-001`, `PLACE-002`)

```yaml
# Wrong
profile: guide
ods:
  tags: [billing]

# Right
tags:
  - billing
ods:
  profile: guide
```

Universal on top. Engine under `ods:`. No exceptions.

### 3. JSON (or any non-doc) in `depends` (graph purity)

```yaml
# Wrong
ods:
  depends:
    - ../auth/sessions.md
    - ../schemas/refund-request.json

# Right
ods:
  depends:
    - ../auth/sessions.md
  context:
    load:
      - ../schemas/refund-request.json
```

`depends` is Markdown documents only.

### 4. A cycle in `depends` (`GRAPH-004`)

```yaml
# refunds.md
ods:
  depends: [../auth/sessions.md]

# sessions.md
ods:
  depends: [../guides/refunds.md]   # loop
```

Demote the weaker edge:

```yaml
# sessions.md
ods:
  related: [../guides/refunds.md]
```

Or extract the shared fact into a third document both depend on.

### 5. Line numbers on code paths (`ASSET-003`)

```yaml
# Wrong
ods:
  code:
    - path: src/refund.ts:L45
      role: implementation

# Right
ods:
  code:
    - path: src/refund.ts
      role: implementation
      symbol: processRefund
```

### 6. Execution keys in YAML (agent/skill anti-pattern)

```yaml
# Wrong
role: Autonomous engineer
refusal_guardrails: [Never drop prod]
workflow: [Inspect, Code, Test]
ods:
  profile: agent
```

Those words are headings the `agent` profile already expects (`## Constraints`, `## Steps`, `## Task`). Put them in the body. See [Pick a shape](02-pick-a-shape.md).

## Troubleshooting

- **Error vs warning.** Missing `## Troubleshooting` on a guide is a warning. Anything in the list above except a missing heading is an error (or, for tags-under-`ods:`, a warning that still needs fixing).
- **Still failing?** Read the rule ID in the diagnostic, then the matching row in [`specs/validation.md`](../specs/validation.md).
- **More "why did we design it this way?"** [FAQ](faq.md).
