---
description: "An attested computation whose attester cannot verify the receipt."
x-ods-expect: ATT-003
type: Attested Computation
title: Cohort MRR With An Unverifiable Receipt
runtime: bigquery
parameters:
  - name: cohort_year
    type: integer
    required: true
    default: 2026
executor:
  resource: skills/run-bigquery.md
  receipt:
    - job_id
    - query_hash
    - total_bytes_billed
attester:
  resource: attesters/verify-mrr-receipt.py
ods:
  profile: note
  status: draft
---

# Attester Rejects The Receipt

## Computation

The executor emits a receipt, but the deterministic attester exits non-zero when
inspecting it. The result MUST NOT be elevated to machine-confirmed trust.
