---
description: Test fixture demonstrating the 5 Pareto Core Verbs and attributed relation edges under ods.related.
tags: [test, fixture, related, pareto]
owner: team:billing
ods:
  profile: note
  status: stable
  entity: EnterpriseSubscription
  domain: Billing
  related:
    - "@pricing-faq.md"
    - is_a: Account
    - part_of: BillingEngine
    - owns: [Subscription, Invoice, PaymentMethod]
    - governed_by: RefundPolicy
    - see_also: "@billing-guide.md"
    - predicate: governed_by
      target: "@RefundPolicy"
      role: "Compliance SLA"
      confidence: 0.98
      since: 2026-01-01
      until: 2027-01-01
      cardinality: "1..*"
      description: "Primary refund governance policy"
---

# 5 Pareto Core Verbs Test Fixture

## Overview
Demonstrates `is_a`, `part_of`, `owns`, `governed_by`, `see_also`, and attributed relation edges.
