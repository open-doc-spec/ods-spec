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
- Warnings (such as missing optional profile sections or unrecognized profile names) SHOULD be reported to authors but MUST NOT cause a non-zero exit code.

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
| **Phase 1: Authoring & Verification** | `ods lint` | • YAML syntax<br>• 3-tier key placement<br>• DAG acyclicity (no cycles in `depends`)<br>• File existence on disk for `resources`, `code`, and `load`<br>• Absence of line numbers in code paths | Enforces repository health, consistency, and zero dead links in CI. |
| **Phase 2: AI Context Resolution** | `ods context <id>` | • Traverses `depends` up to `max-depth` (default: 2)<br>• Ingests `context.load` text files<br>• Prunes `context.ignore` and `share: private`<br>• Emits unified bounded prompt payload | Assembles deterministic prompt context within token budget. |

---

## 4. Normative Lint Rules Matrix

All conformant ODS linters MUST enforce the following validation rules:

| Category | Rule Identifier | Rule Condition | Severity | Remediation Action |
| :--- | :--- | :--- | :---: | :--- |
| **Syntax** | `SYNTAX-001` | Frontmatter MUST parse as valid YAML delimited by `---`. | **Error** | Fix YAML syntax error. |
| | `SYNTAX-002` | Frontmatter MUST NOT contain a `title:` key. | **Error** | Remove `title:` from frontmatter; declare title in first `# H1` body heading. |
| **Placement** | `PLACE-001` | `tags` MUST appear at the top level; MUST NOT be nested under `ods:`. | **Warning** | Hoist `tags` to top-level frontmatter. |
| | `PLACE-002` | Engine keys (`profile`, `status`, `depends`, etc.) MUST be nested under `ods:`. | **Error** | Nest engine keys under `ods:` mapping. |
| **Enums** | `ENUM-001` | `ods.status` MUST be one of `draft`, `stable`, `deprecated`, `archived`. | **Error** | Change status to a recognized lifecycle state. |
| | `ENUM-002` | `ods.share` (when present) MUST be one of `public`, `org`, `private`. | **Error** | Set share to `public`, `org`, or `private`. |
| | `ENUM-003` | `ods.code[].role` MUST be one of the 8 standard roles. | **Error** | Change role to a valid standard role (e.g. `entrypoint`, `implementation`). |
| **Graph** | `GRAPH-001` | Document IDs MUST be unique across the workspace. | **Error** | Rename duplicate file or override via `ods.id`. |
| | `GRAPH-002` | `ods.depends` targets MUST resolve to existing documents. | **Error** | Fix or remove dangling dependency path. |
| | `GRAPH-003` | `ods.related` targets MUST resolve to existing documents. | **Error** | Fix or remove dangling related path. |
| | `GRAPH-004` | `ods.depends` graph MUST NOT contain cyclic dependency loops. | **Error** | Break circular dependency loop using `ods.related`. |
| **Assets** | `ASSET-001` | `ods.resources[].path` MUST resolve to an existing file. | **Error** | Fix path or verify file existence on disk. |
| | `ASSET-002` | `ods.code[].path` MUST resolve to an existing file. | **Error** | Fix path or verify source code file on disk. |
| | `ASSET-003` | `ods.code[].path` MUST NOT contain line number suffixes (e.g. `:L45`). | **Error** | Remove `:L45`; use `symbol` field instead. |
| | `ASSET-004` | `ods.context.load` paths MUST resolve to existing files. | **Error** | Fix or remove dangling context load path. |
| **Profiles** | `PROF-001` | `ods.profile` SHOULD resolve to a known standard or custom profile. | **Warning** | Define custom profile in `ods.toml` or fix typo. |
| | `PROF-002` | Document SHOULD contain expected `##` H2 sections for its declared profile. | **Warning** | Add missing section heading or registered alias. |

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
| **Unknown Nested Key under `ods:`** | **Report Warning**: Warn author of unknown engine key; preserve during formatting. |
| **Unrecognized `ods.profile`** | **Warning & Fallback**: Warn author; treat as `note` (no required sections) during lint. |
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
- [ ] Validate expected `##` H2 headings for standard profiles using alias matching.
- [ ] Resolve custom profiles registered in `ods.toml`.
- [ ] Support progressive CLI discovery without generating committed folder indexes.

---

## Navigation & Reading Order

| [← Previous Chapter](indexes.md) | [📑 Specification Index](README.md) | [Next Chapter →](scope.md) |
| :--- | :---: | ---: |
| **08. Workspace Config & Progressive Discovery** | **Open Document Spec (ODS)** | **10. Scope & Architectural Non-Goals** |
