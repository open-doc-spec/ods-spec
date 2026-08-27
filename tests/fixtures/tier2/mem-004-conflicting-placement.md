---
description: "The same memory field declared twice with conflicting values."
x-ods-expect: MEM-004
memory:
  tier: episodic
ods:
  profile: note
  status: draft
  tier: semantic
---

# Conflicting Memory Placement

The canonical `memory:` block says `episodic`; the deprecated flat `ods.tier` says `semantic`. Precedence resolves ties, not contradictions.
