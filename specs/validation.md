---
description: ODS 2.0 lint rules, binary compliance contract, Tier 1/Tier 2 rule matrix,
  diagnostic format, and implementer validation requirements.
profile: note
status: stable
depends:
- README.md
- core.md
- keys.md
related:
- graph.md
- indexes.md
- assets.md
- profiles.md
- context.md
- ../guides/06-run-the-workspace.md
- ../guides/mistakes.md
---

# ODS · Validation & Tooling Contract

This document specifies the **Normative Validation Contract** for Open Document Spec (ODS) **v2.0** and the optional **v2.1** Pareto ontology extension: lint rules, rule severities, unknown-content behavior, diagnostic message format, and implementer checklists.

## At a glance

- **What this chapter defines:** Binary compliance, conformance profiles, the ODS 2.0 lint rule matrix, unknown-key behavior, and the implementer checklist.
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
- Warnings (such as `SYNTAX-002` for a redundant `title:` key) SHOULD be reported to authors but MUST NOT cause a non-zero exit code. Unrecognized profile names are `PROF-001` errors.

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
| **Core** | Tier 1 only: YAML parsing, flat frontmatter schema validation, all `SYNTAX-*`, `ENUM-*`, `CODE-*`, `TITLE-*`, and `ASSET-005` rules, plus unknown-key preservation. | Editor extension, pre-commit hook, JSON Schema validator. |
| **Graph** | Core, plus workspace discovery and every Tier 2 rule over documents: `GRAPH-001..004`, `ASSET-001..002`, `ASSET-004`, `PROF-006`, and `TITLE-001`. | CI linter. |
| **Context** | Graph, plus the bounded context resolution algorithm in [context.md §7](context.md#7-the-context-resolution-algorithm-normative), including `share`, workspace `context.ignore`, staleness filtering, and `load` injection. | Agent runtime, prompt assembler. |

Each profile is a strict superset of the one above it. A tool MUST NOT claim a profile while silently skipping rules it contains; it MAY report an unimplemented rule as "not checked" rather than as a pass.

The **binary compliance contract in §2 is scoped to the declared profile**: `ods lint` at profile *Graph* exits `0` when no Graph-level error is found, and makes no claim about context payload assembly.

---

## 2.2 Required Capabilities (Not a Command Surface)

This specification describes **capabilities**, not a CLI. Every `ods <command>` line in these chapters is a **non-normative illustration** using the reference engine's spelling; a conformant tool may expose the same capability as a library call, a language server request, an editor action, or a differently named command.

A tool claiming a conformance profile MUST provide the capabilities marked for that profile:

| Capability | What it MUST do | Required at |
| :--- | :--- | :--- |
| **Validate** | Evaluate a workspace against the rule matrix for the declared profile and report a binary pass/fail plus per-rule diagnostics. | Core |
| **Adopt** | Ingest untyped Markdown without rewriting it; infer `profile` from headings per [core.md §6](core.md#6-smart-profile-inference-heuristics). | Graph |
| **Format** | Normalize flat frontmatter and emit ordering per [keys.md §5](keys.md#5-canonical-emit-ordering) while preserving every unknown key verbatim. | Graph |
| **Scaffold** | Create a document with valid flat frontmatter and the section placeholders of its profile. | Graph |
| **Relocate** | Move a document and rewrite every inbound reference (`depends`, `related`, `load`, inline links, relative resource paths). | Graph |
| **Archive / Delete** | Set `status: archived` preserving edges, or remove a document and scrub every inbound reference. | Graph |
| **Resolve context** | Execute the bounded context algorithm and emit a deterministic, ordered, token-bounded payload. | Context |

Beyond the exit-code contract in §2, this specification does **not** constrain flag names, output formats, or command names. Those belong to each implementation.

---

## 3. The Two Lifecycle Phases

ODS enforces clear separation between verification in CI and context resolution for AI agents:

| Phase | Command | What is Checked / Executed | Primary Purpose |
| :--- | :--- | :--- | :--- |
| **Phase 1: Authoring & Verification** | `ods lint` | • YAML syntax and flat schema constraints<br>• DAG acyclicity (no cycles in `depends`)<br>• File existence on disk for `resources`, `code`, and `load`<br>• Absence of line numbers in `code` paths<br>• Rejection of the legacy `ods:` wrapper | Enforces repository health, consistency, and zero dead links in CI. |
| **Phase 2: AI Context Resolution** | `ods context <id>` | • Traverses `depends` up to workspace `context.default_max_depth` (default: 2)<br>• Ingests top-level `load` text files<br>• Prunes workspace `context.ignore` and `share: private`<br>• Emits unified bounded prompt payload | Assembles deterministic prompt context within token budget. |

---

## 3.1 Two-Tier Validation Architecture

To ensure high performance and determinism across both offline linters and language servers, validation is split into two explicit tiers:

1. **Tier 1 (Schema Layer — Zero-IO / Fast AST)**:
   - Validates document YAML frontmatter against [`document.schema.json`](../schemas/2.0.0/document.schema.json) and `ods.toml` against [`config.schema.json`](../schemas/2.0.0/config.schema.json) using standard JSON Schema Draft 2020-12 validators.
   - Enforces flat top-level key placement (no `ods:` wrapper), enum validity (`status`, `share`), string-only `code` entries, and string regex patterns.
   - Executes in `<1ms` per document with zero filesystem I/O.
2. **Tier 2 (Semantic & Graph Layer — Deep Workspace Analysis)**:
   - Validates multi-document graph properties: DAG acyclicity (`GRAPH-004`), unique document IDs (`GRAPH-001`), disk file resolution for assets and source code (`ASSET-001`, `ASSET-002`, `ASSET-004`), title/H1 alignment (`TITLE-001`), custom profile registration (`PROF-006`), and — when `spec >= "2.1"` — ontology rules (`ONT-001`, `ENT-001`, `ENT-002`).

Profile section headings are **advisory only** in ODS 2.0. Tools MAY report missing sections as informational hints; there is no `PROF-002` rule.

---

## 4. Normative Lint Rules Matrix (ODS 2.0 / 2.1)

All conformant ODS linters MUST enforce the following validation rules:

| Category | Rule Identifier | Validation Tier | Rule Condition | Severity | Remediation Action |
| :--- | :--- | :---: | :--- | :---: | :--- |
| **Syntax** | `SYNTAX-001` | **Tier 1 (Schema)** | Frontmatter MUST parse as valid YAML delimited by `---`. | **Error** | Fix YAML syntax error. |
| | `SYNTAX-002` | **Tier 1 (Schema)** | Frontmatter carries a `title:` key while showing no OKF signal (`type`, `okf_version`, or `sources`). Suppressed entirely for OKF-flavoured documents. | **Warning** | Move the title to the first `# H1` body heading. Never an error: the OKF v0.2 superset requires `title:` to be accepted. See [core.md §3.1](core.md#31-frontmatter). |
| **Enums** | `ENUM-001` | **Tier 1 (Schema)** | `status` MUST be one of `draft`, `stable`, `deprecated`, `archived`. | **Error** | Change status to a recognized lifecycle state. |
| | `ENUM-002` | **Tier 1 (Schema)** | `share` (when present) MUST be one of `public`, `org`, `private`. | **Error** | Set share to `public`, `org`, or `private`. |
| | `ENUM-006` | **Tier 1 (Schema)** | In ODS 2.1, unknown predicate keys in `related` shorthand are rejected. Only `is_a`, `part_of`, `owns`, `governed_by`, `maps_to`, or `{ predicate: custom, verb, target }` are valid. | **Error** | Use a Pareto predicate or the `custom` verb escape hatch. |
| **Title** | `TITLE-001` | **Tier 2 (Semantic)** | When `title` or `name` is present and no OKF signal is present, the value MUST match the first `# H1` heading after case-insensitive, whitespace-normalized comparison. | **Error** | Align `title`/`name` with the H1, or remove the frontmatter key and rely on the H1 alone. |
| | `TITLE-002` | **Tier 1 (Schema)** | When `title` or `name` is present and the body has no `# H1` heading, report a structural error. OKF documents are exempt when `type:` or `okf_version:` is present. | **Error** | Add a `# H1` heading to the body or remove `title`/`name`. |
| **Graph** | `GRAPH-001` | **Tier 2 (Semantic)** | Document IDs MUST be unique across the workspace. | **Error** | Rename duplicate file or override via `id`. |
| | `GRAPH-002` | **Tier 2 (Semantic)** | `depends` targets MUST resolve to existing Markdown documents. | **Error** | Fix or remove dangling dependency path. |
| | `GRAPH-003` | **Tier 2 (Semantic)** | Every `related` target (plain paths and predicate shorthand targets) MUST resolve to an existing Markdown document. | **Error** | Fix or remove dangling related path. |
| | `GRAPH-004` | **Tier 2 (Semantic)** | `depends` graph MUST NOT contain cyclic dependency loops. | **Error** | Break circular dependency loop using `related`. |
| **Ontology** | `ONT-001` | **Tier 2 (Semantic)** | When `schema` is present (ODS 2.1), the path MUST resolve to an existing file on disk. | **Error** | Fix schema path or add the missing JSON Schema file. |
| | `ENT-001` | **Tier 2 (Semantic)** | When `entity` is present (ODS 2.1), the name MUST resolve to an entity definition document in the workspace. | **Error** | Create the definition doc or fix the entity name. |
| | `ENT-002` | **Tier 2 (Semantic)** | Entity names MUST be unique across the workspace (ODS 2.1). | **Error** | Rename or merge duplicate entity declarations. |
| **Assets & Code** | `ASSET-001` | **Tier 2 (Semantic)** | A local `resources` entry — a bare string without a URL scheme, or a mapping with `path` — MUST resolve to an existing file. URL entries (bare `https://…` strings, or mappings with `url`) are syntax-checked only and MUST NOT be network-fetched. | **Error** | Fix path or verify file existence on disk. |
| | `ASSET-005` | **Tier 1 (Schema)** | A `resources` mapping MUST carry exactly one of `path` or `url`. | **Error** | Remove the redundant key, or add the missing one. |
| | `ASSET-002` | **Tier 2 (Semantic)** | Every `code` path string MUST resolve to an existing file. | **Error** | Fix path or verify source code file on disk. |
| | `CODE-001` / `ASSET-003` | **Tier 1 (Schema)** | Every `code` entry MUST NOT contain a line number suffix (e.g. `:L45`). | **Error** | Remove the line-number suffix from the path string. |
| | `CODE-002` | **Tier 1 (Schema)** | Every `code` entry MUST be a plain string path. Mapping objects with `path`, `role`, or `symbol` are rejected. | **Error** | Rewrite as a string path: `code: ["src/main.ts"]`. |
| | `ASSET-004` | **Tier 2 (Semantic)** | Every `load` path MUST resolve to an existing file. | **Error** | Fix or remove dangling load path. |
| **Profiles** | `PROF-001` | **Tier 1 (Schema)** | `profile` MUST resolve to a known standard or registered custom profile. | **Error** | Fix the profile name or define and register the profile at the path declared in `ods.toml`. |
| | `PROF-006` | **Tier 2 (Semantic)** | Custom profile definition metadata MUST appear only in a profile-definition file selected by `custom_profiles` (or a registered pack). | **Error** | Move the definition to its registered path and use `profile` in ordinary documents. |

### Rules removed in ODS 2.0 (and not restored in 2.1)

The following rule families from ODS 1.1 are **not part of the ODS 2.0+ conformance contract**:

- **`PLACE-*`**: flat frontmatter replaces the `ods:` wrapper; the schema rejects `ods` outright.
- **`DEPR-*`**: deprecated shapes (`ods.relations`, `ods.memory:`, nested `ods.toml` tables) are removed, not warned.
- **`MEM-*`**, **`SYM-*`**: memory tiers and `@` symbolic resolution are out of scope.
- **`PROF-002`**: profile section headings are advisory; missing sections are not lint errors.
- **`ENUM-003`–`ENUM-005`**: code roles, frontmatter `max-depth`, and memory tiers are removed with their keys.

ODS **2.1** restores **`ONT-*`**, **`ENT-*`**, and **`ENUM-006`** for the optional Pareto ontology extension. Ontology lint activates when the workspace declares `spec >= "2.1"` or loads `@ods/pack-pareto-ontology`.

OKF and attestation keys (`type`, `sources`, `runtime`, `executor`, etc.) remain schema-valid for bundle interoperability but have no dedicated ODS lint rule family in 2.0.

---

## 5. Commented Rule Violation Examples

### 5.1 Legacy `ods:` Wrapper (Schema Rejection)

```yaml
# ERRONEOUS CODE (ODS 2.0):
---
description: Checkout guide
profile: guide
ods:                              # REJECTED: ods: wrapper forbidden in 2.0
  status: stable
---

# CORRECTED CODE:
---
description: Checkout guide
profile: guide
status: stable
depends:
  - ../auth/sessions.md
---

# Checkout Guide                      # Title lives in the H1 body
```

### 5.2 Line Numbers in Code Bindings (`CODE-001`)

```yaml
# ERRONEOUS CODE:
code:
  - src/checkout.ts:L45            # ERROR [CODE-001]: line numbers prohibited

# CORRECTED CODE:
code:
  - src/checkout.ts                # CORRECT: clean relative file path
```

### 5.3 Object-Form Code Bindings (`CODE-002`)

```yaml
# ERRONEOUS CODE:
code:
  - path: src/checkout.ts          # ERROR [CODE-002]: code must be plain strings
    role: implementation
    symbol: processCheckout

# CORRECTED CODE:
code:
  - src/checkout.ts
  - tests/checkout.test.ts
```

### 5.4 Cyclic Dependency Loops (`GRAPH-004`)

```yaml
# ERRONEOUS CODE (Doc A depends on Doc B, Doc B depends on Doc A):
# In auth.md:
depends:
  - session.md

# In session.md:
depends:
  - auth.md                         # ERROR [GRAPH-004]: cyclic dependency detected

# CORRECTED CODE:
# In auth.md:
depends:
  - session.md                      # Hard prerequisite

# In session.md:
related:
  - auth.md                         # CORRECT: soft related link (cycles permitted)
```

### 5.5 Title / H1 Mismatch (`TITLE-001`)

```yaml
# ERRONEOUS CODE:
---
title: Refund Processing
profile: note
status: draft
---

# Customer Refund Procedure           # ERROR [TITLE-001]: does not match title:

# CORRECTED CODE:
---
profile: note
status: draft
---

# Refund Processing                   # CORRECT: sole title in H1
```

---

## 6. Unknown-Content Behavior (Normative)

| Encountered Content | Tooling Behavior |
| :--- | :--- |
| **Unknown Top-Level Frontmatter Key** (e.g. `layout`, `hero_image`) | **Preserve and Ignore**: Re-emit untouched during formatting and migrations. |
| **Legacy `ods:` Wrapper** | **Fatal Error**: The document schema rejects `ods` via `propertyNames.not`. |
| **Typed `related` in ODS 2.0 workspaces** | **Fatal Error** (schema): `related` accepts string document paths only until `spec >= "2.1"`. |
| **Unknown predicate in ODS 2.1** | **Fatal Error** (`ENUM-006`): Use one of the five Pareto predicates or `{ predicate: custom, verb, target }`. |
| **Legacy `ods.context` Block** | **Fatal Error**: Use top-level `load` and workspace `[context]` in `ods.toml` instead. |
| **Top-Level Key Listed by Custom Profile `required_keys`** | **Profile-Scoped Requirement**: Validate presence for documents using the declaring custom profile; preserve the key and its value. |
| **Unrecognized `profile`** | **Fatal Profile Error**: Report `PROF-001`; do not fall back to `note` or another profile. |
| **Invalid `share` Value** | **Fatal Error** (`ENUM-002`): Reject immediately to prevent unintended privacy leaks. |

---

## 7. Diagnostic Message Presentation

Conformant ODS tools SHOULD present diagnostic output in a short, directive format featuring:
1. File location (`path:line:col`)
2. Clear error description with Rule Identifier
3. Actionable remediation (`Next: ...`)

```text
error[CODE-001]: line numbers are prohibited in code paths
  --> docs/guides/checkout.md:14:5
   |
14 |   - src/checkout.ts:L45-L60
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   = help: line numbers drift across commits. Bind the whole file instead.
   = next: remove ':L45-L60' from the path string
```

---

## 8. Implementer Conformance Checklist

*This checklist provides an actionable summary for developers building ODS 2.0 parsers, linters, and runtime engines.*

### Frontmatter & Parser
- [ ] Parse frontmatter delimited by `---` as YAML.
- [ ] Preserve all unknown top-level frontmatter keys during read/write cycles.
- [ ] Reject any document containing an `ods:` key.
- [ ] Accept `title:` in frontmatter without error; warn (`SYNTAX-002`) only when no OKF signal is present.
- [ ] Enforce flat top-level placement for all ODS engine keys (`profile`, `status`, `depends`, `related`, `resources`, `code`, `load`, `share`, `id`).

### Graph & DAG Engine
- [ ] Derive document ID from workspace-relative path without `.md`, unless `id` overrides it.
- [ ] Enforce unique document IDs across workspace.
- [ ] Validate that `depends` and `related` targets resolve to real `.md` files.
- [ ] Perform cycle detection on `depends` edges (reject cyclic graphs).
- [ ] Compute backlinks dynamically on demand (never hand-written).

### Assets & Code Engine
- [ ] Validate that local `resources` entries resolve on disk; syntax-check URL entries without fetching them.
- [ ] Validate that all `code` string paths resolve on disk.
- [ ] Reject any `code` entry containing a line number suffix (`:L...`).
- [ ] Reject mapping-object `code` entries (`CODE-002`).

### Profile & Discovery Engine
- [ ] Treat expected profile section headings as advisory hints only (no `PROF-002` error).
- [ ] Fail when any `custom_profiles` path in `ods.toml` is missing or cannot be loaded.
- [ ] Fail when custom profile definition metadata appears outside a registered profile-definition file (`PROF-006`).
- [ ] Fail when `profile` does not resolve to a standard profile or a loaded custom profile; include the configured profile paths in the diagnostic.

### Title Alignment
- [ ] Compare `title`/`name` against the first `# H1` when present (`TITLE-001`), unless OKF signal exempts the document.
- [ ] Error when `title`/`name` is present but the body has no H1 (`TITLE-002`), unless OKF signal exempts the document.

---

## Navigation & Reading Order

| [← Previous Chapter](indexes.md) | [📑 Specification Index](README.md) | [Next Chapter →](scope.md) |
| :--- | :---: | ---: |
| **08. Workspace Config & Progressive Discovery** | **Open Document Spec (ODS)** | **10. Scope & Architectural Non-Goals** |
