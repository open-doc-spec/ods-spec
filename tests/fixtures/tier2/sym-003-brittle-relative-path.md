---
description: "A deep relative path used where a clean @handle would resolve."
x-ods-expect: SYM-003
ods:
  profile: note
  status: draft
  depends:
    - ../../../specs/keys.md
---

# Brittle Relative Path

`../../../specs/keys.md` traverses three directory levels. `@keys.md` resolves
to the same document and survives a move.
