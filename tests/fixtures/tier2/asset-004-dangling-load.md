---
description: "A context.load fixture that does not exist on disk."
x-ods-expect: ASSET-004
ods:
  profile: note
  status: draft
  context:
    load:
      - ./fixtures/missing-payload.json
---

# Dangling Context Load
