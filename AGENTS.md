---
description: Operational heuristics, golden rules, and bounded context algorithms for AI agents authoring and navigating ODS workspaces.
tags:
  - agent
  - ods
  - guidelines
  - ai
owner: team:ods
created: 2026-08-14
updated: 2026-08-14
ods:
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

1. **Never Invent `title:` in Frontmatter**:
   - In pure ODS authoring, the document title exists **only** as the first `# H1` line in the Markdown body prose.
   - For Google OKF v0.2 compatibility, top-level `title:` and `type:` are preserved and accepted. In a document that carries no OKF signal (`type`, `okf_version`, `sources`), a `title:` key is reported as a `SYNTAX-002` **warning**, not an error.

2. **Strict 3-Layer Key Placement & Universal Metadata**:
   - **Layer 1: Universal & OKF native keys** MUST be placed at the **top level** of frontmatter.
   - **Layer 2: Scoped ODS engine keys** MUST be placed directly under the **`ods:`** block.
   - **Layer 3: Workspace boundary keys** belong ONLY in root `ods.toml`.
   - Never nest universal keys (`tags`, `description`, `owner`, `author`, `created`, `created_at`, `updated`, `updated_at`) under `ods:` — the document schema rejects them there outright (`PLACE-001`).
   - Canonical, normative membership of each layer: [`specs/keys.md` §3](./specs/keys.md#3-the-3-layer-key-placement-architecture) and, for Layer 3, [`specs/indexes.md` §3](./specs/indexes.md#3-workspace-configuration-key-reference). Do not maintain a second copy here.

3. **Maintain Knowledge Graph Purity**:
   - `ods.depends` is strictly for conceptual dependencies to other **Markdown documents**.
   - Do NOT place non-document fixtures (JSON schemas, sample CSVs, mock payloads) in `depends`. Put auxiliary prompt files in **`ods.context.load`**.

4. **Refactor-Resilient Code Bindings & Resources**:
   - In `ods.code`, file paths MUST NOT include line numbers (e.g. `:L45` is forbidden).
   - Support simple string shorthand (`code: ["src/main.rs"]`), targeted symbol strings or lists (`symbol: [TestA, TestB]`), and optional descriptions.
   - In `ods.resources`, use string paths, external URLs (`https://figma.com/...`), or detailed maps with `title` and `description`.

5. **Unified Related & Pareto Directed Relations**:
   - Use `ods.related` for both simple lateral reading links (`- @faq.md`) and Pareto directed semantic relation edges (`- is_a: Account`, `- owns: [@Subscription, @Invoice]`, `- governed_by: @RefundPolicy`).
   - Use Universal `@` handles (`@Subscription`, `@tokens.md`) instead of brittle relative file paths (`../../path/to/file.md`). The attributed `{ predicate: owns, target: ... }` object form is also valid and is required when an edge needs `role`, `confidence`, `since`/`until`, or `cardinality`.
   - Predicates are a **closed set**. Do not invent bare `snake_case` verbs — an unrecognized key is rejected by the schema. For a domain-specific verb use `{ predicate: custom, custom_predicate: <verb>, target: ... }`. See [`specs/graph.md` §4.1](./specs/graph.md#41-the-complete-predicate-vocabulary).

6. **Preserve Third-Party and Unknown Frontmatter**:
   - If a document contains metadata for SSGs (e.g. Hugo `layout`, Astro `hero_image`, Jekyll `permalink`), agents MUST preserve those keys verbatim when editing the file.

7. **Path-Derived Document IDs**:
   - By default, a document's ID is its workspace-relative path without the `.md` extension (e.g., `guides/checkout.md` → `guides/checkout`).
   - Only specify an explicit `ods.id` when renaming a file where you need to preserve existing inbound links without cascading rewrites.

8. **Graph Integrity & Acyclicity**:
   - Hard prerequisites belong in `ods.depends`. Soft references belong in `ods.related`.
   - The `depends` graph MUST NOT contain cyclic loops.

9. **Leverage JSON Schema 1.1 for Syntactic Validation**:
   - When generating or updating frontmatter, validate structure against [`schemas/1.1.0/document.schema.json`](./schemas/1.1.0/document.schema.json).
   - Recognize that `$schema` in frontmatter is optional; never reject or alter valid documents that omit `$schema`.

---

## 2. Bounded Context Loading Algorithm for Agents

When answering questions, planning code modifications, or debugging issues, agents SHOULD follow this bounded context expansion routine instead of scanning the entire workspace:

1. **Identify Entrypoint Document**: Identify the primary ODS document relevant to the user request (e.g. via `ods find` or `ods overview`).
2. **Auto-Expand Hard Dependencies**: Read the documents listed under `ods.depends` recursively up to `ods.context.max-depth` (default: 2 hops; permitted range 0–10).
3. **Evaluate Trust & Staleness**: Check `verified` (infer trust tier) and skip documents where `now >= stale_after` or `now >= valid_to`.
4. **Load Auxiliary Resources**: Read any files listed under `ods.context.load` and inspect schema shapes in `ods.schema` and `ods.resources`.
5. **Inspect Code Bindings**: Use `ods.code` to jump directly to declared entrypoints, logic implementations, and test fixtures using AST symbol extraction.
6. **Respect Visibility**: If assembling public-facing exports or unprivileged summaries, skip any document or target marked `ods.share: private`.

---

## 3. Standard Document Profile Shapes

When authoring new documents, pick the profile matching the document's intent and scaffold the expected H2 or H3 sections (`##` or `###`).

ODS ships 13 standard profiles: `note` (default), `guide`, `feature`, `decision`, `sop`, `api`, `architecture`, `policy`, `meeting`, `faq`, `checklist`, `agent`, and `skill`.

Canonical catalog with the exact expected sections for each profile: [`specs/profiles.md` §3](./specs/profiles.md#3-standard-profiles-catalog). Recognized heading synonyms: [`specs/profiles.md` §6](./specs/profiles.md#6-section-heading-alias-matching). Do not maintain a second copy here.

---

## 4. Comprehensive Document Template

```markdown
---
# 1. Universal Top-Level Metadata
description: Comprehensive guide for setting up and managing user authentication sessions.
tags:
  - auth
  - security
owner: team:security

# 2. ODS Engine Subsystems
ods:
  profile: guide
  status: stable
  share: public

  # Subsystem 1: Knowledge Graph (Auto-traversed in context resolution)
  depends:
    - ../crypto/jwt-spec.md

  # Subsystem 2: Discovery Graph (Skipped by default in context)
  related:
    - ../policy/data-retention.md

  # Subsystem 3: Asset Catalog (Disk-level files verified by lint)
  resources:
    - path: ../diagrams/session-flow.png

  # Subsystem 4: Code Bindings (Implementation & Tests)
  code:
    - path: src/auth/server.ts
      role: entrypoint
      symbol: startAuthServer
    - path: src/auth/tokens.ts
      role: implementation
      symbol: generateToken
    - path: tests/tokens.test.ts
      role: test

  # Subsystem 5: AI Prompt Bounds & Inclusions
  context:
    max-depth: 2
    load:
      - ../schemas/auth-contract.json
    ignore:
      - legacy/
---

# User Session Management

## Overview
This document explains how session tokens are generated, validated, and revoked.

## Prerequisites
- Node.js 20+ runtime.
- Redis server active on port 6379.

## Steps
1. Initialize the session middleware.
2. Sign JWT payloads with the private key.
3. Validate session tokens on incoming requests.

## Troubleshooting
- **Token Expired**: Verify client system clock synchronization with NTP.
```

---

## 5. Agent & Skill Templates

### 5.1 Agent Instruction Template (`agent.md`)

```markdown
---
description: Autonomous task execution instructions for database migrations.
tags: [agent, db-migration]
owner: team:data-platform
ods:
  profile: agent
  status: stable
  depends:
    - ../specs/migration-plan.md
  code:
    - path: src/db/migrator.ts
      role: entrypoint
      symbol: runMigrations
---

# Database Migration Agent

## Goal
Safely execute schema migrations on staging and verify data integrity.

## Task
Inspect pending migrations, run migration scripts sequentially, and run validation smoke tests.

## Scope
- In Scope: Schema modifications to PostgreSQL tables under `migrations/`.

## Non-Scope
- Direct data deletion, dropping tables, or altering production connection strings.

## Context
TypeScript migration runner wrapping Knex.js.

## Inputs
- Migration script paths in `migrations/*.ts`.

## Constraints
- Must acquire lock before migrating.
- Must abort transaction on first error.

## Priority
1. Data safety and transaction isolation.
2. Speed of execution.

## Steps
1. Verify database connectivity.
2. Run pending migrations in a transaction block.
3. Run verification queries.

## Output
- Migration execution log and status report.

## Success Criteria
- All migrations apply cleanly with exit code 0.

## Failure Modes
- Lock timeout: Rollback and notify on-call channel.

## Dependencies
- `specs/migration-plan.md`

## Assumptions
- Migration target is a PostgreSQL 16 instance.

## Examples
```bash
pnpm run migrate:staging
```
```

### 5.2 Skill Package Template (`SKILL.md`)

```markdown
---
description: Reusable agent capability for linting and formatting ODS documents.
tags: [skill, ods, linter]
owner: team:docs
ods:
  profile: skill
  status: stable
  code:
    - path: src/linter.ts
      role: implementation
      symbol: lintWorkspace
---

# ODS Document Linter Skill

## Purpose
Enables autonomous agents to inspect Markdown workspaces for ODS compliance.

## Capability
- YAML frontmatter validation and 3-tier key placement verification.
- Document graph DAG acyclicity validation.
- Missing section heading detection against profile contracts.

## Activation
- Trigger when user requests documentation verification or repository health checks.

## Scope
- In Scope: `.md` files within the workspace root declared in `ods.toml`.

## Non-Scope
- Source code AST verification or external URL liveness checks.

## Inputs
- Workspace root path.

## Outputs
- Diagnostic error and warning report.

## Workflow
1. Locate `ods.toml` in workspace root.
2. Parse frontmatter across all `.md` files.
3. Validate profile section headings against standard or custom profile contracts.
4. Report binary compliance status (exit 0 or 1).

## Rules
- Strictly preserve unknown third-party frontmatter.
- Report missing profile sections as warnings (`PROF-002`).

## Priority
1. Syntax and graph integrity errors over stylistic warnings.

## Validation
- Confirm zero false-positive diagnostics against standard fixtures.

## Eval
- Run validation against test suites in `tests/fixtures/`.

## Resources
- ODS Specification: `specs/validation.md`.

## Tools
- ODS CLI (`ods lint`).

## Lifecycle
- Initialized on workspace discovery; cache cleaned up on exit.

## Traceability
- Diagnostic logs emitted with RFC 3339 timestamps.
```
