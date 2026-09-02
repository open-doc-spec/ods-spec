# Changelog

All notable changes to the Open Document Spec are documented here.

## [2.1.0] — 2026-09-01

### Added (optional Pareto ontology extension)

- **Frontmatter keys** — `entity`, `domain`, `schema` on concept documents.
- **Typed `related`** — five Pareto predicates (`is_a`, `part_of`, `owns`, `governed_by`, `maps_to`) plus `{ predicate: custom, verb, target }`.
- **Workspace `[ontology]`** — `default_domain`, `strict_schema` only.
- **Schemas** — `schemas/2.1.0/` (superset of 2.0).
- **Lint rules** — `ONT-001`, `ENT-001`, `ENT-002`, `ENUM-006`; `GRAPH-003` covers typed related targets.
- **Guide** — `guides/09-domain-ontology.md`.

Documents without `entity` or typed `related` remain valid 2.0/2.1 documents.

---

## [2.0.0] — 2026-09-01

### Breaking changes (clean break from 1.x)

ODS 2.0 does **not** read 1.x documents. Stay on the `v1.1.final` tag if you need 1.1.

- **Flat frontmatter only** — engine keys are top-level. The `ods:` wrapper is rejected.
- **Removed keys** — `memory:`, `entity`, `domain`, `schema`, `invariants`, `context`, `custom_profile`, typed `related` predicates, `code` object form.
- **Removed workspace tables** — `[ontology]`, `[memory]`, `[attestation]`, `[service]`.
- **Deleted schemas** — `schemas/1.0.0/`, `schemas/1.1.0/` removed.
- **`spec = "2.0"` only** in `ods.toml`.

### Added

- `title:` / `name:` with `TITLE-001` H1 sync.
- Top-level `load:` and `[context]` workspace defaults.
- `specs/engine.md` — reference engine contract.
- String-only `code:` paths.

### Removed

- `PROF-002`, `PLACE-*`, ontology/memory rule families, guide 07.

---

## [1.1.0] — frozen

Final 1.x release. Tag: `v1.1.final`. Not maintained after 2.0.
