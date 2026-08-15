---
description: "Choose an ODS profile from the job the document does, fill the expected headings, and keep execution contracts out of YAML."
tags:
  - learn
  - ods
  - profiles
  - authoring
owner: team:ods
ods:
  profile: guide
  status: stable
  depends:
    - 01-first-document.md
  related:
    - 03-link-documents.md
    - decision-cards.md
    - ../specs/profiles.md
---

# Pick a Shape

## Overview

A **profile** is the *shape* of a document: which H2 or H3 headings a reader should find. It is not a file type and not a layout template.

`guide` got you through [the first document](01-first-document.md). This page teaches you to pick the next shape without memorizing all thirteen standard profiles.

## Prerequisites

- A workspace with `ods.toml` and at least one document.
- Comfort with the three placement rules from [Your first document](01-first-document.md).

## Steps

### 1. Pick from the job, not from a catalog

Ask what the file is *for*:

```text
Are you teaching a procedure?
    yes → guide

Are you recording a choice you already made?
    yes → decision

Are you specifying a product capability?
    yes → feature

Are you writing an on-call / ops procedure?
    yes → sop

None of those / still exploring?
    → note   (default; no required headings)
```

That covers most engineering repos. The other profiles (`api`, `architecture`, `policy`, `meeting`, `faq`, `checklist`, `agent`, `skill`) wait until you have that job. Full templates: [`specs/profiles.md`](../specs/profiles.md). Pocket version: [Decision cards](decision-cards.md).

### 2. Fill the headings the profile promises

| Profile | Expected H2/H3 headings | Why those, not others |
| :--- | :--- | :--- |
| `guide` | Overview, Prerequisites, Steps, Troubleshooting | A how-to that cannot be followed is decoration. |
| `decision` | Context, Decision, Alternatives, Consequences | Future you needs the rejected options, not only the winner. |
| `feature` | Goal, Scope, Requirements, Acceptance Criteria, Risks | A PRD without acceptance criteria cannot be tested. |
| `sop` | Purpose, Prerequisites, Steps, Validation, Rollback | A runbook without rollback is an incident waiting. |
| `note` | *(none)* | Scratchpads should not fail lint. |

Heading names are matched loosely. `## Alternatives Considered` counts as `Alternatives`. `## Out of Scope` counts as `Non-Scope`. You do not need to memorize aliases; write the obvious heading.

Missing expected headings are **warnings**, not errors. The file is still compliant. The warning exists so a `decision` does not ship without `## Consequences`.

### 3. Grow the running example with a decision

The refunds guide assumes a payment provider. Record that choice as `docs/decisions/004-stripe.md`:

```markdown
---
description: Why the billing service uses Stripe as the card processor.
tags:
  - billing
  - architecture
ods:
  profile: decision
  status: stable
---

# ADR 004: Stripe for Card Payments

## Context
The billing service must capture and refund cards without storing PAN data.

## Decision
Use Stripe as the card processor.

## Alternatives
- Adyen: stronger international coverage, higher integration cost for this team.
- In-house processor: rejected; PCI scope is not worth the control.

## Consequences
Refunds go through Stripe's refund API. The dashboard must store `charge_id`, not raw card numbers.
```

Same three placement rules. Different shape. A reader who sees `profile: decision` can trust those four headings exist.

### 4. Use status as a lifecycle, not a review board

| Status | Means |
| :--- | :--- |
| `draft` | Changing; do not treat as source of truth. |
| `stable` | Checked and ready for others to follow. |
| `deprecated` | Still readable; superseded. Point at the replacement with `related` (next page). |
| `archived` | Kept for history; not maintained. |

There is no `in-review` or `needs-approval`. If your team wants a review flag, put it in a heading or a top-level key your SSG already understands — not a new `ods.status` value.

### 5. Keep execution contracts in headings, not in YAML

This is the mistake that shows up the moment people write agent instructions.

**Wrong** — operational keys pollute frontmatter:

```yaml
---
role: Autonomous TypeScript engineer
refusal_guardrails: [Never drop production]
workflow: [Inspect, Code, Test]
ods:
  profile: agent
---
```

**Right** — the profile already reserved body sections for that:

```markdown
---
description: Implement the refunds endpoint from the API contract.
ods:
  profile: agent
  status: draft
---

# Refunds Endpoint Agent

## Goal
Implement POST /api/v1/refunds against the published schema.

## Constraints
Never drop production data.

## Steps
1. Inspect the schema.
2. Write the handler.
3. Run the tests.
```

YAML is for facts a linter and an indexer should enforce. Prompts, guardrails, and workflows are prose. They belong under H2/H3 headings so any human or model can read them without a special parser.

## Troubleshooting

- **"Which profile if it is a bit of everything?"** Use `note` until the job is clear. Do not invent a hybrid.
- **"Can I add extra headings?"** Yes. Expected headings are a minimum, not a maximum.
- **"Can I invent `profile: rfc`?"** Yes, as a custom profile — that is [level 7](07-extend-ods.md). Until then, `feature` or `decision` is enough.
- **"Lint says unknown profile."** Typo, or a custom profile not listed in `ods.toml`. Unknown profiles warn and behave like `note`.

**You can stop here** if you can pick `note` / `guide` / `decision` / `feature` / `sop` and fill the headings.

**Next only if** one document should require another: [03 · Link documents](03-link-documents.md).
