---
description: Operational heuristics, golden rules, and bounded context algorithms
  for AI agents authoring and navigating ODS workspaces.
tags:
- agent
- ods
- guidelines
- ai
owner: team:ods
created: 2026-08-14
updated: 2026-08-14
profile: note
status: stable
share: public
depends:
- specs/README.md
- specs/keys.md
related:
- specs/context.md
- specs/validation.md
- specs/profiles.md
---

# AGENTS.md — Agent Guidelines for Open Document Spec (ODS)

This document provides normative guidance and operational heuristics for AI coding agents, autonomous LLM tools, and prompt engines operating within an **ODS (Open Document Spec)** repository or authoring ODS documents.

---

## 1. Golden Rules for AI Agents

When reading, updating, or generating documentation in an ODS workspace, agents MUST follow these mandatory constraints:

1. **Flat Frontmatter — No `ods:` Wrapper**:
   - ODS 2.0 engine keys (`profile`, `status`, `share`, `depends`, `related`, `resources`, `code`, `load`) live at the **top level** of frontmatter.
   - The `ods:` namespace is **forbidden** in 2.0 documents. Parsers reject it outright.
   - Workspace boundary keys belong **only** in root `ods.toml`.
   - Canonical membership: [`specs/keys.md` §3](./specs/keys.md#3-the-two-layer-key-placement-architecture) and [`specs/indexes.md` §3](./specs/indexes.md#3-workspace-configuration-key-reference).

2. **Title Sync (`title:` / `name:` Optional)**:
   - The document title exists as the first `# H1` line in the Markdown body.
   - You MAY also declare `title:` or `name:` at the top level for OKF compatibility or SSG tooling — but if present, it **MUST** match the H1 exactly (`TITLE-001`).
   - Do not invent a `title:` that disagrees with the heading.

3. **Maintain Knowledge Graph Purity**:
   - `depends` is strictly for conceptual dependencies to other **Markdown documents**.
   - Do NOT place non-document fixtures (JSON schemas, sample CSVs, mock payloads) in `depends`. Put auxiliary prompt files in **`load`**.

4. **String Code Paths**:
   - In `code`, use simple string paths only: `code: ["src/main.rs", "tests/main.test.ts"]`.
   - File paths MUST NOT include line numbers (e.g. `:L45` is forbidden).
   - Do not use object forms with `role`, `symbol`, or `description` — those are 1.x shapes.

5. **Graph Links — Strings and Pareto Predicates**:
   - Use `depends` and `related` for document graph edges.
   - On ODS 2.0 workspaces, use plain path strings only (`- ../auth/sessions.md`).
   - On ODS 2.1 workspaces, `related` MAY use five Pareto predicates (`is_a`, `part_of`, `owns`, `governed_by`, `maps_to`) or `{ predicate: custom, verb, target }`. See [graph.md §4.4](specs/graph.md#44-pareto-ontology-predicates-on-related-ods-21).
   - Never use `@` symbolic handles — always workspace-relative Markdown paths.

6. **Preserve Third-Party and Unknown Frontmatter**:
   - If a document contains metadata for SSGs (e.g. Hugo `layout`, Astro `hero_image`, Jekyll `permalink`), agents MUST preserve those keys verbatim when editing the file.

7. **Path-Derived Document IDs**:
   - By default, a document's ID is its workspace-relative path without the `.md` extension (e.g., `guides/checkout.md` → `guides/checkout`).
   - Only specify an explicit `id` when renaming a file where you need to preserve existing inbound links without cascading rewrites.

8. **Graph Integrity & Acyclicity**:
   - Hard prerequisites belong in `depends`. Soft references belong in `related`.
   - The `depends` graph MUST NOT contain cyclic loops.

9. **Leverage JSON Schema for Syntactic Validation**:
   - When generating or updating frontmatter, validate against [`schemas/2.0.0/document.schema.json`](./schemas/2.0.0/document.schema.json) or [`schemas/2.1.0/document.schema.json`](./schemas/2.1.0/document.schema.json) when using ontology keys.
   - Recognize that `$schema` in frontmatter is optional; never reject or alter valid documents that omit `$schema`.

---

## 2. Bounded Context Loading Algorithm for Agents

When answering questions, planning code modifications, or debugging issues, agents SHOULD follow this bounded context expansion routine instead of scanning the entire workspace:

1. **Identify Entrypoint Document**: Identify the primary ODS document relevant to the user request (e.g. via `ods find` or `ods overview`).
2. **Auto-Expand Hard Dependencies**: Read the documents listed under `depends` recursively up to the workspace `[context].default_max_depth` (default: 2 hops; permitted range 0–10).
3. **Evaluate Trust & Staleness**: Check `verified` (infer trust tier) and skip documents where `now >= stale_after` or `now >= valid_to`.
4. **Load Auxiliary Fixtures**: Read any files listed under `load` and inspect attachments in `resources`.
5. **Inspect Code Bindings**: Use `code` to jump directly to declared source files.
6. **Respect Visibility**: If assembling public-facing exports or unprivileged summaries, skip any document or target marked `share: private`.

---

## 3. Standard Document Profile Shapes

When authoring new documents, pick the profile matching the document's intent and scaffold the expected H2 or H3 sections (`##` or `###`).

ODS ships 13 standard profiles: `note` (default), `guide`, `feature`, `decision`, `sop`, `api`, `architecture`, `policy`, `meeting`, `faq`, `checklist`, `agent`, and `skill`.

Canonical catalog with the exact expected sections for each profile: [`specs/profiles.md` §3](./specs/profiles.md#3-standard-profiles-catalog). Recognized heading synonyms: [`specs/profiles.md` §6](./specs/profiles.md#6-section-heading-alias-matching). Do not maintain a second copy here.

---

## 4. Document Templates

Scaffold new documents from the copy-paste templates in [`specs/profiles.md` §4](specs/profiles.md#4-complete-profile-templates-copy-paste-ready). The §4.11 `agent` and §4.12 `skill` templates cover autonomous agent and skill packages.

The minimal `guide` shape with `depends`, `code`, and `load` is illustrated in §4.2 of that chapter. Do not maintain a second copy of full templates here.

---
