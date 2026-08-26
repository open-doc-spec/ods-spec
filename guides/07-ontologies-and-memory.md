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

# Ontologies & Agent Memory

This guide teaches you how to use **Neuro-Symbolic Ontologies** and **Cognitive Memory** in Open Document Spec (ODS). By the end of this tutorial, you will know how to ground AI coding agents in unambiguous business concepts, enforce hard validation guardrails, and track state changes over time.

---

## Overview

Traditional documentation stores unstructured paragraphs of text. While humans can parse nuance, AI agents frequently misunderstand implicit business rules, invent invalid relationships, and hallucinate outdated facts.

ODS solves this by introducing **Flat Pareto Keys** for:
1. **Ontologies**: Defining clear business entity classes, domain boundaries, typed relationships, and deterministic invariants.
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

## Part 1: Neuro-Symbolic Ontologies

An **Ontology** in ODS gives AI agents a shared, machine-verifiable vocabulary. Instead of guessing how "Customers" relate to "Subscriptions" and "Refunds", you state the rules explicitly in YAML frontmatter.

### The Direct Pareto Ontology Keys

All ontology keys live directly under `ods:` with zero unnecessary nesting:

```yaml
ods:
  profile: note                               # Standard universal profile (or architecture / feature)
  status: stable

  entity: Customer                            # Canonical entity class name
  domain: Billing                             # Business domain partition
  schema: schemas/customer.schema.json        # "Paid at the door" disk validator

  relations:
    - predicate: owns
      target: entities/subscription.md
    - predicate: governed_by
      target: policies/refund-sla.md
    - predicate: maps_to
      target: datasets/bq-customers.md

  invariants:
    - "mrr >= 0"
    - "email is required"
```

| Key | What it Does | Why AI Needs It |
| :--- | :--- | :--- |
| **`entity`** | Names the primary business class or concept. | Disambiguates terms (e.g. `Customer` vs `UserAccount`). |
| **`domain`** | Partitions the workspace into business sub-systems. | Allows AI agents to filter context by domain (e.g. `domain: Billing`). |
| **`schema`** | Points to a disk schema file (`.json`, `.ts`, `.py`). | **Paid-at-the-door contract**: Rejects invalid payloads before code execution. |
| **`relations`** | Declares typed semantic edges between concepts. | Replaces ambiguous prose links with explicit graph relations. |
| **`invariants`**| Non-negotiable boolean business guardrails. | AI agents MUST evaluate these before mutating application state. |

### Standard Semantic Predicates

When connecting entities in `ods.relations`, use these standard predicates:

- `is_a`: Class inheritance (e.g., `EnterpriseCustomer` $\rightarrow$ `Customer`).
- `part_of`: Structural composition (e.g., `InvoiceLineItem` $\rightarrow$ `Invoice`).
- `owns`: Domain ownership & lifecycle containment (e.g., `Customer` $\rightarrow$ `Subscription`).
- `governed_by`: Compliance rules or policies (e.g., `Refund` $\rightarrow$ `RefundPolicy`).
- `maps_to`: Technical asset binding (e.g., `Customer` $\rightarrow$ BigQuery Table / OpenAPI Route).
- `derives_from`: Data lineage or computed derivation.

---

## Part 2: Temporal Agent Memory

As AI agents execute tasks, they observe events and make decisions. **Cognitive Memory** enables agents to record what happened, when it became true, and how entity state mutated.

### The Direct Pareto Memory Keys

Memory keys also sit directly under `ods:` on a standard profile:

```yaml
ods:
  profile: note                               # Standard universal profile (or sop / meeting)
  status: stable

  tier: episodic                              # semantic | procedural | episodic | profile
  valid_from: 2026-08-26T10:00:00Z            # When the fact became true in reality
  valid_to: null                              # null = currently active fact
  asserted_at: 2026-08-26T10:05:00Z           # When the agent observed the event
  pin: true                                   # Protects from automated decay pruning

  mutations:
    - entity: Customer
      id: cust-4048
      property: plan
      old_value: "starter"
      new_value: "enterprise"
      confidence: 1.0
```

| Key | What it Does | Why AI Needs It |
| :--- | :--- | :--- |
| **`tier`** | Classifies memory: `episodic` (events), `profile` (distilled entity state), `procedural` (how-to). | Helps agents retrieve the right level of memory abstraction. |
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
  relations:
    - predicate: owns
      target: subscription.md
    - predicate: governed_by
      target: ../policies/refund-sla.md
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
  ods.relations[0].target 'subscription.md' does not exist on disk.
  Fix: Create 'entities/subscription.md' or update relation target.
```

---

## Summary

- **Ontologies (`ods.entity`, `ods.domain`, `ods.schema`, `ods.relations`, `ods.invariants`)** provide deterministic business meaning and guardrails.
- **Memory (`ods.tier`, `ods.valid_from`, `ods.valid_to`, `ods.mutations`, `ods.pin`)** tracks state evolution over time.
- All keys live directly under `ods:` with zero unnecessary nesting, keeping your documentation clean, readable, and machine-verifiable.
