---
description: Engine implementation requirements for ODS 2.0/2.1 reference tooling (open-doc-spec/ods).
profile: note
status: stable
related:
  - validation.md
  - graph.md
  - ../guides/06-run-the-workspace.md
  - ../guides/09-domain-ontology.md
---

# ODS · Engine Implementation Contract (2.0 / 2.1)

This document specifies what the reference ODS engine ([open-doc-spec/ods](https://github.com/open-doc-spec/ods)) MUST implement for 2.0 adoption and the optional 2.1 Pareto ontology extension. The `ods-spec` repository ships schemas and conformance tests only.

## Required commands

| Command | Purpose |
| :--- | :--- |
| `ods init` | Create `ods.toml` with `spec = "2.0"` and sensible `ignore` defaults |
| `ods lint` | Validate workspace against `schemas/2.0.0/` (or `2.1.0/` when `spec >= "2.1"`) and Tier 2 rules in `validation.md` |
| `ods context <id>` | Assemble bounded prompt payload: walk `depends` to `default_max_depth`, inject `load` |
| `ods fmt` | Emit flat frontmatter; **error** if `ods:` wrapper is present |
| `ods adopt <path>` | Infer `profile` from headings; emit flat keys |
| `ods doctor` | Report hard errors for any legacy 1.x keys or `ods:` wrapper |
| `ods mv` / `ods archive` / `ods rm` | Graph-preserving lifecycle operations per `core.md` §5 |

## Hard rejection rules (no backward compatibility)

The engine MUST reject at parse/lint time:

- `ods:` wrapper in frontmatter
- `spec` values outside the supported enum for the loaded schema (`2.0`/`2.0.0` or `2.1`/`2.1.0`)
- Removed 1.x keys: `memory:`, `invariants`, `context`, `custom_profile`, `ods.relations`, `code` object form with `role`/`symbol`
- `[memory]`, `[attestation]`, `[service]` tables in `ods.toml`

On **`spec = "2.0"`** workspaces, the engine MUST also reject:

- `entity`, `domain`, `schema` frontmatter keys (not in the 2.0 document schema)
- Typed `related` predicate shorthand (string-only `related`)

On **`spec >= "2.1"`** workspaces, the engine MUST accept:

- `entity`, `domain`, `schema`
- Pareto predicate shorthand on `related` (`is_a`, `part_of`, `owns`, `governed_by`, `maps_to`)
- `{ predicate: custom, verb, target }` escape hatch

## ODS 2.1 ontology lint

When `spec >= "2.1"` or `packs` includes `@ods/pack-pareto-ontology`, the engine MUST:

1. **Parse `related` union entries** — distinguish plain paths, single-key predicate shorthand, and custom verb objects.
2. **Resolve predicate targets** — apply `GRAPH-003` to every Markdown path in typed `related` entries.
3. **Validate `schema` paths** — `ONT-001` when `schema` is present.
4. **Index entity names** — `ENT-001` (every `entity` value resolves to a definition doc) and `ENT-002` (no duplicate names).
5. **Reject unknown predicates** — `ENUM-006` at schema validation time.

Typed `related` edges are **not** auto-traversed during `ods context`; only `depends` is walked.

## Context defaults

Read from `ods.toml` `[context]`:

```toml
[context]
default_max_depth = 2
auto_load_resources = false
ignore = []
```

Per-document `max-depth` and `context.load` are **not** supported in 2.0+ frontmatter.

## TITLE-001 algorithm

```text
normalize(s) = lowercase(trim(collapse_whitespace(s)))
```

If `title` or `name` is present and document lacks `type:` or `okf_version:`:

- `TITLE-001` error when `normalize(title_or_name) != normalize(first_h1_text)`
- `TITLE-002` error when no `# H1` exists in body

## Dual-repo release

Ship `ods-spec` schemas and `ods` engine together. Spec-only releases without engine updates block real adoption.
