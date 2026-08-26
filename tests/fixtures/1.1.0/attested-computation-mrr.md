---
type: Attested Computation
title: Monthly Active Customer MRR Calculation
description: Verified BigQuery SQL calculation computing MRR per customer cohort.
tags: [computation, billing, bigquery]
runtime: bigquery
parameters:
  - name: cohort_year
    type: integer
    required: true
    description: Cohort registration calendar year
executor:
  resource: skills/run-bigquery.md
  receipt:
    - job_id
    - query_hash
    - total_bytes_billed
attester:
  resource: attesters/verify-mrr-receipt.py
sources:
  - id: bq-orders
    resource: datasets/billing/orders.sql
    author: team:data-platform
    usage_count: 8500
verified:
  - by: "human:ahormati"
    at: "2026-08-20T00:00:00Z"
ods:
  profile: note
  status: stable
---

# Monthly Active Customer MRR Calculation

## Computation
```sql
SELECT
  customer_id,
  SUM(amount_usd) AS mrr
FROM `analytics.billing.active_subscriptions`
WHERE EXTRACT(YEAR FROM created_at) = @cohort_year
GROUP BY 1;
```

## Parameters
- `@cohort_year`: 4-digit registration year (e.g. 2026).

## Verification Rationale
Queries production BigQuery replica using signed job credentials with cryptographic hash attestation.
