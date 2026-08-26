---
type: BigQuery Table
title: Customers Analytics Table
description: Core production analytics table recording all registered SaaS customer accounts.
tags: [analytics, bigquery, customers]
resource: bq://corp-analytics.production.customers
sources:
  - id: bq-raw
    resource: datasets/billing/raw_customers.sql
    author: team:data-platform
    usage_count: 12500
    last_modified: 2026-08-20T00:00:00Z
usage_window:
  from: "2026-01-01T00:00:00Z"
  to: "2026-06-30T23:59:59Z"
generated:
  by: dbt/v1.8.0
  at: "2026-08-21T04:00:00Z"
verified:
  - by: "human:ahormati"
    at: "2026-08-22T00:00:00Z"
status: stable
stale_after: "2026-12-31T23:59:59Z"
ods:
  profile: note
  status: stable
  entity: Customer
  domain: Analytics
  relations:
    - predicate: maps_to
      target: entities/customer.md
---

# Customers Analytics Table

## Overview
This BigQuery table maintains the single source of truth for customer account states, plan tiers, and MRR.

## Schema Definition
- `customer_id` (STRING, REQUIRED): Unique UUID.
- `plan_tier` (STRING, REQUIRED): `starter`, `pro`, or `enterprise`.
- `mrr_usd` (NUMERIC, REQUIRED): Monthly recurring revenue.
- `updated_at` (TIMESTAMP, REQUIRED): Last modification instant.
