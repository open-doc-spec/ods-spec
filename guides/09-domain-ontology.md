---
description: Optional Pareto ontology workflow for ODS 2.1 — entity definitions, schema binding, and typed related predicates.
profile: guide
status: stable
depends:
  - ../specs/graph.md
  - ../specs/keys.md
related:
  - 03-link-documents.md
  - 08-extend-ods.md
  - ../specs/validation.md
---

# Domain Ontology (ODS 2.1)

ODS **2.1** adds an optional Pareto-style ontology layer on top of the ODS 2.0 core. You do not need it for everyday documentation — enable it when domain concepts, policies, and APIs need explicit semantic links.

## When to use ontology

| Use ontology when… | Stay on 2.0 core when… |
| :--- | :--- |
| You model named concepts (`Customer`, `RefundRequest`) | Plain doc linking is enough |
| Policies govern features (`governed_by`) | You only need "see also" links |
| APIs or tables bind to concepts (`maps_to`) | No structured domain graph is required |

Set `spec = "2.1"` in `ods.toml` (or load `@ods/pack-pareto-ontology`).

```toml
spec = "2.1"

[ontology]
default_domain = "Core"
strict_schema = true
```

---

## Step 1 — Define an entity document

Create a concept-definition document with `entity`, optional `domain`, and optional `schema`:

```yaml
---
title: Customer
entity: Customer
domain: Billing
description: Billable customer in the subscription system.
profile: note
status: stable
schema: schemas/customer.schema.json
resources:
  - schemas/customer.schema.json
load:
  - schemas/customer.schema.json
---

# Customer

## Definition
A person or organization that can be invoiced.
```

- `schema` is linted (`ONT-001`) — the file must exist on disk.
- Mirror the schema in `resources` (human catalog) and `load` (agent injection) when both audiences need it.

---

## Step 2 — Link from feature or API docs

Use typed `related` for domain semantics. Use `depends` only for Markdown prerequisites:

```yaml
---
title: Refund Processing
entity: RefundRequest
profile: guide
status: stable
depends:
  - auth/sessions.md
related:
  - governed_by: policies/refund-policy.md
  - maps_to: api/refunds-api.md
code:
  - apps/billing/src/refund.ts
load:
  - schemas/refund-payload.json
---

# Refund Processing
```

### The five predicates

| Predicate | Meaning |
| :--- | :--- |
| `is_a` | Subtype / specialization |
| `part_of` | Composition |
| `owns` | Lifecycle ownership |
| `governed_by` | Policy or rule document |
| `maps_to` | API, table, or physical binding |

For rare domain verbs, use the escape hatch:

```yaml
related:
  - predicate: custom
    verb: replaces
    target: decisions/legacy-refund-adr.md
```

---

## Step 3 — Rules authors must follow

1. **`depends` = Markdown only** — never put `.json` schemas in `depends`; use `load`.
2. **Typed `related` = semantics** — not auto-traversed by `ods context`.
3. **`schema` is linted** — keep paths workspace-relative and on disk.
4. **Guardrails in prose** — use `profile: policy` or headings; never `invariants:`.

---

## Lint rules (2.1)

| Rule | What it checks |
| :--- | :--- |
| `ONT-001` | `schema` path exists on disk |
| `ENT-001` | `entity` resolves to a definition document |
| `ENT-002` | No duplicate `entity` names |
| `ENUM-006` | Unknown predicate key in `related` |
| `GRAPH-003` | Every `related` target (plain or typed) exists |

Run `ods lint` in CI after adopting 2.1.

---

## You can stop here

If your workspace only needs flat `depends` / `related` strings, stay on `spec = "2.0"`. Ontology is entirely optional.
