---
description: Test fixture demonstrating natural predicate synonyms in ODS 1.1.
tags: [ontology, synonyms]
owner: team:data
ods:
  profile: note
  status: stable
  entity: EnterpriseCustomer
  domain: Billing
  related:
    - extends: Customer
    - contains: [Subscription, Invoice]
    - policy: RefundPolicy
    - table: datasets/bq-customers.sql
    - see: ../valid/valid-guide.md
---

# Natural Predicate Synonyms Test Fixture

## Overview
Demonstrates natural predicate synonyms (extends, contains, policy, table, see) normalizing cleanly.
