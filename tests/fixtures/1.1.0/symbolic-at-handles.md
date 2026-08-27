---
description: Canonical test fixture demonstrating Universal @ Symbolic Handle Resolution across all ODS keys.
tags: [ontology, symbolic, handles]
owner: team:platform
ods:
  profile: note
  status: stable
  entity: CustomerSession
  domain: Identity
  schema: "@session.schema.json"
  depends:
    - "@valid-guide.md"
  related:
    - is_a: "@Account"
    - owns: ["@Subscription", "@Invoice"]
    - governed_by: "@RefundPolicy"
    - see_also: "@valid-agent.md"
  resources:
    - "@session-flow.png"
  code:
    - path: "@server.ts"
      role: entrypoint
      symbol: startAuthServer
  context:
    max-depth: 2
    load:
      - "@sample-payload.json"
---

# Universal @ Symbolic Handle Resolution Test Fixture

## Overview
Demonstrates move-resilient @ handles across schema, depends, related, resources, code, and context.load.
