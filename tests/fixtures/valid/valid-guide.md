---
description: Step-by-step tutorial for issuing authentication tokens.
tags:
  - auth
  - tutorial
owner: team:auth
created: 2026-08-14
updated: 2026-08-14
ods:
  profile: guide
  status: stable
  share: public
  depends:
    - valid-note.md
  code:
    - path: src/auth.ts
      role: entrypoint
      symbol: createToken
  resources:
    - path: diagrams/auth-flow.png
---

# Authentication Token Guide

## Overview
How to generate and verify API tokens.

## Prerequisites
- Node.js 20+

## Steps
1. Install auth package.
2. Configure JWT secret.

## Troubleshooting
Check token expiration timestamps.
