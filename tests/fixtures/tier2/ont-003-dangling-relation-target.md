---
description: "A deprecated ods.relations edge whose target does not resolve."
x-ods-expect: ONT-003
ods:
  profile: note
  status: draft
  entity: OrphanedLedger
  domain: Billing
  relations:
    - predicate: maps_to
      target: datasets/never-created.sql
---

# Dangling Relation Target
