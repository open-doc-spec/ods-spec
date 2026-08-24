---
description: Profile definition for architecture decision records.
tags:
  - profile
  - adr
owner: team:arch
ods:
  custom_profile:
    name: custom-decision
    description: Custom Architecture Decision Record profile contract
    required_sections:
      - Context
      - Decision
      - Consequences
    optional_sections:
      - Alternatives
      - Compliance
    required_keys:
      - owner
    forbidden_keys:
      - title
---

# Custom Decision Profile

This file declares the custom-decision profile contract.
