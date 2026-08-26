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

- **What this chapter defines:** Binary compliance, the lint rule matrix, unknown-key behavior, and the implementer checklist.
- **Why it exists:** Conformance is what `ods lint` can prove, not what an author intended.
- **When you need it:** You are wiring CI, reading an error ID, or implementing a linter.
- **When you can skip it:** You are still writing your first documents — [common mistakes](../guides/mistakes.md) is enough.
- **Learn this first:** [Run the workspace](../guides/06-run-the-workspace.md) · [Common mistakes](../guides/mistakes.md)
- **Prerequisite chapters:** [core.md](core.md), [keys.md](keys.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt), [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt)) when, and only when, they appear in all capitals.

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
| | `SYNTAX-002` | **Tier 1 (Schema)** | Frontmatter MUST NOT contain a `title:` key. | **Error** | Remove `title:` from frontmatter; declare title in first `# H1` body heading. |
| **Placement** | `PLACE-001` | **Tier 1 (Schema)** | `tags` MUST appear at the top level; MUST NOT be nested under `ods:`. | **Warning** | Hoist `tags` to top-level frontmatter. |
| | `PLACE-002` | **Tier 1 (Schema)** | Engine keys (`profile`, `status`, `depends`, etc.) MUST be nested under `ods:`. | **Error** | Nest engine keys under `ods:` mapping. |
| **Enums** | `ENUM-001` | **Tier 1 (Schema)** | `ods.status` MUST be one of `draft`, `stable`, `deprecated`, `archived`. | **Error** | Change status to a recognized lifecycle state. |
| | `ENUM-002` | **Tier 1 (Schema)** | `ods.share` (when present) MUST be one of `public`, `org`, `private`. | **Error** | Set share to `public`, `org`, or `private`. |
| | `ENUM-003` | **Tier 1 (Schema)** | `ods.code[].role` MUST be one of the 8 standard roles. | **Error** | Change role to a valid standard role (e.g. `entrypoint`, `implementation`). |
| **Graph** | `GRAPH-001` | **Tier 2 (Semantic)** | Document IDs MUST be unique across the workspace. | **Error** | Rename duplicate file or override via `ods.id`. |
| | `GRAPH-002` | **Tier 2 (Semantic)** | `ods.depends` targets MUST resolve to existing documents. | **Error** | Fix or remove dangling dependency path. |
| | `GRAPH-003` | **Tier 2 (Semantic)** | `ods.related` targets MUST resolve to existing documents. | **Error** | Fix or remove dangling related path. |
| | `GRAPH-004` | **Tier 2 (Semantic)** | `ods.depends` graph MUST NOT contain cyclic dependency loops. | **Error** | Break circular dependency loop using `ods.related`. |
| **Assets** | `ASSET-001` | **Tier 2 (Semantic)** | `ods.resources[].path` MUST resolve to an existing file. | **Error** | Fix path or verify file existence on disk. |
| | `ASSET-002` | **Tier 2 (Semantic)** | `ods.code[].path` MUST resolve to an existing file. | **Error** | Fix path or verify source code file on disk. |
| | `ASSET-003` | **Tier 1 (Schema)** | `ods.code[].path` MUST NOT contain line number suffixes (e.g. `:L45`). | **Error** | Remove `:L45`; use `symbol` field instead. |
| | `ASSET-004` | **Tier 2 (Semantic)** | `ods.context.load` paths MUST resolve to existing files. | **Error** | Fix or remove dangling context load path. |
| **Profiles** | `PROF-001` | **Tier 1 (Schema)** | `ods.profile` MUST resolve to a known standard or registered custom profile. | **Error** | Fix the profile name or define and register the profile at the path declared in `ods.toml`. |
| | `PROF-002` | **Tier 2 (Semantic)** | Document SHOULD contain expected H2 or H3 sections (`##` or `###`) for its declared profile. | **Warning** | Add missing section heading or registered alias. |
| | `PROF-003` | **Tier 1 / 2** | A document SHOULD contain each non-null top-level key listed by its selected custom profile's `required_keys`. | **Warning** | Add the missing key to top-level frontmatter; do not nest it under `ods:`. |
| | `PROF-004` | **Tier 1 / 2** | A document SHOULD NOT contain a top-level key listed by its selected custom profile's `forbidden_keys`. | **Warning** | Remove the forbidden key or choose a profile that permits it. |
| | `PROF-005` | **Tier 2 (Semantic)** | Every `custom_profiles` path in `ods.toml` MUST resolve to an existing Markdown file or profile directory. | **Error** | Create the profile definition at the exact configured path or update the `custom_profiles` entry. |
| | `PROF-006` | **Tier 1 / 2** | `ods.custom_profile` MUST appear only in a profile-definition file selected by `custom_profiles` (or a registered pack). | **Error** | Move the definition to its registered path and use `ods.profile` in ordinary documents. |
| **Ontology** | `ONT-001` | **Tier 2 (Semantic)** | `ods.schema` MUST resolve to an existing schema file on disk. | **Error** | Fix path or verify schema existence on disk. |
| | `ONT-002` | **Tier 1 (Schema)** | `ods.invariants` expressions MUST parse as valid boolean expressions. | **Error** | Correct invariant expression syntax. |
| | `ONT-003` | **Tier 2 (Semantic)** | `ods.relations[].target` MUST resolve to an existing document. | **Error** | Fix broken semantic relation target. |
| | `ONT-004` | **Tier 2 (Semantic)** | `is_a` inheritance relations MUST NOT contain cyclic loops. | **Error** | Break circular entity inheritance loop. |
| **Memory** | `MEM-001` | **Tier 1 (Schema)** | `ods.valid_to` MUST NOT be chronologically earlier than `ods.valid_from`. | **Error** | Ensure valid_to $\ge$ valid_from. |
| | `MEM-002` | **Tier 2 (Semantic)** | `ods.mutations[].entity` SHOULD resolve to a declared entity class in the workspace. | **Warning** | Declare entity in workspace or check spelling. |
| | `MEM-003` | **Tier 2 (Semantic)** | Stale memory node exceeds retention window without `ods.pin: true`. | **Warning** | Pin memory node or allow pruning during dreaming. |
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
title: "Checkout Guide"               # ERROR [SYNTAX-002]: title in frontmatter
profile: guide                        # ERROR [PLACE-002]: engine key at top level
ods:
  tags: [billing]                     # WARNING [PLACE-001]: tags under ods:
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
| **Top-Level Key Listed by `required_keys`** | **Profile-Scoped Requirement**: Validate presence for documents using the declaring custom profile; preserve the key and its value. |
| **Unknown Nested Key under `ods:`** | **Report Warning**: Warn author of unknown engine key; preserve during formatting. |
| **Unrecognized `ods.profile`** | **Fatal Profile Error**: Report `PROF-001`; do not fall back to `note` or another profile. |
| **Unknown `code` role** | **Fatal Error**: Reject immediately; projects MUST NOT invent custom code roles. |
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
- [ ] Enforce prohibition of `title:` in frontmatter.
- [ ] Enforce top-level placement for `description`, `tags`, `owner`, `created`, `updated`.
- [ ] Enforce nested `ods:` placement for all engine keys.

### Graph & DAG Engine
- [ ] Derive document ID from workspace-relative path without `.md`.
- [ ] Enforce unique document IDs across workspace.
- [ ] Validate that all `ods.depends` and `ods.related` paths resolve to real `.md` files.
- [ ] Perform cycle detection on `ods.depends` edges (reject cyclic graphs).
- [ ] Compute backlinks dynamically on demand (never hand-written).

### Assets & Code Engine
- [ ] Validate that all `ods.resources` paths resolve on disk.
- [ ] Validate that all `ods.code` paths resolve on disk.
- [ ] Reject any `ods.code[].path` containing a line number suffix (`:L...`).
- [ ] Validate that `ods.code[].role` belongs to the 8 standard roles.

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
