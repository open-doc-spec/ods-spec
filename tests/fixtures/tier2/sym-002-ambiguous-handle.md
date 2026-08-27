---
description: "A bare basename handle matching more than one file."
x-ods-expect: SYM-002
ods:
  profile: note
  status: draft
  depends:
    - "@README.md"
---

# Ambiguous Handle

`README.md` exists at the repository root, in `specs/`, in `guides/`, and in `schemas/`. The handle must be disambiguated with a folder prefix.
