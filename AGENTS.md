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
   - The document title exists **only** as the first `# H1` line in the Markdown body prose.
   - Frontmatter MUST NOT contain a `title:` key.

2. **Strict 3-Tier Key Placement**:
   - **Universal keys** (`description`, `tags`, `owner`, `created`, `updated`) MUST be placed at the **top level** of frontmatter.
   - **ODS engine keys** (`profile`, `status`, `id`, `share`, `depends`, `related`, `resources`, `code`, `context`) MUST be nested under the **`ods:`** block.
   - Never nest `tags` or `description` under `ods:`.
   - Never put operational execution keys (`role`, `help`, `qualification_gate`, `context_resolution_priority`, `refusal_guardrails`, `workflow`, `tools`, `eval`, `decision`, `branch`) in frontmatter. Operational contracts belong in standard `##` H2 body headings.
   - Workspace boundary keys (`spec`, `ignore`, `custom_profiles`, `packs`, `aliases`) belong ONLY in root `ods.toml`.

3. **Maintain Knowledge Graph Purity**:
   - `ods.depends` is strictly for conceptual dependencies to other **Markdown documents**.
   - Do NOT place non-document fixtures (JSON schemas, sample CSVs, mock payloads) in `depends`. Put auxiliary prompt files in **`ods.context.load`**.

4. **No Line Numbers in Code Bindings**:
   - In `ods.code`, file paths MUST NOT include line numbers (e.g. `:L45` is forbidden).
   - Use the `symbol` field (function, struct, class, or constant name) for precise symbol-level linking.

5. **Preserve Third-Party and Unknown Frontmatter**:
   - If a document contains metadata for SSGs (e.g. Hugo `layout`, Astro `hero_image`, Jekyll `permalink`), agents MUST preserve those keys verbatim when editing the file.

6. **Path-Derived Document IDs**:
   - By default, a document's ID is its workspace-relative path without the `.md` extension (e.g., `guides/checkout.md` → `guides/checkout`).
   - Only specify an explicit `ods.id` when renaming a file where you need to preserve existing inbound links without cascading rewrites.

7. **Graph Integrity & Acyclicity**:
   - Hard prerequisites belong in `ods.depends`. Soft references belong in `ods.related`.
   - The `depends` graph MUST NOT contain cyclic loops.

---

## 2. Bounded Context Loading Algorithm for Agents

When answering questions, planning code modifications, or debugging issues, agents SHOULD follow this bounded context expansion routine instead of scanning the entire workspace:

1. **Identify Entrypoint Document**: Identify the primary ODS document relevant to the user request (e.g. via `ods find` or `ods overview`).
2. **Auto-Expand Hard Dependencies**: Read the documents listed under `ods.depends` recursively up to `ods.context.max-depth` (default: 2 hops). Note: targets in `depends` do not need re-listing in `context.load`.
3. **Load Auxiliary Resources**: Read any files listed under `ods.context.load` (including non-Markdown JSON schemas, sample CSVs, and fixtures).
4. **Inspect Code Bindings**: Use `ods.code` to jump directly to the declared entrypoints (`role: entrypoint`), logic implementations (`role: implementation`), and test fixtures (`role: test`).
5. **Respect Visibility**: If assembling public-facing exports or unprivileged summaries, skip any document or target marked `ods.share: private`.

---

## 3. Standard Document Profile Shapes

When authoring new documents, pick the profile matching the document's intent and scaffold the expected H2 or H3 sections (`##` or `###`):

| Profile | Primary Intent | Expected Sections |
| :--- | :--- | :--- |
| `note` | Free-form knowledge / quick note (default) | *(none required)* |
| `guide` | Step-by-step how-to tutorial | Overview, Prerequisites, Steps, Troubleshooting |
| `feature` | Product capability / PRD specification | Goal, Scope, Requirements, Acceptance Criteria, Risks |
| `decision` | Architecture Decision Record (ADR) | Context, Decision, Alternatives, Consequences |
| `sop` | Standard operating procedure / runbook | Purpose, Prerequisites, Steps, Validation, Rollback |
| `api` | Endpoint / RPC contract | Overview, Request, Response, Errors, Examples |
| `architecture`| System design and data flow | Overview, Components, Data Flow, Trade-offs |
| `policy` | Governance / team rules | Purpose, Scope, Rules, Exceptions |
| `meeting` | Meeting minutes and team sync notes | Attendees, Agenda, Decisions, Action Items |
| `faq` | Frequently Asked Questions | *(Question/Answer pairs; no fixed H2 list)* |
| `checklist` | Verifiable deployment or release gates | Overview, Items, Verification, Notes |
| `agent` | Autonomous agent instructions / prompt execution contracts (`agent.md`) | Goal, Task, Scope, Non-Scope, Context, Inputs, Constraints, Priority, Steps, Output, Success Criteria, Failure Modes, Dependencies, Assumptions, Examples |
| `skill` | Reusable skill packages and tool contracts (`SKILL.md`) | Purpose, Capability, Activation, Scope, Non-Scope, Inputs, Outputs, Workflow, Rules, Priority, Validation, Eval, Resources, Tools, Lifecycle, Traceability |

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
