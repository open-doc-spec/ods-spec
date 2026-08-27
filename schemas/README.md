# Open Document Spec (ODS) JSON Schemas

This directory contains the official, versioned machine-readable **JSON Schemas (Draft 2020-12)** for Open Document Spec (ODS).

---

## Schema Status

Schemas carry a status. Only **normative** schemas are part of the ODS 1.1 conformance contract; a tool MUST NOT fail a document for violating an experimental schema.

### Normative — Version 1.1.0

| Schema | File | Canonical `$id` URL | Validates |
| :--- | :--- | :--- | :--- |
| **Document Frontmatter** | [`1.1.0/document.schema.json`](1.1.0/document.schema.json) | `…/schemas/1.1.0/document.schema.json` | Markdown YAML frontmatter: the 3-layer key placement, engine keys under `ods:`, and Google OKF v0.2 top-level keys. |
| **Workspace Configuration** | [`1.1.0/config.schema.json`](1.1.0/config.schema.json) | `…/schemas/1.1.0/config.schema.json` | Repository root `ods.toml`. Key reference: [`specs/indexes.md` §3](../specs/indexes.md#3-workspace-configuration-key-reference). |
| **Custom Profile Definition** | [`1.1.0/profile.schema.json`](1.1.0/profile.schema.json) | `…/schemas/1.1.0/profile.schema.json` | `ods.custom_profile` blocks in registered profile-definition files. |

### Experimental — published for review, not required for conformance

These describe richer modelling shapes under consideration for a future revision. They compete with, rather than extend, the flat Pareto keys that ODS 1.1 standardized, so they are deliberately held outside the conformance contract until that overlap is resolved. Their keys are **not** defined in any normative chapter.

| Schema | File | Covers |
| :--- | :--- | :--- |
| **Domain Ontology** | [`1.1.0/ontology.schema.json`](1.1.0/ontology.schema.json) | Structured entity property lists, `entity.extends`, and `guardrails.mutation_gates` (`pre_conditions` / `post_conditions`). The normative equivalents are `ods.entity`, `ods.domain`, `ods.schema`, `ods.invariants`, and `ods.related`. |
| **Agent Memory** | [`1.1.0/memory.schema.json`](1.1.0/memory.schema.json) | `actor`, `session_id`, `temporal.superseded_by`, and the `decision{problem, cause, alternatives, optimal_choice, post_eval, failure_mode}` record. The normative equivalent is the top-level `memory:` block ([`specs/graph.md` §5](../specs/graph.md#5-cognitive-memory--bi-temporal-traversal)). |
| **Attestation** | [`1.1.0/attestation.schema.json`](1.1.0/attestation.schema.json) | A standalone view of `runtime` / `parameters` / `computation` / `executor` / `attester`. Those keys are already normative on `document.schema.json`; this file exists for tools that validate computations in isolation. |

### Superseded — Version 1.0.0

[`1.0.0/`](1.0.0/) holds the ODS 1.0 baseline schemas (`document`, `config`, `profile`). They remain published so existing pins keep resolving. **New workspaces MUST target 1.1.**

**What changed in 1.1** (reconstructed from `x-ods-lifecycle.introduced`):

| Added in 1.1 | Where |
| :--- | :--- |
| `author`, `created_at`, `updated_at` | Top level |
| OKF v0.2 superset: `type`, `title`, `resource`, `sources`, `usage_window`, `generated`, `verified`, `stale_after`, `okf_version` | Top level |
| Attested computations: `runtime`, `parameters`, `computation`, `executor`, `attester` | Top level |
| Cognitive memory: the `memory:` block (`tier`, `valid_from`, `valid_to`, `asserted_at`, `mutations`, `pin`) | Top level |
| Domain ontology: `entity`, `domain`, `schema`, `invariants` | Under `ods:` |
| Typed predicates and attributed objects in `related`; `@` symbolic handles | Under `ods:` |
| `context.trust-min` | Under `ods:` |
| `interface` and `fixture` code roles | `ods.code[].role` |
| `dialect`, `[ontology]`, `[memory]`, `[attestation]`, `[okf]`, `[aliases.sections]`, `[aliases.paths]`, `schemas` | `ods.toml` |

Nothing was removed in 1.1. A valid 1.0 document is a valid 1.1 document; the only migration step is bumping `spec` in `ods.toml`. Legacy flat engine keys (`profile:` / `status:` at top level) predate 1.0's `ods:` namespace and are still accepted on read — see [`specs/core.md` §5.2](../specs/core.md#52-legacy-frontmatter-migration-ods-fmt---migrate).

### Deprecated in 1.1 · removal targeted at 2.0

Carrying `x-ods-lifecycle.status: "deprecated"` with `deprecated_in` and `removed_in`:

- `ods.relations` → use `ods.related` (`DEPR-001`)
- `ods.memory:` and flat `ods.tier` / `valid_from` / `valid_to` / `asserted_at` / `mutations` / `pin` → use the top-level `memory:` block (`DEPR-002`)
- Nested `ods.toml` table forms of `spec`, `ignore`, `custom_profiles`, `packs` → use the flat forms (`DEPR-003`)

Full rationale and schedule: [`specs/scope.md` §7](../specs/scope.md#7-deprecations--versioning-policy).

---

## Spec-to-Schema Traceability Matrix (`x-ods-lifecycle`)

To ensure seamless maintenance, every key in `document.schema.json` carries an `x-ods-lifecycle` annotation pointing directly to its normative governing chapter in `specs/`:

| Frontmatter Key | Placement Layer | Governing Specification Chapter | Normative Section Anchor |
| :--- | :--- | :--- | :--- |
| `description` | Layer 1 / OKF | [`specs/keys.md`](../specs/keys.md) | `#description` |
| `tags` | Layer 1 / OKF | [`specs/keys.md`](../specs/keys.md) | `#tags` |
| `owner`, `author` | Layer 1 | [`specs/keys.md`](../specs/keys.md) | `#owner-and-author` |
| `created` / `created_at`, `updated` / `updated_at` | Layer 1 | [`specs/keys.md`](../specs/keys.md) | `#created-and-created_at`, `#updated-and-updated_at` |
| `type` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#okf-type-interoperability` |
| `title` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#okf-title-handling` |
| `resource` | Layer 1 (OKF Native) | [`specs/assets.md`](../specs/assets.md) | `#okf-resource-uri` |
| `sources` / `usage_window`| Layer 1 (OKF Native) | [`specs/graph.md`](../specs/graph.md) | `#provenance-sources` |
| `generated` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md) | `#actor-generated-metadata` |
| `verified` | Layer 1 (OKF Native) | [`specs/context.md`](../specs/context.md) | `#trust-tier-verification` |
| `status` / `stale_after` | Layer 1 (OKF Native) | [`specs/core.md`](../specs/core.md), [`specs/context.md`](../specs/context.md) | `#status-lifecycle`, `#staleness-gating` |
| `runtime`, `parameters`, `executor`, `attester` | Layer 1 (OKF Attested Computation) | [`specs/assets.md`](../specs/assets.md) | `#attested-computation-contracts` |
| `memory` (`tier`, `valid_from`, `valid_to`, `asserted_at`, `mutations`, `pin`) | Layer 1 / Layer 2 | [`specs/context.md`](../specs/context.md), [`specs/graph.md`](../specs/graph.md) | `#cognitive-memory-block` |
| `ods.profile` | Layer 2 (`ods:`) | [`specs/profiles.md`](../specs/profiles.md) | `#profile-contracts` |
| `ods.status`, `ods.id`, `ods.share` | Layer 2 (`ods:`) | [`specs/core.md`](../specs/core.md), [`specs/context.md`](../specs/context.md) | `#status-lifecycle`, `#visibility-boundaries` |
| `ods.entity`, `ods.domain` | Layer 2 (Domain Modeling) | [`specs/graph.md`](../specs/graph.md) | `#domain-entity-classes` |
| `ods.schema` | Layer 2 (Domain Modeling) | [`specs/validation.md`](../specs/validation.md) | `#schema-disk-contracts` |
| `ods.invariants` | Layer 2 (Domain Modeling) | [`specs/validation.md`](../specs/validation.md) | `#neuro-symbolic-invariants` |
| `ods.depends` | Layer 2 (`ods:`) | [`specs/graph.md`](../specs/graph.md) | `#knowledge-graph-depends` |
| `ods.related` | Layer 2 (`ods:`) | [`specs/graph.md`](../specs/graph.md) | `#discovery-graph-and-semantic-relations` |
| `ods.resources` | Layer 2 (`ods:`) | [`specs/assets.md`](../specs/assets.md) | `#asset-catalog` |
| `ods.code` | Layer 2 (`ods:`) | [`specs/assets.md`](../specs/assets.md) | `#source-code-bindings` |
| `ods.context` (`max-depth`, `trust-min`, `load`, `ignore`) | Layer 2 (`ods:`) | [`specs/keys.md`](../specs/keys.md), [`specs/context.md`](../specs/context.md) | `#79-odscontext` |
| `ods.relations` *(deprecated)* | Layer 2 (`ods:`) | [`specs/keys.md`](../specs/keys.md) | `#713-odsrelations-deprecated` |
| `ods.tier`, `ods.pin`, `ods.valid_from`, `ods.valid_to`, `ods.asserted_at`, `ods.mutations` *(deprecated)* | Layer 2 (`ods:`) | [`specs/graph.md`](../specs/graph.md) | `#51-canonical-placement` |
| `ods.custom_profile` | Layer 2 (`ods:`) | [`specs/profiles.md`](../specs/profiles.md) | `#711-profile-definition-metadata` |

---

## Editor Autocompletion & Validation

### 1. VS Code & Cursor (via YAML Extension)

To get instant autocomplete, hover tooltips, and real-time error squiggles in Markdown frontmatter, add this line at the very top of your frontmatter:

```markdown
---
# yaml-language-server: $schema=https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json
description: Authentication guide for API tokens.
tags: [auth, security]
owner: team:security
ods:
  profile: guide
  status: stable
---
```

Alternatively, configure VS Code's `.vscode/settings.json` workspace settings to apply the schema automatically to all Markdown files:

```json
{
  "yaml.schemas": {
    "https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/document.schema.json": [
      "*.md",
      "docs/**/*.md",
      "specs/**/*.md",
      "guides/**/*.md"
    ]
  }
}
```

### 2. `ods.toml` Validation in Editors (Taplo TOML)

Add the schema association in `.vscode/settings.json` or root `Taplo.toml`:

```json
{
  "evenBetterToml.schema.associations": {
    "ods.toml": "https://raw.githubusercontent.com/open-doc-spec/ods-spec/main/schemas/1.1.0/config.schema.json"
  }
}
```
