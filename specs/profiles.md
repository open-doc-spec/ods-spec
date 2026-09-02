---
description: Standard document profiles, expected section heading contracts, complete
  copy-paste templates, custom profile catalogs, and reusable packs.
profile: note
status: stable
depends:
- README.md
- keys.md
related:
- core.md
- validation.md
- indexes.md
- ../guides/02-pick-a-shape.md
- ../guides/08-extend-ods.md
- ../guides/09-domain-ontology.md
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

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14, exactly as stated in [README.md §1](README.md#1-conformance-language). That is the canonical statement; do not maintain a second copy here.

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

ODS standardizes **13 Universal Profiles** covering human prose structures, engineering workflows, and autonomous agent prompt execution contracts:

| Profile | Intent & Usage | Expected H2/H3 Sections (`##` or `###`) |
| :--- | :--- | :--- |
| **`note`** | Free-form notes, scratchpads, entities, and unstructured knowledge. (Default fallback). | *(None required)* |
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

### 3.1 Orthogonal Keys vs Document Profiles

**Profiles** define prose shapes (`## Overview`, `## Steps`, `## Decision`). Other ODS keys are **orthogonal** — they can appear on any profile:

1. **Graph keys** (`depends`, `related`, `resources`, `code`, `load`) — see [keys.md §7](keys.md#7-ods-engine-keys-flat-top-level) and [graph.md](graph.md).
2. **Pareto ontology (ODS 2.1)** — flat `entity`, `domain`, and `schema` on concept or feature docs; typed predicates on `related`. See [guides/09-domain-ontology.md](../guides/09-domain-ontology.md).
3. **OKF superset** — top-level keys such as `type`, `sources`, and `verified` on standard profiles. See [core.md §3.1](core.md#31-frontmatter).

Memory tiers, `invariants`, and the `ods:` wrapper are **not** part of ODS 2.0+.

Profile section headings are **advisory** in ODS 2.0+. Tools MAY report missing sections as hints; there is no `PROF-002` lint error.

---

## 4. Complete Profile Templates (Copy-Paste Ready)

All templates use **flat frontmatter** (no `ods:` wrapper). Engine keys sit at the top level beside universal metadata.

### 4.1 `note` Template (Default)

```markdown
---
description: Scratchpad for billing domain concepts.
profile: note
status: draft
tags: [billing]
---

# Billing Notes

Free-form content. No required sections.
```

### 4.2 `guide` Template (How-To Tutorial)

```markdown
---
description: Step-by-step tutorial for configuring user authentication sessions.
tags: [auth, setup]
owner: team:platform
profile: guide
status: stable
depends:
  - ../prerequisites/cli-setup.md
code:
  - src/auth.ts
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
- **Error: Redis Connection Timeout**: Verify that your Redis instance is running on port 6379.
```

### 4.3 `feature` Template (PRD / Capability Spec)

```markdown
---
description: Product capability specification for automated customer refund processing.
tags: [billing, payments]
owner: team:billing
profile: feature
status: stable
depends:
  - ../auth/sessions.md
load:
  - ../schemas/refund-payload.json
---

# Customer Refund Processing

## Goal
Enable customer support agents to issue partial or full credit card refunds from the dashboard.

## Scope
- In Scope: Visa, Mastercard, and Stripe payment methods.
- Out of Scope: Wire transfer refunds (v2).

## Requirements
1. Support agents can select order line items to refund.
2. System records agent ID and reason code for auditing.

## Acceptance Criteria
- Refund requests process synchronously with the payment gateway.
- Confirmation email sent within 60 seconds.

## Risks
- Gateway timeouts could cause duplicate refunds without idempotency keys.
```

### 4.4 `decision` Template (ADR)

```markdown
---
description: ADR evaluating Redis vs Memcached for session storage.
tags: [architecture, cache]
owner: team:core
profile: decision
status: stable
---

# ADR 004: Redis for Session Storage

## Context
Our web tier requires a distributed, low-latency in-memory store for sessions.

## Decision
We choose Redis 7 via AWS ElastiCache.

## Alternatives
- **Memcached**: Fast but lacks persistence and pub/sub.
- **PostgreSQL**: Strong ACID but higher read latency.

## Consequences
- Sub-millisecond session lookups and built-in TTL.
- Must monitor cluster memory and failover.
```

### 4.5 `sop` Template (Runbook)

```markdown
---
description: Runbook for PostgreSQL vacuum maintenance.
tags: [database, ops]
owner: team:sre
profile: sop
status: stable
code:
  - scripts/db-vacuum.sh
---

# SOP: PostgreSQL Database Vacuuming

## Purpose
Prevent transaction ID wraparound on high-churn tables.

## Prerequisites
- Superuser credentials in Secrets Manager.
- Maintenance window approval.

## Steps
1. Notify on-call in `#ops-alerts`.
2. Connect to the primary instance.
3. Run `VACUUM (VERBOSE, ANALYZE) billing_transactions;`.

## Validation
Confirm `last_vacuum` updated in `pg_stat_user_tables`.

## Rollback
If CPU exceeds 85% for 2+ minutes, cancel the backend with `pg_cancel_backend`.
```

### 4.6 `api` Template (Endpoint Contract)

```markdown
---
description: API contract for processing transaction refunds.
tags: [api, billing]
owner: team:billing
profile: api
status: stable
code:
  - apps/api/src/routes/refunds.ts
---

# POST /api/v1/refunds

## Overview
Issues a refund against a completed charge.

## Request
- **Headers**: `Authorization: Bearer <token>`, `Content-Type: application/json`
- **Body**: `{ "charge_id": "ch_12345", "amount_cents": 2500, "reason": "customer_return" }`

## Response
- **200 OK**: `{ "refund_id": "ref_98765", "status": "succeeded" }`

## Errors
- `400`: Invalid amount or missing charge ID.
- `404`: Charge not found.
- `409`: Charge already fully refunded.

## Examples
```bash
curl -X POST https://api.example.com/v1/refunds \
  -H "Authorization: Bearer sec_key" \
  -d '{"charge_id":"ch_12345","amount_cents":2500}'
```
```

### 4.7 `architecture` Template

```markdown
---
description: Checkout service architecture overview.
profile: architecture
status: stable
depends:
  - ../decisions/004-redis-sessions.md
---

# Checkout Service Architecture

## Overview
Event-driven checkout flow with payment gateway integration.

## Components
- API gateway, order service, payment adapter, notification worker.

## Data Flow
Client → API → order queue → payment → webhook → email.

## Trade-offs
Async processing improves resilience; adds observability requirements.
```

### 4.8 `policy` Template

```markdown
---
description: Refund eligibility policy.
profile: policy
status: stable
related:
  - ../features/refunds.md
---

# Refund Eligibility Policy

## Purpose
Define when refunds may be issued.

## Scope
All customer-facing payment channels.

## Rules
1. Refunds within 30 days of purchase.
2. Manager approval required above $500.

## Exceptions
Fraud disputes follow the security incident SOP.
```

### 4.9 `meeting` Template

```markdown
---
description: Sprint planning notes.
profile: meeting
status: draft
---

# Sprint 42 Planning

## Attendees
Alice, Bob, Carol

## Agenda
1. Review carry-over items
2. Capacity planning

## Decisions
Defer international refunds to next sprint.

## Action Items
- [ ] Alice: draft refund API spec
```

### 4.10 `checklist` Template

```markdown
---
description: Production release quality gate.
tags: [release, deployment]
owner: team:platform
profile: checklist
status: stable
depends:
  - ../specs/release-v1.1.md
---

# Production Release Quality Gate

## Overview
Mandatory checks before promoting staging to production.

## Items
- [ ] Migrations tested on staging replica.
- [ ] E2E smoke tests pass.
- [ ] Container scan shows no critical CVEs.

## Verification
Run `pnpm run test:e2e:staging` and confirm sign-off in `#release-approvals`.

## Notes
Deployment freeze Fridays after 14:00 UTC.
```

### 4.11 `agent` Template (`agent.md`)

```markdown
---
description: Agent contract for implementing API endpoints.
tags: [agent, code-gen]
owner: team:ai-platform
profile: agent
status: stable
depends:
  - ../specs/api-contract.md
code:
  - src/server.ts
load:
  - ../schemas/api-schema.json
---

# Feature Implementation Agent

## Goal
Implement a secure REST endpoint matching the OpenAPI schema.

## Task
Generate the handler in TypeScript and register validation middleware.

## Scope
Request parsing, validation, business logic, error formatting.

## Non-Scope
Database migrations or payment webhooks.

## Context
Express with Zod validation; errors use RFC 7807 problem details.

## Inputs
- Route: `POST /api/v1/refunds`
- Schema: `schemas/refund-payload.json`

## Constraints
- No `any` types.
- Sanitize all user input before SQL.

## Priority
1. Security. 2. Test compliance. 3. Lint cleanliness.

## Steps
1. Inspect the schema.
2. Generate `src/routes/refunds.ts`.
3. Add tests and run `pnpm test`.

## Output
Route handler and passing unit tests.

## Success Criteria
`pnpm lint` and `pnpm test` exit 0.

## Failure Modes
Schema failure → 400; DB unavailable → 503 with retry-after.

## Dependencies
- `specs/api-contract.md`

## Assumptions
Node.js 20+ in CI.

## Examples
```json
{ "charge_id": "ch_987654", "amount_cents": 1500 }
```
```

### 4.12 `skill` Template (`SKILL.md`)

```markdown
---
description: Skill for PostgreSQL query plan optimization.
tags: [skill, database]
owner: team:database
profile: skill
status: stable
code:
  - scripts/explain-analyze.sql
---

# PostgreSQL Query Optimization Skill

## Purpose
Analyze slow query plans and recommend indexes.

## Capability
EXPLAIN parsing, missing index detection, sequential scan remediation.

## Activation
Trigger when query time exceeds 250ms or user requests tuning.

## Scope
Single-query optimization and index recommendations.

## Non-Scope
Sharding, pool tuning, hardware resizing.

## Inputs
SQL query, table DDL, row counts.

## Outputs
EXPLAIN analysis, `CREATE INDEX` DDL, estimated improvement.

## Workflow
1. Run `EXPLAIN (ANALYZE, BUFFERS)` in staging.
2. Identify sequential scans.
3. Propose `CREATE INDEX CONCURRENTLY`.
4. Re-measure latency.

## Rules
Never run unbounded queries on production without `LIMIT`.

## Priority
Prevent CPU spikes over index write overhead.

## Validation
Query plan uses index scan; buffer hits improve ≥50%.

## Eval
Benchmark suite `benchmarks/sql-opt-cases.json` — >90% accuracy.

## Resources
`docs/reference/postgres-indexes.md`

## Tools
`psql`, Postgres MCP server.

## Lifecycle
Pre: verify read replica. Post: revert test indexes.

## Traceability
Log runs to `logs/sql-opt-audit.jsonl`.
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

### 6.2 Alias Resolution (Normative)

The alias table above is intentionally overlapping: `Requirements` is both a canonical section of `feature` and an accepted synonym for `Prerequisites`; `Success Criteria` is canonical for `agent` and a synonym for `Acceptance Criteria`; `FAQ` and `Failure Modes` are both canonical elsewhere and synonyms for `Troubleshooting`. Matching therefore MUST follow a fixed order:

1. **Exact canonical match wins.** If a heading exactly matches a canonical section name expected by the document's profile, it satisfies that section and is not considered as an alias for any other.
2. **Alias match second.** A remaining unmatched expected section is satisfied by any heading listed among its synonyms.
3. **One heading satisfies at most one section.** Once a heading is consumed by a section, it is not reused. Where two unmatched sections both accept the same heading, the one declared earlier in the profile's section list takes it.
4. **Matching is case-insensitive** and ignores surrounding punctuation and numbering (`## 3. Steps` matches `Steps`).
5. **Workspace aliases extend, never replace**, the built-in table. A workspace alias that collides with a built-in canonical name is ignored and reported as a warning.

### 6.3 Workspace Section Aliases (`ods.toml`)

Workspaces MAY define additional section synonyms under the `[aliases.sections]` table:

```toml
# ods.toml
[aliases.sections]
Goal = ["Target", "Business Objective"]
Validation = ["Sanity Checks", "Smoke Tests"]
Eval = ["Benchmark Suite", "Model Evals"]
```

A bare `[aliases]` table is accepted as a legacy spelling of `[aliases.sections]`. See [indexes.md §3.5](indexes.md#35-aliases).

---

## 7. Custom Profiles & Profile Definition Files

Workspaces define domain-specific profiles by creating profile-definition Markdown files and registering their paths in `ods.toml`. See [keys.md §8](keys.md#8-custom-profile-definition-keys) for the key reference.

### 7.1 Custom Profile Definition File (`docs/profiles/rfc.md`)

```markdown
---
name: rfc
description: RFC-style design document shape.
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
- The profile identifier comes from `name` or the file stem (`rfc`).
- Every path in `custom_profiles` MUST exist (`PROF-005`).
- Profile-definition metadata MUST appear only in registered definition files (`PROF-006`).

### 7.1.1 Profile-definition metadata

Profile-definition frontmatter uses the keys in [`profile.schema.json`](../schemas/2.0.0/profile.schema.json):

| Key | Meaning |
| :--- | :--- |
| `name` | Optional profile identifier. When absent, the file stem is used. |
| `description` | Optional one-line summary shown in tooling. |
| `required_sections` | Canonical H2/H3 sections expected in documents using the profile (advisory). |
| `optional_sections` | Recognized but never required sections. |
| `required_keys` | Top-level keys documents using the profile should contain. |
| `optional_keys` | Useful top-level keys that are not required. |
| `forbidden_keys` | Top-level keys that should not appear with the profile. |

**Two ways to declare sections.** A profile may list sections in `required_sections` / `optional_sections`, or as `##` headings in the body with `|` alternatives (as in §7.1). Where both are present, `required_sections` wins; tools SHOULD warn about ambiguity.

Profile-definition keys are not copied into documents using the profile. Engine keys (`profile`, `status`, `depends`, `related`, `resources`, `code`, `load`) remain flat at the top level — never nested under `ods:`.

`required_keys` is presence-only: a key satisfies the requirement when present with a non-null value. Matching is case-insensitive after normalization.

If `profile` does not resolve to a standard or registered custom profile, the tool MUST report `PROF-001`. If `custom_profiles` points to a missing path, report `PROF-005`. If profile-definition metadata appears outside a registered file, report `PROF-006`.

Example target document:

```markdown
---
github-issue: 123
profile: rfc
status: draft
description: Retry policy for payment gateway calls.
---

# RFC: Retry Policy
```

Profile-required metadata is for domain keys (issue IDs, service names). Agent and skill execution contracts belong in Markdown body sections, not `required_keys`.

### 7.2 Registering Custom Profiles in `ods.toml`

```toml
# ods.toml
spec = "2.0"

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

When resolving a document's `profile`, tools MUST search in this priority order:
1. **Standard built-in profiles** (`note`, `guide`, `feature`, `agent`, `skill`, etc.)
2. **Explicit workspace `custom_profiles`** paths declared in `ods.toml`
3. **Imported `packs`** in the order declared in `ods.toml`

If a profile name is declared in multiple places, the first resolved definition wins, and tools SHOULD emit a diagnostic warning. Unresolved profile names MUST NOT fall back to `note` behavior; they produce a `PROF-001` error.

---

## 9. Valid vs. Invalid Profile Usage Examples

### Valid Decision Document

```markdown
---
profile: decision
status: stable
description: ADR for primary database selection.
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

*Why it is valid*: All four expected sections are present (`Alternatives Considered` matches the `Alternatives` alias).

### Invalid Agent Document (Frontmatter Pollution Anti-Pattern)

```markdown
# ERRONEOUS CODE:
---
description: Code generation instructions.
role: Autonomous TypeScript Engineer
refusal_guardrails: [Never drop production DB]
workflow: [Inspect, Code, Test]
profile: agent
status: stable
---

# Code Generation Agent

## Goal
Implement route handlers.
```

*Why it is invalid*: Operational keys like `role:`, `refusal_guardrails:`, and `workflow:` belong in Markdown body headings (`## Constraints`, `## Steps`, `## Workflow`), not in YAML frontmatter.

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
