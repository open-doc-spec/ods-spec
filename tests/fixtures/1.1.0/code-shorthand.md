---
description: Test fixture demonstrating code string shorthand, symbol arrays, and descriptions.
tags: [test, code]
owner: team:security
author: Alice Smith
created_at: 2026-08-26
updated_at: 2026-08-26
ods:
  profile: note
  status: stable
  code:
    - src/auth.ts
    - path: tests/auth.test.ts
      role: test
      symbol:
        - TestJwtSign
        - TestJwtVerify
      description: "Verifies JWT signing and signature verification."
---

# Code Shorthand Test Fixture

## Overview
Demonstrates flexible code binding ergonomics.
