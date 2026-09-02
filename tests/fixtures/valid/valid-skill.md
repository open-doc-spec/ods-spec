---
description: Reusable agent skill for ODS document validation.
tags:
- skill
- ods
owner: team:docs
profile: skill
status: stable
share: public
---

# ODS Linter Skill

## Purpose
Validates ODS documents against formal schemas.

## Capability
Runs schema and semantic validation.

## Activation
Trigger on commit or PR.

## Scope
All markdown files in repository.

## Non-Scope
Code compilation.

## Inputs
Workspace root path.

## Outputs
Diagnostic report.

## Workflow
1. Parse frontmatter.
2. Validate against schema.

## Rules
Preserve third-party metadata.

## Priority
Errors before warnings.

## Validation
Compare against test fixtures.

## Eval
Run fixture test suite.

## Resources
ODS Specification.

## Tools
ODS CLI.

## Lifecycle
Stateless execution.

## Traceability
Log RFC 3339 timestamps.
