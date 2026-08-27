---
description: "Circular is_a inheritance between entity classes."
x-ods-expect: ONT-004
ods:
  profile: note
  status: draft
  entity: CycleAlpha
  domain: Core
  related:
    - is_a: "@CycleBeta"
---

# Inheritance Cycle Alpha

`CycleBeta` declares `is_a: @CycleAlpha` in turn. Classification must not loop.
