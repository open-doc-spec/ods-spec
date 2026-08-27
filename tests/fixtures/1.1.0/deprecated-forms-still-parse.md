---
description: "Every form deprecated in ODS 1.1 must still parse without error."
tags: [deprecation, compatibility]
owner: team:ods
ods:
  profile: note
  status: stable
  entity: LegacyBillingAccount
  domain: Billing
  relations:
    - predicate: owns
      target: tests/fixtures/1.1.0/minimal-starter.md
  tier: episodic
  valid_from: "2026-08-26T00:00:00Z"
  valid_to: null
  asserted_at: "2026-08-26T00:05:00Z"
  pin: true
  mutations:
    - entity: LegacyBillingAccount
      id: acct-1
      property: plan
      old_value: "starter"
      new_value: "growth"
---

# Deprecated Forms

Uses `ods.relations` (DEPR-001) and the flat memory keys (DEPR-002). Both emit warnings and both must keep working until 2.0.
