---
description: Test fixture demonstrating the complete Dual-Graph architecture with typed depends and typed related keys.
tags: [test, fixture, dual-graph, ontology]
owner: team:architecture
ods:
  profile: note
  status: stable
  entity: PaymentProcessor
  domain: Payments
  depends:
    - requires: "@auth-sessions.md"
    - extends: "@base-gateway.md"
    - imports: "@transaction-schema.json"
  related:
    - is_a: Gateway
    - part_of: CoreBillingCluster
    - owns: ["@TransactionBatch", "@SettlementRecord"]
    - governed_by: "@PCICompliancePolicy"
    - see_also: "@payments-faq.md"
---

# Dual-Graph Complete Test Fixture

## Overview
Demonstrates simultaneous use of typed DAG prerequisites (`depends`) and typed domain ontology (`related`).
