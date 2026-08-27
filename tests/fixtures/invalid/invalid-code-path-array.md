---
description: This document contains an illegal array for code path.
tags: [test]
ods:
  profile: note
  status: stable
  code:
    - path:
        - src/a.ts
        - src/b.ts
      role: implementation
---

# Illegal Code Path Array Test

This document must fail validation because code path must be a singular string, not an array.
