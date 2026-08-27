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

## How do `depends` and `related` work with domain ontologies?

`ods.depends` is strictly for hard conceptual prerequisites that form an acyclic DAG. `ods.related` handles soft discovery links as well as typed semantic domain predicates (`is_a`, `part_of`, `owns`, `governed_by`, `maps_to`, `implements`).

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

## How does ODS 1.1 integrate with Google OKF?

ODS 1.1 is a strict superset of Google Open Knowledge Format (OKF v0.2). Any valid OKF bundle from Google's Knowledge Catalog is automatically 100% compliant ODS without file conversions or extra plugins.

## Why are ontology and memory keys flat under `ods:` instead of nested?

To eliminate YAML indentation fatigue. Adding an `ods.ontology:` wrapper adds nesting without adding meaning, so ontology keys sit directly under `ods:` — `ods.entity`, `ods.domain`, `ods.schema`, `ods.invariants`.

Memory is the exception, and deliberately so. Its fields (`tier`, `valid_from`, `valid_to`, `asserted_at`, `mutations`, `pin`) are one coherent record describing a single remembered fact, and they are read by memory tooling that has no interest in the rest of the document. Keeping them in a top-level `memory:` block means that record can be lifted out whole. The flat `ods.tier` / `ods.valid_from` / … spellings still parse but are deprecated — see [`specs/graph.md` §5.1](../specs/graph.md#51-canonical-placement).

## What is memory dreaming?

Dreaming is an asynchronous background process that consolidates episodic agent interaction traces into clean, living entity profiles, resolves contradictory facts, and prunes decayed historical memories.

## Should I start in `specs/` or `guides/`?

Humans: [`guides/README.md`](README.md). Implementers: [`specs/README.md`](../specs/README.md).
