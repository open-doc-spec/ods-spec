---
description: Glossary of ODS 2.0 terms and concepts.
profile: note
status: stable
depends:
  - keys.md
  - graph.md
  - context.md
related:
  - README.md
---

# ODS · Glossary

## A

**Attachment slot** — One of `depends`, `related`, `resources`, `code`, or `load`. Each answers a different question about what a document links to.

## C

**Code binding** — A source file path listed under `code:`. String paths only in 2.0.

**Compliant workspace** — An ODS workspace where `ods lint` exits `0` (zero errors).

**Context assembly** — Phase 2 operation (`ods context`) that walks `depends` and injects `load` fixtures into an AI prompt bundle.

## D

**depends** — Hard Markdown prerequisites. Traversed during context resolution up to `default_max_depth`.

**Document ID** — Workspace-relative path without `.md` extension, unless overridden by `id:`.

## F

**Flat frontmatter** — ODS 2.0 authoring model: all engine keys (`profile`, `depends`, `code`, etc.) at the top level of YAML. No `ods:` wrapper.

## L

**load** — Top-level list of non-Markdown files injected into AI prompt context (JSON, CSV, schemas).

## P

**Profile** — Structural contract (`guide`, `decision`, `feature`, etc.) declared via `profile:`. Section headings are advisory in 2.0.

## R

**related** — Soft lateral document links. Not traversed during context resolution.

**resources** — Human-facing assets (diagrams, PDFs, URLs). Not auto-loaded into prompts unless `auto_load_resources = true`.

## W

**Workspace** — Directory tree whose root contains `ods.toml` with `spec = "2.0"` or `spec = "2.1"`.

**Workspace boundary** — Configuration that lives only in `ods.toml`: `spec`, `ignore`, `[context]`, `custom_profiles`, etc.

## Cross-references

| Term | Normative chapter |
| :--- | :--- |
| Flat keys | [keys.md](keys.md) |
| Graph edges | [graph.md](graph.md) |
| Context defaults | [context.md](context.md) |
| Lint rules | [validation.md](validation.md) |
| Versioning policy | [scope.md](scope.md#7-versioning-policy) |
