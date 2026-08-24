---
description: Autonomous task contract for database schema synchronization.
tags:
  - agent
  - db
owner: team:data
ods:
  profile: agent
  status: stable
  share: org
  code:
    - path: src/migrator.ts
      role: entrypoint
      symbol: syncSchema
---

# Database Sync Agent

## Goal
Automate staging database schema updates.

## Task
Inspect schema diffs and run pending migrations.

## Scope
Tables under the public schema.

## Non-Scope
Data deletion or altering production.

## Context
TypeScript migration runner wrapping PostgreSQL.

## Inputs
Migration files in migrations directory.

## Constraints
Must run in a transaction.

## Priority
Data safety over speed.

## Steps
1. Verify connectivity.
2. Apply migrations.

## Output
Migration execution status log.

## Success Criteria
Zero errors during migration.

## Failure Modes
Rollback transaction on query error.

## Dependencies
None.

## Assumptions
Target is PostgreSQL 16.

## Examples
Run via CLI runner.
