---
description: Canonical test fixture demonstrating Pareto 80/20 relation syntax in ODS 1.1.
tags: [ontology, customer, billing]
owner: team:billing
ods:
  profile: note
  status: stable
  entity: Customer
  domain: Billing
  related:
    - is_a: Account
    - owns: [Subscription, Invoice, PaymentMethod]
    - governed_by: RefundPolicy
    - maps_to: datasets/bq-customers.sql
    - see_also: ../valid/valid-guide.md
---

# Pareto 80/20 Relation Test Fixture

## Overview
Demonstrates simplified `- predicate: target` syntax and symbolic entity targets.
