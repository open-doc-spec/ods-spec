---
description: "Operate an ODS workspace day to day: discover documents, lint in CI, and create, move, archive, or delete files without breaking the graph."
tags:
  - learn
  - ods
  - workspace
  - cli
owner: team:ods
ods:
  profile: guide
  status: stable
  depends:
    - 05-ai-reading-list.md
  related:
    - 07-extend-ods.md
    - ../specs/indexes.md
    - ../specs/validation.md
    - ../specs/core.md
---

# Run the Workspace

## Overview

Authoring is one job. Keeping a shared repo healthy is another.

This page is the operating habit: how you look around, how CI decides pass/fail, and how you create, rename, archive, or delete a document without leaving dead edges behind.

The full `ods.toml` schema and the lint rule IDs live in the spec. You only need a handful of commands here.

## Prerequisites

- An `ods.toml` at the repo root ([Your first document](01-first-document.md)).
- At least a few linked documents ([Link documents](03-link-documents.md)).
- The `ods` CLI for the command examples. The ideas still apply if you move files carefully by hand.

## Steps

### 1. Discover progressively — do not commit an index file

ODS does not want `docs/index.md` trees that every PR rewrites. Ask the workspace questions instead:

```text
ods overview          →  how many docs, which profiles, is lint clean?
ods find --tag billing
ods find --key status=draft
ods ls docs/guides
ods tree docs --depth 2
ods context docs/guides/refunds.md
```

That sequence is the daily loop: pulse → narrow → extract → act.

### 2. Treat lint as a binary CI gate

```bash
ods lint .
```

| Result | Meaning |
| :--- | :--- |
| Exit `0`, zero errors | **Compliant.** Warnings are allowed. |
| Exit `1`, one or more errors | **Non-compliant.** Fix, then re-run. |

Warnings: missing profile headings, missing profile-required metadata keys, unknown profile name, unknown key under `ods:`.
Errors: bad YAML, `title:` in frontmatter, engine keys at the top level, cycles in `depends`, missing paths, line numbers on `code` paths, illegal enums.

Rule IDs and remediations: [`specs/validation.md`](../specs/validation.md).

A typical GitHub Actions step is exactly `ods lint .`.

### 3. Grow `ods.toml` only when a setting earns its line

Minimum, which you already have:

```toml
spec = "0.1"
```

Useful next:

```toml
spec = "0.1"

ignore = [
  "src",
  "target",
  "node_modules",
  "dist"
]
```

`src` in `ignore` means "do not scan application source as if it were docs." It does **not** stop `ods.code` from pointing into `src`. Bindings still work; those files are just not treated as documents.

Leave `custom_profiles`, `packs`, and `[aliases]` until [Extend ODS](07-extend-ods.md).

### 4. Use lifecycle commands so the graph stays true

| Job | Command | What it also does |
| :--- | :--- | :--- |
| Create a scaffolded doc | `ods new docs/guides/chargebacks.md` | Writes valid frontmatter and the profile's heading placeholders. |
| Rename or move | `ods mv old.md new.md` | Rewrites inbound `depends`, `related`, `context.load`, and Markdown links. |
| Keep history, stop maintaining | `ods archive path.md` | Sets `status: archived`. Edges stay, so old context still resolves. |
| Delete | `ods rm path.md` | Removes the file **and** scrubs inbound edges so nothing dangles. |

Moving with `git mv` or Finder leaves every other file pointing at a ghost. That is the bug `ods mv` exists to prevent.

### 5. Adopt existing Markdown instead of rewriting it

A repo full of heading-shaped docs can be enrolled:

```bash
ods adopt docs/
```

The tool infers a profile from headings when it can (`## Context` + `## Decision` → `decision`) and leaves unknown frontmatter alone. You review the result; you do not start from a blank tree.

## Troubleshooting

- **"overview says non-compliant but I only have warnings."** Warnings do not fail the workspace. Look for an actual error ID.
- **"I ignored `src` and now my code bindings vanished."** They should not. `ignore` excludes documents, not `ods.code` targets. If a binding path is wrong, that is `ASSET-002`, not ignore.
- **"Where is `ods doctor` / LSP / skill install?"** Those live in the [engine repo](https://github.com/open-doc-spec/ods), not in this specification. This page is the portable contract: lint, discover, create, move, archive, delete.

**You can stop here** if the team can lint in CI and rename a file without breaking links.

**Next only if** you need custom profiles, heading aliases, packs, or the implementer map: [07 · Extend ODS](07-extend-ods.md).
