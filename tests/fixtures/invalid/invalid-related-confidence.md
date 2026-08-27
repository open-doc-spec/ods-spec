---
description: This document contains an out-of-range confidence score in an attributed relation edge.
tags: [test]
ods:
  profile: note
  status: stable
  related:
    - predicate: governed_by
      target: "@RefundPolicy"
      confidence: 1.5
---

# Illegal Confidence Score Test

This document must fail validation because `confidence` must be between 0.0 and 1.0.
