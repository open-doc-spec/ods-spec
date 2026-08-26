---
description: Episodic agent memory recording customer tier upgrade event.
tags: [memory, episodic, billing]
owner: agent:billing-assistant
ods:
  profile: memory-episode
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
    - entities/customer.md
---

# Customer Plan Upgrade Episode

## Interaction Summary
User requested an immediate account upgrade from Starter to Enterprise plan following contract signature.

## Tool Execution Trace
- Called `stripe.subscriptions.update(cust_4048, plan="enterprise")` -> Success (HTTP 200).
- Emitted audit log `AUDIT-UPGRADE-4048`.

## State Mutations
- `Customer:cust-4048.plan` transitioned from `"starter"` to `"enterprise"`.
