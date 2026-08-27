---
description: "Learn how to ground AI agents in business concepts using Neuro-Symbolic Ontologies and Cognitive Memory in ODS."
tags:
  - guide
  - ontology
  - memory
  - ai
owner: team:docs
ods:
  profile: guide
  status: stable
  depends:
    - 01-first-document.md
    - 03-link-documents.md
    - 04-bind-code-and-files.md
  related:
    - 05-ai-reading-list.md
    - 06-run-the-workspace.md
    - ../specs/graph.md
    - ../specs/context.md
---

# Domain Modeling & Agent Memory

This guide teaches you how to use **Domain Modeling** and **Cognitive Agent Memory** in Open Document Spec (ODS). By the end of this tutorial, you will know how to ground AI coding agents in unambiguous business concepts, enforce hard validation guardrails, and track state changes over time.

---

## Overview

Traditional documentation stores unstructured paragraphs of text. While humans can parse nuance, AI agents frequently misunderstand implicit business rules, invent invalid relationships, and hallucinate outdated facts.

ODS solves this by introducing **Structured Engine Keys** for:
1. **Domain Modeling**: Defining clear business entity classes, domain boundaries, typed relationships, and deterministic invariants.
2. **Cognitive Memory**: Tracking real-world validity windows, agent assertions, and structured state mutations over time.

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                          THE DUAL-GRAPH MODEL                           │
│                                                                         │
│  1. DOMAIN GRAPH (High-Level Business Reality)                          │
│     [Customer] ────owns────► [Subscription]                             │
│         │                                                               │
│         └──────────governed_by───► [Refund SLA]                         │
│                                                                         │
│  2. LEXICAL GRAPH (Markdown Files & Code Bindings)                      │
│     [entities/customer.md] ────depends────► [policies/refund-sla.md]    │
│              │                                                          │
│              └───────────────code─────────► [src/models/customer.ts]    │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Part 1: Domain Modeling & Entities

**Domain Modeling** in ODS gives AI agents a shared, machine-verifiable vocabulary. Instead of guessing how "Customers" relate to "Subscriptions" and "Refunds", you state the rules explicitly in YAML frontmatter.

### The Direct Pareto Domain Keys

All domain modeling keys live directly under `ods:` with zero unnecessary nesting:

```yaml
ods:
  profile: note                               # Standard universal profile (or architecture / feature)
  status: stable

  entity: Customer                            # Canonical entity class name
  domain: Billing                             # Business domain partition
  schema: schemas/customer.schema.json        # "Paid at the door" disk validator

  # Unified Discovery & Semantic Relations (Pareto Shorthand)
  related:
    - policies/refund-sla.md
    - owns: @Subscription
    - governed_by: @RefundPolicy
    - maps_to: datasets/bq-customers.md

  invariants:
    - "mrr >= 0"
    - "email is required"
```

| Key | What it Does | Why AI Needs It |
| :--- | :--- | :--- |
| **`entity`** | Names the primary business class or concept. | Disambiguates terms (e.g. `Customer` vs `UserAccount`). |
| **`domain`** | Partitions the workspace into business sub-systems. | Allows AI agents to filter context by domain (e.g. `domain: Billing`). |
| **`schema`** | Points to a disk schema file (`.json`, `.ts`, `.py`). | **Paid-at-the-door contract**: Rejects invalid payloads before code execution. |
| **`related`** | Declares typed semantic edges between concepts (`owns`, `is_a`, `governed_by`, …) alongside plain "see also" links. | Replaces ambiguous prose links with explicit graph relations. |
| **`invariants`**| Non-negotiable boolean business guardrails. | AI agents MUST evaluate these before mutating application state. |

### Standard Semantic Predicates

Connect entities under **`ods.related`**, using these standard predicates:

- `is_a`: Class inheritance (e.g., `EnterpriseCustomer` $\rightarrow$ `Customer`).
- `part_of`: Structural composition (e.g., `InvoiceLineItem` $\rightarrow$ `Invoice`).
- `owns`: Domain ownership & lifecycle containment (e.g., `Customer` $\rightarrow$ `Subscription`).
- `governed_by`: Compliance rules or policies (e.g., `Refund` $\rightarrow$ `RefundPolicy`).
- `maps_to`: Technical asset binding (e.g., `Customer` $\rightarrow$ BigQuery Table / OpenAPI Route).
- `derives_from`: Data lineage or computed derivation.

The vocabulary is closed — a verb outside it is a lint error rather than a new edge type. For a verb your domain genuinely needs, say so explicitly:

```yaml
ods:
  related:
    - predicate: custom
      custom_predicate: reconciles_with
      target: "@Ledger"
```

Full list with inverses: [`specs/graph.md` §4.1](../specs/graph.md#41-the-complete-predicate-vocabulary).

> **`ods.relations` is deprecated.** It does what `ods.related` already does and is scheduled for removal in 2.0. Existing documents keep working and emit a warning. Write new edges under `ods.related`.

---

## Part 2: Temporal Agent Memory

As AI agents execute tasks, they observe events and make decisions. **Cognitive Memory** enables agents to record what happened, when it became true, and how entity state mutated.

### The Cognitive Memory Block (`memory:`)

Memory properties can be neatly encapsulated in a top-level **`memory:`** block (or placed directly under `ods:`):

```yaml
# Encapsulated Cognitive Memory Block
memory:
  tier: episodic                              # episodic | semantic | procedural | state | profile
  valid_from: "2026-08-26T10:00:00Z"          # When the fact became true in reality
  valid_to: null                              # null = currently active fact
  asserted_at: "2026-08-26T10:05:00Z"         # When the agent observed the event
  pin: true                                   # Protects from automated decay pruning
  mutations:
    - entity: Customer
      id: cust-4048
      property: plan
      old_value: "starter"
      new_value: "enterprise"
      confidence: 1.0

ods:
  profile: note                               # Standard universal profile (or sop / meeting)
  status: stable
```

| Key | What it Does | Why AI Needs It |
| :--- | :--- | :--- |
| **`tier`** | Classifies memory: `episodic` (events), `semantic` (settled facts), `procedural` (how-to), `state` (what is true right now), `profile` (what is persistently true about an actor). | Helps agents retrieve the right level of memory abstraction. |
| **`valid_from`** / **`valid_to`** | Real-world validity time window. | Prevents agents from acting on expired or obsolete information. |
| **`asserted_at`** | The timestamp when the recording agent logged the fact. | Supports bi-temporal audits and multi-agent event sequencing. |
| **`mutations`** | Structured state transition record (`entity`, `id`, `property`, `old_value`, `new_value`). | Enables Graphiti-style temporal attribute diffing without parsing raw prose. |
| **`pin`** | Flag (`true`/`false`) preventing decay pruning. | Keeps critical milestone decisions in active memory indefinitely. |

---

## Step-by-Step Tutorial

Let's build a working ontology entity and record an agent interaction episode!

### Step 1: Create the Entity Definition (`entities/customer.md`)

Create a new file at `entities/customer.md`:

```markdown
---
description: Canonical customer entity definition with neuro-symbolic domain relations and invariants.
tags: [ontology, customer, billing]
owner: team:billing
ods:
  profile: note
  status: stable
  share: public
  entity: Customer
  domain: Billing
  schema: ../schemas/customer.schema.json
  related:
    - owns: "@Subscription"
    - governed_by: "@refund-sla.md"
  invariants:
    - "mrr >= 0"
    - "email is required"
  depends:
    - subscription.md
    - ../policies/refund-sla.md
---

# Customer Entity

## Overview
The Customer entity represents a registered billing account within our SaaS platform.

## Attributes
- `id`: Unique alphanumeric customer ID (e.g. `cust-4048`).
- `name`: Legal account name.
- `plan`: Active tier (`starter`, `pro`, `enterprise`).
- `mrr`: Monthly Recurring Revenue in USD.

## Relationships
- **owns**: `Subscription` (1 to N).
- **governed_by**: `Refund SLA` Policy.

## Invariants
- MRR must never be negative (`mrr >= 0`).
- Valid email address is required for invoicing.

## Examples
```json
{
  "id": "cust-4048",
  "name": "Acme Corp",
  "plan": "enterprise",
  "mrr": 2400.00
}
```
```

### Step 2: Record an Interaction Episode (`memory/upgrade-cust-4048.md`)

When an autonomous agent upgrades a customer, it writes a memory episode:

```markdown
---
description: Episodic agent memory recording customer plan upgrade from Starter to Enterprise.
tags: [memory, episodic, billing]
owner: agent:billing-assistant
ods:
  profile: note
  status: stable
  tier: episodic
  valid_from: "2026-08-26T10:00:00Z"
  valid_to: null
  asserted_at: "2026-08-26T10:05:00Z"
  pin: true
  mutations:
    - entity: Customer
      id: cust-4048
      property: plan
      old_value: "starter"
      new_value: "enterprise"
      confidence: 1.0
  depends:
    - ../entities/customer.md
---

# Customer Plan Upgrade Episode

## Interaction Summary
Customer requested immediate upgrade to Enterprise plan after signing contract `CONT-2026-98`.

## Tool Execution Trace
1. Executed `stripe.subscriptions.update(cust_4048, plan="enterprise")` -> Success (HTTP 200).
2. Emitted audit log `AUDIT-UPGRADE-4048`.

## State Mutations
- `Customer:cust-4048.plan`: `"starter"` $\rightarrow$ `"enterprise"`.
```

---

## Validation & Verification

Run `ods lint` in your terminal to verify your ontology and memory documents:

```bash
$ ods lint .
✓ Checked 2 documents across workspace.
✓ 0 errors, 0 warnings. Workspace is COMPLIANT.
```

If an invariant or relation target is broken, the compiler immediately reports actionable diagnostics:

```text
ERROR [ONT-003] in entities/customer.md:
  ods.related[0] target 'subscription.md' does not exist on disk.
  Fix: Create 'entities/subscription.md' or update the relation target.
```

---

## Part 3: Attested Computations

An ontology says what a `Customer` *is*. An **attested computation** says how a number about customers was *actually produced* — and proves it.

The problem it solves: a document says "MRR was $412,000 in Q2." Six weeks later nobody can say which query produced it, against which table, or whether anyone checked. An LLM asked to "recalculate MRR" cheerfully invents a plausible query.

An attested computation is a document that carries the sanctioned query, its parameter schema, how to run it, and a deterministic script that verifies the run happened:

```markdown
---
type: Attested Computation
title: Monthly Active Customer MRR
description: Verified BigQuery calculation of MRR per customer cohort.
tags: [computation, billing]
runtime: bigquery
parameters:
  - name: cohort_year
    type: integer
    required: true
    default: 2026
executor:
  resource: skills/run-bigquery.md
  receipt:
    - job_id
    - query_hash
    - total_bytes_billed
attester:
  resource: attesters/verify-mrr-receipt.py
verified:
  - by: "human:alice"
    at: "2026-08-20T00:00:00Z"
ods:
  profile: note
  status: stable
---

# Monthly Active Customer MRR

## Computation

```sql
SELECT customer_id, SUM(amount_usd) AS mrr
FROM `analytics.billing.active_subscriptions`
WHERE EXTRACT(YEAR FROM created_at) = @cohort_year
GROUP BY 1;
```

## Parameters
- `@cohort_year`: 4-digit registration year.
```

Four keys carry the weight:

| Key | Answers |
| :--- | :--- |
| `runtime` | Where it runs — `bigquery`, `postgres`, `dbt`, `python`. Required. |
| `parameters` | What may vary, and of what type. The runner refuses a mistyped argument. |
| `executor.receipt` | What evidence the run must emit — a job id, a query hash, bytes billed. |
| `attester.resource` | A **non-LLM** script that inspects the receipt and exits 0 or non-zero. |

That last one is the point. The verification is deterministic code, not a model asserting that things look fine. An agent may only elevate the result to trusted when the attester exits 0.

A workspace can gate what is runnable at all:

```toml
# ods.toml
[attestation]
allowed_runtimes = ["bigquery", "postgres"]
enforce_receipts = true
```

Reach for this when a number in a document drives a decision and "trust me" is not good enough. Skip it entirely otherwise — most documentation never needs it.

Full contract: [`specs/assets.md` §9](../specs/assets.md#9-okf-attested-computation-contracts).

---

## Summary

- **Ontologies (`ods.entity`, `ods.domain`, `ods.schema`, `ods.related`, `ods.invariants`)** provide deterministic business meaning and guardrails.
- **Memory (`memory.tier`, `memory.valid_from`, `memory.valid_to`, `memory.mutations`, `memory.pin`)** tracks state evolution over time, in a top-level `memory:` block.
- Ontology keys live directly under `ods:`; memory keys live in the top-level `memory:` block. The flat `ods.tier` / `ods.valid_from` / … spellings still parse, but they are deprecated — see [`specs/graph.md` §5.1](../specs/graph.md#51-canonical-placement).
