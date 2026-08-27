---
description: "An attested computation whose default violates its own parameter type."
x-ods-expect: ATT-002
type: Attested Computation
title: Cohort MRR With A Mistyped Parameter
runtime: bigquery
parameters:
  - name: cohort_year
    type: integer
    required: true
    description: Cohort registration calendar year
    default: "two thousand twenty six"
executor:
  resource: skills/run-bigquery.md
  receipt:
    - job_id
    - query_hash
ods:
  profile: note
  status: draft
---

# Parameter Type Mismatch

## Computation

The declared parameter type is `integer`; the supplied default is a string.
Binding it would pass a malformed argument to the runtime.
