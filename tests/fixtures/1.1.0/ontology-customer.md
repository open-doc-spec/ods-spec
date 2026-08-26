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
  schema: schemas/customer.schema.json
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
  depends:
    - entities/subscription.md
    - policies/refund-sla.md
---

# Customer Entity

## Overview
The Customer entity represents a legal or individual billing entity registered in the system.

## Attributes
- `id`: Unique identifier (e.g. `cust-4048`).
- `name`: Organization or individual full name.
- `plan`: Active tier (`starter`, `pro`, `enterprise`).
- `mrr`: Monthly recurring revenue in USD.

## Relationships
- **owns**: `Subscription` (1 to N).
- **governed_by**: `Refund SLA` (Policy).
- **maps_to**: BigQuery `analytics.customers` table.

## Invariants
- MRR must be non-negative (`mrr >= 0`).
- Valid contact email is mandatory.

## Examples
```json
{
  "id": "cust-4048",
  "name": "Acme Corp",
  "plan": "enterprise",
  "mrr": 2400.00
}
```
