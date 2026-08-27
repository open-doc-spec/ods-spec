---
description: Test fixture demonstrating code binding where symbol is an array and path/role are singular.
tags: [code, ast, binding]
owner: team:auth
ods:
  profile: note
  status: stable
  code:
    - path: src/auth/tokens.ts
      role: implementation
      symbol:
        - generateToken
        - verifyToken
        - revokeToken
      description: JWT token generation and verification routines
---

# Code Multi-Symbol Binding Test Fixture

## Overview
Demonstrates strict singularity for path, role, description with array support strictly on symbol.
