---
description: "Standard document profiles, expected section heading contracts, complete copy-paste templates, custom profile catalogs, and reusable packs."
ods:
  profile: "note"
  status: "stable"
  depends:
    - README.md
    - keys.md
  related:
    - core.md
    - validation.md
    - indexes.md
    - ../guides/02-pick-a-shape.md
    - ../guides/07-extend-ods.md
---

# ODS · Document Profiles & Shapes

This document specifies **Document Profiles** in Open Document Spec (ODS): their purpose, standard shapes, section heading validation, optional profile-required metadata keys, copy-paste templates, custom profile catalogs, and reusable packs.

## At a glance

- **What this chapter defines:** The 13 standard profiles, expected H2/H3 headings, aliases, profile-definition metadata, custom profiles, and packs.
- **Why it exists:** A `decision` should contain the same sections in every repo so humans and agents know where to look.
- **When you need it:** You are picking a shape, authoring a template, or validating headings.
- **When you can skip it:** You only write how-tos — `profile: guide` is enough ([Pick a shape](../guides/02-pick-a-shape.md)).
- **Learn this first:** [Pick a shape](../guides/02-pick-a-shape.md)
- **Prerequisite chapters:** [keys.md](keys.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt), [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt)) when, and only when, they appear in all capitals.

---

## 2. What is a Profile?

A **Profile** defines the *structural shape* and semantic nature of a document by establishing the list of expected H2 or H3 section headings (`##` or `###`). A custom profile MAY also declare profile-required document metadata keys.

Profile section matching is heading-level agnostic between H2 and H3: an expected section written as either `## Context` or `### Context` satisfies the same profile requirement. The document title remains the first `#` H1 heading, and H1, H4, or deeper headings MUST NOT satisfy an expected profile section.

- A profile is **not** a file extension or a layout template.
- A profile is a **structural validation contract** that ensures documents of a specific kind (e.g. PRDs, ADRs, SOPs, Guides, Agent Prompts, Skills) contain all required sections.
- Profile-required metadata is additive to the section contract. It does not create a new ODS engine key or a closed registry of third-party metadata.
- When an AI agent or human reads a document with `profile: decision`, they can rely on finding `## Context`, `## Decision`, `## Alternatives`, and `## Consequences`.

---

## 3. Standard Profiles Catalog

ODS provides 13 built-in standard profiles that cover common software engineering, autonomous agent execution, and organizational documentation:

| Profile | Intent & Usage | Expected H2/H3 Sections (`##` or `###`) |
| :--- | :--- | :--- |
| **`note`** | Free-form notes, scratchpads, and unstructured knowledge. (Default profile). | *(None required)* |
| **`guide`** | Step-by-step tutorials, setup instructions, and how-to procedures. | `Overview`, `Prerequisites`, `Steps`, `Troubleshooting` |
| **`feature`** | Product Requirement Documents (PRDs), feature specs, and user capabilities. | `Goal`, `Scope`, `Requirements`, `Acceptance Criteria`, `Risks` |
| **`decision`** | Architecture Decision Records (ADRs), RFC outcomes, and technical trade-offs. | `Context`, `Decision`, `Alternatives`, `Consequences` |
| **`sop`** | Standard Operating Procedures, incident runbooks, and disaster recovery plans. | `Purpose`, `Prerequisites`, `Steps`, `Validation`, `Rollback` |
| **`api`** | HTTP/gRPC interface definitions, webhook contracts, and RPC specifications. | `Overview`, `Request`, `Response`, `Errors`, `Examples` |
| **`architecture`** | System design documents, subsystem overviews, and data-flow specifications. | `Overview`, `Components`, `Data Flow`, `Trade-offs` |
| **`policy`** | Security policies, governance rules, and organizational compliance standards. | `Purpose`, `Scope`, `Rules`, `Exceptions` |
| **`meeting`** | Meeting minutes, team synchronizations, and retrospective notes. | `Attendees`, `Agenda`, `Decisions`, `Action Items` |
| **`faq`** | Frequently Asked Questions and troubleshooting indexes. | *(Question/Answer pairs; no fixed H2 list)* |
| **`checklist`** | Verifiable quality gates, deployment checklists, and release criteria. | `Overview`, `Items`, `Verification`, `Notes` |
| **`agent`** | Autonomous agent instructions, prompt execution contracts, and agent runbooks (`agent.md`). | `Goal`, `Task`, `Scope`, `Non-Scope`, `Context`, `Inputs`, `Constraints`, `Priority`, `Steps`, `Output`, `Success Criteria`, `Failure Modes`, `Dependencies`, `Assumptions`, `Examples` |
| **`skill`** | Reusable agent capability packages, tool integrations, and execution runbooks (`SKILL.md`). | `Purpose`, `Capability`, `Activation`, `Scope`, `Non-Scope`, `Inputs`, `Outputs`, `Workflow`, `Rules`, `Priority`, `Validation`, `Eval`, `Resources`, `Tools`, `Lifecycle`, `Traceability` |

---

## 4. Complete Profile Templates (Copy-Paste Ready)

### 4.1 `guide` Template (How-To Tutorial)
```markdown
---
description: Step-by-step tutorial for configuring user authentication sessions.
tags: [auth, setup]
owner: team:platform
ods:
  profile: guide
  status: stable
  depends:
    - ../prerequisites/cli-setup.md
  code:
    - path: src/auth.ts
      role: implementation
      symbol: initAuth
---

# User Session Setup Guide

## Overview
This guide walks through configuring session tokens for client web applications.

## Prerequisites
- Node.js 20+ and pnpm installed.
- Valid API credentials configured in `.env`.

## Steps
1. Initialize the session middleware in your application entrypoint.
2. Configure token expiration thresholds.
3. Validate session connectivity with the Redis cache.

## Troubleshooting
- **Error: Redis Connection Timeout**: Verify that your Redis instance is running and reachable on port 6379.
```

### 4.2 `feature` Template (PRD / Capability Spec)
```markdown
---
description: Product capability specification for automated customer refund processing.
tags: [billing, payments]
owner: team:billing
ods:
  profile: feature
  status: stable
  depends:
    - ../auth/sessions.md
  context:
    load:
      - ../schemas/refund-payload.json
---

# Customer Refund Processing

## Goal
Enable customer support agents to issue partial or full credit card refunds directly from the dashboard.

## Scope
- In Scope: Visa, Mastercard, and Stripe payment methods.
- Out of Scope: Direct wire transfer refunds and international currencies (v2).

## Requirements
1. Support agents can select one or more order line items to refund.
2. System must record the agent ID and reason code for auditing.
3. Total refund amount cannot exceed the original transaction value.

## Acceptance Criteria
- Refund requests are processed synchronously with the payment gateway.
- Successful refunds generate a confirmation email to the customer within 60 seconds.

## Risks
- Gateway API timeouts could lead to duplicate refund requests if idempotency keys are not enforced.
```

### 4.3 `decision` Template (ADR)
```markdown
---
description: Architectural decision record evaluating Redis vs Memcached for session storage.
tags: [architecture, cache]
owner: team:core
ods:
  profile: decision
  status: stable
---

# ADR 004: Redis for Session Storage

## Context
Our web tier requires a distributed, low-latency in-memory store for user authentication sessions.

## Decision
We choose Redis 7 as our primary session store, deployed via AWS ElastiCache.

## Alternatives
- **Memcached**: Extremely fast and simple, but lacks persistence, pub/sub for token invalidation, and rich data structures.
- **PostgreSQL**: Strong ACID guarantees, but disk I/O introduces unacceptable latency under high read volume.

## Consequences
- We gain sub-millisecond session lookup latency and built-in TTL expiration.
- We must monitor ElastiCache cluster memory usage and manage cluster failover replication.
```

### 4.4 `sop` Template (Standard Operating Procedure / Runbook)
```markdown
---
description: Runbook for performing scheduled PostgreSQL database maintenance and vacuuming.
tags: [database, ops]
owner: team:sre
ods:
  profile: sop
  status: stable
  code:
    - path: scripts/db-vacuum.sh
      role: pipeline
---

# SOP: PostgreSQL Database Vacuuming

## Purpose
Execute periodic vacuuming on high-churn transaction tables to prevent transaction ID wraparound and reclaim disk space.

## Prerequisites
- Superuser database credentials in AWS Secrets Manager.
- Maintenance window approval from the release coordinator.

## Steps
1. Notify on-call team in `#ops-alerts`.
2. Connect to the read-write primary instance.
3. Run `VACUUM (VERBOSE, ANALYZE) billing_transactions;`.

## Validation
Query `pg_stat_user_tables` to confirm that `last_vacuum` timestamp reflects the current execution time.

## Rollback
If database CPU exceeds 85% for more than 2 minutes, terminate the vacuum backend query using `SELECT pg_cancel_backend(pid);`.
```

### 4.5 `api` Template (Endpoint / Interface Contract)
```markdown
---
description: API endpoint contract for processing transaction refunds.
tags: [api, billing]
owner: team:billing
ods:
  profile: api
  status: stable
  code:
    - path: apps/api/src/routes/refunds.ts
      role: entrypoint
      symbol: HandleRefundPost
---

# POST /api/v1/refunds

## Overview
Issues a refund against a completed charge.

## Request
- **Headers**: `Authorization: Bearer <token>`, `Content-Type: application/json`
- **Body**:
  ```json
  {
    "charge_id": "ch_12345",
    "amount_cents": 2500,
    "reason": "customer_return"
  }
  ```

## Response
- **Status Code**: `200 OK`
  ```json
  {
    "refund_id": "ref_98765",
    "status": "succeeded",
    "created_at": 1705315200
  }
  ```

## Errors
- `400 Bad Request`: Invalid amount or missing charge ID.
- `404 Not Found`: Charge ID does not exist.
- `409 Conflict`: Charge has already been fully refunded.

## Examples
```bash
curl -X POST https://api.example.com/v1/refunds \
  -H "Authorization: Bearer sec_key" \
  -d '{"charge_id":"ch_12345","amount_cents":2500}'
```
```

### 4.6 `agent` Template (Autonomous Agent & Prompt Instruction Contract / `agent.md`)
```markdown
---
description: Autonomous coding agent prompt contract for implementing API endpoints.
tags: [agent, code-gen]
owner: team:ai-platform
ods:
  profile: agent
  status: stable
  depends:
    - ../specs/api-contract.md
  code:
    - path: src/server.ts
      role: entrypoint
      symbol: createServer
  context:
    max-depth: 2
    load:
      - ../schemas/api-schema.json
---

# Feature Implementation Agent

## Goal
Implement a secure, type-safe REST endpoint matching the specified OpenAPI schema.

## Task
Read the target schema, generate the endpoint handler in TypeScript, and register route validation middleware.

## Scope
- In Scope: Request parsing, parameter validation, business logic invocation, and error response formatting.

## Non-Scope
- Database schema migrations or changes to external payment webhooks.

## Context
Our API service uses Express with Zod validation. All errors must map to standard RFC 7807 problem details.

## Inputs
- Route path: `/api/v1/refunds`
- Method: `POST`
- Schema path: `schemas/refund-payload.json`

## Constraints
- Must not use `any` types in TypeScript code.
- Must sanitize all user-supplied query parameters before executing SQL queries.
- Must execute within a 15-minute agent timeout budget.

## Priority
1. Security and input validation.
2. Compliance with existing test suites.
3. Code cleanliness and lint compliance.

## Steps
1. Inspect the OpenAPI schema in `schemas/refund-payload.json`.
2. Generate route handler in `src/routes/refunds.ts`.
3. Add unit test assertions in `tests/refunds.test.ts`.
4. Run `pnpm test` to verify zero regressions.

## Output
- Completed route handler file.
- Passing unit test suite covering success (200) and failure (400, 404, 500) cases.

## Success Criteria
- `pnpm lint` and `pnpm test` exit with status code 0.
- All acceptance criteria in `specs/api-contract.md` are satisfied.

## Failure Modes
- Schema validation failure: Return `400 Bad Request` with structured error array.
- Database connection failure: Return `503 Service Unavailable` with retry-after header.

## Dependencies
- `specs/api-contract.md`
- `schemas/api-schema.json`

## Assumptions
- Node.js 20+ runtime is pre-configured in the CI environment.

## Examples
```json
// Example valid request payload
{
  "charge_id": "ch_987654",
  "amount_cents": 1500
}
```
```

### 4.7 `skill` Template (Reusable Agent Capability Package / `SKILL.md`)
```markdown
---
description: Reusable skill package for analyzing and optimizing PostgreSQL query plans.
tags: [skill, database, postgres]
owner: team:database
ods:
  profile: skill
  status: stable
  code:
    - path: scripts/explain-analyze.sql
      role: pipeline
---

# PostgreSQL Query Optimization Skill

## Purpose
Enables autonomous agents and developers to systematically analyze slow query plans and apply index optimizations.

## Capability
- EXPLAIN (ANALYZE, BUFFERS) query plan parsing.
- Missing index identification.
- Sequential scan bottleneck remediation.

## Activation
- Trigger when query execution time exceeds 250ms.
- Trigger when user requests database query performance tuning.

## Scope
- In Scope: Single-query plan optimization, B-Tree and GIN index recommendations.

## Non-Scope
- Sharding, connection pool tuning, or hardware resizing.

## Inputs
- Target SQL query string.
- Table schema DDL.
- Current table row count.

## Outputs
- Formatted `EXPLAIN` execution analysis.
- Recommended `CREATE INDEX` DDL statements.
- Estimated execution time reduction percentage.

## Workflow
1. Execute `EXPLAIN (ANALYZE, BUFFERS)` on the target query in staging.
2. Identify sequential scans and high-cost nested loop joins.
3. Propose index creation with minimal lock contention (`CONCURRENTLY`).
4. Re-run query plan to measure latency improvements.

## Rules
- Never run un-indexed queries against production tables without `LIMIT`.
- Always generate index names adhering to `idx_<table_name>_<column_name>`.

## Priority
1. Prevent database CPU spikes.
2. Minimize index write overhead on high-throughput tables.

## Validation
- Verify query plan uses index scan instead of sequential scan.
- Verify buffer read hits improve by at least 50%.

## Eval
- Test against standard benchmark suite: `benchmarks/sql-opt-cases.json`.
- Must achieve >90% accuracy on standard optimizer evaluation scenarios.

## Resources
- PostgreSQL 16 Official Indexing Documentation: `docs/reference/postgres-indexes.md`.

## Tools
- `psql` CLI / Postgres MCP Server.
- `pg_stat_statements` integration.

## Lifecycle
- Pre-execution: Verify read-only replica connection.
- Execution: Plan inspection and test execution.
- Post-execution: Revert temporary test indexes.

## Traceability
- Log all optimization runs to `logs/sql-opt-audit.jsonl` with execution timestamp, agent ID, and latency deltas.
```

---

## 5. Expected Section Heading Rationales

| Profile | Expected Section | Why this section is required |
| :--- | :--- | :--- |
| **`guide`** | `Overview` | Gives the reader immediate context on what the guide accomplishes. |
| | `Prerequisites` | Prevents wasted effort by listing required permissions, tools, and setup. |
| | `Steps` | Provides the sequential, actionable procedure. |
| | `Troubleshooting` | Answers common failure points without requiring external support. |
| **`decision`** | `Context` | Explains the motivating problem, constraints, and current state. |
| | `Decision` | States the exact architectural or technical choice made. |
| | `Alternatives` | Documents what options were evaluated and rejected (preventing repeated debates). |
| | `Consequences` | Highlights trade-offs, ongoing costs, and positive/negative outcomes. |
| **`sop`** | `Purpose` | Explains when and why this runbook must be executed. |
| | `Prerequisites` | Safety checklist and credentials required before execution. |
| | `Steps` | Exact operational commands or actions. |
| | `Validation` | Objective checks to confirm the operation succeeded. |
| | `Rollback` | Step-by-step recovery plan if the procedure fails or causes an outage. |
| **`agent`** | `Goal` | Defines the high-level objective of the autonomous agent instruction document. |
| | `Task` | Explicit, actionable assignment the model must execute. |
| | `Scope` & `Non-Scope` | Prevents scope creep, unrequested code modifications, and agent hallucinations. |
| | `Context` | Provides necessary background domain knowledge, frameworks, and architectural style. |
| | `Inputs` & `Output` | Explicitly binds the expected parameter schema and return format. |
| | `Constraints` | Declares non-negotiable safety guardrails, refusal boundaries, and style limits. |
| | `Priority` | Resolves conflicting trade-offs (e.g. correctness > speed > brevity). |
| | `Steps` | Defines the deterministic sequence of operations or tool invocations. |
| | `Success Criteria` | Verifiable, objective conditions required for task completion. |
| | `Failure Modes` | Prescribes how the agent must handle timeouts, missing resources, and errors. |
| | `Dependencies` & `Assumptions` | Declares required documentation/assets and environment preconditions. |
| | `Examples` | Grounding few-shot demonstrations to eliminate ambiguity. |
| **`skill`** | `Purpose` & `Capability` | Explains the intent and specific capabilities provided by the reusable skill. |
| | `Activation` | Specifies exact trigger conditions and heuristics so agents know when to invoke it. |
| | `Scope` & `Non-Scope` | Delineates capability boundaries to prevent inappropriate skill activation. |
| | `Inputs` & `Outputs` | Defines the data and arguments accepted and produced by the skill workflow. |
| | `Workflow` | Step-by-step execution procedure for performing the skill. |
| | `Rules` | Operating rules, safety guardrails, and compliance requirements. |
| | `Priority` | Execution precedence when multiple skills or steps interact. |
| | `Validation` & `Eval` | Verification checks and benchmark evaluation criteria for automated testing. |
| | `Resources` & `Tools` | Non-Markdown fixtures, scripts, MCP servers, and tool integrations required. |
| | `Lifecycle` & `Traceability` | Pre/post hooks, lifecycle state machine, and telemetry/audit logging contracts. |

---

## 6. Section Heading Alias Matching

Human and AI authors frequently use natural variations of heading titles. ODS validation tools MUST perform **alias matching** before reporting a missing section.

### 6.1 Standard Built-in Aliases

| Canonical Section | Recognized Synonyms & Aliases |
| :--- | :--- |
| **`Goal`** | `Objective`, `Objectives`, `Purpose`, `Intent` |
| **`Task`** | `Assignment`, `Prompt`, `Mission`, `Job` |
| **`Scope`** | `In Scope`, `Boundaries`, `Applicability` |
| **`Non-Scope`** | `Out of Scope`, `Exclusions`, `Non Goals`, `Non-Goals` |
| **`Requirements`** | `Functional Requirements`, `Needs`, `Specifications` |
| **`Acceptance Criteria`** | `Acceptance`, `Success Criteria`, `Definition of Done` |
| **`Overview`** | `Introduction`, `Summary`, `Background`, `About` |
| **`Prerequisites`** | `Requirements`, `Before You Begin`, `Setup Required` |
| **`Steps`** | `Instructions`, `Procedure`, `Process`, `Execution` |
| **`Troubleshooting`** | `Common Issues`, `FAQ`, `Failure Modes`, `Debugging` |
| **`Context`** | `Background`, `Problem Statement`, `Motivation` |
| **`Decision`** | `Outcome`, `Chosen Option`, `Resolution` |
| **`Alternatives`** | `Options Considered`, `Alternative Approaches`, `Other Solutions` |
| **`Consequences`** | `Outcome`, `Implications`, `Impact`, `Trade-offs` |
| **`Validation`** | `Verification`, `Checks`, `Testing`, `Confirmation` |
| **`Rollback`** | `Recovery`, `Revert Procedure`, `Undo Steps`, `Failover` |
| **`Rules`** | `Standards`, `Requirements`, `Mandates`, `Guidelines`, `Policies` |
| **`Action Items`** | `Actions`, `Next Steps`, `TODO`, `Follow-ups` |
| **`Risks`** | `Risks & Mitigations`, `Concerns`, `Potential Issues` |
| **`Trade-offs`** | `Tradeoffs`, `Pros & Cons`, `Evaluations` |
| **`Inputs`** | `Parameters`, `Arguments`, `Input Data`, `Input Schema` |
| **`Output` / `Outputs`** | `Deliverable`, `Deliverables`, `Result`, `Results`, `Returns`, `Expected Output`, `Response Format` |
| **`Constraints`** | `Guardrails`, `Refusal Guardrails`, `Limitations`, `Safety Rules` |
| **`Priority`** | `Context Priority`, `Resolution Priority`, `Precedence` |
| **`Failure Modes`** | `Failure Scenarios`, `Edge Cases`, `Error Handling`, `Mitigations` |
| **`Dependencies`** | `Required Docs`, `External Dependencies` |
| **`Assumptions`** | `Defaults`, `Presumptions`, `Prerequisites Assumptions` |
| **`Examples`** | `Sample Inputs`, `Sample Prompts`, `Few-Shot Examples`, `Scenarios` |
| **`Capability`** | `Capabilities`, `Features`, `Actions` |
| **`Activation`** | `Triggers`, `When to Use`, `Activation Conditions`, `Trigger Conditions` |
| **`Workflow`** | `Execution Flow`, `Process Flow`, `Operating Workflow` |
| **`Eval`** | `Evaluation`, `Benchmarks`, `Eval Suite`, `Quality Gates`, `Rubric` |
| **`Resources`** | `References`, `Assets`, `Knowledge Base` |
| **`Tools`** | `Tooling`, `Tool Integrations`, `Functions`, `Commands`, `MCP Servers` |
| **`Lifecycle`** | `Phases`, `State Machine`, `Hooks`, `Execution Lifecycle` |
| **`Traceability`** | `Auditing`, `Provenance`, `Logging`, `Telemetry` |

### 6.2 Workspace Section Aliases (`ods.toml`)

Workspaces MAY define custom section aliases in `ods.toml` under the `[aliases]` table:

```toml
# ods.toml
[aliases]
Goal = ["Target", "Business Objective"]
Validation = ["Sanity Checks", "Smoke Tests"]
Eval = ["Benchmark Suite", "Model Evals"]
```

---

## 7. Custom Profiles & Profile Definition Files

Workspaces can define domain-specific custom profiles by creating profile definition Markdown files and registering their exact paths in `ods.toml`.

### 7.1 Custom Profile Definition File (`docs/profiles/rfc.md`)

```markdown
---
ods:
  custom_profile:
    name: rfc
    required_keys:
      - github-issue
---

# Profile: RFC

## Summary | Executive Summary

## Motivation | Problem Statement

## Proposed Design | Technical Specification

## Drawbacks | Risks

## Unresolved Questions | Open Issues
```

- Pipe characters (`|`) in section headings define acceptable heading alternatives.
- The profile identifier is derived from `ods.custom_profile.name` or the file stem (`rfc`).
- Every path listed in `custom_profiles` MUST exist at the configured location and MUST resolve to a Markdown file or a profile directory. Tools MUST NOT silently skip a missing path or search another location.
- A file containing `ods.custom_profile` MUST be one of the registered profile-definition files (or a file inside a registered profile directory). Ordinary documents MUST use `ods.profile` to select the registered profile.

### 7.1.1 Profile-definition metadata

The `ods.custom_profile` block of a registered profile-definition file MAY contain these profile-definition keys:

| Key | Placement | Meaning |
| :--- | :--- | :--- |
| `name` | `ods.custom_profile.name` | Optional profile identifier. When absent, the file stem is used. |
| `required_keys` | `ods.custom_profile.required_keys` | Keys that documents using the profile SHOULD contain. |
| `optional_keys` | `ods.custom_profile.optional_keys` | Useful keys that are documented for the profile but are not required. |
| `forbidden_keys` | `ods.custom_profile.forbidden_keys` | Keys that documents using the profile SHOULD NOT contain. |

These keys describe the profile definition; they are not copied into documents using the profile. Each `required_keys` entry is matched against a top-level frontmatter key in the target document. Profile-specific document keys MUST NOT be nested under `ods:`. The standard engine keys (`profile`, `status`, `id`, `share`, `depends`, `related`, `resources`, `code`, and `context`) remain separate from profile-definition metadata.

`required_keys`, `optional_keys`, and `forbidden_keys` are optional lists of top-level key names. Add one `-` entry for each key. If a list has no entries, omit that profile-definition key; `[]` is valid YAML for an explicitly empty list but is not required.

`required_keys` is a presence-only contract: a conformant tool MUST NOT infer a value type, enum, or business meaning from it. A key satisfies the requirement when it is present with a non-null YAML value, including an empty list or structured value. An absent or explicit null key does not satisfy it. Key matching is case-insensitive after normalization; authors SHOULD write keys in lowercase.

`optional_keys` and `forbidden_keys` do not define value types. A tool SHOULD report a `PROF-004` warning when a target document contains a `forbidden_keys` entry.

If a document declares an `ods.profile` name that is not a standard profile or a profile loaded from a registered definition path, the tool MUST report a `PROF-001` error. The diagnostic MUST identify the configured `custom_profiles` paths so the author can correct the exact file location or profile name.

If a path declared by `custom_profiles` does not exist, is not a Markdown file or profile directory, or contains invalid profile-definition frontmatter, the tool MUST fail with a `PROF-005` error and identify the configured path. If `ods.custom_profile` appears in a file that is not selected by `custom_profiles` or a registered pack, the tool MUST fail with a `PROF-006` error.

Missing profile-required keys MUST be reported as a profile validation warning (`PROF-003`). Under the binary compliance contract, warnings do not cause a non-zero exit code unless another error is present. Tools MAY offer a stricter policy, but it is outside the ODS 0.1 core contract.

Example target document:

```markdown
---
github-issue: 123
ods:
  profile: rfc
  status: draft
---

# RFC: Retry Policy
```

Profile-required metadata is for domain keys such as issue IDs, service names, or owners. Agent and skill execution contracts remain Markdown body sections, not `required_keys` entries.

### 7.2 Registering Custom Profiles in `ods.toml`

```toml
# ods.toml
spec = "0.1"

custom_profiles = [
  "docs/profiles/rfc.md",
  "docs/profiles/experiment.md"
]
```

---

## 8. ODS Packs (Reusable Profile Catalogs)

An **ODS Pack** is a versioned repository or directory containing reusable profiles, templates, and agent skills.

```toml
# ods.toml
packs = [
  "vendor/engineering-pack",
  "github.com/acme/ods-security-pack"
]
```

### 8.1 Profile Resolution Order
When resolving a document's `ods.profile`, tools MUST search in this priority order:
1. **Standard built-in profiles** (`note`, `guide`, `feature`, `agent`, `skill`, etc.)
2. **Explicit workspace `custom_profiles`** paths declared in `ods.toml`
3. **Imported `packs`** in the order declared in `ods.toml`

If a profile name is declared in multiple places, the first resolved definition wins, and tools SHOULD emit a diagnostic warning. Unresolved profile names MUST NOT fall back to `note` behavior; they produce a `PROF-001` error.

---

## 9. Valid vs. Invalid Profile Usage Examples

### Valid Decision Document
```markdown
---
ods:
  profile: decision
  status: stable
---

# ADR 009: Postgres for Primary Storage

## Context
We need a reliable relational database for financial ledger records.

## Decision
We choose PostgreSQL 16 managed on AWS RDS.

## Alternatives Considered
- DynamoDB: Fast but lacks ACID multi-row transactions.
- MySQL: Viable, but team has deeper Postgres expertise.

## Consequences
We gain strong consistency and JSONB support; we must manage RDS connection pooling.
```
*Why it is valid*: All 4 required sections (`Context`, `Decision`, `Alternatives Considered` [alias], `Consequences`) are present as recognized H2/H3 profile headings.

### Invalid Agent Document (Frontmatter Pollution Anti-Pattern)
```markdown
# ERRONEOUS CODE:
---
description: Code generation instructions.
role: Autonomous TypeScript Engineer            # ERROR: Operational execution key in frontmatter
refusal_guardrails: [Never drop production DB]  # ERROR: Operational execution key in frontmatter
workflow: [Inspect, Code, Test]                 # ERROR: Operational execution key in frontmatter
ods:
  profile: agent
  status: stable
---

# Code Generation Agent

## Goal
Implement route handlers.
```
*Why it is invalid*: Violates 3-tier metadata separation. Operational keys like `role:`, `refusal_guardrails:`, and `workflow:` belong exclusively in Markdown body headings (`## Constraints`, `## Steps`, `## Workflow`), not in YAML frontmatter.

---

## 10. Design Decisions

### Why use H2/H3 headings instead of rigid JSON/YAML schemas for document bodies?
Engineers and authors write Markdown naturally using section headings. Forcing authors into structured JSON arrays or proprietary markdown frontmatter fields damages readability in text editors and breaks standard Markdown rendering.

### Why headings instead of frontmatter keys for agent prompts and skills?
Placing execution contracts, guardrails, workflows, and tools in YAML frontmatter pollutes repository metadata, breaks toolchain neutrality, and bloats machine indexing indexes. Storing them as standard Markdown body headings (`## Task`, `## Constraints`, `## Workflow`) keeps prompt contracts 100% human-readable, token-efficient, and compatible with any LLM framework or Markdown parser.

### Why `agent.md` instead of `prompt.md`?
`agent.md` directly reflects the operational artifact: an executable instruction contract for an autonomous coding agent or LLM worker. "Prompt" remains a generic concept, whereas `agent.md` establishes a consistent naming convention alongside `SKILL.md` and standard ODS profiles.

### Why additive custom profiles without inheritance hierarchies?
Profile inheritance (e.g. `guide` extends `base-doc` extends `root`) adds significant parser complexity and mental overhead with minimal real-world value. Flat, additive profile schemas are easy to inspect, debug, and validate.

---

## Navigation & Reading Order

| [← Previous Chapter](keys.md) | [📑 Specification Index](README.md) | [Next Chapter →](graph.md) |
| :--- | :---: | ---: |
| **03. Frontmatter Key Dictionary** | **Open Document Spec (ODS)** | **05. Document Graph & Identity** |
