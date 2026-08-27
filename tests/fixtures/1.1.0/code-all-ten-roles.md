---
description: "Exercises all ten standard code roles, including interface and fixture."
tags: [code, roles]
owner: team:ods
ods:
  profile: note
  status: stable
  code:
    - path: src/ods-spec-conformance/src/main.rs
      role: entrypoint
      symbol: main
    - path: src/ods-spec-conformance/src/lib.rs
      role: implementation
      symbol: find_broken_links
    - path: src/ods-spec-conformance/src/lib.rs
      role: interface
      symbol: BrokenLink
    - path: src/ods-spec-conformance/tests/conformance.rs
      role: test
      symbol:
        - test_no_broken_internal_links
        - test_prose_matches_schema_enums
    - path: tests/fixtures/1.1.0/minimal-starter.md
      role: fixture
    - path: schemas/1.1.0/document.schema.json
      role: schema
    - path: schemas/1.0.0/document.schema.json
      role: migration
    - path: Cargo.toml
      role: config
    - path: .cargo/config.toml
      role: infrastructure
    - path: ods.toml
      role: pipeline
---

# All Ten Code Roles
