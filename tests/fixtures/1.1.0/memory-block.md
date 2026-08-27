---
description: Episodic agent memory with top-level encapsulated memory block.
tags: [memory, episodic, billing]
owner: agent:billing-assistant
memory:
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
ods:
  profile: note
  status: stable
  depends:
    - ontology-customer.md
---

# Customer Plan Upgrade Episode

## Interaction Summary
User requested an immediate account upgrade from Starter to Enterprise plan following contract signature.

## Tool Execution Trace
- Called `stripe.subscriptions.update(cust_4048, plan="enterprise")` -> Success (HTTP 200).
- Emitted audit log `AUDIT-UPGRADE-4048`.

## State Mutations
- `Customer:cust-4048.plan` transitioned from `"starter"` to `"enterprise"`.
