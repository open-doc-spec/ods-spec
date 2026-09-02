---
description: Short answers to common ODS 2.0 and 2.1 questions.
tags:
  - learn
  - ods
  - questions
owner: team:ods
profile: faq
status: stable
related:
  - 00-why-ods.md
  - decision-cards.md
  - mistakes.md
  - ../specs/core.md
  - ../specs/scope.md
  - ../specs/context.md
---

# FAQ

## Why not a new `.ods` file extension?

Files must open on GitHub, in VS Code, in Obsidian, and in every Static Site Generator without a plugin. `.md` is the whole point.

## Can I use `title:` in frontmatter?

Yes, in ODS 2.0. If you declare `title:` or `name:`, it must match the first `# H1` heading (`TITLE-001`). If you omit it, the H1 is the title.

## Why is there no `ods:` wrapper?

Flat keys are easier to author and lint. The `ods:` namespace was the #1 placement mistake in 1.x.

## How do `depends` and `related` differ?

`depends` lists hard Markdown prerequisites that context resolution walks. `related` lists soft links that are not traversed. In ODS 2.0, both are flat string paths. ODS 2.1 optionally adds typed predicates on `related` — see [Domain ontology](09-domain-ontology.md).

## What is ODS 2.1?

A **minor additive extension** on top of 2.0. It adds optional `entity`, `domain`, `schema`, and five Pareto predicates on `related`. Documents without those keys remain fully valid. Set `spec = "2.1"` in `ods.toml` to enable ontology lint rules.

## When should I use typed `related` predicates?

When you need machine-checkable domain semantics (`governed_by:`, `maps_to:`) between concept docs. For ordinary "see also" links, plain string paths are enough.

## Why doesn't `resources` go into the AI prompt?

It often holds multi-megabyte PDFs and images. Use `load:` for small text fixtures, or set `auto_load_resources = true` in `ods.toml`.

## Where is `max-depth` configured?

In `ods.toml` → `[context].default_max_depth` (default `2`). It is not a frontmatter key in 2.0.

## Why forbid line numbers in `code`?

`:L45` dies when someone adds an import. Bind whole file paths and keep files under ~300 LOC.

## Are profile section headings required?

No. Missing `## Steps` on a `guide` profile is fine. Structure is encouraged; it does not fail CI.

## Do I have to use the `ods` CLI?

No. The spec is the files on disk. The CLI is the reference engine. Another tool may implement the same contract.

## Does ODS 2.0 read 1.x documents?

No. ODS 2.0 is a clean break. Stay on the `v1.1.final` tag or manually flatten your frontmatter.

## How does OKF work in 2.0?

OKF v0.2 keys (`type`, `title`, `sources`, etc.) remain at the top level. Documents with `type:` or `okf_version:` skip `TITLE-001`.
