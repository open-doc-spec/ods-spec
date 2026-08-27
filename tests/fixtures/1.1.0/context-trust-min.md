---
description: "Bounded context that refuses to act on unreviewed documentation."
tags: [context, trust]
owner: team:ods
verified:
  - by: "human:ahormati"
    at: "2026-08-20T00:00:00Z"
ods:
  profile: guide
  status: stable
  depends:
    - ./minimal-starter.md
  context:
    max-depth: 2
    trust-min: human-reviewed
    load:
      - ../../../schemas/1.1.0/profile.schema.json
    ignore:
      - archive/
---

# Trust-Gated Context

## Overview
This document may only be assembled alongside human-reviewed prerequisites.

## Prerequisites
- A workspace with at least one `verified` entry per prerequisite.

## Steps
1. Resolve context with `trust-min: human-reviewed`.
2. Inspect the reported exclusions.

## Troubleshooting
- **Empty payload**: every prerequisite is below the trust bar. Lower `trust-min` or get the docs reviewed.
