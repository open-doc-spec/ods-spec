---
description: "Version history for the Open Document Spec: what each release added, deprecated, and clarified."
tags:
  - changelog
  - versioning
  - ods
owner: team:ods
ods:
  profile: note
  status: stable
  share: public
  related:
    - specs/scope.md
    - schemas/README.md
    - CONTRIBUTING.md
---

# Changelog

Versioning policy — what a MINOR may add and what a MAJOR may remove — is defined in [`specs/scope.md` §7.1](specs/scope.md#71-version-semantics).

---

## 1.1 · Consistency pass

No new keys. This release reconciles the specification with its own schemas, removes duplicated normative statements, and adds machine-checked guards so the two cannot drift apart again.

### Resolved contradictions

Where prose and schema disagreed, the schema won — it is the machine-checked artifact — except for `title:`, where both were wrong.

| Was | Now |
| :--- | :--- |
| Code roles counted as 8 in four chapters, 10 in the schema | **10 roles.** `interface` and `fixture` are documented in [`assets.md` §7](specs/assets.md#7-the-10-standard-code-roles-reference). |
| `title:` a hard `SYNTAX-002` error, while `core.md` required parsers to accept it for OKF | **Warning**, suppressed for OKF-flavoured documents. A spec cannot claim to accept any OKF bundle while erroring on one of its core keys. |
| `tags` under `ods:` a warning in prose, a hard schema failure in practice | **Error**, and extended to every universal key. |
| Memory tiers listed as 4; schema allowed 5 | **5 tiers.** `profile` is documented in [`graph.md` §5.2](specs/graph.md#52-the-5-memory-tiers). |
| `max-depth` ceiling stated as none, 5, and 10 in three places | **0–10**, enforced by `ENUM-005`. |
| `service.mode = "notify"` documented but rejected by the schema | **`poll` \| `watch`**. |
| `related` and `resources` auto-loading marked Yes in one matrix, No in another | `related` contributes **titles only**; `resources` contributes **nothing**. |
| Prose promised "dynamic `snake_case` verbs"; the schema rejected them | Predicates are a **closed set** with an explicit `predicate: custom` escape hatch. |
| `assets.md` required every resource to be a mapping with `path`; schema and fixtures allowed strings and URLs | Three entry shapes, defined in [`assets.md` §5.1](specs/assets.md#51-entry-shapes-normative). |
| `GRAPH-003` demanded every `related` target resolve to a document, though `related` holds entity handles | Split across `GRAPH-003`, `ENT-001`, and `ONT-003` by entry shape. |
| `README.md` described dialects as CLI flags; the schema had four different names | Four dialects, defined in [`indexes.md` §3.2](specs/indexes.md#32-dialects). |

### Newly documented

Features that were schema-valid but had no normative definition:

- **`ods.context.trust-min`** and the trust-tier derivation from `verified` — [`keys.md` §7.9](specs/keys.md#79-odscontext).
- **The complete predicate vocabulary**: core verbs, technical binding verbs, the six accepted aliases (`extends`, `contains`, `policy`, `rule`, `table`, `see`), and every attributed-object field — [`graph.md` §4.1](specs/graph.md#41-the-complete-predicate-vocabulary).
- **The `ods.toml` key reference** — every key, type, default, and required-ness, including the previously undocumented `[attestation]`, `[okf]`, `schemas`, `dialect`, and the full `[memory]` / `[service]` key sets — [`indexes.md` §3](specs/indexes.md#3-workspace-configuration-key-reference).
- **`spec` version format** and what a tool does with an unknown MAJOR or a newer MINOR.
- **`[aliases]` disambiguated** into `[aliases.sections]` (heading synonyms) and `[aliases.paths]` (document shortcuts). The two meanings had been used interchangeably.
- **Alias resolution order** — the built-in alias table overlaps itself (`Requirements`, `Success Criteria`, `FAQ` are each canonical somewhere and a synonym elsewhere), so matching precedence is now normative — [`profiles.md` §6.2](specs/profiles.md#62-alias-resolution-normative).
- **Custom profile keys** `required_sections`, `optional_sections`, `description`, and the rule for declaring sections in frontmatter versus body headings.
- **The minimal conformant document** — [`core.md` §3.0](specs/core.md#30-minimal-conformant-document).
- **Implementer conformance profiles** (Core / Graph / Context / Full) and the required capabilities, replacing the implicit assumption that every tool is a full engine — [`validation.md` §2.1](specs/validation.md#21-implementer-conformance-profiles).

### Deprecated · removal targeted at 2.0

Each still parses and emits a warning. Precedence rules in [`specs/scope.md` §7.2](specs/scope.md#72-deprecated-in-11--scheduled-for-removal-in-20).

- `ods.relations` → `ods.related` (`DEPR-001`)
- `ods.memory:` and flat `ods.tier` / `valid_from` / `valid_to` / `asserted_at` / `mutations` / `pin` → the top-level `memory:` block (`DEPR-002`)
- Nested `ods.toml` table forms of `spec`, `ignore`, `custom_profiles`, `packs` → the flat forms (`DEPR-003`)

### New rules

`ENUM-004` (memory tier), `ENUM-005` (max-depth bounds), `ENUM-006` (closed predicate set), `ASSET-005` (resource declares exactly one of `path`/`url`), `MEM-004` (conflicting memory placement), `DEPR-001`–`DEPR-003`.

### Single sources of truth

Duplicated normative statements were the direct cause of most contradictions above. Each concept now has one home, and every other mention links to it:

| Concept | Canonical home |
| :--- | :--- |
| Profile catalog and expected sections | [`profiles.md` §3](specs/profiles.md#3-standard-profiles-catalog) |
| The 6 canonical recipes | [`keys.md` §1.1](specs/keys.md#11-novice-quick-start-the-6-canonical-document-recipes) |
| Subsystem auto-load / lint matrix | [`keys.md` §4](specs/keys.md#4-subsystem-matrix-of-engine-keys) |
| 3-layer key placement | [`keys.md` §3](specs/keys.md#3-the-3-layer-key-placement-architecture) |
| `ods.toml` keys (Layer 3) | [`indexes.md` §3](specs/indexes.md#3-workspace-configuration-key-reference) |
| `@` handle resolution | [`graph.md` §4.4](specs/graph.md#44-symbolic-entity--handle-resolution-handle) |
| Predicate vocabulary | [`graph.md` §4.1](specs/graph.md#41-the-complete-predicate-vocabulary) |
| Memory semantics and tiers | [`graph.md` §5](specs/graph.md#5-cognitive-memory--bi-temporal-traversal) |
| Knowledge Graph Purity | [`graph.md` §7](specs/graph.md#7-knowledge-graph-purity-normative) |
| Code roles | [`assets.md` §7](specs/assets.md#7-the-10-standard-code-roles-reference) |
| BCP 14 conformance language | [`specs/README.md` §1](specs/README.md#1-conformance-language) |

### Fixed

- 24 broken internal links, 22 of them glossary cross-references orphaned by an earlier chapter renumbering.
- `schemas/README.md` advertised `x-ods-spec` annotations; the schemas carry `x-ods-lifecycle`.
- `README.md` and `core.md` listed 3 schemas; 6 ship.
- A sample `ods overview` in a 1.1 chapter printed `ODS v0.1`.

### Schema status

`ontology.schema.json`, `memory.schema.json`, and `attestation.schema.json` are now marked **experimental** and excluded from the conformance contract. They describe a second, richer modelling path that competes with the flat Pareto keys ODS 1.1 standardized; their keys (`guardrails.mutation_gates`, `decision{}`, `entity.extends`, `actor`, `session_id`) have no normative chapter. They remain published for review.

### Conformance suite

6 tests → 14. New guards: internal link and anchor resolution, prose↔schema enum agreement, `ods.toml` enum agreement, `max-depth` bound agreement, deprecation-record completeness, both config dialects plus negative config fixtures, and a Tier 2 fixture corpus with a coverage test that fails when a rule is added to the matrix without an example. CI now runs on push and pull request.

---

## 1.1 · Initial release

Additive over 1.0. No key was removed; a valid 1.0 document is a valid 1.1 document, and the only migration step is bumping `spec` in `ods.toml`.

**Added at the top level:** `author`, `created_at`, `updated_at`; the Google OKF v0.2 superset (`type`, `title`, `resource`, `sources`, `usage_window`, `generated`, `verified`, `stale_after`, `okf_version`); attested computations (`runtime`, `parameters`, `computation`, `executor`, `attester`); the cognitive `memory:` block.

**Added under `ods:`:** domain ontology (`entity`, `domain`, `schema`, `invariants`); typed predicates and attributed objects in `related`; `@` symbolic handles; `context.trust-min`; the `interface` and `fixture` code roles.

**Added to `ods.toml`:** `dialect`, `schemas`, `[ontology]`, `[memory]`, `[attestation]`, `[okf]`, `[aliases.sections]`, `[aliases.paths]`.

Per-key introduction versions are recorded as `x-ods-lifecycle.introduced` in the schemas; the reconstructed delta table lives in [`schemas/README.md`](schemas/README.md#superseded--version-100).

---

## 1.0 · Baseline

The original format model: optional YAML frontmatter with an `ods:` engine namespace, path-derived document IDs, `depends` / `related` graph edges, `resources` and `code` bindings, bounded `context`, structural profiles, and the root `ods.toml` workspace marker. Schemas preserved at [`schemas/1.0.0/`](schemas/1.0.0/).
