---
description: This document contains an illegal code role under ods.code.
tags:
  - test
ods:
  profile: note
  status: stable
  code:
    - path: src/main.rs
      role: superhero_role
---

# Illegal Code Role Test

This document must fail validation because `superhero_role` is not a valid code role enum.
