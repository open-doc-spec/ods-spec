---
description: "ODS lint rules, binary compliance contract, lifecycle phase separation, unknown-content behavior, diagnostic format, and tooling validation requirements."
ods:
  profile: "note"
  status: "stable"
  depends:
    - README.md
    - core.md
    - keys.md
  related:
    - graph.md
    - indexes.md
    - assets.md
    - profiles.md
    - ../guides/06-run-the-workspace.md
    - ../guides/mistakes.md
---

# ODS · Validation & Tooling Contract

This document specifies the **Normative Validation Contract** for Open Document Spec (ODS): lint rules, rule severities, unknown-content behavior, diagnostic message format, and implementer checklists.

## At a glance

- **What this chapter defines:** Binary compliance, conformance profiles, the lint rule matrix, unknown-key behavior, and the implementer checklist.
- **Why it exists:** Conformance is what `ods lint` can prove, not what an author intended.
- **When you need it:** You are wiring CI, reading an error ID, or implementing a linter.
- **When you can skip it:** You are still writing your first documents — [common mistakes](../guides/mistakes.md) is enough.
- **Learn this first:** [Run the workspace](../guides/06-run-the-workspace.md) · [Common mistakes](../guides/mistakes.md)
- **Prerequisite chapters:** [core.md](core.md), [keys.md](keys.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14, exactly as stated in [README.md §1](README.md#1-conformance-language). That is the canonical statement; do not maintain a second copy here.

---

## 2. Binary Compliance Contract

Conformance for ODS metadata is defined by **validation, not intention**.

- An ODS workspace is evaluated as **Compliant** or **Non-Compliant**.
- When `ods lint` is executed:
  - If **zero errors** are reported: the tool MUST exit with status code **`0`** (**Compliant**).
  - If **one or more errors** are reported: the tool MUST exit with status code **`1`** (**Non-Compliant**).
- Warnings (such as missing optional profile sections) SHOULD be reported to authors but MUST NOT cause a non-zero exit code. Unrecognized profile names are `PROF-001` errors.

```bash
# CI Conformance Check
$ ods lint .
✓ Checked 48 documents across workspace.
✓ 0 errors, 2 warnings. Workspace is COMPLIANT.
$ echo $?
0
```

---

## 2.1 Implementer Conformance Profiles

Not every ODS tool needs to be a full engine. A frontmatter linter, an editor extension, and a context-assembling agent runtime implement different slices of this specification. A conformant tool MUST declare which profile it implements so that users can predict what it will and will not catch.

| Profile | A tool at this level MUST implement | Typical implementation |
| :--- | :--- | :--- |
| **Core** | Tier 1 only: YAML parsing, 3-layer key placement, all `SYNTAX-*`, `PLACE-*`, `ENUM-*`, and `CODE-*` rules, and unknown-key preservation. | Editor extension, pre-commit hook, JSON Schema validator. |
| **Graph** | Core, plus workspace discovery and every Tier 2 rule over documents: `GRAPH-001..004`, `SYM-001..003`, `ENT-001..002`, `ASSET-001..005`, `PROF-001..006`. | CI linter. |
| **Context** | Graph, plus the bounded context resolution algorithm in [context.md §6](context.md#6-the-context-resolution-algorithm-normative), including `share`, `ignore`, staleness, and trust-tier filtering. | Agent runtime, prompt assembler. |
| **Full** | Context, plus the ontology (`ONT-*`), memory (`MEM-*`), OKF (`OKF-*`), and attestation (`ATT-*`) rule families. | Reference engine. |

Each profile is a strict superset of the one above it. A tool MUST NOT claim a profile while silently skipping rules it contains; it MAY report an unimplemented rule as "not checked" rather than as a pass.

The **binary compliance contract in §2 is scoped to the declared profile**: `ods lint` at profile *Graph* exits `0` when no Graph-level error is found, and makes no claim about ontology or attestation validity.

---

## 2.2 Required Capabilities (Not a Command Surface)

This specification describes **capabilities**, not a CLI. Every `ods <command>` line in these chapters is a **non-normative illustration** using the reference engine's spelling; a conformant tool may expose the same capability as a library call, a language server request, an editor action, or a differently named command.

A tool claiming a conformance profile MUST provide the capabilities marked for that profile:

| Capability | What it MUST do | Required at |
| :--- | :--- | :--- |
| **Validate** | Evaluate a workspace against the rule matrix for the declared profile and report a binary pass/fail plus per-rule diagnostics. | Core |
| **Adopt** | Ingest untyped Markdown without rewriting it; infer `ods.profile` from headings per [core.md §7](core.md#7-smart-profile-inference-heuristics). | Graph |
| **Format** | Normalize key placement and emit ordering per [keys.md §5](keys.md#5-canonical-emit-ordering) while preserving every unknown key verbatim. | Graph |
| **Scaffold** | Create a document with valid frontmatter and the section placeholders of its profile. | Graph |
| **Relocate** | Move a document and rewrite every inbound reference (`depends`, `related`, `context.load`, inline links, relative resource paths). | Graph |
| **Archive / Delete** | Set `status: archived` preserving edges, or remove a document and scrub every inbound reference. | Graph |
| **Resolve context** | Execute the bounded context algorithm and emit a deterministic, ordered, token-bounded payload. | Context |

Beyond the exit-code contract in §2, this specification does **not** constrain flag names, output formats, or command names. Those belong to each implementation.

---

## 3. The Two Lifecycle Phases

ODS enforces clear separation between verification in CI and context resolution for AI agents:

| Phase | Command | What is Checked / Executed | Primary Purpose |
| :--- | :--- | :--- | :--- |
| **Phase 1: Authoring & Verification** | `ods lint` | • YAML syntax & schema constraints<br>• 3-tier key placement<br>• DAG acyclicity (no cycles in `depends`)<br>• File existence on disk for `resources`, `code`, and `load`<br>• Absence of line numbers in code paths | Enforces repository health, consistency, and zero dead links in CI. |
| **Phase 2: AI Context Resolution** | `ods context <id>` | • Traverses `depends` up to `max-depth` (default: 2)<br>• Ingests `context.load` text files<br>• Prunes `context.ignore` and `share: private`<br>• Emits unified bounded prompt payload | Assembles deterministic prompt context within token budget. |

---

## 3.1 Two-Tier Validation Architecture

To ensure high performance and determinism across both offline linters and language servers, validation is split into two explicit tiers:

1. **Tier 1 (Schema Layer - Zero-IO / Fast AST)**:
   - Validates document YAML frontmatter against [`document.schema.json`](../schemas/1.1.0/document.schema.json) and `ods.toml` against [`config.schema.json`](../schemas/1.1.0/config.schema.json) using standard JSON Schema Draft 2020-12 validators.
   - Enforces key placement (3-tier model), forbidden keys (e.g. `title:` via `SYNTAX-002`), enum validity (`status`, `share`, `code[].role`), and string regex patterns.
   - Executes in `<1ms` per document with zero filesystem I/O.
2. **Tier 2 (Semantic & Graph Layer - Deep Workspace Analysis)**:
   - Validates multi-document graph properties: DAG acyclicity (`GRAPH-004`), unique document IDs (`GRAPH-001`), disk file resolution for assets and source code (`ASSET-001..004`), and Markdown body heading conformance (`PROF-002`).

---

## 4. Normative Lint Rules Matrix

All conformant ODS linters MUST enforce the following validation rules:

| Category | Rule Identifier | Validation Tier | Rule Condition | Severity | Remediation Action |
| :--- | :--- | :---: | :--- | :---: | :--- |
| **Syntax** | `SYNTAX-001` | **Tier 1 (Schema)** | Frontmatter MUST parse as valid YAML delimited by `---`. | **Error** | Fix YAML syntax error. |
| | `SYNTAX-002` | **Tier 1 (Schema)** | Frontmatter carries a `title:` key while showing no OKF signal (`type`, `okf_version`, or `sources`). Suppressed entirely for OKF-flavoured documents. | **Warning** | Move the title to the first `# H1` body heading. Never an error: the OKF v0.2 superset requires `title:` to be accepted. See [core.md §3.1](core.md#31-frontmatter). |
| **Placement** | `PLACE-001` | **Tier 1 (Schema)** | Universal keys (`tags`, `description`, `owner`, `author`, `created`, `created_at`, `updated`, `updated_at`) MUST appear at the top level; they MUST NOT be nested under `ods:`. | **Error** | Hoist the key to top-level frontmatter. The document schema rejects these names under `ods:` outright, so this cannot be downgraded to a warning. |
| | `PLACE-002` | **Tier 1 (Schema)** | Engine keys (`profile`, `status`, `depends`, etc.) MUST be nested under `ods:`. | **Error** | Nest engine keys under `ods:` mapping. |
| **Enums** | `ENUM-001` | **Tier 1 (Schema)** | `ods.status` MUST be one of `draft`, `stable`, `deprecated`, `archived`. | **Error** | Change status to a recognized lifecycle state. |
| | `ENUM-002` | **Tier 1 (Schema)** | `ods.share` (when present) MUST be one of `public`, `org`, `private`. | **Error** | Set share to `public`, `org`, or `private`. |
| | `ENUM-004` | **Tier 1 (Schema)** | Memory `tier` MUST be one of `episodic`, `semantic`, `procedural`, `state`, `profile`. | **Error** | Change tier to a recognized cognitive tier. |
| | `ENUM-005` | **Tier 1 (Schema)** | `ods.context.max-depth` MUST be an integer in the range `0`–`10`. | **Error** | Lower `max-depth` to 10 or below. |
| | `ENUM-006` | **Tier 1 (Schema)** | A relation predicate MUST belong to the closed vocabulary in [graph.md §4.1](graph.md#41-the-complete-predicate-vocabulary). | **Error** | Use a standard predicate, or `{ predicate: custom, custom_predicate: <verb> }`. |
| | `ENUM-003` | **Tier 1 (Schema)** | `ods.code[].role` MUST be one of the 10 standard roles listed in [assets.md §7](assets.md#7-the-10-standard-code-roles-reference). Applies only to the mapping entry shape; a bare-string entry has no `role` and defaults to `implementation`. | **Error** | Change role to a valid standard role (e.g. `entrypoint`, `implementation`, `interface`). |
| **Graph** | `GRAPH-001` | **Tier 2 (Semantic)** | Document IDs MUST be unique across the workspace. | **Error** | Rename duplicate file or override via `ods.id`. |
| | `GRAPH-002` | **Tier 2 (Semantic)** | `ods.depends` targets MUST resolve to existing documents. | **Error** | Fix or remove dangling dependency path. |
| | `GRAPH-003` | **Tier 2 (Semantic)** | An `ods.related` target that is a **document reference** (a bare string path, or a `@name.md` file handle) MUST resolve to an existing document. Entity handles are checked by `ENT-001` instead, and non-document targets of `maps_to` by `ONT-003`. | **Error** | Fix or remove dangling related path. |
| | `GRAPH-004` | **Tier 2 (Semantic)** | `ods.depends` graph MUST NOT contain cyclic dependency loops. | **Error** | Break circular dependency loop using `ods.related`. |
| **Entities & Symbols** | `ENT-001` | **Tier 2 (Semantic)** | An `ods.related` target that is an **entity handle** (`@Subscription`, or a bare PascalCase identifier with no path separator or file extension) MUST resolve to a document declaring that `ods.entity`. | **Error** | Check entity symbol spelling or declare `ods.entity: Name` in target document. |
| | `ENT-002` | **Tier 2 (Semantic)** | Entity class names declared in `ods.entity` MUST be unique across the workspace. | **Error** | Rename duplicate entity class or disambiguate with `ods.domain`. |
| | `SYM-001` | **Tier 2 (Semantic)** | `@` symbolic file or entity handle MUST resolve to a unique target in the workspace. | **Error** | Check `@handle` spelling or create missing target file/entity. |
| | `SYM-002` | **Tier 2 (Semantic)** | Ambiguous `@` handle matching multiple files MUST be disambiguated with parent folder prefix. | **Error** | Add folder prefix (e.g. `@billing/index.md`). |
| | `SYM-003` | **Tier 2 (Semantic)** | Relative path traverses $>2$ directory levels when a clean `@handle` is available. | **Warning** | Replace brittle `../../` path with clean `@handle`. |
| **Assets & Code** | `ASSET-001` | **Tier 2 (Semantic)** | A local `ods.resources` entry — a bare string without a URL scheme, or a mapping with `path` — MUST resolve to an existing file. URL entries (bare `https://…` strings, or mappings with `url`) are syntax-checked only and MUST NOT be network-fetched. | **Error** | Fix path or verify file existence on disk. |
| | `ASSET-005` | **Tier 1 (Schema)** | An `ods.resources` mapping MUST carry exactly one of `path` or `url`. | **Error** | Remove the redundant key, or add the missing one. |
| | `ASSET-002` | **Tier 2 (Semantic)** | `ods.code[].path` MUST resolve to an existing file. | **Error** | Fix path or verify source code file on disk. |
| | `ASSET-003` / `CODE-001` | **Tier 1 (Schema)** | `ods.code[].path` MUST NOT contain line number suffixes (e.g. `:L45`). | **Error** | Remove `:L45`; use `symbol` field instead. |
| | `CODE-002` | **Tier 1 (Schema)** | In `ods.code`, `path`, `role`, and `description` MUST be singular; only `symbol` MAY be an array. | **Error** | Make `path`/`role` singular string/enum. |
| | `ASSET-004` | **Tier 2 (Semantic)** | `ods.context.load` paths MUST resolve to existing files. | **Error** | Fix or remove dangling context load path. |
| **Profiles** | `PROF-001` | **Tier 1 (Schema)** | `ods.profile` MUST resolve to a known standard or registered custom profile. | **Error** | Fix the profile name or define and register the profile at the path declared in `ods.toml`. |
| | `PROF-002` | **Tier 2 (Semantic)** | Document SHOULD contain expected H2 or H3 sections (`##` or `###`) for its declared profile. | **Warning** | Add missing section heading or registered alias. |
| | `PROF-003` | **Tier 1 / 2** | A document SHOULD contain each non-null top-level key listed by its selected custom profile's `required_keys`. | **Warning** | Add the missing key to top-level frontmatter; do not nest it under `ods:`. |
| | `PROF-004` | **Tier 1 / 2** | A document SHOULD NOT contain a top-level key listed by its selected custom profile's `forbidden_keys`. | **Warning** | Remove the forbidden key or choose a profile that permits it. |
| | `PROF-005` | **Tier 2 (Semantic)** | Every `custom_profiles` path in `ods.toml` MUST resolve to an existing Markdown file or profile directory. | **Error** | Create the profile definition at the exact configured path or update the `custom_profiles` entry. |
| | `PROF-006` | **Tier 1 / 2** | `ods.custom_profile` MUST appear only in a profile-definition file selected by `custom_profiles` (or a registered pack). | **Error** | Move the definition to its registered path and use `ods.profile` in ordinary documents. |
| **Ontology** | `ONT-001` | **Tier 2 (Semantic)** | `ods.schema` MUST resolve to an existing schema file on disk. | **Error** | Fix path or verify schema existence on disk. |
| | `ONT-002` | **Tier 1 (Schema)** | `ods.invariants` expressions MUST parse as valid boolean expressions. | **Error** | Correct invariant expression syntax. |
| | `ONT-003` | **Tier 2 (Semantic)** | A relation target that names a non-document artifact (a dataset, table, or endpoint reached via `maps_to`) MUST resolve to an existing path on disk when it is written as a path. Also applies to targets of the deprecated `ods.relations`. | **Error** | Fix the broken semantic relation target. |
| | `ONT-004` | **Tier 2 (Semantic)** | `is_a` inheritance relations MUST NOT contain cyclic loops. | **Error** | Break circular entity inheritance loop. |
| **Memory** | `MEM-001` | **Tier 1 (Schema)** | `ods.valid_to` MUST NOT be chronologically earlier than `ods.valid_from`. | **Error** | Ensure valid_to $\ge$ valid_from. |
| | `MEM-002` | **Tier 2 (Semantic)** | `ods.mutations[].entity` SHOULD resolve to a declared entity class in the workspace. | **Warning** | Declare entity in workspace or check spelling. |
| | `MEM-003` | **Tier 2 (Semantic)** | Stale memory node exceeds retention window without `ods.pin: true`. | **Warning** | Pin memory node or allow pruning during dreaming. |
| **Deprecations** | `DEPR-001` | **Tier 1 (Schema)** | `ods.relations` is deprecated in favor of `ods.related`. | **Warning** | Move the entries into `ods.related`; the two are merged and de-duplicated by `(predicate, target)`. |
| | `DEPR-002` | **Tier 1 (Schema)** | `ods.memory:` and the flat `ods.tier` / `valid_from` / `valid_to` / `asserted_at` / `mutations` / `pin` keys are deprecated in favor of the top-level `memory:` block. | **Warning** | Move the fields into the top-level `memory:` block. |
| | `DEPR-003` | **Tier 1 (Schema)** | Nested `ods.toml` tables (`spec = { version }`, `ignore = { paths }`, `custom_profiles = { paths }`, `packs = { load }`) are deprecated in favor of the flat forms. | **Warning** | Rewrite using the flat form documented in [indexes.md §3](indexes.md#3-workspace-configuration-key-reference). |
| | `MEM-004` | **Tier 1 (Schema)** | The same memory field is declared with **conflicting values** in more than one placement. | **Error** | Delete the duplicate. Precedence when values agree: `memory:` > `ods.memory:` > flat `ods.*`. |
| **OKF & Attest** | `OKF-001` | **Tier 1 (Schema)** | OKF concepts MUST contain a non-empty `type` field. | **Error** | Provide non-empty `type` string. |
| | `OKF-002` | **Tier 1 (Schema)** | `sources[].resource` MUST NOT be empty. | **Error** | Provide canonical URL or file path in sources entry. |
| | `OKF-003` | **Tier 1 (Schema)** | Concept is stale if `now >= stale_after`. | **Warning** | Refresh concept or update staleness window. |
| | `ATT-001` | **Tier 1 (Schema)** | `type: Attested Computation` MUST declare a `runtime`. | **Error** | Specify execution runtime (e.g. `bigquery`, `postgres`, `dbt`, `python`). |
| | `ATT-002` | **Tier 2 (Semantic)** | Attested computation parameter values MUST match declared `parameters` schema. | **Error** | Pass valid typed parameters. |
| | `ATT-003` | **Tier 2 (Runtime)** | Attester verification code MUST return exit code 0 when inspecting execution receipt. | **Error** | Fix computation logic or check attester assertion. |

---

## 5. Commented Rule Violation Examples

### 5.1 Placement & Title Errors (`SYNTAX-002`, `PLACE-002`)
```yaml
# ERRONEOUS CODE:
---
title: "Checkout Guide"               # WARNING [SYNTAX-002]: no OKF signal; title belongs in the H1
profile: guide                        # ERROR [PLACE-002]: engine key at top level
ods:
  tags: [billing]                     # ERROR [PLACE-001]: universal key nested under ods:
---

# CORRECTED CODE:
---
tags:
  - billing                           # CORRECT: tags at top level
ods:
  profile: guide                      # CORRECT: engine key nested under ods:
  status: stable
---

# Checkout Guide                      # CORRECT: Title declared as first H1 in body
```

### 5.2 Line Numbers in Code Bindings (`ASSET-003`)
```yaml
# ERRONEOUS CODE:
ods:
  code:
    - path: src/checkout.ts:L45-L60   # ERROR [ASSET-003]: line numbers prohibited
      role: implementation

# CORRECTED CODE:
ods:
  code:
    - path: src/checkout.ts           # CORRECT: clean relative file path
      role: implementation
      symbol: processCheckout         # CORRECT: refactor-resilient symbol reference
```

### 5.3 Cyclic Dependency Loops (`GRAPH-004`)
```yaml
# ERRONEOUS CODE (Doc A depends on Doc B, Doc B depends on Doc A):
# In auth.md:
ods:
  depends: [session.md]

# In session.md:
ods:
  depends: [auth.md]                  # ERROR [GRAPH-004]: Cyclic dependency detected

# CORRECTED CODE:
# In auth.md:
ods:
  depends: [session.md]               # Hard prerequisite

# In session.md:
ods:
  related: [auth.md]                  # CORRECT: Changed to soft related link (cycles permitted)
```

---

## 6. Unknown-Content Behavior (Normative)

| Encountered Content | Tooling Behavior |
| :--- | :--- |
| **Unknown Top-Level Frontmatter Key** (e.g. `layout`, `hero_image`) | **Preserve and Ignore**: Re-emit untouched during formatting and migrations. |
| **Deprecated Key** (`ods.relations`, `ods.memory:`, flat memory keys, nested `ods.toml` tables) | **Accept and Warn**: Parse with full semantics, emit the matching `DEPR-*` warning, and preserve on write unless the author explicitly runs a migration. |
| **Unknown Relation Predicate** | **Fatal Error** (`ENUM-006`): the predicate vocabulary is closed. Use `predicate: custom` with `custom_predicate`. |
| **Top-Level Key Listed by `required_keys`** | **Profile-Scoped Requirement**: Validate presence for documents using the declaring custom profile; preserve the key and its value. |
| **Unknown Nested Key under `ods:`** | **Report Warning**: Warn author of unknown engine key; preserve during formatting. |
| **Unrecognized `ods.profile`** | **Fatal Profile Error**: Report `PROF-001`; do not fall back to `note` or another profile. |
| **Unknown `code` role** | **Fatal Error** (`ENUM-003`): Reject immediately; projects MUST NOT invent custom code roles. |
| **Invalid `ods.share` value** | **Fatal Error**: Reject immediately to prevent unintended privacy leaks. |
| **Legacy Flat Engine Keys** (without nested `ods:`) | **Migration Mode**: Accept during read; format tooling (`ods fmt --migrate`) MUST nest under `ods:`. |

---

## 7. Diagnostic Message Presentation

Conformant ODS tools SHOULD present diagnostic output in a short, directive format featuring:
1. File location (`path:line:col`)
2. Clear error description with Rule Identifier
3. Actionable remediation (`Next: ...`)

```text
error[ASSET-003]: line numbers are prohibited in code paths
  --> docs/guides/checkout.md:14:11
   |
14 |     - path: src/checkout.ts:L45-L60
   |             ^^^^^^^^^^^^^^^^^^^^^^^
   = help: line numbers drift across commits. Use 'symbol: processCheckout' instead.
   = next: remove ':L45-L60' from the path and add 'symbol: <name>'
```

---

## 8. Implementer Conformance Checklist

*This checklist provides an actionable summary for developers building ODS parsers, linters, and runtime engines.*

### Frontmatter & Parser
- [ ] Parse frontmatter delimited by `---` as YAML.
- [ ] Preserve all unknown top-level frontmatter keys during read/write cycles.
- [ ] Accept `title:` in frontmatter without error; warn (`SYNTAX-002`) only when no OKF signal is present.
- [ ] Enforce top-level placement for `description`, `tags`, `owner`, `author`, `created`, `created_at`, `updated`, `updated_at`.
- [ ] Enforce nested `ods:` placement for all engine keys.

### Graph & DAG Engine
- [ ] Derive document ID from workspace-relative path without `.md`.
- [ ] Enforce unique document IDs across workspace.
- [ ] Validate that `ods.depends` targets and document-shaped `ods.related` targets resolve to real `.md` files; resolve entity handles against declared `ods.entity` values instead.
- [ ] Perform cycle detection on `ods.depends` edges (reject cyclic graphs).
- [ ] Compute backlinks dynamically on demand (never hand-written).

### Assets & Code Engine
- [ ] Validate that local `ods.resources` entries resolve on disk; syntax-check URL entries without fetching them.
- [ ] Validate that all `ods.code` paths resolve on disk.
- [ ] Reject any `ods.code[].path` containing a line number suffix (`:L...`).
- [ ] Validate that `ods.code[].role` belongs to the 10 standard roles; default a bare-string entry to `implementation`.

### Profile & Discovery Engine
- [ ] Validate expected H2 or H3 headings (`##` or `###`) for standard profiles using alias matching; do not count H1 or H4+ headings.
- [ ] Parse `ods.custom_profile.name`, `required_keys`, `optional_keys`, and `forbidden_keys` from registered custom profile definitions.
- [ ] Fail when any `custom_profiles` path in `ods.toml` is missing, not a Markdown file, or otherwise cannot be loaded.
- [ ] Fail when `ods.custom_profile` appears outside a file selected by `custom_profiles` or a registered pack.
- [ ] Fail when `ods.profile` does not resolve to a standard profile or a loaded custom profile; include the configured profile paths in the diagnostic.
- [ ] Validate each selected custom profile's `required_keys` against top-level document frontmatter and emit `PROF-003` warnings for missing keys.
- [ ] Emit `PROF-004` warnings when selected profile `forbidden_keys` are present.
- [ ] Resolve custom profiles registered in `ods.toml`.
- [ ] Support progressive CLI discovery without generating committed folder indexes.

---

## Navigation & Reading Order

| [← Previous Chapter](indexes.md) | [📑 Specification Index](README.md) | [Next Chapter →](scope.md) |
| :--- | :---: | ---: |
| **08. Workspace Config & Progressive Discovery** | **Open Document Spec (ODS)** | **10. Scope & Architectural Non-Goals** |
