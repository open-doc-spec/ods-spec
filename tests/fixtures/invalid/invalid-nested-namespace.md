---
description: Invalid document attempting nested ontology namespace under ods:.
tags: [test, invalid]
ods:
  profile: note
  status: stable
  ontology:
    entity: Customer
    domain: Billing
---

# Invalid Nested Namespace

This document should fail schema validation because `ontology` is not a permitted key under `ods:`.
