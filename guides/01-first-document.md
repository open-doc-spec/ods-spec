---
description: "Create the smallest useful ODS workspace: ods.toml plus one guide with valid frontmatter and the four expected headings."
tags:
  - learn
  - ods
  - onboarding
  - authoring
owner: team:ods
ods:
  profile: guide
  status: stable
  depends:
    - 00-why-ods.md
  related:
    - 02-pick-a-shape.md
    - mistakes.md
    - ../specs/core.md
    - ../specs/keys.md
---

# Your First ODS Document

## Overview

This page gets you from zero to a valid workspace: a root `ods.toml` and one `guide`. That is enough ODS for a lot of teams.

You will learn only four frontmatter fields (`description`, `tags`, `ods.profile`, `ods.status`) and three placement rules that prevent most lint errors.

## Prerequisites

- You have read [Why ODS exists](00-why-ods.md).
- A folder you can put files in (a real repo, or a scratch directory).
- A text editor. The `ods` CLI is useful for checking, not required to write the files.

## Steps

### 1. Mark the workspace

Create `ods.toml` at the repository root with one required line:

```toml
# ods.toml — repository root only
spec = "0.1"
```

That file is the workspace boundary. Until it exists, a folder of Markdown is just a folder of Markdown.

You can add `ignore = ["node_modules", "target", "dist"]` later. You do not need it yet.

### 2. Write one guide

Create `docs/guides/refunds.md`. This is the running example used on every later page.

```markdown
---
description: How to issue a customer credit-card refund from the billing dashboard.
tags:
  - billing
  - customer-care
ods:
  profile: guide
  status: draft
---

# Refund Processing Guide

## Overview
Follow this procedure when a support agent issues a refund.

## Prerequisites
- Access to the billing dashboard.
- The original charge ID.

## Steps
1. Open **Billing** → **Transactions**.
2. Search for the charge ID.
3. Click **Issue Refund** and confirm the amount.

## Troubleshooting
If the request times out, check that the agent still has a valid session.
```

Copy it as-is. You will grow this file in later pages; you will not replace it.

### 3. Know why each piece is there

**Title lives in the `# H1`**, not in frontmatter. If you write both `title: Refunds` and `# Something Else`, they drift. ODS keeps one home: the heading.

**`description`** is one sentence for search, listings, and tooltips. Not a second copy of the overview.

**`tags`** are loose keywords. Keep them at the **top level**, never under `ods:`.

**`ods.profile: guide`** says "this file should look like a how-to." A guide is expected to have `##` or `###` headings for Overview, Prerequisites, Steps, and Troubleshooting. Those headings are for humans first; lint only warns if they are missing.

**`ods.status: draft`** means work in progress. Change it to `stable` when the steps are true.

### 4. Memorize the three placement rules

These three rules stop most first-week errors. The full dictionary is in [`specs/keys.md`](../specs/keys.md).

1. **No `title:` in frontmatter.** The first `# H1` in the body is the title.
2. **Universal keys stay on top:** `description`, `tags`, `owner`, `created`, `updated`.
3. **Engine keys nest under `ods:`:** `profile`, `status`, and (later) `depends`, `related`, `resources`, `code`, `context`.

Valid shape:

```yaml
---
description: One sentence.
tags:
  - billing
ods:
  profile: guide
  status: draft
---

# Visible Title
```

Invalid shape (do not do this):

```yaml
---
title: Visible Title          # wrong home
profile: guide                # engine key must sit under ods:
ods:
  tags: [billing]             # tags must sit at the top level
---
```

More broken examples: [Common mistakes](mistakes.md).

### 5. Check the workspace (optional but recommended)

If the `ods` CLI is installed:

```bash
ods lint .
```

- Exit `0` and zero errors → the workspace is **compliant**.
- Exit `1` → **non-compliant**. Fix the errors and run it again.

There is no "level 2 compliant." Warnings (for example a missing `## Troubleshooting`) do not fail CI. Errors do.

No CLI? Re-read the three placement rules and the four `guide` headings. You already have a valid document.

### 6. Decide whether to continue

You now have:

- A workspace marker
- One document a human can follow
- Metadata a tool can list and lint

A repository of independent guides like this is already ODS. You do not owe the graph, code bindings, or AI context to anyone.

## Troubleshooting

- **"Do I need `owner` and dates?"** No. Add `owner: team:billing` when accountability matters. Prefer Git history over hand-maintained `created` / `updated` unless you export docs out of Git.
- **"Can I skip frontmatter entirely?"** Yes. A bare `.md` file is a valid document. It is treated as profile `note`.
- **"The title rendered twice."** You probably left `title:` in frontmatter *and* wrote an H1. Delete the frontmatter key.
- **"I use Hugo / Astro `layout`."** Leave those keys at the top level. ODS preserves unknown keys.

**You can stop here** if one well-shaped guide (or a folder of them) is all you need.

**Next only if** you write more than how-tos — ADRs, PRDs, runbooks — and want the right headings: [02 · Pick a shape](02-pick-a-shape.md).
