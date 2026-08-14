---
description: "Short answers to the questions ODS specs usually bury in design-decision sections."
tags:
  - learn
  - ods
  - questions
owner: team:ods
ods:
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

## Why is the title not a frontmatter key?

Two titles drift. The heading is what humans see, so it is the only title.

## Why are there no compliance levels?

"Level 2 of 3" made teams argue about whether CI should pass. `ods lint` either exits 0 or 1.

## Why only `depends` and `related`? Where is `implements`?

Automation only needs "required" vs "optional." Richer ontologies add authoring cost without changing the context walk. Explain nuance in prose.

## Why doesn't `resources` go into the AI prompt?

Because it often holds multi-megabyte PDFs and images. `context.load` is the explicit, small, text-only injection list.

## Why default `max-depth` to 2?

Two hops along real prerequisites usually covers the architecture you need. Deeper walks grow exponentially and drown the prompt.

## Why not hand-written backlinks?

They rot on the first rename. Declare the edge on the dependent document. Let tools compute inbound links.

## Why forbid line numbers in `ods.code`?

`:L45` dies when someone adds an import. A symbol name does not.

## Why can't I invent a ninth code role?

A closed set is how an external agent classifies unknown repos. If it does not fit, pick the nearest role and describe the rest in prose.

## Why are profile headings warnings, not errors?

Adoption must not punish a draft. Structure is encouraged; a missing `## Risks` should not break the build.

## Why TOML for the workspace file and YAML for documents?

Workspace config is typed tables. Document metadata is the YAML authors already write for Hugo and Astro. Mixing both in YAML made the two layers harder to tell apart.

## Do I have to use the `ods` CLI?

No. The spec is the files on disk. The CLI is the reference engine that lints and builds context. Another tool may implement the same contract.

## Is ODS a competitor to Docusaurus / Hugo / Obsidian?

No. Those render or navigate. ODS labels and links the Markdown they already consume. Unknown keys are preserved.

## Should I start in `specs/` or `guides/`?

Humans: [`guides/README.md`](README.md). Implementers: [`specs/README.md`](../specs/README.md).
