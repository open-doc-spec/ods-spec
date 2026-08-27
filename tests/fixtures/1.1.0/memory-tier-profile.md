---
description: "A distilled actor profile memory node, the fifth cognitive tier."
tags: [memory, profile]
owner: team:ods
memory:
  tier: profile
  valid_from: "2026-01-01T00:00:00Z"
  valid_to: null
  asserted_at: "2026-08-27T09:00:00Z"
  pin: true
  mutations:
    - entity: Customer
      id: cust-4048
      property: preferred_channel
      old_value: "email"
      new_value: "slack"
      confidence: 0.92
ods:
  profile: note
  status: stable
---

# Customer 4048 · Distilled Profile

Accumulated across many episodes. Distinct from a `state` node: this survives the expiry of any single observation it was built from.
