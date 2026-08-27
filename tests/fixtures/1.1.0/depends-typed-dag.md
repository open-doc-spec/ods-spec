---
description: Test fixture demonstrating typed DAG prerequisite relations under ods.depends.
tags: [test, fixture, depends, dag]
owner: team:platform
ods:
  profile: note
  status: stable
  depends:
    - "@jwt-auth.md"
    - requires: "@database-setup.md"
    - extends: "@base-service-spec.md"
    - imports: ["@common-types.schema.json", "@events.schema.json"]
    - predicate: requires
      target: "@redis-cluster.md"
      optional: false
      scope: runtime
---

# Typed DAG Dependency Test Fixture

## Overview
Demonstrates typed prerequisite child keys under `ods.depends`.
